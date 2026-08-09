# Design a Client + Server Caching Strategy for a Mobile App with Intermittent Connectivity — High-Level Design Document

## 1. Requirements

### Functional Requirements
- App must remain functional (at least in a read/browse capacity) when the device has no network connectivity
- Locally-made changes while offline must sync to the server once connectivity returns
- Handle conflicts when the same data was modified both locally (offline) and on the server (by another device/session)
- Minimize unnecessary data transfer over unreliable/metered mobile networks

### Non-Functional Requirements
- **Offline-first responsiveness:** UI should never block waiting for network — always render from local cache immediately, update when fresh data arrives
- **Battery/bandwidth efficiency:** Sync strategy must be mindful of mobile battery and data plan constraints, not just "sync everything constantly"
- **Data freshness vs staleness tolerance:** Different data types have different acceptable staleness windows
- **Graceful degradation:** Intermittent connectivity (not just fully offline) must be handled smoothly — brief drops shouldn't cause errors or lost work

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Typical mobile connectivity | Intermittent — WiFi/cellular handoffs, tunnels, elevators, airplane mode |
| Local storage budget | Tens to low hundreds of MB typical for app data cache |
| Sync payload size (delta) | KBs typical, vs potentially MBs for full re-sync |
| Acceptable sync latency (reconnect) | Seconds for small changes, longer tolerated for bulk sync |

---

## 2. The Core Philosophy — Offline-First, Not Offline-Tolerant

```mermaid
flowchart TB
    A["Traditional 'online-first'<br/>design: app assumes network<br/>is available, treats offline<br/>as an ERROR CASE to handle<br/>defensively"] --> A1["Result: poor UX during<br/>connectivity gaps — spinners,<br/>errors, blocked interactions"]

    B["Offline-first design: the<br/>LOCAL cache/database IS the<br/>primary source of truth for<br/>the UI at all times — network<br/>sync is a BACKGROUND CONCERN,<br/>not a blocking requirement<br/>for basic app function"] --> B1["Result: app feels instant<br/>and functional regardless of<br/>connectivity state; network<br/>sync happens transparently<br/>whenever possible"]

    C["This philosophical shift —<br/>treating the network as an<br/>OPTIONAL ENHANCEMENT rather<br/>than a REQUIRED DEPENDENCY —<br/>is the foundation every<br/>other design decision in<br/>this system builds on"] --> B1
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph Device["Mobile Device"]
        UI["App UI"]
        LocalDB[("Local Database<br/>(SQLite/Realm —<br/>source of truth for UI)")]
        SyncEngine["Sync Engine"]
        OutboxQueue[("Outbox Queue<br/>(pending local changes)")]
    end

    subgraph Network["Network (intermittent)"]
        Conn["Connectivity Monitor"]
    end

    subgraph Backend["Backend"]
        SyncAPI["Sync API"]
        ServerDB[("Server Database<br/>(source of truth for<br/>cross-device consistency)")]
        ChangeLog[("Change Log<br/>(for delta sync)")]
    end

    UI <--> LocalDB
    UI -->|"writes go to<br/>local DB immediately"| LocalDB
    LocalDB --> OutboxQueue
    SyncEngine <--> LocalDB
    SyncEngine <--> OutboxQueue
    SyncEngine --> Conn

    SyncEngine <-.->|"sync when<br/>connectivity available"| SyncAPI
    SyncAPI --> ServerDB
    SyncAPI --> ChangeLog
```

**Key idea:** All reads and writes from the UI go through the Local Database — the UI never talks to the network directly. The Sync Engine runs as an independent background process, opportunistically pushing outbound changes and pulling inbound updates whenever connectivity allows, completely decoupled from the UI's rendering path.

---

## 4. Data Model

```mermaid
erDiagram
    LOCAL_RECORD {
        string record_id PK
        string data
        long local_version
        long last_synced_server_version
        string sync_status "synced/pending/conflict"
        timestamp locally_modified_at
    }
    OUTBOX_ENTRY {
        string entry_id PK
        string record_id FK
        string operation "create/update/delete"
        string payload
        timestamp queued_at
        int retry_count
    }
    CHANGE_LOG_ENTRY {
        string record_id FK
        long server_version
        string payload
        timestamp server_committed_at
    }
```

---

## 5. Local Write Flow (While Offline or Online) — Detailed Sequence

```mermaid
sequenceDiagram
    participant User as User
    participant UI as App UI
    participant LocalDB as Local Database
    participant Outbox as Outbox Queue

    User->>UI: Edit item (e.g., update a note)
    UI->>LocalDB: Write change immediately
    LocalDB-->>UI: Confirmed (instant,<br/>no network wait)
    UI-->>User: UI updates immediately<br/>(optimistic — feels instant<br/>regardless of connectivity)

    LocalDB->>Outbox: Enqueue change for<br/>eventual sync<br/>{operation: UPDATE,<br/>record_id, payload}

    Note over Outbox: This happens IDENTICALLY<br/>whether the device is<br/>currently online or offline —<br/>the write path never branches<br/>on connectivity state
```

**Why the write path is identical regardless of connectivity:** This uniformity is what makes offline-first architecture reliable — there's no special "offline mode" code path that could have different bugs than the "online mode" path. Every write always goes local-first and queues for sync; connectivity only affects WHEN the outbox drains, never HOW a write is initially handled.

---

## 6. Connectivity Recovery & Sync Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Conn as Connectivity Monitor
    participant Sync as Sync Engine
    participant Outbox as Outbox Queue
    participant API as Sync API
    participant Server as Server Database
    participant LocalDB as Local Database

    Conn->>Sync: Connectivity restored<br/>(event-driven, e.g., OS<br/>network reachability callback)

    Sync->>Outbox: Drain pending changes<br/>(in original order)

    loop For each queued change
        Sync->>API: Push change<br/>{record_id, operation, payload,<br/>local_version}
        API->>Server: Apply change
        alt No conflict
            Server-->>API: Accepted, new server_version
            API-->>Sync: Success
            Sync->>Outbox: Remove from queue
            Sync->>LocalDB: Update sync_status=synced
        else Conflict detected<br/>(server has a newer version)
            Server-->>API: CONFLICT + current server value
            API-->>Sync: Conflict response
            Sync->>LocalDB: Mark sync_status=conflict<br/>(handled in Section 8)
        end
    end

    Sync->>API: Pull: any server changes<br/>since my last_synced_version?
    API->>Server: Query change log
    Server-->>API: Delta of changes
    API-->>Sync: Changes to apply
    Sync->>LocalDB: Apply incoming changes<br/>(that weren't already<br/>reflected locally)
```

---

## 7. Delta Sync (Bandwidth Efficiency)

```mermaid
flowchart TB
    A["Naive sync: on reconnect,<br/>download the ENTIRE dataset<br/>fresh from server"] --> A1["Wasteful — most data<br/>hasn't changed since last<br/>sync, especially for brief<br/>connectivity gaps (seconds<br/>to minutes)"]

    B["Delta sync: client tracks<br/>last_synced_server_version<br/>(a checkpoint/cursor)"] --> C["On reconnect, client asks:<br/>'give me only changes AFTER<br/>version X'"]
    C --> D["Server's Change Log<br/>(same underlying concept as<br/>CDC — an ordered, queryable<br/>record of changes) returns<br/>only the DELTA"]
    D --> E["Dramatically reduces data<br/>transfer for typical brief<br/>connectivity gaps — critical<br/>for users on metered/limited<br/>mobile data plans"]
```

*This delta-sync mechanism mirrors the CDC Pipeline design's core principle — maintaining an ordered change log that consumers (in this case, mobile clients rather than downstream services) can resume from at their own last-known checkpoint, rather than requiring a full re-fetch.*

---

## 8. Conflict Resolution (Concurrent Offline + Server Changes)

```mermaid
flowchart TB
    A["Device A edits Note X<br/>while offline (based on<br/>server version 5)"] --> B["Meanwhile, Device B<br/>(same user, different device,<br/>or a collaborator) edits<br/>Note X and syncs to server,<br/>creating version 6"]

    B --> C["Device A reconnects,<br/>attempts to push its<br/>change based on stale<br/>version 5"]
    C --> D{"Conflict Resolution<br/>Strategy"}

    D --> E["Last-Write-Wins<br/>(by timestamp)"]
    E --> E1["Simple, but can silently<br/>discard Device A's<br/>legitimate edit"]

    D --> F["Field-level merge<br/>(if the data model supports it —<br/>e.g., different fields<br/>changed on each side)"]
    F --> F1["Merges non-conflicting<br/>field changes automatically,<br/>only surfaces TRUE conflicts<br/>(same field changed both<br/>places) to the user"]

    D --> G["Present both versions,<br/>let user manually resolve<br/>(same pattern as the<br/>Distributed File Storage<br/>design's 'conflicted copy')"]

    H["Choice depends on data<br/>criticality — this design<br/>uses field-level merge where<br/>the schema allows it, falling<br/>back to explicit user<br/>resolution for genuine<br/>same-field conflicts"] -.-> F
```

---

## 9. Data Prioritization & Selective Sync

```mermaid
flowchart TB
    A["Not all data is equally<br/>important to sync/cache —<br/>especially under constrained<br/>bandwidth/storage"] --> B{"Prioritization Strategy"}

    B --> C["Critical data<br/>(user's own recent content,<br/>active work-in-progress)"]
    C --> C1["Always synced first,<br/>eagerly cached, never evicted<br/>under storage pressure"]

    B --> D["Important but not urgent<br/>(recently viewed content,<br/>likely to be revisited)"]
    D --> D1["Cached opportunistically,<br/>synced when convenient<br/>(e.g., WiFi available)"]

    B --> E["Low priority<br/>(rarely-accessed historical<br/>data)"]
    E --> E1["Synced on-demand only,<br/>NOT proactively cached —<br/>fetched fresh if/when<br/>actually requested"]

    F["Network-aware sync policy:<br/>large/low-priority syncs<br/>deferred until WiFi is<br/>available, not consumed<br/>from the user's cellular<br/>data plan"] -.-> D1
```

---

## 10. Handling Intermittent (Not Fully Offline) Connectivity

```mermaid
sequenceDiagram
    participant Sync as Sync Engine
    participant API as Sync API

    Sync->>API: Push change 1
    Note over Sync,API: Connection drops mid-request<br/>(tunnel, elevator, etc.)
    API--xSync: Request fails/times out

    Sync->>Sync: Change remains in Outbox<br/>(was NEVER removed, since<br/>no success confirmation<br/>was received)

    Note over Sync: Connectivity monitor detects<br/>connection restored moments later

    Sync->>API: Retry push change 1<br/>(idempotent — includes the<br/>same local_version reference,<br/>same pattern as the<br/>Idempotent API Requests design)
    API-->>Sync: Success
    Sync->>Sync: NOW remove from Outbox
```

**Why idempotency matters critically here:** Intermittent connectivity means requests can fail at any point — including AFTER the server successfully processed them but BEFORE the client received confirmation. Without idempotent retry handling, this ambiguous "did it actually work?" scenario could cause either lost changes (if the client gives up) or duplicate changes (if the client blindly retries without deduplication) — the same idempotency key pattern from the dedicated Idempotent API Requests design applies directly here.

---

## 11. Component Responsibilities Summary

```mermaid
mindmap
  root((Mobile Offline Caching HLD))
    Local Database
      Source of truth for UI
      Always available, instant reads/writes
    Outbox Queue
      Pending local changes
      Survives app restarts
    Sync Engine
      Background, connectivity-aware
      Push and pull sync logic
    Connectivity Monitor
      OS-level network reachability
      Triggers sync attempts
    Change Log (server-side)
      Ordered, resumable delta source
      Same pattern as CDC pipeline
    Conflict Resolver
      Field-level merge where possible
      User-facing resolution as fallback
```

---

## 12. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Architecture philosophy | Offline-first (local DB as source of truth for UI) | Makes the app feel instant and functional regardless of connectivity, rather than treating offline as a degraded error state |
| Write path | Always local-first, queued for async sync | Uniform behavior regardless of connectivity state eliminates an entire class of "offline mode" bugs |
| Sync mechanism | Delta sync via server-side change log | Dramatically reduces bandwidth usage on typical brief connectivity gaps, critical for metered mobile data |
| Conflict resolution | Field-level merge with user-facing fallback | Automatically resolves the common case (non-overlapping edits) while never silently discarding genuinely conflicting user work |
| Retry safety | Idempotent operations via version/key tracking | Handles the ambiguous "request failed but may have succeeded" scenario inherent to intermittent connectivity |
| Data prioritization | Tiered sync urgency + network-aware scheduling | Respects mobile battery/bandwidth constraints rather than naively syncing everything as fast as possible |

---

## 13. Bottlenecks & Scaling Considerations

- **Local storage limits** — mobile devices have finite storage; the local cache needs an eviction policy (similar principles to the Distributed Cache design's LRU eviction) for lower-priority cached data, while never evicting unsynced outbox entries.
- **Outbox queue growth during extended offline periods** — a user offline for days accumulates many pending changes; the sync engine must handle draining a potentially large backlog efficiently on reconnect, likely batching rather than one-request-per-change.
- **Battery impact of background sync** — aggressive background syncing drains battery; must respect OS-level background execution limits and ideally batch sync attempts rather than maintaining constant connection attempts.
- **Change log retention on the server** — similar to the CDC Pipeline design's transaction log retention concern, if a device is offline long enough that the server's change log has rotated past its last-synced checkpoint, a full re-sync (not just delta) becomes necessary — this boundary needs clear, tested handling.
- **Multi-device conflict complexity growth** — the more devices/sessions a single user has active simultaneously, the more frequently genuine conflicts arise; this is a natural consequence of the offline-first model and needs to be a well-tested, well-designed user-facing experience, not an edge case.
- **Testing intermittent connectivity scenarios** — this system's core value proposition is specifically about handling FLAKY (not just fully-on or fully-off) connectivity; testing must include simulated mid-request drops, slow/degraded connections, and rapid connect/disconnect cycling, not just simple airplane-mode-toggle testing.
