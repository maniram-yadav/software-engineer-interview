# Design a Load Balancer From Scratch — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Distribute incoming traffic across a pool of backend servers according to a configurable algorithm
- Continuously health-check backend servers, automatically removing unhealthy ones from rotation
- Support both Layer 4 (TCP/transport-level) and Layer 7 (HTTP/application-level) load balancing
- Support session affinity ("sticky sessions") where a client's requests consistently reach the same backend

### Non-Functional Requirements
- **Extremely high throughput:** The load balancer sits in the critical path of essentially ALL platform traffic — must handle very high connection/request rates
- **Minimal added latency:** Every request now passes through this extra hop before reaching its actual destination
- **High availability:** A load balancer failure can take down access to an entire pool of otherwise-healthy backend servers
- **Fair, efficient distribution:** Backend load should be genuinely balanced, not accidentally skewed toward particular servers

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Connections/sec (large platform) | Hundreds of thousands to millions |
| Backend servers per pool | Tens to hundreds |
| Health check interval | Few seconds |
| Added latency budget | Sub-millisecond to low single-digit ms |

---

## 2. Layer 4 vs Layer 7 Load Balancing — The Fundamental Architectural Choice

```mermaid
flowchart TB
    A["Layer 4 (Transport Layer)<br/>Load Balancing"] --> A1["Operates on TCP/UDP<br/>connections — makes routing<br/>decisions based ONLY on<br/>IP address + port, WITHOUT<br/>inspecting the actual<br/>application-layer content<br/>(e.g., HTTP headers, URL path)"]
    A1 --> A2["PRO: extremely fast — minimal<br/>processing per connection,<br/>can often just forward raw<br/>packets<br/>CON: can't make routing<br/>decisions based on request<br/>CONTENT (e.g., 'route /api/*<br/>to this pool')"]

    B["Layer 7 (Application Layer)<br/>Load Balancing"] --> B1["Terminates the actual HTTP<br/>connection, inspects request<br/>content (headers, path, cookies)<br/>to make INTELLIGENT routing<br/>decisions, then establishes<br/>a SEPARATE connection to<br/>the chosen backend"]
    B1 --> B2["PRO: enables content-based<br/>routing, request modification,<br/>more sophisticated health/<br/>session logic<br/>CON: more processing overhead<br/>per request — must fully<br/>parse the application protocol"]

    C["Most production systems use<br/>a HYBRID: L4 load balancing<br/>at the outer edge (fast,<br/>high-volume initial<br/>distribution) with L7 load<br/>balancing at an inner layer<br/>(for services needing<br/>content-aware routing) —<br/>similar to the tiered<br/>approach in the Multi-Layer<br/>CDN design"] --> B2
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    Client["Clients"]

    subgraph L4Tier["Layer 4 Load Balancing<br/>(edge, high-throughput)"]
        L4LB["L4 Load Balancer<br/>(connection-level distribution)"]
    end

    subgraph L7Tier["Layer 7 Load Balancing<br/>(content-aware)"]
        L7LB1["L7 LB Instance 1"]
        L7LB2["L7 LB Instance 2"]
    end

    subgraph HealthSystem["Health Checking"]
        HealthChecker["Health Check Workers"]
        HealthState[("Backend Health State")]
    end

    subgraph Backends["Backend Server Pool"]
        Server1["Server 1"]
        Server2["Server 2"]
        Server3["Server 3"]
    end

    Client --> L4LB
    L4LB --> L7LB1
    L4LB --> L7LB2

    L7LB1 --> HealthState
    L7LB2 --> HealthState
    HealthChecker --> Server1
    HealthChecker --> Server2
    HealthChecker --> Server3
    HealthChecker --> HealthState

    L7LB1 -->|"content-aware<br/>routing"| Server1
    L7LB1 --> Server2
    L7LB2 --> Server2
    L7LB2 --> Server3
```

---

## 4. Data Model

```mermaid
erDiagram
    BACKEND_SERVER {
        string server_id PK
        string ip_address
        int port
        string status "healthy/unhealthy/draining"
        int current_connections
        float avg_response_time_ms
    }
    LOAD_BALANCING_POOL {
        string pool_id PK
        string algorithm "round_robin/least_connections/weighted"
        list member_server_ids
    }
    SESSION_AFFINITY_ENTRY {
        string client_identifier PK "e.g., cookie value or IP"
        string assigned_server_id
        timestamp last_seen_at
    }
```

---

## 5. Load Balancing Algorithms

```mermaid
flowchart TB
    A["Load Balancing Algorithm<br/>Choice"] --> B["Round Robin"]
    A --> C["Least Connections"]
    A --> D["Weighted Round Robin"]
    A --> E["Consistent Hashing<br/>(for session affinity)"]

    B --> B1["Simple: cycle through<br/>servers in fixed order<br/>CON: doesn't account for<br/>servers having genuinely<br/>DIFFERENT current load<br/>(e.g., some requests are<br/>much more expensive than<br/>others)"]

    C --> C1["Route to whichever backend<br/>CURRENTLY has the fewest<br/>active connections<br/>PRO: naturally adapts to<br/>uneven request processing<br/>times — a good general-purpose<br/>default"]

    D --> D1["Like round robin, but<br/>servers with more capacity<br/>(e.g., bigger instances)<br/>receive proportionally MORE<br/>traffic — accounts for<br/>heterogeneous server pools"]

    E --> E1["Same consistent hashing<br/>principle as the Distributed<br/>Cache design — ensures the<br/>SAME client consistently<br/>maps to the SAME backend,<br/>useful for session affinity<br/>without needing a separate<br/>lookup table"]
```

---

## 6. Health Checking Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Checker as Health Check Worker
    participant Server as Backend Server
    participant HealthState as Health State Store
    participant LB as Load Balancer

    loop Every health check interval
        Checker->>Server: Health check<br/>(TCP connect for L4,<br/>or HTTP GET /health for L7)

        alt Passes
            Server-->>Checker: Healthy response
            Checker->>HealthState: Mark HEALTHY
        else Fails
            Checker->>HealthState: Record failure<br/>(requires MULTIPLE consecutive<br/>failures before marking<br/>unhealthy — same false-positive<br/>protection principle as the<br/>Global DNS design's health<br/>checking)
        end
    end

    LB->>HealthState: Query current healthy<br/>server list (cached,<br/>refreshed frequently)
    HealthState-->>LB: [server1, server3]<br/>(server2 excluded — failed<br/>health checks)
```

---

## 7. Request Routing Flow (Layer 7) — Detailed Sequence

```mermaid
sequenceDiagram
    participant Client as Client
    participant L7LB as L7 Load Balancer
    participant HealthState as Health State
    participant Server as Selected Backend Server

    Client->>L7LB: HTTP request<br/>(TCP connection terminates HERE)

    L7LB->>L7LB: Parse HTTP request<br/>(headers, path, method)

    L7LB->>HealthState: Get current healthy<br/>servers in target pool
    HealthState-->>L7LB: [server1, server3]

    L7LB->>L7LB: Apply load balancing<br/>algorithm (e.g., least<br/>connections) among<br/>healthy servers only

    L7LB->>Server: Open NEW connection<br/>to chosen backend,<br/>forward request

    Server-->>L7LB: Response
    L7LB-->>Client: Forward response<br/>over the ORIGINAL client<br/>connection
```

**Why L7 load balancing requires TWO separate TCP connections:** Because the load balancer must fully receive and parse the incoming request BEFORE it can make a content-aware routing decision, it necessarily terminates the client's connection first, then establishes an entirely separate connection to the chosen backend — this is fundamentally different from L4 balancing, which can often just forward packets along a single logical flow without this connection-splitting overhead.

---

## 8. Session Affinity ("Sticky Sessions")

```mermaid
flowchart TB
    A["Some applications require<br/>a given client's requests to<br/>consistently reach the SAME<br/>backend server (e.g., an<br/>application holding in-memory<br/>session state, not using a<br/>shared session store)"] --> B{"Affinity Strategy"}

    B --> C["Cookie-based: LB sets a<br/>cookie identifying which<br/>backend served the FIRST<br/>request; subsequent requests<br/>with that cookie route to<br/>the SAME backend"]
    B --> D["Consistent hashing on<br/>client IP: same client IP<br/>always hashes to the same<br/>backend, without needing<br/>to track/store explicit<br/>session state"]

    E["Important caveat: session<br/>affinity works AGAINST even<br/>load distribution — a backend<br/>handling many 'sticky'<br/>long-lived sessions can<br/>become disproportionately<br/>loaded. Modern architectures<br/>generally PREFER stateless<br/>backends with a SHARED<br/>session store (e.g., Redis)<br/>over relying on sticky<br/>sessions, precisely to avoid<br/>this tension"] -.-> D
```

---

## 9. Handling Backend Server Failure Mid-Request

```mermaid
sequenceDiagram
    participant Client as Client
    participant LB as Load Balancer
    participant Server1 as Backend Server 1<br/>(fails mid-request)
    participant Server2 as Backend Server 2

    Client->>LB: Request
    LB->>Server1: Forward to Server 1

    Note over Server1: Server crashes or<br/>connection drops<br/>mid-response

    Server1--xLB: Connection lost

    LB->>LB: Detect failure<br/>(connection reset/timeout)
    LB->>HealthState: Immediately mark<br/>Server 1 as unhealthy<br/>(faster than waiting for<br/>the next scheduled health<br/>check cycle)

    alt Request is safely retryable<br/>(e.g., idempotent GET,<br/>no partial response<br/>sent to client yet)
        LB->>Server2: Retry against a<br/>DIFFERENT healthy server
        Server2-->>LB: Response
        LB-->>Client: Return response<br/>(client never saw the failure)
    else Not safely retryable<br/>(e.g., partial response<br/>already streamed, or a<br/>non-idempotent write)
        LB-->>Client: Return error —<br/>retry decision left to<br/>client, since blindly<br/>retrying could cause<br/>duplicate side effects
    end
```

**Why retry safety depends on idempotency, connecting to earlier designs:** This is the exact same principle explored in depth in the Idempotent API Requests design — a load balancer can only safely retry a failed request against a different backend if doing so can't cause harmful duplicate effects; blindly retrying a non-idempotent write operation risks the same double-processing danger covered extensively in that design.

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Load Balancer HLD))
    L4 Load Balancer
      Connection-level distribution
      Fast, minimal processing
    L7 Load Balancer
      Content-aware routing
      Terminates and re-establishes connections
    Health Checker
      Continuous backend monitoring
      Multiple-failure threshold
    Health State Store
      Current healthy server list
      Cached, low-latency reads
    Load Balancing Algorithm
      Round robin, least connections, weighted, consistent hash
      Applied only across healthy servers
    Session Affinity Layer
      Cookie or hash-based stickiness
      Tension with even load distribution
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Architecture tier | Hybrid L4 (edge) + L7 (content-aware inner tier) | L4 provides maximum throughput for initial high-volume distribution; L7 enables intelligent routing where genuinely needed, without paying its overhead everywhere |
| Default algorithm | Least connections | Naturally adapts to servers with genuinely different current load, unlike round robin's assumption that all requests are equally expensive |
| Health check failure threshold | Multiple consecutive failures required | Prevents false-positive removal of a healthy server due to a single transient blip, same principle as the Global DNS design's health checking |
| Session affinity | Supported, but with explicit tradeoff acknowledgment | Sticky sessions create genuine tension with even load distribution; modern designs generally prefer shared session stores where feasible |
| Failure handling | Immediate unhealthy marking + conditional retry based on idempotency | Faster failure detection than waiting for the next health check cycle; retry safety must respect the same idempotency principles as any distributed system |

---

## 12. Bottlenecks & Scaling Considerations

- **L7 processing overhead at very high request volume** — fully parsing and re-establishing connections for every request is meaningfully more expensive than L4's largely pass-through approach; at extreme scale, this argues for pushing as much traffic as possible to the faster L4 tier, reserving L7 specifically for routes that genuinely need content-aware decisions.
- **Health check storm at large backend pool sizes** — checking hundreds of backend servers frequently generates meaningful aggregate health-check traffic; needs the checker fleet to scale proportionally with backend pool size, similar to the same concern noted in the Global DNS design.
- **Load balancer itself needing to be highly available** — a single load balancer instance is a single point of failure for its entire backend pool; production deployments require the load balancer layer itself to be redundant (often via additional upstream L4 balancing or DNS-based failover), avoiding simply moving the single-point-of-failure problem down one layer rather than solving it.
- **Connection draining during backend deployment** — when intentionally taking a backend server out of rotation (e.g., for a deployment), abruptly cutting its connections mid-request causes user-visible errors; needs a "draining" state where the server stops receiving NEW connections but existing ones are allowed to complete gracefully before final removal.
- **Uneven load from session affinity** — as noted, sticky sessions can create hot backends; monitoring per-backend load distribution specifically when affinity is enabled is important to catch this skew before it becomes a genuine capacity problem.
- **TLS termination overhead** — if the load balancer also handles TLS termination (common for L7 balancers), the cryptographic handshake and encryption/decryption work adds meaningful CPU cost per connection; often mitigated via connection reuse/keep-alive and, at extreme scale, dedicated hardware acceleration for cryptographic operations.
- **Geographic/multi-region load balancing coordination** — for a globally distributed backend pool, this single-region load balancer design needs to interoperate with the geo-routing capabilities covered in the Global DNS design — DNS-level routing directs users to the correct REGIONAL load balancer, which then handles the finer-grained distribution within that region, forming a coordinated two-tier system rather than either mechanism operating in isolation.
