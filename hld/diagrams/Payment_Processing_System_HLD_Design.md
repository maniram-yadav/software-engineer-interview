# Design a Payment Processing System — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Process payments across multiple methods (card, bank transfer, wallet)
- Support authorize → capture → settle flow (and refunds/voids)
- Maintain an accurate, auditable ledger of all money movement
- Integrate with external payment networks/processors (Visa/Mastercard rails, ACH, etc.)
- Reconciliation against processor statements
- Fraud detection integrated into the flow

### Non-Functional Requirements
- **Correctness is paramount:** Money must never be created or destroyed — every cent accounted for
- **Idempotency:** Retries must never result in duplicate charges
- **Auditability:** Every transaction must be traceable and immutable once recorded
- **Consistency:** Strong consistency for balance/ledger operations — no eventual consistency for money
- **Availability:** High, but never at the cost of correctness (better to reject than to double-process)

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Transactions/sec (large platform) | ~5,000-10,000 |
| Peak (Black Friday-style events) | ~50,000+/sec |
| Ledger entries per transaction | 2+ (double-entry: debit + credit) |
| External processor latency | 200ms - 2s |
| Reconciliation frequency | Daily batch against processor settlement files |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Client["Merchant/App<br/>(initiates payment)"]
    Gateway["Payment API Gateway"]

    subgraph Core["Core Services"]
        PaymentSvc["Payment Orchestration Service"]
        FraudSvc["Fraud Detection Service"]
        LedgerSvc["Ledger Service<br/>(double-entry bookkeeping)"]
        RoutingSvc["Payment Routing Service<br/>(processor selection)"]
    end

    subgraph External["External Payment Networks"]
        Processor1["Processor A<br/>(e.g., Visa network)"]
        Processor2["Processor B<br/>(e.g., ACH/bank rails)"]
    end

    subgraph Storage["Storage Layer"]
        TxnDB[("Transaction DB<br/>(strongly consistent,<br/>append-only)")]
        LedgerDB[("Ledger DB<br/>(immutable double-entry records)")]
        IdempotencyStore[("Idempotency Key Store")]
    end

    subgraph Async["Async / Reconciliation"]
        Kafka["Kafka<br/>(PaymentSettled events)"]
        ReconSvc["Reconciliation Service"]
        SettlementFiles["Processor Settlement Files<br/>(daily batch)"]
    end

    Client --> Gateway --> PaymentSvc
    PaymentSvc --> FraudSvc
    PaymentSvc --> IdempotencyStore
    PaymentSvc --> RoutingSvc
    RoutingSvc --> Processor1
    RoutingSvc --> Processor2

    PaymentSvc --> TxnDB
    PaymentSvc --> LedgerSvc --> LedgerDB
    PaymentSvc --> Kafka

    Kafka --> ReconSvc
    SettlementFiles --> ReconSvc
    ReconSvc --> LedgerDB
```

**Key idea:** Every payment operation produces an **immutable ledger entry** using double-entry bookkeeping — money is never simply "updated," it's always recorded as a debit from one account and a matching credit to another. This makes the system inherently auditable and makes bugs detectable (the books must always balance).

---

## 3. Data Model — Double-Entry Ledger

```mermaid
erDiagram
    ACCOUNT ||--o{ LEDGER_ENTRY : "has entries"
    TRANSACTION ||--o{ LEDGER_ENTRY : produces
    TRANSACTION ||--o| PAYMENT_METHOD : "uses"
    MERCHANT ||--o{ TRANSACTION : receives

    ACCOUNT {
        string account_id PK
        string owner_type "customer/merchant/platform"
        string currency
    }
    TRANSACTION {
        string transaction_id PK
        string idempotency_key
        string type "charge/refund/payout"
        string status "authorized/captured/settled/failed/voided"
        float amount
        string currency
        timestamp created_at
    }
    LEDGER_ENTRY {
        string entry_id PK
        string transaction_id FK
        string account_id FK
        string entry_type "debit/credit"
        float amount
        timestamp recorded_at
        bool immutable "always true once written"
    }
    PAYMENT_METHOD {
        string method_id PK
        string type "card/bank/wallet"
        string tokenized_reference "never raw card data"
    }
    MERCHANT {
        string merchant_id PK
        string account_id FK
    }
```

**Key modeling principle:** `LEDGER_ENTRY` rows are **never updated or deleted** — corrections are made by inserting new offsetting entries (e.g., a refund creates new debit/credit entries, it doesn't modify the original charge's entries). This immutability is what makes the ledger auditable.

---

## 4. Authorize → Capture → Settle Flow

```mermaid
stateDiagram-v2
    [*] --> Authorized: Funds held (not yet moved)
    Authorized --> Captured: Merchant confirms delivery/fulfillment
    Authorized --> Voided: Authorization cancelled before capture
    Captured --> Settled: Funds actually transferred (T+1/T+2 typical)
    Captured --> Refunded: Post-capture refund issued
    Settled --> Refunded: Refund after settlement
    Voided --> [*]
    Refunded --> [*]
    Settled --> [*]
```

*This mirrors real card-network mechanics: **authorization** just confirms funds/credit are available and places a hold; **capture** is the merchant's confirmation to actually proceed with the charge; **settlement** is the actual bank-to-bank money movement, which often happens on a T+1 or T+2 delay through batch processing on the card networks.*

---

## 5. Payment Authorization Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant M as Merchant/App
    participant PS as Payment Service
    participant Idem as Idempotency Store
    participant Fraud as Fraud Service
    participant Route as Routing Service
    participant Proc as External Processor
    participant Ledger as Ledger Service
    participant DB as Transaction DB

    M->>PS: Authorize {amount, payment_method, idempotency_key}

    PS->>Idem: Check idempotency_key
    alt Key already processed
        Idem-->>PS: Existing result found
        PS-->>M: Return original result (no reprocessing)
    else New request
        Idem-->>PS: Not seen before
        PS->>Fraud: Score transaction risk
        Fraud-->>PS: Risk score: LOW

        PS->>Route: Select processor for this payment method
        Route-->>PS: Route to Processor A

        PS->>Proc: Authorize request (with idempotency key passed through)
        Proc-->>PS: Authorization approved (auth_code)

        PS->>DB: Persist transaction (status=AUTHORIZED)
        PS->>Ledger: Record ledger entries<br/>(debit: customer hold, credit: pending)
        PS->>Idem: Store result under idempotency_key
        PS-->>M: Authorization successful
    end
```

---

## 6. Double-Entry Ledger — Why It Matters

```mermaid
flowchart TB
    A["Customer pays $100 for an order"] --> B["Ledger Entry 1:<br/>DEBIT Customer's payment method: $100"]
    A --> C["Ledger Entry 2:<br/>CREDIT Platform's pending account: $100"]

    D["Order fulfilled, capture triggered"] --> E["Ledger Entry 3:<br/>DEBIT Platform's pending account: $97<br/>(after $3 platform fee)"]
    D --> F["Ledger Entry 4:<br/>CREDIT Merchant's payable account: $97"]
    D --> G["Ledger Entry 5:<br/>DEBIT Platform's pending account: $3"]
    D --> H["Ledger Entry 6:<br/>CREDIT Platform's revenue account: $3"]

    I["Invariant check"] -.-> J["SUM(all debits) always equals<br/>SUM(all credits), globally.<br/>If not, there's a bug — immediately detectable."]
```

*This is the fundamental correctness mechanism of any real payment system: because every single money movement is recorded as a balanced debit/credit pair, you can always run an integrity check that the books balance. A single "just update the balance" approach has no equivalent self-checking property.*

---

## 7. Idempotency — Preventing Duplicate Charges

```mermaid
sequenceDiagram
    participant C as Client
    participant PS as Payment Service
    participant Idem as Idempotency Store
    participant Proc as External Processor

    Note over C,PS: Client's network times out after sending request<br/>Client doesn't know if it succeeded

    C->>PS: POST /charge {idempotency_key: "key-001"} (Attempt 1)
    PS->>Idem: Lock key-001 (in-progress)
    PS->>Proc: Process charge
    Note over PS,Proc: Response lost due to network issue

    C->>PS: POST /charge {idempotency_key: "key-001"} (Retry)
    PS->>Idem: Check key-001 status
    Idem-->>PS: Status = IN_PROGRESS (from attempt 1)
    PS-->>C: 409 - Request already in progress, please wait

    Note over PS,Proc: Original attempt eventually completes
    PS->>Idem: Update key-001 = COMPLETED, result stored

    C->>PS: POST /charge {idempotency_key: "key-001"} (Retry 2)
    PS->>Idem: Check key-001 status
    Idem-->>PS: Status = COMPLETED, cached result
    PS-->>C: Return original result (no duplicate charge)
```

**Key design point:** The idempotency store must handle the *in-progress* state explicitly, not just completed results — a naive implementation that only checks "was this already completed" is vulnerable to a second concurrent retry racing in while the first attempt is still processing.

---

## 8. Refund Flow

```mermaid
sequenceDiagram
    participant M as Merchant
    participant PS as Payment Service
    participant Ledger as Ledger Service
    participant Proc as External Processor
    participant DB as Transaction DB

    M->>PS: Refund {original_transaction_id, amount, idempotency_key}
    PS->>DB: Verify original transaction is CAPTURED/SETTLED
    DB-->>PS: Valid, eligible for refund

    PS->>Proc: Issue refund via processor
    Proc-->>PS: Refund confirmed

    PS->>DB: Create new transaction (type=REFUND, linked to original)
    PS->>Ledger: Record NEW offsetting ledger entries<br/>(never modify original charge's entries)
    PS-->>M: Refund processed
```

---

## 9. Reconciliation Against Processor Settlement Files

```mermaid
flowchart TB
    A["Daily: Processor sends<br/>settlement file<br/>(all transactions they actually settled)"] --> B["Reconciliation Service"]
    B --> C["Compare against internal Ledger DB<br/>for the same date range"]
    C --> D{"Every internal transaction<br/>has a matching settlement entry?"}
    D -- Yes --> E["Mark transactions as<br/>SETTLED + reconciled"]
    D -- "Discrepancy found" --> F["Flag for manual review:<br/>- Missing settlement<br/>- Amount mismatch<br/>- Unexpected processor entry"]
    F --> G["Finance/ops team investigates<br/>and resolves discrepancy"]
```

*Reconciliation exists because the internal system's view of "this payment succeeded" and the processor's actual settled state can drift — due to processor-side failures, chargebacks, or timing differences. This daily batch check is the safety net that catches money-accounting bugs before they compound.*

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Payment System HLD))
    Payment Orchestration Service
      Authorize/capture/void coordination
      Idempotency enforcement
    Ledger Service
      Double-entry bookkeeping
      Immutable entries only
    Fraud Detection Service
      Real-time risk scoring
      Pre-authorization check
    Routing Service
      Processor selection
      Failover between processors
    Reconciliation Service
      Daily settlement file comparison
      Discrepancy flagging
    Idempotency Store
      In-progress + completed state tracking
      Prevents duplicate processing
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Money accounting model | Double-entry ledger, immutable entries | Self-checking correctness (debits always equal credits); auditability; corrections via new entries, never mutation |
| Payment flow | Authorize → Capture → Settle (not instant single-step charge) | Mirrors real card network mechanics; allows cancellation before funds actually move |
| Idempotency | Explicit in-progress + completed states in idempotency store | Naive "check if completed" alone is vulnerable to concurrent retry races |
| Consistency model | Strong consistency for ledger/balance operations | Money can never tolerate eventual consistency — a "temporarily wrong balance" is unacceptable |
| Reconciliation | Daily batch comparison against processor settlement files | Catches drift between internal state and actual settled reality; essential safety net |
| Processor integration | Routing layer with failover | Avoids single point of failure on one payment processor; enables smart routing by cost/success rate |

---

## 12. Bottlenecks & Scaling Considerations

- **Ledger write throughput** — every transaction produces multiple immutable ledger entries; the ledger DB must support high append-only write throughput while maintaining strong consistency, often via sharding by account_id.
- **External processor latency in the critical path** — authorization calls to Visa/Mastercard/ACH rails add 200ms-2s; must have circuit breakers and processor failover to avoid cascading slowness during a processor outage.
- **Idempotency store as a single point of contention** — every single payment request touches it first; must be extremely available and low-latency (in-memory KV with durable backing) since it gates all downstream processing.
- **Fraud detection latency vs accuracy tradeoff** — thorough fraud scoring takes time, but authorization needs to feel instant; typically balanced with fast rule-based pre-checks synchronously and deeper ML scoring asynchronously (with ability to reverse a transaction post-hoc if flagged).
- **Peak event scaling (Black Friday)** — transaction volume can spike 5-10x normal; ledger sharding strategy and processor rate limits must be stress-tested well ahead of predictable peak events.
- **Multi-currency complexity** — cross-currency transactions require careful handling of exchange rates at the exact transaction time, recorded immutably alongside the ledger entries for audit purposes.
- **Chargebacks and disputes** — represent a delayed, out-of-band reversal that must be modeled as new ledger entries long after original settlement, requiring the system to handle "correcting the past" without ever editing historical records.
