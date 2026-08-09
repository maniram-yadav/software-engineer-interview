# Design a System for Idempotent API Requests at Scale — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Clients can safely retry any mutating API request (POST, PATCH) without risk of duplicate side effects
- Support idempotency keys supplied by clients
- Return the original response for a retried request with the same key, not re-execute the operation
- Handle concurrent retries of the same request (race condition between two near-simultaneous retries)
- Support idempotency across distributed backend services, not just a single monolith

### Non-Functional Requirements
- **Correctness above all:** This system exists specifically to prevent duplicate financial/state-changing operations — false negatives (allowing a duplicate through) are unacceptable
- **Low latency overhead:** Idempotency checking must add minimal latency to every mutating request
- **Availability:** The idempotency layer itself must not become a single point of failure for the whole API
- **Bounded storage:** Idempotency records can't be kept forever — need a sensible expiration policy
- **Distributed consistency:** Two concurrent requests with the same key from different servers must not both "win"

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Mutating API requests/sec | ~100,000 |
| Retry rate (typical) | ~1-5% of requests are retries |
| Idempotency key TTL | 24 hours (typical — long enough to cover realistic retry windows) |
| Idempotency store size | Proportional to (requests/sec × TTL) |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Client["API Client"]
    Gateway["API Gateway"]

    subgraph IdemLayer["Idempotency Layer"]
        IdemMiddleware["Idempotency Middleware"]
        IdemStore[("Idempotency Key Store<br/>(Redis / low-latency KV,<br/>with TTL)")]
    end

    subgraph Backend["Backend Service"]
        BusinessLogic["Business Logic<br/>(e.g., charge payment,<br/>create order)"]
        DB[("Application Database")]
    end

    Client -->|"Request + Idempotency-Key header"| Gateway
    Gateway --> IdemMiddleware
    IdemMiddleware <--> IdemStore
    IdemMiddleware -->|"New request — proceed"| BusinessLogic
    IdemMiddleware -->|"Duplicate — return cached response"| Client
    BusinessLogic --> DB
    BusinessLogic --> IdemMiddleware
```

**Key idea:** The idempotency layer sits as **middleware** in front of business logic — its entire job is to answer one question before any side-effecting code runs: "have I already processed this exact request?" If yes, it short-circuits and returns the original result without touching business logic again. If no, it lets the request through and durably records the outcome once complete.

---

## 3. Data Model

```mermaid
erDiagram
    IDEMPOTENCY_RECORD {
        string idempotency_key PK
        string request_fingerprint "hash of method+path+body"
        string status "in_progress/completed/failed"
        int response_status_code
        string response_body
        timestamp created_at
        timestamp expires_at
    }
```

**Key design point — the `request_fingerprint` field:** Storing a hash of the request's method/path/body alongside the key allows the system to detect a client error — reusing the same idempotency key for a genuinely *different* request (e.g., different amount) — and reject it as a conflict, rather than silently returning the wrong cached response.

---

## 4. Idempotent Request Flow — Detailed Sequence (Happy Path)

```mermaid
sequenceDiagram
    participant C as Client
    participant MW as Idempotency Middleware
    participant Store as Idempotency Store
    participant BL as Business Logic
    participant DB as Database

    C->>MW: POST /charge<br/>Idempotency-Key: abc-123<br/>{amount: 50}

    MW->>Store: GET abc-123
    Store-->>MW: Not found (first attempt)

    MW->>Store: SET abc-123 = {status: IN_PROGRESS,<br/>fingerprint: hash(request)}<br/>(atomic SETNX — only if not exists)
    Store-->>MW: Lock acquired

    MW->>BL: Proceed with charge
    BL->>DB: Execute charge, create order record
    DB-->>BL: Success
    BL-->>MW: Result: {status: 200, order_id: X}

    MW->>Store: UPDATE abc-123 = {status: COMPLETED,<br/>response: {status:200, order_id:X}}
    MW-->>C: Return response (200, order_id: X)
```

---

## 5. Retry Flow — Same Request, Already Completed

```mermaid
sequenceDiagram
    participant C as Client (retrying after timeout)
    participant MW as Idempotency Middleware
    participant Store as Idempotency Store
    participant BL as Business Logic

    C->>MW: POST /charge<br/>Idempotency-Key: abc-123<br/>{amount: 50}<br/>(retry — original response was lost<br/>due to network timeout)

    MW->>Store: GET abc-123
    Store-->>MW: Found: {status: COMPLETED,<br/>response: {status:200, order_id:X}}

    MW->>MW: Verify request_fingerprint matches<br/>(same amount, same endpoint)

    Note over MW,BL: Business logic is NEVER called —<br/>charge is not processed again

    MW-->>C: Return cached response<br/>(200, order_id: X)<br/>— identical to original
```

---

## 6. Concurrent Retry Race Condition — Detailed Handling

```mermaid
sequenceDiagram
    participant C1 as Request Thread 1
    participant C2 as Request Thread 2 (near-simultaneous)
    participant Store as Idempotency Store
    participant BL as Business Logic

    Note over C1,C2: Two requests with the SAME<br/>idempotency key arrive<br/>almost simultaneously<br/>(e.g., client retried too fast,<br/>original still processing)

    C1->>Store: SETNX abc-123 = IN_PROGRESS
    Store-->>C1: Success (this thread won the race)

    C2->>Store: SETNX abc-123 = IN_PROGRESS
    Store-->>C2: Failed — key already exists<br/>(atomic operation, only one winner)

    C2->>C2: Detected concurrent in-flight request
    C2->>C2: Poll/wait briefly for C1 to complete<br/>(short backoff retry loop)

    C1->>BL: Proceed with actual operation
    BL-->>C1: Result
    C1->>Store: UPDATE abc-123 = COMPLETED + response

    C2->>Store: GET abc-123 (retry poll)
    Store-->>C2: COMPLETED + response
    C2-->>C2: Return same response as C1<br/>— never executed business logic itself
```

**Why atomic SETNX (SET if Not eXists) is essential:** Without an atomic "claim this key" operation, both concurrent threads could each check "does this key exist?", both see "no," and both proceed to execute the business logic — defeating the entire purpose of idempotency. The atomicity of SETNX in Redis (or an equivalent DB unique-constraint insert) is what makes exactly one thread the legitimate "winner" for actually executing the operation.

---

## 7. Handling a Conflicting Reuse of an Idempotency Key

```mermaid
flowchart TB
    A["Request arrives with<br/>Idempotency-Key: abc-123"] --> B["Lookup existing record"]
    B --> C{"Record exists?"}
    C -- No --> D["Proceed as new request"]
    C -- Yes --> E["Compare request_fingerprint<br/>of incoming request vs stored"]
    E --> F{"Fingerprints match?"}
    F -- Yes --> G["Legitimate retry —<br/>return cached response"]
    F -- No --> H["CONFLICT — same key,<br/>different request body/params"]
    H --> I["Return 409 Conflict —<br/>client error: key reused<br/>for a different operation"]
```

*This protects against a client bug (or misuse) where the same idempotency key is accidentally reused for what's actually a different logical request — silently returning the wrong cached result would be far worse than an explicit error.*

---

## 8. Distributed Idempotency Across Multiple Services

```mermaid
flowchart TB
    A["Client request:<br/>'Place Order'<br/>Idempotency-Key: xyz-789"] --> B["API Gateway<br/>(idempotency check #1)"]
    B --> C{"Already processed<br/>at gateway level?"}
    C -- Yes --> D["Return cached final response"]
    C -- No --> E["Order Service"]

    E --> F["Order Service internally calls:<br/>Payment Service + Inventory Service"]
    F --> G["Payment Service<br/>(idempotency check #2,<br/>using DERIVED key:<br/>xyz-789:payment)"]
    F --> H["Inventory Service<br/>(idempotency check #3,<br/>using DERIVED key:<br/>xyz-789:inventory)"]

    G & H --> I["Each downstream service<br/>maintains ITS OWN idempotency<br/>record, keyed off a deterministic<br/>derivation of the original key"]
```

**Why derived keys per downstream call:** A single idempotency key can't be reused verbatim across multiple distinct downstream operations within the same logical request (charging payment vs reserving inventory are different operations) — each needs its own idempotency scope, but deterministically derived from the original client-supplied key so that a full end-to-end retry of "Place Order" correctly deduplicates at every layer.

---

## 9. Idempotency Key Expiration & Storage Management

```mermaid
flowchart TB
    A["Idempotency record created<br/>with TTL (e.g., 24 hours)"] --> B["Redis native TTL<br/>expiry mechanism"]
    B --> C["Record automatically removed<br/>after TTL — no manual<br/>cleanup job needed"]

    D["Why 24 hours?"] --> E["Long enough to cover realistic<br/>client retry windows<br/>(network issues, client crashes<br/>and resumes, etc.)"]
    D --> F["Short enough to bound<br/>storage growth to a<br/>manageable, predictable size"]

    G["After TTL expires"] --> H["A 'retry' with the same key<br/>would now be treated as a<br/>NEW request — acceptable tradeoff,<br/>since realistic retries happen<br/>within minutes/hours, not days"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Idempotent API HLD))
    Idempotency Middleware
      Intercepts mutating requests
      Checks/claims key atomically
      Short-circuits duplicates
    Idempotency Store
      Redis with TTL
      Atomic SETNX claiming
      Request fingerprint storage
    Business Logic
      Executed exactly once per key
      Never called for confirmed duplicates
    Derived Key Scheme
      Per-downstream-service scoping
      Deterministic derivation from client key
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Claiming mechanism | Atomic SETNX (set-if-not-exists) | The only way to guarantee exactly one concurrent request "wins" and executes business logic; without atomicity, race conditions defeat the entire purpose |
| Storage | Redis (or DB with unique constraint) with TTL | Low-latency reads/writes on the hot path; TTL bounds storage growth automatically without manual cleanup |
| Conflict detection | Store request fingerprint, compare on reuse | Protects against client bugs reusing a key for a genuinely different request — fails loudly (409) rather than silently returning wrong data |
| Concurrent retry handling | Losing thread polls/waits for winner's result | Ensures both concurrent requests ultimately return the identical, correct response rather than one succeeding and one erroring |
| Multi-service idempotency | Deterministically derived keys per downstream call | A single client key can't directly serve multiple distinct downstream operations within one logical request; derivation preserves end-to-end retry safety |
| Key expiration | ~24 hour TTL | Balances realistic retry window coverage against unbounded storage growth |

---

## 12. Bottlenecks & Scaling Considerations

- **Idempotency store as a new critical dependency** — every single mutating request now depends on this store's availability and latency; it must be highly available and fast, similar to the criticality of a rate limiter's shared state store.
- **Long-running operations under lock** — if the "winning" request's business logic takes a long time, concurrent retries polling for its result add latency; consider a reasonable poll timeout with a clear error response rather than blocking indefinitely.
- **Storage sizing at scale** — with 100,000 req/sec and a 24-hour TTL, the idempotency store must comfortably hold a full day's worth of key records — needs capacity planning proportional to peak sustained traffic, not average.
- **Fingerprint collision risk** — using a hash for the request fingerprint (rather than storing the full request body) is more storage-efficient but introduces a theoretical hash collision risk; in practice negligible with a strong hash function (e.g., SHA-256), but worth being deliberate about.
- **Client key generation quality** — the entire system's safety depends on clients generating genuinely unique keys per logical operation (e.g., UUID per checkout attempt, not per page load); poor client-side key generation (e.g., reusing a key across genuinely different requests) undermines the guarantee no matter how well the server-side is built.
- **Cross-region idempotency** — for globally distributed systems, ensuring the same idempotency key check is consistent across regions requires either a globally-replicated store (added latency) or accepting region-scoped idempotency guarantees as a deliberate, documented limitation.
- **Failure during business logic execution** — if the business logic crashes mid-execution after claiming the key but before completing, the key must not remain stuck in `IN_PROGRESS` forever; needs a secondary shorter timeout on the `IN_PROGRESS` state itself, distinct from the overall record TTL, to allow eventual retry after a genuine failure.
