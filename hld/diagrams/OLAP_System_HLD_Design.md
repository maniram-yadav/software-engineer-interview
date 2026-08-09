# Design an OLAP System for Real-Time Business Intelligence — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Support complex analytical queries: multi-dimensional aggregations, GROUP BY across many dimensions, drill-down/roll-up
- Ingest data continuously from operational systems (orders, users, events) with near-real-time freshness
- Support ad-hoc, exploratory queries from business analysts (not just predefined dashboards)
- Handle both real-time ("what happened in the last hour") and historical ("compare this quarter to last year") queries

### Non-Functional Requirements
- **Query latency:** Complex aggregation queries over billions of rows should return in seconds, not minutes
- **Freshness:** Data should be queryable within minutes of being generated in operational systems (this is what makes it "real-time" BI, distinct from traditional overnight-batch data warehouses)
- **Scale:** Petabyte-scale historical data, with continuous high-volume ingestion
- **Concurrent analyst usage:** Many analysts running complex, resource-intensive queries simultaneously without starving each other

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Events/transactions ingested/sec | ~100,000 |
| Total historical data | Petabyte scale |
| Typical query scan volume | Millions to billions of rows per query |
| Concurrent analyst queries | Hundreds |
| Freshness target | < 5 minutes from event to queryable |

---

## 2. OLTP vs OLAP — Why a Separate System Is Necessary

```mermaid
flowchart TB
    A["OLTP (operational databases)<br/>— e.g., the orders DB<br/>powering checkout"] --> A1["Optimized for: many small,<br/>fast, precise transactions<br/>(row-oriented storage,<br/>indexed point lookups)"]
    A --> A2["Query pattern: 'get THIS<br/>order by ID' — touches<br/>a few rows"]

    B["OLAP (analytical systems)<br/>— e.g., 'total revenue by<br/>region by product category<br/>this quarter'"] --> B1["Optimized for: scanning and<br/>aggregating MASSIVE numbers<br/>of rows, but only a FEW<br/>columns at a time<br/>(columnar storage)"]
    B --> B2["Query pattern: touches<br/>millions/billions of rows,<br/>but only 3-4 columns<br/>out of possibly 50+"]

    C["Running OLAP-style queries<br/>directly against OLTP databases<br/>would either be catastrophically<br/>slow OR would degrade the<br/>OLTP system's transactional<br/>performance for real customers —<br/>hence a SEPARATE, purpose-built<br/>system is standard practice"] --> B1
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph Sources["Operational Data Sources"]
        OrdersDB[("Orders DB<br/>(OLTP)")]
        UsersDB[("Users DB<br/>(OLTP)")]
        EventStream["Clickstream/Event<br/>Producers"]
    end

    subgraph Ingestion["Ingestion Layer"]
        CDC["CDC Connectors<br/>(stream DB changes)"]
        Kafka["Kafka<br/>(unified event/change stream)"]
    end

    subgraph Processing["Stream Processing"]
        StreamETL["Stream Processor<br/>(transform, denormalize,<br/>enrich)"]
    end

    subgraph Storage["OLAP Storage Engine"]
        ColumnStore[("Columnar Store<br/>(sharded, compressed)")]
        MaterializedViews[("Pre-aggregated<br/>Materialized Views")]
    end

    subgraph QueryPath["Query Layer"]
        QueryEngine["Distributed Query Engine<br/>(MPP — Massively Parallel<br/>Processing)"]
        BI["BI Tools / Dashboards /<br/>Ad-hoc SQL"]
    end

    OrdersDB --> CDC
    UsersDB --> CDC
    EventStream --> Kafka
    CDC --> Kafka

    Kafka --> StreamETL --> ColumnStore
    ColumnStore --> MaterializedViews

    BI --> QueryEngine
    QueryEngine --> ColumnStore
    QueryEngine --> MaterializedViews
```

**Key idea:** Data flows continuously from operational systems into the OLAP store via CDC and streaming (not nightly batch dumps, which is the traditional data warehouse approach) — this is precisely what makes the system "real-time" rather than "next-day." Once ingested, data is stored in a fundamentally different physical layout (columnar) optimized for the very different query pattern OLAP demands.

---

## 4. Columnar Storage — The Core Technical Foundation

```mermaid
flowchart TB
    A["Row-Oriented Storage<br/>(OLTP style)"] --> B["Row 1: [order_id:1, user:A,<br/>amount:50, region:US, date:...]"]
    A --> C["Row 2: [order_id:2, user:B,<br/>amount:75, region:EU, date:...]"]
    D["Query: SUM(amount) WHERE<br/>region='US'"] --> E["Must read EVERY COLUMN<br/>of EVERY row, even though<br/>only 'amount' and 'region'<br/>are actually needed —<br/>wasteful I/O"]

    F["Column-Oriented Storage<br/>(OLAP style)"] --> G["amount column: [50, 75, ...]<br/>(stored contiguously)"]
    F --> H["region column: [US, EU, ...]<br/>(stored contiguously)"]
    I["Same query"] --> J["Reads ONLY the amount and<br/>region columns — skips ALL<br/>other columns entirely<br/>(user, date, etc.) —<br/>dramatically less I/O"]
    I --> K["Additionally: values within<br/>a single column are highly<br/>similar/repetitive, enabling<br/>MUCH better compression<br/>than row-oriented storage"]
```

**Why this is the single most important architectural choice for OLAP:** Analytical queries characteristically touch a small fraction of columns but a huge fraction of rows — the exact inverse of typical OLTP access patterns. Columnar storage is purpose-built for this, delivering both massive I/O reduction (skip irrelevant columns entirely) and much better compression (similar values stored together compress far better than mixed row data).

---

## 5. Data Ingestion Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant OLTP as Orders DB (OLTP)
    participant CDC as CDC Connector
    participant K as Kafka
    participant ETL as Stream Processor
    participant Col as Columnar Store

    Note over OLTP: New order inserted<br/>(normal transactional write)

    CDC->>OLTP: Tail transaction log<br/>(same mechanism as the<br/>CDC Pipeline design)
    CDC->>K: Publish change event<br/>{order_id, user_id, amount,<br/>region, timestamp}

    K->>ETL: Consume change event
    ETL->>ETL: Denormalize/enrich<br/>(e.g., join in product category<br/>from a reference table,<br/>flatten nested structures)

    ETL->>Col: Write to columnar store<br/>(batched, columnar-formatted)
    Col->>Col: Update relevant<br/>materialized views incrementally
```

---

## 6. Query Execution — Massively Parallel Processing (MPP)

```mermaid
flowchart TB
    A["Analyst query:<br/>'Revenue by region and<br/>category, last quarter'"] --> B["Query Coordinator<br/>parses and plans query"]
    B --> C["Query plan distributed<br/>to MANY worker nodes<br/>in parallel"]

    C --> D["Worker 1: scans its shard<br/>of the columnar data,<br/>computes partial aggregates"]
    C --> E["Worker 2: scans its shard,<br/>computes partial aggregates"]
    C --> F["Worker 3: scans its shard,<br/>computes partial aggregates"]

    D & E & F --> G["Coordinator merges<br/>partial results into<br/>final aggregated answer"]
    G --> H["Return to analyst"]
```

```mermaid
sequenceDiagram
    participant Analyst as Analyst (BI Tool)
    participant Coord as Query Coordinator
    participant W1 as Worker 1 (shard 1)
    participant W2 as Worker 2 (shard 2)
    participant W3 as Worker 3 (shard 3)

    Analyst->>Coord: SQL query
    Coord->>Coord: Parse, optimize,<br/>create distributed execution plan

    par Parallel execution across all workers
        Coord->>W1: Execute partial aggregation<br/>on shard 1
        W1-->>Coord: Partial result
    and
        Coord->>W2: Execute partial aggregation<br/>on shard 2
        W2-->>Coord: Partial result
    and
        Coord->>W3: Execute partial aggregation<br/>on shard 3
        W3-->>Coord: Partial result
    end

    Coord->>Coord: Merge partial results<br/>(e.g., sum the partial sums)
    Coord-->>Analyst: Final aggregated result
```

**Why MPP is essential at this scale:** A query scanning billions of rows would take far too long on a single machine, regardless of how well-optimized the storage format is. Distributing the scan and partial aggregation across many worker nodes in parallel — then cheaply merging the much-smaller partial results — is what makes multi-second response times achievable even at petabyte scale.

---

## 7. Materialized Views (Pre-Aggregation for Common Queries)

```mermaid
flowchart TB
    A["Common, frequently-run query:<br/>'daily revenue by region'"] --> B{"Compute this from<br/>raw data every time?"}
    B -- "Naive: yes" --> C["Every single query scans<br/>and aggregates raw event-level<br/>data from scratch —<br/>wasteful, repeated work"]

    B -- "Optimized: no" --> D["Maintain a MATERIALIZED VIEW —<br/>a pre-aggregated table,<br/>incrementally updated as<br/>new data arrives"]
    D --> E["Query for 'daily revenue<br/>by region' now just reads<br/>the SMALL pre-aggregated<br/>table directly — near-instant,<br/>regardless of how much raw<br/>data underlies it"]

    F["Tradeoff: materialized views<br/>only accelerate KNOWN, common<br/>query patterns — ad-hoc/novel<br/>analyst queries still need to<br/>hit the full columnar store"] -.-> D
```

*This is conceptually identical to the downsampling/rollup pattern used in the Analytics/Metrics Dashboard and Time-Series Database designs — precomputing common aggregations trades storage and update complexity for dramatically faster reads on predictable query patterns.*

---

## 8. Incremental Materialized View Update — Detailed Sequence

```mermaid
sequenceDiagram
    participant ETL as Stream Processor
    participant Col as Columnar Store (raw)
    participant MV as Materialized View<br/>(daily_revenue_by_region)

    ETL->>Col: New order event arrives:<br/>{region:'US', amount:50,<br/>date:'2026-08-09'}

    ETL->>MV: Incrementally update:<br/>daily_revenue_by_region['US']['2026-08-09']<br/>+= 50

    Note over MV: The materialized view stays<br/>continuously up-to-date via<br/>incremental updates — NOT<br/>via periodically recomputing<br/>the entire aggregation from<br/>scratch, which would defeat<br/>the real-time freshness goal
```

---

## 9. Query Resource Isolation (Multi-Analyst Concurrency)

```mermaid
flowchart TB
    A["Many analysts running<br/>queries simultaneously"] --> B{"Resource Management Strategy"}

    B --> C["Query queuing/prioritization<br/>(e.g., quick dashboard<br/>queries prioritized over<br/>long-running exploratory<br/>ad-hoc analysis)"]
    B --> D["Resource pools/quotas<br/>per team or query type<br/>(prevents one analyst's<br/>massive query from<br/>starving others)"]
    B --> E["Query result caching<br/>(identical or overlapping<br/>queries reuse cached results<br/>rather than rescanning)"]

    F["This is the same 'noisy<br/>neighbor' problem seen in<br/>the Multi-Tenant SaaS Database<br/>design, applied to analytical<br/>workloads instead of<br/>transactional ones"] -.-> B
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((OLAP System HLD))
    CDC Connectors
      Stream changes from OLTP sources
      Near-real-time capture
    Stream Processor
      Denormalization/enrichment
      Incremental materialized view updates
    Columnar Store
      Compressed, column-oriented
      Sharded across workers
    Materialized Views
      Pre-aggregated common queries
      Incrementally maintained
    Query Coordinator
      Distributed query planning
      Partial result merging
    Worker Nodes
      Parallel shard scanning
      MPP execution
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Storage layout | Columnar, not row-oriented | Matches the OLAP access pattern (few columns, many rows) far better than row storage, enabling both I/O reduction and superior compression |
| Data freshness mechanism | CDC + streaming ingestion, not nightly batch | Achieves near-real-time freshness (minutes, not hours/overnight), distinguishing this from traditional data warehousing |
| Query execution model | Massively Parallel Processing (MPP) | Distributes scan/aggregation work across many nodes, making multi-second responses achievable even at petabyte scale |
| Pre-aggregation | Materialized views for common patterns | Trades storage/update complexity for dramatically faster reads on predictable, frequently-run queries |
| Separation from OLTP | Entirely separate system, fed via CDC | Prevents analytical query load from degrading transactional system performance, and allows independent optimization of each for its actual workload |
| Concurrency management | Query queuing/resource pools/caching | Prevents any single analyst's heavy query from starving others sharing the same infrastructure |

---

## 12. Bottlenecks & Scaling Considerations

- **Ingestion lag under high source volume** — if CDC/streaming ingestion can't keep pace with operational write volume, the "real-time" freshness promise degrades; needs monitoring and independently scalable ingestion capacity, decoupled from query-serving capacity.
- **Materialized view explosion** — maintaining too many materialized views for every conceivable query pattern becomes an operational burden (each needs incremental update logic and storage); requires disciplined prioritization based on actual query frequency, not speculative coverage.
- **Ad-hoc query unpredictability** — unlike materialized-view-backed dashboards, genuinely novel analyst queries can scan enormous data volumes unpredictably; needs query cost estimation and potentially query complexity limits to prevent a single runaway query from consuming disproportionate cluster resources.
- **Shard skew for columnar data** — if data isn't evenly distributed across worker shards (e.g., heavily skewed by date or region), MPP query performance suffers since overall query latency is bounded by the slowest worker — careful partition key selection for the columnar store matters as much as shard key selection does for OLTP sharding.
- **Compression vs update flexibility tradeoff** — highly compressed columnar formats are excellent for read-heavy analytical workloads but are typically less efficient for frequent small updates; this is part of why data usually flows in as append-only events/batches rather than supporting arbitrary in-place updates like an OLTP system would.
- **Schema evolution** — as operational systems evolve their schemas, the OLAP ingestion pipeline (CDC, stream transformation, columnar store schema) must handle these changes gracefully without breaking existing materialized views or historical query compatibility.
- **Cost management at petabyte scale** — storage and compute costs for both the raw columnar store and its materialized views grow substantial at this scale; tiered storage (hot/warm/cold, similar to the pattern in Log Aggregation and Analytics Dashboard designs) and careful retention policies become important cost-control levers, not just performance optimizations.
