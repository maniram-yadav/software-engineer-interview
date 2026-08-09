# Design a GDPR-Compliant Data Deletion System Across Microservices — High-Level Design Document

## 1. Requirements

### Functional Requirements
- When a user exercises their "right to be forgotten," permanently delete their personal data across EVERY service that stores it
- Provide verifiable confirmation that deletion actually completed everywhere, not just "request submitted"
- Handle data that's replicated across caches, backups, logs, analytics warehouses, and third-party processors
- Support legally-required exceptions (e.g., data that must be retained for financial/legal compliance despite a deletion request)

### Non-Functional Requirements
- **Completeness (paramount):** Missing even ONE service that retains the user's data is a compliance failure — this isn't a "best effort" system
- **Auditability:** Must produce evidence of what was deleted, when, and confirmation it propagated everywhere — regulators can and do ask for this
- **Bounded time:** GDPR specifies deletion must complete "without undue delay," generally interpreted as within 30 days
- **Correctness under partial failure:** If one downstream service is temporarily unavailable, the request must not be silently dropped — it must be retried until confirmed complete

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Deletion requests/day | Hundreds to low thousands (varies heavily by platform size/region) |
| Services storing personal data | Dozens across a typical microservices platform |
| Time to complete a request | Target: hours to low days, legal maximum: 30 days |
| Data locations per user | Operational DBs, caches, search indexes, analytics warehouse, logs, backups, third-party processors |

---

## 2. The Core Challenge — Data Sprawls Far Beyond the "Obvious" Database

```mermaid
flowchart TB
    A["User's personal data<br/>doesn't live in just ONE place"] --> B["Primary operational databases<br/>(the 'obvious' location)"]
    A --> C["Derived caches<br/>(Redis, CDN-cached responses<br/>containing user data)"]
    A --> D["Search indexes<br/>(user profile indexed<br/>for search)"]
    A --> E["Analytics warehouse<br/>(user behavior data,<br/>potentially retained<br/>for years)"]
    A --> F["Application/access logs<br/>(user IDs, IPs, possibly<br/>PII in log lines)"]
    A --> G["Backups<br/>(point-in-time snapshots<br/>containing the user's data<br/>as it existed at backup time)"]
    A --> H["Third-party processors<br/>(email service providers,<br/>analytics vendors, support<br/>ticket systems)"]

    I["A deletion system that only<br/>handles the primary database<br/>is NOT actually GDPR compliant —<br/>this comprehensive scope is<br/>THE defining challenge"] --> H
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    User["User Requests Deletion<br/>(via privacy portal/support)"]

    subgraph Orchestration["Deletion Orchestration Layer"]
        DeletionSvc["Deletion Request Service"]
        DeletionLog[("Deletion Request Log<br/>— durable, source of truth<br/>for the whole request lifecycle")]
        Registry[("Data Location Registry<br/>— which services/systems<br/>hold personal data")]
    end

    subgraph Fanout["Fan-Out Execution"]
        Kafka["Kafka<br/>(DeletionRequested events)"]
        ServiceHandlers["Per-Service Deletion Handlers"]
    end

    subgraph DataStores["Data Locations"]
        OpDB[("Operational DBs")]
        Cache[("Caches")]
        Search[("Search Index")]
        Analytics[("Analytics Warehouse")]
        Backups[("Backup Snapshots")]
        ThirdParty["Third-Party Processors"]
    end

    ComplianceDash["Compliance Verification<br/>Dashboard"]

    User --> DeletionSvc
    DeletionSvc --> DeletionLog
    DeletionSvc --> Registry
    DeletionSvc --> Kafka

    Kafka --> ServiceHandlers
    ServiceHandlers --> OpDB
    ServiceHandlers --> Cache
    ServiceHandlers --> Search
    ServiceHandlers --> Analytics
    ServiceHandlers --> Backups
    ServiceHandlers --> ThirdParty

    ServiceHandlers -->|"confirm completion"| DeletionLog
    DeletionLog --> ComplianceDash
```

**Key idea:** This is fundamentally a **saga-style orchestrated workflow** (same pattern as the Distributed Transaction Saga design) — but instead of "all-or-nothing with compensation on failure," the goal is "eventually ALL services confirm completion, with persistent retry until they do." The Data Location Registry is the critical piece of institutional knowledge that makes "every place this data lives" a known, queryable fact rather than tribal knowledge scattered across teams.

---

## 4. Data Model

```mermaid
erDiagram
    DELETION_REQUEST ||--o{ SERVICE_DELETION_TASK : "requires completion from"

    DELETION_REQUEST {
        string request_id PK
        string user_id FK
        string status "in_progress/completed/partially_blocked"
        timestamp requested_at
        timestamp legal_deadline
        timestamp completed_at
    }
    SERVICE_DELETION_TASK {
        string task_id PK
        string request_id FK
        string service_name
        string status "pending/completed/failed/exempted"
        string exemption_reason "nullable — e.g. legal retention requirement"
        timestamp completed_at
        int retry_count
    }
    DATA_LOCATION_REGISTRY_ENTRY {
        string service_name PK
        string data_types_stored
        string deletion_endpoint
        bool has_legal_retention_exception
    }
```

---

## 5. Deletion Request Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant User as User
    participant DelSvc as Deletion Service
    participant Log as Deletion Request Log
    participant Registry as Data Location Registry
    participant K as Kafka
    participant Handlers as Service Handlers (many)

    User->>DelSvc: Request deletion of my data
    DelSvc->>Log: Create DELETION_REQUEST<br/>{user_id, status=IN_PROGRESS,<br/>legal_deadline=+30 days}

    DelSvc->>Registry: Query: which services<br/>store data for this user type?
    Registry-->>DelSvc: List of ~30 services<br/>with deletion endpoints

    loop For each registered service
        DelSvc->>Log: Create SERVICE_DELETION_TASK<br/>{service_name, status=PENDING}
    end

    DelSvc->>K: Publish DeletionRequested event<br/>{user_id, request_id}

    K->>Handlers: Fan out to ALL<br/>subscribed service handlers<br/>(each service owns its<br/>OWN deletion logic)

    DelSvc-->>User: Request accepted,<br/>tracking_id provided<br/>(NOT "completed" —<br/>this will take time)
```

---

## 6. Per-Service Deletion Handling — Detailed Sequence

```mermaid
sequenceDiagram
    participant K as Kafka
    participant Handler as Order Service<br/>Deletion Handler
    participant DB as Order Service DB
    participant Log as Deletion Request Log

    K->>Handler: Consume DeletionRequested<br/>{user_id, request_id}

    Handler->>Handler: Check: does this service<br/>have a LEGAL RETENTION<br/>EXCEPTION for this data?<br/>(e.g., financial records<br/>often must be retained<br/>for tax/audit purposes<br/>DESPITE a deletion request)

    alt Legal exception applies
        Handler->>Handler: ANONYMIZE instead of delete<br/>(e.g., replace user_id with<br/>a random token, strip<br/>directly-identifying fields,<br/>but retain the financial<br/>record structure required<br/>by law)
        Handler->>Log: Report task status=COMPLETED,<br/>exemption_reason="7-year<br/>financial retention requirement"
    else No exception — full deletion required
        Handler->>DB: DELETE all records<br/>WHERE user_id = X<br/>(hard delete, not soft delete —<br/>GDPR requires the data to<br/>be genuinely unrecoverable,<br/>not just hidden)
        DB-->>Handler: Confirmed deleted
        Handler->>Log: Report task status=COMPLETED
    end
```

**Why anonymization is a legitimate, important alternative to deletion:** GDPR itself recognizes that some data must be retained for OTHER legal obligations (tax law, financial audit requirements) that can conflict with the right to erasure. The correct handling isn't to ignore the deletion request, but to STRIP the personally-identifying elements while retaining the legally-required record in an anonymized form — this must be an explicit, documented, and consistently-applied policy per data type, not an ad-hoc judgment call made differently by different teams.

---

## 7. Handling Retries and Partial Failures

```mermaid
sequenceDiagram
    participant Handler as Service Deletion Handler
    participant DB as Service Database
    participant Log as Deletion Request Log
    participant Retrier as Retry Monitor

    Handler->>DB: Attempt deletion
    Note over DB: Database temporarily<br/>unavailable
    DB--xHandler: Failure/timeout

    Handler->>Log: Report task status=FAILED,<br/>retry_count+1

    loop Retry Monitor, periodic sweep
        Retrier->>Log: Query tasks WHERE<br/>status=FAILED AND<br/>retry_count < max_retries
        Log-->>Retrier: List of failed tasks

        Retrier->>Handler: Re-trigger deletion attempt<br/>(idempotent — same pattern<br/>as the Idempotent API<br/>Requests design; deleting<br/>an already-deleted or<br/>never-existed record is<br/>safely a no-op)
    end

    Note over Retrier: If retries exhausted<br/>AND legal_deadline approaching,<br/>ESCALATE to on-call/compliance<br/>team — this cannot simply<br/>be silently abandoned
```

**Why idempotency matters here specifically:** A "delete user's data" operation is naturally idempotent (deleting something already deleted is a safe no-op), which makes aggressive, persistent retry a safe strategy — unlike operations where retrying could cause harmful duplication, this system can and should retry relentlessly until every service confirms completion, without special-casing "was this already done?" logic.

---

## 8. Handling Backups (The Hardest Data Location)

```mermaid
flowchart TB
    A["User's data exists in a<br/>backup snapshot taken<br/>BEFORE their deletion request"] --> B["Backups are typically<br/>IMMUTABLE, point-in-time<br/>snapshots — you cannot<br/>selectively edit ONE user's<br/>data out of an existing<br/>backup file"]

    B --> C{"Backup Handling Strategy"}
    C --> D["Wait for natural backup<br/>expiration/rotation<br/>(e.g., 30-90 day retention<br/>policy) — the deletion<br/>request is considered<br/>fulfilled once ALL backups<br/>containing the data have<br/>naturally rotated out"]
    C --> E["Maintain a 'deletion<br/>manifest' — if a backup<br/>ever needs to be RESTORED,<br/>immediately re-apply all<br/>pending deletion requests<br/>against the restored data<br/>before it becomes live again"]

    F["This is a well-established,<br/>GDPR-recognized approach —<br/>regulators generally accept<br/>that backups have a bounded<br/>retention window rather than<br/>requiring real-time backup<br/>editing, AS LONG AS this<br/>policy is documented and<br/>consistently enforced"] -.-> D
```

---

## 9. Compliance Verification & Reporting

```mermaid
sequenceDiagram
    participant Compliance as Compliance Officer
    participant Dashboard as Compliance Dashboard
    participant Log as Deletion Request Log

    Compliance->>Dashboard: Query request status<br/>for user_id=X (e.g., in<br/>response to a regulator<br/>audit or user follow-up)

    Dashboard->>Log: Fetch DELETION_REQUEST<br/>+ all SERVICE_DELETION_TASKs

    Log-->>Dashboard: Full task breakdown:<br/>28 services COMPLETED,<br/>2 services EXEMPTED<br/>(with documented legal reason)

    Dashboard-->>Compliance: Present verifiable,<br/>auditable completion report —<br/>NOT just "request submitted"

    Note over Dashboard: This report itself becomes<br/>part of the audit trail —<br/>demonstrating genuine,<br/>systematic compliance rather<br/>than a best-effort attempt
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((GDPR Deletion HLD))
    Deletion Request Service
      Entry point, orchestration
      Creates trackable request
    Data Location Registry
      Comprehensive service inventory
      Prevents missed data locations
    Deletion Request Log
      Durable source of truth
      Per-service task tracking
    Service Deletion Handlers
      Per-service deletion logic
      Legal exception handling
    Retry Monitor
      Persistent, idempotent retry
      Escalation on exhaustion
    Compliance Dashboard
      Verifiable completion reporting
      Audit trail for regulators
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Orchestration model | Saga-style, fan-out with persistent tracking | Deletion must eventually reach EVERY service, not just attempt once — this requires durable tracking of per-service completion, not fire-and-forget |
| Legal exceptions | Anonymization instead of deletion, explicitly documented | GDPR itself recognizes conflicting legal retention requirements; the correct response is documented anonymization, not ignoring the request |
| Retry strategy | Persistent, idempotent retry until confirmed complete | Deletion is naturally idempotent, making aggressive retry safe; silent abandonment of a failed deletion task is a compliance failure |
| Backup handling | Bounded retention window + deletion manifest for restores | Real-time backup editing is generally infeasible; this is a well-established, regulator-accepted alternative approach |
| Verification | Explicit compliance dashboard with per-service audit trail | "Request submitted" is not equivalent to "deletion completed" — genuine compliance requires provable, granular completion evidence |
| Data location tracking | Central, actively-maintained registry | Without this, "did we miss a service" becomes tribal knowledge rather than a systematically verifiable fact |

---

## 12. Bottlenecks & Scaling Considerations

- **Registry staleness is a genuine compliance risk** — if a new microservice starts storing personal data but isn't registered in the Data Location Registry, deletion requests will silently miss it; this requires organizational process (not just technical design) — e.g., a mandatory registry-update step in any new service's launch checklist, not purely an engineering solution.
- **Third-party processor deletion isn't fully within the platform's control** — services like email providers or analytics vendors require their OWN deletion API calls, and their completion confirmation depends on THEIR reliability, not just the platform's own infrastructure; this requires the same retry/tracking rigor applied to internal services, extended to external dependencies.
- **Legal deadline monitoring** — the 30-day (or jurisdiction-specific) legal deadline needs proactive monitoring with escalation BEFORE it's breached, not reactive discovery after the fact; requires the same alerting discipline as any other SLA-bound business process.
- **Cross-request interactions** — if a user submits a NEW data-generating action (e.g., places an order) WHILE a deletion request for their PREVIOUS data is still in progress, the system needs clear semantics for how these interact, rather than allowing a race condition that could either incorrectly delete new legitimate data or leave old data undeleted.
- **Testing completeness is uniquely hard to verify** — unlike most systems where correctness can be tested against a known specification, "did we successfully identify and delete from EVERY location personal data could exist" is fundamentally difficult to exhaustively verify — this argues for periodic manual compliance audits/penetration-testing-style reviews as a complement to automated testing, not a replacement for it.
- **Volume scaling during regulatory events** — a sudden surge in deletion requests (e.g., following negative press or a new regulation taking effect) could create a burst far exceeding normal volume; the orchestration and per-service handler capacity should be designed to absorb such spikes without breaching the legal completion deadline for the backlog.
