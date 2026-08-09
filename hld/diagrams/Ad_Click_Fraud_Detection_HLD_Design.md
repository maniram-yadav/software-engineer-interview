# Design an Ad Click Aggregation / Fraud Detection System — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Ingest ad click/impression events at massive volume
- Aggregate clicks/impressions per ad, campaign, advertiser over various time windows
- Detect fraudulent clicks in near-real-time (bots, click farms, competitor click fraud)
- Support both real-time dashboards (approximate, fast) and billing-accurate reports (exact, delayed)
- Prevent double-counting/duplicate click events

### Non-Functional Requirements
- **High write throughput:** Massive click/impression event volume, especially during peak ad-serving hours
- **Near-real-time aggregation:** Dashboards should reflect activity within seconds to a minute
- **Correctness for billing:** Financial reporting (advertisers are charged based on this data) must be exactly accurate, even if real-time dashboards are approximate
- **Fraud detection latency:** Should ideally flag fraud before or shortly after billing, not weeks later
- **Idempotency:** Duplicate event delivery (network retries) must not inflate click counts

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Ad impressions/sec (platform-wide) | ~1M+ |
| Ad clicks/sec | ~10,000-50,000 (much lower than impressions) |
| Estimated fraud rate (industry) | 5-20% of clicks, varies by vertical |
| Aggregation windows | 1-min (real-time), hourly, daily (billing) |
| Event replay/backfill need | Billing corrections require exact reprocessing capability |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    subgraph Sources["Event Sources"]
        AdServer["Ad Serving Infrastructure<br/>(impressions, clicks)"]
    end

    subgraph Ingestion["Ingestion Layer"]
        EventGW["Event Gateway<br/>(validation, dedup key assignment)"]
        Kafka["Kafka<br/>(click/impression event stream)"]
    end

    subgraph RealtimePath["Real-Time Path (Approximate)"]
        StreamAgg["Stream Aggregator<br/>(Flink — windowed counting)"]
        RealtimeStore[("Real-time Aggregate Store<br/>Redis — approximate, fast)")]
    end

    subgraph BatchPath["Batch Path (Exact, Billing-Accurate)"]
        RawEventStore[("Raw Event Store<br/>(durable, immutable log)")]
        BatchJob["Batch Aggregation Job<br/>(Spark — hourly/daily, exact)"]
        BillingStore[("Billing-Accurate Aggregates<br/>(data warehouse)")]
    end

    subgraph FraudPath["Fraud Detection Path"]
        FraudDetector["Fraud Detection Service<br/>(rule-based + ML)"]
        FraudSignals[("Fraud Signal Store<br/>IP reputation, device fingerprints)")]
        FraudDB[("Flagged Click Store")]
    end

    AdServer --> EventGW --> Kafka

    Kafka --> StreamAgg --> RealtimeStore
    Kafka --> RawEventStore --> BatchJob --> BillingStore
    Kafka --> FraudDetector
    FraudDetector --> FraudSignals
    FraudDetector --> FraudDB

    BatchJob -.->|"excludes flagged fraud<br/>from billing totals"| FraudDB
```

**Key idea:** This system deliberately runs **two parallel aggregation paths** — a fast, approximate real-time path for dashboards (stream processing, eventual consistency acceptable), and a slower, exact batch path for billing (where correctness matters far more than speed). Trying to make one pipeline serve both needs would force an unnecessary compromise on one or the other.

---

## 3. Data Model

```mermaid
erDiagram
    CLICK_EVENT {
        string event_id PK "idempotency key"
        string ad_id FK
        string campaign_id FK
        string user_id
        string ip_address
        string device_fingerprint
        timestamp clicked_at
        bool is_fraud "set by fraud detector, async"
    }
    CAMPAIGN_AGGREGATE {
        string campaign_id FK
        timestamp window_start
        string window_type "1min/hourly/daily"
        long click_count
        long impression_count
        long fraud_excluded_count
    }
    FRAUD_SIGNAL {
        string ip_address PK
        float reputation_score
        int recent_click_velocity
        timestamp last_updated
    }
```

---

## 4. Click Event Ingestion — Detailed Sequence

```mermaid
sequenceDiagram
    participant User as User (clicks ad)
    participant AdServer as Ad Server
    participant GW as Event Gateway
    participant K as Kafka

    User->>AdServer: Click on ad
    AdServer->>GW: Send click event<br/>{ad_id, user_id, ip, timestamp,<br/>client-generated event_id}

    GW->>GW: Validate event schema
    GW->>GW: Assign/verify idempotency key<br/>(event_id from client, or<br/>generated deterministically from<br/>ad_id+user_id+timestamp+nonce)

    GW->>K: Publish to click-events topic<br/>(partitioned by campaign_id)

    Note over GW: If client retries due to<br/>network timeout, same event_id<br/>is reused — downstream<br/>deduplication catches it
```

---

## 5. Real-Time Aggregation (Approximate Path)

```mermaid
sequenceDiagram
    participant K as Kafka
    participant Flink as Stream Aggregator
    participant Redis as Real-time Store

    K->>Flink: Consume click events
    Flink->>Flink: Deduplicate within window<br/>(track seen event_ids in<br/>a bounded time-windowed set)
    Flink->>Flink: Windowed count<br/>(tumbling 1-minute windows,<br/>grouped by campaign_id)

    Note over Flink: Watermark-based handling<br/>for slightly late-arriving events<br/>(small grace period, then<br/>window closes and finalizes)

    Flink->>Redis: Increment campaign counter<br/>for current window
    Redis-->>Flink: Ack

    Note over Redis: Dashboard queries read<br/>directly from Redis —<br/>fast, "good enough" accuracy,<br/>NOT used for billing
```

---

## 6. Exact Batch Aggregation (Billing-Accurate Path)

```mermaid
sequenceDiagram
    participant Raw as Raw Event Store
    participant Batch as Batch Aggregation Job
    participant Fraud as Fraud Flagged Store
    participant Billing as Billing Aggregate Store

    Note over Batch: Runs hourly/daily,<br/>well after real-time window closes<br/>(allows time for late data<br/>AND fraud detection to complete)

    Batch->>Raw: Read ALL raw events<br/>for the period (complete, ordered)
    Batch->>Batch: Deduplicate exactly<br/>by event_id (exhaustive,<br/>not windowed/approximate)
    Batch->>Fraud: Fetch fraud-flagged event_ids<br/>for this period
    Fraud-->>Batch: List of fraudulent event_ids

    Batch->>Batch: Exclude fraud-flagged events<br/>from billable count
    Batch->>Billing: Write final, exact aggregate<br/>(this is what advertisers are billed on)

    Note over Billing: Immutable once written —<br/>corrections require an explicit<br/>reprocessing/adjustment entry,<br/>never silent overwrites<br/>(financial audit trail)
```

**Why batch (not stream) for billing:** Billing correctness requires waiting for the complete, ordered dataset — including late-arriving events and completed fraud analysis — before finalizing numbers advertisers will be charged on. Real-time streaming inherently trades some completeness for speed (bounded lateness windows), which is the right tradeoff for dashboards but the wrong one for money.

---

## 7. Fraud Detection Pipeline

```mermaid
flowchart TB
    A["Click event"] --> B["Rule-Based Filters<br/>(fast, cheap, high-confidence)"]
    B --> C["Known bot IP ranges"]
    B --> D["Click velocity anomaly<br/>(1000 clicks/sec from one IP)"]
    B --> E["Impossible geography<br/>(click from IP + immediate<br/>click from opposite hemisphere)"]

    B --> F{"Rule match?"}
    F -- Yes, high confidence --> G["Flag as FRAUD immediately"]
    F -- No / uncertain --> H["ML Model Scoring<br/>(behavioral pattern analysis)"]

    H --> I["Features: click timing patterns,<br/>device fingerprint reuse,<br/>conversion rate for this source,<br/>historical fraud correlation"]
    I --> J{"Fraud probability<br/>score"}
    J -- "High" --> G
    J -- "Low" --> K["Legitimate — proceed to billing"]
    J -- "Borderline" --> L["Flag for manual review<br/>queue"]
```

---

## 8. Fraud Signal Aggregation — Detailed Sequence

```mermaid
sequenceDiagram
    participant K as Kafka
    participant FraudDet as Fraud Detector
    participant SigStore as Fraud Signal Store
    participant FraudDB as Flagged Click Store

    K->>FraudDet: Consume click event
    FraudDet->>SigStore: Lookup IP reputation,<br/>recent click velocity for this IP/device

    alt No prior signal
        SigStore-->>FraudDet: New IP, neutral score
    else Known signal
        SigStore-->>FraudDet: Reputation score,<br/>recent activity pattern
    end

    FraudDet->>FraudDet: Apply rules + ML scoring
    FraudDet->>SigStore: Update velocity counter<br/>for this IP/device (sliding window)

    alt Flagged as fraud
        FraudDet->>FraudDB: Record flagged event_id<br/>+ reason/confidence
    end
```

---

## 9. Deduplication Strategy (Preventing Double-Counting)

```mermaid
flowchart TB
    A["Click event arrives"] --> B{"Deduplication layer"}
    B --> C["Real-time path:<br/>bounded time-window<br/>seen-event_id set<br/>(approximate, memory-bounded)"]
    B --> D["Batch path:<br/>exhaustive dedup against<br/>the complete raw event log<br/>for the billing period<br/>(exact, no time-window limit)"]

    E["Why two different<br/>dedup strategies?"] --> F["Real-time can't hold<br/>an unbounded set of<br/>seen IDs in memory forever —<br/>accepts small error margin"]
    E --> G["Batch has the full,<br/>bounded dataset for<br/>the period available at once —<br/>can dedupe exactly"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Ad Click Aggregation HLD))
    Event Gateway
      Schema validation
      Idempotency key handling
    Stream Aggregator
      Windowed approximate counting
      Powers real-time dashboards
    Raw Event Store
      Immutable, complete log
      Source of truth for billing
    Batch Aggregation Job
      Exact deduplication
      Billing-accurate totals
    Fraud Detector
      Rule-based + ML scoring
      Real-time flagging
    Fraud Signal Store
      IP/device reputation
      Velocity tracking
    Billing Aggregate Store
      Immutable financial record
      Fraud-excluded final counts
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Dual aggregation paths | Real-time (approximate) + Batch (exact) | Dashboards need speed and can tolerate small error; billing needs exact correctness and can tolerate delay — these are fundamentally different requirements |
| Billing timing | Delayed (hourly/daily), not real-time | Must wait for complete data and finished fraud analysis before finalizing amounts advertisers are charged |
| Fraud detection | Layered: fast rules first, ML for uncertain cases | Cheap rule-based filters catch obvious fraud instantly; expensive ML scoring reserved for genuinely ambiguous cases |
| Deduplication | Different strategies per path (windowed vs exhaustive) | Real-time has memory constraints; batch has the complete bounded dataset available, enabling exact dedup |
| Billing record immutability | Append-only with explicit adjustment entries | Financial records require an audit trail — silent overwrites of billing numbers are never acceptable |
| Idempotency | Client-generated event_id, deduplicated downstream | Network retries must never cause double-billing an advertiser for the same click |

---

## 12. Bottlenecks & Scaling Considerations

- **Fraud detection latency vs billing deadline** — if fraud analysis takes longer than the batch billing window allows, either the billing job must wait longer (delaying advertiser reports) or accept a small window where late-detected fraud requires a billing adjustment/refund after the fact.
- **Real-time dedup memory bounds** — the windowed seen-event-id set in the stream aggregator must be carefully bounded (e.g., Bloom filter or bounded LRU) to avoid unbounded memory growth, accepting a small false-negative dedup rate as the tradeoff.
- **Fraud signal store hot keys** — a coordinated click-farm attack targeting one campaign creates extremely high write velocity to that campaign's fraud signals; needs to handle bursty, adversarial traffic patterns specifically (this is an adversarial system, unlike most others in this list).
- **Adversarial adaptation** — fraud detection is fundamentally an arms race; rule-based detection catches known patterns but sophisticated fraud evolves to evade them — the ML layer and continuous retraining are essential, not optional, components.
- **Raw event storage growth** — storing every raw click/impression event for exact batch reprocessing at billion-scale daily volume requires efficient compressed storage and clear retention policy (how long must raw data be kept for billing dispute resolution?).
- **Cross-checking real-time vs batch numbers** — operationally, the two pipelines can diverge (by design, due to approximate vs exact dedup) — dashboards should clearly communicate that real-time numbers are estimates, and reconciliation processes should flag if divergence exceeds expected bounds (signaling a pipeline bug, not just normal approximation).
- **Billing dispute reprocessing** — advertisers disputing charges require the ability to exactly reprocess a historical period's raw events, which is why the raw event store's durability and retention design is as critical as the live pipeline itself.
