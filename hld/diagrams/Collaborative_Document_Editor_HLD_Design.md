# Design a Real-Time Collaborative Document Editor — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Multiple users can edit the same document simultaneously
- Changes from one user appear near-instantly for all others
- No lost updates — concurrent edits must merge correctly, not overwrite
- Support cursor/selection presence (see where others are typing)
- Offline editing with sync on reconnect
- Full document version history / undo

### Non-Functional Requirements
- **Latency:** Local edits should render instantly (< 16ms, no perceptible lag); remote edits should propagate < 100-200ms
- **Consistency:** All clients must eventually converge to the identical document state
- **Conflict resolution:** Automatic — users should never see manual "merge conflict" dialogs
- **Scale:** Most documents have small concurrent editor counts (2-20), but the platform serves millions of documents simultaneously
- **Durability:** No edit should ever be silently lost, even across disconnects

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Concurrent active documents | ~1M+ |
| Avg concurrent editors/doc | 2-5 (occasionally 50+) |
| Edit operations/sec/active doc | 1-10 (keystroke-level) |
| Total edit ops/sec (platform-wide) | ~1M+ |
| Document size | KB to low MB range |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    ClientA["Client A<br/>(Editor)"]
    ClientB["Client B<br/>(Editor)"]
    ClientC["Client C<br/>(Editor)"]

    subgraph Edge["Real-time Layer"]
        LB["Load Balancer<br/>(routes to doc's owning server)"]
        DocServer["Document Session Server<br/>(holds in-memory doc state<br/>+ WebSocket connections)"]
    end

    subgraph Core["Core Services"]
        SyncEngine["Sync Engine<br/>(OT/CRDT conflict resolution)"]
        PresenceSvc["Presence Service<br/>(cursors, selections)"]
        PersistSvc["Persistence Service<br/>(periodic snapshot + op log)"]
    end

    subgraph Storage["Storage Layer"]
        OpLog[("Operation Log<br/>(append-only, per document)")]
        SnapshotStore[("Document Snapshots<br/>(periodic full-state checkpoints)")]
        MetaDB[("Document Metadata DB<br/>(permissions, ownership)")]
    end

    ClientA <-->|"WebSocket"| LB
    ClientB <-->|"WebSocket"| LB
    ClientC <-->|"WebSocket"| LB
    LB <--> DocServer

    DocServer --> SyncEngine
    DocServer --> PresenceSvc
    DocServer --> PersistSvc

    PersistSvc --> OpLog
    PersistSvc --> SnapshotStore
    DocServer --> MetaDB
```

**Key idea:** Each active document is "owned" by exactly one Document Session Server at a time, which holds the authoritative in-memory state and mediates all concurrent edits through the Sync Engine. This avoids distributed consensus on every keystroke — conflict resolution happens in one place per document.

---

## 3. Core Conflict Resolution Approaches

```mermaid
flowchart TB
    A["Two users edit same<br/>document region simultaneously"] --> B{"Conflict Resolution<br/>Strategy"}
    B --> C["Operational Transformation (OT)"]
    B --> D["CRDT<br/>(Conflict-free Replicated Data Type)"]

    C --> C1["Each op is transformed against<br/>concurrent ops before applying"]
    C --> C2["Requires central server<br/>to serialize operation order"]
    C --> C3["Used by: Google Docs"]

    D --> D1["Each character/element has<br/>a unique, orderable ID"]
    D --> D2["Merges are commutative —<br/>any order produces same result"]
    D --> D3["Works well peer-to-peer,<br/>no central authority required"]
    D --> D4["Used by: Figma, Notion (variants)"]
```

*This design uses **OT** as the primary approach (Google Docs-style, server-mediated) since it requires less client-side complexity and metadata overhead than CRDTs, at the cost of needing a central sequencing authority per document.*

---

## 4. Operational Transformation — How It Works

```mermaid
sequenceDiagram
    participant A as Client A
    participant Server as Doc Server (Sync Engine)
    participant B as Client B

    Note over A,B: Both start with: "Hello World" (version 5)

    A->>A: User types "!" at position 11<br/>→ Op: Insert("!", pos=11)
    A->>Server: Send Op1 {insert "!" @11, based on v5}

    B->>B: User deletes "World"<br/>→ Op: Delete(pos=6, len=5)
    B->>Server: Send Op2 {delete @6 len=5, based on v5}

    Note over Server: Server received Op1 first, applied it (v5→v6)
    Server->>Server: Apply Op1: "Hello World!" (v6)

    Note over Server: Op2 arrives, but was based on v5 (stale)<br/>Server must TRANSFORM Op2 against Op1
    Server->>Server: Transform: Op2 unaffected since<br/>Op1's insert was after Op2's delete range
    Server->>Server: Apply transformed Op2: "Hello !" (v7)

    Server-->>A: Broadcast transformed Op2 (v7)
    Server-->>B: Broadcast Op1 + ack of Op2 (v7)

    Note over A,B: Both clients converge to "Hello !" (v7)
```

**Key idea:** When two operations are based on the same starting version but conflict in position, the server transforms the later-arriving operation's coordinates against the earlier one so it still makes semantic sense when applied — this is the mathematical core of OT.

---

## 5. Document Edit Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client
    participant DS as Document Session Server
    participant SE as Sync Engine
    participant OpLog as Operation Log
    participant Other as Other Connected Clients

    C->>C: User types character<br/>(applied locally instantly — optimistic UI)
    C->>DS: Send Op {type, position, content, base_version}

    DS->>SE: Transform op against any concurrent<br/>ops since base_version
    SE-->>DS: Transformed op + new version number

    DS->>OpLog: Append transformed op (durable log)
    DS->>Other: Broadcast transformed op to all<br/>other connected clients

    Other->>Other: Apply op to local document state

    DS-->>C: Ack with final version number
    C->>C: Reconcile local optimistic state<br/>if transformation changed anything
```

---

## 6. Document Loading & Persistence Strategy

```mermaid
flowchart TB
    A["Client opens document"] --> B["Document Session Server:<br/>Is doc already active in memory?"]
    B -- Yes --> C["Attach client to existing session<br/>Send current in-memory state"]
    B -- No --> D["Load latest Snapshot from store"]
    D --> E["Replay Operation Log entries<br/>since snapshot"]
    E --> F["Reconstruct current document state<br/>in memory"]
    F --> G["Start new session,<br/>attach client"]

    H["Periodic background job"] -.->|"every N ops or T seconds"| I["Persistence Service:<br/>Write new Snapshot"]
    I --> J["Truncate/archive old<br/>Operation Log entries"]
```

**Key idea:** The system never replays the *entire* history from scratch — periodic snapshots bound how much of the operation log needs replaying to reconstruct current state, keeping document load times fast even for documents with years of edit history.

---

## 7. Presence & Cursor Sharing

```mermaid
flowchart LR
    A["User moves cursor /<br/>selects text"] --> B["Client sends lightweight<br/>presence update<br/>(NOT persisted, ephemeral)"]
    B --> C["Document Session Server"]
    C --> D["Broadcast to all other<br/>connected clients on this doc"]
    D --> E["Other clients render<br/>colored cursor + selection<br/>with user's name/avatar"]

    F["Client disconnects"] --> G["Server detects socket close"]
    G --> H["Broadcast 'user left'<br/>presence update"]
```

*Presence updates are high-frequency but ephemeral — they're never written to the Operation Log or persisted, since they represent transient UI state, not document content.*

---

## 8. Offline Editing & Reconnect Sync

```mermaid
sequenceDiagram
    participant C as Client (goes offline)
    participant DS as Document Session Server

    Note over C: Network lost
    C->>C: Continue editing locally<br/>(queue ops in local buffer,<br/>optimistic UI throughout)

    Note over C: Network restored
    C->>DS: Reconnect + send<br/>{last_known_version, queued_ops[]}

    DS->>DS: Fetch all ops that occurred<br/>on server since last_known_version
    DS->>DS: Transform client's queued ops<br/>against all missed server ops

    DS->>C: Send missed ops (for client to apply)
    DS->>DS: Apply client's transformed<br/>queued ops to authoritative state
    DS->>DS: Broadcast client's ops to<br/>other connected clients

    DS-->>C: Ack, new synchronized version
    Note over C: Client document state now<br/>fully converged with server
```

---

## 9. Document Server Ownership & Failover

```mermaid
flowchart TB
    A["Document D is being edited"] --> B["Owned by Document Session Server X<br/>(holds authoritative in-memory state)"]
    B --> C{"Server X crashes/restarts"}
    C -- Yes --> D["Load Balancer detects failure"]
    D --> E["Route reconnecting clients to<br/>new Document Session Server Y"]
    E --> F["Server Y reconstructs state:<br/>Snapshot + Operation Log replay<br/>(same as cold-start load)"]
    F --> G["Clients resume editing<br/>with minimal disruption"]
```

*Because the Operation Log is the durable source of truth (not the in-memory state), losing a Document Session Server is recoverable — any server can reconstruct the exact same state by replaying the log, making ownership reassignment safe.*

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Collab Editor HLD))
    Document Session Server
      Holds in-memory authoritative state
      Mediates all edits for its documents
      WebSocket connection host
    Sync Engine
      OT transformation logic
      Version sequencing
    Operation Log
      Durable append-only edit history
      Source of truth for reconstruction
    Snapshot Store
      Periodic full-state checkpoints
      Bounds replay cost on load
    Presence Service
      Ephemeral cursor/selection broadcast
      Not persisted
    Persistence Service
      Background snapshotting
      Log truncation/archival
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Conflict resolution | Operational Transformation | Well-suited to a centralized server model; less client metadata overhead than CRDTs |
| Document ownership | One session server per active document | Avoids distributed consensus on every keystroke; conflict resolution centralized and simple |
| Durability model | Append-only op log + periodic snapshots | Bounds recovery/replay time while keeping every edit durably recorded |
| Presence updates | Ephemeral, not persisted | Cursor position isn't document content — persisting it would bloat the op log for no value |
| Optimistic local editing | Apply locally instantly, reconcile async | Local typing must never feel laggy; reconciliation happens transparently in the background |
| Offline support | Queue ops locally, transform-and-replay on reconnect | Users expect to keep typing through brief network drops without losing work |
| Server failover | Stateless reconstruction from log + snapshot | Any server can take over a document; no special leader-election needed for document ownership |

---

## 12. Bottlenecks & Scaling Considerations

- **Hot documents with many concurrent editors** (50+ users on one doc, e.g., large team meeting notes) — a single session server's broadcast fanout and OT transformation load can become a bottleneck; may need sharded broadcast groups or CRDT fallback for very high concurrency documents.
- **Operation Log write throughput** — extremely active documents generate many small ops/sec; batch writes to the log where possible without sacrificing durability guarantees.
- **Snapshot frequency tradeoff** — too frequent = wasted storage/compute; too infrequent = slow reconstruction on load/failover. Tune based on op volume per document (e.g., snapshot every 1000 ops or 5 minutes, whichever first).
- **Cross-region latency** — users far from the document's owning server experience higher round-trip latency for their edits to be acknowledged/broadcast; may need regional session server placement with cross-region replication for global documents.
- **Large document initial load time** — very large documents (many MB, years of history) need efficient snapshot formats (not naive full-text) to avoid slow cold starts.
- **Session server memory pressure** — holding many large documents in memory simultaneously requires LRU-style eviction of inactive document sessions, with fast reload from snapshot+log on next access.
- **OT transformation complexity for rich content** — plain text OT is well-understood; transforming operations on rich formatting (tables, embedded images, nested lists) is significantly harder to get correct and is where most real-world bugs live.
