# Design a Message Queue (Kafka-like) — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Producers publish messages to named topics
- Consumers subscribe to topics and process messages
- Support multiple independent consumer groups reading the same topic
- Preserve message ordering within a partition
- Support both at-least-once and (optionally) exactly-once processing semantics
- Durable storage — messages survive broker restarts

### Non-Functional Requirements
- **High throughput:** Millions of messages/sec across the cluster
- **Low latency:** Sub-10ms produce/consume latency for real-time use cases
- **Durability:** Committed messages must survive broker failure (replication)
- **Horizontal scalability:** Both storage and throughput scale by adding brokers/partitions
- **Ordering guarantee:** Strict ordering only required within a partition, not globally across a topic

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Messages/sec (platform-wide) | ~1M-10M |
| Avg message size | ~1KB |
| Retention period | 7 days (configurable) |
| Topics | Thousands |
| Partitions per topic | 10s-100s, based on throughput needs |
| Replication factor | 3 (typical) |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    subgraph Producers["Producers"]
        P1["Producer 1"]
        P2["Producer 2"]
    end

    subgraph Cluster["Broker Cluster"]
        B1["Broker 1<br/>(Partition 0 leader,<br/>Partition 1 replica)"]
        B2["Broker 2<br/>(Partition 1 leader,<br/>Partition 2 replica)"]
        B3["Broker 3<br/>(Partition 2 leader,<br/>Partition 0 replica)"]
    end

    subgraph Coordination["Cluster Coordination"]
        Controller["Controller<br/>(elected broker,<br/>manages partition leadership)"]
        MetaStore["Metadata Store<br/>(ZooKeeper/KRaft —<br/>topic configs, partition assignments)"]
    end

    subgraph Consumers["Consumer Groups"]
        CG1["Consumer Group A<br/>(3 consumer instances)"]
        CG2["Consumer Group B<br/>(1 consumer instance,<br/>independent offset tracking)"]
    end

    P1 -->|"produce(topic, key, value)"| B1
    P2 -->|"produce(topic, key, value)"| B2

    B1 -.->|"replicate"| B2
    B2 -.->|"replicate"| B3
    B3 -.->|"replicate"| B1

    Controller --> MetaStore
    B1 --> Controller
    B2 --> Controller
    B3 --> Controller

    B1 -->|"consume"| CG1
    B2 -->|"consume"| CG1
    B3 -->|"consume"| CG1

    B1 -->|"consume (independently)"| CG2
```

**Key idea:** A topic is split into **partitions**, each of which is an ordered, append-only log. Each partition has one "leader" broker (handling all reads/writes for it) and several follower replicas. This partitioning is what enables horizontal scaling of both throughput (parallel writes across partitions) and consumption (parallel reads across partitions) — at the cost of only guaranteeing ordering *within* a partition, not across the whole topic.

---

## 3. Core Data Model — The Log Abstraction

```mermaid
flowchart LR
    subgraph Partition0["Partition 0 (ordered, append-only log)"]
        M0["Offset 0"] --> M1["Offset 1"] --> M2["Offset 2"] --> M3["Offset 3"] --> M4["Offset 4<br/>(latest)"]
    end

    Note1["New messages always appended<br/>at the end (offset N+1)"]
    Note2["Consumers track their own<br/>'current offset' position<br/>and read forward from there"]
    Note3["Messages are immutable<br/>once written"]
```

```mermaid
erDiagram
    TOPIC ||--o{ PARTITION : "divided into"
    PARTITION ||--o{ MESSAGE : contains
    CONSUMER_GROUP ||--o{ PARTITION_OFFSET : tracks

    TOPIC {
        string topic_name PK
        int partition_count
        int replication_factor
        int retention_ms
    }
    PARTITION {
        string topic_name FK
        int partition_id
        string leader_broker_id
        list replica_broker_ids
    }
    MESSAGE {
        string topic_name FK
        int partition_id FK
        long offset PK
        string key
        bytes value
        timestamp produced_at
    }
    PARTITION_OFFSET {
        string consumer_group_id FK
        string topic_name FK
        int partition_id FK
        long committed_offset
    }
```

---

## 4. Partitioning Strategy (Key-Based Routing)

```mermaid
flowchart TB
    A["Producer sends message<br/>with key = 'user_123'"] --> B["partition = hash(key) % num_partitions"]
    B --> C["Message always routes to<br/>the SAME partition for<br/>the same key"]
    C --> D["Guarantees: all messages<br/>for user_123 are strictly<br/>ordered relative to each other"]

    E["Message with no key"] --> F["Round-robin or<br/>sticky-random partitioning<br/>(load balancing only,<br/>no ordering guarantee needed)"]
```

**Why key-based partitioning matters:** If you need "all events for user X processed in order," you achieve this not through complex global coordination but simply by routing all of user X's messages to the same partition — ordering is a natural consequence of a single append-only log, not something that needs to be separately engineered.

---

## 5. Producer Publish Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant P as Producer
    participant Meta as Metadata Cache (client-side)
    participant Leader as Partition Leader Broker
    participant F1 as Follower Replica 1
    participant F2 as Follower Replica 2

    P->>Meta: Which broker leads partition for this key?
    Meta-->>P: Broker 1 (cached, refreshed periodically)

    P->>Leader: Produce message (topic, partition, key, value)
    Leader->>Leader: Append to local log

    alt acks=all (strongest durability)
        Leader->>F1: Replicate
        Leader->>F2: Replicate
        F1-->>Leader: Ack
        F2-->>Leader: Ack
        Leader-->>P: Ack (committed to quorum)
    else acks=1 (leader only)
        Leader-->>P: Ack (written to leader only,<br/>not yet replicated — faster but riskier)
    else acks=0 (fire and forget)
        Leader-->>P: No ack — fastest, least safe
    end
```

*The `acks` setting is a direct latency/durability tradeoff knob exposed to the producer — critical financial events might use `acks=all`, while high-volume low-stakes telemetry might use `acks=1` for throughput.*

---

## 6. Consumer Group Semantics

```mermaid
flowchart TB
    A["Topic with 6 partitions"] --> B["Consumer Group A<br/>(3 consumer instances)"]
    B --> C["Consumer A1: Partitions 0, 1"]
    B --> D["Consumer A2: Partitions 2, 3"]
    B --> E["Consumer A3: Partitions 4, 5"]

    A --> F["Consumer Group B<br/>(1 consumer instance,<br/>independent of Group A)"]
    F --> G["Consumer B1: ALL partitions 0-5<br/>(separate offset tracking)"]

    Note1["Each partition is consumed by<br/>EXACTLY ONE consumer within a group<br/>— this is how parallelism is achieved<br/>while preserving per-partition ordering"]
    Note2["Different consumer groups are<br/>fully independent — each maintains<br/>its own offset position,<br/>enabling multiple 'readers'<br/>of the same stream for different purposes"]
```

**Why consumer groups matter:** This is what lets the same event stream serve multiple independent purposes simultaneously — e.g., one consumer group updates a search index from order events, another triggers email notifications, and a third feeds an analytics pipeline — all reading the same partitions at their own pace without interfering with each other.

---

## 7. Consumer Rebalancing (Group Membership Changes)

```mermaid
sequenceDiagram
    participant C1 as Consumer 1
    participant C2 as Consumer 2
    participant C3 as Consumer 3 (new, joining)
    participant GC as Group Coordinator (broker)

    Note over C1,C2: Initially: C1 owns partitions 0-2,<br/>C2 owns partitions 3-5

    C3->>GC: JoinGroup request
    GC->>GC: Detect membership change,<br/>trigger rebalance

    GC->>C1: Revoke current partition assignments
    GC->>C2: Revoke current partition assignments
    C1->>C1: Commit current offsets before giving up partitions
    C2->>C2: Commit current offsets

    GC->>GC: Recompute assignment:<br/>C1: partitions 0-1<br/>C2: partitions 2-3<br/>C3: partitions 4-5

    GC->>C1: New assignment: 0-1
    GC->>C2: New assignment: 2-3
    GC->>C3: New assignment: 4-5

    Note over C1,C3: Each consumer resumes from<br/>its last COMMITTED offset for<br/>its newly assigned partitions —<br/>no message loss, but a brief<br/>pause during rebalance
```

---

## 8. Delivery Semantics — At-Least-Once vs Exactly-Once

```mermaid
flowchart TB
    A["Consumer processes message"] --> B{"When is offset committed?"}
    B --> C["Commit BEFORE processing"]
    B --> D["Commit AFTER processing<br/>(at-least-once, most common)"]
    B --> E["Atomic commit WITH processing<br/>(exactly-once, transactional)"]

    C --> C1["Risk: crash after commit,<br/>before processing = message LOST"]
    D --> D1["Risk: crash after processing,<br/>before commit = message<br/>REPROCESSED on restart<br/>(duplicate, but not lost)"]
    E --> E1["Requires transactional writes<br/>spanning the message consumption<br/>AND the side-effect (e.g., DB write)<br/>— complex but eliminates both risks"]

    F["Most systems choose D<br/>+ idempotent consumer logic —<br/>simpler than true exactly-once,<br/>same practical outcome"]
```

---

## 9. Replication & Leader Election (Broker Failure Handling)

```mermaid
sequenceDiagram
    participant Leader as Partition Leader (Broker 1)
    participant F1 as Follower (Broker 2)
    participant F2 as Follower (Broker 3)
    participant Controller as Cluster Controller

    Note over Leader: Broker 1 crashes

    Controller->>Controller: Detect failure<br/>(missed heartbeat)
    Controller->>Controller: Check In-Sync Replica (ISR) set<br/>for this partition: {Broker 2, Broker 3}
    Controller->>F1: Elect Broker 2 as new leader<br/>(was fully caught up — in ISR)

    F1->>F1: Becomes new partition leader

    Controller->>Controller: Update metadata:<br/>partition leader = Broker 2

    Note over F2: Broker 3 continues<br/>replicating from new leader (Broker 2)

    Note over Leader: Broker 1 eventually recovers
    Leader->>F1: Rejoins as a follower,<br/>catches up from new leader
```

**Why the In-Sync Replica (ISR) set matters:** Only replicas that are fully caught up with the leader (in the ISR) are eligible for leader election on failure. This prevents data loss — electing a replica that had fallen behind would silently drop the most recent committed messages.

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Message Queue HLD))
    Broker
      Stores partition logs
      Serves produce/consume requests
      Replicates to followers
    Controller
      Elected from broker pool
      Manages partition leadership
      Handles failover
    Metadata Store
      Topic/partition configuration
      Broker cluster membership
      ZooKeeper or built-in Raft (KRaft)
    Producer Client
      Key-based partition routing
      Configurable ack levels
    Consumer Group Coordinator
      Partition assignment
      Rebalance orchestration
    Offset Store
      Per-group, per-partition position
      Enables independent consumption
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Ordering guarantee | Per-partition only, not global | Global ordering would require serializing all writes through one point, destroying horizontal scalability |
| Partitioning key | Hash of message key | Ensures related messages (same entity) land in the same partition, achieving ordering where it matters without extra coordination |
| Durability | Configurable acks (0/1/all) + replication factor | Exposes an explicit latency/durability tradeoff knob per use case, rather than a one-size-fits-all guarantee |
| Consumer scaling | Consumer groups with partition-exclusive assignment | Enables parallel consumption while preserving per-partition order; multiple independent groups allow multiple use cases on one stream |
| Failover | ISR-based leader election | Only promotes replicas guaranteed to have all committed data — prevents silent data loss on failover |
| Delivery semantics | At-least-once + idempotent consumers (default) | True exactly-once is complex/costly; at-least-once with dedup achieves the same practical safety far more simply |

---

## 12. Bottlenecks & Scaling Considerations

- **Partition count as the scaling lever** — throughput scales with partition count (more parallelism), but too many partitions per broker increases overhead (open file handles, replication traffic, longer leader election); needs to be sized deliberately, not maximized blindly.
- **Hot partitions from skewed keys** — if one key (e.g., a viral user_id) dominates traffic, its partition becomes a bottleneck regardless of overall cluster capacity; may require key salting or dedicated handling for known hot entities.
- **Consumer lag under processing slowdowns** — if consumers can't keep up with producers, lag grows; needs monitoring and either consumer scale-out (more instances up to partition count) or backpressure on producers.
- **Rebalance storms** — frequent consumer join/leave (e.g., flaky deployments) triggers repeated rebalances, each pausing consumption briefly; incremental/cooperative rebalancing protocols reduce this pause compared to full stop-the-world rebalances.
- **Replication bandwidth** — a replication factor of 3 triples the network/disk write load per message; this is the direct cost of durability and must be budgeted into cluster capacity planning.
- **Retention vs storage cost** — long retention windows (weeks/months) on high-volume topics require substantial disk capacity; tiered storage (moving older segments to cheaper object storage) is a common mitigation in modern systems.
- **Cross-datacenter replication** — for disaster recovery or multi-region access, replicating entire topics across regions adds significant complexity around consistency and conflict handling, usually solved with a separate mirroring/replication tool rather than the core broker protocol itself.
