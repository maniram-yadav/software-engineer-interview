# Design a Multi-Region System with Strong Consistency — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Data must be replicated across multiple geographic regions for disaster recovery and read locality
- Writes must be linearizable/strongly consistent — a read after a committed write must never see stale data, regardless of which region serves it
- System must remain available for reads even if one region fails
- Support cross-region transactions where needed (e.g., financial transfers)

### Non-Functional Requirements
- **Consistency:** Strong/linearizable consistency is a hard requirement (this is the defining constraint of the whole design)
- **Availability:** Must tolerate a full region outage without data loss, though CAP theorem means write availability may be sacrificed during partitions
- **Latency:** Cross-region consensus inherently adds latency (speed of light is a real constraint) — must be minimized, not eliminated
- **Durability:** No committed write can ever be lost, even in a regional disaster

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Regions | 3-5 (e.g., US-East, US-West, EU, APAC) |
| Inter-region latency | 50-150ms depending on geographic distance |
| Writes/sec (global) | ~100,000 |
| Consensus round-trip overhead | Directly bounded by cross-region network latency |
| Read latency target (local region) | < 10ms if served locally |

---

## 2. The Core Tension — CAP Theorem in Practice

```mermaid
flowchart TB
    A["Network Partition Occurs<br/>Between Regions"] --> B{"System Must Choose"}
    B --> C["Consistency (CP)<br/>Reject writes on the<br/>minority side to avoid<br/>diverging data"]
    B --> D["Availability (AP)<br/>Accept writes on both sides,<br/>reconcile conflicts later"]

    C --> C1["This design's choice —<br/>strong consistency required,<br/>availability sacrificed<br/>during partition"]
    D --> D1["Not chosen here —<br/>appropriate for systems like<br/>DynamoDB/Cassandra where<br/>eventual consistency is acceptable"]

    E["Note: Under NORMAL operation<br/>(no partition), the system is<br/>both consistent AND available —<br/>CAP only forces a choice<br/>DURING an actual partition"]
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph RegionUS["Region: US-East"]
        AppUS["Application Servers"]
        ReplicaUS["Data Replica (Follower)"]
    end

    subgraph RegionEU["Region: EU-West"]
        AppEU["Application Servers"]
        ReplicaEU["Data Replica (Follower)"]
    end

    subgraph RegionAPAC["Region: APAC"]
        AppAPAC["Application Servers"]
        LeaderAPAC["Data Replica<br/>(Current LEADER)"]
    end

    subgraph Consensus["Consensus Layer (Cross-Region)"]
        ConsensusGroup["Raft/Paxos Consensus Group<br/>spanning all regions"]
    end

    ClientUS["Client (US)"] --> AppUS
    ClientEU["Client (EU)"] --> AppEU
    ClientAPAC["Client (APAC)"] --> AppAPAC

    AppUS -->|"Reads: local replica<br/>Writes: forward to leader"| ReplicaUS
    AppEU -->|"Reads: local replica<br/>Writes: forward to leader"| ReplicaEU
    AppAPAC -->|"Reads/Writes: local<br/>(leader is here)"| LeaderAPAC

    AppUS -.->|"Write forwarded"| LeaderAPAC
    AppEU -.->|"Write forwarded"| LeaderAPAC

    LeaderAPAC <--> ConsensusGroup
    ReplicaUS <--> ConsensusGroup
    ReplicaEU <--> ConsensusGroup
```

**Key idea:** A single logical dataset has one leader at a time (residing in one region), with synchronously-replicated followers in other regions via a consensus protocol. Writes from any region must reach the leader and achieve quorum acknowledgment before being considered committed — this is what enforces strong consistency at the cost of write latency for regions far from the leader.

---

## 4. Strongly Consistent Write Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant ClientEU as Client (EU region)
    participant AppEU as App Server (EU)
    participant LeaderAPAC as Leader (APAC region)
    participant FollowerUS as Follower (US region)
    participant FollowerEU as Follower (EU region)

    ClientEU->>AppEU: Write request
    AppEU->>LeaderAPAC: Forward write to leader<br/>(cross-region hop)

    LeaderAPAC->>LeaderAPAC: Append to local log
    LeaderAPAC->>FollowerUS: Replicate (cross-region)
    LeaderAPAC->>FollowerEU: Replicate (cross-region)

    FollowerUS-->>LeaderAPAC: Ack
    FollowerEU-->>LeaderAPAC: Ack

    Note over LeaderAPAC: Quorum achieved<br/>(majority of replicas acked)
    LeaderAPAC->>LeaderAPAC: Mark write COMMITTED

    LeaderAPAC-->>AppEU: Write acknowledged
    AppEU-->>ClientEU: Success

    Note over ClientEU: Total latency = round trip<br/>to leader + quorum replication —<br/>unavoidably includes<br/>cross-region network hops
```

**Why this is inherently slower than a single-region system:** Strong consistency requires waiting for a quorum of geographically distributed replicas to acknowledge before confirming a write — there's no way around the physical network latency this requires. This is a deliberate, unavoidable tradeoff, not an implementation inefficiency.

---

## 5. Read Path — Consistency Options

```mermaid
flowchart TB
    A["Read request arrives<br/>in Region X"] --> B{"Consistency requirement<br/>for this read?"}

    B --> C["Linearizable Read<br/>(must reflect latest committed write)"]
    C --> C1["Must be served by (or confirmed with)<br/>the current leader —<br/>incurs cross-region latency<br/>if leader is elsewhere"]

    B --> D["Bounded Staleness Read<br/>(acceptable to be slightly behind)"]
    D --> D1["Can be served from local<br/>regional replica —<br/>fast, no cross-region hop,<br/>but data may be milliseconds stale"]

    B --> E["Read-Your-Writes<br/>(user must see their own<br/>recent writes, but not<br/>necessarily others')"]
    E --> E1["Route read to whichever<br/>replica the user's own<br/>write was committed through,<br/>or track a session token"]
```

*Most real-world systems don't uniformly require full linearizability for every read — offering tiered read consistency options (with clear tradeoffs) lets different use cases within the same system choose the right latency/consistency point for their needs.*

---

## 6. Leader Placement & Failover

```mermaid
sequenceDiagram
    participant LeaderAPAC as Leader (APAC)
    participant FollowerUS as Follower (US)
    participant FollowerEU as Follower (EU)
    participant ConsensusGroup as Consensus Group

    Note over LeaderAPAC: APAC region suffers<br/>a full outage

    FollowerUS->>ConsensusGroup: Detects missed heartbeats<br/>from leader
    FollowerEU->>ConsensusGroup: Same detection

    ConsensusGroup->>ConsensusGroup: Initiate leader election<br/>among remaining regions

    FollowerUS->>FollowerEU: Request votes
    FollowerEU->>FollowerUS: Grant vote<br/>(if FollowerUS's log is at least as up-to-date)

    Note over FollowerUS: Achieves majority vote
    FollowerUS->>FollowerUS: Becomes new leader

    ConsensusGroup->>ConsensusGroup: Update routing:<br/>all writes now forward to US region

    Note over LeaderAPAC: When APAC recovers,<br/>rejoins as a follower,<br/>catches up via replicated log —<br/>no data loss since only<br/>COMMITTED (quorum-acked)<br/>writes were ever acknowledged<br/>to clients
```

**Why no data loss on failover:** Because writes are only acknowledged to clients after achieving quorum (majority) replication, any write the client believes succeeded is guaranteed to exist on a majority of replicas — including at least one that participates in electing the new leader. A minority-replicated, unacknowledged write might be lost, but the client was never told it succeeded in the first place.

---

## 7. Leader Placement Strategy (Minimizing Latency Impact)

```mermaid
flowchart TB
    A["Where should the leader<br/>be placed?"] --> B{"Strategy"}
    B --> C["Static: always in one<br/>designated 'primary' region"]
    B --> D["Dynamic: migrate leader<br/>toward the region with<br/>the most write traffic"]
    B --> E["Sharded: different data<br/>shards have leaders in<br/>different regions<br/>(geo-partitioned by data locality)"]

    E --> E1["e.g., European users' data<br/>has its leader in EU region —<br/>writes for that shard are<br/>fast for EU users, slower<br/>for US users accessing it"]
    E1 --> E2["Most practical approach<br/>when data has natural<br/>geographic affinity<br/>(e.g., user's own data)"]
```

---

## 8. Cross-Region Transactions (Multi-Shard Atomicity)

```mermaid
sequenceDiagram
    participant Client as Client
    participant Coordinator as Transaction Coordinator
    participant ShardA as Shard A (leader in US)
    participant ShardB as Shard B (leader in EU)

    Client->>Coordinator: Begin transaction:<br/>transfer $100 from<br/>Account A (Shard A) to<br/>Account B (Shard B)

    Coordinator->>ShardA: Prepare: debit $100
    Coordinator->>ShardB: Prepare: credit $100

    ShardA-->>Coordinator: Prepared (locked, not yet committed)
    ShardB-->>Coordinator: Prepared (locked, not yet committed)

    alt Both prepared successfully
        Coordinator->>ShardA: Commit
        Coordinator->>ShardB: Commit
        ShardA-->>Coordinator: Committed
        ShardB-->>Coordinator: Committed
        Coordinator-->>Client: Transaction success
    else Either fails to prepare
        Coordinator->>ShardA: Abort
        Coordinator->>ShardB: Abort
        Coordinator-->>Client: Transaction failed, rolled back
    end
```

*This is a classic two-phase commit (2PC) pattern layered on top of the per-shard consensus groups — necessary when a single logical transaction must atomically span data with leaders in different regions/shards, at the cost of additional latency and coordinator complexity.*

---

## 9. Component Responsibilities Summary

```mermaid
mindmap
  root((Multi-Region Strong Consistency HLD))
    Consensus Group
      Cross-region Raft/Paxos
      Quorum-based commit
      Leader election on failure
    Leader Replica
      Handles all writes for its shard
      Replicates to followers
    Follower Replicas
      Serve local reads
      Participate in quorum acks
      Available for leader election
    Transaction Coordinator
      Two-phase commit across shards
      Cross-region atomicity
    Routing Layer
      Forwards writes to current leader
      Routes reads per consistency tier
```

---

## 10. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Consistency model | CP (favor consistency over availability during partition) | Explicit requirement — strong consistency is non-negotiable for this system's use case |
| Replication protocol | Consensus-based (Raft/Paxos) with quorum commit | Only mechanism that guarantees no data loss and no split-brain across regions |
| Leader placement | Sharded/geo-partitioned by data locality where possible | Minimizes cross-region latency for the common case where data has a natural "home" region |
| Read consistency | Tiered options (linearizable / bounded-staleness / read-your-writes) | Not every read needs full linearizability; offering options avoids paying maximum latency cost universally |
| Cross-shard transactions | Two-phase commit coordinator | Necessary for atomicity when a transaction spans shards with different regional leaders |
| Failover | Automatic leader re-election via consensus | Ensures continued availability (for writes) after a regional outage, without manual intervention |

---

## 11. Bottlenecks & Scaling Considerations

- **Cross-region write latency is fundamental, not fixable** — no architecture eliminates the speed-of-light cost of achieving quorum across geographically distant regions; the only lever is *minimizing which writes need to cross regions* (via smart sharding/leader placement).
- **Leader region becomes a hotspot** — all writes for a given shard funnel through one region; if that region's traffic grows disproportionately, may need finer-grained sharding to distribute leadership more evenly.
- **Quorum size vs fault tolerance tradeoff** — a 5-region deployment requiring 3-of-5 quorum tolerates 2 simultaneous region failures; fewer regions or a stricter quorum requirement reduces this tolerance — this is a deliberate capacity planning decision.
- **Two-phase commit blocking** — if the transaction coordinator crashes mid-transaction (after "prepare" but before "commit/abort"), participating shards can be left holding locks indefinitely; production systems need coordinator failure recovery (e.g., a backup coordinator that can resume from a durable transaction log).
- **Read replica staleness monitoring** — for bounded-staleness reads, the system must actively track and expose replication lag so applications can make informed decisions about which consistency tier to request.
- **Network partition duration** — a prolonged partition means the leader's region (if it's on the minority side) becomes fully unavailable for writes; this is the direct, accepted cost of choosing CP over AP, and must be clearly communicated to downstream consumers/SLAs.
- **Testing and chaos engineering** — a system this reliant on correct consensus behavior under failure needs extensive fault-injection testing (simulated partitions, leader crashes) since these failure paths are rare in normal operation but catastrophic if subtly broken.
