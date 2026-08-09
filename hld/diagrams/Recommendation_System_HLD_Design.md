# Design a Recommendation System — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Recommend items (products, videos, articles) personalized to each user
- Support multiple contexts: homepage recommendations, "similar items," "because you watched X"
- Incorporate both explicit signals (ratings, likes) and implicit signals (views, clicks, dwell time)
- Handle cold start for new users and new items gracefully
- Update recommendations as user behavior evolves (not static/stale)

### Non-Functional Requirements
- **Latency:** Recommendations must render in < 100-200ms as part of page load
- **Scale:** Millions of users, millions of items, billions of interaction events
- **Freshness:** Recent interactions should influence recommendations within minutes, not days
- **Diversity:** Avoid narrow, repetitive recommendations (filter bubble problem)
- **Explainability (nice-to-have):** Ability to justify "why was this recommended"

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Users | ~100M |
| Items | ~10M |
| Interaction events/day | ~1B |
| User-item interaction matrix | Extremely sparse (~99.9%+ empty) |
| Recommendation requests/sec | ~50,000 |
| Model retraining cadence | Daily (batch) + real-time signal blending |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    subgraph Ingestion["Signal Ingestion"]
        Events["User Events<br/>(clicks, views, purchases, ratings)"]
        Kafka["Kafka<br/>(interaction event stream)"]
    end

    subgraph Offline["Offline / Batch Layer"]
        BatchJob["Batch Training Pipeline<br/>(Spark — daily)"]
        CFModel["Collaborative Filtering Model<br/>(matrix factorization)"]
        ContentModel["Content-Based Model<br/>(item embeddings)"]
        ItemSimilarity[("Precomputed Item Similarity<br/>(item → similar items)")]
        UserEmbeddings[("Precomputed User Embeddings")]
    end

    subgraph Online["Online / Real-Time Layer"]
        StreamProc["Stream Processor<br/>(recent interactions)"]
        RealtimeSignals[("Real-time Signal Store<br/>(last N interactions per user)")]
    end

    subgraph Serving["Serving Layer"]
        RecoSvc["Recommendation Service"]
        CandidateGen["Candidate Generation<br/>(multiple sources)"]
        RankingSvc["Ranking Service"]
        DiversityFilter["Diversity/Business Rules Filter"]
    end

    Client["Client<br/>(Homepage/App)"]

    Events --> Kafka
    Kafka --> BatchJob
    Kafka --> StreamProc

    BatchJob --> CFModel --> ItemSimilarity
    BatchJob --> ContentModel --> ItemSimilarity
    BatchJob --> UserEmbeddings

    StreamProc --> RealtimeSignals

    Client --> RecoSvc
    RecoSvc --> CandidateGen
    CandidateGen --> ItemSimilarity
    CandidateGen --> UserEmbeddings
    CandidateGen --> RealtimeSignals

    CandidateGen --> RankingSvc --> DiversityFilter --> RecoSvc
    RecoSvc --> Client
```

**Key idea:** Recommendations blend two time horizons — an **offline layer** that periodically (e.g., daily) computes expensive collaborative filtering and embeddings across the entire interaction history, and an **online layer** that incorporates a user's most recent actions in real time. Serving a request means combining precomputed candidates with fresh signals, then ranking — never computing everything from scratch per-request.

---

## 3. Data Model

```mermaid
erDiagram
    USER ||--o{ INTERACTION : performs
    ITEM ||--o{ INTERACTION : "receives"
    ITEM ||--o{ ITEM_SIMILARITY : "similar to"
    USER ||--o{ USER_EMBEDDING : "represented by"

    USER {
        string user_id PK
        timestamp signup_date
    }
    ITEM {
        string item_id PK
        string category
        map content_features
    }
    INTERACTION {
        string user_id FK
        string item_id FK
        string type "view/click/purchase/rating"
        float weight "implicit signal strength"
        timestamp ts
    }
    USER_EMBEDDING {
        string user_id FK
        vector embedding
        timestamp computed_at
    }
    ITEM_SIMILARITY {
        string item_id FK
        string similar_item_id FK
        float similarity_score
    }
```

---

## 4. Collaborative Filtering — Matrix Factorization

```mermaid
flowchart TB
    A["User-Item Interaction Matrix<br/>(sparse: mostly empty cells)"] --> B["Matrix Factorization<br/>(e.g., ALS - Alternating Least Squares)"]
    B --> C["Decompose into:<br/>User Embedding Matrix (U)<br/>× Item Embedding Matrix (I)"]
    C --> D["Each user represented as<br/>a dense low-dimensional vector<br/>(e.g., 128 dimensions)"]
    C --> E["Each item represented as<br/>a dense low-dimensional vector<br/>(same dimensionality)"]
    D & E --> F["Predicted affinity(user, item) =<br/>dot product of their embeddings"]
    F --> G["High dot product = strong<br/>predicted preference,<br/>even for items never<br/>directly interacted with"]
```

**Why this works:** Matrix factorization discovers latent patterns — e.g., it might learn a "prefers sci-fi" dimension without ever being told genres exist, purely from the pattern of which users interacted with which items. This is what lets it recommend items a user has never seen but is likely to enjoy, based on similarity to users with overlapping taste.

---

## 5. Candidate Generation (Multiple Sources Blended)

```mermaid
flowchart TB
    A["Recommendation Request<br/>for user U"] --> B["Source 1: Collaborative Filtering<br/>Top-K items by embedding dot product"]
    A --> C["Source 2: Content-Based<br/>Items similar to what U has liked<br/>(based on item features)"]
    A --> D["Source 3: Real-time Signals<br/>'Because you just viewed X'<br/>— items similar to X"]
    A --> E["Source 4: Popularity/Trending<br/>Fallback for cold-start users"]

    B & C & D & E --> F["Merge candidate pools<br/>(union, dedup)"]
    F --> G["~500-1000 candidates<br/>passed to ranking stage"]
```

*Blending multiple candidate sources — not relying on just one model — improves both coverage (different sources catch different valid recommendations) and resilience (if one signal source is sparse for a given user, others compensate).*

---

## 6. Recommendation Serving Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client
    participant RS as Recommendation Service
    participant CG as Candidate Generation
    participant Emb as User Embeddings Store
    participant ItemSim as Item Similarity Store
    participant RT as Real-time Signal Store
    participant Rank as Ranking Service
    participant Filter as Diversity Filter

    C->>RS: GET /recommendations?user_id=U
    RS->>CG: Generate candidates

    par Parallel candidate fetching
        CG->>Emb: Get user embedding
        Emb-->>CG: vector
        CG->>ItemSim: Top-K similar items via<br/>approximate nearest neighbor search
        ItemSim-->>CG: Candidate set A
    and
        CG->>RT: Get user's last N interactions
        RT-->>CG: Recent item_ids
        CG->>ItemSim: Similar items to recent interactions
        ItemSim-->>CG: Candidate set B
    end

    CG-->>RS: Merged candidate pool (~500 items)
    RS->>Rank: Score all candidates<br/>(predicted engagement)
    Rank-->>RS: Ranked list

    RS->>Filter: Apply diversity rules<br/>(cap items per category,<br/>remove already-purchased)
    Filter-->>RS: Final list (top 20)
    RS-->>C: Return recommendations
```

---

## 7. Cold Start Handling

```mermaid
flowchart TB
    A["New user<br/>(no interaction history)"] --> B{"Cold Start Strategy"}
    B --> C["Show popularity-based<br/>recommendations<br/>(globally trending items)"]
    B --> D["Ask onboarding questions<br/>(explicit preference signals)"]
    B --> E["Use demographic/contextual<br/>signals if available<br/>(location, device, referral source)"]

    F["New item<br/>(no interaction history)"] --> G{"Cold Start Strategy"}
    G --> H["Content-based matching<br/>using item's own features<br/>(no interactions needed)"]
    G --> I["Boost visibility temporarily<br/>to gather initial interaction signal<br/>(exploration)"]

    J["Both converge to<br/>collaborative filtering<br/>once enough interaction<br/>data accumulates"]
```

---

## 8. Exploration vs Exploitation (Multi-Armed Bandit Approach)

```mermaid
flowchart TB
    A["Purely exploiting known preferences"] --> B["Risk: recommendations become<br/>a narrow filter bubble,<br/>never surfacing new interests"]

    C["Bandit-based exploration"] --> D["Reserve small % of<br/>recommendation slots for<br/>exploratory/untested items"]
    D --> E["Track engagement on<br/>exploratory picks"]
    E --> F["Successful exploratory items<br/>feed back into future<br/>exploitation-based ranking"]

    G["Balances immediate engagement<br/>(exploitation) against<br/>discovering new signal<br/>(exploration) — classic<br/>multi-armed bandit tradeoff"]
```

---

## 9. Real-Time Signal Incorporation

```mermaid
sequenceDiagram
    participant U as User
    participant App as Application
    participant K as Kafka
    participant Stream as Stream Processor
    participant RT as Real-time Signal Store

    U->>App: Views/clicks Item X
    App->>K: Emit InteractionEvent

    K->>Stream: Consume event
    Stream->>Stream: Update user's recent<br/>interaction window<br/>(sliding, last 20-50 items)
    Stream->>RT: Write updated signal

    Note over RT: Next recommendation request<br/>for this user incorporates<br/>Item X immediately —<br/>no need to wait for<br/>next batch training cycle
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Recommendation System HLD))
    Batch Training Pipeline
      Matrix factorization (CF)
      Content-based embeddings
      Daily/periodic retraining
    Stream Processor
      Real-time interaction ingestion
      Sliding recent-activity window
    Candidate Generation
      Multi-source blending
      CF + content + real-time + trending
    Ranking Service
      Predicted engagement scoring
      Combines candidate signals
    Diversity Filter
      Category caps
      Business rule enforcement
    Cold Start Handler
      Popularity fallback
      Content-based for new items
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Model architecture | Offline batch training + online real-time signal blending | Expensive collaborative filtering can't run per-request; real-time signals fill the freshness gap between training runs |
| Candidate generation | Multiple blended sources, not single model | Improves coverage and resilience — different sources compensate for each other's weaknesses per user |
| Cold start | Popularity + content-based fallback | Collaborative filtering fundamentally requires interaction history that doesn't exist yet for new users/items |
| Exploration | Reserved bandit-based exploratory slots | Pure exploitation creates filter bubbles and never discovers new valid recommendations |
| Similarity search | Approximate nearest neighbor (ANN) | Exact nearest-neighbor search across millions of item embeddings is too slow for real-time serving; ANN trades small accuracy loss for speed |
| Retraining cadence | Daily batch + continuous real-time overlay | Full model retraining is expensive; daily cadence balances model freshness against compute cost |

---

## 12. Bottlenecks & Scaling Considerations

- **Approximate nearest neighbor search at scale** — finding "top-K similar items" across millions of embeddings must use ANN structures (e.g., HNSW, FAISS-style indexes) rather than brute-force distance computation, which wouldn't meet latency requirements.
- **Training pipeline compute cost** — matrix factorization over billions of interactions is a significant batch compute job; needs efficient distributed processing (Spark) and careful scheduling to complete within the retraining window.
- **Real-time signal store latency** — must support very fast reads (single-digit ms) since it's on the critical serving path for every recommendation request.
- **Sparse interaction data for niche items/users** — the long tail of items with very few interactions produces unreliable collaborative filtering signal; content-based approaches and popularity fallback compensate but never fully solve this.
- **Feedback loop bias** — recommendations influence what users see, which influences what they interact with, which trains future recommendations — this self-reinforcing loop can amplify existing biases if not actively counteracted with exploration.
- **Candidate pool size vs ranking cost** — larger candidate pools improve recommendation quality but increase ranking compute cost per request; this is the same multi-stage funnel tradeoff seen in general feed ranking systems.
- **A/B testing complexity** — recommendation quality is inherently hard to measure with simple metrics; requires careful experiment design (e.g., long-term engagement, not just immediate click-through) to avoid optimizing for short-term metrics that harm long-term satisfaction.
