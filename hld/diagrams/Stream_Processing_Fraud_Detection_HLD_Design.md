# Design a Real-Time Fraud/Anomaly Detection Pipeline Using Stream Processing — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Continuously analyze a high-volume event stream (transactions, logins, activity) to detect anomalous patterns
- Support windowed aggregations (e.g., "transaction count in the last 5 minutes," "spending velocity over a rolling hour")
- Correctly handle events that arrive LATE or OUT OF ORDER relative to when they actually occurred
- Trigger alerts/actions when anomaly patterns are detected, with bounded, predictable latency

### Non-Functional Requirements
- **Correctness under time skew:** Real-world distributed event sources don't deliver events in perfect order — the system must produce correct results despite this
- **Bounded processing latency:** Windows must eventually "close" and produce results, even if some expected late data never arrives — waiting forever isn't acceptable
- **Exactly-once processing semantics (ideally):** Duplicate processing of the same event shouldn't double-count toward anomaly thresholds
- **High throughput:** Must sustain the platform's full event volume continuously, not in periodic batches

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Events/sec (platform-wide) | Hundreds of thousands |
| Typical event lateness | Milliseconds to a few seconds; occasionally minutes (mobile/network issues) |
| Window sizes | Seconds to hours, depending on the specific anomaly pattern being detected |
| Alert latency target | Seconds from pattern occurrence to alert |

---

## 2. The Core Problem — Event Time vs Processing Time

```mermaid
flowchart TB
    A["EVENT TIME: when something<br/>ACTUALLY happened in the<br/>real world (e.g., a user's<br/>phone recorded a transaction<br/>at 10:00:00)"] --> A1["This is the timestamp that<br/>actually matters for correct<br/>business logic — 'how many<br/>transactions occurred between<br/>10:00 and 10:05' must be<br/>based on WHEN THEY HAPPENED"]

    B["PROCESSING TIME: when the<br/>stream processor actually<br/>RECEIVES and processes the<br/>event (e.g., arrives at<br/>10:00:03 due to network<br/>delay, or 10:04:50 due to<br/>a mobile device being<br/>offline and syncing later)"] --> B1["Using processing time for<br/>windowing would produce<br/>INCORRECT results — an event<br/>that actually happened at<br/>9:59:58 but arrives late at<br/>10:00:03 would incorrectly<br/>get counted in the WRONG<br/>window if the system naively<br/>windows by arrival time"]

    C["Correct stream processing<br/>MUST window by EVENT TIME,<br/>while gracefully handling the<br/>reality that events arrive<br/>in PROCESSING TIME order,<br/>which is rarely identical<br/>to event-time order"] --> B1
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph Sources["Event Sources"]
        Mobile["Mobile Clients<br/>(intermittent connectivity)"]
        Servers["Backend Servers"]
    end

    subgraph Ingestion["Ingestion"]
        Kafka["Kafka<br/>(partitioned by entity,<br/>e.g., user_id)"]
    end

    subgraph StreamProcessing["Stream Processing Engine (Flink)"]
        WindowAssigner["Window Assigner<br/>(event-time based)"]
        WatermarkGen["Watermark Generator"]
        AggregationOp["Windowed Aggregation<br/>Operator"]
        AnomalyDetector["Anomaly Detection Logic"]
    end

    subgraph StateBackend["State Management"]
        CheckpointStore[("Checkpoint Store<br/>— processing state snapshots")]
    end

    subgraph Output["Output"]
        AlertSvc["Alert Service"]
        ResultStore[("Aggregation Results Store")]
    end

    Mobile --> Kafka
    Servers --> Kafka
    Kafka --> WindowAssigner
    WindowAssigner --> WatermarkGen
    WatermarkGen --> AggregationOp
    AggregationOp --> AnomalyDetector
    AnomalyDetector --> AlertSvc
    AggregationOp --> ResultStore

    AggregationOp <--> CheckpointStore
```

**Key idea:** The Watermark Generator is the component that makes correct event-time processing possible — it continuously estimates "how far behind could any still-arriving event's timestamp be," giving the Window Assigner a principled basis for deciding when a window has waited "long enough" for late data and can safely close and emit results.

---

## 4. Watermarks — The Core Mechanism for Handling Late Data

```mermaid
flowchart TB
    A["As events stream in, the<br/>processor tracks the MAXIMUM<br/>event-time timestamp seen<br/>SO FAR"] --> B["Watermark = max_event_time_seen<br/>MINUS a configured<br/>'allowed lateness' buffer<br/>(e.g., 30 seconds)"]

    B --> C["Watermark advancing to time T<br/>is a DECLARATION: 'I don't<br/>expect any more events with<br/>event_time earlier than T —<br/>it's now safe to close and<br/>emit results for windows<br/>ending at or before T'"]

    C --> D["If an event LATER arrives<br/>with an event_time EARLIER<br/>than the current watermark<br/>(genuinely late, beyond the<br/>allowed buffer), the system<br/>must have an explicit policy:<br/>drop it, or reopen/update<br/>the already-closed window's<br/>result"]
```

```mermaid
sequenceDiagram
    participant Events as Incoming Events<br/>(event_time varies)
    participant Watermark as Watermark Tracker
    participant Window as Window [10:00-10:05)

    Events->>Watermark: Event A, event_time=10:03
    Watermark->>Watermark: Update watermark = 10:03 - 30s buffer

    Events->>Watermark: Event B, event_time=10:01<br/>(arrived AFTER A, but its<br/>event_time is EARLIER —<br/>out-of-order arrival)
    Watermark->>Window: Still within buffer,<br/>include in window [10:00-10:05)

    Events->>Watermark: Event C, event_time=10:06
    Watermark->>Watermark: Watermark advances to 10:06 - 30s

    Note over Watermark: Watermark now exceeds 10:05<br/>(window's end + buffer<br/>has passed)
    Watermark->>Window: CLOSE window [10:00-10:05),<br/>emit final aggregated result

    Events->>Watermark: Event D, event_time=10:02<br/>(arrives VERY late,<br/>beyond the buffer)
    Watermark->>Watermark: Window already closed —<br/>apply late-data policy<br/>(drop, or side-output for<br/>separate handling)
```

**Why the "allowed lateness" buffer is a deliberate, tunable business tradeoff, not a fixed technical constant:** A larger buffer means the system waits longer before finalizing results (higher latency for alerts), but catches more genuinely late-arriving events correctly; a smaller buffer produces faster results but risks incorrectly excluding legitimately late data from its rightful window. This tradeoff must be tuned based on the actual real-world latency characteristics of the specific event sources (e.g., mobile clients need a more generous buffer than server-to-server events).

---

## 5. Data Model

```mermaid
erDiagram
    RAW_EVENT {
        string event_id PK
        string entity_id "e.g. user_id"
        string event_type
        float amount
        timestamp event_time "when it ACTUALLY happened"
        timestamp ingestion_time "when Kafka received it"
    }
    WINDOW_AGGREGATE {
        string entity_id FK
        timestamp window_start
        timestamp window_end
        int event_count
        float total_amount
        string status "open/closed"
    }
    ANOMALY_ALERT {
        string alert_id PK
        string entity_id FK
        string pattern_matched
        timestamp detected_at
        map supporting_evidence
    }
```

---

## 6. Windowed Aggregation Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant K as Kafka
    participant WindowOp as Window Assigner
    participant State as Windowing State<br/>(per-entity, per-window)
    participant Watermark as Watermark Tracker
    participant Detector as Anomaly Detector

    K->>WindowOp: Event {user_id: X,<br/>amount: $500, event_time: T}

    WindowOp->>WindowOp: Determine which window(s)<br/>this event belongs to<br/>(e.g., 5-minute tumbling<br/>window containing T)

    WindowOp->>State: Add event to that<br/>window's running aggregate<br/>for user_id X

    K->>Watermark: (continuously) update<br/>based on observed event times

    Watermark->>WindowOp: Watermark has passed<br/>window's end + buffer

    WindowOp->>State: Retrieve FINAL aggregate<br/>for the now-closed window
    State-->>WindowOp: {event_count: 47,<br/>total_amount: $12,400}

    WindowOp->>Detector: Evaluate against<br/>anomaly thresholds/patterns
    Detector->>Detector: e.g., total_amount exceeds<br/>3x this user's historical<br/>average for this window size

    alt Anomaly detected
        Detector->>Detector: Trigger alert
    end

    WindowOp->>State: Clean up closed<br/>window's state<br/>(free memory)
```

---

## 7. Exactly-Once Processing via Checkpointing

```mermaid
sequenceDiagram
    participant K as Kafka
    participant StreamProc as Stream Processor
    participant State as Windowing State
    participant Checkpoint as Checkpoint Store

    loop Periodic checkpoint (e.g., every 10s)
        StreamProc->>StreamProc: Pause processing momentarily<br/>(coordinated checkpoint barrier,<br/>similar in spirit to the<br/>ML Training Pipeline design's<br/>synchronized checkpoint step)

        StreamProc->>Checkpoint: Save: current windowing<br/>state (all in-progress<br/>aggregates) + Kafka<br/>consumer offsets

        StreamProc->>StreamProc: Resume processing
    end

    Note over StreamProc: If the processor CRASHES<br/>between checkpoints...

    StreamProc->>Checkpoint: On restart, load LAST<br/>checkpoint's state and offset
    StreamProc->>K: Resume consuming from<br/>the CHECKPOINTED offset<br/>(not from the very<br/>beginning, and not from<br/>wherever it happened to<br/>crash — precisely from the<br/>last CONSISTENT checkpoint)

    Note over StreamProc: This guarantees exactly-once<br/>PROCESSING SEMANTICS — any<br/>events between the checkpoint<br/>and the crash are naturally<br/>REPROCESSED (since the offset<br/>rewinds to the checkpoint),<br/>but because the WINDOWING<br/>STATE was ALSO restored to<br/>that same consistent point,<br/>reprocessing them produces<br/>the CORRECT final aggregate<br/>without double-counting
```

**Why coordinated checkpointing of BOTH state and offsets together is essential:** If only the Kafka offset were checkpointed (without the windowing state), a crash-and-restart would resume consuming from the right position but with EMPTY aggregation state — silently losing all in-progress window contributions. Checkpointing both together atomically is what makes the "rewind and reprocess" recovery strategy produce exactly-once-correct results rather than either data loss or double-counting.

---

## 8. Side Outputs for Late/Dropped Data

```mermaid
flowchart TB
    A["Event arrives AFTER its<br/>window has already closed<br/>and emitted results (beyond<br/>the allowed lateness buffer)"] --> B{"Late Data Policy"}

    B --> C["Silently drop"]
    C --> C1["Simplest, but LOSES<br/>information — inappropriate<br/>for fraud detection where<br/>missing a genuinely late<br/>but real anomalous event<br/>could mean a missed<br/>detection"]

    B --> D["Side output to a<br/>SEPARATE stream"]
    D --> D1["Late events are NOT<br/>silently discarded — they're<br/>routed to a dedicated<br/>'late events' stream for<br/>SEPARATE handling (e.g., a<br/>slower, more lenient batch<br/>reconciliation process that<br/>runs periodically to catch<br/>and correct for genuinely<br/>late fraud signals)"]

    E["This design uses side<br/>outputs — acknowledging that<br/>real-time processing has an<br/>inherent latency/completeness<br/>tradeoff, while ensuring late<br/>data ISN'T simply lost,<br/>just handled through a<br/>DIFFERENT, slower path"] -.-> D1
```

---

## 9. Component Responsibilities Summary

```mermaid
mindmap
  root((Stream Processing Fraud Detection HLD))
    Window Assigner
      Event-time based bucketing
      Not processing-time based
    Watermark Generator
      Tracks maximum observed event time
      Triggers window closure
    Windowing State
      Per-entity running aggregates
      Checkpointed for recovery
    Anomaly Detector
      Evaluates closed window results
      Triggers alerts
    Checkpoint Store
      Coordinated state and offset snapshots
      Enables exactly-once semantics
    Late Data Side Output
      Separate stream for genuinely late events
      Avoids silent data loss
```

---

## 10. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Time semantics | Event-time windowing, not processing-time | Produces business-logic-correct results regardless of network delays or out-of-order arrival, which processing-time windowing cannot guarantee |
| Late data handling | Watermarks with configurable allowed lateness | Provides a principled, tunable mechanism for balancing result latency against completeness, rather than an arbitrary fixed cutoff |
| Genuinely late events | Side output to a separate stream, not silent drop | Fraud detection specifically cannot afford to silently lose signal; late events get a slower but still-processed path |
| Fault tolerance | Coordinated checkpointing of state and offsets together | The only way to achieve exactly-once processing semantics — checkpointing either alone would produce either data loss or double-counting on recovery |
| Alerting trigger point | Only after window closure (watermark-confirmed) | Ensures anomaly detection operates on complete, correct aggregates rather than partial, still-accumulating data |

---

## 11. Bottlenecks & Scaling Considerations

- **Watermark straggler problem** — if even a SINGLE event source has unusually high latency (e.g., one particular mobile carrier with poor connectivity), the GLOBAL watermark (which must account for the slowest-arriving legitimate data) gets held back, delaying window closure for ALL entities, not just the slow source — may require per-key or per-source watermarking strategies for genuinely heterogeneous latency environments.
- **Windowing state memory growth** — maintaining in-progress aggregates for potentially millions of concurrent entities (e.g., every active user) across multiple simultaneous open windows requires substantial memory; state backend choice (in-memory vs disk-spillable) directly impacts both performance and the maximum sustainable entity cardinality.
- **Checkpoint overhead vs recovery time tradeoff** — same fundamental tuning tension as the WAL & Recovery System and ML Training Pipeline designs: frequent checkpoints add processing overhead but bound recovery replay cost; infrequent checkpoints reduce overhead but widen the reprocessing window on failure.
- **Allowed lateness buffer tuning per use case** — different anomaly patterns may need genuinely different lateness tolerances (a "5 rapid transactions" pattern needs fast detection with a tight buffer; a "unusual monthly spending pattern" can tolerate a much more generous buffer) — a single global lateness policy may not fit all detection patterns running on the same platform.
- **Backpressure during traffic spikes** — if downstream anomaly detection logic or alert delivery can't keep pace with a sudden event volume spike, the stream processor needs proper backpressure handling to avoid unbounded memory growth, rather than naively buffering indefinitely.
- **False positive tuning at scale** — as explored in the dedicated Fraud Detection System and Bot Detection designs, real-time anomaly thresholds require continuous calibration against both false-positive and false-negative costs; the windowed aggregation infrastructure described here is the DATA PIPELINE foundation, but the actual anomaly THRESHOLDS and patterns require the same ongoing, adversarially-aware tuning discipline covered in those dedicated designs.
- **Testing time-based logic correctness** — event-time processing with watermarks is notoriously difficult to test correctly, since bugs often only manifest under specific out-of-order arrival patterns; thorough testing requires deliberately constructing test event streams with controlled, adversarial timing/ordering scenarios rather than only testing with naturally-ordered data.
