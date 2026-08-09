# Design a Real-Time Credit Card Fraud Detection System — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Score every transaction for fraud risk BEFORE it's approved — this is a synchronous, blocking decision, not after-the-fact analysis
- Combine rule-based checks (known fraud patterns) with ML-based scoring (learned patterns)
- Support a human review queue for ambiguous/borderline transactions
- Continuously incorporate feedback (confirmed fraud, false positives) to improve future scoring

### Non-Functional Requirements
- **Extremely low latency:** The fraud check happens IN-LINE with the payment authorization flow — must complete in tens of milliseconds, not seconds, or it delays every single legitimate transaction too
- **High availability:** A fraud detection outage cannot be allowed to block all transactions platform-wide — needs a well-defined fail-safe behavior
- **High accuracy with asymmetric costs:** A false negative (missed fraud) costs real money; a false positive (blocking a legitimate transaction) costs customer trust/friction — these have different, context-dependent costs
- **Adaptability:** Fraud patterns evolve constantly and adversarially — the system must be continuously retrainable, not a fixed, static ruleset

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Transactions/sec (platform-wide peak) | ~50,000 |
| Latency budget for fraud check | < 50-100ms (part of overall payment auth latency budget) |
| Fraud rate (industry typical) | 0.1-1% of transactions |
| Feature computation | Must be pre-computed/cached — no time for expensive queries during the synchronous check |

---

## 2. The Core Architectural Constraint — This Is a Synchronous, Latency-Critical Decision

```mermaid
flowchart TB
    A["Credit card transaction<br/>initiated at checkout"] --> B["Payment authorization flow<br/>(talks to card network,<br/>issuing bank, etc.)"]
    B --> C["Fraud check MUST complete<br/>WITHIN this flow — the<br/>transaction is literally<br/>APPROVED OR DECLINED based<br/>on this decision"]

    C --> D["This is fundamentally<br/>different from the Ad Click<br/>Fraud Detection design's<br/>APPROACH — that system could<br/>flag fraud asynchronously,<br/>AFTER the click already<br/>happened, adjusting billing<br/>later. Credit card fraud<br/>detection has NO SUCH LUXURY<br/>— the decision gates the<br/>transaction itself, in<br/>real time"]

    E["Consequence: EVERY component<br/>in this system's hot path<br/>must be optimized for<br/>extreme low latency —<br/>there is no 'compute this<br/>later, correct it after<br/>the fact' option for the<br/>PRIMARY approve/decline<br/>decision"] --> D
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    PaymentGW["Payment Gateway<br/>(orchestrates the<br/>authorization flow)"]

    subgraph FraudSystem["Fraud Detection System"]
        RuleEngine["Rule Engine<br/>(fast, deterministic checks)"]
        FeatureStore["Online Feature Store<br/>(pre-computed, low-latency)"]
        MLScorer["ML Model Scorer"]
        DecisionEngine["Decision Engine<br/>(combines signals,<br/>makes final call)"]
    end

    subgraph AsyncPath["Async Feedback Loop"]
        Kafka["Kafka<br/>(transaction outcomes)"]
        FeatureUpdater["Feature Update Workers"]
        ReviewQueue["Human Review Queue<br/>(borderline cases)"]
        TrainingPipeline["Model Retraining Pipeline"]
    end

    PaymentGW -->|"Synchronous call,<br/>tight latency budget"| RuleEngine
    RuleEngine --> DecisionEngine
    DecisionEngine --> FeatureStore
    FeatureStore --> MLScorer
    MLScorer --> DecisionEngine
    DecisionEngine -->|"APPROVE / DECLINE / REVIEW"| PaymentGW

    PaymentGW --> Kafka
    Kafka --> FeatureUpdater --> FeatureStore
    DecisionEngine -->|"borderline score"| ReviewQueue
    Kafka --> TrainingPipeline
    TrainingPipeline -.->|"periodically deploys<br/>updated model"| MLScorer
```

**Key idea:** The synchronous hot path (rule engine → feature lookup → ML scoring → decision) is deliberately kept as lean as possible, drawing only from PRE-COMPUTED, low-latency data sources. All the expensive, time-consuming work — updating features from new transaction history, retraining models, human review — happens entirely in a separate asynchronous path that feeds back into the hot path's data sources without ever blocking it.

---

## 4. Data Model

```mermaid
erDiagram
    TRANSACTION {
        string transaction_id PK
        string card_id
        string merchant_id
        float amount
        string location
        timestamp initiated_at
        string decision "approve/decline/review"
        float fraud_score
    }
    CARD_FEATURE_PROFILE {
        string card_id PK
        float avg_transaction_amount
        int transactions_last_1hr
        string common_merchant_categories
        string common_locations
        timestamp last_transaction_at
        timestamp last_transaction_location_ts
    }
    REVIEW_CASE {
        string case_id PK
        string transaction_id FK
        string status "pending/confirmed_fraud/confirmed_legit"
        string reviewer_id
        timestamp reviewed_at
    }
```

---

## 5. Real-Time Transaction Scoring Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Merchant as Merchant/Checkout
    participant Gateway as Payment Gateway
    participant Rules as Rule Engine
    participant FS as Feature Store
    participant ML as ML Scorer
    participant Decision as Decision Engine

    Merchant->>Gateway: Authorize transaction<br/>{card, amount, merchant, location}
    Gateway->>Rules: Evaluate fast rules<br/>(< 5ms budget)

    Rules->>Rules: Check: card on known<br/>stolen-card blocklist?
    Rules->>Rules: Check: transaction amount<br/>exceeds hard velocity limit?

    alt High-confidence rule match (e.g., blocklisted card)
        Rules-->>Gateway: IMMEDIATE DECLINE<br/>(skip ML scoring entirely —<br/>no need for further analysis)
    else No high-confidence rule match
        Rules->>FS: Fetch pre-computed features<br/>for this card_id<br/>(< 10ms budget — single<br/>low-latency KV lookup)
        FS-->>Rules: Feature vector<br/>{avg_amount, txn_velocity,<br/>location_history, ...}

        Rules->>ML: Score transaction<br/>(< 20ms budget)
        ML->>ML: Run inference using<br/>current transaction +<br/>fetched features
        ML-->>Rules: Fraud probability score

        Rules->>Decision: Combine rule signals<br/>+ ML score
        Decision->>Decision: Apply decision thresholds

        alt Score below LOW threshold
            Decision-->>Gateway: APPROVE
        else Score above HIGH threshold
            Decision-->>Gateway: DECLINE
        else Score in AMBIGUOUS range
            Decision-->>Gateway: APPROVE, but flag for<br/>async review (don't block<br/>the customer for<br/>borderline cases)
        end
    end

    Gateway-->>Merchant: Final authorization decision<br/>(total elapsed: well under<br/>the latency budget)
```

**Why the "ambiguous range" defaults to approve-but-flag, not block-and-wait:** Blocking a legitimate customer's transaction for manual review would be an unacceptable customer experience for the (statistically likely) common case where the transaction is actually fine. The system optimizes for NOT interrupting the customer's real-time experience, accepting some risk on ambiguous cases in exchange for reviewing them asynchronously and taking corrective action (chargebacks, account flags) if fraud is later confirmed.

---

## 6. Feature Engineering — What Makes Scoring Fast

```mermaid
flowchart TB
    A["ML Model needs RICH<br/>features to score accurately<br/>— but computing them from<br/>scratch during the<br/>synchronous request would<br/>blow the latency budget"] --> B{"Feature Computation Strategy"}

    B --> C["Pre-computed, continuously<br/>updated features<br/>(same architecture as the<br/>general Ranking Feature Store<br/>design)"]
    C --> C1["'Average transaction amount<br/>for this card, last 30 days'<br/>— computed and updated<br/>incrementally as new<br/>transactions occur, NOT<br/>recalculated from raw<br/>history on every request"]

    B --> D["Point-in-request features<br/>(fast, computed from THIS<br/>transaction alone)"]
    D --> D1["'Is this merchant category<br/>unusual for 2am?' — simple<br/>calculations using only<br/>the current transaction's<br/>own attributes, no lookup<br/>needed"]

    E["The synchronous path NEVER<br/>computes anything requiring<br/>a scan of historical raw<br/>transaction data — that<br/>expensive work happens ONLY<br/>in the async feature-update<br/>pipeline"] -.-> C1
```

---

## 7. Asynchronous Feature Update Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Gateway as Payment Gateway
    participant K as Kafka
    participant Updater as Feature Update Worker
    participant FS as Feature Store

    Gateway->>K: Emit TransactionCompleted event<br/>(regardless of approve/decline)

    K->>Updater: Consume event
    Updater->>Updater: Incrementally update running<br/>aggregates for this card_id:<br/>- transaction count (sliding window)<br/>- average amount<br/>- recent location history<br/>- recent merchant categories

    Updater->>FS: Write updated feature values

    Note over FS: Next transaction for this<br/>card_id will see these<br/>UPDATED features in its<br/>synchronous scoring path —<br/>freshness lag is typically<br/>seconds, which is more than<br/>fast enough given fraud<br/>patterns rarely require<br/>sub-second feature freshness
```

---

## 8. Rule Engine vs ML Model — Complementary, Not Redundant

```mermaid
flowchart TB
    A["Why use BOTH rules AND<br/>ML, rather than just one?"] --> B["Rules: fast, deterministic,<br/>EXPLAINABLE, handle KNOWN<br/>high-confidence patterns"]
    B --> B1["e.g., 'card reported stolen'<br/>— no need for probabilistic<br/>scoring, this is an<br/>absolute, immediate decline"]
    B --> B2["Advantage: zero false<br/>negatives for KNOWN patterns,<br/>instant to update (add a<br/>rule) without model<br/>retraining"]

    C["ML: probabilistic, catches<br/>SUBTLE, EVOLVING, previously<br/>unseen patterns that no<br/>human would think to<br/>write as an explicit rule"] --> C1["e.g., 'this specific<br/>combination of transaction<br/>timing, amount, and merchant<br/>category subtly resembles<br/>patterns seen in past<br/>confirmed fraud cases'"]
    C --> C2["Advantage: adapts to<br/>NOVEL fraud patterns rules<br/>haven't been written for yet"]

    D["Layering both: rules catch<br/>the 'obvious' cases instantly<br/>and cheaply (often SKIPPING<br/>the more expensive ML<br/>inference entirely), while<br/>ML handles the nuanced,<br/>evolving remainder"] -.-> C2
```

---

## 9. Human Review Queue & Feedback Loop

```mermaid
sequenceDiagram
    participant Decision as Decision Engine
    participant Queue as Review Queue
    participant Analyst as Fraud Analyst
    participant K as Kafka
    participant Training as Model Retraining Pipeline

    Decision->>Queue: Flag transaction<br/>(ambiguous score) for review

    Analyst->>Queue: Investigate case<br/>(check transaction details,<br/>contact customer if needed)
    Analyst->>Queue: Resolve: CONFIRMED_FRAUD<br/>or CONFIRMED_LEGITIMATE

    Queue->>K: Emit labeled outcome<br/>{transaction_id, true_label}

    K->>Training: Consume labeled outcomes<br/>(this is GROUND TRUTH<br/>training data — far more<br/>valuable than the model's<br/>own predictions)

    Note over Training: Periodically retrain the<br/>ML model incorporating<br/>this new labeled data,<br/>improving future accuracy<br/>on similar patterns
```

**Why human-reviewed outcomes are the most valuable training data:** Unlike many ML systems that can bootstrap from historical data alone, fraud detection benefits enormously from a continuous stream of DEFINITIVELY LABELED outcomes (confirmed fraud vs confirmed legitimate) — this human-in-the-loop feedback is what allows the model to genuinely learn and adapt to evolving fraud patterns, rather than becoming stale.

---

## 10. Handling Fraud Detection System Failure (Fail-Safe Design)

```mermaid
flowchart TB
    A["Fraud Detection System<br/>becomes unavailable/times out"] --> B{"Fail-Safe Policy"}

    B --> C["Fail Open<br/>(approve transactions<br/>without fraud check)"]
    C --> C1["Risk: temporary window of<br/>zero fraud protection<br/>Benefit: doesn't block ALL<br/>legitimate commerce platform-wide"]

    B --> D["Fail Closed<br/>(decline all transactions)"]
    D --> D1["Risk: a fraud system outage<br/>becomes a COMPLETE payment<br/>outage — likely FAR more<br/>costly than temporary<br/>fraud exposure"]

    E["Typical production choice:<br/>Fail Open, but with<br/>TIGHTENED rule-based-only<br/>fallback thresholds during<br/>the outage window (e.g.,<br/>stricter velocity limits<br/>applied via a simpler,<br/>independently-available<br/>rule check) rather than<br/>a complete bypass"] -.-> C1
```

---

## 11. Component Responsibilities Summary

```mermaid
mindmap
  root((Fraud Detection HLD))
    Rule Engine
      Fast, deterministic checks
      High-confidence immediate decisions
    Online Feature Store
      Pre-computed, low-latency
      Never computed fresh in hot path
    ML Scorer
      Probabilistic inference
      Catches novel patterns
    Decision Engine
      Combines rule and ML signals
      Applies approve/decline/review thresholds
    Feature Update Workers
      Async, incremental aggregation
      Feeds the online feature store
    Review Queue
      Human-in-the-loop for ambiguous cases
      Generates ground-truth labels
    Retraining Pipeline
      Incorporates labeled feedback
      Periodic model updates
```

---

## 12. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Architecture | Synchronous, latency-critical hot path + fully decoupled async feedback loop | The approve/decline decision gates the transaction itself in real time, unlike systems (e.g., ad click fraud) that can correct after the fact |
| Detection approach | Layered rules + ML, not either alone | Rules provide fast, explainable, zero-latency handling of known patterns; ML adapts to novel, evolving patterns rules can't anticipate |
| Feature computation | Fully pre-computed, incrementally updated | The synchronous path cannot afford to compute anything from raw historical data on the fly |
| Ambiguous case handling | Approve-and-flag, not block-and-wait | Prioritizes not disrupting the common case (legitimate transactions) over catching every possible fraud case synchronously |
| Failure mode | Fail open with tightened fallback rules | A fraud system outage causing a complete payment outage is generally far more costly than a temporary window of reduced fraud protection |
| Model improvement | Human-reviewed ground truth feeds retraining | Definitively labeled outcomes are the highest-value training signal for continuously adapting to evolving fraud patterns |

---

## 13. Bottlenecks & Scaling Considerations

- **Feature store latency is the dominant hot-path risk** — since this is the one component in the synchronous path requiring a network round-trip to external state (not just in-request computation), its latency and availability directly bound the entire fraud check's latency budget; needs to be treated with the same criticality as the idempotency store in the Idempotent API Requests design.
- **Model inference latency at peak load** — ML scoring must sustain the platform's full peak transaction rate without latency degradation; this often requires model optimization (quantization, distillation) specifically for the inference-speed/accuracy tradeoff, not just using the most accurate model available regardless of latency cost.
- **Adversarial adaptation** — as explicitly noted in the related Ad Click Fraud Detection design, this is fundamentally an adversarial system; fraudsters actively probe and adapt to evade known detection patterns, making continuous retraining and rule updates a permanent operational necessity, not a one-time build.
- **Feature staleness during high-velocity fraud attempts** — a fraudster making many rapid transactions might exploit the (typically seconds-long) lag between a transaction occurring and its effect appearing in pre-computed features; may need a lightweight, very-low-latency real-time velocity check layered alongside the standard feature store for the most time-sensitive signals.
- **False positive cost asymmetry across contexts** — declining a legitimate $10,000 transaction from a long-standing high-value customer has different real business cost than declining a $10 transaction from a new customer; decision thresholds may need to be context-aware (customer tenure, transaction history) rather than a single global threshold.
- **Review queue backlog during fraud spikes** — a coordinated fraud attack can generate a surge of ambiguous-scored transactions simultaneously, overwhelming the human review capacity; needs either auto-scaling review capacity or temporary threshold tightening (accepting more false positives) during detected attack windows.
- **Cross-border/multi-currency complexity** — transaction patterns, typical amounts, and fraud indicators vary significantly by region and currency; a single global model/feature set may underperform compared to regionally-tuned scoring, adding a dimension of complexity to both the feature store and model architecture beyond what's shown in this simplified design.
