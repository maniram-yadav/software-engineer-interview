# Design a Distributed Saga Orchestrator for Long-Running Business Workflows — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Coordinate multi-step business workflows spanning many services, where individual steps may take minutes to DAYS to complete (e.g., "onboard a new enterprise customer": provisioning, legal review, payment setup, welcome sequence)
- Support workflows that pause waiting for EXTERNAL events (human approval, a third-party webhook, a scheduled delay) not just synchronous service calls
- Recover and resume in-progress workflows correctly after orchestrator restarts/crashes
- Provide visibility into exactly where any given workflow instance currently stands

### Non-Functional Requirements
- **Durability across arbitrarily long timeframes:** Unlike the earlier Distributed Transaction Saga design's typically-fast checkout flow, this must correctly handle workflows genuinely spanning days or weeks
- **Correct compensation on failure:** Same core requirement as the earlier saga design — failed workflows must be cleanly unwound
- **Scalability:** Must manage potentially millions of concurrent long-running workflow instances simultaneously
- **Auditability:** Given the business-critical, long-running nature of these workflows, complete history of every step/decision must be available

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Concurrent active workflow instances | Millions |
| Workflow duration | Minutes to weeks |
| Steps per workflow | 5-50 typical |
| Orchestrator restart frequency | Regular (deployments), must never lose workflow progress |

---

## 2. How This Differs From the Earlier Distributed Transaction Saga Design

```mermaid
flowchart TB
    A["Earlier Distributed Transaction<br/>Saga design (e.g., 'place<br/>order'): workflow completes<br/>in SECONDS, orchestrator<br/>holds the ENTIRE workflow's<br/>execution in memory/process<br/>for its short lifetime"] --> A1["Appropriate for fast,<br/>synchronous-feeling business<br/>operations"]

    B["THIS design: workflows span<br/>MINUTES TO WEEKS, frequently<br/>WAITING on external events<br/>(human approval, scheduled<br/>delays, third-party callbacks)<br/>— the orchestrator CANNOT<br/>hold these in active memory<br/>the whole time; it must be<br/>able to fully SUSPEND and<br/>later RESUME each workflow<br/>instance, potentially across<br/>MULTIPLE orchestrator process<br/>restarts during the<br/>workflow's lifetime"] --> B1["This durability-across-<br/>suspension requirement is<br/>the primary NEW architectural<br/>challenge this design adds<br/>beyond the earlier, faster<br/>saga pattern"]
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    Client["Workflow Trigger<br/>(e.g., new customer signup)"]

    subgraph Orchestration["Workflow Orchestration Engine"]
        WorkflowEngine["Workflow Execution Engine"]
        WorkflowDefRegistry[("Workflow Definition Registry<br/>— declarative step sequences")]
    end

    subgraph Persistence["Durable Workflow State"]
        WorkflowState[("Workflow Instance State<br/>— current step, variables,<br/>full history")]
        Timers[("Scheduled Timer Store<br/>— for delays/timeouts")]
    end

    subgraph ExternalTriggers["External Event Sources"]
        HumanApproval["Human Approval UI"]
        Webhook["Third-Party Webhooks"]
        ScheduledTimer["Timer Expiration"]
    end

    subgraph Services["Participating Services"]
        Provisioning["Provisioning Service"]
        Legal["Legal Review Service"]
        Payment["Payment Setup Service"]
    end

    Client --> WorkflowEngine
    WorkflowEngine --> WorkflowDefRegistry
    WorkflowEngine <--> WorkflowState
    WorkflowEngine <--> Timers

    WorkflowEngine --> Provisioning
    WorkflowEngine --> Legal
    WorkflowEngine --> Payment

    HumanApproval --> WorkflowEngine
    Webhook --> WorkflowEngine
    ScheduledTimer --> WorkflowEngine
```

**Key idea:** Every meaningful state change in a workflow instance's progress is DURABLY persisted BEFORE the engine considers that step complete — this means the in-memory workflow engine process is fundamentally STATELESS with respect to any individual workflow instance's progress; any engine instance can pick up and continue ANY workflow instance at ANY time, purely by reading its current durable state, which is what makes suspension across process restarts (even weeks apart) work correctly.

---

## 4. Data Model

```mermaid
erDiagram
    WORKFLOW_DEFINITION {
        string workflow_type PK
        string version
        list step_definitions
    }
    WORKFLOW_INSTANCE {
        string instance_id PK
        string workflow_type FK
        string current_step
        string status "running/waiting/completed/failed/compensating"
        map workflow_variables
        timestamp started_at
        timestamp last_updated_at
    }
    STEP_EXECUTION_HISTORY {
        string instance_id FK
        string step_name
        string status "completed/failed/compensated"
        map step_result
        timestamp executed_at
    }
    PENDING_TIMER {
        string timer_id PK
        string instance_id FK
        timestamp fires_at
        string resume_step
    }
```

---

## 5. Workflow Execution Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Trigger as Signup Event
    participant Engine as Workflow Engine
    participant WorkflowState as Workflow State Store
    participant Provisioning as Provisioning Service

    Trigger->>Engine: Start workflow:<br/>customer_onboarding,<br/>{customer_id: X}

    Engine->>WorkflowState: Create WORKFLOW_INSTANCE<br/>{status: RUNNING,<br/>current_step: "provision_account"}

    Engine->>Provisioning: Execute step:<br/>provision_account

    alt Step succeeds
        Provisioning-->>Engine: Success, {account_id: Y}
        Engine->>WorkflowState: Record step COMPLETED,<br/>update current_step to NEXT<br/>step in the workflow<br/>definition
    else Step fails
        Engine->>WorkflowState: Record step FAILED
        Engine->>Engine: Begin compensation<br/>(same reverse-order<br/>rollback pattern as the<br/>Distributed Transaction<br/>Saga design)
    end
```

---

## 6. Suspending on an External Event (Human Approval) — Detailed Sequence

```mermaid
sequenceDiagram
    participant Engine as Workflow Engine
    participant WorkflowState as Workflow State Store
    participant Approver as Human Approver

    Engine->>WorkflowState: Reach step:<br/>"await_legal_approval"
    Engine->>WorkflowState: Update instance status<br/>= WAITING, current_step<br/>= "await_legal_approval"

    Note over Engine: Engine process is now<br/>COMPLETELY FREE to work on<br/>OTHER workflow instances —<br/>it does NOT hold this<br/>instance in memory or block<br/>on this step at all

    Engine->>Approver: Send notification:<br/>"Please review customer X"

    Note over Approver: DAYS may pass —<br/>engine process might<br/>RESTART multiple times<br/>during this wait, entirely<br/>irrelevant to this workflow<br/>instance's eventual progress

    Approver->>Engine: Submit approval decision<br/>{instance_id: ABC,<br/>decision: APPROVED}

    Engine->>WorkflowState: Load CURRENT state for<br/>instance ABC<br/>(regardless of which engine<br/>process instance handles<br/>this — any instance can<br/>resume ANY workflow, since<br/>state is fully externalized)

    WorkflowState-->>Engine: {current_step:<br/>"await_legal_approval",<br/>status: WAITING}

    Engine->>Engine: Resume execution from<br/>this exact point — proceed<br/>to the NEXT step in the<br/>workflow definition
    Engine->>WorkflowState: Update status = RUNNING,<br/>current_step = next step
```

**Why this suspend/resume capability is the defining technical achievement of this design:** The workflow engine holds ZERO in-memory state for a waiting instance — everything needed to resume is durably externalized. This means the engine can be redeployed, scaled up/down, or crash and restart any number of times during a multi-day wait, and the workflow resumes correctly exactly where it left off, driven entirely by its durable state rather than any particular process's memory or lifetime.

---

## 7. Timer-Based Delays (Scheduled Resumption)

```mermaid
sequenceDiagram
    participant Engine as Workflow Engine
    participant WorkflowState as Workflow State
    participant TimerStore as Pending Timer Store
    participant TimerSweeper as Timer Sweeper<br/>(scheduled background process)

    Engine->>WorkflowState: Reach step:<br/>"wait_3_days_then_send_followup"
    Engine->>TimerStore: Create timer:<br/>{instance_id: ABC,<br/>fires_at: now+3days,<br/>resume_step: "send_followup"}
    Engine->>WorkflowState: Update status = WAITING

    Note over TimerSweeper: 3 days later...

    loop Periodic sweep (same pattern<br/>as the Distributed Job<br/>Scheduler design)
        TimerSweeper->>TimerStore: Find timers WHERE<br/>fires_at <= now()
        TimerStore-->>TimerSweeper: Timer for instance ABC<br/>has fired

        TimerSweeper->>Engine: Resume instance ABC<br/>at step "send_followup"
        Engine->>WorkflowState: Update status = RUNNING,<br/>continue execution
    end
```

**Why this reuses the same fundamental pattern as the Distributed Job Scheduler design:** A workflow "waiting 3 days" is conceptually identical to scheduling a delayed job — this design deliberately builds on the same durable, TTL-based, periodically-swept scheduling mechanism established in that dedicated design, rather than inventing a parallel timer mechanism.

---

## 8. Compensation for Long-Running Workflows

```mermaid
sequenceDiagram
    participant Engine as Workflow Engine
    participant WorkflowState as Workflow State
    participant Legal as Legal Service
    participant Provisioning as Provisioning Service
    participant Payment as Payment Service

    Note over Engine: Workflow reaches<br/>"payment_setup" step,<br/>which FAILS<br/>(e.g., invalid payment method)

    Engine->>WorkflowState: Record FAILED,<br/>begin COMPENSATING

    Note over Engine: Must undo completed steps<br/>in REVERSE order — but this<br/>might mean undoing actions<br/>taken DAYS ago, potentially<br/>by a completely different<br/>engine process instance

    Engine->>WorkflowState: Query STEP_EXECUTION_HISTORY<br/>for this instance —<br/>determine exactly which<br/>steps actually completed
    WorkflowState-->>Engine: [provision_account: COMPLETED,<br/>legal_approval: COMPLETED]

    Engine->>Legal: Compensate: revoke<br/>legal approval status
    Engine->>Provisioning: Compensate: deprovision<br/>the account

    Engine->>WorkflowState: Mark all steps COMPENSATED,<br/>instance status = FAILED
```

**Why the durable execution history is essential for correct compensation:** Because a long-running workflow's completed steps might have happened days or weeks before a LATER step's failure, the compensation logic can't rely on any in-memory record of "what we did" — it must reconstruct exactly which steps genuinely completed by querying the durable STEP_EXECUTION_HISTORY, ensuring compensation is accurate even if the compensating engine process is entirely different from whichever process executed the original steps.

---

## 9. Workflow Versioning (Handling In-Flight Instances During Definition Changes)

```mermaid
flowchart TB
    A["Workflow definition for<br/>'customer_onboarding' is<br/>UPDATED (e.g., a new step<br/>added) while THOUSANDS of<br/>instances are currently<br/>mid-execution under the<br/>OLD definition"] --> B{"Versioning Strategy"}

    B --> C["In-flight instances continue<br/>executing under the<br/>DEFINITION VERSION they<br/>STARTED with — the<br/>WORKFLOW_INSTANCE record<br/>captures which version it's<br/>bound to at creation time"]

    B --> D["NEW workflow instances,<br/>started AFTER the definition<br/>update, use the NEW version"]

    C & D --> E["This prevents the chaos of<br/>an in-flight instance<br/>suddenly encountering a<br/>DIFFERENT step sequence<br/>mid-execution than what it<br/>started with — same<br/>principle as NOT retroactively<br/>changing a database schema<br/>migration already in<br/>progress"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Long-Running Saga Orchestrator HLD))
    Workflow Execution Engine
      Stateless with respect to instances
      Any process can resume any instance
    Workflow Definition Registry
      Declarative step sequences
      Versioned for in-flight compatibility
    Workflow Instance State
      Durable current position
      Complete execution history
    Pending Timer Store
      Scheduled resumption
      Same pattern as Job Scheduler design
    External Event Handlers
      Human approval, webhooks
      Resume execution on arrival
    Compensation Logic
      Reverse-order rollback
      Driven by durable history, not memory
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Engine statefulness | Fully stateless with respect to individual workflow instances | Enables any engine process to resume any workflow instance at any time, essential for correctness across restarts during multi-day/week waits |
| State externalization | Every meaningful progress point durably persisted before considered complete | Makes suspend/resume across arbitrary time spans and process restarts fundamentally correct, not just "usually works" |
| Delay/timer mechanism | Reuses the Distributed Job Scheduler design's pattern | A workflow "waiting N days" is conceptually identical to a delayed job — no need for a parallel, separate timer mechanism |
| Compensation logic | Driven by durable execution history, not in-memory tracking | Ensures correct rollback even when the compensating process differs entirely from whichever process executed the original steps |
| Workflow versioning | In-flight instances bound to their starting definition version | Prevents chaos from a definition change altering the step sequence for already-executing instances |

---

## 12. Bottlenecks & Scaling Considerations

- **Workflow state store as a critical, high-cardinality dependency** — with potentially millions of concurrent instances, this store faces both high write volume (every step transition) and high read volume (resuming instances); needs the same sharding/partitioning considerations as other high-cardinality stores covered in prior designs (e.g., by instance_id hash).
- **Timer sweep scalability at massive scale** — periodically scanning for "which of potentially millions of timers have fired" needs an efficient time-indexed query structure, same fundamental scaling concern as the Distributed Job Scheduler design's due-job scanning.
- **Long-running workflow observability** — given multi-week workflow durations, operators need robust tooling to answer "where is instance X right now, and why has it been stuck at this step for 3 days" — this requires purpose-built dashboarding beyond simple state-store queries, especially for diagnosing workflows that are stuck due to a genuine bug rather than a legitimate long wait.
- **External event correlation at scale** — matching an incoming webhook or approval submission back to the CORRECT waiting workflow instance (among potentially millions of concurrently waiting instances) requires efficient, well-indexed lookup by whatever correlation identifier the external system provides.
- **Compensation complexity for very long chains** — a workflow that failed after 40 completed steps requires compensating ALL 40 in reverse order; some of these compensating actions may themselves fail (e.g., trying to deprovision a resource that was already manually modified by a human during the multi-week window) — needs robust partial-compensation-failure handling and likely human escalation paths for cases automated rollback can't cleanly resolve.
- **Workflow definition testing** — given that a single definition change affects potentially millions of NEW instances going forward while millions of EXISTING instances continue under old versions, thorough testing must validate correctness for the new definition WITHOUT assuming clean-slate deployment — a genuinely more complex testing surface than typical stateless service deployments.
- **Cross-workflow dependencies** — real business processes sometimes require ONE long-running workflow to trigger or wait on ANOTHER (e.g., customer onboarding waiting on a separate compliance review workflow); this composition of workflows-within-workflows adds a meaningful additional layer of orchestration complexity beyond the single-workflow model covered in this design's core architecture.
