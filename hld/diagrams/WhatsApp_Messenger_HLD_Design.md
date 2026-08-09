# Design WhatsApp/Messenger — High-Level Design Document

## 1. Requirements

### Functional Requirements
- 1:1 and group messaging (text, media, voice notes)
- Message delivery guarantees: sent, delivered, read receipts
- Online/offline presence, "last seen"
- End-to-end encryption
- Message sync across multiple devices
- Offline message queuing (deliver when user comes back online)

### Non-Functional Requirements
- **Scale:** ~2B users, ~100B messages/day
- **Low latency:** Message delivery < 100ms for online recipients
- **Durability:** Messages must never be lost, even if recipient is offline for days
- **At-least-once delivery**, deduplicated at the client
- **High availability:** Chat must work through network flakiness on mobile

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| DAU | ~2B |
| Messages/day | ~100B |
| Messages/sec (avg) | ~1.2M |
| Messages/sec (peak, e.g. New Year) | ~5M+ |
| Avg message size | ~100 bytes (text) |
| Concurrent WebSocket/persistent connections | ~500M+ at peak |
| Connection servers needed | Thousands, each holding ~50K-100K connections |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    ClientA["Client A<br/>(Sender)"]
    ClientB["Client B<br/>(Recipient)"]

    subgraph Edge["Connection Layer"]
        LB["Load Balancer<br/>(sticky routing)"]
        ConnServer1["Connection Server 1<br/>(holds persistent WS connections)"]
        ConnServer2["Connection Server 2"]
    end

    subgraph Core["Core Messaging Services"]
        MsgSvc["Message Service<br/>(routing, persistence)"]
        PresenceSvc["Presence Service<br/>(online/offline/last seen)"]
        GroupSvc["Group Management Service"]
        MediaSvc["Media Service"]
    end

    subgraph Storage["Storage Layer"]
        MsgQueue[("Per-user Message Queue<br/>(offline delivery buffer)")]
        MsgStore[("Message Store<br/>(encrypted, short-retention)")]
        SessionStore[("Device/Session Registry<br/>(which conn server per user)")]
        MediaStore[("Object Storage<br/>(encrypted media blobs)")]
    end

    ClientA -->|"Persistent connection<br/>(WebSocket/MQTT)"| LB
    LB --> ConnServer1
    ClientB -->|"Persistent connection"| LB
    LB --> ConnServer2

    ConnServer1 --> MsgSvc
    MsgSvc --> SessionStore
    MsgSvc --> MsgQueue
    MsgSvc --> MsgStore
    MsgSvc -->|"Route to recipient's<br/>connection server"| ConnServer2
    ConnServer2 -->|"Push message"| ClientB

    MsgSvc --> PresenceSvc
    MsgSvc --> GroupSvc
    ClientA --> MediaSvc --> MediaStore
```

**Key idea:** Every client holds a long-lived persistent connection to a connection server. A `SessionStore` maps `user_id → which connection server they're attached to`, so the Message Service can route a message to the exact server holding the recipient's live socket — without broadcasting to the whole fleet.

---

## 3. Data Model

```mermaid
erDiagram
    USER ||--o{ DEVICE : owns
    USER ||--o{ CONVERSATION : "participates in"
    CONVERSATION ||--o{ MESSAGE : contains
    USER ||--o{ MESSAGE : sends
    CONVERSATION ||--o{ CONVERSATION_MEMBER : has

    USER {
        string user_id PK
        string phone_number
        timestamp last_seen
        string presence_status
    }
    DEVICE {
        string device_id PK
        string user_id FK
        string public_key
        string push_token
    }
    CONVERSATION {
        string conversation_id PK
        string type "1:1 or group"
        timestamp created_at
    }
    CONVERSATION_MEMBER {
        string conversation_id FK
        string user_id FK
        timestamp joined_at
    }
    MESSAGE {
        string message_id PK
        string conversation_id FK
        string sender_id FK
        bytes encrypted_payload
        string status "sent/delivered/read"
        timestamp sent_at
        string client_message_id "for dedup"
    }
```

---

## 4. Message Send Flow (Online Recipient) — Detailed Sequence

```mermaid
sequenceDiagram
    participant A as Client A (Sender)
    participant CS1 as Connection Server 1
    participant MS as Message Service
    participant SS as Session Store
    participant DB as Message Store
    participant CS2 as Connection Server 2
    participant B as Client B (Recipient)

    A->>CS1: Send encrypted message (client_message_id: XYZ)
    CS1->>MS: Forward message
    MS->>DB: Persist message (status = SENT)
    MS-->>CS1: Ack
    CS1-->>A: ACK (single checkmark ✓)

    MS->>SS: Lookup: which conn server holds user B?
    SS-->>MS: Connection Server 2

    MS->>CS2: Route message to B's server
    CS2->>B: Push message over persistent connection
    B-->>CS2: Client ACK (message received)
    CS2->>MS: Delivery confirmation
    MS->>DB: Update status = DELIVERED
    MS->>CS1: Notify sender
    CS1-->>A: Delivered (double checkmark ✓✓)

    B->>CS2: User opens chat (read receipt)
    CS2->>MS: Read event
    MS->>DB: Update status = READ
    MS->>CS1: Notify sender
    CS1-->>A: Read (blue checkmarks)
```

---

## 5. Message Send Flow (Offline Recipient) — Detailed Sequence

```mermaid
sequenceDiagram
    participant A as Client A (Sender)
    participant MS as Message Service
    participant SS as Session Store
    participant Q as Per-user Message Queue
    participant DB as Message Store
    participant Push as Push Notification Service
    participant B as Client B (comes online later)

    A->>MS: Send message
    MS->>DB: Persist message (status = SENT)
    MS->>SS: Lookup B's connection server
    SS-->>MS: No active connection (offline)

    MS->>Q: Enqueue message in B's offline queue
    MS->>Push: Trigger push notification (APNs/FCM)
    Push-->>B: OS-level push notification

    Note over B: Later, user opens app
    B->>MS: Reconnect (WebSocket handshake)
    MS->>SS: Register B's new connection
    MS->>Q: Drain B's offline queue
    Q-->>MS: All pending messages
    MS-->>B: Deliver all queued messages in order
    MS->>DB: Update statuses = DELIVERED
```

**Key design point:** The offline queue guarantees at-least-once delivery — nothing is dropped if the recipient is offline, and the client deduplicates using `client_message_id` in case of retries or reconnect races.

---

## 6. End-to-End Encryption (Simplified — Signal Protocol Style)

```mermaid
sequenceDiagram
    participant A as Client A
    participant Server as Server (relay only)
    participant B as Client B

    Note over A,B: Initial key exchange (once, on first contact)
    A->>Server: Fetch B's public prekey bundle
    Server-->>A: B's identity key + signed prekey + one-time prekey
    A->>A: Derive shared secret (X3DH key agreement)

    Note over A,B: Every subsequent message
    A->>A: Encrypt message with per-message key<br/>(Double Ratchet — key changes every message)
    A->>Server: Send ciphertext only
    Server->>Server: Store/route ciphertext<br/>(cannot decrypt — no keys held)
    Server->>B: Deliver ciphertext
    B->>B: Decrypt with matching ratchet state
```

**Key idea:** The server is a **blind relay** — it only ever stores/forwards encrypted ciphertext. It never has access to plaintext or the encryption keys, so even a full server compromise doesn't expose message content. This is why "message storage" on the server is short-retention: once delivered, plaintext-decryptable copies exist only on client devices.

---

## 7. Group Messaging Fanout

```mermaid
flowchart TB
    A["User sends message to Group G<br/>(50 members)"] --> B["Message Service"]
    B --> C["Group Service: fetch member list"]
    C --> D["Encrypt message once per recipient<br/>(sender encrypts to each member's key,<br/>or uses group session key)"]
    D --> E{"For each of 50 members"}
    E --> F["Online?"]
    F -- Yes --> G["Route via their connection server<br/>(real-time push)"]
    F -- No --> H["Enqueue in their offline queue<br/>+ trigger push notification"]
```

*Group encryption in practice uses **sender keys** (a shared symmetric key per group, rotated on membership change) rather than encrypting individually to each member's key — this avoids O(N) encryption cost per message for large groups.*

---

## 8. Presence System ("Online" / "Last Seen")

```mermaid
flowchart LR
    A["Client connects<br/>(WebSocket open)"] --> B["Connection Server marks<br/>user ONLINE in Presence Service"]
    B --> C["Presence Service notifies<br/>relevant contacts (who have them open)"]

    D["Client disconnects<br/>(app backgrounded/network lost)"] --> E["Connection Server marks<br/>user OFFLINE + timestamp"]
    E --> F["Presence Service stores<br/>last_seen = timestamp"]
    F --> G["Notifies relevant contacts<br/>of status change"]

    H["Heartbeat / keepalive ping"] -.->|"every 30s"| A
```

*Presence is deliberately **not** broadcast globally — it's only pushed to users who currently have that contact's chat open or in their contact list view, to avoid massive unnecessary fanout for a low-value signal.*

---

## 9. Component Responsibilities Summary

```mermaid
mindmap
  root((WhatsApp HLD))
    Connection Servers
      Hold persistent WebSocket connections
      Sticky routing per user
      Horizontal scaling by connection count
    Message Service
      Routing between connection servers
      Persistence + status tracking
      Dedup via client_message_id
    Session Store
      user_id to connection server mapping
      Multi-device session tracking
    Offline Queue
      Per-user pending message buffer
      Drained on reconnect
    Presence Service
      Online/offline/last-seen tracking
      Targeted fanout, not global broadcast
    Group Service
      Membership management
      Sender-key based group encryption
    Push Notification Service
      APNs/FCM integration
      Wakes up offline clients
```

---

## 10. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Connection model | Persistent WebSocket/MQTT per client | Enables real-time push without polling; critical for instant delivery UX |
| Routing | Session store maps user → connection server | Avoids broadcasting messages to the entire server fleet |
| Delivery guarantee | At-least-once + client-side dedup | Simpler server logic; exactly-once is enforced cheaply at the edge via message IDs |
| Offline handling | Per-user durable queue + push notification | Ensures no message loss regardless of recipient connectivity |
| Encryption | End-to-end (Signal Protocol / Double Ratchet) | Server never sees plaintext — strongest privacy guarantee, survives server compromise |
| Group encryption | Sender-key model | Avoids O(N) per-message encryption cost for large groups |
| Message retention on server | Short-lived, deleted after delivery confirmation | Server storage isn't the source of truth long-term; devices hold decrypted history |
| Presence fanout | Targeted, not global | Presence updates for 2B users would be enormous if broadcast indiscriminately |

---

## 11. Bottlenecks & Scaling Considerations

- **Connection server capacity** — each server can hold a bounded number of concurrent sockets (memory/FD limits); scale horizontally, use efficient protocols (MQTT/custom binary over WebSocket) to minimize per-connection overhead.
- **Session store as a critical hot path** — every message send requires a session lookup; must be extremely low-latency (in-memory KV, e.g., Redis) and highly available, since it's on the critical path for every single message.
- **Reconnect storms** — mobile networks cause frequent disconnects (switching wifi/cellular); connection servers must handle high churn without overwhelming the session store with re-registration traffic.
- **Multi-device sync** — a user with phone + web + desktop needs messages delivered to all active sessions consistently; requires per-device delivery tracking, not just per-user.
- **Offline queue growth for inactive users** — a user offline for weeks accumulates a large queue; needs bounded retention (e.g., messages older than 30 days expire) and efficient bulk-drain on reconnect.
- **Group fanout at scale** — very large groups (thousands of members, e.g., broadcast lists/communities) stress the per-member fanout path; may need dedicated batch-fanout workers separate from the 1:1 hot path.
- **Push notification reliability** — APNs/FCM aren't instant or guaranteed; can't be the sole mechanism for "did the user get notified" — server-side offline queue remains the source of truth.
