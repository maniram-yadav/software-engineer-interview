# Design a Distributed Consensus System (Raft/Paxos) — High-Level Design Document

## 1. Requirements

### Functional Requirements
- A cluster of nodes must agree on a single sequence of values/commands (a replicated log), despite failures
- Any node can propose a value; the system must converge on exactly one accepted value per log position
- Support leader-based writes for efficiency, with automatic leader election on failure
- All committed entries must eventually be visible identically on every healthy node

### Non-Functional Requirements
- **Safety (the paramount property):** Never allow two different values to be committed for the same log position — under ANY sequence of failures, message delays, or network partitions
- **Liveness:** The cluster must be able to make progress (elect a leader, commit new entries) as long as a majority of nodes are up and can communicate
- **Fault tolerance:** Tolerates up to `(N-1)/2` node failures in a cluster of N nodes
- **No split-brain:** At most one leader can be active and able to commit entries at any given time

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Typical cluster size | 3, 5, or 7 nodes (odd, for clean majorities) |
| Fault tolerance (N=5) | Tolerates 2 simultaneous node failures |
| Log replication latency | Bounded by round-trip to a majority of nodes |
| Leader election timeout | Randomized, typically 150-300ms range |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    subgraph Cluster["Consensus Cluster (5 nodes)"]
        Leader["Node 1: LEADER"]
        F1["Node 2: Follower"]
        F2["Node 3: Follower"]
        F3["Node 4: Follower"]
        F4["Node 5: Follower"]
    end

    Client["Client"]

    Client -->|"Propose command"| Leader
    Leader -->|"AppendEntries RPC"| F1
    Leader -->|"AppendEntries RPC"| F2
    Leader -->|"AppendEntries RPC"| F3
    Leader -->|"AppendEntries RPC"| F4

    F1 -.->|"Ack"| Leader
    F2 -.->|"Ack"| Leader
    F3 -.->|"Ack"| Leader
    F4 -.->|"Ack"| Leader

    Note1["Once majority (3 of 5) ack,<br/>entry is COMMITTED"]
```

**Key idea:** Exactly one node is the leader at any time; all client writes go through it. The leader replicates each new log entry to followers and only considers it "committed" once a **majority** have durably stored it — this majority requirement is the mathematical foundation that makes the whole system safe under partial failure.

---

## 3. Node States & Transitions (Raft's Core State Machine)

```mermaid
stateDiagram-v2
    [*] --> Follower
    Follower --> Candidate: Election timeout<br/>(no heartbeat from leader)
    Candidate --> Leader: Wins majority of votes
    Candidate --> Follower: Discovers current leader<br/>OR higher term seen
    Candidate --> Candidate: Election timeout again<br/>(split vote, retry)
    Leader --> Follower: Discovers node with<br/>higher term (was partitioned,<br/>a new leader was elected)
```

*Every node is always in exactly one of these three states. This simplicity — just three possible roles with clear transition triggers — is a large part of why Raft is considered more understandable than the original Paxos formulation, while providing equivalent safety guarantees.*

---

## 4. Leader Election — Detailed Sequence

```mermaid
sequenceDiagram
    participant N1 as Node 1
    participant N2 as Node 2
    participant N3 as Node 3
    participant N4 as Node 4
    participant N5 as Node 5

    Note over N1,N5: Current leader (N1) crashes

    Note over N2: N2's election timeout fires first<br/>(randomized timeout — reduces<br/>chance of simultaneous elections)
    N2->>N2: Increment term (term=5)<br/>Transition to CANDIDATE<br/>Vote for self

    N2->>N3: RequestVote(term=5, candidate=N2,<br/>last_log_index, last_log_term)
    N2->>N4: RequestVote(term=5, ...)
    N2->>N5: RequestVote(term=5, ...)

    N3->>N3: Check: term=5 > my term (4)?<br/>Haven't voted this term?<br/>Candidate's log at least as up-to-date?
    N3-->>N2: Vote granted

    N4->>N4: Same checks pass
    N4-->>N2: Vote granted

    Note over N2: Received votes from self + N3 + N4<br/>= 3 of 5 = MAJORITY
    N2->>N2: Becomes LEADER for term 5

    N2->>N3: Heartbeat (empty AppendEntries)
    N2->>N4: Heartbeat
    N2->>N5: Heartbeat (N5 was slow to respond,<br/>discovers new leader via heartbeat)
```

**Why "last log up-to-date" matters in voting:** A node only grants its vote if the candidate's log is at least as current as its own — this prevents a node that missed recent committed entries from becoming leader and potentially overwriting/losing that committed data. This single rule is central to Raft's safety guarantee.

---

## 5. Log Replication — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client
    participant L as Leader
    participant F1 as Follower 1
    participant F2 as Follower 2

    C->>L: Command: SET x=5

    L->>L: Append to local log<br/>(uncommitted, index=10)

    par Replicate to followers
        L->>F1: AppendEntries(entries=[SET x=5],<br/>prev_log_index=9, prev_log_term=4)
        F1->>F1: Check: does my log have<br/>entry at index 9 with term 4?
        F1->>F1: Yes — append new entry
        F1-->>L: Success
    and
        L->>F2: AppendEntries(same)
        F2->>F2: Same consistency check
        F2-->>L: Success
    end

    Note over L: Received success from<br/>majority (self + F1 + F2 = 3 of 5)
    L->>L: Mark entry index=10 as COMMITTED
    L-->>C: Success

    Note over L,F2: On NEXT heartbeat, leader informs<br/>followers of new commit_index —<br/>followers apply committed entries<br/>to their local state machine
```

**Why the `prev_log_index`/`prev_log_term` consistency check matters:** Before accepting a new entry, a follower verifies its log matches the leader's log up to that point. If it doesn't match (e.g., the follower has a stale/incorrect entry from an old leader), the leader detects this and works backward, forcing the follower's log to converge with the leader's — this is how Raft guarantees log consistency across the cluster even after leader changes.

---

## 6. Safety Guarantee — Why Split-Brain Is Impossible

```mermaid
flowchart TB
    A["Network partitions cluster<br/>of 5 nodes into two groups"] --> B["Group A: 3 nodes<br/>(has majority)"]
    A --> C["Group B: 2 nodes<br/>(minority)"]

    B --> D["Can elect a leader<br/>(3 votes = majority of 5)"]
    B --> E["Can commit new entries<br/>(3 acks = majority of 5)"]

    C --> F["CANNOT elect a leader<br/>(2 votes ≠ majority of 5)"]
    C --> G["CANNOT commit anything<br/>— remains without a leader,<br/>rejects/queues client writes"]

    H["Mathematical guarantee:<br/>any two majorities out of N nodes<br/>MUST overlap by at least 1 node —<br/>this overlap is what prevents<br/>two independent leaders from<br/>both committing conflicting entries"]
```

*This overlap property is the single most important insight in majority-quorum consensus: since any two majority sets must share at least one common node, it's mathematically impossible for two different "majorities" to simultaneously commit two different values for the same log position — the shared node would have to agree to both, which the protocol explicitly prevents (a node only votes/acks once per term/index).*

---

## 7. Handling a Stale Leader Rejoining (Term Numbers)

```mermaid
sequenceDiagram
    participant OldLeader as Old Leader (Node 1, term 4)
    participant NewLeader as Current Leader (Node 2, term 5)
    participant Follower as Follower (Node 3)

    Note over OldLeader: Node 1 was partitioned,<br/>still believes it's leader (term 4)<br/>Meanwhile cluster elected<br/>Node 2 as leader (term 5)

    Note over OldLeader: Partition heals

    OldLeader->>Follower: AppendEntries(term=4, ...)<br/>(stale leader still trying to lead)

    Follower->>Follower: Compare: incoming term (4)<br/>vs my current term (5)
    Follower-->>OldLeader: Reject — my term (5) is higher

    OldLeader->>OldLeader: Discovers higher term exists<br/>Immediately steps down to FOLLOWER
    OldLeader->>NewLeader: (implicitly) recognizes<br/>Node 2 as legitimate leader

    Note over OldLeader: Node 1 now follows Node 2,<br/>catches up on any missed<br/>committed entries
```

**Why monotonically increasing terms matter:** Every node always defers to a higher term number it observes — this is the mechanism that guarantees a stale leader (one that was partitioned and doesn't know a new election happened) immediately and safely steps down the moment it learns a newer term exists, rather than continuing to issue conflicting commands.

---

## 8. Paxos vs Raft — Conceptual Comparison

```mermaid
flowchart TB
    A["Consensus Protocol Family"] --> B["Paxos<br/>(original, 1989)"]
    A --> C["Raft<br/>(2014, designed for understandability)"]

    B --> B1["Roles: Proposers, Acceptors, Learners<br/>— can overlap on same node"]
    B --> B2["No inherent concept of<br/>a stable 'leader' — multi-Paxos<br/>adds this as an optimization"]
    B --> B3["Notoriously difficult to<br/>correctly implement from<br/>the original paper alone"]

    C --> C1["Roles: explicit Leader/Follower/Candidate"]
    C --> C2["Strong leader model built-in<br/>from the start — all writes<br/>go through one leader"]
    C --> C3["Explicit log replication +<br/>leader election as separate,<br/>well-defined sub-problems"]

    D["Both provide equivalent<br/>safety guarantees —<br/>Raft's contribution is primarily<br/>pedagogical clarity and<br/>ease of correct implementation,<br/>not new theoretical capability"]
```

---

## 9. Component Responsibilities Summary

```mermaid
mindmap
  root((Distributed Consensus HLD))
    Leader
      Handles all client writes
      Replicates log to followers
      Sends periodic heartbeats
    Follower
      Passive, responds to RPCs
      Grants votes during election
      Applies committed entries
    Candidate
      Transient state during election
      Requests votes from peers
    Replicated Log
      Ordered sequence of commands
      Source of truth once committed
    Term Counter
      Monotonically increasing
      Resolves stale-leader conflicts
    Majority Quorum
      Core safety mechanism
      Guarantees overlap property
```

---

## 10. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Consensus family | Raft (leader-based) over classic multi-Paxos | Explicit strong-leader model simplifies reasoning and implementation while providing equivalent safety |
| Commit rule | Majority quorum acknowledgment | Mathematically guarantees any two majorities overlap, making split-brain commits impossible |
| Cluster size | Odd numbers (3/5/7) | Avoids wasted fault tolerance — an even-sized cluster gains no extra resilience over the next-lower odd size |
| Leader election trigger | Randomized timeout | Reduces probability of simultaneous candidacies (split votes) that would otherwise repeatedly stall election |
| Term numbers | Monotonically increasing, compared on every RPC | Provides an unambiguous way for stale leaders/followers to detect they're behind and defer to current state |
| Log consistency enforcement | prev_log_index/term check on every AppendEntries | Ensures followers' logs stay consistent with the leader's, even after leader changes mid-stream |

---

## 11. Bottlenecks & Scaling Considerations

- **Write throughput bounded by slowest majority node** — every committed write requires a round trip to a majority of nodes; adding more nodes to a cluster does NOT increase write throughput (it actually can decrease it, since more nodes must acknowledge) — consensus clusters are scaled for fault tolerance, not throughput.
- **This is why consensus is used sparingly** — real systems typically use a small consensus cluster (e.g., 3-5 nodes) purely for critical coordination metadata (leader election, configuration, locks) rather than for high-volume application data, which gets sharded across many independent replica sets instead.
- **Network partition duration** — a minority partition remains completely unable to make progress for as long as the partition lasts; this is the direct, intentional cost of prioritizing safety (CP) over availability during partitions.
- **Leader as a bottleneck for large clusters** — all client interaction funnels through one node; for read scaling, followers can often serve reads directly (with appropriate staleness caveats), but all writes remain leader-bound.
- **Log compaction/snapshotting** — an ever-growing replicated log eventually needs periodic snapshotting (compacting old entries into a state snapshot) so that new nodes joining or recovering nodes don't need to replay the entire history from the beginning.
- **Configuration changes (adding/removing nodes)** — changing cluster membership itself must go through the consensus protocol to avoid a window where old and new configurations could both form independent majorities (a subtle correctness issue Raft handles via joint consensus during the transition).
- **Cross-region consensus latency** — if cluster nodes span geographic regions (as in globally distributed strongly-consistent systems), majority round-trip time is bounded by inter-region network latency, directly limiting write throughput for globally-replicated consensus groups.
