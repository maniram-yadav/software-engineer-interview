# Design an Event Sourcing System for a Banking Application — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Record every state-changing action (deposit, withdrawal, transfer) as an immutable event, rather than just storing current account balances
- Reconstruct an account's current state by replaying its event history
- Support point-in-time queries — "what was this account's balance as of last Tuesday?"
- Support snapshotting to avoid replaying an account's ENTIRE history on every read

### Non-Functional Requirements
- **Complete auditability:** Every single balance change must be traceable to a specific originating event — a hard regulatory requirement in banking
- **Correctness above all:** Financial calculations must be exact, with no possibility of silently-lost or duplicated transactions
- **Durability:** Events, once recorded, must never be lost — they ARE the source of truth, not a derived convenience
- **Reasonable read performance:** Despite being event-sourced, common operations (checking current balance) must remain fast

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Accounts | Millions |
| Transactions/sec (platform-wide) | Thousands |
| Avg events per account (over lifetime) | Hundreds to thousands |
| Snapshot interval | Every N events (e.g., 100) |

---

## 2. The Core Principle — Store What Happened, Not Just the Current State

```mermaid
flowchart TB
    A["Traditional approach: store<br/>CURRENT STATE directly —<br/>account_balance = $1,250,<br/>UPDATED in place with each<br/>transaction"] --> A1["Problem: once a value is<br/>overwritten, the HISTORY of<br/>HOW it got there is LOST —<br/>you know the current balance,<br/>but not the sequence of<br/>transactions that produced it,<br/>without a SEPARATE audit log<br/>that could drift out of sync<br/>with the actual balance"]

    B["Event Sourcing: NEVER store<br/>or update current state<br/>directly — instead, store the<br/>COMPLETE, IMMUTABLE SEQUENCE<br/>OF EVENTS that occurred<br/>(Deposited $500, Withdrew $200,<br/>...) — current state is always<br/>DERIVED by replaying these<br/>events, never stored as the<br/>primary source of truth"] --> B1["This makes the event log<br/>ITSELF the audit trail —<br/>there's no possibility of<br/>the 'current balance' and<br/>'transaction history'<br/>diverging, because the balance<br/>IS COMPUTED FROM the history,<br/>not maintained as a separate<br/>fact that could drift"]
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    Client["Banking Application<br/>(deposit/withdraw/transfer requests)"]

    subgraph CommandSide["Command Processing"]
        CommandHandler["Command Handler<br/>(validates business rules)"]
    end

    subgraph EventStore["Event Store"]
        EventLog[("Append-Only Event Log<br/>per account, ordered)")]
        SnapshotStore[("Periodic Snapshots<br/>(bounds replay cost)")]
    end

    subgraph QuerySide["Query/Read Side"]
        StateReconstructor["State Reconstruction<br/>(replay from snapshot + events)"]
        ReadCache[("Current-State Cache<br/>(for fast, common reads)")]
    end

    Client --> CommandHandler
    CommandHandler --> EventLog

    EventLog --> StateReconstructor
    SnapshotStore --> StateReconstructor
    StateReconstructor --> ReadCache

    Client -->|"Query balance"| ReadCache
```

**Key idea:** Writes (commands) always append new events to the immutable log — never modify existing entries. Reads reconstruct current state by starting from the nearest snapshot and replaying subsequent events — the same snapshot-plus-replay pattern established in both the WAL & Recovery System and Document Versioning designs, applied here to financial account state specifically.

---

## 4. Data Model

```mermaid
erDiagram
    ACCOUNT ||--o{ EVENT : "has history of"
    ACCOUNT ||--o{ SNAPSHOT : "has periodic"

    ACCOUNT {
        string account_id PK
        string current_snapshot_id FK
    }
    EVENT {
        string event_id PK
        string account_id FK
        long sequence_number "strictly increasing per account"
        string event_type "Deposited/Withdrew/TransferredOut/TransferredIn"
        float amount
        map metadata "originating transaction_id,<br/>actor, timestamp"
        timestamp occurred_at
    }
    SNAPSHOT {
        string snapshot_id PK
        string account_id FK
        long as_of_sequence_number
        float balance_at_snapshot
        timestamp created_at
    }
```

**Why the sequence number is strictly per-account, not global:** Each account's event history is an independent, ordered log — replaying account A's events never needs to consider account B's events at all, which is what allows different accounts to be processed and scaled completely independently of one another.

---

## 5. Command Processing (Write Path) — Detailed Sequence

```mermaid
sequenceDiagram
    participant Client as Banking App
    participant Handler as Command Handler
    participant Reconstructor as State Reconstructor
    participant EventLog as Event Log

    Client->>Handler: Command: Withdraw $200<br/>from account_123

    Handler->>Reconstructor: Get CURRENT state<br/>of account_123<br/>(needed to validate<br/>sufficient balance)
    Reconstructor-->>Handler: Current balance: $1,250

    Handler->>Handler: Validate business rule:<br/>$1,250 >= $200? Yes, proceed

    Handler->>EventLog: Append new event:<br/>{type: Withdrew, amount: 200,<br/>sequence: 847, account_id: 123}

    EventLog-->>Handler: Confirmed appended<br/>(durable)
    Handler-->>Client: Withdrawal successful,<br/>new balance: $1,050
```

**Why validation requires reconstructing current state FIRST:** Unlike a traditional system that can directly check a stored balance column, an event-sourced system must derive the current state (by replay) BEFORE it can validate a new command against business rules (e.g., "sufficient funds") — this is a genuine architectural cost of event sourcing, mitigated by the snapshot mechanism keeping reconstruction fast.

---

## 6. State Reconstruction (Read Path) — Detailed Sequence

```mermaid
sequenceDiagram
    participant Query as Balance Query
    participant Reconstructor as State Reconstructor
    participant SnapshotStore as Snapshot Store
    participant EventLog as Event Log

    Query->>Reconstructor: Get current balance<br/>for account_123

    Reconstructor->>SnapshotStore: Get latest snapshot<br/>for account_123
    SnapshotStore-->>Reconstructor: Snapshot at sequence 800:<br/>balance = $980

    Reconstructor->>EventLog: Get events for account_123<br/>WHERE sequence > 800

    EventLog-->>Reconstructor: [Deposited $300 (801),<br/>Withdrew $30 (802),<br/>... up to sequence 847]

    Reconstructor->>Reconstructor: Apply each event in order:<br/>$980 +300 -30 ... = $1,250

    Reconstructor-->>Query: Current balance: $1,250
```

**Why this stays fast even for old accounts with thousands of historical events:** Just as in the WAL & Recovery System and Document Versioning designs, the snapshot bounds the replay window — regardless of whether an account has 100 or 100,000 total historical events, reconstruction only ever needs to replay events SINCE the most recent snapshot, keeping read latency predictable and bounded.

---

## 7. Snapshotting Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Trigger as Snapshot Trigger<br/>(every N events, e.g., 100)
    participant EventLog as Event Log
    participant SnapshotStore as Snapshot Store

    Note over Trigger: Account 123 just reached<br/>its 900th event

    Trigger->>EventLog: Get all events from<br/>last snapshot (seq 800)<br/>through current (seq 900)
    EventLog-->>Trigger: 100 events

    Trigger->>Trigger: Replay to compute<br/>current balance: $1,450

    Trigger->>SnapshotStore: Store new snapshot:<br/>{sequence: 900, balance: $1,450}

    Note over SnapshotStore: Old snapshot (seq 800)<br/>can be safely archived/deleted —<br/>future reads only need the<br/>LATEST snapshot plus events<br/>since it, same as the WAL<br/>and Document Versioning<br/>designs' checkpoint pattern
```

---

## 8. Point-in-Time Queries (A Natural Consequence of Event Sourcing)

```mermaid
flowchart TB
    A["Regulatory/audit request:<br/>'What was account_123's<br/>balance at 3pm last<br/>Tuesday?'"] --> B["Find the snapshot at or<br/>before that timestamp"]
    B --> C["Replay events from that<br/>snapshot forward, but ONLY<br/>up to events with<br/>occurred_at <= the target<br/>timestamp"]
    C --> D["This produces the EXACT<br/>historical balance as it<br/>genuinely existed at that<br/>moment — a capability that's<br/>NATURALLY available in an<br/>event-sourced system, but<br/>would require a SEPARATE,<br/>purpose-built historical<br/>tracking mechanism in a<br/>traditional current-state-only<br/>system"]
```

**Why this is a genuine architectural advantage, not just a nice side effect:** In a traditional system storing only current balance, answering "what was the balance at time T" requires EITHER a separate point-in-time snapshot system (extra infrastructure) OR is simply IMPOSSIBLE if that historical data was never separately captured. In event sourcing, this capability falls out naturally from the fundamental way the system already stores data — connecting directly to the same point-in-time correctness principle explored in the Feature Store design's training data generation.

---

## 9. Handling Transfers (Multi-Account Events)

```mermaid
sequenceDiagram
    participant Client as Banking App
    participant Handler as Command Handler
    participant EventLogA as Account A's Event Log
    participant EventLogB as Account B's Event Log

    Client->>Handler: Transfer $500 from<br/>Account A to Account B

    Handler->>Handler: Validate Account A<br/>has sufficient balance<br/>(reconstruct current state)

    Note over Handler: This is a TRANSACTIONAL<br/>operation spanning TWO<br/>independent event logs —<br/>same coordination challenge<br/>as the Distributed Transaction<br/>Saga design

    Handler->>EventLogA: Append: TransferredOut<br/>{amount: 500, to: B,<br/>transaction_id: T789}
    Handler->>EventLogB: Append: TransferredIn<br/>{amount: 500, from: A,<br/>transaction_id: T789}

    Note over EventLogA,EventLogB: Both events share the SAME<br/>transaction_id — allowing<br/>this multi-account operation<br/>to be traced and verified<br/>as a single logical unit,<br/>even though it's recorded<br/>as two separate events in<br/>two separate account logs
```

**Why the shared `transaction_id` matters for correctness verification:** Since each account maintains an independent event log, a transfer necessarily produces two separate events — the shared transaction_id lets auditors and reconciliation processes verify that every "TransferredOut" event has a matching "TransferredIn" event elsewhere, catching any inconsistency (e.g., a partial failure that recorded one side but not the other) that would otherwise be invisible.

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Event Sourcing Banking HLD))
    Command Handler
      Validates business rules
      Requires current state reconstruction first
    Event Log
      Append-only, immutable
      Per-account, strictly ordered
    Snapshot Store
      Periodic state checkpoints
      Bounds replay cost
    State Reconstructor
      Snapshot plus replay
      Powers both current and point-in-time queries
    Read Cache
      Fast path for common balance checks
      Derived from reconstruction, not source of truth
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Core storage model | Immutable event log as source of truth, not current-state storage | Makes the audit trail structurally guaranteed to match reality — current state is DERIVED from history, never divergent from it |
| Read performance | Snapshot + replay | Bounds reconstruction cost regardless of an account's total historical event count, mirroring the WAL and Document Versioning designs' checkpoint pattern |
| Point-in-time queries | Natural consequence of the event-sourced model | Falls out of the architecture directly, rather than requiring separate historical-tracking infrastructure |
| Multi-account transactions | Shared transaction_id across separate per-account events | Enables cross-log consistency verification despite each account's log being independently managed |
| Write validation | Requires state reconstruction before command processing | A genuine architectural cost of event sourcing, directly mitigated by snapshot-bounded reconstruction speed |

---

## 12. Bottlenecks & Scaling Considerations

- **Write validation latency** — since every command requires reconstructing current state FIRST (to validate business rules like sufficient balance), this adds a read-before-write step to every single transaction; the read cache and snapshot mechanism directly mitigate this, but it remains a genuine architectural overhead compared to a traditional direct-update system.
- **Snapshot frequency tuning** — same fundamental tradeoff as the WAL & Recovery System design: frequent snapshots increase storage/compute overhead, infrequent snapshots increase replay cost for reads — tuned based on actual per-account transaction velocity.
- **Cross-account transaction coordination** — transfers spanning two accounts introduce the same distributed coordination challenges covered in the Distributed Transaction Saga design; a partial failure (one side's event recorded, the other's not) requires reconciliation processes actively monitoring for unmatched transaction_ids.
- **Event schema evolution** — as the banking application's business logic evolves over years, the STRUCTURE of events themselves may need to change (new event types, additional metadata fields); since old events are immutable and permanent, the state reconstruction logic must remain capable of correctly interpreting events recorded under OLDER schema versions indefinitely, not just the current one.
- **Storage growth over very long account lifetimes** — accounts open for decades accumulate enormous event histories; while snapshotting bounds READ cost, the underlying storage for the full historical log continues growing and must be planned for at platform scale, though this is generally an acceptable cost given banking's inherent regulatory requirement to retain complete transaction history regardless of storage architecture chosen.
- **Regulatory reporting and compliance queries** — beyond simple point-in-time balance queries, real banking compliance often requires more complex historical analysis (e.g., "show me all transactions over $10,000 in the last year across all accounts") — this typically requires a SEPARATE analytical system fed by the event stream (similar to the OLAP System design's ingestion approach), since the account-by-account event logs aren't optimized for this kind of cross-account analytical query pattern.
- **Testing reconstruction correctness** — because state is always DERIVED, not directly observed, thorough testing must verify that replaying any given event sequence ALWAYS produces the mathematically correct resulting state — this is a different testing discipline than verifying direct state mutations, requiring careful property-based or exhaustive-sequence testing of the event-application logic itself.
