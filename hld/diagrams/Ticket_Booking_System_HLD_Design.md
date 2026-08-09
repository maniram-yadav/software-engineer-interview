# Design a Ticket Booking System (Ticketmaster-style) — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Browse events, view seat maps and availability
- Select specific seats and hold them temporarily during checkout
- Complete purchase (payment) to convert hold into confirmed booking
- Handle massive concurrent demand for popular events (on-sale moment)
- Support general admission (no seat selection) and reserved seating
- Cancellations/refunds release seats back to inventory

### Non-Functional Requirements
- **Correctness:** Never double-sell the same seat — this is the core invariant
- **Extreme burst scale:** A popular concert on-sale can see 1M+ users hit "buy" within seconds
- **Fairness:** Users who click first should generally get priority (or queue-based fairness)
- **Low seat-map latency:** Real-time seat availability must render quickly and accurately
- **Hold expiry:** Seats held during checkout but not purchased must release automatically

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Concurrent users at on-sale moment | ~1M+ for a major event |
| Seats per venue | ~20,000 (large stadium) |
| Seat hold requests/sec (peak burst) | ~100,000+/sec |
| Hold duration | 5-10 minutes typical |
| Read:Write ratio (browsing vs buying) | Very read-heavy outside on-sale spikes |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Client["Client<br/>(Web/Mobile)"]
    Gateway["API Gateway"]

    subgraph WaitingRoom["Admission Control"]
        VirtualQueue["Virtual Waiting Room<br/>(queue-based admission)"]
    end

    subgraph Core["Core Services"]
        EventSvc["Event/Venue Service"]
        SeatSvc["Seat Availability Service"]
        HoldSvc["Seat Hold Service"]
        CheckoutSvc["Checkout Orchestrator"]
        PaymentSvc["Payment Service"]
        BookingSvc["Booking Service"]
    end

    subgraph Storage["Storage Layer"]
        SeatMapCache[("Seat Map Cache<br/>(Redis - real-time state)")]
        SeatDB[("Seat/Booking DB<br/>(source of truth,<br/>strongly consistent)")]
        EventDB[("Event Metadata DB")]
        HoldStore[("Seat Holds<br/>(Redis, TTL-based)")]
    end

    Client --> Gateway
    Gateway --> VirtualQueue
    VirtualQueue -->|"Admitted in order"| EventSvc
    Gateway --> EventSvc --> EventDB
    Gateway --> SeatSvc --> SeatMapCache

    VirtualQueue --> HoldSvc
    HoldSvc --> HoldStore
    HoldSvc --> SeatDB

    HoldSvc --> CheckoutSvc
    CheckoutSvc --> PaymentSvc
    CheckoutSvc --> BookingSvc --> SeatDB
```

**Key idea:** The single hardest constraint is "never sell the same seat twice" under extreme concurrent load. This drives a **two-phase model**: (1) a short-lived, TTL-based *hold* on a seat while the user completes checkout, and (2) a *commit* that converts the hold into a permanent, strongly-consistent booking — combined with a **virtual waiting room** that controls how many users even reach the seat-selection step at once.

---

## 3. Data Model

```mermaid
erDiagram
    VENUE ||--o{ SEAT : has
    EVENT ||--o{ SHOWTIME : has
    SHOWTIME ||--o{ SEAT_STATUS : "tracks per-showtime"
    SEAT ||--o{ SEAT_STATUS : "has status per showtime"
    USER ||--o{ BOOKING : makes
    BOOKING ||--o{ BOOKING_SEAT : contains
    SEAT_STATUS ||--o| BOOKING_SEAT : "referenced by"

    VENUE {
        string venue_id PK
        string name
        json seat_map_layout
    }
    SEAT {
        string seat_id PK
        string venue_id FK
        string section
        string row
        string seat_number
    }
    EVENT {
        string event_id PK
        string venue_id FK
        string name
    }
    SHOWTIME {
        string showtime_id PK
        string event_id FK
        timestamp start_time
    }
    SEAT_STATUS {
        string showtime_id FK
        string seat_id FK
        string status "available/held/booked"
        string hold_id "nullable"
        timestamp hold_expires_at "nullable"
    }
    BOOKING {
        string booking_id PK
        string user_id FK
        string showtime_id FK
        string idempotency_key
        string status "pending/confirmed/cancelled"
        float total_amount
    }
    BOOKING_SEAT {
        string booking_id FK
        string seat_id FK
        float price
    }
```

---

## 4. Virtual Waiting Room Flow

```mermaid
sequenceDiagram
    participant U as User
    participant VQ as Virtual Queue Service
    participant QStore as Queue Store (Redis)
    participant SeatSvc as Seat Selection Service

    U->>VQ: Click "Buy Tickets" for hot event
    VQ->>QStore: Assign queue position + ticket token
    QStore-->>VQ: Position 45,231 of 200,000
    VQ-->>U: Show waiting room UI<br/>with position + est. wait

    loop Periodic status check
        U->>VQ: Poll status (with ticket token)
        VQ->>QStore: Check if admitted yet
        alt Not yet admitted
            QStore-->>VQ: Still waiting, position updated
            VQ-->>U: Updated position
        else Admitted (based on downstream capacity)
            QStore-->>VQ: Admitted! Session token issued
            VQ-->>U: Redirect to seat selection<br/>with time-limited session
        end
    end
```

**Key idea:** The waiting room is a deliberate **admission control valve** — it caps how many users can concurrently hit the seat-selection and hold services, throttled to match what the downstream system can actually handle without lock contention collapse. Being 45,000th in line is far better UX than getting through immediately and hitting constant "seat unavailable" errors from a stampede.

---

## 5. Seat Hold Flow — Preventing Double-Booking

```mermaid
sequenceDiagram
    participant U as User (admitted from queue)
    participant HoldSvc as Seat Hold Service
    participant Cache as Seat Map Cache (Redis)
    participant DB as Seat DB (source of truth)

    U->>HoldSvc: Select seats {seat_ids: [A12, A13]}

    HoldSvc->>DB: BEGIN TRANSACTION
    HoldSvc->>DB: SELECT status WHERE seat_id IN (A12,A13) FOR UPDATE
    DB-->>HoldSvc: Both status = 'available'

    alt All seats available
        HoldSvc->>DB: UPDATE seats SET status='held', hold_expires_at=now+8min
        HoldSvc->>DB: COMMIT
        HoldSvc->>Cache: Update seat map cache (reflect held status)
        HoldSvc-->>U: Seats held! You have 8 minutes to complete checkout
    else One or more seats already taken
        HoldSvc->>DB: ROLLBACK
        HoldSvc-->>U: "Seat A13 was just taken, please reselect"
    end
```

**Key design point:** Just like the inventory reservation problem, this uses `SELECT ... FOR UPDATE` (or an equivalent atomic conditional update) to prevent the race where two users simultaneously see seat A13 as "available" and both attempt to hold it. The database transaction, not the application logic, is the source of truth for the hold decision.

---

## 6. Seat Map Real-Time Updates (Read Path)

```mermaid
flowchart TB
    A["User views seat map"] --> B["Seat Availability Service"]
    B --> C["Read from Seat Map Cache<br/>(Redis - fast, slightly stale OK)"]
    C --> D["Render seat map:<br/>available/held/booked colors"]

    E["Seat status changes<br/>(held/booked/released)"] --> F["Write-through to Cache<br/>+ Publish update event"]
    F --> G["WebSocket push to<br/>users currently viewing this seat map"]
    G --> H["Client updates seat colors<br/>in real-time without refresh"]
```

*The seat map for browsing is read from a cache that's "close enough to real-time" (updated via write-through + pub/sub push) — but the actual **hold/commit decision** always goes through the strongly consistent database transaction. This separates the read-heavy browsing experience from the write-critical booking decision.*

---

## 7. Checkout & Booking Confirmation — Detailed Sequence

```mermaid
sequenceDiagram
    participant U as User
    participant CO as Checkout Orchestrator
    participant PS as Payment Service
    participant BS as Booking Service
    participant DB as Seat DB

    U->>CO: Complete checkout {held seats, payment info, idempotency_key}
    CO->>DB: Verify hold still valid (not expired)

    alt Hold expired
        DB-->>CO: Hold expired
        CO-->>U: "Your hold expired, please reselect seats"
    else Hold still valid
        CO->>PS: Charge payment
        alt Payment succeeds
            PS-->>CO: Confirmed
            CO->>BS: Create booking record
            BS->>DB: UPDATE seats SET status='booked' WHERE hold matches
            BS->>DB: INSERT booking + booking_seats
            BS-->>CO: Booking confirmed
            CO-->>U: Tickets confirmed! (booking_id)
        else Payment fails
            PS-->>CO: Declined
            CO->>DB: Release hold (status back to 'available')
            CO-->>U: "Payment failed, seats released"
        end
    end
```

---

## 8. Hold Expiry Sweep (Background Process)

```mermaid
flowchart LR
    A["Background sweeper<br/>(runs every few seconds)"] --> B["Query seats WHERE<br/>status='held' AND hold_expires_at < now"]
    B --> C["For each expired hold"]
    C --> D["Atomic UPDATE:<br/>status='held' AND hold_expires_at<now<br/>→ status='available'"]
    D --> E["Update Seat Map Cache<br/>+ publish availability event"]
    E --> F["Seat becomes visible again<br/>to other users in real-time"]
```

*Using a conditional `WHERE status='held' AND hold_expires_at < now` in the release update (rather than a blind update) guards against a race where a user completes checkout in the exact instant the sweeper runs — the sweeper only releases holds that are actually still expired at update time.*

---

## 9. Handling Extreme On-Sale Contention (Hot Event)

```mermaid
flowchart TB
    A["Taylor Swift tickets go on sale:<br/>2M users, 20K seats"] --> B["Virtual Waiting Room<br/>admits users in controlled batches"]
    B --> C["Admitted users see<br/>seat map (best-available or manual)"]
    C --> D{"Seat selection mode"}
    D -- "Best Available<br/>(most common at scale)" --> E["System auto-assigns from<br/>remaining pool — no user-vs-user<br/>race on specific seats"]
    D -- "Manual seat picking" --> F["User picks specific seat<br/>→ higher contention,<br/>more 'seat just taken' errors"]
    E --> G["Atomic hold + checkout"]
    F --> G
    G --> H["Sold out state reached<br/>→ reject remaining queue instantly"]
```

*Many large-scale platforms default hot on-sales to "best available" seat assignment rather than open seat-map picking specifically because it eliminates the worst contention pattern — thousands of users simultaneously fighting over the same visually "good" seats.*

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Ticket Booking HLD))
    Virtual Queue Service
      Admission control
      Fair ordering
      Rate-matches downstream capacity
    Seat Hold Service
      Atomic conditional holds
      TTL-based expiry
      Prevents double-booking
    Seat Availability Service
      Cached read path for browsing
      Real-time push updates
    Checkout Orchestrator
      Validates hold still active
      Coordinates payment + booking
    Booking Service
      Converts hold to confirmed booking
      Durable booking record
    Hold Expiry Sweeper
      Background reclaim of abandoned holds
      Conditional atomic release
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Admission control | Virtual waiting room before seat selection | Caps concurrent load on the hold service to what it can safely handle without lock contention collapse |
| Double-booking prevention | Atomic conditional DB transaction (`SELECT FOR UPDATE`) | Application-level check-then-act is unsafe under concurrency; DB-level atomicity is the only reliable guarantee |
| Hold model | Short TTL hold, separate from final booking commit | Gives users time to complete payment without permanently locking seats from other buyers |
| Seat map browsing | Cached, near-real-time, eventually consistent | Browsing doesn't need strict consistency; only the hold/commit decision does |
| Hot event seat assignment | Default to "best available" over manual picking | Eliminates the worst-case contention pattern of many users racing for the same visible seats |
| Hold release | Conditional sweep (`WHERE status AND expires_at`) | Prevents a race between expiry sweep and last-second checkout completion |

---

## 12. Bottlenecks & Scaling Considerations

- **Lock contention on popular sections** — even with best-available assignment, extremely popular sections (front row) can still create hot rows; consider randomized/sharded assignment within a price tier to spread contention.
- **Virtual queue fairness at massive scale** — must ensure queue admission is genuinely FIFO-ish and resistant to bots/scalpers cutting the line (CAPTCHA, rate limiting per account/IP at queue entry).
- **Seat map cache staleness during high churn** — during an on-sale spike, seat status changes extremely fast; push-based cache invalidation (not polling) is essential to avoid showing wildly stale "available" seats.
- **Payment gateway latency inside a time-boxed hold** — if payment processing is slow, users can lose their hold mid-checkout; consider extending hold TTL once payment is actually in-flight rather than using a single fixed window.
- **Database write throughput during peak hold-creation** — thousands of concurrent hold transactions per second on a single event's seat rows; may require sharding seats by section/row to distribute write load across partitions.
- **Refund/cancellation seat release** — must re-enter the same atomic status-transition logic as original booking to avoid conflicts with concurrent new bookings for the just-released seat.
- **Multi-event scaling** — architecture must isolate hot events from each other; a massive on-sale for one concert shouldn't degrade booking performance for unrelated events sharing the same infrastructure.
