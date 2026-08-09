# Linearizability vs Eventual Consistency — High-Level Design Document

## 1. Requirements & Framing

This question asks you to **design a system requiring each consistency model** and justify the tradeoffs with a concrete example — it's fundamentally a comparative systems-theory question, not a single architecture. This document covers:
1. A system requiring **linearizability** (e.g., a distributed lock / bank account balance)
2. A system where **eventual consistency** is the right choice (e.g., a social media like counter)
3. The theoretical distinction, and why the "right" choice depends entirely on the use case

### Key Non-Functional Tradeoff
| Property | Linearizable | Eventually Consistent |
|---|---|---|
| Latency | Higher (requires coordination) | Lower (local writes/reads) |
| Availability during partition | Reduced (may reject requests) | Full (always accepts) |
| Read result | Always reflects latest write | May be stale, temporarily |
| Use case fit | Financial balances, locks, inventory counts | Like counts, view counts, presence status |

---

## 2. Conceptual Definitions

```mermaid
flowchart TB
    A["Linearizability"] --> A1["Every operation appears to<br/>take effect INSTANTANEOUSLY<br/>at some point between its<br/>invocation and response"]
    A --> A2["Once a write completes,<br/>EVERY subsequent read,<br/>from ANY node, sees it<br/>(or a later write) —<br/>no stale reads, ever"]
    A --> A3["Equivalent to: the system<br/>behaves as if there were<br/>only ONE copy of the data"]

    B["Eventual Consistency"] --> B1["If no new writes occur,<br/>all replicas WILL eventually<br/>converge to the same value"]
    B --> B2["No guarantee on HOW LONG<br/>convergence takes, or that<br/>reads reflect the most<br/>recent write in the meantime"]
    B --> B3["Different nodes may return<br/>different (but each individually<br/>valid, older) values<br/>simultaneously"]
```

---

## 3. System A — Requires Linearizability: Distributed Bank Account Balance

### 3.1 High-Level Architecture

```mermaid
flowchart TB
    Client1["Client 1<br/>(Withdraw $100)"]
    Client2["Client 2<br/>(Withdraw $100,<br/>same account)"]

    subgraph ConsensusGroup["Consensus-Based Store (Raft/Paxos)"]
        Leader["Leader Node<br/>(handles ALL writes)"]
        F1["Follower 1"]
        F2["Follower 2"]
    end

    Client1 --> Leader
    Client2 --> Leader

    Leader -->|"replicate + quorum ack<br/>BEFORE responding"| F1
    Leader -->|"replicate + quorum ack"| F2
```

**Why linearizability is non-negotiable here:** Consider an account with $150. If both clients' withdrawal requests were allowed to read a stale balance ("I see $150, so $100 is fine") without linearizable ordering, **both** could succeed — overdrawing the account by $50. A financial balance is exactly the case where "every read must reflect the absolute latest state" isn't a nice-to-have, it's the core correctness property the whole system exists to provide.

### 3.2 Linearizable Withdrawal Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant C1 as Client 1 (Withdraw $100)
    participant C2 as Client 2 (Withdraw $100)
    participant Leader as Leader (single source of truth)

    Note over Leader: Balance = $150

    C1->>Leader: Withdraw $100
    Leader->>Leader: Check balance ($150) >= $100? Yes
    Leader->>Leader: Atomically deduct: balance = $50
    Leader->>Leader: Replicate to quorum, commit
    Leader-->>C1: Success, new balance = $50

    C2->>Leader: Withdraw $100<br/>(arrives after C1's write is committed)
    Leader->>Leader: Check balance ($50) >= $100? NO
    Leader-->>C2: REJECTED — insufficient funds

    Note over Leader: Because ALL operations are<br/>serialized through the single<br/>leader with quorum commit,<br/>C2 is GUARANTEED to see<br/>C1's already-committed write —<br/>no possibility of both succeeding
```

### 3.3 Tradeoff Cost of This Choice

```mermaid
flowchart TB
    A["Every balance check/withdrawal<br/>MUST go through the leader,<br/>with quorum replication"] --> B["Cost: Higher latency<br/>(cross-node coordination<br/>on every operation)"]
    A --> C["Cost: Reduced availability<br/>during network partition —<br/>minority-side replicas CANNOT<br/>serve writes at all"]
    D["Benefit: Absolute correctness —<br/>the one property that matters<br/>most for money"] --> E["This tradeoff is CORRECT<br/>for this use case — an<br/>unavailable bank temporarily<br/>is vastly preferable to an<br/>incorrect bank balance"]
```

---

## 4. System B — Eventual Consistency Is the Right Choice: Social Media Like Counter

### 4.1 High-Level Architecture

```mermaid
flowchart TB
    Client1["User A (likes post)<br/>Region: US"]
    Client2["User B (views post)<br/>Region: EU"]

    subgraph USRegion["US Region"]
        USReplica[("Like Counter Replica<br/>(US)")]
    end

    subgraph EURegion["EU Region"]
        EUReplica[("Like Counter Replica<br/>(EU)")]
    end

    Client1 -->|"Like (fast, local write)"| USReplica
    Client2 -->|"Read (fast, local read)"| EUReplica

    USReplica -.->|"Async replication<br/>(propagates over next<br/>few hundred ms - seconds)"| EUReplica
```

**Why eventual consistency is the RIGHT choice here (not a compromise):** If User B in the EU reads the like count a few hundred milliseconds before User A's like in the US has propagated, they see "1,203 likes" instead of "1,204 likes." This is completely inconsequential — no one is harmed, no invariant is violated, and the value converges moments later. Forcing this operation through linearizable consensus would add meaningful latency to every single like/view, for a property where nobody would ever notice or care about a fleeting few-hundred-millisecond discrepancy.

### 4.2 Eventually Consistent Write Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant A as User A (US)
    participant USRep as US Replica
    participant EURep as EU Replica
    participant B as User B (EU)

    A->>USRep: Like post (local write)
    USRep->>USRep: Increment counter: 1203 → 1204
    USRep-->>A: Success (fast — no cross-region wait)

    USRep->>EURep: Async replicate increment<br/>(happens in background,<br/>not blocking A's response)

    Note over B: B reads BEFORE replication<br/>arrives (race condition, but harmless)
    B->>EURep: Read like count
    EURep-->>B: 1203 (stale by one increment)

    Note over EURep: Replication arrives shortly after
    EURep->>EURep: Apply increment: 1203 → 1204

    Note over B: If B refreshes moments later,<br/>sees 1204 — system has<br/>CONVERGED, as eventual<br/>consistency guarantees
```

### 4.3 Conflict Resolution for Concurrent Writes (CRDT-style Counter)

```mermaid
flowchart TB
    A["Two regions increment<br/>the SAME counter concurrently<br/>(User A likes in US,<br/>User C likes in APAC,<br/>at nearly the same instant)"] --> B["Naive last-write-wins<br/>would LOSE one increment"]
    A --> C["CRDT G-Counter approach:<br/>each replica tracks its OWN<br/>increment count separately"]
    C --> D["US replica: +1 (its own increments)"]
    C --> E["APAC replica: +1 (its own increments)"]
    D & E --> F["Total count = SUM of all<br/>replicas' individual counts —<br/>merge is commutative,<br/>associative, and idempotent"]
    F --> G["BOTH increments are<br/>preserved correctly,<br/>regardless of the order<br/>replication messages arrive in"]
```

**Why this matters:** This is the elegant solution eventual consistency enables that strict linearizability wouldn't even need — because the operation (increment) is commutative, the system can accept concurrent local writes on every replica *and* guarantee mathematically correct convergence without ever needing cross-region coordination on the write path at all.

### 4.4 Tradeoff Benefit of This Choice

```mermaid
flowchart TB
    A["Every like/view is a<br/>fast, purely local operation"] --> B["Benefit: Very low latency<br/>(no cross-region round trip<br/>on the write path)"]
    A --> C["Benefit: Full availability —<br/>works even during a network<br/>partition; each side just<br/>keeps accepting local writes"]
    D["Cost: Brief, harmless<br/>staleness windows"] --> E["This tradeoff is CORRECT<br/>for this use case — a like<br/>counter that's occasionally<br/>off by a handful for a few<br/>hundred milliseconds costs<br/>nothing, while requiring<br/>consensus for every like<br/>would meaningfully hurt<br/>user experience at massive scale"]
```

---

## 5. Side-by-Side Comparison

```mermaid
flowchart LR
    subgraph Linear["Linearizable System<br/>(Bank Balance)"]
        L1["Single leader,<br/>quorum-committed writes"]
        L2["Every read reflects<br/>latest write, always"]
        L3["Reduced availability<br/>during partition"]
        L4["Higher latency<br/>per operation"]
    end

    subgraph Eventual["Eventually Consistent<br/>(Like Counter)"]
        E1["Multiple independent<br/>replicas, local writes"]
        E2["Reads may be briefly stale"]
        E3["Full availability,<br/>even during partition"]
        E4["Lower latency<br/>per operation"]
    end

    Middle["The CAP theorem in practice:<br/>during a partition, Linear chooses<br/>Consistency over Availability (CP);<br/>Eventual chooses Availability<br/>over Consistency (AP).<br/>Neither choice is universally<br/>'better' — it depends entirely<br/>on what an incorrect vs<br/>unavailable read costs you."]
```

---

## 6. Decision Framework — Choosing the Right Model

```mermaid
flowchart TB
    A["Does a stale read cause<br/>real harm (financial loss,<br/>double-booking, safety issue)?"] --> B{"Yes"}
    A --> C{"No — cosmetic/informational<br/>only, self-corrects quickly"}

    B --> D["Use linearizable/strong<br/>consistency<br/>Examples: bank balances,<br/>inventory counts, distributed<br/>locks, seat reservations"]

    C --> E["Use eventual consistency<br/>Examples: like/view counters,<br/>presence/last-seen status,<br/>recommendation caches,<br/>analytics dashboards"]

    F["Middle ground exists too:<br/>'read-your-writes' or<br/>'bounded staleness' consistency<br/>models for cases needing<br/>SOME guarantee but not full<br/>linearizability"] -.-> A
```

---

## 7. Component Responsibilities Summary

```mermaid
mindmap
  root((Consistency Models HLD))
    Linearizable System
      Single leader / consensus group
      Quorum-committed writes
      Used for: money, locks, inventory
    Eventually Consistent System
      Multiple independent replicas
      Async replication
      CRDT for conflict-free merging
      Used for: counters, presence, caches
    Decision Framework
      Cost of stale read vs cost of unavailability
      Not a technical choice alone —
      a business/product judgment call
```

---

## 8. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Bank balance consistency | Linearizable (consensus-based) | A stale read directly causes real financial harm (double-spending); correctness is non-negotiable even at latency/availability cost |
| Like counter consistency | Eventual (CRDT-based) | A stale read is cosmetically inconsequential and self-corrects within moments; forcing consensus here would only add cost with no real benefit |
| Conflict resolution (eventual) | CRDT (commutative merge) | Enables correct convergence without ANY cross-replica coordination on the write path, when the operation itself (increment) is naturally commutative |
| Availability during partition | System-dependent, deliberate choice | This is literally the CAP theorem's central tradeoff — there's no universally correct answer, only a correct answer for each specific use case |

---

## 9. Bottlenecks & Scaling Considerations

- **Linearizable systems don't scale writes horizontally** — since all writes must be serialized through a single logical point (leader/consensus group), throughput is fundamentally bounded regardless of how many nodes you add; this is precisely why linearizability is reserved for the specific data that truly needs it, not applied system-wide.
- **Eventually consistent systems must handle conflict resolution explicitly** — "eventually consistent" doesn't mean "no work required"; concurrent writes to the same key need a defined merge strategy (CRDT, last-write-wins with vector clocks, application-level reconciliation) or data can silently diverge incorrectly.
- **Mixed systems are the norm in practice** — real-world platforms (e.g., a social media app) use linearizable consistency for payment/billing data and eventual consistency for engagement metrics, within the SAME overall product — the consistency model is a per-data-type decision, not a single global architectural choice.
- **Monitoring convergence lag** — for eventually consistent systems, actively tracking replication lag matters; if lag grows unexpectedly (network issues, replica overload), staleness windows that were "a few hundred milliseconds, harmless" can silently become "minutes, user-visible and confusing."
- **Testing for the specific failure mode each model risks** — linearizable systems need chaos testing for partition/leader-failure correctness; eventually consistent systems need testing for conflict-resolution correctness under genuinely concurrent writes from multiple replicas.
