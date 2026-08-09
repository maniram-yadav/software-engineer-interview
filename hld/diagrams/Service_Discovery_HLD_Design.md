# Design a Service Discovery System for a Dynamic Microservices Environment — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Allow services to register themselves (and their network location) when they start up
- Allow services to discover the current, healthy network locations of other services they need to call
- Automatically remove services from discoverability when they become unhealthy or shut down
- Support multiple instances of the same service (for load distribution)

### Non-Functional Requirements
- **Handle constant churn:** In a dynamic environment (auto-scaling, frequent deployments, container orchestration), service instances start and stop CONSTANTLY — this must be the normal case, not an edge case
- **Low-latency lookups:** Service discovery sits on the critical path before almost every inter-service call
- **Consistency vs availability tradeoff:** Must decide how quickly registry changes need to propagate vs how available the registry itself needs to be
- **Self-healing:** Stale entries (services that crashed without deregistering) must not persist indefinitely

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Service instances (platform-wide) | Thousands, constantly changing |
| Registration/deregistration events/sec | Hundreds during normal operation, spikes during mass deployments/scaling events |
| Discovery lookups/sec | Very high — essentially proportional to inter-service call volume |
| Registry update propagation target | Seconds |

---

## 2. The Core Problem — Why Static Configuration Doesn't Work Here

```mermaid
flowchart TB
    A["Traditional approach:<br/>hardcode backend service<br/>IP addresses/hostnames in<br/>configuration files"] --> A1["Works fine for a SMALL,<br/>STATIC set of services that<br/>rarely change location"]

    B["Dynamic microservices reality:<br/>services auto-scale (instance<br/>count changes constantly),<br/>get redeployed (IP addresses<br/>change with every deployment),<br/>and can fail/restart at any<br/>moment"] --> B1["Static configuration becomes<br/>IMMEDIATELY stale — a<br/>hardcoded IP address might<br/>point to a server that no<br/>longer exists, or miss NEW<br/>instances that just scaled up"]

    C["Service discovery solves this<br/>by making 'where is service X<br/>currently running' a QUERYABLE,<br/>CONTINUOUSLY-UPDATED fact,<br/>rather than a static assumption<br/>baked into configuration"] --> B1
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph Services["Service Instances"]
        InstanceA1["Service A<br/>Instance 1"]
        InstanceA2["Service A<br/>Instance 2"]
        InstanceB1["Service B<br/>Instance 1"]
    end

    subgraph Registry["Service Registry"]
        RegistryCluster["Registry Cluster<br/>(consensus-based,<br/>e.g., etcd/Consul —<br/>same pattern as the<br/>Leader Election design)"]
    end

    subgraph HealthCheck["Health Verification"]
        HealthChecker["Health Check Mechanism<br/>(active checks or<br/>passive heartbeats)"]
    end

    subgraph Consumer["Consuming Service"]
        ServiceC["Service C<br/>(needs to call Service A)"]
        LocalCache["Local Registry Cache"]
    end

    InstanceA1 -->|"1. Register on startup"| RegistryCluster
    InstanceA2 -->|"1. Register on startup"| RegistryCluster
    InstanceB1 -->|"1. Register on startup"| RegistryCluster

    InstanceA1 -.->|"2. Periodic heartbeat"| HealthChecker
    InstanceA2 -.->|"2. Periodic heartbeat"| HealthChecker
    HealthChecker --> RegistryCluster

    ServiceC -->|"3. Query: where is<br/>Service A?"| LocalCache
    LocalCache -.->|"periodically refresh"| RegistryCluster
    ServiceC -->|"4. Direct call"| InstanceA1
```

**Key idea:** This shares substantial architectural DNA with the Service Mesh design's Service Registry component (indeed, in many real deployments, this IS the same underlying system) — the registry is a small, highly-consistent cluster tracking service locations, while consuming services maintain a locally-cached view for fast, low-latency lookups rather than querying the registry on every single call.

---

## 4. Data Model

```mermaid
erDiagram
    SERVICE_INSTANCE {
        string instance_id PK
        string service_name
        string ip_address
        int port
        string status "healthy/unhealthy"
        timestamp registered_at
        timestamp last_heartbeat_at
        string ttl_lease_id
    }
```

---

## 5. Registration Flow (Two Approaches) — Detailed Sequence

```mermaid
flowchart TB
    A["How does a service<br/>instance get registered?"] --> B{"Registration Pattern"}

    B --> C["Self-Registration<br/>(Client-Side)"]
    C --> C1["The service instance itself,<br/>on startup, actively calls<br/>the registry's API to<br/>register its own location"]
    C --> C2["PRO: simple, no external<br/>dependency<br/>CON: couples every service's<br/>code to the registry's API —<br/>every language/framework<br/>needs its own registration<br/>client"]

    B --> D["Third-Party Registration<br/>(Platform-Side)"]
    D --> D1["The container orchestration<br/>platform itself (e.g.,<br/>Kubernetes) automatically<br/>registers/deregisters<br/>instances as it starts/stops<br/>them — the SERVICE code<br/>itself has NO awareness of<br/>registration at all"]
    D --> D2["PRO: services remain<br/>completely decoupled from<br/>discovery mechanics<br/>CON: depends on the<br/>orchestration platform<br/>correctly tracking and<br/>reporting instance lifecycle"]

    E["Most modern container-<br/>orchestrated environments<br/>(Kubernetes) favor third-party<br/>registration — the platform<br/>itself already knows exactly<br/>when instances start/stop,<br/>making this the natural,<br/>DRY source of truth"] -.-> D2
```

```mermaid
sequenceDiagram
    participant Orchestrator as Container Orchestrator<br/>(e.g., Kubernetes)
    participant Instance as New Service Instance
    participant Registry as Service Registry

    Orchestrator->>Instance: Start new instance<br/>(auto-scaling or deployment)
    Instance->>Instance: Application boots up

    Orchestrator->>Orchestrator: Detect instance is running<br/>(platform-level awareness)
    Orchestrator->>Registry: Register: {service_name,<br/>ip_address, port}<br/>WITH A TTL LEASE

    Note over Registry: TTL lease means this<br/>registration AUTOMATICALLY<br/>EXPIRES unless renewed —<br/>same lease/heartbeat pattern<br/>as the Leader Election<br/>and Distributed Lock<br/>Manager designs
```

---

## 6. Health Verification & Automatic Deregistration — Detailed Sequence

```mermaid
sequenceDiagram
    participant Orchestrator as Container Orchestrator
    participant Instance as Service Instance
    participant Registry as Service Registry

    loop Periodic renewal (well within TTL)
        Orchestrator->>Instance: Health check<br/>(is the instance actually<br/>responsive?)
        alt Healthy
            Instance-->>Orchestrator: OK
            Orchestrator->>Registry: Renew TTL lease
        else Unhealthy/unresponsive
            Orchestrator->>Registry: STOP renewing lease<br/>(deliberately let it expire)
        end
    end

    Note over Registry: If lease isn't renewed<br/>within its TTL window,<br/>the registry AUTOMATICALLY<br/>removes the entry —<br/>no explicit "deregister"<br/>call is strictly required,<br/>making this self-healing<br/>even if an instance crashes<br/>abruptly without any<br/>graceful shutdown sequence
```

**Why TTL-based expiry (not explicit deregistration alone) is essential for correctness:** A gracefully shutting-down service can explicitly deregister itself, but a service that CRASHES (power loss, OOM kill, network partition) has no opportunity to send a clean "I'm going away" message — TTL-based lease expiry ensures the registry self-heals in EITHER case, treating "stopped renewing" as the reliable signal rather than depending on a graceful goodbye that might never come.

---

## 7. Service Discovery Lookup Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant ServiceC as Service C<br/>(needs to call Service A)
    participant LocalCache as Local Registry Cache<br/>(in-process or sidecar,<br/>same pattern as the<br/>Service Mesh design)
    participant Registry as Service Registry

    ServiceC->>LocalCache: Where is Service A?

    alt Cache is fresh (within refresh interval)
        LocalCache-->>ServiceC: [instance1_ip, instance2_ip]<br/>(no network call to<br/>the registry needed)
    else Cache needs refresh
        LocalCache->>Registry: Query current healthy<br/>instances of Service A
        Registry-->>LocalCache: Current instance list
        LocalCache->>LocalCache: Update local cache
        LocalCache-->>ServiceC: Return current instances
    end

    ServiceC->>ServiceC: Apply load balancing<br/>(e.g., round robin) among<br/>returned instances
```

**Why local caching matters despite the registry being the source of truth:** Querying the central registry on EVERY single inter-service call would make the registry an extreme high-traffic bottleneck and add unnecessary latency to every call; local caching (refreshed periodically or via push notifications from the registry) lets the vast majority of lookups resolve instantly from memory, with the registry only consulted when the cache genuinely needs updating.

---

## 8. Push-Based vs Pull-Based Update Propagation

```mermaid
flowchart TB
    A["How do local caches learn<br/>about registry CHANGES<br/>(new instances, removed<br/>instances)?"] --> B{"Propagation Model"}

    B --> C["Pull-based (polling)"]
    C --> C1["Local cache periodically<br/>asks the registry: 'anything<br/>changed since I last checked?'<br/>PRO: simple<br/>CON: inherent delay between<br/>a change happening and the<br/>cache learning about it<br/>(bounded by poll interval)"]

    B --> D["Push-based (watch/subscribe)"]
    D --> D1["Registry actively NOTIFIES<br/>subscribed clients the<br/>moment a relevant change<br/>occurs (e.g., via a<br/>long-lived watch connection,<br/>similar to the mechanism<br/>in the Leader Election<br/>design's watch-based<br/>discovery)<br/>PRO: near-instant propagation<br/>CON: more complex, registry<br/>must maintain many open<br/>watch connections"]

    E["Production systems often<br/>use a HYBRID: push-based<br/>watches for fast update<br/>propagation, PLUS periodic<br/>polling as a safety-net<br/>reconciliation in case a<br/>watch connection was<br/>silently missed/dropped"] -.-> D1
```

---

## 9. Handling Registry Cluster Failure (Registry's Own Availability)

```mermaid
flowchart TB
    A["The Service Registry itself<br/>must be highly available —<br/>if IT fails, service-to-service<br/>discovery breaks platform-wide"] --> B["Same consensus-based<br/>clustering approach as the<br/>Distributed Consensus and<br/>Leader Election designs —<br/>an odd-numbered cluster<br/>(3, 5 nodes) tolerating<br/>minority node failures<br/>while remaining available"]

    B --> C["Additionally: because<br/>CONSUMING services maintain<br/>LOCAL caches, a brief<br/>registry outage doesn't<br/>immediately break service<br/>discovery platform-wide —<br/>existing cached entries<br/>remain usable, just<br/>increasingly STALE the<br/>longer the outage persists"]

    D["This graceful degradation<br/>property — local caches<br/>providing a buffer against<br/>brief registry unavailability —<br/>is an important resilience<br/>characteristic, though it<br/>doesn't help with NEW<br/>instances that need to<br/>register during the outage"] -.-> C
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Service Discovery HLD))
    Registry Cluster
      Consensus-based, highly available
      Source of truth for instance locations
    Container Orchestrator
      Third-party registration
      Health check driven lease renewal
    TTL Lease Mechanism
      Self-healing on crash
      No dependency on graceful shutdown
    Local Registry Cache
      Fast, low-latency lookups
      Buffers brief registry outages
    Push/Pull Propagation
      Hybrid watch plus polling
      Balances speed and reliability
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Registration pattern | Third-party (platform-driven), not self-registration | Decouples service code entirely from discovery mechanics; leverages the orchestrator's already-authoritative knowledge of instance lifecycle |
| Deregistration mechanism | TTL lease expiry, not explicit-only | Self-heals correctly even when instances crash abruptly without a graceful shutdown opportunity |
| Lookup architecture | Local caching with periodic/push refresh | Prevents the registry from becoming an extreme bottleneck under high inter-service call volume |
| Update propagation | Hybrid push (watch) + pull (polling safety net) | Combines near-instant propagation with resilience against silently dropped watch connections |
| Registry availability | Consensus-based clustering (same as Leader Election design) | Only mechanism providing both correctness and fault tolerance for this critical shared dependency |

---

## 12. Bottlenecks & Scaling Considerations

- **Registration/deregistration churn during mass events** — a large-scale deployment or auto-scaling event can generate a burst of near-simultaneous registration changes; the registry cluster must handle this write burst without degrading lookup performance for unrelated, concurrent discovery queries.
- **Cache staleness during rapid scaling** — if instances scale up rapidly (e.g., responding to a traffic spike) but consuming services' local caches haven't yet refreshed, newly-added capacity isn't immediately utilized — the propagation delay directly impacts how quickly a scale-up event translates into actual traffic distribution improvement.
- **TTL tuning tradeoff** — shorter TTLs mean FASTER detection of crashed instances but require MORE frequent heartbeat/renewal traffic; longer TTLs reduce this overhead but mean a crashed instance remains "discoverable" (and potentially receiving misdirected traffic) for longer — same fundamental tuning tradeoff as the Distributed Lock Manager design's lease TTL.
- **Split registry views during network partition** — if the registry cluster itself experiences an internal partition, the same split-brain risks and quorum-based mitigation covered in the Network Partition Detection design apply directly here — a minority-partition registry node must not serve authoritative (potentially stale/incorrect) discovery answers.
- **Cross-region/cross-cluster service discovery** — for platforms spanning multiple regions or Kubernetes clusters, extending discovery across these boundaries requires either a federated registry architecture or careful cross-cluster synchronization, adding meaningful complexity beyond a single-cluster registry design.
- **Discovery for external/third-party dependencies** — this design covers INTERNAL service-to-service discovery; calls to external third-party APIs typically use traditional DNS rather than this dynamic registry mechanism, meaning a complete platform architecture needs both discovery patterns coexisting for different categories of dependencies.
- **Bootstrapping problem** — a brand-new service instance needs to know HOW to reach the registry itself before it can discover anything else; this initial "how do I find the registry" bootstrapping typically relies on more traditional, relatively static configuration (e.g., a well-known registry endpoint), representing the one piece of the system that intentionally ISN'T dynamically discovered.
