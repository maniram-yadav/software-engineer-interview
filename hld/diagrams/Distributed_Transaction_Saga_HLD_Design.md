# Design a Distributed Transaction System Across Microservices — High-Level Design Document

## 1. Requirements

### Functional Requirements
- A single business operation (e.g., "place order") must atomically succeed or fail across multiple independently-owned microservices (Order, Payment, Inventory, Shipping)
- If any step fails partway through, previously completed steps must be undone (compensated)
- Support both synchronous (blocking, immediate consistency) and asynchronous (eventual consistency) coordination patterns
- Provide visibility into transaction/saga state for debugging and monitoring

### Non-Functional Requirements
- **No partial completion left silently unresolved:** every started transaction must reach a definitively known final state (all committed or all compensated)
- **Availability:** shouldn't require all services to be simultaneously available and fast — must tolerate individual service slowness/failure gracefully
- **Scalability:** must not become a bottleneck as transaction volume grows
- **Idempotency:** each step (and each compensation) may be retried and must not cause duplicate side effects

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Transactions/sec | ~10,000 |
| Avg services involved per transaction | 3-5 |
| Avg transaction completion time | Seconds (async saga) vs sub-second (sync 2PC, when it works) |
| Compensation rate | Small % of transactions require rollback (failures, inventory conflicts, etc.) |

---

## 2. Two Fundamental Approaches

```mermaid
flowchart TB
    A["Distributed Transaction<br/>Coordination Pattern"] --> B["Two-Phase Commit (2PC)"]
    A --> C["Saga Pattern"]

    B --> B1["Synchronous, blocking"]
    B --> B2["Strong consistency —<br/>all-or-nothing, atomically"]
    B --> B3["Requires all participants<br/>to support a 'prepare' phase<br/>and hold locks"]
    B --> B4["Poor fit for microservices —<br/>couples availability of ALL<br/>services together"]

    C --> C1["Asynchronous, non-blocking"]
    C --> C2["Eventual consistency —<br/>sequence of local transactions<br/>with compensating actions"]
    C --> C3["Each service commits its<br/>own local transaction<br/>independently"]
    C --> C4["Better fit for microservices —<br/>services remain independently<br/>available/scalable"]
```

*This design primarily uses the **Saga pattern** for microservices — 2PC's requirement that all participants block and hold locks until every service responds fundamentally conflicts with microservice architecture's goal of independent service availability. A single slow/down service in 2PC blocks the entire transaction and holds locks across all others.*

---

## 3. High-Level Architecture (Orchestration-Based Saga)

```mermaid
flowchart TB
    Client["Client"]
    Gateway["API Gateway"]

    subgraph Orchestrator["Saga Orchestrator"]
        SagaEngine["Saga Execution Engine"]
        SagaLog[("Saga State Log<br/>(durable, source of truth)")]
    end

    subgraph Services["Participating Microservices"]
        OrderSvc["Order Service"]
        PaymentSvc["Payment Service"]
        InventorySvc["Inventory Service"]
        ShippingSvc["Shipping Service"]
    end

    Client --> Gateway --> SagaEngine
    SagaEngine --> SagaLog

    SagaEngine -->|"1. Create Order"| OrderSvc
    SagaEngine -->|"2. Reserve Payment"| PaymentSvc
    SagaEngine -->|"3. Reserve Inventory"| InventorySvc
    SagaEngine -->|"4. Schedule Shipping"| ShippingSvc

    OrderSvc -.->|"success/failure"| SagaEngine
    PaymentSvc -.->|"success/failure"| SagaEngine
    InventorySvc -.->|"success/failure"| SagaEngine
    ShippingSvc -.->|"success/failure"| SagaEngine
```

**Key idea:** A central **Saga Orchestrator** explicitly drives the sequence of steps, tracking state in a durable log. Each participating service performs its own local transaction and reports success/failure back — the orchestrator decides what happens next, including triggering compensating actions if something fails partway through. This is in contrast to a "choreography" approach where services react to each other's events without central coordination.

---

## 4. Data Model

```mermaid
erDiagram
    SAGA_INSTANCE ||--o{ SAGA_STEP : contains
    SAGA_STEP ||--o| COMPENSATION_ACTION : "has compensating"

    SAGA_INSTANCE {
        string saga_id PK
        string type "e.g. place_order"
        string status "in_progress/completed/compensating/failed"
        timestamp started_at
        timestamp completed_at
    }
    SAGA_STEP {
        string step_id PK
        string saga_id FK
        int sequence_order
        string service_name
        string status "pending/completed/failed/compensated"
        string idempotency_key
        timestamp executed_at
    }
    COMPENSATION_ACTION {
        string step_id FK
        string compensation_type "e.g. refund_payment"
        string status "pending/completed"
    }
```

---

## 5. Successful Saga Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client
    participant Orch as Saga Orchestrator
    participant Log as Saga State Log
    participant Order as Order Service
    participant Pay as Payment Service
    participant Inv as Inventory Service

    C->>Orch: Place order request
    Orch->>Log: Create saga_id, status=IN_PROGRESS

    Orch->>Order: Step 1: Create order (idempotency_key=saga_id:1)
    Order-->>Orch: Success, order_id=X
    Orch->>Log: Record step 1 = COMPLETED

    Orch->>Pay: Step 2: Reserve payment (idempotency_key=saga_id:2)
    Pay-->>Orch: Success, payment_reserved
    Orch->>Log: Record step 2 = COMPLETED

    Orch->>Inv: Step 3: Reserve inventory (idempotency_key=saga_id:3)
    Inv-->>Orch: Success, inventory_reserved
    Orch->>Log: Record step 3 = COMPLETED

    Orch->>Log: Update saga status = COMPLETED
    Orch-->>C: Order placed successfully
```

---

## 6. Failed Saga Flow — Compensation (Rollback) Sequence

```mermaid
sequenceDiagram
    participant Orch as Saga Orchestrator
    participant Log as Saga State Log
    participant Order as Order Service
    participant Pay as Payment Service
    participant Inv as Inventory Service

    Note over Orch: Steps 1 (order) and 2 (payment)<br/>already completed successfully

    Orch->>Inv: Step 3: Reserve inventory
    Inv-->>Orch: FAILURE — out of stock

    Orch->>Log: Record step 3 = FAILED
    Orch->>Log: Update saga status = COMPENSATING

    Note over Orch: Must undo steps 1 and 2,<br/>in REVERSE order

    Orch->>Pay: Compensate: refund/release payment reservation
    Pay-->>Orch: Compensation successful
    Orch->>Log: Record step 2 = COMPENSATED

    Orch->>Order: Compensate: cancel order
    Order-->>Orch: Compensation successful
    Orch->>Log: Record step 1 = COMPENSATED

    Orch->>Log: Update saga status = FAILED (fully compensated)
```

**Why compensation happens in reverse order:** Steps often have dependencies — inventory reservation might depend on payment being reserved first. Undoing in reverse order (most recent first) respects those same dependencies in the opposite direction, minimizing the chance of a compensating action itself failing due to an inconsistent intermediate state.

---

## 7. Orchestration vs Choreography

```mermaid
flowchart TB
    A["Saga Coordination Style"] --> B["Orchestration<br/>(centralized)"]
    A --> C["Choreography<br/>(decentralized)"]

    B --> B1["Central orchestrator explicitly<br/>calls each service in sequence"]
    B --> B2["Easy to understand/debug —<br/>saga logic lives in ONE place"]
    B --> B3["Orchestrator is a single<br/>logical dependency<br/>(but can be made highly available)"]

    C --> C1["Each service publishes events;<br/>other services react independently<br/>(no central coordinator)"]
    C --> C2["More decoupled —<br/>no single service knows<br/>the whole flow"]
    C --> C3["Harder to debug/trace —<br/>saga logic is implicitly<br/>scattered across many services'<br/>event handlers"]

    D["This design uses orchestration<br/>for complex, multi-step business<br/>flows where visibility and<br/>explicit control matter most"]
```

---

## 8. Choreography-Based Saga (Alternative Pattern)

```mermaid
sequenceDiagram
    participant Order as Order Service
    participant K as Event Bus (Kafka)
    participant Pay as Payment Service
    participant Inv as Inventory Service

    Order->>Order: Create order (local transaction)
    Order->>K: Publish OrderCreated event

    K->>Pay: Consume OrderCreated
    Pay->>Pay: Reserve payment (local transaction)
    Pay->>K: Publish PaymentReserved event

    K->>Inv: Consume PaymentReserved
    Inv->>Inv: Reserve inventory (local transaction)
    alt Success
        Inv->>K: Publish InventoryReserved event
        Note over K: Order Service consumes this,<br/>marks order as CONFIRMED
    else Failure
        Inv->>K: Publish InventoryReservationFailed event
        K->>Pay: Consume failure event
        Pay->>Pay: Compensate: release payment
        Pay->>K: Publish PaymentReleased event
        K->>Order: Consume, mark order CANCELLED
    end
```

*In choreography, there's no central coordinator — each service knows only "what event do I react to, and what event do I emit next/on failure." This scales well and avoids a central dependency, but the overall business flow logic becomes implicit, spread across many services' event handlers, making it harder to answer "what's the current state of this order" without piecing together events from multiple services.*

---

## 9. Ensuring Idempotency of Each Saga Step

```mermaid
flowchart TB
    A["Orchestrator retries<br/>a step due to timeout<br/>(may have actually succeeded<br/>on the service side already)"] --> B["Each step call includes<br/>a deterministic idempotency_key<br/>(saga_id + step_sequence)"]
    B --> C["Target service checks:<br/>have I already processed<br/>this idempotency_key?"]
    C -- Yes --> D["Return original result,<br/>don't re-execute"]
    C -- No --> E["Execute, record result<br/>under this key"]

    F["Same principle applies to<br/>compensation actions —<br/>a retried compensation must<br/>also be idempotent<br/>(e.g., 'refund' should be safe<br/>to call twice)"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Distributed Transaction HLD))
    Saga Orchestrator
      Drives step sequence
      Triggers compensation on failure
      Central visibility point
    Saga State Log
      Durable source of truth
      Enables recovery after orchestrator crash
    Participating Services
      Execute local transactions
      Expose compensating actions
      Idempotent step handlers
    Event Bus (choreography alt.)
      Decouples service reactions
      No central coordinator
    Compensation Logic
      Reverse-order undo
      Must itself be idempotent
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Coordination pattern | Saga (not 2PC) | 2PC's blocking, lock-holding nature conflicts with microservices' independent-availability goals; saga trades strong consistency for availability and resilience |
| Saga style | Orchestration (primary) | Centralizes complex business flow logic for visibility/debuggability, at the cost of a coordinator dependency (made HA) |
| Consistency model | Eventual (via compensation) | The system passes through intermediate, partially-completed states visible to other parts of the system — this must be an accepted design tradeoff, not hidden |
| Failure recovery | Reverse-order compensation | Respects the same dependency ordering as forward execution, in reverse |
| Idempotency | Deterministic keys per step + compensation | Ensures safe retries at every stage of both the happy path and the rollback path |
| Orchestrator durability | Persistent saga state log | Allows the orchestrator to recover and resume in-flight sagas after a crash, rather than losing transaction state |

---

## 12. Bottlenecks & Scaling Considerations

- **Compensating actions aren't always perfectly reversible** — e.g., "send a confirmation email" can't be un-sent; sagas must be designed around genuinely compensatable actions, or accept that some side effects (like notifications) are best deferred until the saga is guaranteed to succeed.
- **Orchestrator as a critical dependency** — while it doesn't block services synchronously like 2PC, an unavailable orchestrator halts new saga progress; must be deployed with high availability (multiple instances, durable state log as source of truth for recovery).
- **Long-running sagas and stuck states** — a saga waiting on a slow downstream service can remain "in progress" for extended periods; needs timeout policies and alerting for sagas exceeding expected duration, to avoid silent stuck transactions.
- **Semantic/business-level failures vs technical failures** — "inventory reservation failed because out of stock" is a valid business outcome requiring compensation, distinct from "inventory service timed out" which might warrant a retry instead — the orchestrator's failure-handling logic must distinguish these cases.
- **Dual-write problem** — services must atomically both perform their local business transaction AND publish the resulting event/report status back to the orchestrator; naive separate writes can fail independently, requiring patterns like the transactional outbox to guarantee both happen together.
- **Testing complexity** — with many possible failure points (any step can fail, any compensation can fail), thorough testing requires deliberately simulating failures at every stage — sagas have combinatorially more failure paths to validate than a single-service transaction.
- **Cross-saga resource contention** — many concurrent sagas competing for the same limited resource (e.g., same inventory item) can cause a cascade of reservations-then-compensations under high contention; may need additional coordination (e.g., the inventory reservation system itself, as covered in the e-commerce checkout design) to reduce this churn.
