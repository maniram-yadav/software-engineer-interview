# Design a Distributed Counter (CRDT-based) — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Increment/decrement a counter from any replica, with no single coordination point
- All replicas must eventually converge to the mathematically correct total, regardless of message ordering
- Support both increment-only counters (e.g., view counts) and increment/decrement counters (e.g., inventory-like tallies)
- Counter must remain available for writes even during network partitions between replicas

### Non-Functional Requirements
- **High availability:** Writes must succeed locally even if the replica is fully partitioned from all others
- **Eventual accuracy:** Once all replicas can communicate again, the merged total must be exactly correct — no lost or double-counted increments
- **Low latency:** Increments should be near-instant local operations, never blocked on cross-replica coordination
- **Scalability:** Must handle very high increment rates (e.g., viral content view counting) across many replicas

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Increments/sec (single hot counter, viral event) | ~100,000+ |
| Replicas | 3-20, spread across regions/data centers |
| Merge/sync frequency | Sub-second to a few seconds |
| Per-replica state size | O(number of replicas) — small and bounded |

---

## 2. The Core Problem — Why a Naive Distributed Counter Fails

```mermaid
flowchart TB
    A["Naive approach: each replica<br/>stores ONE shared integer,<br/>increments it, and replicates<br/>the new value to others"] --> B["Problem: Replica A has count=10,<br/>increments to 11, replicates"]
    A --> C["Meanwhile Replica B ALSO<br/>has count=10 (hasn't seen A's<br/>update yet), increments to 11,<br/>replicates"]
    B & C --> D["Both replicas now claim<br/>the 'true' value is 11 —<br/>but TWO increments actually<br/>happened. The real total<br/>should be 12. ONE increment<br/>was silently lost."]

    E["CRDT-based counters solve this<br/>by never merging raw totals —<br/>instead tracking each replica's<br/>OWN contribution separately"]
```

---

## 3. G-Counter (Grow-Only Counter) — Core CRDT Structure

```mermaid
flowchart TB
    A["G-Counter State<br/>= map of replica_id → count<br/>(each replica tracks ONLY<br/>its own increments)"] --> B["Replica A's local state:<br/>{A: 5, B: 3, C: 2}"]
    B --> C["Total value = SUM of all entries<br/>= 5+3+2 = 10"]

    D["Rule: Replica X may ONLY<br/>increment its OWN entry<br/>(X's count), NEVER another<br/>replica's entry"] --> E["This is what makes<br/>concurrent increments from<br/>different replicas naturally<br/>conflict-free — they're<br/>modifying different map keys"]
```

**Why this design eliminates the lost-update problem entirely:** Since each replica only ever increments its own dedicated counter within the map, there is no shared mutable value that two replicas could race on. The "total" is always just a derived sum — a read-time computation, never a stored value that needs coordinated updates.

---

## 4. Merge Operation — The Heart of the CRDT

```mermaid
flowchart TB
    A["Replica A's state:<br/>{A:5, B:3, C:2}"] --> C["MERGE = element-wise MAXIMUM<br/>for each replica_id key"]
    B["Replica B's state:<br/>{A:4, B:6, C:2}<br/>(B has seen more of its<br/>own increments, but is<br/>slightly behind on A's)"] --> C

    C --> D["Merged state:<br/>{A: max(5,4)=5,<br/>B: max(3,6)=6,<br/>C: max(2,2)=2}"]
    D --> E["Merged total = 5+6+2 = 13"]

    F["Why MAXIMUM (not sum)<br/>for merging?"] --> G["Each replica's own counter<br/>only ever increases (grow-only) —<br/>so the higher value SEEN for<br/>a given replica_id is always<br/>the more up-to-date one.<br/>Taking max is idempotent and<br/>order-independent — merge the<br/>same two states any number<br/>of times, in any order,<br/>and you always get the<br/>same correct result."]
```

**This is what makes it a CRDT (Conflict-free Replicated Data Type):** the merge function is **commutative** (order doesn't matter), **associative** (grouping doesn't matter), and **idempotent** (merging the same state twice doesn't cause double-counting) — these three mathematical properties together guarantee that no matter how or when replicas exchange state, they will always converge to the identical, correct result.

---

## 5. High-Level Architecture

```mermaid
flowchart TB
    subgraph RegionA["Region A"]
        ReplicaA[("Replica A<br/>local state: {A:n, ...}")]
        AppA["App Servers (Region A)"]
    end

    subgraph RegionB["Region B"]
        ReplicaB[("Replica B<br/>local state: {B:n, ...}")]
        AppB["App Servers (Region B)"]
    end

    subgraph RegionC["Region C"]
        ReplicaC[("Replica C<br/>local state: {C:n, ...}")]
        AppC["App Servers (Region C)"]
    end

    AppA -->|"Increment (local, fast)"| ReplicaA
    AppB -->|"Increment (local, fast)"| ReplicaB
    AppC -->|"Increment (local, fast)"| ReplicaC

    ReplicaA <-.->|"Periodic gossip/merge<br/>(async, eventual)"| ReplicaB
    ReplicaB <-.->|"Periodic gossip/merge"| ReplicaC
    ReplicaA <-.->|"Periodic gossip/merge"| ReplicaC
```

**Key idea:** Each region's replica accepts writes purely locally — an increment never waits for any cross-region communication. Separately, replicas periodically **gossip** their state to each other and merge, converging the global count over time. This decouples write availability/latency completely from convergence, which is precisely the point of using a CRDT.

---

## 6. Increment Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant App as Application (Region A)
    participant ReplicaA as Replica A

    App->>ReplicaA: Increment counter "video_views:123"
    ReplicaA->>ReplicaA: local_state[A] += 1<br/>(purely local operation,<br/>no network call)
    ReplicaA-->>App: Success (near-instant)

    Note over ReplicaA: No coordination with<br/>Replica B or C occurred —<br/>this write is complete and<br/>durable from Region A's<br/>perspective immediately
```

---

## 7. Gossip/Sync Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant ReplicaA as Replica A<br/>{A:100, B:0, C:0}
    participant ReplicaB as Replica B<br/>{A:0, B:80, C:0}

    loop Periodic sync (e.g., every 1-2 seconds)
        ReplicaA->>ReplicaB: Send local state {A:100, B:0, C:0}
        ReplicaB->>ReplicaB: Merge: max({A:0,B:80,C:0},<br/>{A:100,B:0,C:0})<br/>= {A:100, B:80, C:0}

        ReplicaB->>ReplicaA: Send local state {A:0, B:80, C:0}<br/>(or its NEW merged state)
        ReplicaA->>ReplicaA: Merge: max({A:100,B:0,C:0},<br/>{A:0,B:80,C:0})<br/>= {A:100, B:80, C:0}
    end

    Note over ReplicaA,ReplicaB: Both converge to the<br/>SAME state {A:100,B:80,C:0}<br/>= total 180, regardless of<br/>network delays or message<br/>reordering during this process
```

---

## 8. PN-Counter (Supporting Decrements)

```mermaid
flowchart TB
    A["G-Counter only supports<br/>increment — how do we<br/>support decrement too?"] --> B["PN-Counter =<br/>TWO G-Counters combined:<br/>P (positive/increments)<br/>N (negative/decrements)"]

    B --> C["Increment: increment own<br/>entry in P counter"]
    B --> D["Decrement: increment own<br/>entry in N counter<br/>(NOT decrement anything —<br/>still grow-only internally!)"]

    C & D --> E["Effective value =<br/>SUM(P) - SUM(N)"]

    F["Merge each of P and N<br/>independently using the<br/>same element-wise max rule<br/>as a plain G-Counter"] --> E
```

**Why decrements are also modeled as increments internally:** This preserves the grow-only property that makes the merge function well-defined and conflict-free — by never actually decreasing any stored value, the same max-based merge logic applies uniformly to both counters, and the final subtraction happens only at read time.

---

## 9. Reading the Current Value

```mermaid
sequenceDiagram
    participant Client as Client
    participant ReplicaX as Any Replica

    Client->>ReplicaX: GET current count
    ReplicaX->>ReplicaX: SUM all entries in<br/>local state map<br/>(P counter, or P-N for PN-Counter)
    ReplicaX-->>Client: Return computed total

    Note over Client: Value returned reflects<br/>THIS replica's current view —<br/>may be slightly behind the<br/>true global total if recent<br/>increments elsewhere haven't<br/>yet propagated via gossip.<br/>This is the accepted<br/>eventual-consistency tradeoff.
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((CRDT Counter HLD))
    Local Replica State
      Per-replica increment map
      Grow-only by construction
    Increment Operation
      Purely local
      No cross-replica coordination
    Merge Function
      Element-wise maximum
      Commutative, associative, idempotent
    Gossip Protocol
      Periodic async state exchange
      Drives eventual convergence
    PN-Counter Extension
      Separate P and N G-Counters
      Enables decrement support
    Read Path
      Sum of local state
      May be slightly stale
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Data structure | G-Counter (per-replica increment map) | Eliminates the lost-update race condition inherent to naive shared-integer replication |
| Merge strategy | Element-wise maximum | Provides the commutative/associative/idempotent properties needed for order-independent, conflict-free convergence |
| Decrement support | PN-Counter (two combined G-Counters) | Preserves the grow-only property that makes merging well-defined, rather than trying to make a single counter both grow and shrink |
| Write path | Fully local, no coordination | Maximizes availability and minimizes latency — the entire point of choosing a CRDT over a coordinated counter |
| Convergence mechanism | Periodic gossip/anti-entropy | Decouples write latency from consistency — replicas sync independently of the write path |
| Consistency model | Eventual (strong eventual consistency) | Appropriate for counters where a brief staleness window is harmless (see the Linearizability vs Eventual Consistency design for the broader decision framework) |

---

## 12. Bottlenecks & Scaling Considerations

- **State size grows with replica count** — the per-replica map has one entry per replica that has ever incremented the counter; for a bounded, known set of server-side replicas this stays small, but if individual clients were (incorrectly) used as "replicas," this would grow unboundedly — same lesson as vector clocks.
- **Gossip frequency vs convergence latency tradeoff** — more frequent gossip reduces the staleness window but increases network chatter; less frequent gossip reduces overhead but widens the gap between "locally visible" and "globally true" counts.
- **Hot counter contention within a single replica** — even though cross-replica coordination is eliminated, an extremely high increment rate hitting one specific replica's local counter still needs efficient local concurrency handling (e.g., sharding the counter across multiple local shards/cores, then summing).
- **Read consistency expectations** — clients must understand that reading from different replicas can return different (both valid, but different) totals until convergence completes; if an application needs a guaranteed-accurate read (e.g., for billing), a CRDT counter is the wrong tool — that calls back to the linearizable approach instead.
- **State transfer efficiency at very large replica counts** — gossiping the full state map on every sync is cheap when replica count is small (tens), but for extremely large replica sets, delta-based gossip (sending only what's changed since last sync) becomes important to avoid unnecessary bandwidth.
- **Garbage collection of retired replicas** — if a replica is permanently decommissioned, its entry in the state map remains forever (it's grow-only, never removed) unless an explicit, carefully-coordinated pruning mechanism is added — an unbounded, rarely-addressed edge case in long-lived CRDT deployments.
