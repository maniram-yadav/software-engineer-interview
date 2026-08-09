# Design a Distributed File Storage System (Dropbox/Google Drive) — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Upload, download, delete files/folders
- Sync files across multiple devices automatically
- Support large files via chunked upload/download
- File versioning (restore previous versions)
- Sharing files/folders with other users, with permissions
- Conflict resolution when the same file is edited offline on two devices

### Non-Functional Requirements
- **Scale:** ~500M users, exabytes of total storage
- **Bandwidth efficiency:** Only sync the parts of a file that changed (delta sync), not the whole file
- **Durability:** 99.999999999% (11 nines) — data must never be lost
- **Availability:** Sync should resume gracefully after any connectivity interruption
- **Consistency:** Eventually consistent across devices is acceptable; conflicts must be handled gracefully, not silently overwritten

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| DAU | ~100M |
| Avg storage/user | ~10GB |
| Total storage | ~5 exabytes |
| File uploads/sec (platform-wide) | ~50,000 |
| Avg file size | Highly variable — KB (docs) to GB (video) |
| Chunk size | ~4MB (common choice, balances overhead vs granularity) |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Client["Client Apps<br/>(Desktop sync agent / Web / Mobile)"]
    Gateway["API Gateway"]

    subgraph Core["Core Services"]
        MetadataSvc["Metadata Service<br/>(file/folder tree, versions)"]
        SyncSvc["Sync Service<br/>(change detection, delta computation)"]
        ChunkSvc["Chunking Service<br/>(splits files, dedup)"]
        ShareSvc["Sharing/Permissions Service"]
        NotifSvc["Notification Service<br/>(push change events to devices)"]
    end

    subgraph Storage["Storage Layer"]
        MetadataDB[("Metadata DB<br/>(file tree, version history)")]
        BlockStore[("Block/Chunk Storage<br/>(S3-like object storage,<br/>content-addressed)")]
        DedupIndex[("Chunk Hash Index<br/>(for deduplication)")]
    end

    Client -->|"Upload/download"| Gateway
    Gateway --> MetadataSvc --> MetadataDB
    Gateway --> SyncSvc
    SyncSvc --> ChunkSvc
    ChunkSvc --> DedupIndex
    ChunkSvc --> BlockStore
    Gateway --> ShareSvc --> MetadataDB

    SyncSvc --> NotifSvc
    NotifSvc -->|"Push: 'file changed'"| Client
```

**Key idea:** Files are never treated as monolithic blobs — they're split into content-addressed **chunks**. This enables two critical features: (1) **delta sync** — only upload/download the chunks that actually changed, and (2) **deduplication** — if two users store the identical file (or two versions share unchanged sections), the identical chunks are stored only once platform-wide.

---

## 3. Data Model

```mermaid
erDiagram
    USER ||--o{ FILE : owns
    FILE ||--o{ FILE_VERSION : "has history of"
    FILE_VERSION ||--o{ CHUNK_REFERENCE : "composed of"
    CHUNK_REFERENCE }o--|| CHUNK : references
    FILE ||--o{ SHARE : "shared via"
    USER ||--o{ DEVICE : registers

    USER {
        string user_id PK
        string email
        long storage_quota
        long storage_used
    }
    FILE {
        string file_id PK
        string owner_id FK
        string parent_folder_id
        string name
        string current_version_id FK
    }
    FILE_VERSION {
        string version_id PK
        string file_id FK
        int version_number
        long total_size
        timestamp created_at
        string created_by_device_id
    }
    CHUNK {
        string chunk_hash PK "content hash, e.g. SHA-256"
        long size
        int reference_count
        string storage_location
    }
    CHUNK_REFERENCE {
        string version_id FK
        string chunk_hash FK
        int sequence_order
    }
    SHARE {
        string share_id PK
        string file_id FK
        string shared_with_user_id
        string permission "view/edit"
    }
```

**Key modeling decision:** A `FILE_VERSION` doesn't store the file content directly — it stores an **ordered list of chunk references**. Two versions of a mostly-unchanged large file share almost all the same chunk references, so storing a new version costs almost nothing beyond the changed chunks.

---

## 4. File Upload Flow — Chunking & Deduplication

```mermaid
sequenceDiagram
    participant C as Client (Sync Agent)
    participant Chunk as Chunking Service
    participant Dedup as Dedup Index
    participant Block as Block Storage
    participant Meta as Metadata Service

    C->>C: Split file into ~4MB chunks
    C->>C: Compute hash (SHA-256) for each chunk

    loop For each chunk
        C->>Chunk: Check if chunk_hash already exists
        Chunk->>Dedup: Lookup hash
        alt Chunk already exists (dedup hit)
            Dedup-->>Chunk: Exists, increment reference_count
            Chunk-->>C: Skip upload — already stored
        else New chunk
            Dedup-->>Chunk: Not found
            C->>Block: Upload chunk bytes
            Block-->>C: Stored
            Chunk->>Dedup: Register new chunk_hash
        end
    end

    C->>Meta: Create FILE_VERSION with<br/>ordered list of chunk_hashes
    Meta-->>C: Version created, file_id updated
```

**Why content-addressed chunks:** Using the chunk's own content hash as its identifier means identical content — whether from the same user re-uploading, two different users with the same file, or unchanged sections across versions — is automatically recognized and deduplicated, without any explicit "is this the same file" logic needed.

---

## 5. Delta Sync — Detecting What Changed

```mermaid
flowchart TB
    A["File modified locally"] --> B["Sync Agent re-chunks<br/>the modified file"]
    B --> C["Compare new chunk hash list<br/>against previous version's<br/>chunk hash list"]
    C --> D{"Which chunks differ?"}
    D --> E["Unchanged chunks:<br/>no action needed"]
    D --> F["Changed/new chunks:<br/>upload only these"]
    F --> G["Create new FILE_VERSION<br/>referencing mix of old<br/>+ new chunk hashes"]
```

*A single-line edit at the start of a large document might shift byte offsets throughout the file — this is why **content-defined chunking** (using rolling hash boundaries like Rabin fingerprinting, rather than fixed byte offsets) is used in production systems: it re-anchors chunk boundaries around actual content changes, so an insertion near the top doesn't cause every subsequent chunk to appear "changed" due to offset shifting.*

---

## 6. Multi-Device Sync Propagation

```mermaid
sequenceDiagram
    participant D1 as Device 1 (uploads change)
    participant SyncSvc as Sync Service
    participant Meta as Metadata Service
    participant Notif as Notification Service
    participant D2 as Device 2
    participant D3 as Device 3 (offline)

    D1->>SyncSvc: Upload new FILE_VERSION
    SyncSvc->>Meta: Persist new version
    SyncSvc->>Notif: Broadcast change event<br/>for this file to all other devices

    Notif->>D2: Push "file changed" notification<br/>(device is online)
    D2->>SyncSvc: Request delta<br/>(current_version vs latest_version)
    SyncSvc-->>D2: List of changed chunk_hashes
    D2->>D2: Download only changed chunks,<br/>reconstruct file locally

    Note over D3: Device 3 is offline — misses push
    Note over D3: Later, Device 3 reconnects
    D3->>SyncSvc: Poll: "what's changed since<br/>my last known version?"
    SyncSvc-->>D3: Diff since last sync point
    D3->>D3: Apply delta, catch up
```

---

## 7. Conflict Resolution (Concurrent Offline Edits)

```mermaid
flowchart TB
    A["Device A edits File X offline<br/>(based on version 5)"] --> B["Device B also edits<br/>File X offline (based on version 5)"]
    B --> C["Device A reconnects first,<br/>uploads as version 6"]
    C --> D["Device B reconnects,<br/>tries to upload based on version 5"]
    D --> E{"Sync Service:<br/>base_version (5) !=<br/>current_version (6)?"}
    E -- "Conflict detected" --> F["Do NOT silently overwrite"]
    F --> G["Create both as separate files:<br/>'File X' (version 6, from A)<br/>'File X (Device B's conflicted copy)'<br/>(new version, from B)"]
    G --> H["Notify both users of<br/>the conflict for manual resolution"]
```

**Why this approach:** Silently picking a "winner" (e.g., last-write-wins) risks silently destroying a user's work — unacceptable for a file storage product. Instead, the system detects the version mismatch and preserves **both** edits as separate items, letting the user manually reconcile — the same strategy Dropbox and Google Drive actually use ("conflicted copy" files).

---

## 8. File Versioning & Storage Reclamation

```mermaid
flowchart TB
    A["File has 50 historical versions"] --> B["Each version references<br/>a subset of chunks"]
    B --> C["User deletes an old version<br/>(or retention policy expires it)"]
    C --> D["Decrement reference_count<br/>for each chunk in that version"]
    D --> E{"Any chunk's<br/>reference_count == 0?"}
    E -- Yes --> F["Chunk eligible for<br/>garbage collection<br/>(no version anywhere references it)"]
    E -- No --> G["Chunk still in use<br/>by another version/file —<br/>keep it"]
```

*This reference-counting garbage collection is what makes deduplication safe — a chunk is only physically deleted once nothing in the entire system still points to it.*

---

## 9. Sharing & Permissions

```mermaid
flowchart LR
    A["User A shares Folder F<br/>with User B (edit permission)"] --> B["Share Service creates<br/>SHARE record"]
    B --> C["User B's file tree view<br/>now includes Folder F<br/>(logical link, not a copy)"]
    C --> D["User B edits a file in F"]
    D --> E["Permission check:<br/>does User B have 'edit' on this file?"]
    E -- Yes --> F["New FILE_VERSION created,<br/>visible to all users with access to F"]
    E -- No --> G["Reject — view-only"]
```

*Shared folders are implemented as metadata-level references, not physical file copies — this keeps storage costs proportional to actual unique content, not the number of users a folder is shared with.*

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Distributed File Storage HLD))
    Metadata Service
      File/folder tree
      Version history
      Permission checks
    Sync Service
      Change detection
      Delta computation
      Multi-device propagation
    Chunking Service
      Content-defined chunking
      Hash computation
    Block Storage
      Content-addressed object store
      Durable, replicated
    Dedup Index
      Chunk hash lookup
      Reference counting
    Notification Service
      Push change events
      Wakes up online devices
    Sharing Service
      Metadata-level folder linking
      Permission enforcement
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| File representation | Content-addressed chunks, not monolithic blobs | Enables delta sync and cross-user/cross-version deduplication naturally |
| Chunking strategy | Content-defined (rolling hash) over fixed-offset | Avoids cascading "everything changed" false positives from small edits near the start of a file |
| Conflict resolution | Preserve both versions as separate files, never silent overwrite | File storage products must never silently destroy user data — user must resolve conflicts explicitly |
| Sharing model | Metadata-level linking, not physical copying | Keeps storage cost proportional to unique content regardless of how widely something is shared |
| Storage reclamation | Reference-counted garbage collection | Makes deduplication safe — chunks only deleted when truly orphaned across the entire system |
| Consistency model | Eventually consistent across devices | Real-time strict consistency across arbitrarily many offline-capable devices isn't achievable; explicit conflict handling compensates |

---

## 12. Bottlenecks & Scaling Considerations

- **Metadata DB as a hot path** — every file operation touches metadata (tree structure, versions, permissions); needs to be highly available and low-latency, often sharded by `user_id` or `file_id` hash.
- **Small file overhead** — chunking a 10KB file into a single "chunk" still incurs metadata overhead per chunk; systems often special-case very small files to avoid chunking overhead disproportionate to file size.
- **Dedup index scale** — a global chunk hash index across exabytes of data is enormous; typically sharded by hash prefix, similar to a distributed hash table.
- **Sync storm on large shared folder changes** — if a folder shared with 10,000 people gets a new file, notification fanout to that many devices simultaneously needs the same hybrid push/pull thinking as a social feed's celebrity-fanout problem.
- **Mobile bandwidth/battery constraints** — mobile sync agents can't behave like desktop agents (constant background chunking/hashing drains battery); typically use coarser sync intervals, wifi-only large-file sync options, and deferred chunk hashing.
- **Garbage collection safety** — reference-count-based deletion must be carefully race-condition-free (a chunk shouldn't be deleted the instant its count hits zero if another version creation referencing it is in-flight); usually handled with a grace period before physical deletion.
- **Large file initial upload** — multi-GB files need resumable, parallel chunk upload (not a single monolithic request) to tolerate connection drops without restarting from scratch.
