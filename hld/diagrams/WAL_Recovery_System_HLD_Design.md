# Design a Write-Ahead Log (WAL) & Recovery System for a Custom Database Engine — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Every data-modifying operation must be durably logged before being applied to the main data store
- On crash/restart, the database must be able to recover to a consistent state, replaying any committed-but-not-yet-flushed changes
- Support atomic multi-operation transactions (all-or-nothing)
- Enable point-in-time recovery (restore to any prior committed state, for backup/restore scenarios)

### Non-Functional Requirements
- **Durability (the paramount property):** Once a write is acknowledged to the client, it must survive any subsequent crash — no exceptions
- **Performance:** WAL writes are on the critical path of every transaction; must be optimized for sequential I/O
- **Recovery speed:** Time to recover after a crash should be bounded and predictable, not proportional to the entire database history
- **Correctness under partial writes:** Must handle the case where a crash occurs mid-write to the log itself

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Transactions/sec | ~50,000 |
| Avg WAL record size | ~200 bytes |
| WAL write throughput | ~10MB/sec sustained |
| Checkpoint interval | Every few minutes or N MB of WAL growth |
| Target recovery time | Seconds to low minutes, even after a hard crash |

---

## 2. The Core Principle — Why "Write-Ahead"

```mermaid
flowchart TB
    A["Naive approach: modify data<br/>pages directly on disk,<br/>THEN consider the write done"] --> B["Problem: if the process<br/>crashes mid-write to a data<br/>page (e.g., power loss during<br/>a multi-byte disk write),<br/>the page can be left in a<br/>CORRUPTED, inconsistent state<br/>— neither the old nor the<br/>new value, but garbage"]

    C["Write-Ahead Logging principle:<br/>NEVER modify the actual data<br/>page until the INTENDED change<br/>has been durably recorded in<br/>a separate, simple, append-only<br/>log first"] --> D["If a crash occurs before the<br/>data page is updated, the WAL<br/>entry still exists — recovery<br/>can replay it. If a crash<br/>occurs mid-page-update, the<br/>WAL entry lets recovery<br/>REDO the change correctly<br/>from a known-good starting point"]
```

**The fundamental guarantee this provides:** By ensuring the log record hits durable storage *before* the corresponding data page modification, the system guarantees it can always reconstruct the correct final state after a crash — the log becomes the authoritative record of "what should have happened," independent of whether the actual data pages were fully updated when the crash occurred.

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    Client["Client Transaction"]

    subgraph Engine["Database Engine"]
        TxnMgr["Transaction Manager"]
        BufferPool["Buffer Pool<br/>(in-memory page cache)"]
        WALWriter["WAL Writer"]
    end

    subgraph Storage["Durable Storage"]
        WALFile[("WAL File<br/>(sequential, append-only)")]
        DataFiles[("Data Files<br/>(actual table/index pages)")]
        Checkpoint[("Checkpoint Markers")]
    end

    Client --> TxnMgr
    TxnMgr --> WALWriter
    WALWriter -->|"1. Write log record<br/>+ fsync (durable)"| WALFile
    TxnMgr -->|"2. THEN modify<br/>in-memory page"| BufferPool
    BufferPool -.->|"3. Eventually flushed<br/>to disk (lazily, async)"| DataFiles

    TxnMgr --> Checkpoint
```

**Key idea:** Note the strict ordering — the WAL write (step 1, synchronous, durable) always happens **before** the in-memory buffer pool modification (step 2), and the actual data file update (step 3) can happen much later, asynchronously. This ordering is what the entire durability guarantee rests on.

---

## 4. WAL Record Structure

```mermaid
erDiagram
    WAL_RECORD {
        long lsn PK "Log Sequence Number, monotonic"
        long transaction_id
        string operation_type "INSERT/UPDATE/DELETE/COMMIT/ABORT"
        string table_name
        bytes before_image "old value, for undo"
        bytes after_image "new value, for redo"
        long prev_lsn_for_txn "links to this txn's previous record"
    }
```

**Why both before-image and after-image are stored:** The after-image supports **redo** (reapplying a committed change that didn't make it to the data file before a crash), while the before-image supports **undo** (rolling back a change from a transaction that was in-progress but never committed when the crash occurred). Having both is what enables the full ARIES-style recovery algorithm (covered below).

---

## 5. Transaction Write Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client
    participant TxnMgr as Transaction Manager
    participant WAL as WAL Writer
    participant WALFile as WAL File (disk)
    participant Buffer as Buffer Pool (memory)

    C->>TxnMgr: BEGIN TRANSACTION
    TxnMgr->>TxnMgr: Assign transaction_id

    C->>TxnMgr: UPDATE row X SET value=100
    TxnMgr->>WAL: Log record: {txn_id, UPDATE,<br/>before=50, after=100, lsn=1001}
    WAL->>WALFile: Append + fsync<br/>(BLOCKS until durable on disk)
    WALFile-->>WAL: Durable

    WAL-->>TxnMgr: Log write confirmed
    TxnMgr->>Buffer: NOW modify the in-memory<br/>page (row X = 100)

    C->>TxnMgr: COMMIT
    TxnMgr->>WAL: Log record: {txn_id, COMMIT, lsn=1002}
    WAL->>WALFile: Append + fsync
    WALFile-->>WAL: Durable

    TxnMgr-->>C: Transaction committed<br/>(ONLY after COMMIT record<br/>is durably logged)

    Note over Buffer: The actual data page might<br/>still only exist in memory —<br/>it will be flushed to the<br/>data file LATER, asynchronously,<br/>during normal buffer pool<br/>eviction or the next checkpoint
```

**Why the client only gets "committed" after the COMMIT log record is durable:** This is the precise moment the durability guarantee kicks in — even if the process crashes one instruction later, the WAL contains everything needed to reconstruct this transaction's effects on restart. The actual data page update can safely lag behind in memory.

---

## 6. Checkpointing (Bounding Recovery Time)

```mermaid
flowchart TB
    A["Problem without checkpoints:<br/>after a crash, recovery would<br/>need to replay the ENTIRE WAL<br/>from the very beginning of<br/>time — unbounded, growing<br/>recovery time as the database<br/>ages"] --> B["Checkpoint solution:<br/>periodically, flush ALL dirty<br/>(modified but not-yet-persisted)<br/>buffer pool pages to their<br/>data files, then write a<br/>CHECKPOINT marker to the WAL"]

    B --> C["Checkpoint marker records:<br/>'as of this point, everything<br/>before this LSN is confirmed<br/>durably applied to data files'"]

    C --> D["Recovery after a crash only<br/>needs to replay WAL records<br/>AFTER the most recent<br/>checkpoint — bounding recovery<br/>time to 'work done since<br/>last checkpoint', not<br/>'work done since database<br/>creation'"]
```

```mermaid
sequenceDiagram
    participant Timer as Checkpoint Trigger<br/>(time or WAL-size based)
    participant CkptMgr as Checkpoint Manager
    participant Buffer as Buffer Pool
    participant DataFiles as Data Files
    participant WAL as WAL File

    loop Every checkpoint interval
        Timer->>CkptMgr: Trigger checkpoint
        CkptMgr->>Buffer: Flush all dirty pages<br/>to their data files
        Buffer->>DataFiles: Write pages (fsync)
        DataFiles-->>Buffer: Confirmed durable

        CkptMgr->>WAL: Write CHECKPOINT record<br/>(marks this point as<br/>the new recovery starting line)
        CkptMgr->>WAL: Old WAL records before<br/>this checkpoint can now<br/>be safely archived/truncated
    end
```

---

## 7. Crash Recovery — The ARIES-Style Three-Phase Algorithm

```mermaid
flowchart TB
    A["Database restarts<br/>after a crash"] --> B["Phase 1: ANALYSIS"]
    B --> B1["Scan WAL from the last<br/>checkpoint forward, determine:<br/>which transactions were<br/>IN-PROGRESS (not committed)<br/>at crash time, and which<br/>pages might be dirty"]

    B1 --> C["Phase 2: REDO"]
    C --> C1["Replay ALL logged operations<br/>from the checkpoint forward<br/>(both committed AND uncommitted<br/>transactions) — reconstructs<br/>the exact state as it was<br/>at the moment of the crash"]

    C1 --> D["Phase 3: UNDO"]
    D --> D1["For transactions identified<br/>in Phase 1 as NOT committed,<br/>roll back their changes using<br/>the before-images —<br/>restoring the database to a<br/>state as if those transactions<br/>never happened"]

    D1 --> E["Database is now in a<br/>consistent state: all<br/>committed transactions'<br/>effects are present, all<br/>uncommitted transactions'<br/>effects are removed"]
```

**Why REDO-then-UNDO (not just skip uncommitted transactions during redo):** It might seem simpler to just redo only committed transactions — but determining "was this committed" often requires first reconstructing the full state (since a transaction's commit record itself is part of what needs replaying). The REDO-everything-then-UNDO-the-losers approach is what makes recovery a clean, mechanical, provably-correct process rather than requiring complex conditional logic during replay.

---

## 8. Recovery Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Restart as Database Restart Process
    participant WAL as WAL File
    participant Analysis as Analysis Phase
    participant Redo as Redo Phase
    participant Undo as Undo Phase
    participant DataFiles as Data Files

    Restart->>WAL: Locate most recent<br/>CHECKPOINT record

    Restart->>Analysis: Scan forward from checkpoint
    Analysis->>WAL: Read all records to end of log
    Analysis->>Analysis: Build list of transactions<br/>active at crash time (no<br/>COMMIT/ABORT record found)

    Restart->>Redo: Begin REDO phase
    Redo->>WAL: Replay every logged change<br/>from checkpoint to end,<br/>REGARDLESS of commit status
    Redo->>DataFiles: Apply after-images<br/>to reconstruct crash-time state

    Restart->>Undo: Begin UNDO phase
    Undo->>Undo: For each transaction in the<br/>"active at crash" list<br/>(from Analysis phase)
    Undo->>DataFiles: Apply before-images<br/>in REVERSE order,<br/>rolling back their changes

    Restart-->>Restart: Database now consistent —<br/>safe to accept new connections
```

---

## 9. Handling a Crash DURING WAL Writing Itself

```mermaid
flowchart TB
    A["What if the crash happens<br/>WHILE writing a WAL record<br/>(partial/torn write)?"] --> B["Each WAL record includes<br/>a checksum/CRC"]
    B --> C["During recovery's Analysis<br/>phase, if a record's checksum<br/>doesn't match its content,<br/>treat it (and everything<br/>after it in the log) as<br/>NEVER HAVING HAPPENED"]
    C --> D["This is safe because: the<br/>corresponding transaction's<br/>COMMIT record (if any) would<br/>also be incomplete/missing —<br/>so the transaction is correctly<br/>treated as uncommitted and<br/>rolled back/ignored"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((WAL and Recovery HLD))
    WAL Writer
      Sequential append-only log
      Synchronous fsync before ack
      Checksummed records
    Transaction Manager
      Coordinates log-then-modify order
      Tracks transaction state
    Buffer Pool
      In-memory page cache
      Lazy/async flush to disk
    Checkpoint Manager
      Periodic dirty page flush
      Bounds recovery replay window
    Recovery Process
      Analysis: find in-progress txns
      Redo: replay all changes
      Undo: rollback uncommitted
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Write ordering | Log record durable BEFORE data page modification | This ordering is the entire foundation of the durability guarantee — reversing it would allow corrupted, unrecoverable states |
| Commit acknowledgment | Only after COMMIT log record is fsynced | Ensures the client's understanding of "committed" precisely matches what recovery can actually guarantee to reconstruct |
| Recovery algorithm | ARIES-style REDO-then-UNDO | Provides a clean, mechanical, provably-correct recovery process rather than complex conditional replay logic |
| Checkpointing | Periodic dirty-page flush + WAL truncation point | Bounds recovery time to "since last checkpoint" rather than the entire database history |
| Corruption handling | Per-record checksums | Safely and correctly handles the edge case of a crash occurring mid-write to the log itself |
| Data page flush timing | Lazy/asynchronous, decoupled from transaction commit | Allows the hot commit path to only pay the cost of a sequential WAL write, not a full random-access data page write |

---

## 12. Bottlenecks & Scaling Considerations

- **WAL fsync is the critical-path bottleneck** — since every commit must wait for a durable disk write, WAL throughput directly bounds transaction throughput; this is why WAL design prioritizes sequential-only writes (fast on both spinning disks and SSDs) and why many systems offer a "group commit" optimization (batching multiple transactions' log writes into one fsync call) to amortize this cost.
- **Checkpoint frequency tradeoff** — too frequent checkpointing adds I/O overhead and can cause latency spikes (flushing many dirty pages at once); too infrequent lengthens recovery time after a crash — this is tuned based on acceptable recovery time objectives (RTO) versus steady-state performance impact.
- **WAL storage growth and archival** — even with truncation after checkpoints, the WAL requires ongoing storage management; many systems archive old WAL segments to cheaper storage (supporting point-in-time recovery further back than the live truncation point) rather than deleting them outright.
- **Buffer pool size vs checkpoint cost** — a larger buffer pool holds more dirty pages between checkpoints, meaning each checkpoint has more to flush; this interacts with available memory and overall I/O capacity in ways that need holistic tuning, not independent optimization.
- **Group commit contention** — batching multiple transactions' commits into one fsync improves throughput but adds latency to the fastest individual transaction (it must wait for the batch); this is a direct throughput/latency tradeoff exposed as a tunable parameter in most production database engines.
- **Replication built on top of WAL** — many database replication mechanisms (streaming replication to standby replicas) work by shipping WAL records to followers, who replay them — meaning WAL design decisions here directly constrain and enable replication architecture, an important connection to the broader system beyond just crash recovery.
- **Testing recovery correctness is uniquely hard** — since the entire point of this system is correct behavior after ARBITRARY crash points, testing requires deliberately injecting crashes at many different points during transaction processing (including mid-WAL-write) and verifying recovery always produces a correct, consistent result — this needs dedicated fault-injection test infrastructure, not just normal-path testing.
