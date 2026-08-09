# Design a Distributed Key-Value Store (DynamoDB-style) — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Basic operations: `PUT(key, value)`, `GET(key)`, `DELETE(key)`
- Data automatically partitioned (sharded) across many nodes
- Data replicated across nodes for durability and availability
- Configurable consistency (tunable per-request: strong vs eventual)
- Automatic handling of node failures without data loss

### Non-Functional Requirements
- **Availability > strict consistency** by default (AP system, per CAP theorem), with tunable consistency
- **Horizontal scalability:** Add/remove nodes without full system downtime or full data reshuffle
- **Low latency:** Single-digit millisecond reads/writes at p99
- **Partition tolerance:** Must continue operating during network partitions between nodes
- **Durability:** Once a write is acknowledged, it must survive node failures

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Nodes in cluster | Hundreds to thousands |
| Total keys | Billions |
| Replication factor | Typically 3 |
| Reads/writes per sec (cluster-wide) | Millions |
| Latency target | < 10ms p99 for single-key operations |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Client["Client"]
    Coordinator["Coordinator Node<br/>(any node can act as coordinator<br/>for a given request)"]

    subgraph Ring["Consistent Hash Ring (Cluster)"]
        NodeA["Node A"]
        NodeB["Node B"]
        NodeC["Node C"]
        NodeD["Node D"]
        NodeE["Node E"]
    end

    subgraph PerNode["Per-Node Storage Engine"]
        MemTable["MemTable<br/>(in-memory, sorted)"]
        WAL["Write-Ahead Log<br/>(durability)"]
        SSTables["SSTables on disk<br/>(immutable, sorted)"]
    end

    subgraph Coordination["Cluster Coordination"]
        Gossip["Gossip Protocol<br/>(membership, failure detection)"]
        HintedHandoff["Hinted Handoff<br/>(temporary failure buffering)"]
        MerkleTree["Merkle Trees<br/>(anti-entropy repair)"]
    end

    Client --> Coordinator
    Coordinator --> NodeA
    Coordinator --> NodeB
    Coordinator --> NodeC

    NodeA -.->|"gossip"| NodeB
    NodeB -.->|"gossip"| NodeC
    NodeC -.->|"gossip"| NodeD
    NodeD -.->|"gossip"| NodeE
    NodeE -.->|"gossip"| NodeA

    NodeA --> MemTable
    MemTable --> WAL
    MemTable -->|"flush when full"| SSTables
```

**Key idea:** There is no single master — **any node can coordinate any request**, data is partitioned via consistent hashing across the ring, and cluster membership/failure detection happens via a decentralized gossip protocol rather than a central coordinator. This is the architecture underlying systems like DynamoDB, Cassandra, and Riak.

---

## 3. Consistent Hashing — Data Partitioning

```mermaid
flowchart TB
    A["Hash Ring (0 to 2^128)"] --> B["Each node owns a range<br/>of the hash space"]
    B --> C["key = hash('user:123')"]
    C --> D["Walk ring clockwise from key's hash<br/>position to find owning node"]
    D --> E["Node found = primary replica owner"]
    E --> F["Next N-1 nodes clockwise<br/>= additional replicas<br/>(replication factor N=3)"]

    G["Node added/removed"] --> H["Only keys in the affected<br/>ring segment need to move<br/>— NOT the entire dataset"]
```

**Why consistent hashing over simple `hash(key) % num_nodes`:** With modulo hashing, adding or removing a single node forces nearly *all* keys to remap (since the modulus changes). Consistent hashing bounds the reshuffling to only the keys in the ring segment adjacent to the changed node — critical for scaling a cluster without massive data movement.

---

## 4. Data Model & Storage Engine (LSM-Tree Based)

```mermaid
sequenceDiagram
    participant C as Client
    participant Node as Storage Node
    participant WAL as Write-Ahead Log
    participant Mem as MemTable (in-memory)
    participant SST as SSTables (on-disk)

    C->>Node: PUT(key, value)
    Node->>WAL: Append write (durability guarantee first)
    WAL-->>Node: Fsync'd to disk
    Node->>Mem: Insert into in-memory sorted structure
    Node-->>C: Write acknowledged

    Note over Mem: MemTable grows until it hits size threshold
    Mem->>SST: Flush to new immutable SSTable file
    Note over SST: Background compaction periodically<br/>merges multiple SSTables,<br/>removing deleted/overwritten keys
```

**Key idea (LSM-Tree):** Writes are always fast because they only ever go to an in-memory structure (after being durably logged) — never requiring in-place disk seeks. Data is later flushed to immutable sorted files (SSTables) and periodically **compacted** in the background to reclaim space and maintain read efficiency.

---

## 5. Write Path — Quorum-Based Replication

```mermaid
sequenceDiagram
    participant C as Client
    participant Coord as Coordinator Node
    participant R1 as Replica 1
    participant R2 as Replica 2
    participant R3 as Replica 3

    C->>Coord: PUT(key, value) with W=2 (write quorum)
    Coord->>R1: Replicate write
    Coord->>R2: Replicate write
    Coord->>R3: Replicate write

    par Replicas respond independently
        R1-->>Coord: Ack
        R2-->>Coord: Ack
    and
        R3-->>Coord: (slow/delayed)
    end

    Note over Coord: W=2 acks received (R1, R2)<br/>— quorum satisfied, don't wait for R3
    Coord-->>C: Write successful

    Note over R3: R3 eventually receives the write<br/>via retry or anti-entropy repair
```

**Key idea:** With replication factor N=3, the coordinator doesn't need all 3 replicas to acknowledge before returning success — just a **write quorum W** (e.g., W=2). This is the core availability/latency tradeoff: waiting for fewer replicas means faster, more available writes, at the cost of temporarily inconsistent replicas that get reconciled later.

---

## 6. Read Path — Quorum-Based Reads & Conflict Resolution

```mermaid
sequenceDiagram
    participant C as Client
    participant Coord as Coordinator Node
    participant R1 as Replica 1
    participant R2 as Replica 2
    participant R3 as Replica 3

    C->>Coord: GET(key) with R=2 (read quorum)
    Coord->>R1: Fetch value
    Coord->>R2: Fetch value

    R1-->>Coord: value=X, version=vector_clock_A
    R2-->>Coord: value=Y, version=vector_clock_B

    alt Versions are consistent
        Coord-->>C: Return value
    else Versions conflict (concurrent writes detected)
        Coord->>Coord: Compare vector clocks
        alt One version causally descends the other
            Coord-->>C: Return the newer version
        else Concurrent, unresolvable automatically
            Coord-->>C: Return BOTH versions<br/>(let application/client resolve)
        end
    end

    Note over Coord: Read-repair: if replicas disagreed,<br/>asynchronously push correct value<br/>to the stale replica
```

**Key idea (tunable consistency):** `R + W > N` guarantees strong consistency (at least one replica in any read quorum overlaps with any write quorum). Choosing `R + W ≤ N` favors availability/latency over strict consistency — this is the "tunable consistency" DynamoDB-style systems are known for, decided per-operation by the client.

---

## 7. Vector Clocks — Detecting Concurrent Writes

```mermaid
flowchart TB
    A["Write 1 by Client A:<br/>value=X, clock={A:1}"] --> B["Write 2 by Client B<br/>(based on Write 1):<br/>value=Y, clock={A:1, B:1}"]
    A --> C["Write 3 by Client C<br/>(also based on Write 1,<br/>concurrently with B):<br/>value=Z, clock={A:1, C:1}"]

    B --> D{"Compare clocks<br/>{A:1,B:1} vs {A:1,C:1}"}
    C --> D
    D --> E["Neither clock dominates the other<br/>→ TRUE CONCURRENT CONFLICT<br/>→ both versions returned to client"]
```

*Vector clocks let the system distinguish "this write happened strictly after that one" (safe to auto-resolve) from "these writes happened concurrently, neither aware of the other" (genuine conflict requiring application-level resolution, e.g., merging a shopping cart's contents from both versions).*

---

## 8. Node Failure Handling — Hinted Handoff

```mermaid
sequenceDiagram
    participant Coord as Coordinator Node
    participant R1 as Replica 1 (healthy)
    participant R2 as Replica 2 (DOWN)
    participant R3 as Replica 3 (healthy)
    participant Hint as Temporary hint holder

    Coord->>R1: Replicate write
    R1-->>Coord: Ack
    Coord->>R2: Replicate write
    Note over R2: Node down, no response
    Coord->>R3: Replicate write
    R3-->>Coord: Ack

    Note over Coord: W=2 satisfied (R1, R3)
    Coord->>Hint: Store "hint" for R2:<br/>{key, value, intended_for: R2}
    Coord-->>Coord: Write still succeeds

    Note over R2: R2 comes back online
    Hint->>R2: Deliver hinted write
    R2->>R2: Apply write, now caught up
```

*Hinted handoff lets the system keep accepting writes at full replication intent even when a replica is temporarily down — a nearby healthy node holds a "hint" (essentially an IOU) and delivers it once the failed node recovers, rather than either blocking the write or permanently under-replicating.*

---

## 9. Anti-Entropy Repair — Merkle Trees

```mermaid
flowchart TB
    A["Two replicas may have<br/>silently diverged over time<br/>(missed hints, etc.)"] --> B["Periodic background<br/>anti-entropy process"]
    B --> C["Each replica builds a<br/>Merkle tree of its key range<br/>(hash tree, leaves = key ranges)"]
    C --> D["Compare root hashes<br/>between two replicas"]
    D --> E{"Root hashes match?"}
    E -- Yes --> F["Replicas are identical<br/>— no repair needed<br/>(cheap, single comparison)"]
    E -- No --> G["Recursively compare child<br/>hashes to find exactly<br/>which key ranges differ"]
    G --> H["Sync only the divergent<br/>key ranges<br/>— not a full data comparison"]
```

*Merkle trees let two replicas efficiently determine whether (and where) their data has diverged by comparing tree hashes top-down, without transferring or comparing every single key — critical for keeping repair traffic manageable across a large dataset.*

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Distributed KV Store HLD))
    Consistent Hash Ring
      Data partitioning
      Bounded reshuffling on scale events
    Coordinator Role
      Any node, per-request
      Manages quorum reads/writes
    Storage Engine (LSM-Tree)
      WAL for durability
      MemTable for fast writes
      SSTables + compaction
    Gossip Protocol
      Decentralized membership
      Failure detection
    Hinted Handoff
      Buffers writes during
      temporary node failure
    Vector Clocks
      Causal ordering
      Concurrent write detection
    Merkle Trees
      Efficient anti-entropy repair
      Detects replica divergence
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Partitioning | Consistent hashing | Bounds data movement to a small ring segment when nodes join/leave, unlike modulo hashing |
| Replication | Configurable N (typically 3), quorum-based R/W | Tunable per-operation tradeoff between latency, availability, and consistency |
| Consistency model | Eventual by default, tunable to strong (R+W>N) | Prioritizes availability (AP over CP) but allows callers to opt into stronger guarantees when needed |
| Storage engine | LSM-Tree (WAL + MemTable + SSTables) | Optimized for extremely high write throughput; sequential disk writes instead of random-access updates |
| Conflict resolution | Vector clocks + application-level merge for true conflicts | Distinguishes safely-resolvable causal ordering from genuine concurrent conflicts that need app logic |
| Failure handling | Hinted handoff (short-term) + Merkle tree repair (long-term) | Keeps the system available during transient failures while ensuring eventual full consistency |
| Cluster membership | Gossip protocol (decentralized) | No single point of failure for membership/failure detection, unlike a central coordinator |

---

## 12. Bottlenecks & Scaling Considerations

- **Hot keys / partition skew** — a single extremely popular key can overwhelm the few nodes that own its hash range; mitigated via key-level caching or splitting hot keys with a sharding suffix (e.g., `key#1`, `key#2`) and merging on read.
- **Compaction overhead** — background SSTable compaction competes for disk I/O with live read/write traffic; must be carefully throttled to avoid latency spikes during compaction cycles.
- **Read amplification in LSM-trees** — a `GET` may need to check the MemTable plus multiple SSTables (until compacted); bloom filters per SSTable are used to quickly skip files that definitely don't contain the key.
- **Vector clock growth** — clocks can grow unbounded with many concurrent writers over time; typically pruned/truncated with a bounded size, trading some historical causal precision for bounded metadata overhead.
- **Gossip convergence time at very large cluster sizes** — thousands of nodes can take longer to converge on membership state changes; often mitigated with gossip fan-out tuning or hierarchical gossip structures.
- **Rebalancing cost during cluster resize** — even with consistent hashing's bounded reshuffling, large-scale node additions still require substantial background data transfer; must be rate-limited to avoid starving live traffic of bandwidth/disk I/O.
- **Cross-datacenter replication** — extending this model globally adds significant write latency if requiring cross-region quorum; typically handled with per-datacenter local quorums plus asynchronous cross-region replication for disaster recovery.
