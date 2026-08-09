# Design a Time-Series Database (Prometheus/InfluxDB-style) — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Efficiently store and query data points indexed by timestamp
- Support high-cardinality labeled metrics (metric_name + key-value tags)
- Range queries over arbitrary time windows, with aggregation functions
- Automatic data retention/expiration policies
- Support both high-frequency writes (ingestion) and ad-hoc analytical queries

### Non-Functional Requirements
- **Write throughput:** Millions of data points/sec sustained ingestion
- **Storage efficiency:** Time-series data compresses extremely well if encoded correctly — must exploit this
- **Query latency:** Sub-second for typical dashboard queries over recent data
- **Compression:** Raw storage would be prohibitively expensive at this write volume without aggressive compression
- **Cardinality management:** Must handle (or protect against) high-cardinality label combinations gracefully

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Data points ingested/sec | ~5M |
| Bytes per raw data point (naive) | ~16 bytes (8-byte timestamp + 8-byte value) |
| Bytes per data point (compressed) | ~1-2 bytes (via delta + XOR encoding) |
| Daily raw volume (uncompressed) | ~7TB/day |
| Daily volume (compressed) | ~500GB-1TB/day |
| Unique time series (cardinality) | ~10M+ |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    subgraph Ingestion["Ingestion Layer"]
        WriteAPI["Write API<br/>(remote_write protocol)"]
        WAL["Write-Ahead Log<br/>(durability before<br/>in-memory buffering)"]
    end

    subgraph Storage["Storage Engine"]
        MemBuffer["In-Memory Buffer<br/>(recent, uncompressed chunks)"]
        Compactor["Compaction/Compression<br/>Worker"]
        ChunkStore[("Compressed Chunk Store<br/>(columnar, on-disk)")]
        Index[("Inverted Index<br/>(label → series_id mapping)")]
    end

    subgraph QueryPath["Query Layer"]
        QueryEngine["Query Engine<br/>(PromQL-style)"]
        QueryAPI["Query API"]
    end

    Client["Metric Sources<br/>(app instrumentation)"]
    Dashboard["Dashboards/Alerting"]

    Client --> WriteAPI --> WAL --> MemBuffer
    MemBuffer --> Compactor --> ChunkStore
    WriteAPI --> Index

    Dashboard --> QueryAPI --> QueryEngine
    QueryEngine --> Index
    QueryEngine --> MemBuffer
    QueryEngine --> ChunkStore
```

**Key idea:** Time-series databases exploit a structural property most general-purpose databases don't: **within a single series, consecutive values tend to be similar, and timestamps are evenly spaced and monotonically increasing.** The entire storage engine is built around specialized compression that takes advantage of exactly this pattern, achieving compression ratios (10-20x+) that general-purpose row-oriented storage can't match.

---

## 3. Data Model

```mermaid
erDiagram
    SERIES {
        string series_id PK "hash of metric_name + labels"
        string metric_name
        map labels
    }
    CHUNK {
        string series_id FK
        timestamp start_time
        timestamp end_time
        bytes compressed_data
    }
```

```mermaid
flowchart LR
    A["http_requests_total{method='GET', status='200', host='web-1'}"] --> B["series_id = hash(sorted label set)"]
    B --> C["Append-only stream of<br/>(timestamp, value) pairs<br/>for this exact series_id,<br/>grouped into time-bounded chunks"]
```

---

## 4. Compression — The Core Technical Innovation (Gorilla-style)

```mermaid
flowchart TB
    A["Raw data points:<br/>(t1, 100.0), (t2, 100.5),<br/>(t3, 100.3), (t4, 100.3)..."] --> B["Timestamp Compression"]
    A --> C["Value Compression"]

    B --> B1["Store FIRST timestamp fully"]
    B --> B2["Subsequent timestamps:<br/>store DELTA-OF-DELTA<br/>(since intervals between<br/>points are usually constant,<br/>e.g., every 15 seconds,<br/>this delta-of-delta is<br/>usually ZERO — encodes<br/>to just 1 bit!)"]

    C --> C1["Store FIRST value fully<br/>(as raw bits)"]
    C --> C2["Subsequent values: XOR<br/>with previous value"]
    C2 --> C3["If XOR result is mostly<br/>zero bits (value barely<br/>changed, common for metrics<br/>like CPU%), store only the<br/>small number of<br/>meaningfully-different bits"]

    B2 & C3 --> D["Combined: typical metric<br/>data point compresses from<br/>16 bytes down to<br/>~1.3-2 bytes on average"]
```

**Why this works so well for metrics specifically:** Most real-world metrics (CPU usage, request counts, latencies) change gradually and are sampled at regular intervals — exactly the pattern this delta-of-delta timestamp encoding and XOR-based value encoding is designed to exploit. This is fundamentally different from general-purpose compression (like gzip) because it's tailored to the specific statistical properties of time-series data.

---

## 5. Write Path — Detailed Sequence

```mermaid
sequenceDiagram
    participant App as Application<br/>(metric source)
    participant API as Write API
    participant WAL as Write-Ahead Log
    participant Mem as In-Memory Buffer
    participant Idx as Series Index

    App->>API: Write data point<br/>{metric, labels, timestamp, value}

    API->>Idx: Lookup/create series_id<br/>for this label combination
    Idx-->>API: series_id (new or existing)

    API->>WAL: Append to WAL<br/>(durability BEFORE ack)
    WAL-->>API: Fsynced

    API->>Mem: Add to in-memory chunk<br/>for this series_id
    API-->>App: Ack

    Note over Mem: Data remains in fast,<br/>mutable in-memory form<br/>until compaction —<br/>recent data queries hit<br/>this buffer directly
```

---

## 6. Compaction Flow (Memory → Compressed Disk Storage)

```mermaid
sequenceDiagram
    participant Timer as Compaction Trigger<br/>(time-based or size-based)
    participant Mem as In-Memory Buffer
    participant Compactor as Compactor Worker
    participant Chunk as Chunk Store (disk)
    participant WAL as Write-Ahead Log

    loop Every compaction interval (e.g., 2 hours)
        Timer->>Compactor: Trigger compaction
        Compactor->>Mem: Read accumulated data points<br/>for the period, grouped by series
        Mem-->>Compactor: Raw in-memory data

        Compactor->>Compactor: Apply delta/XOR compression<br/>per series, write to<br/>columnar chunk format

        Compactor->>Chunk: Write compressed chunk<br/>(immutable once written)
        Compactor->>Mem: Clear compacted data<br/>from memory
        Compactor->>WAL: Truncate WAL entries<br/>now safely persisted in chunk
    end
```

**Why the WAL matters:** If the process crashes before compaction runs, in-memory (uncompressed) data would be lost — the WAL provides durability for that window by recording every write to disk immediately (in a simple, fast append-only format) before it's later reorganized into the efficient compressed chunk format. On restart, the WAL is replayed to reconstruct the in-memory state.

---

## 7. Query Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client (Dashboard)
    participant QAPI as Query API
    participant Idx as Series Index
    participant Mem as In-Memory Buffer
    participant Chunk as Chunk Store

    C->>QAPI: Query: avg(cpu_usage) by host,<br/>range=[now-6h, now]
    QAPI->>Idx: Resolve label matchers<br/>to matching series_ids

    Idx-->>QAPI: List of matching series_ids

    par Fetch from both storage tiers
        QAPI->>Mem: Get recent data<br/>(last few hours, still in-memory)
        Mem-->>QAPI: Uncompressed recent points
    and
        QAPI->>Chunk: Get older data from<br/>compressed chunks
        Chunk->>Chunk: Decompress relevant chunks
        Chunk-->>QAPI: Decompressed historical points
    end

    QAPI->>QAPI: Merge both sources,<br/>apply aggregation (avg by host)
    QAPI-->>C: Return time-series result
    set
```

---

## 8. Cardinality Explosion Protection

```mermaid
flowchart TB
    A["Incoming write with labels:<br/>{metric:'http_requests',<br/>user_id:'12345', ...}"] --> B{"Would this label combination<br/>create a NEW series?"}
    B -- Yes --> C["Check: has this metric_name<br/>already exceeded its configured<br/>max series count?"]
    C -- "Under limit" --> D["Allow — create new series"]
    C -- "Over limit" --> E["REJECT this data point<br/>(or drop the offending label)<br/>— log a cardinality<br/>violation warning"]

    F["Why this matters critically:<br/>a single poorly-designed metric<br/>(e.g., labeling by raw user_id<br/>instead of a bounded category)<br/>can create MILLIONS of new<br/>series, each requiring index<br/>entries and storage overhead —<br/>this is the most common<br/>cause of TSDB outages<br/>in production"]
```

---

## 9. Index Structure (Label → Series Lookup)

```mermaid
flowchart TB
    A["Inverted Index"] --> B["Label: metric_name='cpu_usage'<br/>→ [series_1, series_5, series_9, ...]"]
    A --> C["Label: host='web-1'<br/>→ [series_1, series_2, series_7, ...]"]
    A --> D["Label: env='production'<br/>→ [series_1, series_3, series_5, ...]"]

    E["Query: cpu_usage{host='web-1',<br/>env='production'}"] --> F["Intersect the postings lists<br/>for all three label matchers"]
    F --> G["Result: series_1<br/>(only series matching ALL<br/>three label conditions)"]
```

*This is conceptually the same inverted-index intersection technique used in the Search Engine design — label-based series lookup in a TSDB is essentially a specialized full-text search problem over structured tag data.*

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Time-Series DB HLD))
    Write API
      Ingestion endpoint
      Series ID resolution
    Write-Ahead Log
      Durability before compaction
      Crash recovery source
    In-Memory Buffer
      Recent, mutable data
      Fast for live dashboards
    Compactor
      Delta + XOR compression
      Memory to disk transition
    Chunk Store
      Immutable compressed storage
      Columnar, time-partitioned
    Series Index
      Label to series_id mapping
      Inverted index intersection
    Cardinality Guard
      Per-metric series limits
      Prevents index/storage blowup
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Compression strategy | Delta-of-delta timestamps + XOR value encoding | Exploits the specific statistical regularity of time-series data (regular intervals, gradual value changes) far better than general-purpose compression |
| Write path | WAL + in-memory buffer, compacted periodically | Balances write durability (WAL) against write speed (memory) and long-term storage efficiency (compacted chunks) |
| Storage layout | Columnar, time-partitioned chunks | Query patterns are almost always time-range-bounded; partitioning by time makes range queries efficient and retention/deletion trivial |
| Cardinality control | Hard limits enforced at ingestion | The most common real-world cause of TSDB failure is uncontrolled label cardinality; proactive limits prevent this rather than reacting after degradation |
| Index structure | Inverted index over labels | Enables efficient multi-label query resolution via postings-list intersection, same principle as full-text search |
| Query merge | Combine in-memory (recent) + compressed (historical) sources | Recent data needs to be queryable before compaction completes; querying both tiers transparently gives a complete, current view |

---

## 12. Bottlenecks & Scaling Considerations

- **Cardinality is the dominant scaling risk** — far more than raw data point volume, the number of DISTINCT series is what stresses index size and memory; this is why cardinality guards are a first-class architectural concern, not an afterthought.
- **Compaction lag under sustained high write load** — if compaction can't keep pace with ingestion, the in-memory buffer grows unboundedly, risking memory exhaustion; needs monitoring and the ability to scale compaction workers independently of ingestion capacity.
- **Query cost for very wide aggregations** — a query aggregating across millions of series (e.g., "sum across ALL hosts") requires touching and decompressing many chunks; needs efficient parallel chunk processing and possibly pre-aggregated rollups for extremely common wide queries (similar to the downsampling approach in the general Analytics Dashboard design).
- **Out-of-order/late-arriving writes** — the compression scheme assumes roughly monotonic, evenly-spaced timestamps; handling significantly out-of-order writes (common with mobile/edge metric sources) requires either buffering longer before compaction or accepting reduced compression efficiency for affected series.
- **Long-term retention storage costs** — even with excellent compression, retaining raw-resolution data indefinitely at high ingestion volume is expensive; downsampling older data to coarser resolution (same pattern as general metrics systems) is typically layered on top of this core storage engine.
- **Sharding for horizontal scale** — a single node's ingestion/query capacity is inherently bounded; production TSDBs at this scale shard series across many storage nodes (often by hash of series_id), requiring a distributed query layer to fan out and merge results across shards.
