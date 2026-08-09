# Design Network Partition Detection & Resolution — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Detect when a network partition has split the cluster into disconnected groups
- Distinguish between "node crashed" and "node is unreachable but alive" (a genuinely hard problem)
- Ensure the system behaves safely during a partition (no split-brain data corruption)
- Automatically reconcile/heal state once the partition resolves

### Non-Functional Requirements
- **Safety over liveness during partition:** When forced to choose, prevent incorrect behavior even if it means reduced availability
- **Fast detection:** Minimize the window where the system operates under a stale view of cluster membership
- **Graceful degradation:** The system should degrade predictably (e.g., minority side becomes read-only) rather than fail unpredictably
- **Automatic healing:** Once connectivity is restored, the cluster should reconcile without requiring manual intervention

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Heartbeat interval | 100ms - 1s, depending on desired detection speed |
| Failure detection timeout | 3-10x heartbeat interval (avoid false positives from single missed beat) |
| Typical partition duration (real world) | Seconds to minutes; rarely hours |
| Reconciliation complexity | Proportional to divergence accumulated during partition |

---

## 2. The Fundamental Problem — You Cannot Distinguish "Dead" from "Unreachable"

```mermaid
flowchart TB
    A["Node B stops responding<br/>to Node A's heartbeats"] --> B{"What actually happened?"}
    B --> C["Node B crashed<br/>(genuinely down)"]
    B --> D["Node B is alive, but the<br/>NETWORK between A and B<br/>is partitioned<br/>(B is fine, reachable by others)"]
    B --> E["Node B is alive but<br/>extremely slow<br/>(GC pause, overload)"]

    F["From Node A's perspective,<br/>ALL THREE scenarios look<br/>IDENTICAL: no response<br/>within the timeout window"] --> G["This is the core theoretical<br/>limitation (related to the<br/>FLP impossibility result) —<br/>no algorithm can PERFECTLY<br/>distinguish these cases in an<br/>asynchronous network.<br/>Real systems make a<br/>PRACTICAL judgment call<br/>via timeouts, accepting some<br/>risk of getting it wrong."]
```

---

## 3. High-Level Architecture — Partition-Aware Cluster

```mermaid
flowchart TB
    subgraph MajoritySide["Majority Partition (3 of 5 nodes)"]
        M1["Node 1 (Leader)"]
        M2["Node 2"]
        M3["Node 3"]
    end

    subgraph MinoritySide["Minority Partition (2 of 5 nodes)"]
        Min1["Node 4"]
        Min2["Node 5"]
    end

    NetworkPartition["✂️ Network Partition<br/>(no communication possible<br/>between the two groups)"]

    M1 <--> M2
    M2 <--> M3
    M1 <--> M3

    Min1 <--> Min2

    M1 -.-x NetworkPartition
    NetworkPartition -.-x Min1

    Note1["Majority side: has quorum,<br/>can elect/keep leader,<br/>continues serving writes"]
    Note2["Minority side: lacks quorum,<br/>MUST refuse writes to<br/>avoid diverging from<br/>majority side's state"]
```

**Key idea:** This builds directly on the consensus/quorum mechanisms covered in the Distributed Consensus design — partition detection and resolution isn't a separate bolt-on system, it's the natural consequence of a well-designed quorum-based protocol. The "detection" is really just each side recognizing whether or not it can still achieve a majority.

---

## 4. Failure Detection Mechanism — Detailed Sequence

```mermaid
sequenceDiagram
    participant N1 as Node 1
    participant N2 as Node 2
    participant N3 as Node 3 (about to be partitioned away)

    loop Every heartbeat_interval (e.g., 200ms)
        N1->>N3: Heartbeat ping
        N3-->>N1: Ack
    end

    Note over N1,N3: Network partition occurs

    N1->>N3: Heartbeat ping
    Note over N3: No response (partitioned)

    N1->>N1: Missed heartbeat count: 1
    N1->>N3: Retry heartbeat
    Note over N3: No response
    N1->>N1: Missed heartbeat count: 2

    Note over N1: After N consecutive misses<br/>(exceeding timeout threshold)
    N1->>N1: Mark Node 3 as SUSPECTED_DOWN<br/>Remove from active quorum calculation

    Note over N1: Node 1 re-evaluates:<br/>can I still reach a majority<br/>of the ORIGINAL cluster size?<br/>(2 of 3 remaining = yes, still majority)
```

**Why multiple consecutive missed heartbeats (not just one):** A single missed heartbeat could easily be a transient network blip rather than a genuine partition/failure — requiring several consecutive misses before declaring a node "suspected down" trades a small amount of detection latency for significantly fewer false-positive partition declarations.

---

## 5. Quorum-Based Split Behavior

```mermaid
flowchart TB
    A["Partition detected —<br/>cluster split into two groups"] --> B["Each side independently<br/>counts: how many of the<br/>ORIGINAL cluster members<br/>can I still reach?"]

    B --> C{"Can reach a<br/>MAJORITY (> N/2)<br/>of original members?"}
    C -- Yes --> D["This side CONTINUES<br/>operating normally —<br/>elects/retains leader,<br/>accepts writes"]
    C -- No --> E["This side ENTERS<br/>DEGRADED MODE —<br/>refuses writes<br/>(read-only at most,<br/>or fully unavailable,<br/>depending on design)"]

    F["Critical property:<br/>AT MOST ONE side can ever<br/>have a majority — this is<br/>what makes the split-brain<br/>problem solvable via quorum,<br/>as established in the<br/>Distributed Consensus design"]
```

---

## 6. Minority Side Behavior — Detailed Sequence

```mermaid
sequenceDiagram
    participant Client as Client (talking to minority side)
    participant Min1 as Node 4 (minority partition)

    Client->>Min1: Write request

    Min1->>Min1: Attempt to replicate to quorum<br/>(needs 3 of 5, only has 2<br/>reachable including self)
    Min1->>Min1: Quorum NOT achieved

    Min1-->>Client: REJECTED —<br/>cannot guarantee consistency<br/>(unavailable for writes)

    Client->>Min1: Read request (if reads<br/>are configured to also<br/>require quorum)
    Min1->>Min1: Same quorum check fails
    Min1-->>Client: REJECTED or<br/>STALE READ WARNING<br/>(depending on read<br/>consistency policy)

    Note over Min1: Node continues heartbeating,<br/>waiting to detect when<br/>connectivity is restored
```

*This is the direct, correct consequence of choosing consistency over availability during a partition (the CP choice in CAP theorem terms) — the minority side deliberately sacrifices its own availability specifically to prevent it from silently diverging from the majority side's state.*

---

## 7. Partition Healing / Reconciliation — Detailed Sequence

```mermaid
sequenceDiagram
    participant Min as Minority Node (was isolated)
    participant Maj as Majority Leader
    participant Log as Replicated Log

    Note over Min,Maj: Network connectivity restored

    Min->>Maj: Resume heartbeat/gossip contact
    Maj->>Min: Current term is HIGHER than<br/>what Min last knew

    Min->>Min: Recognize: I am behind,<br/>step down from any stale<br/>self-belief about leadership<br/>(same term-comparison rule<br/>as covered in Consensus design)

    Min->>Log: Request log entries since<br/>my last known committed index
    Log-->>Min: Stream of committed entries<br/>Min missed during partition

    Min->>Min: Apply missed entries in order,<br/>catch up to current state

    Note over Min: Min rejoins as a follower,<br/>fully reconciled —<br/>no manual intervention needed,<br/>because it never accepted<br/>any conflicting writes during<br/>the partition (it was correctly<br/>unavailable, not incorrectly<br/>diverged)
```

**Why healing is straightforward here:** Because the minority side was correctly prevented from accepting writes during the partition (rather than accepting them and creating conflicting state), reconciliation is just "catch up on missed history" — a simple log replay — rather than a complex conflict-resolution problem. This is the payoff for accepting reduced availability during the partition.

---

## 8. Contrast: What Happens WITHOUT Proper Quorum Enforcement

```mermaid
flowchart TB
    A["System WITHOUT quorum-based<br/>partition handling<br/>(e.g., naive dual-leader setup)"] --> B["BOTH sides of the partition<br/>continue accepting writes<br/>independently, believing<br/>they're each still the<br/>legitimate leader"]
    B --> C["Partition heals"]
    C --> D["Now TWO divergent, conflicting<br/>histories of writes exist —<br/>this is the split-brain<br/>data corruption scenario"]
    D --> E["Reconciliation requires<br/>complex, often LOSSY<br/>conflict resolution —<br/>exactly what quorum-based<br/>design prevents entirely<br/>by ensuring it never happens<br/>in the first place"]
```

---

## 9. Detecting Partial/Asymmetric Partitions (Harder Case)

```mermaid
flowchart TB
    A["Asymmetric partition:<br/>Node A can send messages<br/>TO Node B, but Node B's<br/>responses never reach<br/>Node A (one-way failure)"] --> B["This is HARDER to detect<br/>than a clean, symmetric<br/>partition — A sees no<br/>response and assumes B<br/>is down, while B is actually<br/>receiving A's messages fine"]

    C["Mitigation: use a THIRD<br/>node as an indirect health<br/>check — 'can you reach<br/>Node B? I can't'"] --> D["If a majority of OTHER<br/>nodes can confirm they<br/>also can't reach B, the<br/>cluster gains more confidence<br/>B is genuinely partitioned,<br/>not just having an asymmetric<br/>issue specific to A"]
```

*Real-world network partitions aren't always the clean "two isolated groups" textbook scenario — asymmetric and partial partitions are common in practice (e.g., a router misconfiguration affecting only certain paths), which is why production systems often incorporate indirect/gossip-based health checking rather than relying solely on direct pairwise heartbeats.*

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Network Partition HLD))
    Heartbeat Mechanism
      Periodic liveness checks
      Multiple-miss threshold
      Reduces false positives
    Quorum Calculator
      Tracks reachable majority
      Determines operational mode
    Majority Side
      Continues normal operation
      Elects/retains leader
    Minority Side
      Refuses writes
      Degrades safely
      Awaits reconnection
    Reconciliation Process
      Log replay on rejoin
      Term-based deference
    Indirect Health Checking
      Gossip-based confirmation
      Handles asymmetric partitions
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Failure detection | Timeout-based with multiple-miss threshold | No algorithm can perfectly distinguish "dead" from "unreachable" in an asynchronous network; timeouts are the practical, universally-used compromise |
| Split behavior | Quorum-based (majority continues, minority refuses writes) | Directly prevents split-brain — mathematically guarantees at most one side can make progress |
| Minority side policy | Reject writes (and optionally reads) | Prioritizes correctness over availability during the partition, per the CP choice in CAP theorem |
| Healing mechanism | Log replay from last known committed index | Works cleanly specifically because the minority side never accepted conflicting writes to begin with |
| Partial partition handling | Indirect/gossip-based health confirmation | Direct pairwise heartbeats alone can't reliably detect asymmetric network failures |

---

## 12. Bottlenecks & Scaling Considerations

- **Heartbeat timeout tuning is a genuine tradeoff, not a solved problem** — too aggressive causes false-positive partition declarations during normal transient network jitter (unnecessarily reducing availability); too lenient delays legitimate failure detection — this must be tuned empirically per deployment environment's actual network reliability characteristics.
- **Minority side complete unavailability may be too strict for some use cases** — some systems allow the minority side to continue serving stale reads (with clear staleness warnings) rather than fully refusing, trading a bit of consistency guarantee for partial availability — this is a deliberate design choice depending on the application's tolerance.
- **Partition detection latency directly impacts recovery time objective (RTO)** — the entire failover/degradation sequence can't begin until the partition is detected; systems with strict RTO requirements need aggressive (but carefully tuned) heartbeat intervals.
- **Cascading detection during broader outages** — a large-scale network event affecting many nodes simultaneously can trigger a flood of near-simultaneous partition detections and leader re-elections across many independent clusters/shards, potentially overwhelming the coordination infrastructure — worth considering rate-limiting or staggering re-election attempts during mass-failure scenarios.
- **Testing partition scenarios is notoriously difficult** — genuine network partitions are hard to reliably simulate in testing environments; chaos engineering tools (deliberately injecting network partitions, packet loss, asymmetric failures) are essential for validating this logic actually behaves correctly, since the failure modes are rare in normal operation but catastrophic if subtly wrong.
- **Client-side behavior during partition** — clients connected to the minority side need clear, well-defined error handling (retry with backoff, fail over to a different node) rather than hanging indefinitely waiting for a response that will never come until the partition heals.
