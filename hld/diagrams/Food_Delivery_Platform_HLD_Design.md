# Design a Food Delivery Platform — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Customers browse restaurants, view menus, place orders
- Restaurants receive and accept/reject orders, mark food ready
- Drivers get matched to pick up and deliver orders
- Real-time order tracking (prep status, driver location, ETA)
- Three-sided marketplace: customers, restaurants, drivers all need coordinated state
- Payment processing and restaurant/driver payouts

### Non-Functional Requirements
- **Scale:** Millions of orders/day, geographically distributed demand
- **Real-time coordination:** Order state must sync accurately across customer, restaurant, and driver apps
- **Time-sensitive matching:** Driver assignment must account for restaurant prep time, not just proximity
- **Availability:** A regional outage shouldn't affect other cities (similar to Uber's model)
- **Consistency:** Order status must never show conflicting states to different parties

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Orders/day (large platform) | ~5M |
| Orders/sec (peak, dinner rush) | ~500-1,000/sec |
| Avg order lifecycle duration | 30-45 minutes |
| Active drivers (peak, per city) | Thousands |
| Location updates/sec (drivers) | Similar firehose pattern to ride-hailing |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    CustApp["Customer App"]
    RestApp["Restaurant App/Tablet"]
    DriverApp["Driver App"]

    Gateway["API Gateway"]

    subgraph Core["Core Services"]
        MenuSvc["Menu/Catalog Service"]
        OrderSvc["Order Orchestrator"]
        RestaurantSvc["Restaurant Service<br/>(accept/reject, prep time)"]
        DispatchSvc["Driver Dispatch Service"]
        LocationSvc["Location Service"]
        PaymentSvc["Payment Service"]
    end

    subgraph Geo["Geospatial Layer"]
        GeoIndex[("Driver Geospatial Index<br/>(Redis)")]
    end

    subgraph Storage["Storage Layer"]
        OrderDB[("Order DB<br/>(state machine, transactional)")]
        MenuDB[("Menu/Restaurant DB")]
        DriverStore[("Driver Availability Store")]
    end

    subgraph Async["Async Layer"]
        Kafka["Kafka<br/>(OrderStatusChanged events)"]
        NotifSvc["Notification Service"]
    end

    CustApp --> Gateway --> MenuSvc --> MenuDB
    Gateway --> OrderSvc --> OrderDB
    RestApp --> Gateway --> RestaurantSvc --> OrderDB
    DriverApp --> Gateway --> LocationSvc --> GeoIndex

    OrderSvc --> DispatchSvc
    DispatchSvc --> GeoIndex
    DispatchSvc --> DriverStore

    OrderSvc --> PaymentSvc
    OrderSvc --> Kafka
    Kafka --> NotifSvc
    NotifSvc --> CustApp
    NotifSvc --> RestApp
    NotifSvc --> DriverApp
```

**Key idea:** This is a **three-sided marketplace** where a single order touches customer, restaurant, and driver simultaneously — the Order Orchestrator acts as the single source of truth for order state, and every party's view is derived from (never independently mutates) that central state machine.

---

## 3. Data Model

```mermaid
erDiagram
    CUSTOMER ||--o{ ORDER : places
    RESTAURANT ||--o{ MENU_ITEM : offers
    RESTAURANT ||--o{ ORDER : receives
    ORDER ||--o{ ORDER_ITEM : contains
    DRIVER ||--o{ DELIVERY : performs
    ORDER ||--|| DELIVERY : "fulfilled by"

    CUSTOMER {
        string customer_id PK
        string default_address
    }
    RESTAURANT {
        string restaurant_id PK
        string name
        string status "open/closed/busy"
        int avg_prep_time_min
        float lat
        float lng
    }
    MENU_ITEM {
        string item_id PK
        string restaurant_id FK
        string name
        float price
        bool available
    }
    ORDER {
        string order_id PK
        string customer_id FK
        string restaurant_id FK
        string status "placed/accepted/preparing/ready/picked_up/delivered/cancelled"
        float total_amount
        timestamp placed_at
        timestamp estimated_delivery_at
    }
    ORDER_ITEM {
        string order_id FK
        string item_id FK
        int quantity
    }
    DRIVER {
        string driver_id PK
        string status "available/assigned/offline"
        float lat
        float lng
    }
    DELIVERY {
        string delivery_id PK
        string order_id FK
        string driver_id FK
        string status "assigned/en_route_to_restaurant/picked_up/en_route_to_customer/delivered"
    }
```

---

## 4. Order Lifecycle State Machine

```mermaid
stateDiagram-v2
    [*] --> Placed
    Placed --> Accepted: Restaurant accepts
    Placed --> Rejected: Restaurant rejects / no response
    Accepted --> Preparing: Kitchen starts prep
    Preparing --> ReadyForPickup: Food ready
    ReadyForPickup --> PickedUp: Driver collects order
    PickedUp --> Delivered: Driver completes delivery
    Accepted --> Cancelled: Customer/restaurant cancels
    Preparing --> Cancelled: Rare — issue at restaurant
    Rejected --> [*]
    Delivered --> [*]
    Cancelled --> [*]
```

*This is the backbone the entire system revolves around — every service (dispatch, notifications, payment capture) reacts to transitions in this single state machine rather than maintaining independent state.*

---

## 5. Order Placement Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Customer App
    participant OO as Order Orchestrator
    participant Rest as Restaurant Service
    participant Pay as Payment Service
    participant DB as Order DB
    participant K as Kafka
    participant R as Restaurant App

    C->>OO: Place order {restaurant_id, items, address}
    OO->>Rest: Validate restaurant is open + items available
    Rest-->>OO: Valid

    OO->>Pay: Authorize payment (hold, not capture yet)
    Pay-->>OO: Authorization successful

    OO->>DB: Create order (status = PLACED)
    OO->>K: Emit OrderPlaced event
    K->>R: Push new order notification

    OO-->>C: Order placed, awaiting restaurant confirmation

    Note over R: Restaurant has limited time<br/>(e.g., 2 min) to respond
    R->>OO: Accept order + estimated_prep_time
    OO->>DB: Update status = ACCEPTED
    OO->>Pay: Capture payment (now that restaurant confirmed)
    OO->>K: Emit OrderAccepted event
```

**Key design point:** Payment is **authorized** (funds held, not charged) when the order is placed, but only **captured** (actually charged) once the restaurant accepts. This avoids charging a customer for an order the restaurant ends up rejecting (out of stock, closed unexpectedly).

---

## 6. Driver Dispatch — Timing-Aware Matching

```mermaid
flowchart TB
    A["Order status = ACCEPTED<br/>with estimated_prep_time = 15 min"] --> B["Dispatch Service:<br/>When to assign a driver?"]
    B --> C{"Dispatch Strategy"}
    C --> D["Too early: driver waits idle<br/>at restaurant, wastes driver time"]
    C --> E["Too late: food gets cold<br/>waiting for driver to arrive"]
    C --> F["Optimal: assign driver so<br/>arrival time ≈ food ready time"]
    F --> G["Dispatch Service calculates:<br/>driver_ETA_to_restaurant ≈ prep_time_remaining"]
    G --> H["Query nearby available drivers<br/>via Geospatial Index"]
    H --> I["Select driver whose ETA to restaurant<br/>best matches food-ready time"]
```

*This is the key differentiator from ride-hailing dispatch: it's not simply "nearest available driver" — it's **time-synchronized matching** where the system deliberately delays assignment until the driver's travel time to the restaurant roughly aligns with when the food will actually be ready.*

---

## 7. Driver Dispatch — Detailed Sequence

```mermaid
sequenceDiagram
    participant OO as Order Orchestrator
    participant DS as Dispatch Service
    participant GeoIdx as Geospatial Index
    participant D as Candidate Driver
    participant DriverStore as Driver State Store

    OO->>DS: Order accepted, prep_time=15min, restaurant_location

    Note over DS: Wait until prep_time - avg_driver_ETA<br/>before triggering dispatch
    DS->>GeoIdx: Find available drivers near restaurant
    GeoIdx-->>DS: Candidates with ETAs

    DS->>DS: Rank by (ETA closeness to food-ready time,<br/>driver rating, distance)
    DS->>D: Send delivery offer
    alt Driver accepts
        D-->>DS: Accept
        DS->>DriverStore: Mark driver ASSIGNED
        DS->>OO: Driver assigned, notify customer + restaurant
    else Driver declines/times out
        DS->>DS: Offer to next candidate
    end
```

---

## 8. Real-Time Order Tracking (Customer View)

```mermaid
flowchart LR
    A["Order status changes<br/>(kitchen, dispatch, driver location)"] --> B["Kafka: OrderStatusChanged /<br/>DriverLocationUpdated"]
    B --> C["Per-order Pub/Sub Channel"]
    C --> D["Customer App<br/>(subscribed to their order_id)"]
    D --> E["Live UI updates:<br/>'Preparing' → 'Driver en route'<br/>→ live map → 'Delivered'"]
```

*Same per-entity pub/sub pattern as ride-hailing's live trip tracking — the customer subscribes to updates scoped to their specific order, not a broad broadcast channel.*

---

## 9. Handling Restaurant Non-Response / Rejection

```mermaid
flowchart TB
    A["Order sent to restaurant"] --> B{"Restaurant responds<br/>within timeout (2 min)?"}
    B -- "Accept" --> C["Proceed to preparation"]
    B -- "Reject" --> D["Cancel order<br/>Release payment authorization"]
    B -- "No response (timeout)" --> E["Auto-cancel<br/>(restaurant likely overwhelmed/offline)"]
    D --> F["Notify customer:<br/>'Restaurant unable to fulfill'<br/>+ suggest alternatives"]
    E --> F
```

---

## 10. Multi-Region / City-Based Partitioning

```mermaid
flowchart TB
    subgraph CityA["City A Deployment"]
        DispatchA["Dispatch Service (City A)"]
        GeoA["Geo Index (City A)"]
    end
    subgraph CityB["City B Deployment"]
        DispatchB["Dispatch Service (City B)"]
        GeoB["Geo Index (City B)"]
    end

    Note1["Same rationale as ride-hailing:<br/>matching is inherently local.<br/>City-level isolation contains<br/>outages and simplifies scaling."]
```

---

## 11. Component Responsibilities Summary

```mermaid
mindmap
  root((Food Delivery HLD))
    Order Orchestrator
      Central order state machine
      Coordinates all three sides
    Restaurant Service
      Accept/reject handling
      Prep time estimation
      Menu availability
    Dispatch Service
      Time-synchronized driver matching
      Not just nearest-driver
    Location Service
      Driver location ingestion
      Geospatial indexing
    Payment Service
      Authorize-then-capture flow
      Restaurant/driver payouts
    Notification Service
      Fan-out to customer, restaurant, driver
      Real-time status push
```

---

## 12. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Order state ownership | Single central state machine (Order Orchestrator) | Prevents conflicting views across customer/restaurant/driver apps |
| Payment timing | Authorize at order placement, capture at restaurant acceptance | Avoids charging for orders the restaurant can't fulfill |
| Driver dispatch timing | Time-synchronized to food-ready estimate, not immediate nearest-match | Minimizes both driver idle-wait time and food getting cold |
| Restaurant response | Hard timeout with auto-cancel fallback | Prevents orders hanging indefinitely if a restaurant is unresponsive |
| Regional architecture | City-level partitioning | Delivery matching is inherently local; contains blast radius |
| Order tracking | Per-order pub/sub channel | Scoped, efficient real-time updates without broad broadcast overhead |

---

## 13. Bottlenecks & Scaling Considerations

- **Prep time estimation accuracy** — inaccurate restaurant prep-time estimates cascade into bad dispatch timing (driver waits too long or arrives too early); often improved with historical data per restaurant rather than restaurant-reported estimates alone.
- **Dinner rush concentration** — demand is extremely peaked around meal times in each city; dispatch and geospatial services need to handle sharp load spikes, not just steady average throughput.
- **Driver supply shortages in specific micro-areas** — a popular restaurant cluster can outstrip nearby driver supply even if city-wide supply looks adequate; may need surge incentives similar to ride-hailing to pull drivers into under-supplied zones.
- **Order state consistency across three apps** — must guard against race conditions like a restaurant marking "ready" at the exact moment the customer cancels; state transitions need to be atomic and validate current state before applying.
- **Multi-restaurant orders / large catering orders** — significantly complicate the "single restaurant, single driver" assumption; often require pooling multiple drivers or accepting longer fulfillment windows.
- **Real-time location fanout during peak hours** — thousands of simultaneous active deliveries per city, each needing live tracking pushed to a customer, adds up to substantial pub/sub fanout load requiring the same firehose-handling infrastructure as ride-hailing.
- **Payment payout reconciliation** — restaurant and driver payouts must reconcile precisely against completed orders, refunds, and cancellations; typically handled by a separate async ledger/reconciliation system rather than inline with the order flow.
