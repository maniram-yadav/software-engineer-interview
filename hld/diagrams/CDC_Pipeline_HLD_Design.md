# Design a Change Data Capture (CDC) Pipeline — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Capture every insert, update, and delete happening in a source database, in the exact order they occurred
- Stream these changes to downstream consumers (search indexes, caches, analytics warehouses, other microservices)
- Preserve transactional boundaries where relevant (changes from a single source transaction should be identifiable as a group)
- Support multiple independent downstream consumers reading the same change stream

### Non-Functional Requirements
- **Low source impact:** Capturing changes must NOT meaningfully degrade the performance of the source production database
- **No missed changes:** Every committed change must eventually reach downstream consumers — this is a durability/completeness requirement
- **Low latency:** Changes should propagate to downstream systems within seconds, not minutes
- **Ordering guarantee:** Changes to the same row must be delivered downstream in the same order they were committed at the source

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Source database write rate | ~10,000-50,000 changes/sec |
| CDC propagation latency target | < 1-5 seconds |
| Downstream consumers | Multiple independent (search, cache, analytics, other services) |
| Change event size | ~500 bytes - few KB (depending on row size) |

---

## 2. The Core Design Choice — Log-Based CDC vs Query-Based Polling

```mermaid
flowchart TB
    A["CDC Implementation<br/>Approach"] --> B["Query-Based Polling<br/>(periodically SELECT rows<br/>WHERE updated_at > last_check)"]
    A --> C["Log-Based CDC<br/>(tail the database's own<br/>internal transaction log)"]

    B --> B1["CON: adds direct query load<br/>to the production database<br/>on every poll cycle"]
    B --> B2["CON: cannot reliably detect<br/>DELETES (the row is just<br/>gone, nothing to query)"]
    B --> B3["CON: polling interval directly<br/>trades off latency vs<br/>source database load"]

    C --> C1["PRO: reads the database's<br/>OWN internal replication log<br/>(e.g., MySQL binlog,<br/>PostgreSQL WAL) — the exact<br/>same mechanism the database<br/>uses for its own replication"]
    C --> C2["PRO: captures EVERY change<br/>including deletes, with<br/>minimal overhead — essentially<br/>free from the database's<br/>perspective, since it's just<br/>reading a log it already writes"]
    C --> C3["PRO: naturally preserves<br/>exact commit order"]

    D["CHOSEN: Log-based CDC —<br/>the industry-standard approach<br/>(e.g., Debezium) for exactly<br/>these reasons"] --> C
```

**Why log-based CDC is almost universally preferred in production:** Every transactional database already maintains an internal write-ahead log for its own crash recovery and replication purposes (this connects directly to the WAL & Recovery System design). Log-based CDC simply taps into this existing, already-durable, already-ordered stream — rather than adding new query load or missing deletes entirely, as polling-based approaches do.

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph Source["Source Database"]
        OLTP[("Production Database<br/>(MySQL/PostgreSQL)")]
        TxLog[("Internal Transaction Log<br/>(binlog/WAL)")]
    end

    subgraph CDCLayer["CDC Capture Layer"]
        LogReader["Log Reader Connector<br/>(e.g., Debezium)"]
        OffsetStore[("Offset/Position Store<br/>tracks last-read log position")]
    end

    subgraph Streaming["Distribution Layer"]
        Kafka["Kafka<br/>(per-table topics)"]
    end

    subgraph Consumers["Downstream Consumers"]
        SearchIdx["Search Index Updater"]
        CacheInv["Cache Invalidator"]
        Analytics["Analytics Warehouse ETL"]
        OtherSvc["Other Microservice<br/>(event-driven sync)"]
    end

    OLTP --> TxLog
    TxLog --> LogReader
    LogReader --> OffsetStore
    LogReader --> Kafka

    Kafka --> SearchIdx
    Kafka --> CacheInv
    Kafka --> Analytics
    Kafka --> OtherSvc
```

**Key idea:** The Log Reader Connector is the only component that ever touches the source database directly — and it does so by reading a log the database was already writing anyway, not by issuing queries against live tables. Everything downstream consumes from Kafka, completely decoupled from the source database's load and availability.

---

## 4. Data Model — Change Event Structure

```mermaid
erDiagram
    CHANGE_EVENT {
        string event_id PK
        string table_name
        string operation "INSERT/UPDATE/DELETE"
        map before_state "null for INSERT"
        map after_state "null for DELETE"
        long source_log_position "LSN/binlog offset"
        long transaction_id
        timestamp committed_at
    }
```

```mermaid
flowchart LR
    A["Source DB: UPDATE orders<br/>SET status='shipped'<br/>WHERE order_id=123"] --> B["Change Event:<br/>{table: orders, op: UPDATE,<br/>before: {status:'pending'},<br/>after: {status:'shipped'},<br/>order_id: 123,<br/>log_position: 4829103}"]
```

**Why both before and after state matter:** Downstream consumers often need more than just "this changed" — a cache invalidator just needs the key, but an analytics pipeline computing "time in each order status" needs the before-state to calculate duration, and audit/compliance consumers need the full before/after diff for record-keeping.

---

## 5. Change Capture Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant App as Application
    participant DB as Source Database
    participant TxLog as Transaction Log
    participant Reader as Log Reader Connector
    participant Offset as Offset Store
    participant K as Kafka

    App->>DB: UPDATE orders SET status='shipped'<br/>WHERE order_id=123
    DB->>TxLog: Write to internal transaction log<br/>(normal DB operation,<br/>NOT CDC-specific)
    DB-->>App: Commit acknowledged

    loop Continuous tailing
        Reader->>TxLog: Read next log entry<br/>from last known position
        TxLog-->>Reader: Log entry (the UPDATE above)

        Reader->>Reader: Parse into structured<br/>change event
        Reader->>K: Publish to topic "orders-changes"

        Reader->>Offset: Record new log position<br/>(for crash recovery —<br/>resume from here, not<br/>from the beginning)
    end
```

---

## 6. Handling Initial Snapshot (Bootstrapping)

```mermaid
flowchart TB
    A["New CDC pipeline starting up<br/>for the FIRST time"] --> B["Problem: the transaction log<br/>only contains RECENT changes —<br/>it doesn't have the FULL<br/>current state of a table that<br/>already has millions of<br/>existing rows"]

    B --> C["Solution: Initial Snapshot Phase"]
    C --> D["Step 1: Record current<br/>log position (e.g., LSN=5000)<br/>BEFORE starting the snapshot"]
    C --> E["Step 2: Bulk-read the ENTIRE<br/>current table state<br/>(e.g., SELECT * FROM orders),<br/>publish each row as a<br/>synthetic 'INSERT' event"]
    C --> F["Step 3: AFTER snapshot<br/>completes, resume normal<br/>log-tailing from the<br/>recorded position (LSN=5000)"]

    G["Why record the position<br/>BEFORE snapshotting?"] --> H["Any changes that occur<br/>DURING the (potentially<br/>long-running) snapshot are<br/>captured in the log starting<br/>from LSN=5000 — resuming<br/>from exactly this point<br/>ensures NO changes are<br/>missed and NONE are<br/>double-processed"]
```

```mermaid
sequenceDiagram
    participant Reader as Log Reader Connector
    participant DB as Source Database
    participant TxLog as Transaction Log
    participant K as Kafka

    Reader->>TxLog: Record current position: LSN=5000
    Reader->>DB: Bulk read: SELECT * FROM orders<br/>(potentially millions of rows,<br/>done in batches)
    DB-->>Reader: Rows, streamed in batches

    loop For each row in snapshot
        Reader->>K: Publish as synthetic INSERT event
    end

    Note over Reader: Snapshot complete —<br/>now resume normal tailing
    Reader->>TxLog: Resume reading from LSN=5000<br/>(captures anything that<br/>happened DURING the snapshot)
```

---

## 7. Ordering Guarantees Across Multiple Tables/Partitions

```mermaid
flowchart TB
    A["Kafka topic partitioning<br/>strategy for change events"] --> B{"Partition by what?"}

    B --> C["Partition by TABLE"]
    C --> C1["All changes to 'orders' table<br/>go to one partition —<br/>preserves order for that<br/>table, but different tables<br/>have no relative ordering<br/>guarantee (usually fine,<br/>since they're independent)"]

    B --> D["Partition by PRIMARY KEY<br/>(e.g., hash(order_id))"]
    D --> D1["All changes to a SPECIFIC ROW<br/>go to the same partition —<br/>guarantees a consumer sees<br/>that row's changes in the<br/>exact order they were<br/>committed, even amid<br/>massive overall throughput"]

    E["CHOSEN: partition by primary key<br/>(same principle as the Message<br/>Queue design's key-based<br/>partitioning) — because<br/>most downstream consumers<br/>care most about per-row<br/>ordering, not global<br/>cross-row ordering"] --> D
```

---

## 8. Handling Source Database Failover

```mermaid
sequenceDiagram
    participant Primary as Source DB Primary
    participant Reader as Log Reader Connector
    participant Offset as Offset Store
    participant NewPrimary as New DB Primary<br/>(after failover)

    Note over Primary: Primary crashes,<br/>database cluster fails over

    Reader->>Reader: Detect connection loss<br/>to Primary
    Reader->>Offset: Retrieve last confirmed<br/>log position

    Reader->>NewPrimary: Reconnect, resume tailing<br/>from last confirmed position

    alt New primary has the position<br/>(was a fully-caught-up replica)
        NewPrimary-->>Reader: Resume successfully,<br/>no data loss
    else Position not available<br/>(replica was behind)
        NewPrimary-->>Reader: Position not found —<br/>requires re-snapshot or<br/>manual intervention
        Note over Reader: This gap risk is why<br/>database replication lag<br/>monitoring matters for<br/>CDC reliability, not just<br/>database availability
    end
```

---

## 9. Downstream Consumer Patterns

```mermaid
flowchart TB
    A["Change Event Stream<br/>(Kafka topic)"] --> B["Search Index Updater"]
    A --> C["Cache Invalidator"]
    A --> D["Analytics ETL"]
    A --> E["Cross-Service Sync"]

    B --> B1["On UPDATE/INSERT: re-index<br/>the document in Elasticsearch<br/>On DELETE: remove from index"]
    C --> C1["On ANY change to a row:<br/>invalidate/evict the<br/>corresponding cache key<br/>(same pattern as cache-aside<br/>invalidation in the<br/>Distributed Cache design)"]
    D --> D1["Transform and load into<br/>columnar OLAP store<br/>(directly feeds the OLAP<br/>System design's ingestion layer)"]
    E --> E1["Another microservice reacts<br/>to the change as a domain<br/>event (e.g., Inventory Service<br/>reacts to Order Service's<br/>changes) — same pattern as<br/>choreography-based sagas"]
```

*This is precisely why CDC is such a foundational infrastructure pattern — it's the underlying mechanism that powers cache invalidation, search indexing, OLAP ingestion, and event-driven microservice communication, all from a single, low-overhead capture point at the source database.*

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((CDC Pipeline HLD))
    Log Reader Connector
      Tails source transaction log
      Zero query load on source
      Handles initial snapshot
    Offset Store
      Tracks last-read log position
      Enables crash-safe resume
    Kafka Distribution Layer
      Per-primary-key partitioning
      Preserves per-row ordering
      Multiple independent consumers
    Downstream Consumers
      Search indexing
      Cache invalidation
      Analytics ETL
      Cross-service event sync
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Capture mechanism | Log-based (tail transaction log), not query polling | Near-zero source database overhead, captures deletes correctly, naturally preserves commit order |
| Initial data load | Snapshot-then-tail with recorded position boundary | Ensures no changes are missed or double-processed during the transition from snapshot to live tailing |
| Event partitioning | By primary key (row identity) | Guarantees per-row change ordering for consumers, which is the ordering guarantee that actually matters for most use cases |
| Distribution layer | Kafka, decoupling source from consumers | Multiple independent downstream systems can consume the same change stream without adding load back to the source database |
| Position tracking | Persistent offset store, resumable on restart/failover | Enables crash-safe recovery without needing to re-snapshot the entire source database |

---

## 12. Bottlenecks & Scaling Considerations

- **Source database log retention** — the transaction log itself typically has limited retention (databases don't keep it forever); if the CDC connector falls too far behind or is down too long, the needed log segments may have been purged, forcing a full re-snapshot — monitoring connector lag relative to log retention window is critical.
- **Large table initial snapshot time** — bootstrapping CDC for a table with billions of existing rows can take a very long time; needs to be done in a way that doesn't overwhelm the source database (throttled, batched reads) and should ideally run during lower-traffic periods for the first deployment.
- **Schema change handling** — when the source table's schema changes (column added/removed/renamed), the CDC pipeline must handle this gracefully — typically by also capturing schema-change events from the log and propagating schema evolution to downstream consumers rather than breaking silently.
- **Downstream consumer lag** — if a consumer (e.g., the search indexer) falls behind the change stream, Kafka's durable buffering (same pattern as the general Message Queue design) absorbs this without data loss, but monitoring per-consumer lag is essential to catch systemic issues before they become large backlogs.
- **Multi-table transaction atomicity downstream** — if a single source transaction modified multiple tables atomically, but those tables' change events land in different Kafka partitions/topics, downstream consumers reconstructing "what happened in this transaction" need the shared transaction_id to correlate related events — this correlation logic must be deliberately preserved through the pipeline, not assumed.
- **Connector high availability** — the Log Reader Connector itself is a critical single point in the pipeline; production deployments typically run it with failover capability (though usually as an active-passive pair, since only one instance should be tailing the log at a time to avoid duplicate event production).
- **Cost of maintaining exactly-once semantics end-to-end** — while the source log-tailing itself is naturally ordered and complete, guaranteeing downstream consumers process each event exactly once (not just at-least-once) requires the same idempotency patterns covered in the Idempotent API Requests design — CDC delivers reliable at-least-once delivery, and consumers are responsible for their own deduplication.
