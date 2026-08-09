# Design a Feature Store for ML Models — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Store and serve features (computed inputs to ML models) for both model TRAINING (offline, batch) and model SERVING (online, real-time)
- Guarantee that a model sees the EXACT SAME feature values during training as it will during real-time serving for equivalent inputs
- Support feature versioning and discovery (data scientists finding and reusing existing features rather than reinventing them)
- Support both batch-computed features (e.g., daily aggregates) and streaming/real-time features (e.g., "clicks in the last 5 minutes")

### Non-Functional Requirements
- **Point-in-time correctness (the defining challenge):** Training data must reflect EXACTLY what would have been known at the historical moment being trained on — not future information "leaking" backward
- **Low-latency online serving:** Real-time model inference needs feature lookups in single-digit milliseconds
- **High-throughput offline access:** Training jobs need to pull large volumes of historical feature data efficiently for batch processing
- **Consistency between online and offline paths:** The same logical feature must be computed identically whether accessed via the batch or streaming path

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Distinct features registered | Hundreds to thousands, across many teams/models |
| Online feature lookups/sec | Very high — one per model inference request |
| Offline training data volume | Terabytes, spanning months/years of historical feature values |
| Feature freshness (streaming features) | Seconds |
| Feature freshness (batch features) | Hours to a day |

---

## 2. The Core Problem This System Solves — Training/Serving Skew

```mermaid
flowchart TB
    A["Without a feature store:<br/>each team writes its OWN<br/>feature computation logic —<br/>once for offline TRAINING<br/>(e.g., a Spark job scanning<br/>historical data), and<br/>SEPARATELY for online SERVING<br/>(e.g., a real-time API call)"] --> B["These two independent<br/>implementations of the<br/>'same' feature can subtly<br/>DIVERGE — different edge<br/>case handling, different<br/>time-window boundaries,<br/>different null-handling"]

    B --> C["Result: the model performs<br/>well in offline evaluation<br/>(trained and tested on<br/>consistent, batch-computed<br/>features) but UNDERPERFORMS<br/>in production, because the<br/>REAL-TIME features it actually<br/>receives don't quite match<br/>what it learned from —<br/>this is TRAINING/SERVING SKEW,<br/>one of the most common and<br/>hardest-to-debug problems<br/>in production ML"]

    D["A feature store's core value<br/>proposition: compute each<br/>feature's logic in EXACTLY<br/>ONE place, serve it through<br/>BOTH the training and serving<br/>paths, eliminating the<br/>divergence risk structurally,<br/>not through process discipline<br/>alone"] --> C
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph Sources["Data Sources"]
        OLTP[("Operational DBs")]
        EventStream["Event Streams<br/>(clicks, transactions)"]
    end

    subgraph Definition["Feature Definition Layer"]
        FeatureRegistry[("Feature Registry<br/>— definitions, versions,<br/>ownership, documentation")]
    end

    subgraph Computation["Feature Computation"]
        BatchPipeline["Batch Pipeline<br/>(Spark — periodic)"]
        StreamPipeline["Stream Pipeline<br/>(Flink — continuous)"]
    end

    subgraph Storage["Dual Storage"]
        OfflineStore[("Offline Store<br/>(data warehouse — full<br/>history, point-in-time queryable)")]
        OnlineStore[("Online Store<br/>(low-latency KV — current<br/>values only)")]
    end

    subgraph Consumers["Consumers"]
        TrainingJob["Model Training Pipeline"]
        ServingSvc["Real-Time Model Serving"]
    end

    OLTP --> BatchPipeline
    EventStream --> StreamPipeline

    BatchPipeline --> OfflineStore
    BatchPipeline --> OnlineStore
    StreamPipeline --> OnlineStore
    StreamPipeline --> OfflineStore

    FeatureRegistry -.->|"defines computation logic<br/>used by BOTH pipelines"| BatchPipeline
    FeatureRegistry -.-> StreamPipeline

    OfflineStore --> TrainingJob
    OnlineStore --> ServingSvc
```

**Key idea:** The Feature Registry holds the SINGLE, authoritative definition of how each feature is computed — both the batch and streaming pipelines implement this SAME definition (ideally sharing actual code, not just a specification), and write their results into BOTH stores. This structural sharing is what prevents the divergence at the root of training/serving skew, rather than relying on two independent teams/pipelines staying manually synchronized.

---

## 4. Data Model

```mermaid
erDiagram
    FEATURE_DEFINITION {
        string feature_name PK
        string entity_type "e.g. user, product"
        string computation_logic
        string data_type
        string owner_team
        int version
    }
    OFFLINE_FEATURE_VALUE {
        string feature_name FK
        string entity_id
        string value
        timestamp event_timestamp "WHEN this value became true —<br/>critical for point-in-time correctness"
    }
    ONLINE_FEATURE_VALUE {
        string feature_name FK
        string entity_id PK
        string current_value "only the LATEST value,<br/>no history"
        timestamp last_updated_at
    }
```

**Why the offline store retains full history with `event_timestamp`, while online only keeps the latest value:** Training needs to reconstruct "what was true AT THE TIME of each historical training example" — requiring the full timestamped history. Serving only ever needs "what's true RIGHT NOW" for a live inference request — no history needed, which is why the online store can stay small, fast, and simple by design.

---

## 5. Point-in-Time Correct Training Data Generation — Detailed Sequence

```mermaid
sequenceDiagram
    participant DS as Data Scientist
    participant TrainJob as Training Pipeline
    participant OfflineStore as Offline Feature Store

    DS->>TrainJob: Define training set:<br/>label events (e.g., "user X<br/>churned on 2026-03-15")<br/>+ requested features

    loop For each labeled training example
        TrainJob->>OfflineStore: Get feature values for<br/>user X AS OF 2026-03-15<br/>(NOT the current/latest values —<br/>the values that were TRUE<br/>at that historical moment)

        OfflineStore->>OfflineStore: Query: feature value WHERE<br/>event_timestamp <= 2026-03-15<br/>ORDER BY event_timestamp DESC<br/>LIMIT 1 (most recent value<br/>as of that point in time)

        OfflineStore-->>TrainJob: Point-in-time correct<br/>feature values
    end

    TrainJob->>TrainJob: Assemble complete training<br/>dataset — every row's features<br/>reflect ONLY information that<br/>was genuinely available at<br/>that row's timestamp
```

**Why this "as of" query is the single hardest technical problem in feature store design:** Naively joining "user X's current profile data" against a historical label from months ago would leak FUTURE information into training (e.g., training a churn model using the user's CURRENT status, which obviously already reflects whether they churned) — this is called "label leakage," and it's one of the most common, subtle bugs in ML pipelines. Point-in-time correct joins are specifically engineered to prevent this by always fetching feature values as they existed at the exact historical moment relevant to each training example.

---

## 6. Real-Time Online Feature Serving — Detailed Sequence

```mermaid
sequenceDiagram
    participant App as Application<br/>(requesting a prediction)
    participant ServingSvc as Model Serving Service
    participant OnlineStore as Online Feature Store

    App->>ServingSvc: Predict for user_id=X
    ServingSvc->>OnlineStore: Batch fetch all required<br/>features for user_id=X<br/>(single low-latency call,<br/>NOT one call per feature)

    OnlineStore-->>ServingSvc: Feature vector<br/>{feature_1: val, feature_2: val, ...}

    ServingSvc->>ServingSvc: Run model inference<br/>using this feature vector
    ServingSvc-->>App: Prediction result
```

**Why batching feature fetches into a single call matters:** If a model requires 50 features, making 50 separate network round-trips to fetch them individually would badly blow the latency budget for real-time inference; the online store is designed to support efficient batch retrieval of an entity's full feature vector in one call, similar in spirit to why the Distributed Cache design emphasizes batched operations for hot-path efficiency.

---

## 7. Streaming Feature Computation — Detailed Sequence

```mermaid
sequenceDiagram
    participant EventStream as Event Stream (Kafka)
    participant StreamProc as Stream Pipeline (Flink)
    participant OnlineStore as Online Feature Store
    participant OfflineStore as Offline Feature Store

    EventStream->>StreamProc: User click event

    StreamProc->>StreamProc: Compute streaming feature:<br/>"clicks in last 5 minutes"<br/>(windowed aggregation)

    StreamProc->>OnlineStore: Update CURRENT value<br/>(overwrites previous value —<br/>online store only cares<br/>about "now")

    StreamProc->>OfflineStore: ALSO append this value<br/>with its event_timestamp<br/>(preserves history for<br/>FUTURE point-in-time<br/>training queries)

    Note over OnlineStore,OfflineStore: Both stores are updated<br/>from the SAME computation,<br/>using the SAME logic —<br/>this is what keeps online<br/>and offline features<br/>genuinely consistent
```

---

## 8. Feature Registry & Discovery

```mermaid
flowchart TB
    A["Data Scientist starting<br/>a new model project"] --> B["Search Feature Registry:<br/>'is there already a feature<br/>for user purchase frequency?'"]

    B --> C{"Feature already exists?"}
    C -- Yes --> D["Reuse existing feature —<br/>avoids duplicate computation<br/>logic, ensures consistency<br/>with other models already<br/>using this same feature"]
    C -- No --> E["Define new feature,<br/>register it with:<br/>computation logic, owner,<br/>documentation, data type"]

    E --> F["New feature becomes<br/>discoverable for FUTURE<br/>projects — the registry's<br/>value compounds over time<br/>as an organization's<br/>feature catalog grows"]

    G["This registry function<br/>mirrors the broader<br/>organizational value of a<br/>documented service/API<br/>catalog — preventing<br/>redundant, potentially<br/>INCONSISTENT reimplementation<br/>of the same underlying<br/>concept across teams"] -.-> D
```

---

## 9. Handling Feature Versioning

```mermaid
flowchart TB
    A["Feature definition needs<br/>to change (e.g., improve<br/>the computation logic for<br/>'average order value')"] --> B{"Versioning Strategy"}

    B --> C["Breaking change<br/>(different output semantics)"]
    C --> D["Register as a NEW version<br/>(e.g., avg_order_value_v2) —<br/>existing models continue<br/>using v1 unaffected until<br/>explicitly migrated"]

    B --> E["Non-breaking refinement<br/>(e.g., bug fix, same semantics)"]
    E --> F["Update in place, but<br/>RECOMPUTE historical offline<br/>values if the fix affects<br/>past data — otherwise models<br/>retrained later would see<br/>inconsistent historical vs<br/>current computation logic"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Feature Store HLD))
    Feature Registry
      Single source of truth definitions
      Discovery and reuse
      Versioning
    Batch Pipeline
      Periodic large-scale computation
      Writes to both stores
    Stream Pipeline
      Real-time windowed computation
      Writes to both stores
    Offline Store
      Full timestamped history
      Point-in-time correct queries
    Online Store
      Latest values only
      Low-latency batch retrieval
    Training Pipeline
      Consumes offline store
      Point-in-time joins
    Serving Service
      Consumes online store
      Single-call feature vector fetch
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Core architecture | Shared feature definitions feeding dual storage (offline + online) | Structurally eliminates training/serving skew by ensuring identical computation logic feeds both paths, rather than relying on process discipline across separate implementations |
| Offline storage | Full timestamped history | Enables point-in-time correct training data generation, preventing label leakage from future information |
| Online storage | Latest-value-only, optimized for low latency | Real-time inference never needs historical values — keeping this store simple and fast by design |
| Feature discovery | Central registry with documentation | Prevents redundant, potentially inconsistent reimplementation of the same conceptual feature across different teams/models |
| Feature fetching (serving) | Batched, single-call retrieval of full feature vector | Avoids the latency cost of many sequential round-trips for models requiring dozens of features |
| Versioning | New version for breaking changes, in-place fix + historical recompute for non-breaking | Balances allowing feature improvement against not silently breaking models already depending on existing behavior |

---

## 12. Bottlenecks & Scaling Considerations

- **Point-in-time join computational cost** — generating training datasets with proper point-in-time correctness across many features and millions of historical examples is computationally expensive; this is often the single most resource-intensive job in an ML pipeline, requiring careful query optimization and often specialized point-in-time join engines rather than naive SQL joins.
- **Online store latency under high fan-out** — models requiring very large feature vectors (hundreds of features) still face aggregate latency cost even with batched retrieval; may require the online store itself to be sharded/optimized specifically for wide, single-entity reads.
- **Streaming feature computation backpressure** — similar to the Analytics/Metrics Dashboard design's stream processing concerns, if the streaming pipeline falls behind event volume, online feature freshness degrades exactly when real-time signals matter most (e.g., during a traffic spike).
- **Feature registry governance at scale** — as the number of registered features grows into the thousands across many teams, without active curation the registry risks becoming cluttered with duplicate, poorly-documented, or abandoned features — this requires ongoing organizational process (feature review, deprecation policies), not just the technical registry infrastructure alone.
- **Consistency window between batch and streaming updates for the same conceptual feature** — if a feature has BOTH a batch-computed baseline AND a streaming real-time adjustment (e.g., daily aggregate plus "activity in the last few minutes"), reconciling these into a single coherent feature value requires careful design to avoid double-counting or gaps.
- **Backfilling historical feature values for NEW features** — when a data scientist defines a brand-new feature, generating its historical values for past training data (not just going-forward) can require reprocessing large volumes of historical raw data — this backfill capability needs to be a first-class, well-supported operation, not an afterthought, since new feature ideas are a constant, ongoing need.
- **Cross-team feature dependency management** — as more models depend on shared features (which is the whole point), a change to a widely-used feature's computation logic can have wide-reaching downstream impact across many models simultaneously; needs the same careful change-management rigor as changing a widely-depended-upon shared API or library.
