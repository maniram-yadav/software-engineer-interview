# Design a Vector Clock / Causal Ordering System for a Distributed Database — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Track causal relationships between events/writes across multiple distributed replicas
- Determine whether one event happened-before another, happened-after, or is concurrent with it
- Detect genuine write conflicts (concurrent, unordered writes to the same key from different replicas)
- Support conflict resolution when concurrent writes are detected (application-level or automatic)

### Non-Functional Requirements
- **Correctness:** Must never incorrectly report two causally-related events as concurrent, or vice versa
- **Low overhead:** Vector clock metadata shouldn't dominate the actual data size for typical writes
- **Scalability:** Must remain practical as the number of replicas/nodes grows (this is vector clocks' known weak point)
- **No reliance on synchronized physical clocks:** Must work correctly even with arbitrary clock skew across nodes

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Typical replica count | 3-10 (vector clock size grows linearly with this) |
| Vector clock overhead per write | ~8 bytes × number of replicas |
| Writes/sec (platform-wide) | Varies — this is metadata overhead on top of normal write volume |
| Conflict rate | Usually low (<1% of writes) in well-partitioned key spaces |

---

## 2. The Core Problem — Why Physical Timestamps Aren't Enough

```mermaid
flowchart TB
    A["Two replicas, Node A and Node B,<br/>each write to the same key<br/>independently"] --> B["Naive approach:<br/>compare wall-clock timestamps,<br/>latest wins"]
    B --> C["Problem: clock skew between<br/>machines means 'later timestamp'<br/>doesn't reliably mean<br/>'actually happened after'"]
    B --> D["Problem: even with perfect clocks,<br/>timestamps alone can't tell you<br/>whether one write CAUSALLY<br/>depended on the other, or<br/>whether they were truly<br/>independent/concurrent"]

    E["Vector clocks solve this by<br/>tracking LOGICAL causality —<br/>not physical time at all"]
```

---

## 3. Vector Clock Structure & Rules

```mermaid
flowchart TB
    A["Vector Clock = array of counters,<br/>one per replica/node<br/>e.g., [A:0, B:0, C:0] initially"] --> B["Rule 1: On a local event<br/>at Node X, increment<br/>ONLY that node's own counter"]
    A --> C["Rule 2: When sending a message,<br/>attach current vector clock"]
    A --> D["Rule 3: On receiving a message,<br/>merge: take the ELEMENT-WISE<br/>MAXIMUM of local clock and<br/>received clock, then increment<br/>own counter"]
```

```mermaid
flowchart LR
    A["Node A: [1,0,0]<br/>(A's local write)"] -->|"send"| B["Node B receives,<br/>merges: max([0,0,0],[1,0,0])<br/>= [1,0,0], then increments B:<br/>[1,1,0]"]
    B -->|"send"| C["Node C receives,<br/>merges: max([0,0,0],[1,1,0])<br/>= [1,1,0], then increments C:<br/>[1,1,1]"]
```

---

## 4. Comparing Vector Clocks — Determining Causality

```mermaid
flowchart TB
    A["Compare Vector Clock V1 vs V2"] --> B{"Is V1[i] <= V2[i]<br/>for ALL i, AND<br/>V1 != V2?"}
    B -- Yes --> C["V1 HAPPENED-BEFORE V2<br/>(V1 → V2)<br/>V2 causally depends on V1"]

    A --> D{"Is V2[i] <= V1[i]<br/>for ALL i, AND<br/>V1 != V2?"}
    D -- Yes --> E["V2 HAPPENED-BEFORE V1<br/>(V2 → V1)"]

    A --> F{"Neither condition holds<br/>(some indices V1 is bigger,<br/>others V2 is bigger)"}
    F -- Yes --> G["V1 and V2 are CONCURRENT<br/>(neither caused the other) —<br/>this is a genuine conflict<br/>if both wrote the same key"]
```

### Worked Example

```mermaid
flowchart TB
    A["V1 = [2,1,0]<br/>V2 = [2,1,1]"] --> B["Compare element-wise:<br/>2<=2 ✓, 1<=1 ✓, 0<=1 ✓<br/>V1 <= V2 everywhere,<br/>and V1 != V2"]
    B --> C["Conclusion: V1 HAPPENED-BEFORE V2<br/>— V2's writer had already<br/>observed everything V1 knew<br/>about, plus one more event"]

    D["V3 = [2,1,0]<br/>V4 = [1,2,0]"] --> E["Compare element-wise:<br/>2>1 but 1<2 — MIXED<br/>Neither is uniformly<br/>less-than-or-equal"]
    E --> F["Conclusion: V3 and V4<br/>are CONCURRENT —<br/>genuine conflict if both<br/>wrote the same key independently"]
```

---

## 5. High-Level Architecture

```mermaid
flowchart TB
    subgraph Nodes["Distributed Database Nodes"]
        NodeA["Node A<br/>(local vector clock state)"]
        NodeB["Node B<br/>(local vector clock state)"]
        NodeC["Node C<br/>(local vector clock state)"]
    end

    subgraph Storage["Per-Key Storage"]
        KeyStore[("Key → {value, vector_clock}<br/>Multiple concurrent versions<br/>stored if conflict detected")]
    end

    subgraph ConflictHandling["Conflict Resolution"]
        AutoMerge["Automatic Merge<br/>(if mergeable — e.g., CRDT-based)"]
        AppResolve["Application-Level Resolution<br/>(present both versions to<br/>client/user for manual merge)"]
    end

    Client["Client Write/Read"]

    Client --> NodeA
    NodeA --> KeyStore
    NodeA -.->|"replicate + vector clock"| NodeB
    NodeA -.->|"replicate + vector clock"| NodeC

    KeyStore --> AutoMerge
    KeyStore --> AppResolve
```

**Key idea:** Each stored value carries its vector clock alongside it. When a replica receives a write for a key, it compares the incoming vector clock against what it currently has stored. If the incoming write's clock strictly dominates (happened-after) the stored version, it's a clean update. If they're concurrent, **both versions must be preserved** — this is the core design implication of vector clocks: the system can detect conflicts precisely, but resolving them is a separate, deliberate step.

---

## 6. Write Flow with Conflict Detection — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client
    participant NodeA as Node A
    participant Store as Key Store
    participant NodeB as Node B (replica)

    C->>NodeA: Write key="profile", value="Alice, NYC"
    NodeA->>NodeA: Increment own clock: [2,1,0]
    NodeA->>Store: Store {value: "Alice, NYC", clock: [2,1,0]}
    NodeA-->>C: Ack

    NodeA->>NodeB: Replicate write + clock [2,1,0]

    Note over NodeB: Node B ALSO independently<br/>received a write for the SAME key<br/>from a different client,<br/>with clock [1,2,0] (concurrent!)

    NodeB->>NodeB: Compare incoming [2,1,0]<br/>vs local stored [1,2,0]
    NodeB->>NodeB: Neither dominates —<br/>CONCURRENT, genuine conflict

    NodeB->>Store: Store BOTH versions:<br/>{value: "Alice, NYC", clock:[2,1,0]}<br/>{value: "Alice, Boston", clock:[1,2,0]}

    Note over NodeB: Conflict surfaced to<br/>application/client on next read —<br/>NOT silently resolved by the<br/>storage layer itself
```

---

## 7. Conflict Resolution Strategies

```mermaid
flowchart TB
    A["Concurrent writes detected<br/>(genuine conflict)"] --> B{"Resolution Strategy"}

    B --> C["Last-Write-Wins (LWW)<br/>by physical timestamp"]
    C --> C1["Simple, but can silently<br/>discard a legitimate<br/>concurrent update —<br/>use only when losing data<br/>is acceptable"]

    B --> D["Application-Level Merge<br/>(present both versions,<br/>let client/user decide)"]
    D --> D1["Classic example: Amazon<br/>DynamoDB's original shopping<br/>cart — concurrent adds to a<br/>cart are merged as a UNION<br/>of items, not one overwriting<br/>the other"]

    B --> E["CRDT-Based Automatic Merge<br/>(when the data type supports<br/>a mathematically well-defined<br/>merge function)"]
    E --> E1["e.g., a set: union of both<br/>versions; a counter: sum<br/>of independent increments"]

    F["Choice depends entirely<br/>on the semantics of the data —<br/>there's no universal<br/>'correct' resolution"]
```

---

## 8. Read Repair (Reconciling Divergent Replicas)

```mermaid
sequenceDiagram
    participant C as Client
    participant NodeA as Node A
    participant NodeB as Node B
    participant NodeC as Node C

    C->>NodeA: Read key="profile" (quorum read from A, B, C)
    NodeA-->>C: {value:"Alice, NYC", clock:[2,1,0]}
    NodeB-->>C: {value:"Alice, NYC", clock:[2,1,0]}
    NodeC-->>C: {value:"Alice, Boston", clock:[1,2,0]}
    Note over NodeC: C hasn't yet received A's<br/>later replicated write

    C->>C: Compare returned clocks:<br/>detect A/B agree, C is behind<br/>(or genuinely concurrent — check!)

    alt C's version is strictly older (happened-before)
        C->>NodeC: Read repair: push newer version<br/>{value:"Alice, NYC", clock:[2,1,0]}
        NodeC->>NodeC: Update to latest
    else Genuinely concurrent versions across replicas
        C->>C: Surface conflict to application<br/>for resolution (as in section 7)
    end
```

*"Read repair" is a common pattern in eventually-consistent systems (like Dynamo-style databases) where the act of reading from multiple replicas also opportunistically fixes replicas that have fallen behind — spreading the cost of consistency maintenance across normal read traffic rather than requiring a dedicated background process alone.*

---

## 9. The Scalability Problem With Vector Clocks

```mermaid
flowchart TB
    A["Vector clock size grows<br/>LINEARLY with number of<br/>replicas/actors that have<br/>ever written"] --> B["10 replicas = 10-element vector<br/>attached to EVERY write"]
    A --> C["1,000 replicas/clients<br/>(e.g., if clients themselves<br/>are tracked as actors) =<br/>impractically large metadata<br/>per write"]

    D["Mitigation Strategies"] --> E["Limit vector clock actors<br/>to SERVER replicas only,<br/>never individual clients"]
    D --> F["Vector clock pruning —<br/>drop entries for replicas<br/>inactive beyond a threshold<br/>(accepting a small correctness<br/>risk for those rare cases)"]
    D --> G["Alternative: Dotted Version<br/>Vectors or Interval Version<br/>Vectors (more compact variants<br/>used in production systems<br/>like Riak)"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Vector Clock HLD))
    Local Clock State
      Per-replica counter array
      Incremented on local events
    Clock Merge Logic
      Element-wise maximum
      Applied on message receipt
    Causality Comparator
      Determines happened-before
      vs concurrent relationship
    Key Store
      Stores multiple versions
      when conflicts detected
    Conflict Resolver
      LWW / app-level / CRDT merge
      Use-case dependent strategy
    Read Repair
      Opportunistic consistency fix
      Piggybacks on normal reads
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Causality tracking | Vector clocks over physical timestamps | Physical clocks can't reliably distinguish "happened after" from "just has a later clock reading due to skew" |
| Conflict handling | Preserve both concurrent versions, don't silently pick one | Silently discarding one concurrent write loses data the application might genuinely need to reconcile |
| Resolution strategy | Pluggable (LWW / app-level / CRDT) based on data semantics | No single strategy is correct for all data types — a shopping cart needs union-merge, a "last status update" might be fine with LWW |
| Actor scope | Server replicas only, not individual clients | Keeps vector clock size bounded and practical; unbounded actor growth defeats the mechanism's efficiency |
| Consistency check on read | Read repair using quorum reads | Spreads consistency-maintenance cost across normal traffic rather than requiring a dedicated always-on background process |

---

## 12. Bottlenecks & Scaling Considerations

- **Metadata overhead grows with replica count** — this is vector clocks' fundamental scalability limitation; must be bounded by keeping the "actor" set to a small, stable number of server replicas rather than allowing it to grow with client count or over time.
- **Conflict resolution burden shifts to the application** — vector clocks excel at *detecting* conflicts precisely, but resolution logic must still be designed thoughtfully per data type; this is often underestimated complexity when adopting vector-clock-based systems.
- **Multiple stored versions increase storage cost** — keys with frequent concurrent writes accumulate multiple unresolved versions until reconciled; needs monitoring for keys with pathologically high conflict rates (often a sign of a data modeling issue — that key is being written to independently from too many places).
- **Clock pruning correctness tradeoff** — dropping inactive replica entries from vector clocks (to bound size) introduces a small risk of misclassifying causality involving very old writes; this tradeoff must be deliberate and bounded (e.g., prune only after a replica has been gone far longer than any realistic reconciliation window).
- **Testing causality logic is non-intuitive** — off-by-one errors in the increment/merge rules can silently corrupt causality tracking; this logic benefits enormously from extensive property-based testing (generating random concurrent event sequences and verifying causality relationships hold) rather than just example-based unit tests.
- **Interaction with sharding** — vector clocks track causality per-key (or per-shard) typically, not globally across an entire distributed database — designers must be explicit about the scope of causal guarantees (e.g., "causality is tracked within a key, not across different keys/shards") to avoid overpromising consistency the mechanism doesn't actually provide.
