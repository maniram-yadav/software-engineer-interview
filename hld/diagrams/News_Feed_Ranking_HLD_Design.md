# Design a News Feed Ranking System — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Given a candidate set of posts (from follow graph, groups, ads), rank them for a specific user
- Balance freshness (recent posts) vs relevance (predicted engagement)
- Support real-time signals (a post going viral should surface faster)
- Support explainability/debugging of why a post ranked where it did
- A/B testable — ranking model changes must be measurable

### Non-Functional Requirements
- **Latency:** Full ranking pipeline < 150ms p99 (feed must feel instant)
- **Scale:** ~500M DAU requesting feeds, each with 100s–1000s of candidates to score
- **Freshness:** New signals (likes, comments) should influence ranking within seconds
- **Consistency:** Not critical — feed ranking is inherently probabilistic/personalized

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Feed requests/sec (peak) | ~500,000 |
| Avg candidates scored per request | ~500 |
| Total scoring ops/sec | ~250M/sec |
| Feature store lookups/sec | Very high — needs aggressive caching |
| Model inference latency budget | < 50ms of the 150ms total |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Client["Client Requests Feed"]
    Gateway["API Gateway"]
    FeedOrchestrator["Feed Orchestrator Service"]

    subgraph Candidate["1. Candidate Generation"]
        CG1["Follow Graph Candidates<br/>(posts from followed users)"]
        CG2["Group/Community Candidates"]
        CG3["Ad Candidates"]
        CG4["Recommendation Candidates<br/>(similar users, ML-suggested)"]
    end

    subgraph Features["2. Feature Enrichment"]
        FeatureStore["Feature Store<br/>(user features, post features,<br/>author-viewer affinity)"]
        RealtimeSignals["Real-time Signal Stream<br/>(live engagement counters)"]
    end

    subgraph Ranking["3. Ranking"]
        LightRanker["Light Ranker<br/>(cheap model, prunes 1000s → 100s)"]
        HeavyRanker["Heavy Ranker<br/>(deep model, scores final ~100)"]
    end

    subgraph PostProcess["4. Post-Processing"]
        Filter["Filtering<br/>(seen posts, blocked users, policy)"]
        Diversity["Diversity Injection<br/>(avoid same-author clustering)"]
        AdsInject["Ad Insertion"]
    end

    Client --> Gateway --> FeedOrchestrator
    FeedOrchestrator --> CG1
    FeedOrchestrator --> CG2
    FeedOrchestrator --> CG3
    FeedOrchestrator --> CG4

    CG1 --> FeatureStore
    CG2 --> FeatureStore
    CG3 --> FeatureStore
    CG4 --> FeatureStore
    FeatureStore --> RealtimeSignals

    FeatureStore --> LightRanker
    LightRanker --> HeavyRanker
    HeavyRanker --> Filter
    Filter --> Diversity
    Diversity --> AdsInject
    AdsInject --> Client
```

**Key idea:** Ranking happens in funnel stages — thousands of raw candidates are cheaply pruned down before an expensive deep model scores only the survivors. This keeps latency bounded even as the candidate pool grows.

---

## 3. Ranking Funnel (Multi-Stage Scoring)

```mermaid
flowchart LR
    A["Candidate Pool<br/>~5,000 posts"] --> B["Stage 1: Candidate Generation<br/>Rule-based / ANN retrieval<br/>~5,000 → 1,000"]
    B --> C["Stage 2: Light Ranking<br/>Simple model (logistic regression /<br/>shallow tree), cheap features<br/>1,000 → 100"]
    C --> D["Stage 3: Heavy Ranking<br/>Deep neural net, full feature set<br/>100 → 100 (scored)"]
    D --> E["Stage 4: Business Logic<br/>Filters, diversity, ads<br/>100 → 50 (final feed page)"]
```

*Each stage trades scoring accuracy for speed — early stages are cheap and approximate to survive high volume; late stages are expensive but only run on a small surviving set.*

---

## 4. Feature Categories

```mermaid
mindmap
  root((Ranking Features))
    User Features
      Past engagement history
      Time-of-day activity pattern
      Interests/topics embedding
    Post Features
      Age/freshness
      Media type
      Current engagement velocity
      Content embedding
    Author-Viewer Affinity
      Interaction frequency
      Relationship strength score
      Profile visit history
    Real-time Signals
      Likes/comments in last N minutes
      Trending velocity
      Live viewer count
    Context Features
      Device type
      Network speed
      Session length so far
```

---

## 5. Feature Store Architecture

```mermaid
flowchart TB
    subgraph Offline["Offline Pipeline"]
        BatchJob["Batch ETL Jobs<br/>(Spark, daily/hourly)"]
        HistData[("Historical Interaction Data<br/>(Data Warehouse)")]
    end

    subgraph Online["Online Pipeline"]
        Stream["Stream Processor<br/>(Flink/Kafka Streams)"]
        EventBus["Event Bus<br/>(likes, comments, views)"]
    end

    subgraph Store["Feature Store"]
        OfflineStore[("Offline Feature Store<br/>(for training)")]
        OnlineStore[("Online Feature Store<br/>(Redis/low-latency KV,<br/>for real-time serving)")]
    end

    HistData --> BatchJob --> OfflineStore
    EventBus --> Stream --> OnlineStore
    OfflineStore -.->|"Point-in-time consistent<br/>sync"| OnlineStore

    RankerService["Ranking Service"] --> OnlineStore
    TrainingPipeline["Model Training Pipeline"] --> OfflineStore
```

**Key challenge — training/serving skew:** The offline store (used for model training) and online store (used at serving time) must return the *same* feature values for the same point in time, or the model will underperform in production relative to training. This is the **point-in-time correctness** problem.

---

## 6. Ranking Request — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client
    participant FO as Feed Orchestrator
    participant CG as Candidate Generators
    participant FS as Feature Store
    participant LR as Light Ranker
    participant HR as Heavy Ranker
    participant PP as Post-Processor

    C->>FO: GET /feed
    FO->>CG: Fetch candidates (parallel calls)
    CG-->>FO: ~5,000 candidate post_ids

    FO->>FS: Batch fetch features for user + candidates
    FS-->>FO: Feature vectors

    FO->>LR: Score all candidates (cheap model)
    LR-->>FO: Top 100 by light score

    FO->>HR: Score top 100 (deep model)
    HR-->>FO: Final relevance scores

    FO->>PP: Apply filters, diversity, ads
    PP-->>FO: Final ranked list (50 posts)

    FO-->>C: Return ranked feed
```

---

## 7. Real-Time Signal Propagation

```mermaid
sequenceDiagram
    participant U as User
    participant EngageSvc as Engagement Service
    participant K as Kafka
    participant StreamProc as Stream Processor (Flink)
    participant OnlineFS as Online Feature Store

    U->>EngageSvc: Like/Comment on Post X
    EngageSvc->>K: Emit EngagementEvent

    K->>StreamProc: Consume event
    StreamProc->>StreamProc: Aggregate (windowed count,<br/>engagement velocity)
    StreamProc->>OnlineFS: Update post_X engagement features

    Note over OnlineFS: Next feed request for any user<br/>sees updated "trending" signal<br/>within seconds
```

---

## 8. A/B Testing & Model Rollout

```mermaid
flowchart TB
    A["New Ranking Model Trained"] --> B["Offline Evaluation<br/>(NDCG, engagement prediction AUC)"]
    B --> C{"Passes offline<br/>metrics bar?"}
    C -- No --> D["Iterate on model"]
    C -- Yes --> E["Canary Rollout<br/>(1% of traffic)"]
    E --> F["Monitor online metrics<br/>(session time, DAU, complaints)"]
    F --> G{"Metrics healthy?"}
    G -- No --> H["Rollback"]
    G -- Yes --> I["Gradual Ramp<br/>5% → 25% → 50% → 100%"]
    I --> J["Full Rollout"]
```

---

## 9. Component Responsibilities Summary

| Component | Responsibility |
|---|---|
| **Feed Orchestrator** | Coordinates the entire ranking pipeline per request |
| **Candidate Generators** | Pull raw candidate posts from multiple sources (graph, groups, ads, ML recs) |
| **Feature Store (online)** | Low-latency serving of precomputed + real-time features |
| **Feature Store (offline)** | Historical features for model training, point-in-time correct |
| **Light Ranker** | Cheap model to prune large candidate pools fast |
| **Heavy Ranker** | Expensive deep model for final precision scoring on survivors |
| **Post-Processor** | Business rules: filtering seen content, diversity, ad injection |
| **Stream Processor** | Real-time aggregation of engagement signals feeding back into ranking |

---

## 10. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Ranking architecture | Multi-stage funnel (light → heavy) | Bounds latency; expensive models only run on a pruned candidate set |
| Feature storage | Separate online/offline stores | Online needs low-latency KV access; offline needs large-scale batch analytics |
| Real-time signals | Stream processing (Flink/Kafka Streams) | Engagement-driven ranking needs sub-minute freshness, not batch-hour freshness |
| Model rollout | Canary + gradual ramp | Ranking changes directly affect engagement/revenue; must de-risk before full rollout |
| Diversity injection | Post-processing step, not in model | Keeps the ranking model focused on relevance; diversity is a business rule layered on top |
| Ads | Injected after organic ranking | Cleanly separates relevance optimization from monetization logic |

---

## 11. Bottlenecks & Scaling Considerations

- **Feature store read amplification** — every feed request needs features for hundreds of candidates × millions of requests/sec. Mitigate with aggressive caching and batching lookups.
- **Heavy ranker latency** — deep models are expensive; cap candidates entering this stage strictly (e.g., top 100 only) and consider model distillation for speed.
- **Training/serving skew** — feature drift between offline training data and online serving data silently degrades model quality; needs continuous monitoring (feature distribution parity checks).
- **Cold-start users/posts** — new users/posts lack engagement history; fall back to content-based or popularity-based features until enough signal accumulates.
- **Real-time signal storms** (viral post) — engagement events can spike Kafka throughput; ensure stream processor auto-scales and uses windowed aggregation rather than per-event recompute.
- **Feedback loops** — ranking model trained on past engagement can reinforce existing biases (popular posts get shown more → get more engagement → rank higher); needs exploration/exploitation balance (e.g., small % randomized exposure).
