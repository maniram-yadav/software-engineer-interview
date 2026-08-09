# Design a Document Versioning & History System — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Store the complete edit history of a document, supporting "restore to any prior version"
- Show a human-readable diff between any two versions
- Support efficient storage — don't naively store a full copy of the document for every single edit
- Support named/labeled snapshots (e.g., "final draft," explicit save points) alongside continuous auto-versioning
- Attribute each change to a specific user and timestamp

### Non-Functional Requirements
- **Storage efficiency:** A document edited thousands of times over its life shouldn't require thousands of full copies
- **Retrieval speed:** Reconstructing any historical version should be fast, not requiring an expensive replay of the entire edit history from the beginning
- **Durability:** Version history is often legally/organizationally important — must never be silently lost
- **Scale:** Millions of documents, each with potentially thousands of versions

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Documents | ~1B |
| Avg versions per document (over lifetime) | ~500-2,000 (frequent auto-save) |
| Avg document size | ~50KB |
| Naive storage (full copy per version) | Would be ~50-100TB just for one popular document set — clearly infeasible |
| Diff-based storage (actual) | Orders of magnitude smaller, proportional to actual changes |

---

## 2. The Core Problem — Why Storing Full Copies Doesn't Scale

```mermaid
flowchart TB
    A["Document edited 1,000 times<br/>over its lifetime,<br/>each edit changes ~1% of content"] --> B{"Storage Strategy"}

    B --> C["Naive: store FULL COPY<br/>for every version"]
    C --> C1["1,000 versions × 50KB<br/>= 50MB for ONE document —<br/>at 1B documents, this is<br/>catastrophically expensive<br/>and almost entirely redundant<br/>data (99% of each version<br/>is identical to the last)"]

    B --> D["Diff-based: store only<br/>WHAT CHANGED between<br/>consecutive versions"]
    D --> D1["1,000 small diffs, each<br/>proportional to the ~1%<br/>actual change — dramatically<br/>smaller total storage,<br/>directly proportional to<br/>actual edit volume, not<br/>version COUNT × full size"]
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    Client["Client<br/>(Editor)"]
    Gateway["API Gateway"]

    subgraph Core["Core Services"]
        DocSvc["Document Service"]
        VersionSvc["Versioning Service"]
        DiffEngine["Diff/Patch Engine"]
    end

    subgraph Storage["Storage Layer"]
        SnapshotStore[("Snapshot Store<br/>periodic FULL copies)")]
        DeltaStore[("Delta/Diff Store<br/>incremental changes<br/>between snapshots)")]
        MetaDB[("Version Metadata DB<br/>version_id, author, timestamp,<br/>label)")]
    end

    Client --> Gateway --> DocSvc
    DocSvc --> VersionSvc
    VersionSvc --> DiffEngine
    DiffEngine --> DeltaStore
    VersionSvc --> SnapshotStore
    VersionSvc --> MetaDB
```

**Key idea:** This design uses a **hybrid snapshot + delta chain** approach — periodic full snapshots (e.g., every 50th version) act as fast "anchor points," with delta/diff records capturing the incremental changes between them. Reconstructing any version means starting from the nearest preceding snapshot and applying a bounded, small number of deltas — never replaying the entire edit history from the document's creation.

---

## 4. Data Model

```mermaid
erDiagram
    DOCUMENT ||--o{ VERSION : "has history of"
    VERSION ||--o| SNAPSHOT : "may be a full snapshot"
    VERSION ||--o| DELTA : "or a delta from previous"

    DOCUMENT {
        string document_id PK
        string current_version_id FK
    }
    VERSION {
        string version_id PK
        string document_id FK
        string author_id
        timestamp created_at
        string label "nullable, e.g. 'Final Draft'"
        string storage_type "snapshot/delta"
        string prev_version_id FK
    }
        SNAPSHOT {
        string version_id FK
        bytes full_content
    }
    DELTA {
        string version_id FK
        bytes diff_patch "e.g. in a diff format<br/>like Myers diff or JSON patch"
    }
```

---

## 5. Writing a New Version — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client
    participant DocSvc as Document Service
    participant VerSvc as Versioning Service
    participant Diff as Diff Engine
    participant Delta as Delta Store
    participant Snap as Snapshot Store
    participant Meta as Version Metadata DB

    C->>DocSvc: Save document (new content)
    DocSvc->>VerSvc: Create new version

    VerSvc->>VerSvc: Check: has it been N versions<br/>(e.g., 50) since the last<br/>snapshot for this document?

    alt Time for a new snapshot
        VerSvc->>Snap: Store FULL content<br/>as new snapshot
        VerSvc->>Meta: Record version:<br/>storage_type=SNAPSHOT
    else Normal incremental version
        VerSvc->>Diff: Compute diff between<br/>previous version's content<br/>and new content
        Diff-->>VerSvc: Compact diff/patch
        VerSvc->>Delta: Store the diff
        VerSvc->>Meta: Record version:<br/>storage_type=DELTA,<br/>prev_version_id=X
    end

    VerSvc-->>DocSvc: New version_id created
    DocSvc-->>C: Save confirmed
```

---

## 6. Reconstructing a Historical Version — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client
    participant VerSvc as Versioning Service
    participant Meta as Version Metadata DB
    participant Snap as Snapshot Store
    participant Delta as Delta Store

    C->>VerSvc: Get document as of version_id=V847

    VerSvc->>Meta: Walk backward from V847<br/>to find the nearest<br/>preceding SNAPSHOT
    Meta-->>VerSvc: Nearest snapshot: V800<br/>(chain: V800→V801→...→V847,<br/>47 deltas to apply)

    VerSvc->>Snap: Fetch full content at V800
    Snap-->>VerSvc: Base document content

    loop Apply each delta in order, V801 through V847
        VerSvc->>Delta: Fetch delta for this version
        Delta-->>VerSvc: Diff/patch
        VerSvc->>VerSvc: Apply patch to running content
    end

    VerSvc-->>C: Reconstructed content at V847
```

**Why the snapshot interval bounds reconstruction cost:** With snapshots every 50 versions, reconstructing ANY version requires at most 50 delta applications — regardless of whether the document has 100 or 100,000 total versions in its history. This is precisely the same "bound recovery cost via periodic checkpoints" principle used in the WAL & Recovery System design, applied here to document version reconstruction instead of database crash recovery.

---

## 7. Diff Algorithm Choice

```mermaid
flowchart TB
    A["Diff Algorithm Options"] --> B["Line-based diff<br/>(Myers algorithm —<br/>like 'git diff')"]
    A --> C["Character/word-based diff"]
    A --> D["Structural diff<br/>(operates on the document's<br/>actual data structure —<br/>e.g., JSON patch, or<br/>rich-text operation log)"]

    B --> B1["Good for: plain text,<br/>code documents"]
    C --> C1["Good for: prose documents<br/rich text where line<br/>boundaries are less meaningful"]
    D --> D1["Good for: structured documents<br/>(rich text with formatting,<br/>tables, embedded objects) —<br/>diffing the underlying DATA<br/>MODEL rather than rendered<br/>text avoids losing formatting<br/>information in the diff"]

    E["Choice depends on document<br/>type — a rich text editor<br/>(like the Collaborative Document<br/>Editor design) benefits from<br/>structural diffs that align<br/>with its own operation model,<br/>since it likely ALREADY has<br/>an operation log from<br/>real-time collaborative editing<br/>that can double as version history"]
```

**Connection to the Collaborative Document Editor design:** If the same product already implements real-time collaborative editing (via Operational Transformation, as in that earlier design), the operation log used for real-time sync is often the SAME underlying data that powers version history — each "version" boundary is simply a labeled point in that same operation stream, avoiding the need for an entirely separate diffing mechanism.

---

## 8. Named/Labeled Versions (Explicit Save Points)

```mermaid
flowchart TB
    A["Continuous auto-versioning<br/>(every save = new version,<br/>happens automatically,<br/>often frequent)"] --> B["User explicitly labels<br/>a specific version:<br/>'Final Draft - Sent to Client'"]
    B --> C["Metadata DB: add label<br/>field to that version_id —<br/>NO special storage treatment<br/>needed, it's just a<br/>queryable annotation on<br/>an existing version"]

    D["Benefit: labeled versions<br/>appear prominently in the<br/>version history UI, while<br/>the full continuous history<br/>remains available but<br/>de-emphasized — same<br/>underlying storage mechanism,<br/>different presentation"] --> C
```

---

## 9. Snapshot Interval Tuning (Storage vs Reconstruction Speed Tradeoff)

```mermaid
flowchart TB
    A["Snapshot Interval Choice"] --> B["Frequent snapshots<br/>(e.g., every 10 versions)"]
    A --> C["Infrequent snapshots<br/>(e.g., every 200 versions)"]

    B --> B1["PRO: fast reconstruction<br/>(few deltas to apply)"]
    B --> B2["CON: more storage<br/>(more full copies)"]

    C --> C1["PRO: less storage<br/>(fewer full copies)"]
    C --> C2["CON: slower reconstruction<br/>for versions far from the<br/>nearest snapshot (more<br/>deltas to apply in sequence)"]

    D["Adaptive approach: snapshot<br/>frequency can scale with<br/>ACCESS PATTERNS — e.g.,<br/>snapshot more frequently<br/>near the CURRENT/recent<br/>versions (most commonly<br/>accessed) and less frequently<br/>for old, rarely-viewed<br/>history"] -.-> B
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Document Versioning HLD))
    Versioning Service
      Orchestrates save flow
      Decides snapshot vs delta
      Reconstruction logic
    Diff Engine
      Computes compact diffs
      Algorithm depends on doc type
    Snapshot Store
      Periodic full copies
      Fast reconstruction anchors
    Delta Store
      Incremental changes
      Bulk of storage savings
    Version Metadata DB
      Author, timestamp, labels
      Chain linkage (prev_version_id)
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Storage strategy | Hybrid snapshot + delta chain | Bounds reconstruction cost (limited deltas to replay) while avoiding the storage explosion of full copies per version |
| Snapshot interval | Periodic (e.g., every 50 versions), possibly adaptive | Balances storage efficiency against reconstruction speed; tunable per access pattern |
| Diff algorithm | Depends on document type (line/character/structural) | Structural diffs preserve rich formatting information that plain text diffs would lose |
| Labeled versions | Metadata annotation, not separate storage | Reuses the same underlying version chain; labels are just a queryable attribute |
| Integration with real-time editing | Shared operation log where applicable | Avoids maintaining two separate change-tracking systems when one (the collaborative editor's op log) can serve both purposes |

---

## 12. Bottlenecks & Scaling Considerations

- **Reconstruction cost for pathologically long delta chains** — if snapshot creation logic has a bug (or is disabled for some documents), a delta chain could grow unbounded, making old-version reconstruction increasingly slow; needs monitoring and enforcement of the snapshot interval invariant.
- **Diff computation cost for very large documents** — computing a diff for a massive document (e.g., a huge spreadsheet or codebase file) on every save can itself become a performance bottleneck; may need to bound diff computation time or fall back to snapshot-only storage for exceptionally large documents.
- **Storage growth for extremely long-lived, frequently-edited documents** — even with efficient delta storage, documents edited continuously over years accumulate substantial history; may need a retention/pruning policy for very old, non-labeled versions (e.g., "keep every version from the last 30 days, but only daily snapshots beyond that") — similar to the tiered retention pattern in the Time-Series Database and Log Aggregation designs.
- **Concurrent version creation** — if collaborative editing allows near-simultaneous saves, the versioning service needs clear ordering logic (which is often already solved by the collaborative editor's own operation sequencing, if integrated as described in Section 7).
- **Cross-version diff/comparison UI performance** — showing a diff between two ARBITRARY historical versions (not just consecutive ones) requires reconstructing both full versions first, then diffing them — this is a more expensive operation than simple sequential reconstruction and may benefit from caching frequently-compared version pairs.
- **Storage backend choice for deltas** — delta records are typically small and numerous; a storage engine optimized for many small sequential writes (similar operational profile to a WAL) is often a better fit than a general-purpose document store for this specific access pattern.
