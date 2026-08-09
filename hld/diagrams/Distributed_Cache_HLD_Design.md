# Design a Distributed Cache — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Standard cache API: GET, SET, DELETE, with TTL support
- Data distributed across many nodes (dataset too large for one machine)
- Support for common eviction policies (LRU, LFU, TTL-based)
- Handle node failures without losing the whole cache
- Support cache invalidation (explicit and TTL-based)

### Non-Functional Requirements
- **Latency:** Sub-millisecond GET/SET at p99
- **Scale:** Millions of keys, 100K+ ops/sec per node, horizontally scalable
- **Availability over consistency:** A stale cache read is usually fine; an unavailable cache is not
- **Hot key resilience:** A small number of very popular keys shouldn't overwhelm a single node
- **Graceful degradation:** Cache miss should fall back to source-of-truth DB, never hard-fail the request

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Total keys | ~1B |
| Avg value size | ~1KB |
| Total dataset size | ~1TB (must be sharded across many nodes) |
| Ops/sec (platform-wide) | ~10M+ |
| Nodes needed (assuming 64GB RAM/node, ~50GB usable) | ~20+ nodes minimum, scaled for redundancy |
| Target hit ratio | > 90% for a well-tuned cache |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Client["Application Servers<br/>(Cache Clients)"]

    subgraph ClientLib["Client-Side Logic"]
        HashRing["Consistent Hashing Ring<br/>(client-side or via proxy)"]
    end

    subgraph Cluster["Cache Cluster"]
        Node1["Cache Node 1<br/>(Primary, shard A)"]
        Node1R["Cache Node 1 Replica"]
        Node2["Cache Node 2<br/>(Primary, shard B)"]
        Node2R["Cache Node 2 Replica"]
        Node3["Cache Node 3<br/>(Primary, shard C)"]
        Node3R["Cache Node 3 Replica"]
    end

    subgraph Coordination["Cluster Coordination"]
        ConfigSvc["Config/Membership Service<br/>(ZooKeeper/etcd)"]
    end

    subgraph Backing["Source of Truth"]
        DB[("Primary Database")]
    end

    Client --> HashRing
    HashRing -->|"key hashes to shard A"| Node1
    HashRing -->|"key hashes to shard B"| Node2
    HashRing -->|"key hashes to shard C"| Node3

    Node1 -.->|"async replication"| Node1R
    Node2 -.->|"async replication"| Node2R
    Node3 -.->|"async replication"| Node3R

    Node1 -.-> ConfigSvc
    Node2 -.-> ConfigSvc
    Node3 -.-> ConfigSvc
    HashRing -.->|"watches cluster topology"| ConfigSvc

    Client -->|"on cache miss"| DB
```

**Key idea:** Clients (or a thin proxy layer) use **consistent hashing** to determine which cache node owns a given key, avoiding a single coordinator bottleneck for routing. A separate lightweight coordination service tracks cluster membership so clients know when nodes join/leave.

---

## 3. Consistent Hashing — How Key Routing Works

```mermaid
flowchart TB
    A["Hash Ring (0 to 2^32-1)"] --> B["Node A occupies positions:<br/>hash(NodeA#1), hash(NodeA#2)... (virtual nodes)"]
    A --> C["Node B occupies positions:<br/>hash(NodeB#1), hash(NodeB#2)..."]
    A --> D["Node C occupies positions:<br/>hash(NodeC#1), hash(NodeC#2)..."]

    E["Key 'user:123'"] --> F["hash('user:123') = position P"]
    F --> G["Walk clockwise from P<br/>to find first node position"]
    G --> H["Owned by that node"]

    I["Node B fails/removed"] --> J["Only keys mapped to<br/>Node B's ring segments<br/>need to move"]
    J --> K["Other nodes' key ownership<br/>unaffected — minimal reshuffling"]
```

**Why consistent hashing (not simple `hash(key) % N`):** With modulo hashing, adding/removing a single node remaps almost every key, causing a cache-wide stampede to the backing DB. Consistent hashing (with virtual nodes for even distribution) ensures only the fraction of keys owned by the changed node need to move.

---

## 4. Data Model / Storage Structure (Per Node)

```mermaid
erDiagram
    CACHE_ENTRY {
        string key PK
        bytes value
        timestamp expires_at
        timestamp last_accessed
        int access_frequency
    }
```

*Internally, each node maintains an in-memory hash table for O(1) key lookup, paired with an eviction-policy-specific auxiliary structure (doubly-linked list for LRU, frequency buckets for LFU, or a min-heap of expiry times for TTL sweeping).*

---

## 5. Cache Read/Write Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant App as Application Server
    participant Hash as Consistent Hash Router
    participant Cache as Cache Node
    participant DB as Database

    App->>Hash: GET user:123
    Hash->>Hash: Compute hash, find owning node
    Hash->>Cache: GET user:123

    alt Cache hit
        Cache-->>Hash: Return value
        Hash-->>App: Return value
    else Cache miss
        Cache-->>Hash: Not found
        Hash-->>App: Not found
        App->>DB: Query database directly
        DB-->>App: Value
        App->>Hash: SET user:123 = value, TTL=300s
        Hash->>Cache: SET user:123
        Cache-->>Hash: Ack
    end
```

---

## 6. Cache-Aside Pattern (Most Common Strategy)

```mermaid
flowchart TB
    A["Application needs data"] --> B{"Check cache first"}
    B -- "Hit" --> C["Return cached value<br/>(fast path)"]
    B -- "Miss" --> D["Query database<br/>(slow path)"]
    D --> E["Write result to cache<br/>with TTL"]
    E --> F["Return value to application"]

    G["Data is updated<br/>(write path)"] --> H["Write to database<br/>(source of truth)"]
    H --> I["Invalidate/delete<br/>corresponding cache key<br/>(NOT update cache directly)"]
```

**Why invalidate rather than update-on-write:** Updating the cache directly on every DB write risks the cache and DB drifting out of sync if the write path has multiple steps or partial failures. Deleting the cache key and letting the next read repopulate it (cache-aside) is simpler and self-healing — worst case is one extra DB read, not permanent staleness.

---

## 7. Eviction Policies

```mermaid
flowchart LR
    A["Cache node reaches<br/>memory limit"] --> B{"Eviction Policy"}
    B --> C["LRU<br/>(Least Recently Used)"]
    B --> D["LFU<br/>(Least Frequently Used)"]
    B --> E["TTL-based<br/>(expire oldest first)"]

    C --> C1["Evict item at tail of<br/>doubly-linked access list"]
    D --> D1["Evict item with<br/>lowest access count"]
    E --> E1["Evict item with<br/>nearest expiry time"]

    F["Common in practice:<br/>LRU + TTL combined —<br/>TTL as a hard ceiling,<br/>LRU for memory pressure"]
```

---

## 8. Replication & Failover

```mermaid
sequenceDiagram
    participant Client as App Client
    participant Primary as Primary Node (Shard A)
    participant Replica as Replica Node (Shard A)
    participant Config as Config Service

    Client->>Primary: SET key = value
    Primary->>Primary: Write to local memory
    Primary-->>Client: Ack (fast, doesn't wait for replica)
    Primary-->>Replica: Async replicate write

    Note over Primary: Primary node crashes

    Config->>Config: Detect Primary failure<br/>(missed heartbeats)
    Config->>Replica: Promote to Primary
    Config->>Client: Notify: shard A now served by<br/>former Replica

    Client->>Replica: Subsequent requests<br/>routed to new Primary
```

**Tradeoff:** Async replication means a small window of potential data loss on failover (writes not yet replicated when primary crashes) — but this is generally acceptable for a cache, since the backing database remains the true source of truth and a lost cache entry just means a future cache miss, not lost data.

---

## 9. Hot Key Mitigation

```mermaid
flowchart TB
    A["Single key (e.g., viral post)<br/>receiving 100K+ reads/sec"] --> B{"Mitigation Strategy"}
    B --> C["Local in-process cache<br/>on application servers<br/>(L1 cache before hitting L2 distributed cache)"]
    B --> D["Key replication:<br/>store hot key on multiple nodes,<br/>client picks one at random"]
    B --> E["Request coalescing:<br/>concurrent misses for same key<br/>trigger only ONE backend fetch"]

    C --> F["Reduces network calls<br/>to distributed cache entirely"]
    D --> G["Spreads load across<br/>N nodes instead of 1"]
    E --> H["Prevents thundering herd<br/>on cache miss/expiry"]
```

---

## 10. Thundering Herd / Cache Stampede Prevention

```mermaid
sequenceDiagram
    participant C1 as Client 1
    participant C2 as Client 2
    participant C3 as Client 3
    participant Cache as Cache Node
    participant DB as Database

    Note over Cache: Popular key expires simultaneously

    C1->>Cache: GET hot_key
    Cache-->>C1: Miss
    C1->>Cache: Acquire lock for hot_key rebuild
    Cache-->>C1: Lock acquired

    C2->>Cache: GET hot_key
    Cache-->>C2: Miss
    C2->>Cache: Acquire lock for hot_key rebuild
    Cache-->>C2: Lock busy — wait/retry shortly

    C3->>Cache: GET hot_key
    Cache-->>C3: Same — wait/retry

    C1->>DB: Query (only ONE request hits DB)
    DB-->>C1: Value
    C1->>Cache: SET hot_key = value
    C1->>Cache: Release lock

    C2->>Cache: Retry GET hot_key
    Cache-->>C2: Hit (populated by C1)
    C3->>Cache: Retry GET hot_key
    Cache-->>C3: Hit
```

*Without this lock-based coalescing, a popular key expiring under high load causes hundreds of concurrent clients to simultaneously miss and hammer the database — the classic "thundering herd" or "cache stampede" problem.*

---

## 11. Component Responsibilities Summary

```mermaid
mindmap
  root((Distributed Cache HLD))
    Consistent Hash Router
      Client-side or proxy-based
      Minimal reshuffling on topology change
    Cache Nodes
      In-memory storage
      Eviction policy enforcement
      Primary/replica pairs
    Config/Membership Service
      Cluster topology tracking
      Failure detection
      Failover coordination
    Replication
      Async primary to replica
      Availability over consistency
    Stampede Protection
      Lock-based rebuild coalescing
      Request deduplication
```

---

## 12. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Key routing | Consistent hashing with virtual nodes | Minimizes key remapping when nodes join/leave; avoids cache-wide stampede on topology change |
| Consistency model | Eventual (async replication) | A cache prioritizes availability and speed; the backing DB remains the source of truth for correctness |
| Write pattern | Cache-aside with invalidation-on-write | Simpler and self-healing compared to write-through/write-behind; avoids cache/DB drift |
| Eviction | LRU + TTL combined | TTL bounds staleness; LRU handles memory pressure for keys that haven't expired yet |
| Hot key handling | L1 local cache + replication + coalescing | No single mitigation is sufficient alone; layered defenses handle different traffic patterns |
| Failure handling | Fail open (fall back to DB on cache unavailability) | A cache outage should degrade performance, not take down the whole application |

---

## 13. Bottlenecks & Scaling Considerations

- **Hot keys / celebrity content** — a single popular key can overwhelm one node even with consistent hashing (which balances key *count*, not key *popularity*); needs explicit hot-key detection and mitigation (see above).
- **Memory fragmentation** — long-running cache nodes with variable-size values can suffer internal fragmentation; many production caches (Redis, Memcached) use slab allocation to manage this.
- **Rebalancing cost during scale-out** — even with consistent hashing, adding nodes still requires migrating some data; do this gradually with rate-limited migration to avoid impacting live traffic.
- **Split-brain during network partitions** — if the config service itself partitions, two nodes might both believe they're primary for the same shard; use a consensus-based config store (etcd/ZooKeeper) to avoid this.
- **Cache warming after full restart** — a cold cache after a major outage causes a massive DB load spike; consider pre-warming from a snapshot or gradually ramping traffic back rather than instant full cutover.
- **Cross-region caching** — global applications may need region-local cache clusters (to avoid cross-region latency) with careful invalidation propagation, since a write in one region needs to invalidate stale entries in others.
