# Design a Live Chat Support System — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Customers can start a chat and be routed to an available support agent
- Real-time bidirectional messaging between customer and agent
- Presence indicators: online/offline, typing indicators
- Queueing when no agents are available, with position/wait-time estimate
- Chat history persisted and searchable
- Support handoff (transfer between agents, escalation to specialist)
- Agents can handle multiple concurrent chats

### Non-Functional Requirements
- **Scale:** ~100K concurrent chats at peak (e.g., large e-commerce platform)
- **Low latency:** Messages and typing indicators should feel instant (< 200ms)
- **Availability:** Chat must degrade gracefully — never lose a customer's queue position
- **Durability:** No message loss, even across agent reassignment or reconnects
- **Fairness:** Queueing and routing must be fair (FIFO within priority tiers, skill-based routing)

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Concurrent chats (peak) | ~100,000 |
| Concurrent agents online | ~5,000 |
| Avg chats per agent (concurrent) | ~3-5 |
| Messages/sec (platform-wide) | ~10,000 |
| Avg chat session duration | ~10-15 minutes |
| Typing indicator events/sec | High-frequency, ephemeral — much higher than message volume |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Customer["Customer<br/>(Web/Mobile Widget)"]
    Agent["Support Agent<br/>(Agent Console)"]

    subgraph Edge["Connection Layer"]
        LB["Load Balancer"]
        ConnServer["Connection Server<br/>(WebSocket, sticky routing)"]
    end

    subgraph Core["Core Services"]
        ChatSvc["Chat Session Service"]
        QueueSvc["Queue Management Service"]
        RoutingSvc["Routing/Matching Service<br/>(skill-based, load-aware)"]
        PresenceSvc["Presence Service"]
    end

    subgraph Storage["Storage Layer"]
        SessionStore[("Session/Connection Registry<br/>(Redis)")]
        QueueStore[("Queue State<br/>(Redis - priority queue per skill/team)")]
        ChatDB[("Chat History DB<br/>(Cassandra/Postgres)")]
        AgentStateStore[("Agent Availability &<br/>Capacity Store")]
    end

    Customer <-->|"WebSocket"| LB
    Agent <-->|"WebSocket"| LB
    LB <--> ConnServer

    ConnServer --> ChatSvc
    ChatSvc --> ChatDB
    ChatSvc --> SessionStore

    Customer -->|"Start chat request"| QueueSvc
    QueueSvc --> QueueStore
    QueueSvc --> RoutingSvc
    RoutingSvc --> AgentStateStore
    RoutingSvc -->|"Assign agent"| ChatSvc

    ConnServer --> PresenceSvc
    PresenceSvc --> AgentStateStore
```

**Key idea:** Chat routing is a two-phase problem: (1) a **Queue Service** holds waiting customers in priority order until an agent is available, then (2) a **Routing Service** matches the customer to the best available agent by skill/load — only after a match is made does the actual bidirectional chat session begin over the connection layer.

---

## 3. Data Model

```mermaid
erDiagram
    CUSTOMER ||--o{ CHAT_SESSION : initiates
    AGENT ||--o{ CHAT_SESSION : handles
    CHAT_SESSION ||--o{ MESSAGE : contains
    CHAT_SESSION ||--o{ TRANSFER_EVENT : "has (if transferred)"

    CUSTOMER {
        string customer_id PK
        string name
        string tier "priority level"
    }
    AGENT {
        string agent_id PK
        string name
        string status "online/busy/offline"
        int max_concurrent_chats
        int current_chat_count
        list skills
    }
    CHAT_SESSION {
        string session_id PK
        string customer_id FK
        string agent_id FK
        string status "queued/active/transferred/closed"
        string required_skill
        timestamp queued_at
        timestamp started_at
        timestamp ended_at
    }
    MESSAGE {
        string message_id PK
        string session_id FK
        string sender_id
        string sender_type "customer/agent"
        string text
        timestamp sent_at
    }
    TRANSFER_EVENT {
        string transfer_id PK
        string session_id FK
        string from_agent_id
        string to_agent_id
        string reason
        timestamp transferred_at
    }
```

---

## 4. Chat Initiation & Queueing Flow

```mermaid
sequenceDiagram
    participant C as Customer
    participant GW as API Gateway
    participant QS as Queue Service
    participant Q as Queue Store (Redis)
    participant RS as Routing Service
    participant AS as Agent State Store

    C->>GW: Start chat {topic, priority_tier}
    GW->>QS: Enqueue request
    QS->>Q: Push to priority queue<br/>(keyed by required_skill)
    QS-->>C: Queue position + estimated wait time

    loop Every few seconds
        RS->>AS: Check for available agents<br/>matching queue's skill requirements
        AS-->>RS: List of agents with capacity
        RS->>Q: Peek next customer in queue
        alt Match found
            RS->>Q: Pop customer from queue
            RS->>AS: Reserve agent slot<br/>(increment current_chat_count)
            RS->>QS: Create active chat session
            QS-->>C: Chat matched! Connect to agent
            QS-->>Agent: New chat assigned
        else No agent available
            RS->>RS: Wait, retry next cycle
        end
    end
```

**Key idea:** Queue position is never lost even if the customer's connection drops — the queue entry lives server-side in Redis, keyed by `customer_id`/`session_id`, so a reconnecting client simply re-attaches to its existing queue slot rather than losing its place in line.

---

## 5. Agent Routing / Matching Logic

```mermaid
flowchart TB
    A["Customer enters queue<br/>with required_skill = 'billing'"] --> B["Routing Service scans<br/>available agents"]
    B --> C{"Agent has<br/>'billing' skill?"}
    C -- No --> D["Skip"]
    C -- Yes --> E{"Agent under<br/>max_concurrent_chats?"}
    E -- No --> D
    E -- Yes --> F["Candidate agent"]
    F --> G["Rank candidates by:<br/>- Current load (fewer chats first)<br/>- Idle time (longest-idle first)<br/>- Performance/rating (optional)"]
    G --> H["Assign to top-ranked agent"]
    H --> I["Increment agent's<br/>current_chat_count"]
```

---

## 6. Real-Time Messaging Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Customer
    participant CS1 as Connection Server (Customer)
    participant ChatSvc as Chat Service
    participant DB as Chat DB
    participant CS2 as Connection Server (Agent)
    participant A as Agent

    C->>CS1: Send message
    CS1->>ChatSvc: Forward message
    ChatSvc->>DB: Persist message
    ChatSvc->>CS2: Route to agent's connection server
    CS2->>A: Push message
    A-->>CS2: Ack (message seen in console)

    A->>CS2: Agent replies
    CS2->>ChatSvc: Forward message
    ChatSvc->>DB: Persist message
    ChatSvc->>CS1: Route to customer's connection server
    CS1->>C: Push message
```

*Mirrors the same "session store maps user → connection server" pattern as a messaging system, but here the "conversation" is always exactly 1 customer : 1 (currently assigned) agent, simplifying the fanout compared to group chat.*

---

## 7. Typing Indicators & Presence

```mermaid
flowchart LR
    A["Customer starts typing"] --> B["Client sends ephemeral<br/>'typing' event<br/>(debounced, not persisted)"]
    B --> C["Connection Server"]
    C --> D["Forward directly to<br/>paired agent's connection<br/>(no DB write, no queue)"]
    D --> E["Agent console shows<br/>'Customer is typing...'"]

    F["No typing event for 3s"] --> G["Client sends 'stopped typing'"]
```

---

## 8. Chat Transfer / Escalation Flow

```mermaid
sequenceDiagram
    participant A1 as Agent 1 (current)
    participant ChatSvc as Chat Service
    participant RS as Routing Service
    participant AS as Agent State Store
    participant A2 as Agent 2 (specialist)
    participant C as Customer

    A1->>ChatSvc: Request transfer {session_id, reason, target_skill}
    ChatSvc->>RS: Find eligible agent for target_skill
    RS->>AS: Query available specialists
    AS-->>RS: Agent 2 available

    ChatSvc->>AS: Release Agent 1's slot
    ChatSvc->>AS: Reserve Agent 2's slot
    ChatSvc->>ChatSvc: Update session: agent_id = Agent 2
    ChatSvc->>DB: Log TRANSFER_EVENT

    ChatSvc->>A2: New chat assigned (with full history)
    ChatSvc->>C: Notify: "You've been transferred to a specialist"
    ChatSvc->>A1: Confirm transfer complete

    Note over A2,C: Agent 2 sees full chat history<br/>before responding — context preserved
```

---

## 9. Handling Agent Disconnects (Reliability)

```mermaid
flowchart TB
    A["Agent's WebSocket<br/>connection drops"] --> B["Connection Server detects<br/>disconnect (heartbeat timeout)"]
    B --> C["Presence Service marks<br/>agent OFFLINE"]
    C --> D{"Agent reconnects<br/>within grace period<br/>(e.g., 30s)?"}
    D -- Yes --> E["Re-attach agent to<br/>same active chat sessions<br/>No customer impact"]
    D -- No --> F["Mark agent's active chats<br/>as ORPHANED"]
    F --> G["Routing Service re-queues<br/>orphaned chats with<br/>HIGH priority"]
    G --> H["Route to next available agent<br/>with full chat history intact"]
```

*Because chat history is persisted immediately on every message (not just at session end), an orphaned chat can be handed to a new agent with zero information loss — the new agent sees the entire conversation up to that point.*

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Live Chat HLD))
    Queue Service
      Priority queue per skill/team
      Wait time estimation
      Reconnect-safe queue position
    Routing Service
      Skill-based matching
      Load-aware agent selection
      Handles transfers/escalation
    Chat Session Service
      Message persistence
      Session state management
    Connection Servers
      WebSocket hosting
      Sticky session routing
    Presence Service
      Agent online/offline/busy status
      Typing indicators (ephemeral)
    Agent State Store
      Capacity tracking
      Skill registry
    Chat History DB
      Durable message log
      Searchable transcript archive
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Queueing model | Server-side priority queue per skill, keyed by session | Survives customer disconnects without losing queue position |
| Routing strategy | Skill-based + load-aware ranking | Balances customer needs (right expertise) with agent fairness (even load distribution) |
| Message persistence | Synchronous write on every message | Enables lossless agent handoff/transfer with full context, unlike ephemeral-only chat |
| Typing indicators | Ephemeral, direct connection-server routing | High-frequency, low-value data — persisting or DB-routing would be wasteful |
| Agent disconnect handling | Grace period + re-queue with high priority | Balances tolerance for brief network blips against not leaving customers stranded |
| Transfer/escalation | Explicit session re-assignment with full history handoff | Critical for support quality — new agent must never ask the customer to repeat themselves |

---

## 12. Bottlenecks & Scaling Considerations

- **Queue fairness at scale** — naive FIFO can starve low-priority customers indefinitely during high load; use priority tiers with aging (wait time itself boosts effective priority over time) to prevent starvation.
- **Routing service polling overhead** — constantly polling for available agents across many skill queues can become expensive; prefer event-driven matching (agent becomes available → trigger immediate match attempt) over fixed-interval polling where possible.
- **Connection server capacity during peak support hours** — support traffic is highly time-of-day dependent (spikes during business hours or after outages); auto-scale connection server fleet based on concurrent session count.
- **Orphaned chat storms** — if many agents disconnect simultaneously (e.g., an agent-side outage), re-queueing a flood of high-priority orphaned chats can overwhelm remaining agents; needs backpressure/graceful degradation (e.g., temporarily widen acceptable wait-time SLAs).
- **Agent overload edge cases** — routing logic must strictly enforce `max_concurrent_chats` to avoid overwhelming agents during traffic spikes, even if it means longer queue times platform-wide.
- **Chat history search at scale** — full-text search across millions of historical transcripts needs a dedicated search index (Elasticsearch) rather than querying the primary Chat DB directly.
- **Cross-session context for repeat customers** — linking a customer's chat history across multiple past sessions (for context) requires indexing chats by `customer_id` in addition to `session_id`, adding a secondary access pattern to plan for.
