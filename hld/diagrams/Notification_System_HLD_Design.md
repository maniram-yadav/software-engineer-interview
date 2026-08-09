# Design a Notification System — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Send notifications across multiple channels: push (iOS/Android), email, SMS, in-app
- Support triggered notifications (event-driven, e.g., "someone liked your post") and scheduled/campaign notifications (e.g., marketing blasts)
- Per-user preferences (opt-out of channels/categories)
- Rate limiting (don't spam a user with 50 notifications in a minute)
- Retry on failure with backoff
- Delivery tracking (sent, delivered, opened)

### Non-Functional Requirements
- **Scale:** ~500M users, billions of notifications/day
- **Latency:** Triggered notifications should arrive within seconds
- **Reliability:** At-least-once delivery; no critical notification silently dropped
- **Throughput bursts:** A viral event or campaign can trigger millions of notifications in minutes
- **Provider abstraction:** Must work across many third-party providers (APNs, FCM, SendGrid, Twilio) with different rate limits/APIs

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Notifications/day | ~5B |
| Avg notifications/sec | ~60,000 |
| Peak notifications/sec (campaign blast) | ~500,000+ |
| Channels | Push, Email, SMS, In-app |
| Third-party provider rate limits | Vary widely (SMS often most constrained) |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    subgraph Producers["Notification Producers"]
        AppServices["App Services<br/>(likes, comments, orders, etc.)"]
        CampaignSvc["Campaign/Marketing Service"]
        Scheduler["Scheduled Job Trigger"]
    end

    Gateway["Notification API<br/>(ingestion endpoint)"]

    subgraph Core["Core Notification Pipeline"]
        PrefSvc["Preference Service<br/>(opt-in/out, channel priority)"]
        RateLimiter["Rate Limiter<br/>(per-user, per-channel)"]
        Dedup["Dedup/Batching Service<br/>(merge similar notifications)"]
        Router["Channel Router"]
    end

    subgraph Queues["Per-Channel Queues"]
        PushQueue["Push Queue"]
        EmailQueue["Email Queue"]
        SMSQueue["SMS Queue"]
        InAppQueue["In-App Queue"]
    end

    subgraph Workers["Channel Worker Fleets"]
        PushWorkers["Push Workers"]
        EmailWorkers["Email Workers"]
        SMSWorkers["SMS Workers"]
        InAppWorkers["In-App Workers"]
    end

    subgraph Providers["Third-Party Providers"]
        APNs["APNs (iOS)"]
        FCM["FCM (Android)"]
        SendGrid["SendGrid/SES (Email)"]
        Twilio["Twilio (SMS)"]
        WSGateway["WebSocket Gateway (In-app)"]
    end

    DeliveryDB[("Delivery Tracking DB")]

    AppServices --> Gateway
    CampaignSvc --> Gateway
    Scheduler --> Gateway

    Gateway --> PrefSvc --> RateLimiter --> Dedup --> Router

    Router --> PushQueue --> PushWorkers --> APNs
    PushWorkers --> FCM
    Router --> EmailQueue --> EmailWorkers --> SendGrid
    Router --> SMSQueue --> SMSWorkers --> Twilio
    Router --> InAppQueue --> InAppWorkers --> WSGateway

    PushWorkers --> DeliveryDB
    EmailWorkers --> DeliveryDB
    SMSWorkers --> DeliveryDB
    InAppWorkers --> DeliveryDB
```

**Key idea:** All notification requests flow through a common pipeline (preferences → rate limiting → dedup) before fanning out into **per-channel queues**. Each channel has its own worker fleet and its own retry/backoff logic, because each third-party provider has wildly different rate limits, latency, and failure modes — SMS providers throttle hard, push providers are more forgiving.

---

## 3. Data Model

```mermaid
erDiagram
    USER ||--o{ NOTIFICATION_PREFERENCE : has
    USER ||--o{ NOTIFICATION : receives
    USER ||--o{ DEVICE_TOKEN : registers
    NOTIFICATION ||--o{ DELIVERY_ATTEMPT : "tracked by"

    USER {
        string user_id PK
        string email
        string phone_number
    }
    NOTIFICATION_PREFERENCE {
        string user_id FK
        string category "marketing/transactional/social"
        string channel "push/email/sms/in-app"
        bool enabled
    }
    DEVICE_TOKEN {
        string token_id PK
        string user_id FK
        string platform "ios/android"
        string push_token
        timestamp registered_at
    }
    NOTIFICATION {
        string notification_id PK
        string user_id FK
        string category
        string title
        string body
        string status "queued/sent/delivered/failed/opened"
        timestamp created_at
    }
    DELIVERY_ATTEMPT {
        string attempt_id PK
        string notification_id FK
        string channel
        string provider
        string result
        int retry_count
        timestamp attempted_at
    }
```

---

## 4. Triggered Notification Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant App as App Service (e.g. Like Service)
    participant API as Notification API
    participant Pref as Preference Service
    participant RL as Rate Limiter
    participant Dedup as Dedup/Batch Service
    participant Router as Channel Router
    participant Q as Push Queue
    participant W as Push Worker
    participant FCM as FCM Provider
    participant DB as Delivery Tracking DB

    App->>API: POST /notify {user_id, event: "post_liked", data}
    API->>Pref: Check user's channel preferences
    Pref-->>API: User allows push, opted out of email

    API->>RL: Check rate limit for user
    alt Under limit
        RL-->>API: OK
    else Over limit
        RL-->>API: Reject/defer
        API->>DB: Log as suppressed
    end

    API->>Dedup: Check for similar pending notification<br/>(e.g., "3 people liked your post" batching)
    Dedup-->>API: Not a duplicate, proceed

    API->>Router: Route to enabled channels
    Router->>Q: Enqueue push notification

    Q->>W: Worker picks up job
    W->>DB: Fetch device tokens for user
    W->>FCM: Send push payload
    FCM-->>W: Delivery result
    W->>DB: Record delivery attempt + status
```

---

## 5. Notification Batching / Deduplication

```mermaid
flowchart TB
    A["Event: User B liked Post X"] --> B["Dedup Service:<br/>Check pending notifications<br/>for (user=post_owner, post=X)"]
    B --> C{"Existing pending<br/>notification for this post?"}
    C -- No --> D["Create new notification:<br/>'B liked your post'"]
    C -- Yes --> E["Merge into existing:<br/>'B and 4 others liked your post'"]
    D --> F["Hold briefly<br/>(e.g., 60s debounce window)"]
    E --> F
    F --> G["Send merged/final notification"]
```

*A short debounce window prevents notification spam when multiple events happen in quick succession (e.g., a post getting 10 likes in 30 seconds shouldn't trigger 10 separate push notifications).*

---

## 6. Rate Limiting Strategy

```mermaid
flowchart LR
    A["Incoming notification request"] --> B{"Per-user daily cap<br/>exceeded?"}
    B -- Yes --> C["Drop or defer<br/>(low-priority categories)"]
    B -- No --> D{"Per-channel provider<br/>rate limit near capacity?"}
    D -- Yes --> E["Queue with backpressure<br/>(delay, don't drop)"]
    D -- No --> F["Send immediately"]

    G["Priority tiers"] -.-> H["Critical (security alerts,<br/>OTP) — never rate limited"]
    G -.-> I["Transactional (order updates)<br/>— high priority"]
    G -.-> J["Social (likes, comments)<br/>— subject to batching/limits"]
    G -.-> K["Marketing — most aggressively<br/>throttled, respects quiet hours"]
```

---

## 7. Retry & Failure Handling

```mermaid
sequenceDiagram
    participant W as Channel Worker
    participant Provider as Third-Party Provider
    participant DB as Delivery DB
    participant DLQ as Dead Letter Queue

    W->>Provider: Send notification
    alt Success
        Provider-->>W: 200 OK
        W->>DB: status = DELIVERED
    else Transient failure (5xx, timeout)
        Provider-->>W: Error
        W->>W: Exponential backoff retry<br/>(1s, 2s, 4s, 8s...)
        W->>Provider: Retry attempt
        alt Retry succeeds
            Provider-->>W: 200 OK
            W->>DB: status = DELIVERED (after N retries)
        else Max retries exceeded
            W->>DLQ: Move to dead letter queue
            W->>DB: status = FAILED
        end
    else Permanent failure (invalid token, unsubscribed)
        Provider-->>W: 4xx (e.g., invalid device token)
        W->>DB: status = FAILED
        W->>DB: Mark device token as invalid<br/>(stop future attempts)
    end
```

---

## 8. Scheduled / Campaign Notification Flow (Bulk Fanout)

```mermaid
flowchart TB
    A["Marketing team schedules<br/>campaign for 10M users"] --> B["Campaign Service:<br/>Store campaign definition"]
    B --> C["Scheduler triggers at set time"]
    C --> D["Audience Resolver:<br/>Query segment (e.g., 'users in US, opted-in')"]
    D --> E["Batch Splitter:<br/>Chunk 10M users into<br/>manageable batches (e.g., 10K each)"]
    E --> F["Enqueue batches onto<br/>Channel Queues gradually<br/>(throttled fanout)"]
    F --> G["Channel Workers process<br/>at sustainable rate<br/>(respecting provider limits)"]
    G --> H["Delivery tracking +<br/>campaign analytics rollup"]
```

*Bulk campaigns are deliberately **throttled and staggered** rather than fired all at once — sending 10M notifications instantly would overwhelm both internal queues and third-party provider rate limits (especially SMS/email providers, which often cap sends/sec per account).*

---

## 9. Component Responsibilities Summary

```mermaid
mindmap
  root((Notification System HLD))
    Notification API
      Ingestion endpoint
      Validates + normalizes requests
    Preference Service
      Per-user channel opt-in/out
      Category-level controls
    Rate Limiter
      Per-user caps
      Per-provider throttling
      Priority-tier awareness
    Dedup/Batch Service
      Merge similar events
      Debounce windows
    Channel Router
      Fan out to per-channel queues
    Channel Workers
      Provider-specific integration
      Retry/backoff logic
      Token/contact validation
    Delivery Tracking DB
      Status per attempt
      Analytics (open rates, failures)
```

---

## 10. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Per-channel queues | Separate queue + worker fleet per channel | Each provider has distinct rate limits/latency; isolates failures (SMS outage shouldn't block push) |
| Delivery guarantee | At-least-once with idempotency keys | Losing a notification is worse than an occasional duplicate; client dedupes by notification_id |
| Batching/debounce | Short delay window before sending social notifications | Prevents notification spam from rapid-fire events (many likes in seconds) |
| Bulk campaign fanout | Throttled, staggered batch processing | Prevents overwhelming both internal infra and third-party provider rate limits |
| Priority tiers | Critical > Transactional > Social > Marketing | Ensures security-critical notifications (OTP, fraud alerts) are never delayed by rate limits |
| Failure handling | Distinguish transient vs permanent failures | Retries only make sense for transient errors; permanent failures (invalid token) should stop retrying immediately |

---

## 11. Bottlenecks & Scaling Considerations

- **Third-party provider rate limits** — the single biggest external constraint; workers must respect provider-specific quotas (e.g., Twilio SMS often limited to low hundreds/sec per number) via token-bucket throttling before calling out.
- **Campaign fanout spikes** — a 10M-user campaign firing at once would overwhelm queues; always stagger bulk sends and monitor provider quota consumption in real time.
- **Device token churn** — tokens expire/change frequently (app reinstalls, OS updates); need continuous cleanup of invalid tokens to avoid wasted send attempts and provider penalty flags.
- **Dedup window latency tradeoff** — longer debounce windows reduce notification spam but delay time-sensitive alerts; category-aware windows (short for DMs, longer for social likes) balance this.
- **Delivery tracking DB write volume** — billions of delivery attempts/day need a high-throughput, appendable store (wide-column DB) rather than a traditional relational DB.
- **Cross-channel consistency** — a user might get a push AND an email for the same event if preference logic has bugs; needs careful preference-service testing and possibly a "notification already delivered via X" suppression check across channels.
- **Retry storms** — if a provider goes down entirely, naive retries across millions of queued notifications can create a thundering herd on recovery; use circuit breakers to pause sending to a failing provider rather than retrying indefinitely.
