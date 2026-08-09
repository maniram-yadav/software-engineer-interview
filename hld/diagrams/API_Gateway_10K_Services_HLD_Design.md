# Design an API Gateway Handling Auth, Rate Limiting, and Routing for 10,000+ Backend Services — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Route incoming external requests to the correct one of 10,000+ backend services
- Centrally enforce authentication and authorization before requests reach backend services
- Apply rate limiting per client/API key across potentially thousands of distinct API endpoints
- Support API versioning, request/response transformation, and aggregation of multiple backend calls into one client-facing response

### Non-Functional Requirements
- **Extremely high availability:** The gateway sits in front of EVERY external request to EVERY service — its failure is a total platform outage, similar criticality to the SSO and Global DNS designs
- **Low added latency:** Every single request now has an extra hop through the gateway before reaching its actual destination
- **Configuration scale:** Routing rules for 10,000+ distinct services/endpoints must be manageable, not an unmaintainable flat configuration file
- **Horizontal scalability:** Must handle the platform's ENTIRE external request volume, not just a fraction

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Backend services routed | 10,000+ |
| Requests/sec (platform-wide) | Hundreds of thousands to millions |
| Routing table size | Tens of thousands of distinct route entries |
| Added latency budget | Single-digit milliseconds |

---

## 2. The Core Architectural Challenge — Routing Table Scale

```mermaid
flowchart TB
    A["10,000+ backend services,<br/>each with potentially MULTIPLE<br/>API endpoints/versions"] --> B["Naive approach: a single,<br/>flat configuration file or<br/>database table with every<br/>possible route explicitly<br/>listed"]
    B --> B1["Problem: becomes an<br/>unmanageable, error-prone<br/>bottleneck for CHANGE —<br/>every new service or<br/>endpoint requires editing<br/>this monolithic, shared<br/>configuration, risking<br/>conflicts and slow deployment<br/>cycles across thousands of<br/>independent teams"]

    C["Solution: DECENTRALIZED route<br/>REGISTRATION — each backend<br/>service team registers its<br/>OWN routes independently<br/>(via a self-service API or<br/>declarative config alongside<br/>their own service deployment),<br/>rather than a central team<br/>manually maintaining one<br/>giant routing table"] --> C1["This transforms route<br/>management from a centralized<br/>bottleneck into a<br/>self-service, team-owned<br/>capability — essential at<br/>this scale of service count"]
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    Client["External Clients<br/>(web, mobile, third-party API consumers)"]

    subgraph GatewayCluster["API Gateway Cluster<br/>(horizontally scaled)"]
        Gateway1["Gateway Instance 1"]
        Gateway2["Gateway Instance 2"]
        GatewayN["Gateway Instance N..."]
    end

    subgraph ControlPlane["Gateway Control Plane"]
        RouteRegistry[("Route Registry<br/>— decentralized service registration")]
        AuthConfig[("Auth Policy Store")]
        RateLimitConfig[("Rate Limit Policy Store")]
    end

    subgraph AuthLayer["Auth Services"]
        TokenValidator["Token Validation Service<br/>(connects to SSO/Identity system)"]
    end

    subgraph Backends["10,000+ Backend Services"]
        ServiceA["Service A"]
        ServiceB["Service B"]
        ServiceN["Service N..."]
    end

    Client --> Gateway1
    Client --> Gateway2
    Client --> GatewayN

    Gateway1 --> RouteRegistry
    Gateway1 --> TokenValidator
    Gateway1 --> RateLimitConfig

    Gateway1 -->|"routed request"| ServiceA
    Gateway2 -->|"routed request"| ServiceB
    GatewayN -->|"routed request"| ServiceN

    ServiceA -.->|"self-service<br/>route registration"| RouteRegistry
    ServiceB -.->|"self-service<br/>route registration"| RouteRegistry
```

**Key idea:** Every gateway instance is stateless and horizontally identical, pulling its routing/auth/rate-limit configuration from shared, centrally-managed but DECENTRALLY-POPULATED stores. Backend service teams register their own routes as part of their own deployment process — the gateway's job is to efficiently CONSUME this large, continuously-changing routing dataset, not to be the single point where all route changes must be manually coordinated.

---

## 4. Data Model

```mermaid
erDiagram
    ROUTE {
        string route_id PK
        string path_pattern "e.g. /api/v2/orders/*"
        string backend_service_name
        string backend_endpoint
        string owning_team
        bool auth_required
    }
    RATE_LIMIT_POLICY {
        string policy_id PK
        string applies_to_route FK
        string tier "free/pro/enterprise"
        int requests_per_minute
    }
    API_CLIENT {
        string client_id PK
        string api_key_hash
        string tier
        string owning_organization
    }
```

---

## 5. Request Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Client as External Client
    participant Gateway as Gateway Instance
    participant RouteReg as Route Registry<br/>(cached locally)
    participant Auth as Token Validator
    participant RateLimit as Rate Limiter<br/>(same pattern as the<br/>dedicated Rate Limiter design)
    participant Backend as Backend Service

    Client->>Gateway: GET /api/v2/orders/123<br/>Authorization: Bearer <token>

    Gateway->>RouteReg: Match path to route<br/>(cached, low-latency lookup —<br/>NOT a network call per request)
    RouteReg-->>Gateway: Matched: Order Service,<br/>auth_required=true

    Gateway->>Auth: Validate token
    Auth-->>Gateway: Valid, client_id=X, tier=pro

    Gateway->>RateLimit: Check rate limit<br/>for client_id=X on this route
    alt Within limit
        RateLimit-->>Gateway: Allowed
        Gateway->>Backend: Forward request<br/>(with client identity<br/>attached as trusted headers)
        Backend-->>Gateway: Response
        Gateway-->>Client: Return response
    else Rate limit exceeded
        RateLimit-->>Gateway: Denied
        Gateway-->>Client: 429 Too Many Requests
    end
```

---

## 6. Efficient Route Matching at Scale

```mermaid
flowchart TB
    A["10,000+ services, each with<br/>multiple path patterns —<br/>how does the gateway match<br/>an incoming path QUICKLY?"] --> B{"Route Matching Strategy"}

    B --> C["Naive: linear scan through<br/>ALL route patterns checking<br/>for a match"]
    C --> C1["O(N) per request — becomes<br/>a genuine bottleneck at<br/>tens of thousands of routes<br/>and high request volume"]

    B --> D["Trie-based path matching<br/>(same fundamental structure<br/>as the Autocomplete design's<br/>prefix trie, applied to<br/>URL path segments instead<br/>of search query prefixes)"]
    D --> D1["Path segments form trie<br/>nodes: /api → /v2 → /orders<br/>→ {id} — matching becomes<br/>O(path depth), NOT O(total<br/>route count) — dramatically<br/>faster at scale"]

    E["This design uses a trie-based<br/>router, periodically rebuilt<br/>from the Route Registry's<br/>current state, cached LOCALLY<br/>in each gateway instance's<br/>memory for zero-network-call<br/>routing decisions"] -.-> D1
```

---

## 7. Self-Service Route Registration Flow

```mermaid
sequenceDiagram
    participant Team as Backend Service Team
    participant CI as CI/CD Pipeline
    participant RouteAPI as Route Registration API
    participant Registry as Route Registry
    participant GatewayInstances as All Gateway Instances

    Team->>CI: Deploy new service version<br/>(includes route definition<br/>as declarative config,<br/>e.g., in the service's<br/>own repo)

    CI->>RouteAPI: Register route:<br/>{path: /api/v2/inventory/*,<br/>service: inventory-service,<br/>auth_required: true}

    RouteAPI->>RouteAPI: Validate: no conflicting<br/>route already claimed by<br/>a DIFFERENT team<br/>(prevents accidental<br/>route hijacking)

    RouteAPI->>Registry: Store new route

    Registry-->>GatewayInstances: Propagate update<br/>(via periodic poll or<br/>push-based config<br/>distribution — same<br/>propagation pattern as<br/>the Service Mesh design's<br/>control plane pushing to<br/>sidecars)

    Note over GatewayInstances: New route is now live<br/>across ALL gateway instances,<br/>WITHOUT any gateway<br/>redeployment needed —<br/>purely a configuration<br/>propagation
```

**Why route conflict validation matters:** With thousands of independently-deploying teams, without an explicit check, two different teams could accidentally claim overlapping path patterns — the registration API must actively prevent this at registration time, since silently allowing "last write wins" on a route conflict could cause one team's requests to be misrouted to another team's service, a serious and confusing production issue.

---

## 8. Request Aggregation (Backend-for-Frontend Pattern)

```mermaid
flowchart TB
    A["Mobile client needs data<br/>from THREE different backend<br/>services to render one screen<br/>(user profile, recent orders,<br/>recommendations)"] --> B{"Without aggregation"}
    B --> B1["Client makes 3 SEPARATE<br/>API calls — 3x round-trip<br/>latency, especially painful<br/>on mobile networks"]

    A --> C{"With gateway-level<br/>aggregation"}
    C --> D["Gateway defines a COMPOSITE<br/>route that internally fans<br/>out to all 3 backend services<br/>in PARALLEL, then combines<br/>their responses into ONE<br/>client-facing response"]
    D --> D1["Client makes ONE call,<br/>gateway absorbs the<br/>internal fan-out complexity —<br/>this is the Backend-for-Frontend<br/>(BFF) pattern implemented<br/>at the gateway layer"]
```

```mermaid
sequenceDiagram
    participant Client as Mobile Client
    participant Gateway as API Gateway
    participant Profile as Profile Service
    participant Orders as Orders Service
    participant Reco as Recommendation Service

    Client->>Gateway: GET /mobile/home-screen

    par Parallel backend fan-out
        Gateway->>Profile: Get user profile
        Profile-->>Gateway: Profile data
    and
        Gateway->>Orders: Get recent orders
        Orders-->>Gateway: Order data
    and
        Gateway->>Reco: Get recommendations
        Reco-->>Gateway: Recommendation data
    end

    Gateway->>Gateway: Combine into ONE<br/>composite response
    Gateway-->>Client: Single aggregated response
```

---

## 9. API Versioning Support

```mermaid
flowchart TB
    A["Backend service evolves,<br/>introduces breaking changes<br/>in a new API version"] --> B["Route Registry supports<br/>MULTIPLE simultaneous versions<br/>of the same logical route:<br/>/api/v1/orders AND<br/>/api/v2/orders, potentially<br/>pointing to DIFFERENT backend<br/>service versions"]

    B --> C["Gateway routes each version<br/>independently — existing<br/>clients on v1 continue<br/>working UNCHANGED while new<br/>clients adopt v2, giving<br/>backend teams a genuine<br/>MIGRATION WINDOW rather than<br/>requiring an instantaneous,<br/>synchronized cutover across<br/>every client"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((API Gateway HLD))
    Gateway Instances
      Stateless, horizontally scaled
      Local route cache for speed
    Route Registry
      Decentralized self-service registration
      Conflict prevention
    Trie-Based Router
      O(path depth) matching
      Scales to tens of thousands of routes
    Token Validator
      Centralized auth enforcement
      Connects to identity/SSO system
    Rate Limiter
      Per-client, per-route policies
      Same core mechanism as dedicated Rate Limiter design
    Aggregation Layer
      Backend-for-Frontend pattern
      Parallel fan-out and merge
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Route management model | Decentralized, self-service registration | A centrally-maintained flat routing table becomes an unmanageable bottleneck at 10,000+ services across many independent teams |
| Route matching algorithm | Trie-based path matching | Scales as O(path depth) rather than O(total route count), essential for maintaining low latency at this route table scale |
| Gateway instance state | Stateless, locally-cached configuration | Enables horizontal scaling and avoids making every request dependent on a network call to a central config store |
| Auth enforcement | Centralized at the gateway, before backend routing | Ensures consistent authentication/authorization across all 10,000+ services without requiring each to reimplement it independently |
| Aggregation | Optional Backend-for-Frontend composite routes | Reduces client-side round-trips for screens/use cases needing data from multiple backend services |
| Versioning | Multiple simultaneous route versions supported | Gives backend teams a genuine migration window rather than forcing synchronized, all-at-once client cutovers |

---

## 12. Bottlenecks & Scaling Considerations

- **Configuration propagation latency** — with decentralized, frequent route registration across thousands of teams, the delay between "team registers a new route" and "all gateway instances have it live" directly impacts deployment velocity; needs an efficient, low-latency propagation mechanism (push-based, not slow polling) at this scale.
- **Local route cache memory footprint** — caching tens of thousands of routes in EVERY gateway instance's memory (for zero-network-call matching) requires the trie structure to remain memory-efficient even as route count continues growing.
- **Auth service as a critical shared dependency** — since EVERY authenticated request across ALL 10,000+ services depends on token validation, this component's availability and latency directly bound the entire gateway's performance, similar criticality to the SSO design's session store.
- **Backend service ownership vs gateway team responsibility split** — decentralized route registration empowers teams but also means the central gateway team has LESS direct visibility/control over what's actually routed through their infrastructure; requires monitoring/governance tooling to maintain platform-wide observability despite the decentralized registration model.
- **Aggregation failure handling** — when a Backend-for-Frontend composite route fans out to multiple services and ONE of them fails/times out, the gateway needs clear policy: fail the whole aggregated response, or return partial results with the failed section omitted — this must be a deliberate, documented per-route decision, not an accidental inconsistency.
- **Gateway as a total-platform single point of failure** — given that gateway failure blocks essentially ALL external traffic to ALL services, this component warrants exceptional investment in high availability, redundancy across failure domains, and graceful degradation strategies — commensurate with its position as the single mandatory entry point for the entire platform's external-facing traffic.
- **Route conflict resolution edge cases** — beyond simple exact-path conflicts, overlapping WILDCARD patterns from different teams (e.g., `/api/v2/orders/*` vs `/api/v2/orders/special/*`) require careful precedence rules to avoid ambiguous routing behavior, adding meaningful complexity to the route registration validation logic beyond simple duplicate detection.
