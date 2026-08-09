# Design a Globally Distributed Database (Spanner-style) — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Store and serve data across multiple continents/regions
- Support strongly consistent, externally consistent transactions spanning multiple rows/shards, even across regions
- Provide SQL-like query capabilities with strict schema (unlike typical NoSQL eventually-consistent stores)
- Automatic sharding and rebalancing of data across nodes

### Non-Functional Requirements
- **External consistency (the defining property):** If transaction T1 commits before transaction T2 starts (in real, wall-clock time), then T1's effects must be visible to T2 — this is a STRONGER guarantee than plain linearizability, because it holds across the entire globally distributed system, not just a single register
- **Global scale:** Petabytes of data, millions of QPS, spanning many datacenters worldwide
- **High availability:** Survives datacenter and regional failures via synchronous replication
- **Horizontal scalability:** Both storage and throughput must scale by adding more machines/shards

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Regions | 5+ globally distributed |
| Replicas per shard | 3-5 (spread across regions/zones) |
| Clock uncertainty bound (TrueTime-style) | Single-digit milliseconds |
| Cross-region commit latency | Tens to low hundreds of ms, bounded by network + clock uncertainty wait |
| Read latency (local, snapshot) | Single-digit ms |

---

## 2. The Core Innovation — TrueTime (Bounded Clock Uncertainty)

```mermaid
flowchart TB
    A["Traditional distributed systems<br/>treat physical clocks as UNTRUSTWORTHY<br/>for ordering — hence relying purely<br/>on logical mechanisms like vector<br/>clocks or consensus-ordered logs"] --> B["Spanner's key insight:<br/>instead of ignoring physical time,<br/>make clock UNCERTAINTY an explicit,<br/>API-exposed value"]

    B --> C["TrueTime API: TT.now()<br/>returns an INTERVAL [earliest, latest],<br/>not a single timestamp —<br/>an honest admission that<br/>'the real time right now is<br/>SOMEWHERE in this range'"]

    C --> D["Uncertainty bound (ε) kept small<br/>via GPS + atomic clocks in each<br/>datacenter, continuously<br/>cross-checked — typically<br/>just a few milliseconds"]

    D --> E["This bounded uncertainty is<br/>what enables Spanner to safely<br/>use physical timestamps for<br/>global transaction ordering,<br/>something normally considered<br/>unsafe in distributed systems"]
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph GlobalLayer["Global Coordination"]
        TrueTime["TrueTime Service<br/>(GPS + Atomic Clocks,<br/>per-datacenter)"]
    end

    subgraph RegionUS["Region: US"]
        SpannerUS["Spanserver Group<br/>(Paxos-replicated shard leaders)"]
        DataUS[("Data Shards<br/>(tablets)")]
    end

    subgraph RegionEU["Region: EU"]
        SpannerEU["Spanserver Group"]
        DataEU[("Data Shards")]
    end

    subgraph RegionAPAC["Region: APAC"]
        SpannerAPAC["Spanserver Group"]
        DataAPAC[("Data Shards")]
    end

    ClientUS["Client (US)"] --> SpannerUS
    ClientEU["Client (EU)"] --> SpannerEU
    ClientAPAC["Client (APAC)"] --> SpannerAPAC

    SpannerUS --> DataUS
    SpannerEU --> DataEU
    SpannerAPAC --> DataAPAC

    SpannerUS <-->|"Paxos replication<br/>(per-shard groups)"| SpannerEU
    SpannerEU <-->|"Paxos replication"| SpannerAPAC
    SpannerUS <-->|"Paxos replication"| SpannerAPAC

    TrueTime -.->|"Timestamp API"| SpannerUS
    TrueTime -.->|"Timestamp API"| SpannerEU
    TrueTime -.->|"Timestamp API"| SpannerAPAC
```

**Key idea:** Data is broken into shards (Spanner calls them "tablets"), each replicated via its own Paxos group spanning multiple regions. Every read/write transaction gets a globally meaningful timestamp derived from TrueTime, which is what allows the entire globally-distributed system — not just one shard — to maintain a single, consistent, real-time-respecting order of all transactions.

---

## 4. Data Model & Sharding

```mermaid
erDiagram
    TABLE {
        string primary_key PK
        map columns
    }
    TABLET {
        string tablet_id PK
        string key_range_start
        string key_range_end
        list replica_locations
    }
```

```mermaid
flowchart TB
    A["Table with billions of rows,<br/>ordered by primary key"] --> B["Split into contiguous<br/>key-range tablets"]
    B --> C["Tablet 1: keys A-F"]
    B --> D["Tablet 2: keys G-M"]
    B --> E["Tablet 3: keys N-Z"]

    C & D & E --> F["Each tablet independently<br/>replicated via its OWN<br/>Paxos group across<br/>multiple regions"]

    G["Automatic resharding:<br/>if Tablet 2 grows too large<br/>or too hot, it's automatically<br/>split into two smaller tablets,<br/>each getting its own Paxos group"]
```

---

## 5. Read-Write Transaction Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Client as Client
    participant Coordinator as Transaction Coordinator<br/>(a Paxos leader, one of the shards)
    participant ShardA as Shard A Leader
    participant ShardB as Shard B Leader
    participant TT as TrueTime

    Client->>Coordinator: Begin transaction<br/>(reads/writes spanning Shard A and B)

    Coordinator->>ShardA: Acquire locks for<br/>affected rows
    Coordinator->>ShardB: Acquire locks for<br/>affected rows
    ShardA-->>Coordinator: Locks acquired
    ShardB-->>Coordinator: Locks acquired

    Coordinator->>TT: TT.now()
    TT-->>Coordinator: Interval [earliest, latest]

    Coordinator->>Coordinator: Choose commit timestamp s<br/>= latest value in the interval<br/>(ensures s is >= true current time)

    Note over Coordinator: COMMIT WAIT: wait until<br/>TT.now().earliest > s<br/>(guarantees enough real time<br/>has passed that timestamp s<br/>is now definitely in the past<br/>for EVERY node globally,<br/>accounting for clock uncertainty)

    Coordinator->>ShardA: Commit with timestamp s
    Coordinator->>ShardB: Commit with timestamp s
    ShardA-->>Coordinator: Committed
    ShardB-->>Coordinator: Committed

    Coordinator->>ShardA: Release locks
    Coordinator->>ShardB: Release locks
    Coordinator-->>Client: Transaction committed
```

**Why "Commit Wait" is the critical, unique step:** This deliberate pause — waiting out the clock uncertainty window before acknowledging the commit — is precisely what guarantees external consistency. It ensures that by the time any client can possibly learn the transaction committed, that transaction's timestamp is guaranteed to be in the past everywhere in the system, so any subsequent transaction (anywhere globally) will be assigned a strictly later timestamp. This small latency cost (roughly the clock uncertainty bound, typically a few milliseconds) buys a very strong global ordering guarantee.

---

## 6. Snapshot Read Flow (Lock-Free Reads at a Timestamp)

```mermaid
sequenceDiagram
    participant Client as Client
    participant Shard as Shard Leader/Replica
    participant TT as TrueTime

    Client->>Shard: Read (snapshot at current time)
    Shard->>TT: TT.now()
    TT-->>Shard: Interval [earliest, latest]
    Shard->>Shard: Use timestamp = latest<br/>as the snapshot read timestamp

    Shard->>Shard: Read the version of each row<br/>as of that timestamp<br/>(multi-version storage —<br/>keeps historical versions)

    Note over Shard: NO LOCKS REQUIRED —<br/>reading a consistent<br/>snapshot at a fixed<br/>timestamp never conflicts<br/>with concurrent writes,<br/>since writes create NEW<br/>versions rather than<br/>overwriting in place
```

**Why lock-free reads matter for scale:** Because the underlying storage is multi-versioned (every write creates a new timestamped version rather than overwriting), reads at a specific timestamp never need to block on or coordinate with concurrent writes — this is what allows Spanner to serve enormous read volumes without read-side lock contention, while still guaranteeing a perfectly consistent snapshot view.

---

## 7. Paxos Replication Within a Shard

```mermaid
flowchart TB
    A["Shard (Tablet) has 5 replicas<br/>spread across regions:<br/>2 in US, 2 in EU, 1 in APAC"] --> B["One replica is the<br/>Paxos LEADER for this shard<br/>(via the same leader-election<br/>mechanism from the<br/>Distributed Consensus design)"]
    B --> C["All writes to this shard's<br/>key range go through<br/>its leader, replicated to<br/>a majority (3 of 5) before commit"]

    D["Leader placement matters:<br/>if most traffic for this<br/>shard comes from US users,<br/>placing the leader in a US<br/>region minimizes latency<br/>for the common case —<br/>same geo-partitioning<br/>principle as the Multi-Region<br/>Strong Consistency design"]
```

---

## 8. Multi-Version Storage Layer

```mermaid
flowchart TB
    A["Row with primary_key='user_123'"] --> B["Version history<br/>(NOT overwritten in place)"]
    B --> C["Timestamp T1: {name:'Alice'}"]
    B --> D["Timestamp T2: {name:'Alice', city:'NYC'}"]
    B --> E["Timestamp T3 (latest):<br/>{name:'Alice', city:'Boston'}"]

    F["Read at timestamp T2"] --> G["Returns: {name:'Alice', city:'NYC'}<br/>— exactly the state as it<br/>existed at that moment,<br/>regardless of later writes"]

    H["Old versions eventually<br/>garbage collected after<br/>a configured retention window<br/>(balances historical query<br/>capability against storage cost)"]
```

---

## 9. Cross-Shard Transaction Coordination (Two-Phase Commit + Paxos)

```mermaid
flowchart TB
    A["Transaction spans Shard A<br/>(leader in US) and<br/>Shard B (leader in EU)"] --> B["One shard's leader acts as<br/>the Transaction Coordinator<br/>(typically the shard with<br/>the most participants, or<br/>a designated rule)"]

    B --> C["Standard 2PC protocol:<br/>Prepare phase across<br/>all participating shards"]
    C --> D["Each shard's Paxos group<br/>independently REPLICATES<br/>the prepare/commit decision<br/>to its own replicas —<br/>so even if the coordinator<br/>crashes mid-transaction,<br/>the decision survives via<br/>each shard's own consensus log"]

    E["This layered design —<br/>2PC for cross-shard atomicity,<br/>Paxos for within-shard<br/>durability/availability —<br/>is what lets Spanner survive<br/>coordinator failures without<br/>losing in-flight transaction<br/>state"] --> D
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Globally Distributed DB HLD))
    TrueTime Service
      GPS + atomic clock sync
      Exposes bounded uncertainty interval
      Enables commit-wait guarantee
    Spanserver (Shard Leader)
      Handles reads/writes for its tablet
      Paxos-replicated across regions
    Tablet
      Contiguous key-range shard
      Auto-splits when too large/hot
    Transaction Coordinator
      Orchestrates cross-shard 2PC
      Assigns global commit timestamp
    Multi-Version Storage
      Timestamped row versions
      Enables lock-free snapshot reads
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Clock model | TrueTime — bounded uncertainty interval, not a trusted single value | Enables safe use of physical timestamps for global ordering, something normally avoided in distributed systems due to clock skew risk |
| Consistency guarantee | External consistency (stronger than plain linearizability) | Real-world ordering (if T1 commits before T2 starts in wall-clock time) is preserved globally, not just per-object |
| Commit protocol | Commit-wait based on clock uncertainty | The deliberate latency cost of waiting out the uncertainty window is what makes external consistency achievable at all |
| Replication | Paxos per-shard, spanning regions | Provides both durability (survives node failure) and geographic distribution (survives regional failure) |
| Read model | Multi-version storage, lock-free snapshot reads | Avoids read-write lock contention entirely, critical for sustaining massive global read throughput |
| Sharding | Automatic tablet splitting | Scales horizontally without manual intervention as data grows or hotspots emerge |

---

## 12. Bottlenecks & Scaling Considerations

- **Commit-wait latency is a direct, unavoidable cost** — every read-write transaction pays a latency penalty proportional to the clock uncertainty bound; this is why enormous engineering investment (dedicated GPS/atomic clock hardware) goes into keeping that uncertainty window as small as possible (single-digit milliseconds) — it directly determines write latency floor.
- **Cross-shard transactions are inherently more expensive** — 2PC across shards with different regional leaders incurs multiple cross-region round trips; application/schema design should minimize transactions that need to span shards where possible (similar lesson to the Multi-Region Strong Consistency design).
- **Leader placement affects regional latency asymmetrically** — a shard's leader being in one region means writes from other regions always pay that cross-region cost; careful data locality-aware sharding (placing a shard's leader near where most of its traffic originates) mitigates this.
- **Multi-version storage growth** — retaining historical versions for snapshot reads increases storage requirements; needs a garbage collection policy balancing how far back point-in-time queries should be supported against storage cost.
- **TrueTime infrastructure dependency** — this entire consistency model depends on genuinely reliable, low-uncertainty time infrastructure in every datacenter; this is a substantial operational investment that smaller-scale systems typically can't justify, which is part of why this architecture is associated with hyperscale infrastructure specifically.
- **Read-only transaction optimization** — because snapshot reads don't need locks, read-heavy workloads scale extremely well horizontally by simply adding more replicas to serve reads, decoupled entirely from write throughput scaling (which remains bounded by per-shard Paxos leader capacity).
