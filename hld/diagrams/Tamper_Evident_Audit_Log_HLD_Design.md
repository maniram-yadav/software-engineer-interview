# Design a Tamper-Evident Audit Logging System — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Record every security/compliance-relevant action (logins, permission changes, data access, admin actions) as an immutable audit entry
- Allow authorized parties to verify the log's integrity — detect if ANY entry has been modified, deleted, or reordered after the fact
- Support efficient querying (by user, by time range, by action type) despite the append-only, tamper-evidence constraints
- Support long-term retention for compliance (often years)

### Non-Functional Requirements
- **Tamper-evidence (the defining property):** Even someone with elevated database access (e.g., a compromised admin account, or a malicious insider) must NOT be able to silently alter history without detection
- **Non-repudiation:** An action recorded in the log should be attributable and provably genuine — not deniable by the actor who performed it
- **Availability:** Audit logging shouldn't become a bottleneck or single point of failure for the systems generating the events
- **Durability:** Audit logs are often legally required evidence — loss is unacceptable

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Audit events/sec (platform-wide) | Thousands to tens of thousands |
| Retention period | Often 1-7 years, depending on regulatory context |
| Verification frequency | Periodic (e.g., daily automated integrity checks) + on-demand (incident investigation) |
| Query patterns | By user, by time range, by resource, by action type |

---

## 2. The Core Mechanism — Hash Chaining

```mermaid
flowchart TB
    A["Naive audit log: just a<br/>table of {timestamp, user,<br/>action} rows"] --> A1["Problem: anyone with<br/>database write access<br/>(e.g., a DBA, or an attacker<br/>who compromised database<br/>credentials) can simply<br/>UPDATE or DELETE a row —<br/>and there's NOTHING in the<br/>data itself that reveals<br/>this happened"]

    B["Hash-chained log: each<br/>entry includes a cryptographic<br/>hash of the PREVIOUS entry's<br/>content, in addition to its<br/>own content"] --> C["entry_N.hash = SHA256(<br/>entry_N.content +<br/>entry_(N-1).hash)"]

    C --> D["This creates a chain where<br/>modifying ANY historical<br/>entry changes its hash,<br/>which INVALIDATES every<br/>subsequent entry's hash in<br/>the chain — tampering with<br/>entry 500 out of 10,000<br/>breaks the verifiable chain<br/>for entries 500 through<br/>10,000, making the tampering<br/>mathematically detectable"]
```

**Why this works even against someone with database access:** The key insight is that tampering with historical data requires NOT JUST modifying the target entry, but also recalculating and updating every single subsequent entry's hash to maintain a valid-looking chain — and even then, this recalculated chain can be checked against independently-stored checkpoint hashes (covered below) that the tamperer doesn't have write access to modify.

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph Sources["Event Sources"]
        AuthSvc["Auth Service<br/>(logins, permission changes)"]
        DataSvc["Data Access Services"]
        AdminPanel["Admin Panel<br/>(privileged actions)"]
    end

    subgraph Ingestion["Ingestion Layer"]
        AuditAPI["Audit Log API"]
        HashChainer["Hash Chain Calculator"]
    end

    subgraph Storage["Storage Layer"]
        AuditLog[("Append-Only Audit Log<br/>(write-restricted, no UPDATE/DELETE<br/>privileges granted to services)")]
        CheckpointStore[("Independent Checkpoint Store<br/>(periodic chain-head hashes,<br/>stored SEPARATELY with<br/>different access controls)")]
    end

    subgraph Verification["Verification & Query"]
        VerifySvc["Integrity Verification Service"]
        QueryAPI["Audit Query API<br/>(read-only)"]
    end

    AuthSvc --> AuditAPI
    DataSvc --> AuditAPI
    AdminPanel --> AuditAPI

    AuditAPI --> HashChainer
    HashChainer --> AuditLog
    HashChainer -.->|"periodic checkpoint<br/>(e.g., hourly)"| CheckpointStore

    VerifySvc --> AuditLog
    VerifySvc --> CheckpointStore
    QueryAPI --> AuditLog
```

**Key idea:** The Checkpoint Store, holding periodic snapshots of the chain's current hash, is deliberately kept SEPARATE from the main audit log with DIFFERENT access controls (ideally write-once, or held by a different administrative domain entirely) — this is what prevents an attacker with full access to the main log from ALSO rewriting checkpoints to hide their tracks, since compromising both systems simultaneously is a meaningfully higher bar than compromising just one.

---

## 4. Data Model

```mermaid
erDiagram
    AUDIT_ENTRY {
        long sequence_number PK "strictly monotonic"
        string entry_id
        string actor_id "who performed the action"
        string action_type
        string resource_id
        map details
        timestamp occurred_at
        string prev_entry_hash "hash of the previous entry"
        string entry_hash "hash of THIS entry's content + prev_hash"
    }
    CHECKPOINT {
        long checkpoint_id PK
        long covers_up_to_sequence
        string chain_head_hash "snapshot of the hash chain at this point"
        timestamp created_at
        string digital_signature "optional — signed by a<br/>separate signing authority"
    }
```

---

## 5. Writing a New Audit Entry — Detailed Sequence

```mermaid
sequenceDiagram
    participant Source as Event Source<br/>(e.g., Auth Service)
    participant API as Audit Log API
    participant Chainer as Hash Chain Calculator
    participant Log as Append-Only Audit Log

    Source->>API: Record event<br/>{actor_id, action_type,<br/>resource_id, details}

    API->>Log: Fetch the CURRENT last entry's hash<br/>(prev_entry_hash for the new entry)
    Log-->>API: Last entry's hash: "a3f9..."

    API->>Chainer: Compute new entry's hash:<br/>SHA256(new_content + "a3f9...")
    Chainer-->>API: New hash: "b7e2..."

    API->>Log: APPEND new entry<br/>{content, prev_entry_hash="a3f9...",<br/>entry_hash="b7e2..."}<br/>(INSERT ONLY — no UPDATE/DELETE<br/>permission granted to this API)

    Log-->>API: Confirmed appended
    API-->>Source: Acknowledged
```

**Why "insert-only" database permissions matter as a complementary control:** The hash chain provides DETECTION of tampering, but restricting the audit log table's database permissions to INSERT-only (revoking UPDATE and DELETE entirely, even for the application's own service account) adds a PREVENTION layer — making unauthorized modification structurally harder to even attempt, not just detectable after the fact.

---

## 6. Periodic Checkpointing — Detailed Sequence

```mermaid
sequenceDiagram
    participant Timer as Checkpoint Scheduler
    participant Log as Audit Log
    participant CheckpointStore as Independent Checkpoint Store
    participant SigningAuthority as External Signing Service<br/>(optional, stronger guarantee)

    loop Every checkpoint interval (e.g., hourly)
        Timer->>Log: Get current chain head<br/>(latest entry's hash,<br/>latest sequence_number)
        Log-->>Timer: {sequence: 48291,<br/>hash: "f92c..."}

        Timer->>SigningAuthority: Request digital signature<br/>over this checkpoint<br/>(optional — adds<br/>non-repudiation from an<br/>INDEPENDENT authority)
        SigningAuthority-->>Timer: Signed checkpoint

        Timer->>CheckpointStore: Store checkpoint<br/>{sequence: 48291,<br/>hash: "f92c...", signature}<br/>(this store has DIFFERENT<br/>access controls than the<br/>main audit log)
    end
```

**Why external signing adds a meaningfully stronger guarantee:** Even if an attacker somehow compromises BOTH the audit log's database AND the checkpoint store, a checkpoint independently signed by a genuinely separate system (potentially even an external, third-party timestamping/notary service) cannot be forged without ALSO compromising that separate signing authority — this is the same "distribute trust across independent parties" principle as the Secrets Management design's Shamir's Secret Sharing, applied to log integrity instead of key protection.

---

## 7. Integrity Verification Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Verifier as Verification Service<br/>(scheduled or on-demand)
    participant Log as Audit Log
    participant CheckpointStore as Checkpoint Store

    Verifier->>CheckpointStore: Get last known-good checkpoint<br/>{sequence: 40000, hash: "d81a..."}
    CheckpointStore-->>Verifier: Checkpoint data

    Verifier->>Log: Fetch all entries from<br/>sequence 40001 to current

    loop For each entry in sequence
        Verifier->>Verifier: Recompute expected hash:<br/>SHA256(entry.content +<br/>previous_computed_hash)
        Verifier->>Verifier: Compare against the<br/>entry's STORED hash
        alt Hashes match
            Verifier->>Verifier: Continue to next entry
        else Hashes DON'T match
            Verifier->>Verifier: TAMPERING DETECTED at<br/>this specific sequence number —<br/>halt and ALERT immediately
        end
    end

    Verifier->>Verifier: Verify final computed hash<br/>matches the LATEST checkpoint<br/>(if one exists for this range)

    Verifier-->>Verifier: Report: chain VERIFIED INTACT<br/>from sequence 40001 to current<br/>(or report exact point of failure)
```

**Why verification starts from a checkpoint, not from the very beginning:** Just as the WAL & Recovery System design uses checkpoints to bound RECOVERY replay cost, this design uses checkpoints to bound VERIFICATION cost — re-verifying the ENTIRE historical chain from day one on every check would become increasingly expensive as the log grows; verification only needs to re-walk the chain since the last TRUSTED checkpoint.

---

## 8. Detecting Different Types of Tampering

```mermaid
flowchart TB
    A["Tampering Detection Scenarios"] --> B["Entry MODIFIED<br/>(content changed after<br/>the fact)"]
    A --> C["Entry DELETED<br/>(removed from the sequence)"]
    A --> D["Entry INSERTED<br/>(fake entry added,<br/>e.g., backdating a<br/>fabricated 'approval')"]
    A --> E["Entries REORDERED"]

    B --> B1["Detected: recomputed hash<br/>for the modified entry<br/>won't match its stored hash"]
    C --> C1["Detected: sequence_number<br/>GAP — e.g., entry 501<br/>is simply missing between<br/>500 and 502"]
    D --> D1["Detected: inserting a fake<br/>entry requires either breaking<br/>sequence numbering (detectable<br/>gap/duplicate) OR recalculating<br/>the ENTIRE subsequent chain<br/>(detectable via checkpoint<br/>mismatch)"]
    E --> E1["Detected: each entry's<br/>prev_entry_hash explicitly<br/>encodes the expected ORDER —<br/>reordering breaks this<br/>reference chain"]
```

---

## 9. Handling Legitimate Log Archival (Not Tampering)

```mermaid
flowchart TB
    A["Old audit entries need to<br/>move to cheaper, long-term<br/>storage (similar to the tiered<br/>retention pattern in the Log<br/>Aggregation design)"] --> B["This is LEGITIMATE data<br/>lifecycle management, not<br/>tampering — but the system<br/>must distinguish it from<br/>unauthorized deletion"]

    B --> C["Archival process: move<br/>entries to cold storage,<br/>but PRESERVE their hashes<br/>and the checkpoint chain<br/>intact — verification can<br/>still walk the FULL chain<br/>by reading from cold storage<br/>when needed, just more slowly"]

    D["Critical distinction: archival<br/>RELOCATES data while preserving<br/>verifiability; tampering<br/>REMOVES or ALTERS data —<br/>the hash chain must survive<br/>the archival process intact<br/>for this distinction to hold"] --> C
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Tamper-Evident Audit Log HLD))
    Audit Log API
      Ingestion entry point
      Never allows UPDATE/DELETE
    Hash Chain Calculator
      Computes chained hashes
      Links each entry to predecessor
    Append-Only Audit Log
      Insert-only database permissions
      Primary storage
    Checkpoint Store
      Separate access controls
      Periodic chain-head snapshots
    External Signing Authority
      Independent trust anchor
      Optional, stronger non-repudiation
    Verification Service
      Recomputes and compares hashes
      Bounded by checkpoint intervals
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Core integrity mechanism | Hash chaining (each entry references previous entry's hash) | Makes any historical modification mathematically detectable — altering one entry invalidates the chain for everything after it |
| Database permissions | Insert-only, no UPDATE/DELETE granted | Adds a prevention layer alongside detection — makes unauthorized modification structurally harder to attempt, not just detectable |
| Checkpoint storage | Separate system, different access controls | Prevents a single compromised system/credential from being able to both tamper with the log AND cover its tracks by also rewriting checkpoints |
| External signing (optional) | Independent signing authority for checkpoints | Provides the strongest possible non-repudiation guarantee, requiring compromise of a genuinely separate system to forge |
| Verification scope | Incremental, from last trusted checkpoint | Bounds verification cost as the log grows, same principle as checkpoint-bounded recovery in the WAL design |
| Archival handling | Preserve hash chain integrity through cold storage migration | Distinguishes legitimate data lifecycle management from actual tampering — both must remain verifiable, unlike destructive deletion |

---

## 12. Bottlenecks & Scaling Considerations

- **Sequential write dependency** — because each entry's hash depends on the PREVIOUS entry's hash, writes are inherently sequential/serialized at the hash-chaining step, which can become a throughput bottleneck at very high audit event volume; may require sharding into multiple independent chains (e.g., per-service or per-region chains) with periodic cross-chain checkpointing, accepting that verification then needs to account for multiple parallel chains.
- **Verification cost growth** — even with checkpoint-bounded incremental verification, extremely high-volume systems accumulate a large number of entries between checkpoints; more frequent checkpointing reduces per-verification cost but increases checkpoint storage/signing overhead — a tunable tradeoff based on actual event volume.
- **Query performance vs append-only constraint** — supporting efficient queries (by user, time range, action type) on an append-only log typically requires secondary indexes (same considerations as the Secondary Index System design) that must ALSO be carefully designed not to become an alternate, unprotected avenue for inferring or reconstructing tampered data.
- **Long-term cryptographic hash algorithm obsolescence** — hash algorithms considered secure today (e.g., SHA-256) may eventually be weakened by future cryptographic advances; systems with very long retention requirements (years) should have a plan for potentially re-hashing or adding stronger algorithm layers over time, without breaking historical verifiability.
- **Insider threat with checkpoint-store access** — if a single administrator has legitimate access to BOTH the main audit log AND the checkpoint store (e.g., a small organization without strict separation of duties), the "independent access control" protection is weakened; this is as much an organizational/process control question as a technical one.
- **Balancing detection speed against verification cost** — running full integrity verification continuously (after every single write) would be prohibitively expensive; scheduled periodic verification (e.g., daily) means tampering could theoretically go undetected for up to that interval — this detection latency is a deliberate, documented tradeoff, not an oversight.
