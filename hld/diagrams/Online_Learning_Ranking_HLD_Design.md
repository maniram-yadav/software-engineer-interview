# Design a Personalization/Ranking System With Online Learning — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Rank/select content or actions personalized to each user, learning and adapting from real-time feedback (clicks, engagement)
- Balance EXPLOITATION (showing content known to perform well) against EXPLORATION (trying less-certain options to gather new information)
- Update the ranking policy CONTINUOUSLY from live feedback, not just via periodic full retraining
- Support multiple simultaneous "arms"/options being evaluated (e.g., many possible articles, ads, or recommendations)

### Non-Functional Requirements
- **Real-time adaptation:** Unlike the batch-trained ranking models in the earlier News Feed Ranking design, this system must update its policy from feedback WITHIN the same session/minutes, not the next day's training run
- **Regret minimization:** The system should mathematically minimize cumulative "regret" — the gap between what it showed and what the BEST possible choice would have been, over time
- **Cold-start handling:** New content/options with no feedback history yet must still get a fair chance to be shown and evaluated
- **Scale:** Must make this exploration/exploitation decision for millions of ranking decisions per second

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Ranking decisions/sec | Millions (one per user impression/request) |
| Distinct "arms" (content options) | Thousands to millions, depending on domain |
| Feedback latency | Seconds (click) to longer (conversion) |
| Policy update frequency | Continuous/near-real-time, not batch-only |

---

## 2. The Core Problem — Why Pure Exploitation Fails

```mermaid
flowchart TB
    A["Pure exploitation strategy:<br/>ALWAYS show the option with<br/>the HIGHEST currently-known<br/>performance"] --> B["Problem: an option that<br/>hasn't been shown much yet<br/>might ACTUALLY be better,<br/>but the system doesn't know<br/>this because it lacks enough<br/>data — pure exploitation<br/>NEVER gathers that data,<br/>since it never gives<br/>uncertain options a chance"]

    B --> C["Result: the system gets<br/>PERMANENTLY STUCK exploiting<br/>whatever happened to look<br/>good EARLY (potentially by<br/>chance/noise), while genuinely<br/>superior options remain<br/>forever undiscovered — this<br/>is a well-known failure<br/>mode called premature<br/>convergence"]

    D["The exploration/exploitation<br/>tradeoff — the CENTRAL problem<br/>this system solves — requires<br/>DELIBERATELY sacrificing some<br/>immediate performance<br/>(showing uncertain options<br/>sometimes) in exchange for<br/>the LONG-TERM information<br/>needed to make better<br/>decisions overall"] --> C
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    Client["User Request<br/>(needs a ranked/selected result)"]

    subgraph Serving["Real-Time Serving Layer"]
        BanditSvc["Bandit Decision Service"]
        ArmStats[("Arm Statistics Store<br/>— per-arm performance<br/>estimates, updated continuously")]
    end

    subgraph Feedback["Feedback Loop"]
        FeedbackCollector["Feedback Collector<br/>(clicks, conversions)"]
        Kafka["Kafka<br/>(feedback event stream)"]
        StatsUpdater["Statistics Update Workers"]
    end

    subgraph ColdStart["Cold-Start Handling"]
        NewArmDetector["New Arm Detector"]
    end

    Client --> BanditSvc
    BanditSvc --> ArmStats
    BanditSvc -->|"selected option"| Client

    Client -->|"user interacts<br/>(click/no-click)"| FeedbackCollector
    FeedbackCollector --> Kafka
    Kafka --> StatsUpdater
    StatsUpdater --> ArmStats

    NewArmDetector --> ArmStats
```

**Key idea:** Unlike the batch-trained ranking model in the general News Feed Ranking design, this system's core statistics are updated CONTINUOUSLY from a live feedback stream — the loop from "show an option" to "observe the outcome" to "update future decisions" happens in near-real-time, which is precisely what "online learning" means as distinct from periodic offline retraining.

---

## 4. Data Model

```mermaid
erDiagram
    ARM {
        string arm_id PK
        string content_type
        int total_impressions
        int total_successes "clicks/conversions"
        float estimated_success_rate
        float uncertainty_bound
        timestamp first_shown_at
    }
    IMPRESSION_EVENT {
        string impression_id PK
        string arm_id FK
        string user_id
        timestamp shown_at
        bool resulted_in_success "nullable until observed"
    }
    CONTEXT_FEATURES {
        string impression_id FK
        map user_context "for contextual bandits"
    }
```

---

## 5. Multi-Armed Bandit Algorithms — Core Strategies

```mermaid
flowchart TB
    A["Bandit Algorithm Choice"] --> B["Epsilon-Greedy"]
    A --> C["Upper Confidence Bound<br/>(UCB)"]
    A --> D["Thompson Sampling"]

    B --> B1["Simple: with probability ε<br/>(e.g., 10%), pick a RANDOM<br/>arm (exploration); otherwise<br/>pick the current best-known<br/>arm (exploitation)"]
    B --> B2["CON: explores uniformly<br/>at random, even OBVIOUSLY<br/>bad arms get equal<br/>exploration budget as<br/>promising uncertain ones"]

    C --> C1["Pick the arm with the<br/>highest UPPER BOUND of its<br/>confidence interval — arms<br/>with FEWER observations have<br/>WIDER confidence intervals,<br/>naturally getting explored<br/>more until enough data<br/>narrows the uncertainty"]
    C --> C2["PRO: exploration is<br/>PROPORTIONAL to uncertainty,<br/>not random — much more<br/>efficient than epsilon-greedy"]

    D --> D1["Bayesian approach: maintain<br/>a PROBABILITY DISTRIBUTION<br/>over each arm's true success<br/>rate; on each decision,<br/>SAMPLE from each arm's<br/>distribution and pick the<br/>arm with the highest sample"]
    D --> D2["PRO: naturally balances<br/>exploration/exploitation<br/>based on genuine uncertainty,<br/>often performs excellently<br/>in practice, and is<br/>relatively simple to implement"]

    E["This design uses Thompson<br/>Sampling as the primary<br/>strategy — strong empirical<br/>performance and a principled<br/>Bayesian foundation"] -.-> D2
```

---

## 6. Thompson Sampling — Detailed Mechanics

```mermaid
flowchart TB
    A["Each arm's success rate is<br/>modeled as a Beta distribution<br/>Beta(successes+1, failures+1)"] --> B["Arm A: shown 100 times,<br/>15 successes →<br/>Beta(16, 86) — fairly<br/>confident, narrow distribution"]
    A --> C["Arm B: shown 5 times,<br/>2 successes →<br/>Beta(3, 4) — very uncertain,<br/>WIDE distribution"]

    B & C --> D["On each ranking decision:<br/>SAMPLE a random value from<br/>EACH arm's current distribution"]
    D --> E["Arm A sample: 0.16<br/>(close to its mean, narrow<br/>distribution → sample close<br/>to the estimate)"]
    D --> F["Arm B sample: 0.55<br/>(could be anywhere due to<br/>high uncertainty — this time<br/>happened to sample high)"]

    E & F --> G["Select the arm with the<br/>HIGHEST sampled value —<br/>here, Arm B wins this round,<br/>despite having a LOWER<br/>average success rate,<br/>specifically BECAUSE its<br/>high uncertainty gave it<br/>a chance to be sampled high"]

    H["Over many rounds, arms<br/>with genuinely poor<br/>performance get sampled<br/>high less and less often<br/>as their distributions<br/>narrow with more data —<br/>naturally converging toward<br/>exploiting the true best<br/>arm while never fully<br/>abandoning exploration"] -.-> G
```

---

## 7. Real-Time Decision Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant User as User
    participant Bandit as Bandit Decision Service
    participant Stats as Arm Statistics Store

    User->>Bandit: Request content<br/>(e.g., which article to show)

    Bandit->>Stats: Fetch current distribution<br/>parameters for ALL candidate arms
    Stats-->>Bandit: {arm_1: Beta(16,86),<br/>arm_2: Beta(3,4),<br/>arm_3: Beta(50,50), ...}

    Bandit->>Bandit: Sample a value from<br/>EACH arm's distribution

    Bandit->>Bandit: Select arm with<br/>HIGHEST sampled value

    Bandit-->>User: Show selected content<br/>(the "arm" chosen)

    Bandit->>Bandit: Record impression:<br/>{impression_id, arm_id,<br/>shown_at} — outcome<br/>unknown yet, pending
```

---

## 8. Feedback Incorporation Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant User as User
    participant FeedbackCollector as Feedback Collector
    participant K as Kafka
    participant Updater as Statistics Updater
    participant Stats as Arm Statistics Store

    User->>FeedbackCollector: User clicks (or doesn't,<br/>within a timeout window)
    FeedbackCollector->>K: Emit outcome event<br/>{impression_id, arm_id,<br/>success: true/false}

    K->>Updater: Consume event

    Updater->>Stats: Update arm's distribution:<br/>IF success: successes += 1<br/>ELSE: failures += 1

    Note over Stats: The arm's Beta distribution<br/>parameters shift slightly with<br/>every single observation —<br/>this is what makes it TRUE<br/>online learning, not periodic<br/>batch retraining
```

**Why this is fundamentally different from the batch-retrained ranking model in the News Feed Ranking design:** There, model weights update through periodic (e.g., daily) training runs on accumulated historical data. Here, EVERY SINGLE feedback event immediately and incrementally updates the relevant arm's statistics — the system's "knowledge" evolves continuously throughout the day, incorporating information from minutes ago into the very next ranking decision.

---

## 9. Contextual Bandits (Personalization Layer)

```mermaid
flowchart TB
    A["Basic bandit: ONE global<br/>success rate estimate per arm,<br/>same for every user"] --> B["Limitation: ignores that<br/>different USERS likely have<br/>genuinely different<br/>preferences — Arm A might<br/>be great for user segment 1<br/>but poor for segment 2"]

    C["Contextual bandit: success<br/>rate estimate is a FUNCTION<br/>of the user's CONTEXT<br/>(demographics, past behavior,<br/>session signals), not a<br/>single global number"] --> D["Instead of Beta(successes,<br/>failures) per arm, maintain<br/>a lightweight model (e.g.,<br/>logistic regression) predicting<br/>success probability GIVEN<br/>both the arm AND the<br/>user's context features"]

    D --> E["This transforms the bandit<br/>from a one-size-fits-all<br/>ranking into a genuinely<br/>PERSONALIZED one — the same<br/>underlying exploration/<br/>exploitation math, but<br/>applied per-context rather<br/>than globally"]
```

---

## 10. Cold-Start Handling for New Arms

```mermaid
flowchart TB
    A["New content/option added<br/>(e.g., a newly published<br/>article) with ZERO<br/>impression history"] --> B["Initialize with a NEUTRAL,<br/>WIDE prior distribution:<br/>Beta(1, 1) — uniform,<br/>maximally uncertain"]

    B --> C["Because this distribution<br/>is so wide/uncertain, Thompson<br/>Sampling will NATURALLY tend<br/>to give this new arm<br/>reasonable exploration<br/>opportunities early on —<br/>no special-case logic needed,<br/>the algorithm's inherent<br/>uncertainty-seeking behavior<br/>handles cold-start<br/>automatically"]

    D["This is a notable advantage<br/>of the Thompson Sampling/<br/>Bayesian approach — cold-start<br/>is handled elegantly as a<br/>natural CONSEQUENCE of the<br/>algorithm, rather than<br/>requiring separate, bolted-on<br/>special-case handling"] -.-> C
```

---

## 11. Component Responsibilities Summary

```mermaid
mindmap
  root((Online Learning Ranking HLD))
    Bandit Decision Service
      Thompson Sampling selection
      Real-time, per-request decisions
    Arm Statistics Store
      Continuously updated distributions
      Low-latency read path
    Feedback Collector
      Captures click/conversion outcomes
      Timeout handling for non-events
    Statistics Updater
      Incremental distribution updates
      True online learning, not batch
    Contextual Layer
      Per-user-segment personalization
      Lightweight predictive model per arm
```

---

## 12. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Core algorithm | Thompson Sampling | Naturally balances exploration/exploitation based on genuine statistical uncertainty, strong empirical performance, elegant cold-start handling |
| Learning cadence | Continuous, per-feedback-event updates | This IS the defining property of online learning — distinct from the periodic batch retraining used in the broader News Feed Ranking design |
| Personalization | Contextual bandits (context-conditioned success estimates) | A single global estimate per arm ignores genuine differences in user preference across segments |
| Cold-start | Wide, neutral prior distributions | Handled naturally by the algorithm's inherent uncertainty-seeking behavior, without requiring special-case bootstrapping logic |
| Uncertainty representation | Full probability distributions, not point estimates | Point estimates alone can't distinguish "confidently mediocre" from "uncertain and possibly great" — the distribution's WIDTH is itself essential information |

---

## 13. Bottlenecks & Scaling Considerations

- **Arm statistics store as a critical low-latency dependency** — every single ranking decision requires reading current distribution parameters; must be extremely low-latency and highly available, similar criticality to the online feature store in the Feature Store design.
- **Update contention for extremely popular arms** — a highly-trafficked arm receiving thousands of simultaneous feedback events creates write contention on its statistics; may require the same approximate/batched-update techniques discussed in the Distributed Cache and CRDT Counter designs, rather than perfectly synchronous updates for every single event.
- **Delayed feedback handling** — some outcomes (e.g., a purchase conversion, as opposed to an immediate click) may only be observable minutes or hours after the impression; the system needs clear policy for how long to wait before considering an impression a "failure" by default, and how to retroactively correct statistics if a delayed success arrives after that window.
- **Contextual model complexity vs latency** — richer contextual models (more features, more sophisticated per-context prediction) improve personalization quality but increase per-decision computation cost; this is the same multi-stage funnel tradeoff explored in the general Recommendation System and News Feed Ranking designs.
- **Non-stationarity (arms' true performance changing over time)** — a piece of content's genuine appeal can drift (news becomes stale, trends change); pure cumulative statistics eventually become slow to adapt to such drift, often requiring a "decay" or "sliding window" adjustment so older observations count less than recent ones.
- **Arm explosion at scale** — with potentially millions of distinct arms (e.g., every individual piece of content on a large platform), maintaining and querying per-arm statistics at this scale requires the same sharding/partitioning considerations as any large-scale key-value system, plus the specific challenge that popular arms need low-latency access while millions of rarely-shown arms need efficient, cheap storage.
- **Evaluation and safety** — deploying a live-learning system means mistakes correct themselves over time, but a poorly-tuned exploration rate or a bug in the reward signal computation can cause real, immediate business harm before self-correction kicks in; this argues for careful monitoring dashboards and possibly bounded exploration budgets (a maximum "risk" any single arm can be shown) as a safety net, similar in spirit to the automated rollback mechanisms in the ML Model Serving design.
