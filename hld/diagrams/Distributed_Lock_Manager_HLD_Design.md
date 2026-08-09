# Design a Distributed Lock Manager — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Acquire and release locks on named resources across multiple processes/machines
- Support both exclusive locks and (optionally) shared/read locks
- Locks must automatically expire if the holder crashes (no permanent deadlock)
- Support reentrant locks (same holder can re-acquire without deadlocking itself, optional)
- Notify waiters when a lock becomes available (optional, vs pure polling)

### Non-Functional Requirements
- **Safety (correctness) above all:** Two clients must never simultaneously believe they hold the same exclusive lock — this is the entire point of the system
- **Liveness:** A crashed lock holder must not permanently block others from acquiring the lock
- **Availability:** The lock service itself must survive node failures without losing lock state
- **Low latency:** Lock acquisition should be fast (< 10-50ms) since it's often on a hot path
- **Fencing:** Must protect against the "paused/slow client thinks it still holds the lock" problem

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Lock acquisitions/sec (platform-wide) | ~10,000-100,000 |
| Typical lock hold duration | Milliseconds to seconds |
| Concurrent distinct locks | Millions (many different resources) |
| Lock service nodes | Odd number (3, 5, 7) for consensus quorum |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    subgraph Clients["Client Processes"]
        C1["Client 1<br/>(Service Instance A)"]
        C2["Client 2<br/>(Service Instance B)"]
        C3["Client 3<br/>(Service Instance C)"]
    end

    subgraph LockCluster["Lock Manager Cluster (Consensus-based)"]
        Leader["Leader Node<br/>(handles all writes)"]
        Follower1["Follower Node 1"]
        Follower2["Follower Node 2"]
    end

    subgraph Storage["Replicated Log / State"]
        Log[("Replicated Log<br/>(Raft/Paxos)"]
        LockTable[("Lock State Table<br/>resource → holder, expiry, fence_token")]
    end

    ProtectedResource["Protected Resource<br/>(e.g., shared file, DB row,<br/>critical section)"]

    C1 <-->|"Acquire/Release"| Leader
    C2 <-->|"Acquire/Release"| Leader
    C3 <-->|"Acquire/Release"| Leader

    Leader -->|"Replicate"| Follower1
    Leader -->|"Replicate"| Follower2
    Leader --> Log --> LockTable

    C1 -.->|"Access resource with<br/>fence token"| ProtectedResource
```

**Key idea:** The lock manager itself is a small distributed system requiring **consensus** (Raft/Paxos-based, like ZooKeeper/etcd/Consul) — because the lock state (who holds what) must never diverge between nodes, or two clients could be told by different nodes that they each hold the same exclusive lock.

---

## 3. Data Model

```mermaid
erDiagram
    LOCK {
        string resource_id PK
        string holder_id
        long fence_token "monotonically increasing"
        timestamp acquired_at
        timestamp expires_at
        string lock_type "exclusive/shared"
    }
    LOCK_WAITER {
        string resource_id FK
        string waiter_id
        timestamp queued_at
    }
```

---

## 4. Basic Lock Acquisition Flow

```mermaid
sequenceDiagram
    participant C as Client
    participant Leader as Lock Manager (Leader)
    participant Log as Replicated Log
    participant R as Protected Resource

    C->>Leader: ACQUIRE_LOCK(resource_id=X, ttl=30s)
    Leader->>Leader: Check LockTable: is X currently held?
    alt Lock available
        Leader->>Log: Propose: grant lock to C, fence_token = N+1
        Log->>Log: Replicate to quorum of followers
        Log-->>Leader: Committed (majority ack)
        Leader-->>C: GRANTED (fence_token = N+1, expires_at)
        C->>R: Perform operation, presenting fence_token N+1
    else Lock held by another client
        Leader-->>C: DENIED (or queued, depending on mode)
    end
```

---

## 5. The Fencing Token Problem — Why Simple Locks Aren't Enough

```mermaid
sequenceDiagram
    participant C1 as Client 1
    participant Leader as Lock Manager
    participant R as Protected Resource (e.g., storage)

    C1->>Leader: ACQUIRE_LOCK(X)
    Leader-->>C1: GRANTED (fence_token = 33)

    Note over C1: Client 1 experiences a long GC pause<br/>(stops responding for 40 seconds)

    Note over Leader: Lock TTL (30s) expires while C1 is paused
    Leader->>Leader: Lock X expires, becomes available

    participant C2 as Client 2
    C2->>Leader: ACQUIRE_LOCK(X)
    Leader-->>C2: GRANTED (fence_token = 34)

    Note over C1: Client 1 resumes from GC pause,<br/>doesn't know its lock expired!
    C1->>R: Write to resource<br/>(presents STALE fence_token = 33)

    R->>R: Check: is 33 >= last_seen_token (34)?
    R-->>C1: REJECTED — token 33 is stale<br/>(a higher token 34 already seen)

    C2->>R: Write to resource<br/>(presents fence_token = 34)
    R->>R: 34 >= last_seen_token (34) — accept, update last_seen to 34
    R-->>C2: Accepted
```

**Why this matters — this is the crux of the whole problem:** A lock's TTL expiring doesn't guarantee the original holder has actually stopped working (it might just be paused — GC, slow network, OS scheduling). The **fencing token** (a monotonically increasing number issued with each lock grant) lets the *protected resource itself* reject stale writes from a client that no longer legitimately holds the lock, even if that client doesn't know it lost the lock. Without this, a naive TTL-based lock provides an illusion of safety, not real safety.

---

## 6. Lock Release & Automatic Expiry

```mermaid
flowchart TB
    A["Client explicitly releases lock"] --> B["Leader updates LockTable:<br/>resource marked available"]
    A2["Client never releases<br/>(crash/network partition)"] --> C["TTL-based expiry:<br/>background sweep or lazy check<br/>on next acquisition attempt"]
    B --> D["Notify next waiter<br/>(if using queued/notify mode)"]
    C --> D
```

```mermaid
sequenceDiagram
    participant Sweep as Expiry Sweeper (background)
    participant LockTable as Lock State Table
    participant Waiter as Next Waiting Client

    loop Periodic sweep
        Sweep->>LockTable: Find locks WHERE expires_at < now()
        LockTable-->>Sweep: Expired lock: resource X, held by Client 1
        Sweep->>LockTable: Mark resource X as available<br/>(fence_token counter NOT reset — keeps incrementing)
        Sweep->>Waiter: Notify if a waiter is queued for X
    end
```

---

## 7. Lock Renewal (Heartbeating) for Long-Running Operations

```mermaid
sequenceDiagram
    participant C as Client (long-running task)
    participant Leader as Lock Manager

    C->>Leader: ACQUIRE_LOCK(X, ttl=10s)
    Leader-->>C: GRANTED (fence_token=50, expires in 10s)

    loop Every 3-4 seconds (well within TTL)
        C->>Leader: RENEW_LOCK(X, fence_token=50)
        Leader->>Leader: Verify C still holds this token,<br/>extend expires_at
        Leader-->>C: Renewed, new expiry
    end

    Note over C: If C crashes, heartbeats stop,<br/>lock naturally expires via TTL<br/>— no manual cleanup needed

    C->>Leader: Task complete — RELEASE_LOCK(X, fence_token=50)
    Leader-->>C: Released
```

*Short TTLs with frequent renewal (rather than one long TTL) give faster failure detection — if a client crashes, the lock becomes available again within one TTL period rather than however long the original operation was expected to take.*

---

## 8. Consensus Layer — Why Raft/Paxos Underpins This

```mermaid
flowchart TB
    A["Lock grant decision"] --> B["Must be agreed upon by<br/>a majority of lock manager nodes"]
    B --> C["Leader proposes: 'grant lock to Client X'"]
    C --> D["Followers vote/acknowledge"]
    D --> E{"Majority (quorum)<br/>acknowledges?"}
    E -- Yes --> F["Committed — now the<br/>official, durable state"]
    E -- No --> G["Not committed —<br/>grant does not take effect"]

    H["Leader crashes"] --> I["Remaining nodes elect<br/>new leader via Raft"]
    I --> J["New leader has full<br/>replicated log —<br/>no lock state lost"]
```

*This is precisely why production distributed locks are built on top of consensus systems like ZooKeeper, etcd, or Consul rather than a single arbitrarily-chosen coordinator node — a single node granting locks unilaterally would be a single point of failure and, worse, a single point of potential incorrectness during a network partition.*

---

## 9. Handling Network Partitions (Split-Brain Prevention)

```mermaid
flowchart TB
    A["Network partition splits<br/>lock cluster into two groups"] --> B["Group 1: 2 nodes<br/>(has majority, out of 3)"]
    A --> C["Group 2: 1 node<br/>(minority)"]

    B --> D["Group 1 can still elect leader,<br/>commit new lock grants —<br/>maintains quorum"]
    C --> E["Group 2 CANNOT commit anything<br/>— no quorum, refuses writes"]

    F["Result: only ONE side<br/>of the partition can grant locks —<br/>no split-brain possible"]
```

**Why odd-numbered clusters (3, 5, 7):** A quorum-based system needs a strict majority to make progress. With an odd number of nodes, any network partition can produce at most one side with a majority — guaranteeing at most one "active" side can ever grant locks, which is what prevents split-brain lock grants.

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Distributed Lock Manager HLD))
    Leader Node
      Handles all lock grant/release requests
      Proposes state changes to log
    Consensus Log
      Raft/Paxos replicated log
      Source of truth for lock state
    Lock State Table
      resource to holder mapping
      Fencing token counter per resource
    Expiry Sweeper
      TTL-based automatic release
      Handles crashed-holder recovery
    Fencing Token
      Monotonic counter per lock
      Enforced by protected resource itself
    Client SDK
      Acquire/release/renew API
      Heartbeat-based renewal
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Underlying consensus | Raft/Paxos-based cluster (odd node count) | Only mechanism that provides both safety (no split-brain) and liveness (survives node failure) for lock state |
| Failure detection | TTL-based expiry, not indefinite hold | A crashed client must not permanently block a resource; TTL bounds the worst-case unavailability |
| Stale-holder protection | Fencing tokens, checked by the protected resource | TTL expiry alone can't guarantee the original holder has actually stopped — the resource itself must be the final enforcement point |
| Renewal | Short TTL + frequent heartbeat renewal | Faster failure detection than one long TTL matched to expected task duration |
| Lock queueing | Notify-based (not busy-polling) where possible | Reduces load on the lock service and improves fairness/latency for waiters |

---

## 12. Bottlenecks & Scaling Considerations

- **Leader as a bottleneck** — all writes go through a single leader in Raft-style systems; for extremely high lock-acquisition throughput, consider partitioning locks across multiple independent consensus groups (sharding by resource_id hash) rather than one global cluster.
- **Fencing token adoption gap** — the fencing token only protects against stale writes if the *protected resource itself* checks and enforces it; this requires cooperation from every system that uses the lock — a weak link if any integration skips the check.
- **TTL tuning tradeoff** — too short causes false expiry under normal GC pauses/network hiccups (leading to two "valid" holders briefly, mitigated only by fencing); too long delays recovery from genuine crashes. Must be tuned per use case, ideally paired with heartbeat renewal rather than one static long TTL.
- **Thundering herd on lock release** — many waiters for a popular resource all attempting to acquire simultaneously when it's released; needs either queued/notify semantics or randomized backoff on retry to avoid a stampede.
- **Clock dependency for TTL** — expiry timing depends on the lock manager's own clock, not the client's; clients experiencing clock drift relative to the server must still respect the server's determination of expiry, which is why fencing tokens (not client-side timers) are the real safety mechanism.
- **Cross-region latency** — a lock manager cluster spanning regions adds significant acquisition latency due to consensus round-trips; most designs keep the lock service regional/local to where it's used, accepting that cross-region coordination needs a different mechanism entirely.
