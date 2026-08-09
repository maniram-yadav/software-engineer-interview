# Design a Leader Election System for a Cluster of 1000+ Nodes — High-Level Design Document

## 1. Requirements

### Functional Requirements
- A large cluster (1000+ nodes) must agree on exactly one active leader at any time
- Automatic failover — if the leader crashes/becomes unreachable, a new leader is elected without manual intervention
- Nodes must be able to reliably determine "who is the current leader" (leader discovery)
- Support graceful leader handoff for planned maintenance (not just crash recovery)

### Non-Functional Requirements
- **Split-brain prevention (paramount):** At most one node may believe it is the active leader at any given time
- **Scalability:** Election mechanism must not require O(N) coordination overhead across all 1000+ nodes directly — this would not scale
- **Fast failover:** New leader should be established within seconds of the previous leader's failure
- **Low steady-state overhead:** Leader election machinery shouldn't consume significant resources during normal (non-election) operation

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Cluster size | 1,000-10,000 nodes |
| Actual consensus participants | Small (3-7 dedicated coordinator nodes) — NOT all 1000+ |
| Failover target | < 5-10 seconds |
| Leader lease renewal interval | ~1-2 seconds |
| Heartbeat/health check interval | Sub-second |

---

## 2. The Key Insight — Don't Run Consensus Across All 1000+ Nodes

```mermaid
flowchart TB
    A["Naive approach:<br/>Run Raft/Paxos consensus<br/>directly among all 1000+ nodes"] --> B["Problem: consensus round-trip<br/>requires majority acknowledgment —<br/>with 1000+ participants, this is<br/>extremely slow and operationally<br/>fragile (many more failure<br/>combinations to handle)"]

    C["Actual production approach:<br/>Small dedicated coordination<br/>service (3-7 nodes) running<br/>consensus, separate from the<br/>1000+ application nodes"] --> D["The 1000+ nodes don't<br/>participate in consensus at all —<br/>they simply ASK the small<br/>coordination cluster<br/>'who is the leader?'"]
```

**This is the single most important design decision:** Systems like ZooKeeper, etcd, and Consul exist precisely because running full consensus among a large, dynamic set of application nodes doesn't scale. Instead, a small, stable coordination cluster runs consensus among *itself*, and the large application cluster treats it as an external, trusted authority for leader election — reducing an O(N) coordination problem to an O(1) lookup against a much smaller, purpose-built system.

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph AppCluster["Application Cluster (1000+ nodes)"]
        Node1["App Node 1<br/>(candidate)"]
        Node2["App Node 2<br/>(candidate)"]
        Node3["App Node 3<br/>(current leader)"]
        NodeN["... App Node 1000+"]
    end

    subgraph CoordCluster["Coordination Service (5 nodes, Raft-based)"]
        C1["Coordinator 1<br/>(Raft leader)"]
        C2["Coordinator 2"]
        C3["Coordinator 3"]
        C4["Coordinator 4"]
        C5["Coordinator 5"]
    end

    Node1 -->|"Attempt to acquire<br/>leader lock/lease"| CoordCluster
    Node2 -->|"Attempt to acquire"| CoordCluster
    Node3 -->|"Currently holds lease,<br/>renews periodically"| CoordCluster
    NodeN -->|"Watch: who is leader?"| CoordCluster

    C1 <--> C2
    C1 <--> C3
    C1 <--> C4
    C1 <--> C5
```

**Key idea:** The small coordination cluster acts as a **distributed lock manager** (see the earlier Distributed Lock Manager design — this is a direct application of that pattern). "Being the leader" is modeled as "holding a specific named lock" — whichever application node successfully acquires the lock is the leader; everyone else watches for the lock to become available.

---

## 4. Leader Election via Distributed Lock — Detailed Sequence

```mermaid
sequenceDiagram
    participant N1 as App Node 1
    participant N2 as App Node 2
    participant N3 as App Node 3
    participant Coord as Coordination Service

    Note over N1,N3: All nodes start up, race to become leader

    N1->>Coord: Try to create ephemeral node<br/>"/election/leader" (or acquire lock)
    N2->>Coord: Try to create same node
    N3->>Coord: Try to create same node

    Note over Coord: Coordination service guarantees<br/>only ONE creation succeeds<br/>(atomic operation)

    Coord-->>N1: SUCCESS — you are the leader
    Coord-->>N2: FAILED — node already exists
    Coord-->>N3: FAILED — node already exists

    N2->>Coord: Watch "/election/leader" for changes
    N3->>Coord: Watch "/election/leader" for changes

    Note over N1: Node 1 is now leader,<br/>begins leader duties
    N1->>Coord: Periodic heartbeat/lease renewal
```

---

## 5. Failover on Leader Crash — Detailed Sequence

```mermaid
sequenceDiagram
    participant N1 as App Node 1 (Leader)
    participant Coord as Coordination Service
    participant N2 as App Node 2 (Watcher)
    participant N3 as App Node 3 (Watcher)

    Note over N1: Node 1 crashes<br/>(process dies, network fails, etc.)

    Note over Coord: Session/lease with Node 1<br/>times out (no heartbeat received<br/>within TTL window)
    Coord->>Coord: Automatically delete/release<br/>the ephemeral leader node<br/>(tied to Node 1's session)

    Coord-->>N2: Notify: leader node deleted<br/>(watch triggered)
    Coord-->>N3: Notify: leader node deleted

    N2->>Coord: Attempt to create leader node
    N3->>Coord: Attempt to create leader node

    Coord-->>N2: SUCCESS — you are the new leader
    Coord-->>N3: FAILED — N2 got there first

    N3->>Coord: Re-establish watch on new leader
    Note over N2: Node 2 begins leader duties<br/>Total failover time ≈<br/>session timeout + election race
```

**Why ephemeral nodes/sessions matter:** The leader's "claim" on leadership is tied to an active session with the coordination service (via periodic heartbeats). If the leader crashes, it stops heartbeating, the session expires, and the coordination service *automatically* releases the leadership claim — no separate failure-detection mechanism is needed; session expiry IS the failure detection.

---

## 6. Leader Discovery for the Broader 1000+ Node Cluster

```mermaid
flowchart TB
    A["1000+ nodes need to know<br/>'who is the current leader'<br/>to route requests/coordinate work"] --> B{"Discovery Strategy"}

    B --> C["Direct watch on coordination<br/>service (works, but 1000+<br/>simultaneous watchers on<br/>one small cluster can<br/>still add load)"]

    B --> D["Leader publishes its identity<br/>to a widely-distributed,<br/>cache-friendly location<br/>(e.g., a well-known key in a<br/>distributed cache, refreshed<br/>periodically by the leader itself)"]

    D --> D1["Application nodes read from<br/>this cache/gossip layer instead<br/>of hitting the coordination<br/>service directly — reduces<br/>load on the small, critical<br/>coordination cluster"]
```

*For genuinely massive clusters, even 1000+ simultaneous watches on a 5-node coordination service can become a scaling concern — production systems often add a caching/gossip layer so most nodes discover the leader through a cheap, widely-replicated read rather than a direct connection to the coordination service itself.*

---

## 7. Preventing a "Zombie Leader" (Fencing, Revisited)

```mermaid
sequenceDiagram
    participant N1 as Node 1 (was leader,<br/>experiences long GC pause)
    participant Coord as Coordination Service
    participant N2 as Node 2 (new leader)
    participant Resource as Shared Resource

    Note over N1: Node 1 pauses for 10 seconds<br/>(GC, OS scheduling, etc.)<br/>— stops heartbeating

    Coord->>Coord: Lease expires,<br/>releases leadership claim
    N2->>Coord: Acquires leadership<br/>(fence_token = 42)

    Note over N1: Node 1 resumes,<br/>DOESN'T YET KNOW it<br/>lost leadership
    N1->>Resource: Attempt write<br/>(presents STALE fence_token = 41)

    Resource->>Resource: Check: 41 < last_seen (42)?
    Resource-->>N1: REJECTED — stale token

    N2->>Resource: Write with fence_token = 42
    Resource-->>N2: Accepted
```

*This is the exact same fencing token mechanism from the Distributed Lock Manager design — leader election is fundamentally a specialized application of distributed locking, so it inherits the same "paused-not-crashed" correctness hazard and the same solution.*

---

## 8. Graceful Leader Handoff (Planned Maintenance)

```mermaid
sequenceDiagram
    participant N1 as Current Leader
    participant Coord as Coordination Service
    participant N2 as Next Leader Candidate

    Note over N1: Operator initiates planned<br/>maintenance/deployment

    N1->>N1: Finish in-flight critical work<br/>(drain, checkpoint state)
    N1->>Coord: Explicitly release leadership<br/>(voluntary, not timeout-based)

    Coord-->>N2: Notify: leadership available
    N2->>Coord: Acquire leadership
    Coord-->>N2: SUCCESS — new leader

    Note over N1: Node 1 can now be safely<br/>taken down for maintenance —<br/>NO failover delay was needed,<br/>since handoff was voluntary<br/>and immediate, not dependent<br/>on a timeout expiring
```

**Why this matters operationally:** Relying solely on timeout-based failure detection means every planned deployment/restart of the leader incurs the full failover delay (waiting for lease timeout) even though the leader is healthy and could hand off instantly. Supporting explicit voluntary release avoids this unnecessary downtime for routine operational events.

---

## 9. Component Responsibilities Summary

```mermaid
mindmap
  root((Leader Election at Scale HLD))
    Coordination Service
      Small, dedicated consensus cluster
      Source of truth for leadership
      Session/lease management
    Application Nodes
      Candidates for leadership
      Watchers for leader changes
      Periodic heartbeat if leader
    Ephemeral Leadership Claim
      Tied to session/lease
      Auto-released on failure
    Fencing Token
      Monotonic counter
      Protects against zombie leaders
    Discovery/Gossip Layer
      Reduces load on coordination service
      Cache-friendly leader identity lookup
```

---

## 10. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Consensus scope | Small dedicated coordination cluster (3-7 nodes), not all 1000+ app nodes | Running consensus among 1000+ nodes directly doesn't scale; a small, stable coordinator cluster reduces this to an O(1) lookup problem for application nodes |
| Leadership model | Ephemeral node/lock tied to a session | Session expiry naturally IS the failure detection mechanism — no separate heartbeat-monitoring system needed |
| Zombie leader protection | Fencing tokens | A paused (not crashed) former leader could otherwise still issue commands after losing its lease — fencing makes this safe regardless |
| Leader discovery at scale | Caching/gossip layer over direct coordination service watches | Prevents 1000+ simultaneous connections from overwhelming the small coordination cluster |
| Planned maintenance | Explicit voluntary handoff | Avoids unnecessary failover delay for routine, healthy leader transitions |

---

## 11. Bottlenecks & Scaling Considerations

- **Coordination service as a critical, small-blast-radius dependency** — because it's intentionally small (3-7 nodes) and handles only leadership metadata (not application data), it's much easier to keep highly available and low-latency than trying to scale consensus to the full cluster size.
- **Watch/notification fan-out at 1000+ scale** — even with a small coordination cluster, if all 1000+ nodes directly watch it for leader changes, that's still 1000+ open connections/watches on a small cluster; the gossip/caching layer mitigation (section 6) is essential at this scale, not optional.
- **Thundering herd on leader failure** — when the leader's session expires, potentially many candidate nodes may attempt to acquire leadership simultaneously; the coordination service's atomic creation guarantee handles correctness, but a large candidate pool still generates a burst of near-simultaneous requests worth monitoring.
- **Lease timeout tuning** — too short risks false failover during transient network blips or GC pauses (mitigated by fencing, but still causes unnecessary leadership churn); too long delays genuine failover — typically tuned to a few seconds based on realistic heartbeat reliability.
- **Coordination service becomes a single point of failure if under-provisioned** — while intentionally small, it must still itself be deployed with proper fault tolerance (odd node count, spread across failure domains) since ALL leader election for the entire 1000+ node cluster depends on its availability.
- **Cross-region leader election** — if the 1000+ node cluster spans regions, the coordination service's placement becomes critical; a coordination service in one region adds latency for election operations from distant regions, and a regional outage affecting the coordination service itself would halt leader election cluster-wide.
