# Design a Distributed Job Scheduler / Task Queue — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Schedule jobs to run at a specific time or on a recurring cron-like schedule
- Support immediate (fire-and-forget) task queueing in addition to scheduled jobs
- Guarantee job execution — no silently dropped jobs
- Support job priorities, retries with backoff, and dead-letter handling for permanently failing jobs
- Support job dependencies (job B runs only after job A completes)
- Idempotent execution — a job accidentally run twice shouldn't cause harmful side effects

### Non-Functional Requirements
- **Exactly-once-ish execution:** In practice, "effectively-once" via at-least-once delivery + idempotent handlers
- **Scale:** Millions of scheduled jobs, tens of thousands of jobs executing per second at peak
- **Reliability:** Scheduler crash must not lose or duplicate pending jobs
- **Timing accuracy:** Scheduled jobs should fire within a small tolerance window (e.g., ± few seconds)
- **Horizontal scalability:** Adding more worker capacity should linearly increase throughput

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Total scheduled jobs (at rest) | ~100M |
| Jobs executing/sec (peak) | ~50,000 |
| Avg job execution time | Varies: ms (simple) to minutes (batch) |
| Cron-style recurring jobs | ~1M unique schedules |
| Worker fleet size | Elastic, scales with queue depth |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    subgraph Producers["Job Producers"]
        AppSvc["Application Services<br/>(submit jobs via API)"]
        CronDef["Cron/Recurring Job Definitions"]
    end

    subgraph Scheduling["Scheduling Layer"]
        SchedulerSvc["Scheduler Service<br/>(time-based trigger)"]
        JobStore[("Job Definition Store<br/>(durable DB — cron schedules,<br/>one-time jobs, metadata)")]
    end

    subgraph Queueing["Queue Layer"]
        PriorityQueue["Priority Queues<br/>(per job type/priority tier)"]
        DelayQueue["Delay Queue<br/>(jobs not yet due)"]
    end

    subgraph Execution["Execution Layer"]
        WorkerPool["Worker Pool<br/>(auto-scaling)"]
        LeaseManager["Lease/Lock Manager<br/>(prevents duplicate execution)"]
    end

    subgraph Reliability["Reliability Layer"]
        RetryHandler["Retry Handler<br/>(exponential backoff)"]
        DLQ["Dead Letter Queue"]
    end

    AppSvc --> SchedulerSvc
    CronDef --> SchedulerSvc
    SchedulerSvc --> JobStore

    SchedulerSvc -->|"Job due now"| PriorityQueue
    SchedulerSvc -->|"Job due later"| DelayQueue
    DelayQueue -->|"Becomes due"| PriorityQueue

    PriorityQueue --> WorkerPool
    WorkerPool --> LeaseManager
    LeaseManager --> JobStore

    WorkerPool -->|"Failure"| RetryHandler
    RetryHandler -->|"Retry"| PriorityQueue
    RetryHandler -->|"Max retries exceeded"| DLQ
```

**Key idea:** Scheduling (deciding *when* a job should run) and execution (actually running it) are cleanly separated. The Scheduler Service continuously scans for jobs coming due and moves them into execution-ready queues; workers pull from those queues independently, so scheduling accuracy and execution throughput can scale independently of each other.

---

## 3. Data Model

```mermaid
erDiagram
    JOB_DEFINITION ||--o{ JOB_EXECUTION : "triggers instances of"
    JOB_EXECUTION ||--o{ EXECUTION_ATTEMPT : "has retry attempts"

    JOB_DEFINITION {
        string job_id PK
        string type "one-time/recurring"
        string cron_expression "nullable"
        timestamp scheduled_at "nullable, for one-time"
        string payload
        int priority
        int max_retries
        string status "active/paused/deleted"
    }
    JOB_EXECUTION {
        string execution_id PK
        string job_id FK
        timestamp due_at
        string status "pending/leased/running/completed/failed/dead"
        string idempotency_key
    }
    EXECUTION_ATTEMPT {
        string attempt_id PK
        string execution_id FK
        int attempt_number
        string worker_id
        timestamp started_at
        timestamp finished_at
        string result
        string error_message
    }
```

---

## 4. Job Scheduling Flow (Cron-style Recurring Jobs)

```mermaid
sequenceDiagram
    participant Def as Job Definition Store
    participant Sched as Scheduler Service
    participant DelayQ as Delay Queue
    participant PQ as Priority Queue

    loop Every polling interval (e.g., 1s)
        Sched->>Def: Query jobs WHERE next_run_at <= now() + lookahead_window
        Def-->>Sched: List of due/upcoming jobs

        loop For each job
            Sched->>Sched: Create JOB_EXECUTION instance<br/>(idempotency_key = job_id + scheduled_time)
            alt Due now
                Sched->>PQ: Enqueue directly for execution
            else Due soon (within lookahead window)
                Sched->>DelayQ: Enqueue with precise fire time
            end
            Sched->>Def: Update next_run_at<br/>(compute from cron expression)
        end
    end

    loop Continuously
        DelayQ->>DelayQ: Check for entries whose<br/>fire time has arrived
        DelayQ->>PQ: Move due entries to<br/>execution-ready queue
    end
```

**Why a lookahead window + delay queue:** Rather than the scheduler doing a fresh DB scan at the exact millisecond every job is due (expensive and imprecise at scale), it pulls upcoming jobs into an in-memory/Redis-backed delay queue slightly ahead of time, then fires them precisely from there — decoupling expensive DB polling from precise timing.

---

## 5. Job Execution Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant PQ as Priority Queue
    participant W as Worker
    participant Lease as Lease Manager
    participant Store as Job Store
    participant Retry as Retry Handler

    W->>PQ: Poll for next job
    PQ-->>W: Job (execution_id: X)

    W->>Lease: Acquire lease on execution_id X<br/>(TTL: expected_duration + buffer)
    alt Lease acquired
        Lease-->>W: Granted
        W->>Store: Update status = RUNNING
        W->>W: Execute job payload

        alt Success
            W->>Store: Update status = COMPLETED
            W->>Lease: Release lease
        else Failure
            W->>Retry: Report failure
            Retry->>Retry: Check attempt_count vs max_retries
            alt Retries remaining
                Retry->>PQ: Re-enqueue with backoff delay
            else Max retries exceeded
                Retry->>DLQ: Move to dead letter queue
            end
            W->>Lease: Release lease
        end
    else Lease already held (another worker has it)
        Lease-->>W: Denied
        W->>PQ: Skip, poll next job
    end
```

**Why leases, not simple dequeue-and-delete:** If a worker crashes mid-execution after dequeuing a job but before marking it complete, a naive "delete on dequeue" approach loses the job entirely. Leases with a TTL ensure that if a worker dies mid-job, the lease expires and another worker can safely pick it up — at the cost of potential duplicate execution, which is why idempotency matters.

---

## 6. Preventing Duplicate Execution (Idempotency)

```mermaid
flowchart TB
    A["Worker about to execute job"] --> B["Check idempotency_key<br/>in execution ledger"]
    B --> C{"Already marked<br/>COMPLETED for this key?"}
    C -- Yes --> D["Skip execution<br/>(job already done,<br/>this is a duplicate delivery)"]
    C -- No --> E["Proceed with execution"]
    E --> F["On success: atomically mark<br/>idempotency_key as COMPLETED"]

    G["Job handler itself<br/>should also be idempotent"] -.-> H["e.g., 'charge customer $10'<br/>should check 'already charged for<br/>this invoice_id' internally too —<br/>defense in depth"]
```

*Since the lease mechanism only provides **at-least-once** delivery (not exactly-once), true safety requires the job execution ledger to dedupe by idempotency key, AND ideally the job's own business logic to be idempotent as a second layer of protection.*

---

## 7. Retry with Exponential Backoff

```mermaid
flowchart TB
    A["Job fails on attempt 1"] --> B["Backoff = base_delay × 2^attempt<br/>+ random jitter"]
    B --> C["Attempt 1 fails → wait ~2s"]
    C --> D["Attempt 2 fails → wait ~4s"]
    D --> E["Attempt 3 fails → wait ~8s"]
    E --> F{"attempt_count >=<br/>max_retries?"}
    F -- No --> G["Re-enqueue with<br/>calculated delay"]
    F -- Yes --> H["Move to Dead Letter Queue<br/>Alert/page for manual review"]
```

*Jitter (small randomization added to the delay) prevents synchronized retry storms — if 10,000 jobs fail simultaneously due to a downstream outage, staggered jitter prevents them all from retrying at the exact same moment and re-overwhelming the recovering downstream service.*

---

## 8. Job Dependencies (DAG Execution)

```mermaid
flowchart TB
    A["Job A<br/>(extract data)"] --> B["Job B<br/>(transform data)"]
    A --> C["Job C<br/>(validate data)"]
    B --> D["Job D<br/>(load to warehouse)"]
    C --> D

    E["Dependency Tracker"] -.-> F["Job D only enqueued<br/>once BOTH B and C<br/>report COMPLETED"]
```

```mermaid
sequenceDiagram
    participant JobB as Job B (completes)
    participant Tracker as Dependency Tracker
    participant Store as Job Store
    participant PQ as Priority Queue

    JobB->>Tracker: Report completion
    Tracker->>Store: Check all dependencies of Job D
    Store-->>Tracker: Job C status: still PENDING
    Tracker->>Tracker: Job D not ready yet, wait

    Note over Tracker: Later, Job C also completes
    Tracker->>Store: Check all dependencies of Job D
    Store-->>Tracker: Job B: COMPLETED, Job C: COMPLETED
    Tracker->>PQ: All dependencies satisfied — enqueue Job D
```

---

## 9. Distributed Scheduler High Availability

```mermaid
flowchart TB
    A["Multiple Scheduler Service<br/>instances running"] --> B{"Leader Election<br/>(via ZooKeeper/etcd)"}
    B --> C["Active Leader:<br/>Scans job store, enqueues due jobs"]
    B --> D["Standby Instances:<br/>Idle, monitoring leader health"]

    C --> E["Leader crashes/network partition"]
    E --> F["Standbys detect via<br/>lost heartbeat/lease expiry"]
    F --> G["New leader elected"]
    G --> H["Resumes scanning —<br/>no jobs lost since job store<br/>(not leader memory) is source of truth"]
```

*Only one scheduler instance should be actively scanning and enqueueing at a time to avoid duplicate enqueueing of the same due job — leader election ensures this while providing automatic failover.*

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Job Scheduler HLD))
    Scheduler Service
      Leader-elected, single active scanner
      Cron expression evaluation
      Moves due jobs to execution queues
    Delay Queue
      Holds near-future jobs
      Precise fire-time triggering
    Priority Queues
      Execution-ready jobs
      Multiple tiers by priority
    Worker Pool
      Auto-scaling based on queue depth
      Lease-based job claiming
    Lease Manager
      TTL-based job locks
      Prevents concurrent duplicate execution
    Retry Handler
      Exponential backoff with jitter
      Routes to DLQ after max attempts
    Dependency Tracker
      DAG-based job chaining
      Enqueues downstream jobs on completion
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Scheduling vs execution | Cleanly separated layers | Allows independent scaling — scheduling accuracy doesn't depend on execution throughput and vice versa |
| Delivery guarantee | At-least-once + idempotency keys | True exactly-once is effectively impossible in distributed systems; at-least-once + dedup achieves the same practical outcome |
| Duplicate prevention | Lease-based locking with TTL | Balances safety (job never truly lost) against liveness (crashed worker's job is eventually retried, not stuck forever) |
| Retry strategy | Exponential backoff + jitter | Prevents retry storms from synchronized failures overwhelming a recovering downstream dependency |
| Scheduler HA | Leader election, single active scanner | Prevents duplicate enqueueing while still providing automatic failover |
| Job dependencies | DAG tracking via completion events | Enables complex pipelines (ETL-style) without requiring jobs to poll for their dependencies' status |

---

## 12. Bottlenecks & Scaling Considerations

- **Scheduler DB polling at scale** — scanning millions of job definitions for "what's due soon" needs an efficient index (e.g., on `next_run_at`) and a bounded lookahead window; naive full-table scans won't keep up at scale.
- **Lease TTL tuning** — too short risks a slow-but-healthy job being falsely reclaimed by another worker (duplicate execution); too long delays recovery if a worker actually crashes. Should be set based on realistic job duration estimates, ideally with periodic lease renewal (heartbeating) for long-running jobs.
- **Hot job types dominating queue capacity** — a single very high-frequency job type could starve other job types; use separate priority queues per job category, not one global queue.
- **Dead letter queue growth** — DLQ entries need active monitoring/alerting, not silent accumulation — a growing DLQ usually signals a systemic downstream issue worth paging on.
- **Clock synchronization** — scheduler instances and workers must agree on "now" for accurate due-time comparisons; NTP-synced clocks are assumed, but the delay-queue design should tolerate small skew gracefully.
- **Thundering herd on recovery** — after a downstream outage resolves, many backed-up retrying jobs firing simultaneously can re-trigger the outage; jittered backoff (above) plus optional gradual queue-drain rate limiting mitigates this.
- **Recurring job schedule drift** — computing `next_run_at` from a cron expression must be done relative to the *scheduled* time, not the *actual execution* time, to avoid gradual drift if jobs occasionally run late.
