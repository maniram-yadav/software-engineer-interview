# Design a Global DNS System with Health-Based Routing and Failover — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Resolve domain names to IP addresses, routing users to the geographically/topologically nearest healthy endpoint
- Continuously monitor the health of backend endpoints and automatically stop routing to unhealthy ones
- Support automatic failover — if a region/datacenter goes down, traffic reroutes to healthy alternatives within seconds
- Support weighted/percentage-based traffic distribution (for gradual rollouts, A/B testing at the infrastructure level)

### Non-Functional Requirements
- **Extremely high availability:** DNS is the literal first step of nearly every user interaction — a DNS outage is effectively a total platform outage
- **Low resolution latency:** DNS lookups happen before ANY other part of a request — added latency here delays everything downstream
- **Fast failure detection and rerouting:** Users should be routed away from failing infrastructure within seconds, not minutes
- **Global scale:** Must serve resolution queries from users worldwide with consistently low latency, not just from one region

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| DNS queries/sec (large platform) | Hundreds of thousands to millions |
| Health check frequency | Every few seconds per endpoint |
| Failover detection target | < 30 seconds from failure to rerouted traffic |
| DNS TTL (time-to-live) | Seconds to minutes for health-routed records (much shorter than typical static DNS) |

---

## 2. The Core Tension — DNS Caching vs Fast Failover

```mermaid
flowchart TB
    A["Standard DNS design<br/>philosophy: LONG TTLs<br/>(hours/days) — because DNS<br/>resolution should be cached<br/>aggressively for performance,<br/>reducing repeated lookup<br/>overhead"] --> A1["Problem: a long TTL means<br/>that when an endpoint fails,<br/>clients with a CACHED,<br/>now-STALE DNS answer keep<br/>trying to reach the DEAD<br/>endpoint until their cache<br/>naturally expires — potentially<br/>MINUTES or HOURS of continued<br/>failed requests"]

    B["Health-based routing<br/>requirement: SHORT TTLs<br/>(seconds) specifically for<br/>records that need fast<br/>failover capability"] --> B1["Tradeoff: shorter TTLs mean<br/>MORE frequent DNS queries<br/>hitting the authoritative<br/>servers (more load, and<br/>technically slightly higher<br/>average latency due to less<br/>caching) — a DELIBERATE<br/>tradeoff of raw efficiency<br/>for failover responsiveness"]

    C["This tension — caching for<br/>efficiency vs freshness for<br/>failover speed — is the<br/>central design tradeoff this<br/>entire system navigates"] -.-> B1
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    User["User (anywhere globally)"]

    subgraph DNSHierarchy["DNS Resolution Hierarchy"]
        Resolver["User's Local DNS Resolver<br/>(ISP or public, e.g., 8.8.8.8)"]
        AuthNS["Authoritative DNS Servers<br/>(globally distributed,<br/>anycast-routed)"]
    end

    subgraph HealthSystem["Health Monitoring System"]
        HealthCheckers["Distributed Health Checkers<br/>(one fleet per region)"]
        HealthStore[("Health Status Store<br/>— per-endpoint current state")]
    end

    subgraph Backend["Backend Infrastructure"]
        RegionUS["US Region Endpoints"]
        RegionEU["EU Region Endpoints"]
        RegionAPAC["APAC Region Endpoints"]
    end

    User -->|"DNS query"| Resolver
    Resolver -->|"cache miss —<br/>query authoritative"| AuthNS

    AuthNS --> HealthStore
    HealthCheckers --> RegionUS
    HealthCheckers --> RegionEU
    HealthCheckers --> RegionAPAC
    HealthCheckers --> HealthStore

    AuthNS -->|"return healthy,<br/>geographically nearest IP"| Resolver
    Resolver -->|"cached answer,<br/>short TTL"| User
```

**Key idea:** The Authoritative DNS Servers don't just serve static records — they consult a continuously-updated Health Status Store before answering EVERY query, meaning the IP address returned reflects REAL-TIME infrastructure health, not a fixed configuration. This is fundamentally different from traditional DNS, which is purely static/configuration-driven.

---

## 4. Data Model

```mermaid
erDiagram
    ENDPOINT {
        string endpoint_id PK
        string ip_address
        string region
        string status "healthy/unhealthy/degraded"
        timestamp last_health_check_at
        float current_latency_ms
    }
    DNS_RECORD_POLICY {
        string domain PK
        string routing_policy "geo/weighted/failover"
        int ttl_seconds
        list associated_endpoint_ids
    }
    HEALTH_CHECK_RESULT {
        string endpoint_id FK
        timestamp checked_at
        bool passed
        string failure_reason "nullable"
    }
```

---

## 5. Health Check Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Checker as Health Checker<br/>(distributed, multiple<br/>vantage points per endpoint)
    participant Endpoint as Backend Endpoint
    participant HealthStore as Health Status Store

    loop Every few seconds
        Checker->>Endpoint: Health check request<br/>(HTTP GET /health, or<br/>TCP connect, depending<br/>on configured check type)

        alt Endpoint responds successfully
            Endpoint-->>Checker: 200 OK
            Checker->>HealthStore: Record: HEALTHY,<br/>latency measured
        else Endpoint fails to respond<br/>(timeout, connection refused,<br/>error status)
            Checker->>HealthStore: Record: check FAILED
        end
    end

    Note over HealthStore: Status only flips to<br/>UNHEALTHY after MULTIPLE<br/>consecutive failures from<br/>MULTIPLE independent checker<br/>vantage points — avoiding<br/>false positives from a<br/>transient blip or a<br/>checker-side network issue
```

**Why multiple independent vantage points matter for health checking:** A single health checker experiencing ITS OWN network issue could incorrectly report a perfectly healthy endpoint as down — using several geographically distributed checkers and requiring agreement across MULTIPLE of them before declaring an endpoint unhealthy protects against this false-positive risk, similar in principle to the majority-quorum thinking in the Network Partition Detection design.

---

## 6. DNS Query Resolution Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant User as User
    participant LocalResolver as Local DNS Resolver
    participant AuthNS as Authoritative DNS Server
    participant HealthStore as Health Status Store

    User->>LocalResolver: Resolve "api.example.com"

    alt Cached answer available (within TTL)
        LocalResolver-->>User: Return cached IP<br/>(no further lookup needed)
    else Cache miss / TTL expired
        LocalResolver->>AuthNS: Query for "api.example.com"

        AuthNS->>AuthNS: Determine user's approximate<br/>location (from resolver's<br/>source IP, via anycast<br/>routing or EDNS client subnet)

        AuthNS->>HealthStore: Get healthy endpoints,<br/>ordered by proximity to<br/>this user's location
        HealthStore-->>AuthNS: [nearest_healthy_ip,<br/>next_healthy_ip, ...]

        AuthNS-->>LocalResolver: Return nearest HEALTHY IP<br/>+ SHORT TTL (e.g., 30s)
        LocalResolver-->>User: Return IP, cache for<br/>only 30 seconds
    end
```

---

## 7. Automatic Failover Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Endpoint as US-East Endpoint<br/>(fails)
    participant Checker as Health Checkers
    participant HealthStore as Health Status Store
    participant AuthNS as Authoritative DNS
    participant User as Users (querying during<br/>and after the failure)

    Note over Endpoint: US-East region<br/>experiences an outage

    Checker->>Endpoint: Health check
    Note over Endpoint: No response (multiple<br/>consecutive checks, from<br/>multiple vantage points)

    Checker->>HealthStore: Mark US-East endpoint<br/>as UNHEALTHY

    Note over User: A user's cached DNS<br/>answer from BEFORE the<br/>failure expires (short TTL —<br/>within ~30 seconds)

    User->>AuthNS: Fresh DNS query<br/>(cache expired)
    AuthNS->>HealthStore: Get healthy endpoints<br/>near this user

    Note over HealthStore: US-East excluded —<br/>marked unhealthy
    HealthStore-->>AuthNS: Next-nearest HEALTHY<br/>endpoint (e.g., US-West)

    AuthNS-->>User: Return US-West IP instead

    Note over User: Total time from actual<br/>failure to this user being<br/>rerouted: bounded by<br/>(health check detection time)<br/>+ (remaining TTL on their<br/>previously cached answer) —<br/>this is why SHORT TTLs<br/>are essential for fast<br/>failover
```

**Why the short TTL is the critical enabler of fast failover, not just the health checking itself:** Even with instant health detection, a user holding a cached answer with a LONG TTL won't make a new query until that cache expires — they'll keep trying the dead endpoint regardless of how quickly the health system detected the failure. The short TTL is what bounds the WORST CASE time any given user remains misdirected after a failure.

---

## 8. Weighted Traffic Distribution (Gradual Rollouts)

```mermaid
flowchart TB
    A["New infrastructure version<br/>needs gradual traffic rollout<br/>(same canary principle as<br/>the ML Model Serving design,<br/>applied at the DNS/infra level)"] --> B["DNS Record Policy:<br/>weighted routing —<br/>90% of queries → old<br/>infrastructure IP,<br/>10% of queries → new<br/>infrastructure IP"]

    B --> C["Authoritative DNS server<br/>applies weighted random<br/>selection when answering<br/>queries, respecting the<br/>configured percentages"]

    C --> D["As confidence in the new<br/>infrastructure grows,<br/>gradually shift weights:<br/>90/10 → 50/50 → 10/90 → 0/100 —<br/>same gradual-ramp philosophy<br/>as canary model deployment,<br/>applied to infrastructure-level<br/>traffic shifting"]
```

---

## 9. Anycast Routing for Global Low-Latency Resolution

```mermaid
flowchart TB
    A["Problem: a SINGLE physical<br/>DNS server location can't<br/>serve low-latency queries<br/>to users WORLDWIDE — physics<br/>(speed of light) imposes a<br/>hard floor on cross-continent<br/>latency"] --> B["Anycast: the SAME IP address<br/>is announced from MULTIPLE<br/>physical locations globally<br/>via BGP routing"]

    B --> C["Internet routing infrastructure<br/>automatically directs each<br/>user's query to the<br/>TOPOLOGICALLY NEAREST location<br/>announcing that IP — a user<br/>in Tokyo and a user in London<br/>querying the SAME IP address<br/>actually reach DIFFERENT<br/>physical servers, each<br/>nearest to them"]

    D["This is the same fundamental<br/>principle as the Multi-Layer<br/>CDN Caching design's edge<br/>PoP distribution, but applied<br/>at the network routing layer<br/>rather than the application/<br/>content layer"] -.-> C
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Global DNS HLD))
    Authoritative DNS Servers
      Anycast-distributed globally
      Health-aware answer generation
    Health Checkers
      Multiple independent vantage points
      Continuous, frequent checking
    Health Status Store
      Real-time endpoint state
      Consulted on every DNS answer
    TTL Policy
      Short for health-routed records
      Bounds worst-case failover time
    Weighted Routing Policy
      Gradual infrastructure rollout
      Percentage-based traffic shifting
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| TTL strategy | Short TTLs (seconds) for health-routed records | Directly bounds the worst-case time any user remains misdirected to a failed endpoint after failure — the single most important lever for fast failover |
| Health check consensus | Multiple independent vantage points, requiring agreement | Prevents a single checker's own network issue from falsely marking a healthy endpoint as down |
| Global distribution | Anycast routing | The only mechanism providing consistently low-latency DNS resolution to users worldwide, given the hard physical constraint of speed-of-light latency |
| Traffic distribution | Weighted routing support | Enables gradual, safe infrastructure rollouts at the DNS level, mirroring canary deployment principles from application-level rollouts |
| Health-answer coupling | Every DNS answer consults real-time health status | Transforms DNS from a static configuration lookup into a dynamic, infrastructure-aware routing decision |

---

## 12. Bottlenecks & Scaling Considerations

- **Short TTL vs query load tradeoff** — shorter TTLs enable faster failover but directly increase query volume hitting authoritative servers (since clients cache for less time); this requires the authoritative DNS infrastructure itself to be provisioned for meaningfully higher query throughput than a traditional long-TTL setup would require.
- **Health check overhead at scale** — checking every endpoint from every vantage point at a rapid cadence (every few seconds) generates significant aggregate health-check traffic as endpoint count grows; needs careful scaling of the checker fleet independent of, but proportional to, backend infrastructure growth.
- **DNS resolver caching outside your control** — while the authoritative server controls the TTL it ADVERTISES, some intermediate resolvers or client-side caches don't always perfectly respect short TTLs (occasionally over-caching for operational reasons on their end) — this is a real-world limitation of the DNS protocol's trust model that no amount of authoritative-server design can fully eliminate.
- **Health Status Store as the new critical dependency** — since every single DNS answer now depends on this store's current state, its availability and latency directly bound the entire DNS system's — and therefore the whole platform's — availability; this deserves the same critical-path treatment as similar central-dependency stores in other designs (feature stores, session stores, idempotency stores).
- **Split-horizon complexity for enterprise/multi-network scenarios** — some deployments need different DNS answers depending on WHERE the query originates from beyond simple geographic proximity (e.g., internal corporate network vs public internet) — this adds policy complexity beyond the pure health/geo-routing model described here.
- **False failover from health-check blind spots** — a health check verifying "can I reach this endpoint" doesn't necessarily verify "can actual USERS successfully complete their real workflows against this endpoint" — sophisticated systems sometimes need application-level synthetic transaction monitoring, not just basic connectivity checks, to catch subtler classes of degradation that a simple health check would miss.
- **DNSSEC and security considerations** — a globally-distributed, dynamically-answering DNS system is an attractive attack target (DNS spoofing, cache poisoning); production systems need DNSSEC (cryptographically signed DNS responses) and careful protection of the authoritative infrastructure itself, adding meaningful additional complexity beyond the core health-routing logic covered in this design.
