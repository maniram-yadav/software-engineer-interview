# Design an Exactly-Once Stream Processing Pipeline (Kafka/Flink-style) — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Consume events from a source (e.g., Kafka), transform/aggregate them, and write results to a sink (e.g., another Kafka topic, or a database)
- Guarantee each source event's effect is reflected EXACTLY ONCE in the final output — no duplicates, no omissions — even across failures and restarts
- Support stateful processing (aggregations, joins) that must also survive failures without corruption
- Support both Kafka-to-Kafka pipelines and pipelines writing to external systems (databases, APIs)

### Non-Functional Requirements
- **True exactly-once semantics, not just "at-least-once with a hopeful shrug":** This is a precise, well-defined guarantee, not an aspiration
- **Fault tolerance:** Any component (source, processing, sink) can fail and recover without violating the exactly-once guarantee
- **Reasonable throughput despite the strong guarantee:** Exactly-once mechanisms add overhead; the design must minimize this cost
- **Consistency across the entire pipeline:** The guarantee must hold end-to-end, from source consumption through to final sink write — a weak link anywhere breaks the whole guarantee

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Events/sec | Hundreds of thousands |
| Checkpoint interval | Seconds |
| Failure recovery time | Seconds to low minutes |
| Sink types | Kafka (native transactional support) vs external systems (requires additional coordination) |

---

## 2. Why "Exactly-Once" Is Genuinely Hard — Setting Precise Expectations

```mermaid
flowchart TB
    A["AT-MOST-ONCE: process each<br/>event zero or one times —<br/>simplest, but risks SILENT<br/>DATA LOSS on failure<br/>(never acceptable for most<br/>real use cases)"] --> A1["Rarely chosen deliberately"]

    B["AT-LEAST-ONCE: process each<br/>event one or MORE times —<br/>guarantees no data loss, but<br/>risks DUPLICATE processing<br/>on failure/retry"] --> B1["The DEFAULT, easier-to-achieve<br/>guarantee — this is what<br/>most systems provide<br/>WITHOUT special engineering<br/>effort (e.g., the CDC Pipeline<br/>and general Message Queue<br/>designs default to this)"]

    C["EXACTLY-ONCE: process each<br/>event effectively ONE time —<br/>no loss, no duplication —<br/>requires SPECIFIC, deliberate<br/>engineering: either true<br/>end-to-end transactional<br/>coordination, OR at-least-once<br/>delivery COMBINED with<br/>IDEMPOTENT processing that<br/>makes duplicates harmless"] --> C1["This design covers BOTH<br/>legitimate approaches to<br/>achieving this guarantee"]
```

**Why this framing matters upfront:** "Exactly-once" is often loosely used to mean "we tried hard to avoid duplicates" — but the RIGOROUS engineering definition requires either genuine distributed transactions spanning the entire pipeline, or the mathematically equivalent combination of guaranteed-at-least-once delivery plus provably idempotent processing. This design is explicit about which mechanism provides the guarantee at each stage, rather than treating it as a vague aspiration.

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph Source["Source"]
        KafkaSource[("Kafka Source Topic<br/>— committed offsets track<br/>consumption progress")]
    end

    subgraph Processing["Stream Processing Engine"]
        Operators["Processing Operators<br/>(map, filter, aggregate)"]
        StateBackend[("Operator State<br/>— checkpointed")]
        CheckpointCoordinator["Checkpoint Coordinator"]
    end

    subgraph Sink["Sink"]
        TransactionalProducer["Transactional Kafka Producer"]
        KafkaSink[("Kafka Sink Topic")]
    end

    KafkaSource --> Operators
    Operators <--> StateBackend
    CheckpointCoordinator --> Operators
    CheckpointCoordinator --> StateBackend
    Operators --> TransactionalProducer
    TransactionalProducer --> KafkaSink

    CheckpointCoordinator -.->|"coordinates atomic<br/>commit of: consumer offset +<br/>operator state + sink writes"| TransactionalProducer
```

**Key idea:** True end-to-end exactly-once requires the CHECKPOINT COORDINATOR to atomically tie together THREE things at each checkpoint: (1) how far the source has been consumed, (2) the internal processing state at that point, and (3) what's been committed to the sink — this three-way atomicity, spanning across otherwise-independent systems, is the core technical challenge this entire design solves.

---

## 4. Checkpoint-Based Exactly-Once (Two-Phase Commit Across the Pipeline)

```mermaid
sequenceDiagram
    participant Checkpoint as Checkpoint Coordinator
    participant Source as Kafka Source
    participant Operators as Processing Operators
    participant Producer as Transactional Producer
    participant Sink as Kafka Sink

    Note over Checkpoint: Trigger checkpoint N

    Checkpoint->>Source: Inject checkpoint barrier<br/>into the event stream<br/>(a special marker, flows<br/>WITH the data)

    Source->>Operators: Barrier flows through<br/>the processing pipeline

    Operators->>Operators: Upon receiving barrier:<br/>snapshot current internal<br/>state (aggregation values,<br/>etc.)

    Operators->>Producer: Barrier reaches the sink

    Producer->>Sink: PRE-COMMIT phase:<br/>begin Kafka transaction,<br/>write all buffered output<br/>records (but NOT yet<br/>visible to consumers)

    Checkpoint->>Checkpoint: Once ALL components confirm<br/>successful barrier processing<br/>+ state snapshot, the<br/>checkpoint itself is<br/>considered COMPLETE

    Checkpoint->>Producer: COMMIT phase:<br/>finalize the Kafka<br/>transaction — NOW the<br/>output records become<br/>visible to downstream<br/>consumers

    Checkpoint->>Source: Commit consumed offsets<br/>up to this checkpoint
```

**Why this is fundamentally a two-phase commit pattern (connecting to the Distributed Transaction design):** The sink writes happen in a PRE-COMMIT (buffered, not yet visible) state until the ENTIRE checkpoint across the whole pipeline succeeds — only then does the coordinator trigger the final COMMIT, making the output visible. This mirrors the prepare/commit pattern from the Distributed Transaction Saga design, applied specifically to make an entire streaming pipeline's state transition atomic.

---

## 5. Data Model

```mermaid
erDiagram
    CHECKPOINT {
        long checkpoint_id PK
        string status "in_progress/completed/failed"
        map source_offsets "per-partition consumed positions"
        timestamp initiated_at
        timestamp completed_at
    }
    OPERATOR_STATE_SNAPSHOT {
        long checkpoint_id FK
        string operator_id
        bytes state_data
    }
    SINK_TRANSACTION {
        long checkpoint_id FK
        string transaction_id
        string status "pre_committed/committed/aborted"
    }
```

---

## 6. Failure Recovery Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Processor as Stream Processor<br/>(crashes mid-processing)
    participant Restart as Restart Process
    participant CheckpointStore as Checkpoint Store
    participant Source as Kafka Source
    participant Sink as Kafka Sink (transactional)

    Note over Processor: Processor crashes AFTER<br/>checkpoint N completed,<br/>but BEFORE checkpoint N+1

    Restart->>CheckpointStore: Load LATEST completed<br/>checkpoint (N)
    CheckpointStore-->>Restart: State snapshot +<br/>source offsets as of<br/>checkpoint N

    Restart->>Restart: Restore operator state<br/>to checkpoint N's snapshot

    Restart->>Source: Resume consuming from<br/>checkpoint N's committed<br/>offsets<br/>(events processed AFTER<br/>checkpoint N but before<br/>the crash will be<br/>RE-CONSUMED)

    Note over Sink: Any sink transaction that<br/>was PRE-COMMITTED but never<br/>reached final COMMIT<br/>(because checkpoint N+1<br/>never completed) is<br/>explicitly ABORTED —<br/>Kafka's transactional<br/>producer guarantees these<br/>uncommitted writes are<br/>NEVER visible to consumers

    Restart->>Restart: Reprocess events from<br/>checkpoint N's offset forward —<br/>since state was ALSO<br/>restored to that exact<br/>point, this reprocessing<br/>produces the CORRECT result,<br/>with the aborted partial<br/>output from the failed<br/>attempt never having been<br/>visible in the first place
```

**Why this achieves TRUE exactly-once, not just at-least-once:** The critical guarantee is that reprocessed events NEVER produce VISIBLE duplicate output — because any partial sink writes from the failed attempt were still in an uncommitted transactional state (never visible to consumers) at the moment of failure, and get explicitly aborted on recovery. The reprocessing that follows produces the ONE, correct, visible output — downstream consumers never see evidence that reprocessing happened at all.

---

## 7. The Alternative Approach — Idempotent Sinks (When Transactions Aren't Available)

```mermaid
flowchart TB
    A["Not every sink supports<br/>transactional writes the way<br/>Kafka does (e.g., writing to<br/>a REST API, or a database<br/>without appropriate<br/>transactional integration)"] --> B["Alternative: achieve the<br/>SAME exactly-once OUTCOME<br/>via AT-LEAST-ONCE delivery<br/>PLUS IDEMPOTENT sink writes —<br/>mathematically equivalent<br/>result, different mechanism"]

    B --> C["Each output record includes<br/>a DETERMINISTIC, stable<br/>identifier (e.g., derived<br/>from the source event's own<br/>ID, or the checkpoint+offset<br/>position)"]

    C --> D["Sink write logic: 'INSERT<br/>this record, but if a record<br/>with this EXACT identifier<br/>already exists, treat it as<br/>a no-op' — same core<br/>idempotency principle as the<br/>dedicated Idempotent API<br/>Requests design"]

    E["This means even if a<br/>reprocessing-after-failure<br/>scenario causes the SAME<br/>logical output to be WRITTEN<br/>TWICE to the sink, the<br/>second write is a safe,<br/>harmless no-op — achieving<br/>the exactly-once OUTCOME<br/>without requiring the sink<br/>to support true distributed<br/>transactions"] -.-> D
```

---

## 8. Stateful Aggregation Correctness Across Failures

```mermaid
sequenceDiagram
    participant Events as Event Stream
    participant Aggregator as Stateful Aggregation<br/>Operator
    participant State as Checkpointed State

    Events->>Aggregator: Event 1: increment counter
    Aggregator->>State: counter = 1

    Events->>Aggregator: Event 2: increment counter
    Aggregator->>State: counter = 2

    Note over Aggregator: CHECKPOINT occurs here —<br/>state (counter=2) is<br/>durably snapshotted

    Events->>Aggregator: Event 3: increment counter
    Aggregator->>State: counter = 3 (in-memory,<br/>NOT yet checkpointed)

    Note over Aggregator: CRASH before next checkpoint

    Note over Aggregator: RESTART: restore state<br/>from LAST checkpoint<br/>(counter = 2)

    Events->>Aggregator: Event 3 is RE-DELIVERED<br/>(source offset also<br/>rewound to the checkpoint)
    Aggregator->>State: counter = 3<br/>(correctly recomputed,<br/>NOT double-counted as<br/>counter=4, because the<br/>PREVIOUS in-memory<br/>increment from before the<br/>crash was never<br/>checkpointed/committed)
```

**Why this doesn't result in double-counting:** The key insight is that the in-memory state change from processing Event 3 the FIRST time (before the crash) was never durably checkpointed — from the system's committed, durable perspective, Event 3 was NEVER successfully processed at all. Reprocessing it after recovery is therefore the FIRST successful processing of that event from the system's consistent viewpoint, not a duplicate.

---

## 9. Handling Non-Deterministic Processing Logic

```mermaid
flowchart TB
    A["Processing logic that<br/>includes NON-DETERMINISTIC<br/>elements (e.g., 'attach the<br/>current wall-clock timestamp,'<br/>or 'call an external random<br/>ID generator')"] --> B["Problem: if this operation<br/>needs to be REPROCESSED<br/>after a failure, a<br/>non-deterministic result<br/>would PRODUCE A DIFFERENT<br/>OUTPUT the second time —<br/>breaking the exactly-once<br/>guarantee even with perfect<br/>checkpointing"]

    B --> C["Mitigation: any<br/>non-deterministic values<br/>needed by the processing<br/>logic must THEMSELVES be<br/>captured as part of the<br/>checkpointed state (e.g.,<br/>checkpoint the GENERATED<br/>timestamp/ID alongside the<br/>processing state, so<br/>REPROCESSING reuses the<br/>SAME captured value rather<br/>than generating a NEW,<br/>different one)"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Exactly-Once Stream Processing HLD))
    Checkpoint Coordinator
      Orchestrates barrier injection
      Atomically ties source, state, sink
    Checkpoint Barrier
      Flows through the pipeline with data
      Triggers state snapshotting
    Transactional Producer
      Pre-commit then commit pattern
      Guarantees no visible partial output
    State Backend
      Checkpointed operator state
      Restored consistently on recovery
    Idempotent Sink (alternative)
      Deterministic record identifiers
      Achieves exactly-once outcome without transactions
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Core mechanism | Checkpoint barriers + two-phase sink commit | Atomically ties together source offset, processing state, and sink output — the only way to achieve TRUE exactly-once across an entire heterogeneous pipeline |
| Sink strategy (transactional systems) | Pre-commit/commit pattern (e.g., Kafka transactions) | Ensures partial output from a failed attempt is never visible to downstream consumers |
| Sink strategy (non-transactional systems) | At-least-once delivery + idempotent writes | Achieves the mathematically equivalent exactly-once OUTCOME when true distributed transactions aren't available at the sink |
| Recovery approach | Restore state to last checkpoint, rewind and reprocess | Reprocessing from a consistent checkpoint produces correct results without double-counting, since pre-crash uncommitted progress was never durably recorded |
| Non-determinism handling | Checkpoint any non-deterministic values alongside state | Prevents reprocessing from producing genuinely different output than the original (failed) attempt would have |

---

## 12. Bottlenecks & Scaling Considerations

- **Checkpoint barrier alignment overhead** — coordinating a consistent snapshot across many parallel processing operators requires the barrier to propagate through and align across ALL of them before the checkpoint can complete; operators with uneven processing speeds can create alignment delays, directly impacting checkpoint frequency and thus recovery granularity.
- **Checkpoint interval tuning** — the same fundamental tradeoff seen throughout this document series (WAL, ML Training, general Stream Processing): frequent checkpoints add overhead and reduce throughput, infrequent checkpoints widen the reprocessing window on failure — tuned based on actual throughput requirements and acceptable recovery time.
- **Non-transactional sink limitations** — the idempotent-write alternative (Section 7) requires the sink system to actually SUPPORT efficient deduplication by a stable identifier; not every external system (e.g., certain third-party APIs) provides this capability, which can genuinely limit which sinks can participate in a true exactly-once pipeline versus only achieving at-least-once.
- **State size and checkpoint I/O cost** — pipelines with very large accumulated state (e.g., aggregating over long time windows across millions of keys) face substantial I/O cost for each checkpoint's state snapshot; incremental checkpointing (saving only what changed since the last checkpoint, rather than the full state each time) is an important optimization at scale.
- **Cross-system coordination complexity** — true end-to-end exactly-once spanning genuinely independent systems (a Kafka source, a stateful processor, AND an external database sink) requires careful, deliberate engineering at each integration point — this is meaningfully harder than a pure Kafka-to-Kafka pipeline where the same transactional infrastructure spans the whole flow.
- **Testing exactly-once guarantees** — verifying this guarantee actually holds requires deliberately injecting failures at EVERY possible point in the pipeline (mid-checkpoint, mid-sink-write, mid-state-restore) and confirming no duplicates or losses occur — this demands the same rigorous fault-injection testing discipline emphasized in the WAL & Recovery System and ML Training Pipeline designs, since these correctness properties are largely invisible during normal, failure-free operation.
