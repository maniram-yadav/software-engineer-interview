# Design a Secondary Index System for a Distributed Database — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Support efficient lookups on non-primary-key attributes (e.g., "find user by email" when the table is sharded by user_id)
- Keep the secondary index consistent with the underlying primary data as it changes (inserts, updates, deletes)
- Support both exact-match and range queries on indexed attributes
- Support multiple secondary indexes per table without excessive write amplification

### Non-Functional Requirements
- **Consistency between primary data and index:** This is the central challenge — an index that returns stale or missing results undermines its entire purpose
- **Write performance:** Adding secondary indexes shouldn't catastrophically slow down primary writes
- **Query performance:** Secondary index lookups should approach the speed of primary key lookups, not require a full table scan
- **Scalability:** Must work across a sharded/partitioned primary dataset, not just a single-node database

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Primary table size | Billions of rows, sharded by primary key |
| Secondary indexes per table | 2-5 typical |
| Write amplification per indexed write | 1 primary write + 1 index write per index |
| Index consistency lag tolerance | Depends on model chosen — from zero to seconds |

---

## 2. The Core Problem — Why Secondary Indexes Are Hard in Distributed Databases

```mermaid
flowchart TB
    A["Table sharded by user_id<br/>(primary key)"] --> B["Query: 'find user<br/>WHERE email = X'<br/>(email is NOT the shard key)"]

    B --> C{"Without a secondary index"}
    C --> D["Must broadcast the query<br/>to EVERY shard (scatter-gather)<br/>since the email could<br/>belong to a user on ANY<br/>shard — expensive at scale,<br/>same problem seen in the<br/>E-commerce Sharding design"]

    B --> E{"With a secondary index"}
    E --> F["Index maps email → shard/location<br/>directly, avoiding the broadcast —<br/>BUT this index itself must now<br/>be kept synchronized with the<br/>primary data as it changes,<br/>across a distributed system"]

    G["This synchronization problem<br/>— keeping index and primary<br/>data consistent despite being<br/>physically separate, independently<br/>updatable pieces of state —<br/>is THE defining challenge<br/>of this design"] --> F
```

---

## 3. Two Fundamental Index Architectures

```mermaid
flowchart TB
    A["Secondary Index<br/>Architecture"] --> B["Local (Co-located) Secondary Index"]
    A --> C["Global Secondary Index"]

    B --> B1["Index entries stored on the<br/>SAME shard as the primary<br/>data they reference"]
    B --> B2["PRO: writes stay within<br/>a single shard — no<br/>cross-shard coordination,<br/>fully consistent with primary<br/>data automatically (same<br/>transaction/shard)"]
    B --> B3["CON: a query on the indexed<br/>attribute still requires<br/>broadcasting to ALL shards<br/>(since matching rows could<br/>be on any shard) — solves<br/>write consistency but NOT<br/>the scatter-gather read problem"]

    C --> C1["Index entries stored in a<br/>SEPARATE, independently-sharded<br/>structure (sharded by the<br/>INDEXED attribute, not the<br/>primary key)"]
    C --> C2["PRO: queries on the indexed<br/>attribute go directly to ONE<br/>shard of the index — no<br/>scatter-gather needed"]
    C --> C3["CON: writes now span TWO<br/>different shards (primary<br/>data's shard AND index's shard)<br/>— requires distributed<br/>coordination, introducing a<br/>consistency lag/complexity risk"]

    D["This design covers GLOBAL<br/>secondary indexes, since they<br/>solve the more valuable/harder<br/>problem — fast lookups on<br/>non-shard-key attributes,<br/>which is usually the actual goal"] --> C
```

---

## 4. High-Level Architecture (Global Secondary Index)

```mermaid
flowchart TB
    Client["Client Query"]

    subgraph Primary["Primary Data (sharded by user_id)"]
        PShard1[("Primary Shard 1")]
        PShard2[("Primary Shard 2")]
        PShard3[("Primary Shard 3")]
    end

    subgraph IndexLayer["Global Secondary Index (sharded by email)"]
        IShard1[("Index Shard 1")]
        IShard2[("Index Shard 2")]
    end

    subgraph SyncLayer["Index Synchronization"]
        CDC["CDC Stream<br/>(from primary shards)"]
        IndexUpdater["Index Updater Workers"]
    end

    Client -->|"Write: INSERT user<br/>{user_id, email}"| PShard1
    Client -->|"Read: WHERE email=X"| IShard1

    PShard1 --> CDC
    PShard2 --> CDC
    PShard3 --> CDC
    CDC --> IndexUpdater
    IndexUpdater --> IShard1
    IndexUpdater --> IShard2

    IShard1 -.->|"index entry points to<br/>primary shard + row"| PShard1
```

**Key idea:** Writes to primary data and updates to the secondary index are decoupled via CDC (the exact mechanism covered in the CDC Pipeline design) — a write completes fast against the primary shard, and the index catches up asynchronously. This is a deliberate consistency/performance tradeoff, not an oversight, and is explored fully below.

---

## 5. Data Model

```mermaid
erDiagram
    PRIMARY_TABLE {
        string user_id PK
        string email
        string name
    }
    SECONDARY_INDEX_EMAIL {
        string email PK "indexed attribute"
        string user_id "pointer back to primary record"
        string primary_shard_location
    }
```

---

## 6. Synchronous (Strongly Consistent) Index Update — Detailed Sequence

```mermaid
sequenceDiagram
    participant Client as Client
    participant Coordinator as Write Coordinator
    participant PShard as Primary Shard
    participant IShard as Index Shard

    Client->>Coordinator: INSERT user {user_id:1, email:'a@x.com'}

    Coordinator->>PShard: Write primary record
    Coordinator->>IShard: Write index entry<br/>{email:'a@x.com' → user_id:1}

    Note over Coordinator: Both writes coordinated<br/>as a single distributed<br/>transaction (2PC-style,<br/>same pattern as the<br/>Distributed Transaction<br/>Saga design)

    PShard-->>Coordinator: Prepared
    IShard-->>Coordinator: Prepared
    Coordinator->>PShard: Commit
    Coordinator->>IShard: Commit

    Coordinator-->>Client: Write acknowledged<br/>(ONLY after BOTH primary<br/>and index are durably committed)

    Note over Client: Guarantee: any subsequent<br/>read via the index will<br/>ALWAYS see this write —<br/>zero staleness window,<br/>at the cost of higher<br/>write latency (cross-shard<br/>coordination on every write)
```

---

## 7. Asynchronous (Eventually Consistent) Index Update — Detailed Sequence

```mermaid
sequenceDiagram
    participant Client as Client
    participant PShard as Primary Shard
    participant CDC as CDC Stream
    participant Updater as Index Updater
    participant IShard as Index Shard

    Client->>PShard: INSERT user {user_id:1, email:'a@x.com'}
    PShard-->>Client: Write acknowledged<br/>(FAST — no cross-shard<br/>coordination needed)

    PShard->>CDC: Change event captured<br/>(async, via transaction log tailing)
    CDC->>Updater: Consume change event
    Updater->>IShard: Write index entry<br/>{email:'a@x.com' → user_id:1}

    Note over Client: If a client queries the<br/>index IMMEDIATELY after the<br/>primary write, there's a<br/>brief window where the index<br/>hasn't caught up yet —<br/>a query for email='a@x.com'<br/>might return NOT FOUND<br/>momentarily, even though<br/>the primary record exists
```

**The core tradeoff this design must make explicit:** Synchronous updates guarantee the index is never stale, at the cost of significantly higher write latency (every write now requires cross-shard distributed transaction coordination). Asynchronous updates keep writes fast but introduce a brief window where the index can return stale/incomplete results — this is the same linearizability-vs-eventual-consistency tradeoff explored in that dedicated design, applied specifically to the primary/index consistency problem.

---

## 8. Read Flow via Secondary Index — Detailed Sequence

```mermaid
sequenceDiagram
    participant Client as Client
    participant Router as Query Router
    participant IShard as Index Shard
    participant PShard as Primary Shard

    Client->>Router: SELECT * FROM users<br/>WHERE email='a@x.com'

    Router->>Router: hash('a@x.com') → determine<br/>which INDEX shard owns this
    Router->>IShard: Lookup email='a@x.com'
    IShard-->>Router: {user_id:1, primary_shard_location:shard_3}

    Router->>PShard: Fetch full record for user_id=1<br/>from primary shard 3
    PShard-->>Router: Full user record
    Router-->>Client: Return result
```

**Why this is a two-hop lookup:** The index only stores enough information to LOCATE the primary record (the pointer), not necessarily the full record itself — this avoids duplicating the entire row's data in every index (which would multiply storage cost and, more importantly, multiply the consistency-maintenance burden across every field, not just the indexed one).

---

## 9. Handling Index Staleness — Read Repair / Verification (For Async Model)

```mermaid
flowchart TB
    A["Read via index returns<br/>a result"] --> B{"How much staleness<br/>tolerance is acceptable<br/>for this use case?"}

    B --> C["Trust the index result<br/>as-is (fast, no extra check)"]
    C --> C1["Appropriate for: search/discovery<br/>use cases where a brief<br/>staleness window is harmless<br/>(e.g., 'find users by interest')"]

    B --> D["Verify against primary data<br/>before returning<br/>(extra round trip)"]
    D --> D1["Appropriate for: correctness-<br/>sensitive lookups (e.g., login<br/>by email — MUST reflect the<br/>very latest state, can't risk<br/>a stale 'not found' for a<br/>just-registered user)"]

    E["This decision should be made<br/>PER USE CASE, not as a single<br/>global policy — the same index<br/>infrastructure can serve both<br/>needs, with the verification<br/>step being an opt-in choice<br/>at query time"] -.-> D
```

---

## 10. Handling Deletes and Updates (Index Cleanup)

```mermaid
sequenceDiagram
    participant Client as Client
    participant PShard as Primary Shard
    participant CDC as CDC Stream
    participant Updater as Index Updater
    participant IShard as Index Shard

    Client->>PShard: UPDATE user SET email='new@x.com'<br/>WHERE user_id=1<br/>(was 'old@x.com')

    PShard->>CDC: Change event:<br/>{before: email='old@x.com',<br/>after: email='new@x.com'}

    CDC->>Updater: Consume event
    Updater->>IShard: DELETE index entry<br/>for 'old@x.com'
    Updater->>IShard: INSERT index entry<br/>for 'new@x.com' → user_id:1

    Note over IShard: Both operations needed —<br/>simply inserting the new<br/>entry without removing the<br/>old one would leave a<br/>DANGLING, incorrect index<br/>entry pointing to data<br/>that no longer matches
```

**Why the before-image matters here:** This directly parallels why the WAL & Recovery System design stores before-images — without knowing what the OLD indexed value was, the index updater wouldn't know which stale entry to remove, and the index would silently accumulate incorrect entries over time.

---

## 11. Component Responsibilities Summary

```mermaid
mindmap
  root((Secondary Index HLD))
    Primary Data Shards
      Source of truth
      Sharded by primary key
    Global Index Shards
      Sharded by indexed attribute
      Points back to primary location
    Write Coordinator (sync model)
      2PC across primary and index
      Zero staleness, higher latency
    CDC and Index Updater (async model)
      Decoupled, eventually consistent
      Fast writes, brief staleness window
    Query Router
      Two-hop lookup: index then primary
      Determines index shard from query
    Read Repair Layer
      Optional verification step
      Per-use-case staleness tolerance
```

---

## 12. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Index architecture | Global secondary index (separately sharded) | Solves the actual goal — fast, single-shard lookups on non-primary-key attributes — unlike local indexes which still require scatter-gather reads |
| Consistency model | Configurable: synchronous (2PC) or asynchronous (CDC-based) | Different use cases genuinely need different points on the consistency/latency tradeoff spectrum; a single global choice would over- or under-serve some use cases |
| Index entry content | Pointer to primary record, not full duplicate | Avoids multiplying storage and consistency-maintenance burden across every field, not just the indexed one |
| Delete/update handling | Explicit removal of stale entries via before-image tracking | Prevents dangling, incorrect index entries from silently accumulating |
| Read verification | Optional, per-use-case | Balances read latency against staleness tolerance based on actual application requirements, not a one-size-fits-all policy |

---

## 13. Bottlenecks & Scaling Considerations

- **Write amplification** — every additional secondary index roughly doubles (or more) the write cost for indexed writes; systems typically limit the number of secondary indexes per table and encourage careful selection of which attributes genuinely need indexing.
- **CDC lag under high write volume** — in the async model, if the CDC/index-updater pipeline falls behind the primary write rate, the staleness window grows beyond its normal brief duration; needs the same lag monitoring discussed in the CDC Pipeline design.
- **Index shard hot spots** — if the indexed attribute has a skewed value distribution (e.g., many users share very few distinct values), the index shards can become as unevenly loaded as primary shards can with a poor shard key — the same shard key selection principles from the E-commerce Sharding design apply here to the index's own sharding scheme.
- **Cross-shard transaction cost for synchronous model** — every single indexed write pays the latency and complexity cost of distributed transaction coordination; this compounds if a table has multiple synchronous secondary indexes, since each requires its own coordination.
- **Index rebuild for schema/index changes** — adding a NEW secondary index to an existing large table requires a full backfill (scanning all existing primary data to populate the new index) — this needs to be done as a careful, throttled background process (similar to the CDC pipeline's initial snapshot phase) without disrupting live traffic.
- **Orphaned index entries from partial failures** — in the asynchronous model, if the index-updater crashes between processing a delete's "remove old entry" and "insert new entry" steps, the index can be left in a transiently inconsistent state; needs idempotent, resumable processing (tracked via committed offsets, same pattern as the CDC Pipeline design) to guarantee eventual correctness even after failures mid-update.
