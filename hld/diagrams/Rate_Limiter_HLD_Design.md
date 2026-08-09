# Design a Rate Limiter — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Limit the number of requests a client (user/IP/API key) can make within a time window
- Support multiple limit tiers (e.g., free tier: 100 req/min, paid tier: 10,000 req/min)
- Reject requests exceeding the limit with a clear error (HTTP 429) and retry-after hint
- Support different rate-limiting rules per API endpoint

### Non-Functional Requirements
- **Low latency:** Rate limit check must add negligible overhead (< 1-2ms) to every request
- **Accuracy vs performance tradeoff:** Perfectly precise limiting isn't required; approximate correctness at high throughput is acceptable
- **Distributed correctness:** Must work correctly across many API server instances, not just per-instance
- **High availability:** Rate limiter failure should not take down the whole API (fail open or fail closed, chosen deliberately)
- **Scale:** Must handle the same request volume as the API itself — potentially millions of checks/sec

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| API requests/sec (platform-wide) | ~1M |
| Rate limit checks/sec | ~1M (one per request) |
| Unique clients tracked | ~10M+ active API keys/users |
| Latency budget per check | < 1-2ms |
| Storage per client (counter state) | ~50-100 bytes |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Client["API Client"]
    LB["Load Balancer"]

    subgraph Gateway["API Gateway Layer"]
        GW1["Gateway Instance 1"]
        GW2["Gateway Instance 2"]
        GW3["Gateway Instance 3"]
    end

    subgraph RateLimiter["Rate Limiter"]
        RLMiddleware["Rate Limit Middleware<br/>(embedded in each gateway instance)"]
        ConfigSvc["Rate Limit Config Service<br/>(rules per tier/endpoint)"]
    end

    subgraph Storage["Shared State Store"]
        Redis[("Redis Cluster<br/>(counters, sliding window state)")]
    end

    Backend["Backend Services"]

    Client --> LB
    LB --> GW1
    LB --> GW2
    LB --> GW3

    GW1 --> RLMiddleware
    GW2 --> RLMiddleware
    GW3 --> RLMiddleware

    RLMiddleware --> Redis
    RLMiddleware --> ConfigSvc

    RLMiddleware -->|"Allowed"| Backend
    RLMiddleware -->|"Denied"| Client
```

**Key idea:** Because API requests hit many different gateway instances (behind a load balancer), rate limit *state* (counters) must live in a **shared store** (Redis) rather than in-process memory — otherwise each instance would enforce the limit independently, letting a client effectively get N× the intended limit by spreading requests across N gateway instances.

---

## 3. Rate Limiting Algorithms

```mermaid
flowchart TB
    A["Rate Limiting Algorithm"] --> B["Fixed Window Counter"]
    A --> C["Sliding Window Log"]
    A --> D["Sliding Window Counter"]
    A --> E["Token Bucket"]
    A --> F["Leaky Bucket"]

    B --> B1["Simple counter per fixed<br/>time window (e.g., per minute)"]
    B --> B2["Problem: burst at window boundary<br/>can allow 2x limit"]

    C --> C1["Store timestamp of every request"]
    C --> C2["Perfectly accurate, but<br/>high memory cost"]

    D --> D1["Weighted average of current<br/>+ previous window counters"]
    D --> D2["Good accuracy, low memory —<br/>most common production choice"]

    E --> E1["Bucket holds tokens,<br/>refilled at fixed rate"]
    E --> E2["Allows controlled bursts<br/>up to bucket capacity"]

    F --> F1["Requests processed at<br/>fixed output rate"]
    F --> F2["Smooths bursts completely,<br/>no burst allowance"]
```

### Fixed Window Boundary Problem (Why Naive Approach Fails)

```mermaid
flowchart LR
    A["Limit: 100 req/min"] --> B["Window 1: 00:00-00:59<br/>99 requests at 00:59"]
    B --> C["Window 2: 01:00-01:59<br/>99 requests at 01:00"]
    C --> D["Result: 198 requests<br/>in a 2-second span<br/>— limit effectively bypassed"]
```

*This is why production systems almost always use **sliding window counter** or **token bucket** instead of naive fixed windows.*

---

## 4. Token Bucket Algorithm — Detailed Mechanics

```mermaid
flowchart TB
    A["Bucket has capacity C<br/>(e.g., 100 tokens)"] --> B["Tokens refill at rate R<br/>(e.g., 10 tokens/sec)"]
    B --> C["Request arrives"]
    C --> D{"Bucket has<br/>>= 1 token?"}
    D -- Yes --> E["Consume 1 token<br/>Allow request"]
    D -- No --> F["Deny request<br/>Return 429 + Retry-After"]
    E --> G["Bucket refills gradually<br/>up to capacity C over time"]
```

**Why token bucket is the most common choice:** It naturally allows short bursts (a client that's been idle can suddenly send up to `capacity` requests at once) while still enforcing a strict long-term average rate — closely matching how real client traffic behaves (bursty, not perfectly uniform).

---

## 5. Distributed Token Bucket — Redis Implementation

```mermaid
sequenceDiagram
    participant C as Client
    participant GW as Gateway/Rate Limit Middleware
    participant Redis as Redis (Lua script, atomic)

    C->>GW: API request (client_id: X)
    GW->>Redis: EVALSHA rate_limit_script<br/>KEYS[client:X] ARGV[capacity, refill_rate, now]

    Note over Redis: Atomic Lua script:<br/>1. Fetch current tokens + last_refill_time<br/>2. Calculate tokens to add based on elapsed time<br/>3. Cap at bucket capacity<br/>4. If tokens >= 1: decrement, return ALLOW<br/>5. Else: return DENY

    Redis-->>GW: ALLOW (tokens_remaining: 42)
    GW-->>C: Forward request to backend<br/>+ headers: X-RateLimit-Remaining: 42

    Note over C,Redis: Later request when bucket empty
    C->>GW: API request (client_id: X)
    GW->>Redis: Same Lua script
    Redis-->>GW: DENY (retry_after: 3s)
    GW-->>C: 429 Too Many Requests<br/>Retry-After: 3
```

**Why a Lua script (not separate GET+SET calls):** Rate limit checks must be atomic — if a gateway reads the counter, then writes an update, a concurrent request from another gateway instance could race in between and both requests could be allowed when only one should be. A single atomic Redis Lua script (or `MULTI`/`EXEC` transaction) eliminates this race condition entirely.

---

## 6. Sliding Window Counter (Alternative Approach)

```mermaid
flowchart TB
    A["Current time falls<br/>60% into current window"] --> B["Weighted count =<br/>(previous_window_count × 0.4)<br/>+ (current_window_count × 1.0)"]
    B --> C{"Weighted count<br/>&gt; limit?"}
    C -- Yes --> D["Deny request"]
    C -- No --> E["Allow request,<br/>increment current_window_count"]
```

*This smooths out the boundary-burst problem of fixed windows with much less memory than storing every individual request timestamp (sliding window log).*

---

## 7. Multi-Tier Rate Limiting Rules

```mermaid
flowchart TB
    A["Request arrives<br/>with API key"] --> B["Rate Limit Middleware"]
    B --> C["Fetch client's tier from<br/>Config Service<br/>(cached, refreshed periodically)"]
    C --> D{"Tier"}
    D -- Free --> E["100 req/min<br/>burst capacity: 20"]
    D -- Pro --> F["10,000 req/min<br/>burst capacity: 500"]
    D -- Enterprise --> G["Custom negotiated limits<br/>possibly no limit"]

    E & F & G --> H["Apply tier-specific<br/>token bucket parameters"]
    H --> I["Also check<br/>per-endpoint overrides<br/>(e.g., /search: stricter limit)"]
```

---

## 8. Rate Limiter Placement Options

```mermaid
flowchart TB
    A["Where does rate limiting happen?"] --> B["Option 1: Client-side<br/>(SDK enforces limits)"]
    A --> C["Option 2: API Gateway<br/>(centralized middleware)"]
    A --> D["Option 3: Dedicated Rate<br/>Limiting Service (sidecar/standalone)"]
    A --> E["Option 4: CDN/Edge<br/>(e.g., Cloudflare, before origin)"]

    B --> B1["Easy to bypass —<br/>never trust client-side alone"]
    C --> C1["Good balance — centralized,<br/>enforced before hitting backend"]
    D --> D1["Reusable across many services,<br/>but adds network hop"]
    E --> E1["Blocks abusive traffic earliest,<br/>reduces load on origin infra entirely"]

    F["Production systems often<br/>layer multiple:<br/>Edge (coarse) + Gateway (precise)"]
```

---

## 9. Handling Rate Limiter Failure (Fail Open vs Fail Closed)

```mermaid
flowchart TB
    A["Redis/rate limit store<br/>becomes unavailable"] --> B{"Failure Policy"}
    B --> C["Fail Open<br/>Allow all requests through"]
    B --> D["Fail Closed<br/>Deny all requests"]

    C --> C1["Risk: no protection against<br/>abuse during the outage"]
    C --> C2["Benefit: API stays available —<br/>usually the right choice for<br/>general-purpose APIs"]

    D --> D1["Risk: full API outage<br/>caused by rate limiter failure alone"]
    D --> D2["Benefit: appropriate only for<br/>security-critical endpoints<br/>(e.g., login attempt limiting)"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Rate Limiter HLD))
    Rate Limit Middleware
      Embedded in gateway instances
      Calls shared store atomically
      Adds response headers
    Shared State Store
      Redis cluster
      Atomic Lua script execution
      Low-latency counter reads/writes
    Config Service
      Tier definitions
      Per-endpoint overrides
      Cached at gateway for speed
    Algorithm Layer
      Token bucket (default)
      Sliding window counter (alternative)
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Algorithm | Token bucket | Naturally allows short bursts while enforcing long-term average rate; matches real traffic patterns |
| State storage | Shared Redis, not per-instance memory | Multiple gateway instances must share a consistent view of each client's usage |
| Atomicity | Lua script / MULTI-EXEC in Redis | Prevents race conditions between concurrent requests from different gateway instances |
| Placement | Gateway-level (with optional edge layer) | Centralizes enforcement before backend load, while edge layer can pre-filter obvious abuse even earlier |
| Failure policy | Fail open (context-dependent) | Prioritizes API availability for general endpoints; security-critical endpoints may choose fail-closed instead |
| Precision | Approximate (sliding window/token bucket), not perfectly exact | Perfect accuracy isn't worth the memory/latency cost at this scale; slight over/under-allowance is acceptable |

---

## 12. Bottlenecks & Scaling Considerations

- **Redis as a single shared dependency** — every single API request now depends on Redis latency/availability; must be a dedicated, highly-available Redis cluster (not shared with unrelated workloads) with sub-millisecond response times.
- **Hot client keys** — a single very high-traffic API key can create a hot key in Redis; consider local pre-approval caching (e.g., gateway locally allows the first few requests optimistically, reconciling with Redis periodically) for extreme-scale single-client traffic.
- **Config propagation lag** — tier/limit changes (e.g., a customer upgrades their plan) need to propagate to all gateway instances; typically handled via short-TTL caching or a pub/sub invalidation signal rather than a fully synchronous lookup per request.
- **Clock skew across distributed nodes** — sliding window and token bucket calculations depend on timestamps; using Redis server time (not client/gateway local clocks) for all calculations avoids skew-related inconsistencies.
- **Global vs regional rate limiting** — for globally distributed APIs, a client's requests may hit gateways in different regions; requires either a globally-replicated counter store (added latency) or accepting slightly relaxed per-region limits as a practical tradeoff.
- **Distinguishing abuse from legitimate bursts** — overly strict limiting frustrates legitimate bursty clients (e.g., a mobile app syncing after being offline); tuning burst capacity separately from sustained rate is key to good UX.
