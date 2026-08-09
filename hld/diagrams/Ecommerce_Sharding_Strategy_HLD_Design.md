# Design a Database Sharding Strategy for a Rapidly Growing E-commerce Platform — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Distribute a rapidly growing dataset (orders, products, users) across multiple database instances
- Support efficient point lookups (e.g., "get order by order_id") and common range/filter queries
- Support resharding (adding more shards) as data volume grows, WITHOUT downtime
- Handle cross-shard queries/joins where genuinely necessary (e.g., admin reporting)

### Non-Functional Requirements
- **Growth accommodation:** The platform is "rapidly growing" — the sharding scheme must not require a full redesign every time the dataset doubles
- **Availability during resharding:** Migration of data between shards must not take the platform offline
- **Balanced load:** Avoid hot shards — both storage and query load should distribute evenly
- **Minimal cross-shard operations:** Most queries should be satisfiable from a single shard, since cross-shard operations are inherently slower and more complex

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Orders/day (current) | ~1M, growing 3x year-over-year |
| Orders table size (2 years out) | Hundreds of millions of rows |
| Peak writes/sec | ~10,000 (flash sales) |
| Target shard count (initial) | 16-32 shards, designed to grow |

---

## 2. Choosing a Shard Key — The Most Important Decision

```mermaid
flowchart TB
    A["Candidate Shard Keys<br/>for an E-commerce Orders table"] --> B["user_id"]
    A --> C["order_id"]
    A --> D["region/geography"]

    B --> B1["PRO: all of a user's orders<br/>land on one shard —<br/>'get my order history'<br/>is a fast, single-shard query"]
    B --> B2["CON: risk of hot shards if<br/>user activity is skewed<br/>(e.g., a few power-seller<br/>accounts with huge order volume)"]

    C --> C1["PRO: naturally distributes<br/>evenly if IDs are<br/>randomly/uniformly generated"]
    C --> C2["CON: 'get all orders for<br/>this user' now requires<br/>a cross-shard scatter-gather<br/>query — a very common<br/>access pattern made expensive"]

    D --> D1["PRO: natural data locality<br/>for region-specific<br/>compliance/latency needs"]
    D --> D2["CON: uneven shard sizes<br/>if user distribution across<br/>regions is skewed (e.g.,<br/>US region much larger<br/>than others)"]

    E["CHOSEN: user_id —<br/>because 'orders by user' is<br/>the dominant, highest-frequency<br/>query pattern; optimize the<br/>sharding scheme for the<br/>MOST COMMON access pattern,<br/>and accept cross-shard cost<br/>for the rarer ones (e.g.,<br/>admin-side 'all orders today')"]
```

**Why shard key choice is the single most consequential decision:** Every subsequent design decision — how data distributes, which queries stay single-shard vs become cross-shard, how resharding works — flows directly from this choice. Getting it wrong is expensive to fix later, since it typically requires a full data migration to correct.

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    App["Application Servers"]

    subgraph Routing["Shard Routing Layer"]
        ShardRouter["Shard Router / Middleware"]
        ShardMap[("Shard Map<br/>key_range → shard_location")]
    end

    subgraph Shards["Database Shards"]
        Shard1[("Shard 1<br/>user_id hash range A")]
        Shard2[("Shard 2<br/>user_id hash range B")]
        Shard3[("Shard 3<br/>user_id hash range C")]
        ShardN[("Shard N...")]
    end

    subgraph CrossShard["Cross-Shard Query Support"]
        ScatterGather["Scatter-Gather Query Engine<br/>(fans out, merges results)"]
        Analytics[("Analytics Warehouse<br/>(async ETL from all shards)")]
    end

    App --> ShardRouter
    ShardRouter --> ShardMap
    ShardRouter -->|"single-shard query<br/>(most common)"| Shard1
    ShardRouter -->|"single-shard query"| Shard2
    ShardRouter -->|"single-shard query"| Shard3

    App -->|"cross-shard query<br/>(rare, e.g. admin reports)"| ScatterGather
    ScatterGather --> Shard1
    ScatterGather --> Shard2
    ScatterGather --> Shard3

    Shard1 -.->|"CDC/ETL"| Analytics
    Shard2 -.->|"CDC/ETL"| Analytics
    Shard3 -.->|"CDC/ETL"| Analytics
```

**Key idea:** The Shard Router makes sharding transparent to most application code — a query for "orders by user X" gets automatically routed to the single correct shard. Genuinely cross-shard needs (platform-wide reporting) are deliberately routed to a **separate analytics path** (via CDC/ETL, covered in the CDC pipeline design) rather than forcing the live operational shards to support expensive scatter-gather queries as a first-class pattern.

---

## 4. Sharding Strategy: Consistent Hashing vs Range-Based

```mermaid
flowchart TB
    A["Sharding Approach"] --> B["Range-Based<br/>(user_id 1-1M → Shard 1,<br/>1M-2M → Shard 2, etc.)"]
    A --> C["Hash-Based<br/>(hash(user_id) % N → shard)"]
    A --> D["Consistent Hashing<br/>(hash ring with virtual nodes)"]

    B --> B1["PRO: simple, supports<br/>range queries naturally"]
    B --> B2["CON: new users always<br/>land on the newest range —<br/>creates a hot 'newest' shard<br/>for write-heavy workloads"]

    C --> C1["PRO: even distribution"]
    C --> C2["CON: adding/removing shards<br/>(changing N) remaps<br/>almost ALL keys — massive<br/>resharding cost"]

    D --> D1["PRO: even distribution AND<br/>minimal remapping when<br/>shards are added/removed<br/>(same principle as the<br/>Distributed Cache design)"]
    D --> D2["CHOSEN approach for<br/>this rapidly-growing platform"]
```

---

## 5. Consistent Hashing Applied to Database Sharding

```mermaid
flowchart TB
    A["Hash Ring"] --> B["Shard 1 occupies<br/>ring positions via<br/>virtual nodes"]
    A --> C["Shard 2 occupies<br/>ring positions"]
    A --> D["Shard 3 occupies<br/>ring positions"]

    E["hash(user_id=12345)<br/>= ring position P"] --> F["Walk clockwise to find<br/>first shard's virtual node"]
    F --> G["That shard owns this user's data"]

    H["Adding Shard 4<br/>(platform growth)"] --> I["Only the fraction of keys<br/>between Shard 4's new<br/>virtual node positions and<br/>their predecessors need<br/>to MOVE — most existing<br/>key→shard mappings<br/>remain unchanged"]
```

---

## 6. Query Routing Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant App as Application
    participant Router as Shard Router
    participant ShardMap as Shard Map
    participant Shard as Target Shard

    App->>Router: Query: get orders WHERE user_id=12345

    Router->>Router: Compute hash(12345)
    Router->>ShardMap: Lookup: which shard owns<br/>this hash range?
    ShardMap-->>Router: Shard 7

    Router->>Shard: Forward query directly to Shard 7
    Shard-->>Router: Order results
    Router-->>App: Return results

    Note over Router: Application code never<br/>needs to know which physical<br/>shard holds the data —<br/>sharding logic is entirely<br/>encapsulated in the router
```

---

## 7. Resharding Without Downtime — Detailed Sequence

```mermaid
sequenceDiagram
    participant Ops as Ops/Automation
    participant OldShard as Existing Shard
    participant NewShard as New Shard (being added)
    participant ShardMap as Shard Map
    participant Router as Shard Router

    Ops->>NewShard: Provision new shard instance

    Ops->>OldShard: Begin live replication of the<br/>key range being reassigned<br/>to New Shard (dual-write or<br/>CDC-based streaming copy)

    Note over OldShard,NewShard: Replication continues while<br/>OldShard remains fully live —<br/>reads/writes for the affected<br/>range still go to OldShard

    Ops->>Ops: Monitor replication lag<br/>until fully caught up

    Ops->>OldShard: Brief write-pause for the<br/>affected key range<br/>(milliseconds, for final sync)
    Ops->>ShardMap: Atomically update: affected<br/>key range now points to NewShard
    Ops->>OldShard: Resume writes (now redirected<br/>to New Shard via updated map)

    Router->>ShardMap: Next queries for the affected<br/>range now route to NewShard
    Ops->>OldShard: After verifying stability,<br/>delete migrated data from OldShard
```

**Why the brief write-pause matters:** Even with careful live replication, there's a small window where the "cutover" from old to new shard must happen atomically — pausing writes for milliseconds (not the whole migration duration) ensures no write is lost or applied to the wrong shard during the exact moment of cutover, while keeping actual downtime negligible.

---

## 8. Handling Cross-Shard Queries (When Truly Necessary)

```mermaid
sequenceDiagram
    participant Admin as Admin Dashboard
    participant SG as Scatter-Gather Engine
    participant S1 as Shard 1
    participant S2 as Shard 2
    participant S3 as Shard 3

    Admin->>SG: Query: total orders today<br/>(across ALL shards)

    par Fan out to all shards
        SG->>S1: COUNT orders WHERE date=today
        S1-->>SG: 45,000
    and
        SG->>S2: Same query
        S2-->>SG: 38,000
    and
        SG->>S3: Same query
        S3-->>SG: 52,000
    end

    SG->>SG: Aggregate: sum = 135,000
    SG-->>Admin: Total: 135,000 orders today

    Note over SG: This pattern works for simple<br/>aggregations but becomes<br/>increasingly impractical for<br/>complex joins/analytics —<br/>which is why a dedicated<br/>analytics warehouse (fed via<br/>CDC) is preferred for<br/>anything beyond simple<br/>cross-shard aggregates
```

---

## 9. Handling Secondary Access Patterns (Non-Shard-Key Lookups)

```mermaid
flowchart TB
    A["Primary access pattern:<br/>'orders by user_id'<br/>— fast, single-shard<br/>(shard key match)"] --> B["Secondary access pattern:<br/>'find order by order_id'<br/>(e.g., customer support<br/>looking up a specific order,<br/>WITHOUT knowing the user_id)"]

    B --> C{"How to handle a lookup<br/>NOT on the shard key?"}
    C --> D["Option 1: Global secondary<br/>index (separate lookup service:<br/>order_id → user_id → shard)"]
    C --> E["Option 2: Broadcast query<br/>to all shards<br/>(expensive, avoid at scale)"]

    D --> D1["CHOSEN: maintain a lightweight<br/>global index (e.g., in a fast<br/>KV store) mapping order_id → shard,<br/>populated at write time"]
```

```mermaid
sequenceDiagram
    participant Support as Support Agent
    participant Router as Shard Router
    participant GlobalIdx as Global Secondary Index<br/>(order_id → shard)
    participant Shard as Target Shard

    Support->>Router: Find order_id=ORD-98765<br/>(order_id is NOT the shard key)
    Router->>GlobalIdx: Lookup shard for this order_id
    GlobalIdx-->>Router: Shard 12

    Router->>Shard: Fetch order details from Shard 12
    Shard-->>Router: Order data
    Router-->>Support: Return order details
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((E-commerce Sharding HLD))
    Shard Router
      Transparent query routing
      Encapsulates sharding logic
    Shard Map
      Consistent hash ring state
      Key range to shard mapping
    Database Shards
      Independent, horizontally scaled
      Own storage and compute
    Global Secondary Index
      Non-shard-key lookup support
      order_id to shard mapping
    Scatter-Gather Engine
      Rare cross-shard aggregations
      Not for complex analytics
    Analytics Warehouse
      CDC-fed, separate from operational shards
      Handles complex cross-shard reporting
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Shard key | user_id | Optimizes for the dominant access pattern ("orders by user"), accepting cross-shard cost for rarer patterns |
| Distribution algorithm | Consistent hashing | Minimizes data movement when adding shards to accommodate growth — critical for a "rapidly growing" platform |
| Resharding approach | Live replication + brief atomic cutover | Achieves effectively zero-downtime migration, essential for a live e-commerce platform that can't tolerate extended outages |
| Non-shard-key lookups | Global secondary index | Avoids broadcasting every non-shard-key query to all shards, at the cost of maintaining an additional index structure |
| Cross-shard analytics | Separate CDC-fed warehouse, not live scatter-gather | Keeps operational shards fast and simple; complex analytics gets a purpose-built system instead of straining the transactional path |

---

## 12. Bottlenecks & Scaling Considerations

- **Shard key skew** — if certain users generate disproportionately more orders (e.g., B2B power sellers), their shard can become a hot spot despite otherwise-even hashing; may require special-casing exceptionally large accounts (similar to the multi-tenant silo-tier pattern) rather than forcing them into the standard shard.
- **Global secondary index becomes its own scaling concern** — as order volume grows, this index itself needs to scale (likely sharded independently by order_id), and its write path adds latency/complexity to every order creation.
- **Resharding operational complexity** — while the process is designed for zero downtime, it requires careful tooling, monitoring, and rollback capability; resharding should be a well-tested, semi-automated operational procedure, not an ad-hoc manual process, especially as it will need to happen repeatedly as the platform grows.
- **Foreign key/referential integrity across shards** — relationships that naturally span shard boundaries (e.g., an order referencing a product that might be sharded differently) lose database-enforced referential integrity; must be handled at the application level with appropriate validation.
- **Backup and disaster recovery per shard** — each shard needs independent backup scheduling and recovery testing; a full-platform recovery scenario requires coordinating consistent recovery points across all shards.
- **Connection pool management at scale** — application servers connecting to potentially dozens of shards need careful connection pool sizing per shard to avoid exhausting database connection limits as both shard count and application server count grow independently.
- **Monitoring per-shard health and load distribution** — proactive dashboards tracking per-shard storage growth, query latency, and load are essential for anticipating when the next resharding cycle is needed, rather than reactively scrambling once a shard is already overloaded.
