# Design a Content Moderation System Combining ML Models and Human Review — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Automatically screen user-generated content (text, images, video) for policy violations at upload/post time
- Route genuinely ambiguous cases to human moderators for judgment
- Support appeals — a user can contest a moderation decision
- Support different severity tiers with different handling speed/rigor (e.g., child safety content vs. mild spam)

### Non-Functional Requirements
- **Low latency for the common case:** The vast majority of content (which is fine) shouldn't experience meaningful posting delay
- **High recall for severe violations:** Missing genuinely dangerous content (e.g., illegal content) is a far more serious failure than the reverse
- **Human reviewer wellbeing:** Reviewers are repeatedly exposed to disturbing content — the system's design has direct human welfare implications
- **Consistency:** Similar content should receive similar moderation decisions, both across different automated model calls and across different human reviewers

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Content pieces posted/sec (platform-wide) | Tens of thousands |
| ML-flagged-for-review rate | Single-digit percentage typically |
| Human reviewer capacity | Hundreds to thousands of items/reviewer/day, varies by content type |
| Severe-violation response target | Minutes, not hours (e.g., imminent harm content) |

---

## 2. The Core Philosophy — Tiered Response by Confidence and Severity

```mermaid
flowchart TB
    A["Content submitted"] --> B["ML Classification:<br/>confidence score +<br/>severity category"]

    B --> C{"Confidence AND<br/>Severity combined"}

    C --> D["HIGH confidence,<br/>SEVERE violation<br/>(e.g., known CSAM hash match,<br/>explicit violent threat)"]
    D --> D1["IMMEDIATE automated action —<br/>block/remove without waiting<br/>for human review; separately<br/>escalate to specialized teams<br/>for severe categories"]

    C --> E["HIGH confidence,<br/>MINOR violation<br/>(e.g., obvious spam pattern)"]
    E --> E1["Automated action<br/>(remove/flag), LOW priority<br/>for any human audit"]

    C --> F["LOW/MEDIUM confidence,<br/>ANY severity"]
    F --> F1["Route to HUMAN REVIEW QUEUE —<br/>the model isn't confident<br/>enough to act alone,<br/>prioritized by severity"]

    C --> G["Confidently CLEAN content<br/>(vast majority)"]
    G --> G1["Publish immediately,<br/>no review needed"]
```

**Why this two-dimensional (confidence × severity) approach matters:** A naive single-threshold system ("flag if score > X") fails to capture the crucial distinction between "the model is UNCERTAIN" and "the content is SEVERE" — these require genuinely different handling. High-confidence severe violations need instant automated action (waiting for a human reviewer risks real harm), while low-confidence content of ANY severity needs human judgment specifically BECAUSE the model doesn't know enough to decide alone.

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    User["User Posts Content"]

    subgraph Ingestion["Ingestion & Screening"]
        UploadSvc["Upload Service"]
        MLScreening["ML Classification Pipeline<br/>(text/image/video models)"]
        HashMatcher["Known-Bad Hash Matcher<br/>(e.g., CSAM hash databases)"]
    end

    subgraph Decision["Decision Layer"]
        PolicyEngine["Policy Engine<br/>(confidence + severity rules)"]
    end

    subgraph HumanReview["Human Review System"]
        Queue["Priority Review Queue"]
        ReviewerUI["Reviewer Interface"]
        WellbeingControls["Reviewer Wellbeing<br/>Controls"]
    end

    subgraph Feedback["Feedback & Appeals"]
        AppealSvc["Appeals Service"]
        Training["Model Retraining Pipeline"]
    end

    User --> UploadSvc
    UploadSvc --> MLScreening
    UploadSvc --> HashMatcher
    MLScreening --> PolicyEngine
    HashMatcher --> PolicyEngine

    PolicyEngine -->|"immediate action"| UploadSvc
    PolicyEngine -->|"ambiguous"| Queue
    Queue --> ReviewerUI
    ReviewerUI --> WellbeingControls
    ReviewerUI -->|"decision"| UploadSvc

    ReviewerUI --> Training
    User --> AppealSvc
    AppealSvc --> Queue
```

---

## 4. Data Model

```mermaid
erDiagram
    CONTENT_ITEM {
        string content_id PK
        string user_id
        string content_type "text/image/video"
        string status "published/removed/pending_review"
        float ml_confidence_score
        string severity_category
        timestamp posted_at
    }
    REVIEW_CASE {
        string case_id PK
        string content_id FK
        string priority "critical/high/normal"
        string status "queued/in_review/resolved"
        string reviewer_id
        string decision
        timestamp assigned_at
        timestamp resolved_at
    }
    APPEAL {
        string appeal_id PK
        string content_id FK
        string user_id
        string status "pending/upheld/overturned"
        string original_decision
        string appeal_reviewer_id
    }
```

---

## 5. Automated Screening Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant User as User
    participant Upload as Upload Service
    participant HashMatch as Hash Matcher
    participant ML as ML Classification
    participant Policy as Policy Engine
    participant Queue as Review Queue

    User->>Upload: Post content

    par Parallel screening checks
        Upload->>HashMatch: Check against known-bad<br/>content hash databases<br/>(fast, deterministic —<br/>exact/near-exact matches<br/>to previously identified<br/>violating content)
        HashMatch-->>Upload: Match result
    and
        Upload->>ML: Run classification models<br/>(text toxicity, image/video<br/>content classifiers)
        ML-->>Upload: {confidence_score,<br/>category, severity}
    end

    Upload->>Policy: Combine signals

    alt Hash match found (known-bad content)
        Policy-->>Upload: IMMEDIATE BLOCK<br/>+ escalate to specialized team
    else High-confidence ML violation
        Policy-->>Upload: Automated removal
    else Low/medium confidence
        Policy->>Queue: Route to human review<br/>(prioritized by severity)
        Policy-->>Upload: Content PENDING —<br/>may publish provisionally<br/>or hold, depending on<br/>severity category policy
    else Confidently clean
        Policy-->>Upload: Publish immediately
    end
```

**Why provisional publishing is sometimes the right choice for pending review:** For LOW-severity ambiguous content (e.g., borderline spam), holding every such post until human review completes would create unacceptable latency for a large volume of ultimately-fine content; provisional publishing (visible immediately, subject to removal if review finds a violation) balances user experience against moderation thoroughness — but this policy must NEVER apply to high-severity categories, where the precautionary hold is worth the latency cost.

---

## 6. Human Review Queue Prioritization

```mermaid
flowchart TB
    A["Review Queue"] --> B{"Prioritization Factors"}

    B --> C["Severity category<br/>(child safety, imminent<br/>violence > general policy<br/>violations > minor spam)"]
    B --> D["ML confidence<br/>(closer to the decision<br/>boundary = higher priority,<br/>since it's genuinely more<br/>ambiguous)"]
    B --> E["Virality/reach<br/>(content already getting<br/>significant engagement<br/>reviewed sooner — potential<br/>harm scales with exposure)"]
    B --> F["Reporter volume<br/>(content reported by many<br/>users prioritized over<br/>single-report items)"]

    C & D & E & F --> G["Composite priority score<br/>determines queue position —<br/>NOT simple FIFO"]
```

---

## 7. Human Review Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Queue as Review Queue
    participant Reviewer as Human Reviewer
    participant UI as Reviewer Interface
    participant Wellbeing as Wellbeing Controls
    participant Upload as Upload Service
    participant Training as Model Retraining

    Queue->>Reviewer: Assign next case<br/>(by priority)
    Reviewer->>UI: Open case for review

    UI->>Wellbeing: Apply content presentation<br/>safeguards (e.g., blur-by-default<br/>for graphic content, monitor<br/>exposure volume/session time)

    Reviewer->>UI: Review content against<br/>policy guidelines
    Reviewer->>UI: Submit decision:<br/>{violation_confirmed: true/false,<br/>category, action}

    UI->>Upload: Apply decision<br/>(publish/remove/restrict)
    UI->>Training: Record labeled outcome<br/>(ground truth for<br/>future model improvement —<br/>same feedback-loop pattern<br/>as the Fraud Detection<br/>and Bot Detection designs)
```

---

## 8. Reviewer Wellbeing Controls (A Distinctive, Human-Centric Requirement)

```mermaid
flowchart TB
    A["Reviewers are repeatedly<br/>exposed to disturbing content<br/>as an inherent part of the<br/>role — this has genuine,<br/>documented mental health<br/>implications"] --> B{"System-Level Mitigations"}

    B --> C["Content presentation controls:<br/>blur/grayscale by default,<br/>reviewer opts INTO full<br/>visibility only when needed"]
    B --> D["Exposure volume limits:<br/>caps on how much severe<br/>content a single reviewer<br/>handles per session/day,<br/>enforced by the queue<br/>assignment system itself"]
    B --> E["Category rotation:<br/>preventing extended,<br/>uninterrupted exposure to<br/>the SAME disturbing content<br/>category"]
    B --> F["Mandatory breaks and<br/>access to mental health<br/>support resources, integrated<br/>into the workflow itself"]

    G["This is a DELIBERATE design<br/>requirement, not an<br/>afterthought — a system that<br/>maximizes reviewer THROUGHPUT<br/>at the expense of reviewer<br/>WELLBEING is both an ethical<br/>failure and, practically,<br/>leads to reviewer burnout/<br/>turnover that ultimately<br/>degrades moderation quality<br/>anyway"] -.-> D
```

---

## 9. Appeals Process — Detailed Sequence

```mermaid
sequenceDiagram
    participant User as User (content removed)
    participant Appeal as Appeals Service
    participant Queue as Appeals Review Queue
    participant Reviewer as Different Reviewer<br/>(not original decision-maker)
    participant Upload as Upload Service

    User->>Appeal: Submit appeal<br/>{content_id, reasoning}

    Appeal->>Queue: Create appeal case<br/>(routed to a DIFFERENT<br/>reviewer than the original<br/>decision — avoids anchoring<br/>bias from re-reviewing<br/>one's own decision)

    Reviewer->>Queue: Review original content<br/>+ original decision +<br/>user's appeal reasoning
    Reviewer->>Queue: Decision: UPHOLD or<br/>OVERTURN original decision

    alt Overturned
        Queue->>Upload: Restore content
        Queue->>Appeal: Notify user: appeal successful
    else Upheld
        Queue->>Appeal: Notify user: appeal denied<br/>(with policy explanation)
    end

    Note over Queue: Appeal outcomes ALSO feed<br/>back into quality monitoring —<br/>a reviewer/model with a<br/>consistently high OVERTURN<br/>rate signals a calibration<br/>issue worth investigating
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Content Moderation HLD))
    ML Classification Pipeline
      Text/image/video models
      Confidence and severity scoring
    Hash Matcher
      Known-bad content detection
      Fast, deterministic matches
    Policy Engine
      Confidence times severity routing
      Determines automated vs human path
    Review Queue
      Multi-factor prioritization
      Not simple FIFO
    Reviewer Interface
      Decision capture
      Wellbeing safeguards integrated
    Appeals Service
      Independent re-review
      Feeds quality monitoring
    Retraining Pipeline
      Human decisions as ground truth
      Continuous model improvement
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Routing logic | Two-dimensional: confidence × severity | Captures the crucial distinction between model uncertainty and content danger — these require fundamentally different handling, not a single threshold |
| Severe-content handling | Immediate automated action, no human-review delay | For genuinely dangerous content, waiting for human review risks real harm; automated action for high-confidence severe cases is the correct precautionary default |
| Ambiguous content handling | Human review, prioritized by composite severity/confidence/reach score | The model's own uncertainty is precisely the signal that human judgment is needed, not a reason to guess |
| Reviewer wellbeing | Explicit system-level controls (blur, exposure limits, rotation) | A deliberate, first-class design requirement — not just an ethical consideration but a practical necessity to prevent burnout-driven quality degradation |
| Appeals | Independent reviewer, separate from original decision-maker | Avoids anchoring bias; also serves as an ongoing quality-monitoring signal via overturn rates |
| Provisional publishing | Applied only to low-severity ambiguous content | Balances user experience against moderation thoroughness, with severity-based limits preventing this from ever applying to genuinely dangerous content categories |

---

## 12. Bottlenecks & Scaling Considerations

- **Human review capacity as a hard constraint** — unlike most systems where "add more compute" solves scaling, human reviewer capacity scales with HIRING and TRAINING, which is slow and has real limits; the system must be designed to minimize what genuinely NEEDS human review (via increasingly accurate ML models) rather than assuming review capacity can simply scale with content volume.
- **Model accuracy directly determines both false-negative harm and reviewer workload** — a model with poor precision sends too much genuinely-fine content to the review queue (wasting scarce reviewer capacity and adding unnecessary friction for legitimate users); poor recall lets genuine violations through — this dual cost makes ongoing model quality investment a direct lever on both safety AND operational cost.
- **Adversarial content evolution** — similar to fraud and bot detection, bad actors actively adapt content to evade automated detection (e.g., subtle text obfuscation, image manipulation); this requires the same continuous retraining discipline as those other adversarial systems.
- **Cross-cultural and multi-language consistency** — content policy application must remain consistent across vastly different languages, cultural contexts, and content norms; this typically requires region/language-specific model tuning and reviewer expertise, adding significant complexity beyond a single global model/policy.
- **Consistency measurement across reviewers** — with many human reviewers each making judgment calls, measuring and maintaining INTER-REVIEWER CONSISTENCY (similar content receiving similar decisions regardless of who reviews it) requires ongoing calibration exercises and quality auditing, not just individual reviewer accuracy metrics in isolation.
- **Latency vs thoroughness for viral/high-reach content** — content rapidly gaining engagement needs FASTER review specifically because potential harm scales with exposure, but faster review can mean LESS thorough review — this tension requires careful queue-prioritization design (as noted in Section 6) rather than a simple speed-vs-quality tradeoff applied uniformly.
- **Downstream legal and regulatory obligations** — certain content categories (e.g., specific illegal content types) carry legal reporting obligations to authorities once identified; the system's escalation pathways for these categories need to integrate with legal/compliance processes, not just internal moderation workflows — connecting to considerations similar to those in the GDPR Deletion System design's handling of legally-mandated processes.
