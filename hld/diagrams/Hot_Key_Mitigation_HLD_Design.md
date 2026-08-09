# Design a System to Handle "Hot Key" Problems in a Distributed Cache — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Detect when a specific cache key is receiving disproportionately high traffic relative to others
- Distribute the load for that hot key across multiple cache nodes rather than concentrating it on one
- Maintain correctness (all reads still return the right value) despite the load-distribution mechanism
- Support both organic (gradual) and sudden (viral/flash) hot key emergence

### Non-Functional Requirements
- **Single-node capacity limits:** A single cache node/shard has a hard ceiling on requests/sec it can serve, regardless of overall cluster size
- **Minimal detection lag:** A key going viral should be mitigated within seconds, not minutes, to avoid cascading failure
- **Low overhead in the common case:** The mitigation machinery shouldn't add meaningful cost to the 99.9% of keys that are NOT hot
- **Graceful degradation:** Even under extreme hot-key load, the system should degrade predictably, not fail catastrophically

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Normal key traffic | Hundreds to low thousands of req/sec |
| Hot key traffic (viral event) | 100,000+ req/sec for a SINGLE key |
| Single cache node capacity | ~50,000-100,000 ops/sec typical ceiling |
| Detection target | Within a few seconds of onset |

---

## 2. Why This Is a Fundamentally Different Problem From General Cache Scaling

```mermaid
flowchart TB
    A["Normal distributed cache<br/>scaling (consistent hashing,<br/>as in the Distributed Cache<br/>design) solves for EVEN<br/>DISTRIBUTION OF KEYS<br/>across nodes"] --> B["This works great when<br/>traffic is roughly uniform<br/>across keys — each node<br/>gets a proportional, manageable<br/>share of overall load"]

    C["Hot key problem: ONE<br/>SPECIFIC KEY receives<br/>disproportionate traffic —<br/>no matter how well keys<br/>are distributed across<br/>nodes, that ONE key still<br/>lives on exactly ONE node<br/>(or shard) by definition<br/>of standard hashing"] --> D["Adding MORE nodes to the<br/>cluster does NOTHING to<br/>help — the hot key's<br/>traffic doesn't spread out,<br/>it's still concentrated on<br/>whichever single node owns<br/>that key"]

    D --> E["This requires a<br/>FUNDAMENTALLY DIFFERENT<br/>mitigation strategy than<br/>general cluster scaling —<br/>the key itself must be<br/>artificially spread across<br/>MULTIPLE nodes"]
```

---

## 3. High-Level Architecture — Layered Defense

```mermaid
flowchart TB
    Client["Application Servers"]

    subgraph L1["Layer 1: Local In-Process Cache"]
        LocalCache["In-memory cache<br/>on each app server<br/>(no network call at all)"]
    end

    subgraph L2["Layer 2: Hot Key Detection"]
        Detector["Hot Key Detector<br/>(sampling-based traffic monitor)"]
    end

    subgraph L3["Layer 3: Key Replication"]
        ReplicatedKey["Hot key replicated across<br/>MULTIPLE cache nodes<br/>(not just its original owner)"]
    end

    subgraph Cluster["Distributed Cache Cluster"]
        Node1["Cache Node 1<br/>(original owner)"]
        Node2["Cache Node 2<br/>(replica for hot key)"]
        Node3["Cache Node 3<br/>(replica for hot key)"]
    end

    Client --> LocalCache
    LocalCache -->|"local miss"| Detector
    Detector -->|"detects hot key"| ReplicatedKey
    ReplicatedKey --> Node1
    ReplicatedKey --> Node2
    ReplicatedKey --> Node3

    Client -->|"randomly picks one<br/>of the replica nodes"| Node1
    Client -->|"or"| Node2
    Client -->|"or"| Node3
```

**Key idea:** This is a layered defense, not a single mechanism — Layer 1 (local caching) eliminates network calls entirely for the most frequently accessed data, Layer 2 continuously watches for emerging hot keys, and Layer 3 activates only when needed, spreading a detected hot key's load across multiple nodes rather than concentrating it on its single natural owner.

---

## 4. Layer 1 — Local In-Process Caching (First Line of Defense)

```mermaid
sequenceDiagram
    participant Client as Application Server
    participant Local as Local In-Process Cache<br/>(in-memory, per-instance)
    participant Cache as Distributed Cache Cluster

    Client->>Local: GET hot_key
    alt Present in local cache (short TTL, e.g., 1-5 seconds)
        Local-->>Client: Return immediately<br/>(ZERO network calls to<br/>the distributed cache at all)
    else Not in local cache
        Local->>Cache: GET hot_key
        Cache-->>Local: Value
        Local->>Local: Store locally with<br/>short TTL
        Local-->>Client: Return value
    end
```

**Why even a very short local TTL (1-5 seconds) helps enormously:** If an application server would otherwise make 1,000 requests/sec for the same hot key, a 1-second local cache reduces this to just 1 request/sec hitting the distributed cache from that server — a 1000x reduction, achieved with almost no staleness cost for most use cases. Multiplied across every application server instance, this dramatically reduces the load that ever reaches the distributed cache cluster for hot keys.

---

## 5. Layer 2 — Hot Key Detection

```mermaid
flowchart TB
    A["Cache Node receiving requests"] --> B["Sampling-based traffic monitor<br/>(e.g., sample 1-in-100 requests,<br/>track key frequency in a<br/>sliding window)"]

    B --> C{"Is any key's sampled<br/>frequency disproportionately<br/>high relative to normal<br/>traffic patterns?"}
    C -- Yes --> D["Flag as HOT KEY —<br/>trigger mitigation<br/>(Layer 3 replication)"]
    C -- No --> E["Continue normal<br/>single-node serving"]

    F["Why sampling, not counting<br/>every single request?"] --> G["Counting exact frequency for<br/>EVERY key on EVERY request<br/>would itself add significant<br/>overhead to the 99.9% of<br/>normal, non-hot traffic —<br/>sampling gets a statistically<br/>reliable signal at a fraction<br/>of the overhead cost"]
```

```mermaid
sequenceDiagram
    participant Node as Cache Node
    participant Sampler as Sampling Monitor
    participant Detector as Hot Key Detector

    loop On each request (1-in-100 sampled)
        Node->>Sampler: Record key access<br/>(sampled, not every request)
    end

    loop Every detection window (e.g., 1 second)
        Sampler->>Detector: Report sampled frequency<br/>counts per key
        Detector->>Detector: Compare against threshold<br/>(e.g., extrapolated rate ><br/>10,000 req/sec)

        alt Threshold exceeded
            Detector->>Detector: Mark key as HOT
            Detector->>Detector: Trigger Layer 3<br/>replication process
        end
    end
```

---

## 6. Layer 3 — Hot Key Replication (Active Mitigation)

```mermaid
sequenceDiagram
    participant Detector as Hot Key Detector
    participant Owner as Original Owner Node
    participant Node2 as Replica Node 2
    participant Node3 as Replica Node 3
    participant Router as Client-Side Router

    Detector->>Owner: Detected hot_key exceeding<br/>threshold — initiate replication
    Owner->>Node2: Copy current value<br/>for hot_key
    Owner->>Node3: Copy current value<br/>for hot_key

    Detector->>Router: Update routing metadata:<br/>hot_key now available on<br/>[Owner, Node2, Node3]<br/>— NOT just the original owner

    Note over Router: Subsequent client requests<br/>for hot_key are now<br/>RANDOMLY distributed across<br/>all 3 nodes, instead of<br/>ALL concentrating on Owner

    Router->>Owner: 1/3 of traffic
    Router->>Node2: 1/3 of traffic
    Router->>Node3: 1/3 of traffic

    Note over Owner,Node3: Each node now handles only<br/>~33% of the original load —<br/>can scale this to MORE<br/>replica nodes if the key<br/>remains extremely hot
```

**Why this requires breaking the normal consistent-hashing rule deliberately:** Under standard consistent hashing, a given key ALWAYS maps to exactly one node (or a fixed replica set for fault tolerance) — this is what makes lookups efficient. Hot key mitigation deliberately violates this for specific flagged keys, trading the simplicity of "one canonical location" for load distribution — the routing layer must now track this exception explicitly rather than relying on pure hash-based routing for these specific keys.

---

## 7. Handling Writes to a Hot (Replicated) Key

```mermaid
flowchart TB
    A["Write/update to a key that's<br/>CURRENTLY flagged as hot<br/>and replicated across<br/>multiple nodes"] --> B{"Write Strategy"}

    B --> C["Write to ALL replica<br/>locations synchronously"]
    C --> C1["PRO: all replicas stay<br/>consistent immediately<br/>CON: write latency increases<br/>proportionally to replica count"]

    B --> D["Write to owner only,<br/>propagate to replicas<br/>asynchronously"]
    D --> D1["PRO: fast writes<br/>CON: brief window where<br/>replicas serve STALE data<br/>until propagation completes"]

    E["Most hot keys in practice<br/>are READ-heavy (e.g., a<br/>viral post's content, a<br/>trending product's details)<br/>— writes are comparatively<br/>rare, making the async<br/>approach's brief staleness<br/>window an acceptable tradeoff<br/>for most use cases"] -.-> D
```

---

## 8. De-escalation (Returning a Key to Normal Status)

```mermaid
sequenceDiagram
    participant Detector as Hot Key Detector
    participant Router as Client-Side Router
    participant Replicas as Replica Nodes

    loop Continuous monitoring, even after mitigation
        Detector->>Detector: Check: has this key's<br/>traffic dropped back below<br/>the hot-key threshold for<br/>a sustained period<br/>(e.g., 5+ minutes)?
    end

    alt Traffic has normalized
        Detector->>Router: De-escalate: hot_key no<br/>longer needs multi-node<br/>replication
        Router->>Router: Revert to standard<br/>single-owner routing
        Detector->>Replicas: Clean up replicated<br/>copies (except the<br/>original owner)
    end
```

**Why de-escalation matters:** Permanently replicating every key that was ever briefly hot would cause the mitigation mechanism itself to become a growing source of overhead and complexity over time. Automatically reverting keys to normal single-node handling once their traffic normalizes keeps the system's "exception list" small and proportional to CURRENT hot keys, not an ever-growing historical record.

---

## 9. Alternative/Complementary Mitigation: Key Splitting

```mermaid
flowchart TB
    A["Hot key: 'global_counter'<br/>receiving massive<br/>concurrent INCREMENT traffic"] --> B["Instead of replicating the<br/>SAME key, SPLIT it into<br/>multiple sub-keys"]

    B --> C["global_counter:shard_1"]
    B --> D["global_counter:shard_2"]
    B --> E["global_counter:shard_3"]

    C & D & E --> F["Writes distributed across<br/>sub-keys (e.g., client<br/>randomly picks a shard<br/>to increment)"]

    F --> G["Reads: SUM all sub-key<br/>shards to get the true<br/>total value<br/>(same principle as the<br/>CRDT Counter design's<br/>per-replica counter approach)"]
```

*This key-splitting technique is particularly effective for hot counters specifically (as opposed to hot read-mostly content keys), since it avoids the read-after-write consistency complexity of replication — each shard is independently, unambiguously correct, and the true value is simply their sum.*

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Hot Key Mitigation HLD))
    Local In-Process Cache
      First line of defense
      Zero network calls for hits
      Short TTL to bound staleness
    Hot Key Detector
      Sampling-based monitoring
      Threshold-triggered activation
    Replication Mechanism
      Multi-node copies of hot key
      Overrides standard hash routing
    Client-Side Router
      Distributes requests across replicas
      Tracks hot-key exception routing
    De-escalation Process
      Reverts to normal routing
      Keeps exception list bounded
    Key Splitting
      Alternative for hot counters
      Avoids replication consistency complexity
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| First-line defense | Local in-process caching with short TTL | Eliminates the vast majority of hot-key network traffic before it ever reaches the distributed cache cluster, at near-zero staleness cost |
| Detection mechanism | Sampling-based monitoring | Provides a statistically reliable hot-key signal without adding meaningful overhead to the overwhelming majority of normal traffic |
| Core mitigation | Deliberate multi-node replication for flagged keys only | Directly addresses the root cause — a single key's traffic being concentrated on one node — which adding more cluster nodes alone cannot fix |
| Write handling for hot keys | Asynchronous propagation (given read-heavy nature of most hot keys) | Optimizes for the common case (read-heavy hot content) while accepting a brief, usually-acceptable staleness window |
| De-escalation | Automatic reversion after sustained traffic normalization | Keeps the mitigation mechanism's overhead proportional to CURRENT hot keys, not an ever-growing historical list |
| Counter-specific alternative | Key splitting rather than replication | Avoids replication's consistency complexity entirely for the specific, common case of hot increment-heavy counters |

---

## 12. Bottlenecks & Scaling Considerations

- **Detection lag vs false-positive tradeoff** — a shorter detection window catches emerging hot keys faster but risks false-positive flagging from normal traffic bursts; a longer window is more accurate but delays mitigation during exactly the critical early moments of a viral spike — this requires careful tuning based on the platform's actual traffic volatility patterns.
- **Replication factor scaling** — for an EXTREMELY hot key (e.g., a global celebrity event), even 3-5 replica nodes might not be sufficient; the system should support dynamically increasing replica count in response to sustained extreme load, not just a fixed replication factor.
- **Router metadata consistency** — the client-side router's knowledge of "which keys are currently hot and where their replicas live" must itself be kept consistent and low-latency across the entire application server fleet — this metadata distribution is itself a smaller-scale version of the broader cache-coherence problem.
- **Cross-region hot keys** — for a globally distributed cache (connecting to the Multi-Layer CDN design), a globally viral key needs replication awareness across regions, not just within a single cluster — the same principles apply but with added cross-region latency considerations for replica synchronization.
- **Cascading hot keys** — sometimes mitigating one hot key (e.g., successfully spreading load for a viral product's main record) reveals a SECOND hot key underneath (e.g., its associated inventory count now becomes the bottleneck, since ALL the redirected traffic still needs that shared piece of data) — mitigation may need to be applied iteratively, not just once.
- **Testing hot key scenarios proactively** — because hot keys are often triggered by unpredictable external events (viral social media moments, news events), the detection and mitigation machinery benefits enormously from deliberate load testing that simulates sudden extreme concentration on a single key, rather than only being validated reactively after a real incident occurs.
