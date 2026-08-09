# Design a Bot Detection / CAPTCHA-Alternative System at Scale — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Distinguish genuine human traffic from automated bot traffic across web/app requests
- Provide a graduated response — not every suspicious request needs a hard block; some warrant a lightweight challenge
- Support multiple detection signals: behavioral, device/browser fingerprinting, network-level indicators
- Minimize friction for legitimate users while effectively blocking automated abuse

### Non-Functional Requirements
- **Low false-positive rate:** Blocking genuine human users is a severe UX and business cost — must be weighed as seriously as missing actual bots
- **Low latency:** Detection must add minimal overhead to normal request processing, since it runs on a huge fraction of platform traffic
- **Adversarial resilience:** Like fraud detection, this is an active arms race — sophisticated bots specifically try to evade detection
- **Scale:** Must evaluate essentially every incoming request across the platform, not just a sampled subset

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Requests/sec evaluated (platform-wide) | Hundreds of thousands to millions |
| Bot traffic (industry typical estimate) | 20-40%+ of raw internet traffic, varies hugely by endpoint type |
| Detection latency budget | Single-digit to low tens of milliseconds |
| Challenge friction cost | Must be weighed against genuine security value — every CAPTCHA shown has a real conversion/UX cost |

---

## 2. The Core Philosophy — Graduated Response, Not Binary Block/Allow

```mermaid
flowchart TB
    A["Traditional CAPTCHA approach:<br/>show a challenge to<br/>EVERYONE, or nobody —<br/>a blunt, binary, user-hostile<br/>instrument"] --> A1["Problem: legitimate users<br/>hate CAPTCHAs (friction,<br/>accessibility issues,<br/>abandonment), while<br/>sophisticated bots increasingly<br/>solve them anyway (via ML<br/>solving services or<br/>human click-farms)"]

    B["Modern approach: continuous<br/>RISK SCORING per request,<br/>with a GRADUATED response<br/>based on that score"] --> C["Low risk → allow silently,<br/>zero user-visible friction"]
    B --> D["Medium risk → lightweight<br/>invisible challenge<br/>(e.g., background behavioral<br/>check, no user interaction)"]
    B --> E["High risk → visible challenge<br/>(CAPTCHA) or outright block"]

    F["This shifts CAPTCHA from<br/>being the PRIMARY detection<br/>mechanism to being a LAST<br/>RESORT escalation, reserved<br/>only for genuinely ambiguous<br/>cases"] -.-> D
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    Client["Client Request<br/>(web/mobile)"]

    subgraph Collection["Signal Collection Layer"]
        ClientSDK["Client-Side SDK<br/>(behavioral + fingerprint<br/>data collection)"]
        RequestMeta["Request Metadata<br/>(IP, headers, timing)"]
    end

    subgraph Scoring["Risk Scoring Layer"]
        RuleEngine["Rule Engine<br/>(known bad patterns)"]
        MLModel["ML Risk Model"]
        ReputationDB[("IP/Device Reputation Store")]
    end

    subgraph Response["Response Layer"]
        DecisionEngine["Decision Engine"]
        ChallengeSvc["Challenge Service<br/>(invisible + visible challenges)"]
    end

    Client --> ClientSDK
    Client --> RequestMeta

    ClientSDK --> RuleEngine
    RequestMeta --> RuleEngine
    RuleEngine --> ReputationDB
    RuleEngine --> MLModel
    MLModel --> DecisionEngine

    DecisionEngine -->|"low risk"| AllowedTraffic["Request proceeds normally"]
    DecisionEngine -->|"medium risk"| ChallengeSvc
    DecisionEngine -->|"high risk"| BlockedTraffic["Request blocked"]
    ChallengeSvc -->|"passes"| AllowedTraffic
    ChallengeSvc -->|"fails"| BlockedTraffic
```

**Key idea:** This shares significant architectural DNA with the Fraud Detection design — a fast, synchronous, latency-critical scoring decision combining rules, reputation data, and ML — but the RESPONSE mechanism is fundamentally different: fraud detection has only approve/decline/review, while bot detection has an intermediate "challenge" option that lets genuinely ambiguous traffic self-resolve (a human easily passes a lightweight challenge; a simple bot typically doesn't).

---

## 4. Signal Categories

```mermaid
mindmap
  root((Bot Detection Signals))
    Behavioral Signals
      Mouse movement patterns
      Typing cadence/rhythm
      Scroll behavior
      Time-to-interact with page
    Device/Browser Fingerprinting
      Canvas/WebGL fingerprint
      Installed fonts/plugins
      Screen resolution consistency
      Browser automation flags
      (e.g., navigator.webdriver)
    Network-Level Signals
      IP reputation
      Data center vs residential IP
      Request rate from single IP
      TLS fingerprint
    Request Pattern Signals
      Navigation sequence
      (do humans browse this way?)
      Header consistency
      Request timing regularity
```

**Why NO single signal is sufficient alone:** Any individual signal can be spoofed by a sufficiently motivated attacker (residential proxies defeat IP reputation, headless browser tools can fake some fingerprints, replay tools can simulate mouse movement) — but SPOOFING ALL SIGNALS SIMULTANEOUSLY AND CONSISTENTLY is significantly harder and more expensive, which is precisely why combining many independent, cheap-to-collect signals into one composite score is the effective strategy, rather than betting everything on one supposedly bulletproof check.

---

## 5. Real-Time Scoring Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Client as Client
    participant SDK as Client SDK
    participant Gateway as API Gateway
    participant RuleEng as Rule Engine
    participant Rep as Reputation Store
    participant ML as ML Model
    participant Decision as Decision Engine

    Client->>SDK: Page load / action taken
    SDK->>SDK: Collect behavioral +<br/>fingerprint signals<br/>(passively, in background)

    Client->>Gateway: Request<br/>(includes SDK-collected<br/>signal payload)

    Gateway->>RuleEng: Evaluate fast rules<br/>(< 5ms)
    RuleEng->>Rep: Check IP/device reputation<br/>(cached, low-latency lookup)
    Rep-->>RuleEng: Reputation score

    alt Known-bad IP/device (high confidence)
        RuleEng-->>Gateway: IMMEDIATE BLOCK
    else Ambiguous
        RuleEng->>ML: Score full signal set
        ML-->>RuleEng: Bot probability score
        RuleEng->>Decision: Combine signals
        Decision->>Decision: Apply graduated<br/>response thresholds
        Decision-->>Gateway: ALLOW / CHALLENGE / BLOCK
    end
```

---

## 6. Invisible Challenge Flow (Medium Risk)

```mermaid
sequenceDiagram
    participant Client as Client
    participant Challenge as Challenge Service
    participant Decision as Decision Engine

    Decision->>Challenge: Medium risk score —<br/>issue invisible challenge

    Challenge->>Client: Deploy lightweight<br/>background check<br/>(e.g., proof-of-work<br/>computation, or passive<br/>behavioral observation<br/>over next few interactions —<br/>NO visible UI shown to user)

    Note over Client: Genuine human continues<br/>normal browsing — the<br/>challenge resolves<br/>transparently in the background

    Client->>Challenge: Challenge response<br/>(computation result, or<br/>accumulated behavioral<br/>confidence signal)

    Challenge->>Challenge: Evaluate: consistent with<br/>genuine human/legitimate<br/>client behavior?

    alt Passes
        Challenge-->>Client: Proceed normally<br/>(user never knew a<br/>challenge occurred)
    else Fails
        Challenge->>Decision: Escalate to VISIBLE<br/>challenge (CAPTCHA) as<br/>a final tier
    end
```

**Why invisible challenges are preferred over jumping straight to CAPTCHA:** A proof-of-work style computational challenge, for instance, is trivially fast for a normal user's device but adds meaningful COST when performed at the scale a bot operator needs (thousands/millions of requests) — this raises the bot's operating cost without ever showing the legitimate user any interruption at all, making it a strictly better tradeoff than CAPTCHA for the medium-risk tier.

---

## 7. Visible Challenge (CAPTCHA) as Last Resort

```mermaid
flowchart TB
    A["Request reaches HIGH<br/>risk tier, invisible<br/>challenge also failed<br/>or wasn't conclusive"] --> B["Show visible CAPTCHA<br/>(image selection, puzzle,<br/>etc.)"]

    B --> C{"Challenge result"}
    C -- "Solved correctly" --> D["Allow — but note this<br/>doesn't necessarily mean<br/>'definitely human,' since<br/>sophisticated bots CAN<br/>solve CAPTCHAs (via ML<br/>or human solving services)<br/>— treat as ONE signal among<br/>many, feeding back into<br/>future reputation scoring"]
    C -- "Failed / abandoned" --> E["Block, or require<br/>additional verification<br/>(e.g., email/SMS confirmation)"]

    F["Because CAPTCHA is reserved<br/>for only the smallest,<br/>highest-risk fraction of<br/>traffic — not shown to<br/>everyone — the overall<br/>platform-wide UX friction<br/>cost is dramatically lower<br/>than a blanket CAPTCHA<br/>policy, while remaining an<br/>effective final gate for<br/>genuinely suspicious traffic"] -.-> D
```

---

## 8. Reputation Feedback Loop

```mermaid
sequenceDiagram
    participant Decision as Decision Engine
    participant Outcome as Outcome Tracker
    participant K as Kafka
    participant RepUpdater as Reputation Update Worker
    participant Rep as Reputation Store
    participant Training as ML Retraining Pipeline

    Decision->>Outcome: Record outcome<br/>{ip, device_fingerprint,<br/>decision, challenge_result}

    Outcome->>K: Emit outcome event

    K->>RepUpdater: Consume event
    RepUpdater->>Rep: Update reputation score<br/>(e.g., IP that repeatedly<br/>fails challenges accumulates<br/>a worsening reputation,<br/>independent of any single<br/>request's score)

    K->>Training: Feed labeled outcomes<br/>into model retraining<br/>(same continuous-adaptation<br/>pattern as the Fraud<br/>Detection design)
```

---

## 9. Handling Legitimate Automated Traffic (The Nuance This System Must Get Right)

```mermaid
flowchart TB
    A["NOT all automated traffic<br/>is malicious — this is a<br/>critical distinction bot<br/>detection must handle<br/>carefully"] --> B["Legitimate bots:<br/>search engine crawlers,<br/>monitoring/uptime services,<br/>legitimate API integrations,<br/>accessibility tools"]

    B --> C["Allowlisting strategy:<br/>verified, known-legitimate<br/>bots (e.g., Googlebot,<br/>verified via reverse-DNS<br/>+ IP range confirmation)<br/>bypass the standard<br/>risk-scoring pipeline<br/>entirely"]

    D["Malicious/abusive bots:<br/>credential stuffing,<br/>scraping, fake account<br/>creation, inventory hoarding"] --> E["This is what the risk-scoring<br/>system is actually designed<br/>to catch — the goal is<br/>precision in targeting THIS<br/>category, not blanket<br/>automation-hostility"]

    F["Getting this distinction<br/>wrong (blocking legitimate<br/>search engine crawlers, for<br/>instance) has REAL business<br/>consequences — e.g., harming<br/>SEO/discoverability — making<br/>this as important a design<br/>consideration as catching<br/>actual abuse"] -.-> C
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Bot Detection HLD))
    Client SDK
      Passive signal collection
      Behavioral and fingerprint data
    Rule Engine
      Fast known-pattern checks
      High-confidence immediate blocks
    Reputation Store
      IP and device history
      Continuously updated
    ML Risk Model
      Composite signal scoring
      Adapts to evolving patterns
    Decision Engine
      Graduated response thresholds
      Allow, challenge, or block
    Challenge Service
      Invisible then visible tiers
      Cost-imposing, low-friction-first
    Allowlist
      Verified legitimate bots
      Bypasses standard scoring
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Response model | Graduated (allow/invisible-challenge/CAPTCHA/block), not binary | Minimizes friction for the vast legitimate-user majority while reserving maximum friction for genuinely high-risk traffic only |
| Detection approach | Combined multi-signal scoring, no single signal trusted alone | Any individual signal is spoofable; consistently spoofing ALL signals simultaneously is a much higher bar for attackers |
| Challenge escalation order | Invisible (proof-of-work/behavioral) before visible (CAPTCHA) | Imposes real cost on bot operators without any UX friction for legitimate users in the common case |
| Legitimate bot handling | Explicit allowlisting for verified crawlers/integrations | Avoids the real business cost of inadvertently blocking beneficial automated traffic like search engine indexing |
| Continuous adaptation | Reputation feedback loop + ML retraining | Same adversarial-arms-race reality as fraud detection — static rules alone become stale as attackers adapt |
| CAPTCHA role | Last-resort tier, not primary detection mechanism | Reflects the reality that sophisticated bots increasingly solve CAPTCHAs; it's one signal among many, not a definitive human/bot proof |

---

## 12. Bottlenecks & Scaling Considerations

- **Signal collection overhead on client devices** — extensive behavioral/fingerprint collection can impact page load performance and battery life on client devices; needs careful engineering to remain lightweight and non-blocking, collected asynchronously rather than delaying the user-visible page render.
- **Reputation store as a high-traffic dependency** — since nearly every request triggers a reputation lookup, this store faces the same criticality and scaling requirements as the feature store in the Fraud Detection design — low latency, high availability, on the critical path of essentially all traffic.
- **Adversarial evolution requiring continuous investment** — like fraud detection, this is fundamentally an ongoing arms race, not a solved problem; sophisticated bot operators actively study and adapt to detection signals, meaning the ML model and rule set require continuous, dedicated investment rather than being a one-time build.
- **False positive cost at scale** — even a small false-positive rate (e.g., 0.1%) translates to a large absolute number of legitimate users experiencing unnecessary friction at high traffic volumes; requires ongoing monitoring of false-positive rate specifically, not just overall detection effectiveness, and mechanisms for affected users to report/appeal incorrect blocks.
- **Distributed/residential proxy networks** — sophisticated bot operations increasingly route traffic through large networks of residential IP addresses (making IP-reputation-based detection far less effective), pushing detection weight increasingly toward behavioral and fingerprinting signals rather than network-level signals alone.
- **Privacy and regulatory considerations** — extensive behavioral tracking and device fingerprinting for bot detection purposes intersects with privacy regulations (GDPR, CCPA); the signal collection approach needs to be designed with these constraints in mind from the start, not retrofitted later, potentially connecting to the GDPR Deletion System design for how this collected signal data itself must be handled.
- **Mobile app-specific considerations** — mobile environments have different available signals (device attestation APIs, app integrity checks) compared to web browsers, requiring a meaningfully different signal collection strategy per platform rather than a single unified approach across web and native app traffic.
