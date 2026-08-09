# Design a URL Shortener — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Given a long URL, generate a short, unique alias (e.g., `short.ly/aB3xK9`)
- Redirect from short URL to original long URL
- Optional: custom aliases, expiration dates, click analytics
- Prevent collisions — every short code must map to exactly one long URL

### Non-Functional Requirements
- **Read-heavy:** Redirects vastly outnumber URL creations (~100:1 or higher)
- **Low latency redirects:** Should feel instant (< 50ms) — this is on the critical path of every click
- **High availability:** A broken shortener breaks every link that's ever been shared
- **Uniqueness:** No two long URLs should ever collide on the same short code (unless intentionally reused)
- **Scale:** Billions of URLs stored, tens of thousands of redirects/sec at peak

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| New URLs created/day | ~10M |
| Redirects/day | ~1B (100:1 read/write ratio) |
| Redirects/sec (peak) | ~20,000 |
| Short code length | 7 chars, base62 → 62^7 ≈ 3.5 trillion combinations |
| Storage per record | ~500 bytes (short code + long URL + metadata) |
| Total storage (5 years) | ~10M/day × 365 × 5 × 500 bytes ≈ 9TB |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Client["Client<br/>(Browser/App)"]
    LB["Load Balancer"]
    Gateway["API Gateway"]

    subgraph Services["Core Services"]
        ShortenSvc["URL Shortening Service"]
        RedirectSvc["Redirect Service"]
        AnalyticsSvc["Analytics Service"]
    end

    subgraph IDGen["ID Generation"]
        Counter["Distributed Counter /<br/>Snowflake ID Generator"]
    end

    subgraph Storage["Storage Layer"]
        Cache[("Redis Cache<br/>(hot short_code → long_url)")]
        URLDb[("URL Mapping DB<br/>(sharded key-value store)")]
        AnalyticsDb[("Analytics Store<br/>(click events, async)")]
    end

    Kafka["Kafka<br/>(ClickEvent stream)"]

    Client -->|"POST /shorten"| LB --> Gateway --> ShortenSvc
    ShortenSvc --> Counter
    ShortenSvc --> URLDb
    ShortenSvc --> Cache

    Client -->|"GET /aB3xK9"| LB --> Gateway --> RedirectSvc
    RedirectSvc --> Cache
    RedirectSvc --> URLDb
    RedirectSvc --> Kafka
    Kafka --> AnalyticsSvc --> AnalyticsDb
```

**Key idea:** Because redirects are on the hot path and vastly outnumber creations, the redirect flow is optimized to almost always resolve from cache — and click-tracking analytics are pushed off the critical path entirely via an async event stream, so a slow analytics pipeline never adds latency to the redirect itself.

---

## 3. Data Model

```mermaid
erDiagram
    URL_MAPPING {
        string short_code PK
        string long_url
        string user_id "nullable, if authenticated"
        timestamp created_at
        timestamp expires_at "nullable"
        bool is_custom_alias
    }
    CLICK_EVENT {
        string event_id PK
        string short_code FK
        timestamp clicked_at
        string referrer
        string user_agent
        string ip_country
    }
```

---

## 4. Short Code Generation Strategies

```mermaid
flowchart TB
    A["Generate unique short code"] --> B{"Strategy"}

    B --> C["Approach 1: Base62 Encode<br/>a Unique Counter/ID"]
    C --> C1["Get next ID from<br/>distributed counter/Snowflake"]
    C1 --> C2["Encode ID as base62 string<br/>(0-9, a-z, A-Z)"]
    C2 --> C3["Guaranteed unique,<br/>no collision check needed"]

    B --> D["Approach 2: Hash + Truncate"]
    D --> D1["MD5/SHA256 hash of long_url<br/>+ salt/timestamp"]
    D1 --> D2["Take first 7 characters"]
    D2 --> D3["Check for collision in DB<br/>(rare, but must handle)"]
    D3 --> D4{"Collision?"}
    D4 -- Yes --> D5["Add salt, rehash, retry"]
    D4 -- No --> D6["Use this code"]

    B --> E["Approach 3: Pre-generated<br/>Random Code Pool"]
    E --> E1["Background job pre-generates<br/>millions of random unused codes"]
    E1 --> E2["Shortening request simply<br/>pops one from the pool"]
```

*This design uses **Approach 1 (counter + base62)** as primary — it's deterministic, collision-free by construction, and avoids the hash-collision-retry loop entirely.*

---

## 5. Distributed Unique ID Generation

```mermaid
flowchart TB
    A["Need globally unique,<br/>roughly-ordered IDs<br/>at high throughput"] --> B{"Strategy"}
    B --> C["Snowflake-style ID<br/>(timestamp + machine_id + sequence)"]
    B --> D["Range-based allocation<br/>(each server reserves a block<br/>of IDs, e.g., 1M at a time, from a<br/>central counter)"]

    C --> C1["No central bottleneck —<br/>each server generates independently"]
    D --> D1["Central counter only hit<br/>occasionally (per block),<br/>not per-request"]

    E["Chosen approach:<br/>Range-based allocation"] --> F["Shortening Service instance<br/>requests block [1M-2M) from<br/>central counter at startup"]
    F --> G["Serves 1M short-code<br/>requests from local memory<br/>before requesting next block"]
```

**Why range allocation over Snowflake here:** Since we only need uniqueness (not strict time-ordering) and want the shortest possible codes, pre-claiming ID ranges lets each service instance generate short codes locally at zero coordination cost per request, only touching the central counter once per million IDs.

---

## 6. URL Shortening Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client
    participant GW as API Gateway
    participant SS as Shortening Service
    participant IDGen as ID Range Allocator
    participant DB as URL Mapping DB
    participant Cache as Redis Cache

    C->>GW: POST /shorten {long_url, custom_alias?}
    GW->>SS: Forward request

    alt Custom alias requested
        SS->>DB: Check if alias already exists
        alt Alias taken
            DB-->>SS: Exists
            SS-->>C: 409 Conflict — alias unavailable
        else Alias available
            SS->>DB: Insert mapping (custom short_code)
        end
    else Auto-generated short code
        SS->>IDGen: Get next ID from local pre-claimed range
        IDGen-->>SS: Next ID
        SS->>SS: Base62 encode ID → short_code
        SS->>DB: Insert mapping {short_code, long_url}
    end

    DB-->>SS: Success
    SS->>Cache: Pre-populate cache<br/>(new URLs often get clicked quickly)
    SS-->>C: Return short URL
```

---

## 7. Redirect Flow — Detailed Sequence (Hot Path)

```mermaid
sequenceDiagram
    participant C as Client
    participant LB as Load Balancer
    participant RS as Redirect Service
    participant Cache as Redis Cache
    participant DB as URL Mapping DB
    participant K as Kafka

    C->>LB: GET /aB3xK9
    LB->>RS: Route request
    RS->>Cache: GET aB3xK9

    alt Cache hit (common case)
        Cache-->>RS: long_url
    else Cache miss
        RS->>DB: Query short_code = aB3xK9
        DB-->>RS: long_url
        RS->>Cache: SET aB3xK9 = long_url (TTL)
    end

    RS-->>C: HTTP 301/302 redirect to long_url

    RS->>K: Emit ClickEvent (async, fire-and-forget)
    Note over K: Does NOT block the redirect response —<br/>analytics processed asynchronously downstream
```

**301 vs 302 tradeoff:** A 301 (permanent redirect) lets browsers cache the redirect locally, reducing load on the service for repeat clicks — but it also means click analytics for subsequent visits from that browser are lost, since the browser skips the server entirely. A 302 (temporary redirect) ensures every click is tracked, at the cost of higher server load. Most production shorteners use **302** specifically to preserve analytics.

---

## 8. Database Sharding Strategy

```mermaid
flowchart TB
    A["URL Mapping Table<br/>(billions of rows)"] --> B["Shard by hash(short_code)"]
    B --> C["Shard 1<br/>short_codes hashing to range 1"]
    B --> D["Shard 2<br/>short_codes hashing to range 2"]
    B --> E["Shard 3<br/>short_codes hashing to range 3"]

    F["Redirect request for 'aB3xK9'"] --> G["Compute hash(aB3xK9)"]
    G --> H["Route directly to<br/>owning shard"]
```

*Sharding by the short code itself (not the long URL or user) ensures redirect lookups — the hot path — always know exactly which shard to query without a lookup step.*

---

## 9. Analytics Pipeline (Async, Off Critical Path)

```mermaid
flowchart LR
    A["ClickEvent published<br/>to Kafka"] --> B["Stream Processor<br/>(aggregates by short_code,<br/>time window, geography)"]
    B --> C["Analytics Store<br/>(time-series/OLAP DB)"]
    C --> D["Dashboard API<br/>(click counts, referrer breakdown,<br/>geo distribution)"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((URL Shortener HLD))
    Shortening Service
      Custom alias validation
      Short code generation
      DB write + cache pre-population
    Redirect Service
      Cache-first lookup
      Fast HTTP redirect
      Fire-and-forget click event
    ID Range Allocator
      Pre-claims ID blocks
      Near-zero coordination overhead
    URL Mapping DB
      Sharded by short_code
      Optimized for point lookups
    Redis Cache
      Hot short_code to long_url mapping
      Absorbs majority of read traffic
    Analytics Pipeline
      Async click event processing
      Never blocks redirect latency
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Short code generation | Counter + base62 encoding | Collision-free by construction; avoids hash-and-retry complexity |
| ID generation | Range-based pre-allocation per service instance | Minimizes coordination overhead; central counter touched once per million IDs, not per request |
| Redirect status code | 302 (temporary) | Preserves click analytics on every visit, at the cost of slightly higher server load vs 301 |
| Caching strategy | Cache-aside, pre-populate on creation | Redirects are the overwhelming majority of traffic; cache-first keeps the hot path fast |
| Sharding key | `short_code` (not long_url or user_id) | Redirect lookups — the hot path — need direct shard routing without an extra lookup |
| Analytics | Fully async via Kafka | Click tracking must never add latency to the user-facing redirect |

---

## 12. Bottlenecks & Scaling Considerations

- **Redirect service is the dominant load** — with a 100:1+ read/write ratio, nearly all scaling effort goes into the redirect path; cache hit ratio is the single most important performance metric to monitor.
- **Cache capacity vs long-tail URLs** — most traffic concentrates on recently-created/popular links, but billions of rarely-clicked long-tail URLs exist; cache only needs to hold the "hot" working set, not the entire dataset (LRU eviction handles this naturally).
- **Custom alias collisions** — unlike auto-generated codes, custom aliases need an existence check before insert, which is a point of contention if many users race for the same popular word; handle via unique constraint at the DB level as the final arbiter, not just an app-level check.
- **ID range exhaustion coordination** — if a service instance crashes mid-block, some pre-claimed IDs are wasted (never used) — acceptable given the astronomically large ID space (62^7), but worth noting as a deliberate tradeoff.
- **Malicious/spam URL creation** — rate limiting on the shortening endpoint (not the redirect endpoint) is essential to prevent abuse (e.g., mass-generating phishing links).
- **Expired URL cleanup** — URLs with `expires_at` need a background sweep (or lazy deletion on redirect-time check) to avoid serving stale/expired links indefinitely.
- **Global distribution** — redirects benefit from being served close to the user; consider geo-distributed read replicas or edge-cached redirect logic (e.g., at the CDN/edge-worker layer) for the lowest possible latency.
