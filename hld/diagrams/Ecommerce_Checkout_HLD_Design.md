# Design an E-commerce Checkout System — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Add items to cart, view cart, modify quantities
- Checkout flow: address, shipping, payment, order confirmation
- Inventory must be reserved during checkout (no overselling)
- Support multiple payment methods, retries on failure
- Order confirmation and receipt generation
- Handle abandoned carts / checkout timeouts gracefully

### Non-Functional Requirements
- **Correctness over speed:** Never oversell inventory, never double-charge
- **Idempotency:** Network retries must not create duplicate orders/charges
- **Scale:** Flash-sale traffic spikes (10-100x normal load in minutes)
- **Consistency:** Strong consistency required for inventory decrement; eventual consistency acceptable for order history display
- **Availability:** Checkout should degrade gracefully rather than hard-fail during spikes

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Checkouts/sec (normal) | ~1,000 |
| Checkouts/sec (flash sale peak) | ~50,000+ |
| Avg items per order | ~3 |
| Inventory check + reserve latency budget | < 100ms |
| Payment gateway round-trip | 500ms - 2s (external dependency) |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Client["Client<br/>(Web/Mobile)"]
    Gateway["API Gateway"]

    subgraph Core["Core Services"]
        CartSvc["Cart Service"]
        CheckoutSvc["Checkout Orchestrator"]
        InventorySvc["Inventory Service"]
        PricingSvc["Pricing/Tax/Promo Service"]
        PaymentSvc["Payment Service"]
        OrderSvc["Order Service"]
    end

    subgraph Async["Async Processing"]
        Kafka["Kafka<br/>(OrderPlaced, PaymentConfirmed events)"]
        FulfillmentWorker["Fulfillment/Shipping Workers"]
        EmailWorker["Email/Notification Workers"]
    end

    subgraph Storage["Storage Layer"]
        CartStore[("Cart Store<br/>(Redis - session-based)")]
        InventoryDB[("Inventory DB<br/>(strongly consistent counters)")]
        OrderDB[("Order DB<br/>(durable, transactional)")]
        PaymentGW["External Payment Gateway<br/>(Stripe/Adyen)"]
    end

    Client --> Gateway
    Gateway --> CartSvc --> CartStore
    Gateway --> CheckoutSvc

    CheckoutSvc --> InventorySvc --> InventoryDB
    CheckoutSvc --> PricingSvc
    CheckoutSvc --> PaymentSvc --> PaymentGW
    CheckoutSvc --> OrderSvc --> OrderDB

    OrderSvc --> Kafka
    Kafka --> FulfillmentWorker
    Kafka --> EmailWorker
```

**Key idea:** The Checkout Orchestrator coordinates a multi-step process (reserve inventory → calculate price → charge payment → create order) where each step can fail independently. This is a textbook **distributed transaction / saga** problem — money and inventory must never end up inconsistent even when a step fails partway through.

---

## 3. Data Model

```mermaid
erDiagram
    USER ||--o{ CART : owns
    CART ||--o{ CART_ITEM : contains
    USER ||--o{ ORDER : places
    ORDER ||--o{ ORDER_ITEM : contains
    ORDER ||--|| PAYMENT : "paid via"
    PRODUCT ||--o{ INVENTORY_RESERVATION : "reserved from"
    ORDER ||--o{ INVENTORY_RESERVATION : holds

    USER {
        string user_id PK
        string default_address_id
    }
    CART {
        string cart_id PK
        string user_id FK
        timestamp updated_at
    }
    CART_ITEM {
        string cart_id FK
        string product_id FK
        int quantity
    }
    PRODUCT {
        string product_id PK
        string name
        float price
        int available_stock
    }
    INVENTORY_RESERVATION {
        string reservation_id PK
        string product_id FK
        string order_id FK
        int quantity
        string status "held/committed/released"
        timestamp expires_at
    }
    ORDER {
        string order_id PK
        string user_id FK
        string idempotency_key
        string status "pending/paid/failed/cancelled"
        float total_amount
        timestamp created_at
    }
    ORDER_ITEM {
        string order_id FK
        string product_id FK
        int quantity
        float price_at_purchase
    }
    PAYMENT {
        string payment_id PK
        string order_id FK
        string status
        string gateway_transaction_id
    }
```

---

## 4. Checkout Flow — The Saga Pattern

```mermaid
flowchart TB
    A["1. Reserve Inventory<br/>(hold stock, TTL-based)"] --> B{"Success?"}
    B -- No --> Z1["Fail fast:<br/>'Item out of stock'"]
    B -- Yes --> C["2. Calculate Final Price<br/>(tax, shipping, promos)"]
    C --> D["3. Charge Payment"]
    D --> E{"Payment Success?"}
    E -- No --> F["Compensate:<br/>Release inventory reservation"]
    F --> Z2["Fail: 'Payment declined'"]
    E -- Yes --> G["4. Create Order Record"]
    G --> H{"Order creation success?"}
    H -- No --> I["Compensate:<br/>Refund payment + release inventory"]
    I --> Z3["Fail: 'Order could not be completed'"]
    H -- Yes --> J["5. Commit inventory reservation<br/>(convert hold to permanent decrement)"]
    J --> K["6. Emit OrderPlaced event<br/>(async fulfillment, email, etc.)"]
    K --> L["Success: Order Confirmed"]
```

**Key idea:** Every step that changes state (inventory hold, payment charge) has a corresponding **compensating action** (release hold, refund) that runs if a later step fails. This is the Saga pattern — since checkout spans multiple services/systems, a classic all-or-nothing database transaction isn't possible, so correctness is achieved through explicit forward + compensating steps.

---

## 5. Inventory Reservation — Detailed Sequence (Preventing Overselling)

```mermaid
sequenceDiagram
    participant C as Client
    participant CO as Checkout Orchestrator
    participant Inv as Inventory Service
    participant DB as Inventory DB

    C->>CO: Initiate checkout {product_id, qty: 2}
    CO->>Inv: Reserve(product_id, qty=2)

    Inv->>DB: BEGIN TRANSACTION
    Inv->>DB: SELECT available_stock WHERE product_id=X FOR UPDATE
    DB-->>Inv: available_stock = 5

    alt available_stock >= requested_qty
        Inv->>DB: UPDATE available_stock = available_stock - 2
        Inv->>DB: INSERT reservation (status=HELD, expires_at=now+10min)
        Inv->>DB: COMMIT
        Inv-->>CO: Reservation confirmed (reservation_id)
    else Insufficient stock
        Inv->>DB: ROLLBACK
        Inv-->>CO: Insufficient stock error
        CO-->>C: "Only 3 left in stock"
    end
```

**Key design point:** The `SELECT ... FOR UPDATE` (row-level lock) combined with an atomic decrement is what prevents the classic race condition where two concurrent checkouts both read `stock=1` and both proceed to "successfully" reserve the last unit — a pure application-level check-then-act without locking is unsafe under concurrency.

---

## 6. Handling Reservation Expiry (Abandoned Checkout)

```mermaid
flowchart LR
    A["Inventory reserved<br/>(status = HELD, expires_at = +10min)"] --> B{"Checkout completed<br/>within window?"}
    B -- "Yes — payment succeeds" --> C["Reservation status = COMMITTED<br/>(permanent decrement)"]
    B -- "No — user abandons<br/>or times out" --> D["Background job:<br/>Sweep expired reservations"]
    D --> E["Release stock back to<br/>available_stock pool"]
    E --> F["Reservation status = RELEASED"]
```

*A time-boxed hold (e.g., 10 minutes) balances two competing needs: giving a legitimate slow shopper enough time to complete checkout, while not letting abandoned carts lock up inventory indefinitely during high-demand events.*

---

## 7. Idempotent Order Creation (Preventing Duplicate Charges)

```mermaid
sequenceDiagram
    participant C as Client
    participant CO as Checkout Orchestrator
    participant OrderDB as Order DB

    C->>CO: POST /checkout {idempotency_key: "abc123", ...}

    CO->>OrderDB: SELECT * WHERE idempotency_key = 'abc123'
    alt Key already exists
        OrderDB-->>CO: Existing order found (order_id: 789)
        CO-->>C: Return existing order 789<br/>(do NOT reprocess payment)
    else Key not seen before
        OrderDB-->>CO: No existing record
        CO->>CO: Proceed with full checkout saga
        CO->>OrderDB: INSERT order with idempotency_key = 'abc123'
        CO-->>C: New order created
    end

    Note over C,CO: If client's network times out and retries<br/>the same request, the idempotency_key<br/>ensures the second attempt is a no-op,<br/>not a duplicate charge
```

**Key idea:** Every checkout request carries a client-generated `idempotency_key` (typically a UUID generated once per checkout attempt). If the client retries after a timeout (not knowing if the first request succeeded), the server recognizes the duplicate key and returns the original result instead of processing payment twice.

---

## 8. Payment Charging Flow (with Retry Logic)

```mermaid
sequenceDiagram
    participant CO as Checkout Orchestrator
    participant PS as Payment Service
    participant GW as External Payment Gateway

    CO->>PS: Charge {order_id, amount, payment_method, idempotency_key}
    PS->>GW: Create charge request<br/>(pass idempotency_key to gateway too)

    alt Gateway responds successfully
        GW-->>PS: Charge succeeded (transaction_id)
        PS-->>CO: Payment confirmed
    else Gateway times out (unknown result)
        GW-->>PS: Timeout/no response
        PS->>GW: Query charge status by idempotency_key
        alt Charge actually succeeded
            GW-->>PS: Found: charge succeeded
            PS-->>CO: Payment confirmed
        else Charge did not go through
            GW-->>PS: Not found / failed
            PS->>PS: Safe to retry (same idempotency_key)
        end
    else Gateway declines (insufficient funds, fraud flag)
        GW-->>PS: Declined
        PS-->>CO: Payment failed — trigger compensation
    end
```

*Passing the same idempotency key to the external payment gateway itself (most major gateways like Stripe support this natively) is critical — it protects against the ambiguous "did my charge actually go through before the timeout?" scenario, which a client-side retry alone cannot resolve safely.*

---

## 9. Flash Sale / High-Contention Traffic Handling

```mermaid
flowchart TB
    A["Flash sale starts:<br/>50,000 requests/sec for<br/>1,000 units of a product"] --> B["Rate limiter / queue<br/>at API Gateway"]
    B --> C["Virtual waiting room<br/>(queue tickets issued)"]
    C --> D["Admit requests in controlled batches<br/>matching downstream capacity"]
    D --> E["Inventory Service:<br/>Atomic decrement, row-level lock"]
    E --> F{"Stock available?"}
    F -- Yes --> G["Proceed to checkout"]
    F -- No --> H["Immediate 'sold out' response<br/>(no wasted downstream calls)"]
```

*Rather than letting all 50,000 requests hammer the inventory service simultaneously (causing lock contention meltdown), a "virtual waiting room" admits requests at a rate the system can actually process — most e-commerce platforms use this pattern for major flash sales/drops.*

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((E-commerce Checkout HLD))
    Checkout Orchestrator
      Coordinates saga steps
      Handles compensation on failure
      Idempotency key enforcement
    Inventory Service
      Atomic reserve/release
      Row-level locking
      TTL-based hold expiry
    Payment Service
      Gateway integration
      Idempotent charge requests
      Timeout/ambiguity resolution
    Order Service
      Durable order persistence
      Order status tracking
    Pricing Service
      Tax, shipping, promo calculation
    Fulfillment Workers
      Async post-order processing
      Shipping label generation
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Multi-step consistency | Saga pattern with compensating actions | No single database transaction can span cart, inventory, payment gateway, and order services |
| Inventory concurrency control | Row-level locking + atomic decrement | Prevents overselling under concurrent checkout attempts on the same product |
| Reservation model | Time-boxed hold (TTL) rather than instant permanent decrement | Balances giving shoppers time to complete checkout against not locking up stock indefinitely |
| Duplicate request handling | Client-generated idempotency key, checked server-side and at payment gateway | Network retries are inevitable; must not result in double charges or duplicate orders |
| Flash sale traffic | Virtual waiting room / admission control | Prevents lock contention meltdown from massively overwhelming a scarce-inventory resource |
| Post-order processing | Async via event bus (Kafka) | Shipping, email, analytics shouldn't block the user-facing checkout confirmation |

---

## 12. Bottlenecks & Scaling Considerations

- **Inventory row contention on hot products** — a single viral/flash-sale product creates a hot row that many concurrent transactions lock against; consider sharding stock counters (e.g., split 1000 units across 10 counter shards of 100 each) to reduce contention, reconciling at the end.
- **Payment gateway latency** — external dependency (500ms-2s) sits directly in the critical path; must have circuit breakers and clear timeout/retry semantics so a slow gateway doesn't cascade into checkout service exhaustion.
- **Idempotency key storage growth** — needs a bounded retention window (e.g., 24-48 hours) since keys are only useful for detecting near-term retries, not indefinite storage.
- **Reservation expiry sweep job** — must run frequently enough that abandoned holds don't starve legitimate demand during high-traffic events, but not so frequently that it adds unnecessary DB load.
- **Cart service scaling** — carts are read/written far more frequently than checkouts complete; typically backed by a fast KV store (Redis) rather than the transactional order database.
- **Cross-region checkout** — for global platforms, inventory for region-specific warehouses must be tracked separately; a checkout in the EU shouldn't decrement US warehouse stock.
- **Fraud detection latency** — real-time fraud scoring adds latency to the payment step; often run asynchronously post-charge with the ability to reverse/flag suspicious orders rather than blocking checkout entirely.
