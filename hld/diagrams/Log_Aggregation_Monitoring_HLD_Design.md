# Design a Log Aggregation and Monitoring System (ELK/Datadog-style) — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Collect logs from thousands of distributed application/infrastructure instances
- Support full-text search across logs
- Structured log parsing (extract fields from semi-structured log lines)
- Real-time tailing/streaming for live debugging
- Dashboards, alerting on log patterns (e.g., error rate spikes)
- Long-term retention with cost-efficient archival

### Non-Functional Requirements
- **High write throughput:** Massive log volume from thousands of hosts continuously
- **Search latency:** Recent logs should be searchable within seconds of being written
- **Durability:** Logs must not be silently dropped, especially error/critical logs
- **Cost efficiency:** Storage costs must be managed via tiering/compression — raw log volume is enormous
- **Resilience to backpressure:** A logging system slowdown must never crash or block the applications producing logs

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Log lines/sec (platform-wide) | ~1M-5M |
| Avg log line size | ~500 bytes |
| Daily raw log volume | ~50-200 TB/day |
| Search query latency target | < 1-2s for recent logs |
| Retention: hot (searchable) | 7-14 days |
| Retention: cold (archived) | 1+ year |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    subgraph Sources["Log Sources"]
        App1["App Server 1"]
        App2["App Server 2"]
        Infra["Infrastructure<br/>(load balancers, DBs)"]
    end

    subgraph Collection["Collection Layer"]
        Agent1["Log Shipper Agent<br/>(Filebeat/Fluentd — local)"]
        Agent2["Log Shipper Agent"]
    end

    subgraph Buffering["Buffering Layer"]
        Kafka["Kafka<br/>(absorbs volume spikes,<br/>decouples producers from indexers)"]
    end

    subgraph Processing["Processing Layer"]
        Parser["Log Parser/Enricher<br/>(structured field extraction,<br/>Grok patterns)"]
        Indexer["Indexer Workers"]
    end

    subgraph Storage["Storage Layer"]
        HotIndex[("Hot Search Index<br/>(Elasticsearch — recent, fast)")]
        ColdArchive[("Cold Archive<br/>(S3/object storage, compressed)")]
    end

    subgraph QueryPath["Query & Alerting"]
        SearchAPI["Search API"]
        Dashboard["Dashboard UI"]
        AlertEngine["Alert Engine"]
    end

    App1 --> Agent1
    App2 --> Agent2
    Infra --> Agent1

    Agent1 --> Kafka
    Agent2 --> Kafka

    Kafka --> Parser --> Indexer
    Indexer --> HotIndex
    HotIndex -->|"Age out after retention window"| ColdArchive

    HotIndex --> SearchAPI --> Dashboard
    Kafka --> AlertEngine
```

**Key idea:** Kafka sits between log collection and indexing as a **shock absorber** — if the indexing/search layer slows down or a burst of logs arrives (e.g., during an incident, when log volume often spikes dramatically), Kafka buffers the backlog without ever blocking the application generating the logs. This decoupling is the single most important architectural decision in a logging pipeline.

---

## 3. Data Model

```mermaid
erDiagram
    LOG_ENTRY {
        string log_id PK
        string host
        string service
        string level "INFO/WARN/ERROR"
        timestamp ts
        string raw_message
        map extracted_fields "parsed structured data"
        string trace_id "for distributed tracing correlation"
    }
    INDEX_SHARD {
        string index_name PK "e.g. logs-2026.08.09"
        int shard_id
        string node_id
    }
```

*Indices are typically time-partitioned (e.g., one index per day: `logs-2026.08.09`) rather than one giant index — this makes retention/deletion trivial (just drop old daily indices) and keeps individual shard sizes manageable.*

---

## 4. Log Collection Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant App as Application
    participant Agent as Log Shipper Agent<br/>(local to the host)
    participant K as Kafka
    participant Parser as Parser/Enricher
    participant Idx as Indexer
    participant ES as Elasticsearch

    App->>Agent: Write log line to local file/stdout
    Agent->>Agent: Tail file, batch lines<br/>(local buffering, at-least-once)
    Agent->>K: Ship batch to Kafka topic

    Note over Agent: If Kafka is unreachable,<br/>agent buffers locally to disk<br/>— never blocks or crashes<br/>the application

    K->>Parser: Consume batch
    Parser->>Parser: Apply Grok/regex patterns<br/>to extract structured fields<br/>(timestamp, level, service, custom fields)
    Parser->>Idx: Enriched log document

    Idx->>ES: Bulk index into<br/>time-partitioned index
    ES-->>Idx: Ack
```

---

## 5. Log Parsing & Field Extraction

```mermaid
flowchart TB
    A["Raw log line:<br/>'2026-08-09 10:23:01 ERROR<br/>[order-service] Failed to process<br/>order_id=12345 user_id=678 latency_ms=450'"] --> B["Grok Pattern Matching<br/>(regex-based field extraction)"]
    B --> C["Extracted fields:"]
    C --> D["timestamp: 2026-08-09 10:23:01"]
    C --> E["level: ERROR"]
    C --> F["service: order-service"]
    C --> G["order_id: 12345"]
    C --> H["user_id: 678"]
    C --> I["latency_ms: 450"]

    D & E & F & G & H & I --> J["Structured document<br/>indexed into Elasticsearch —<br/>now queryable as:<br/>'latency_ms > 400 AND level=ERROR'<br/>not just raw text search"]
```

**Why structured extraction matters:** Raw full-text search ("find lines containing 'error'") is far less useful than structured queries ("show me all ERROR logs from order-service with latency_ms > 400 in the last hour"). Parsing at ingestion time is what transforms unstructured log noise into an actually queryable dataset.

---

## 6. Search Query Flow

```mermaid
sequenceDiagram
    participant C as Client (Dashboard/CLI)
    participant API as Search API
    participant ES as Elasticsearch Cluster

    C->>API: Query: service=order-service AND<br/>level=ERROR, range=last_1h
    API->>API: Determine which time-partitioned<br/>indices overlap the query range<br/>(e.g., logs-2026.08.09)

    API->>ES: Search across relevant<br/>indices/shards (parallel)
    ES->>ES: Each shard searches its<br/>local inverted index
    ES-->>API: Merged, ranked results

    API-->>C: Return matching log entries
```

---

## 7. Backpressure Handling (Critical Path)

```mermaid
flowchart TB
    A["Log volume spike<br/>(e.g., during an incident,<br/>10x normal volume)"] --> B{"Where does the<br/>backlog accumulate?"}

    B --> C["Kafka absorbs the spike<br/>(durable, disk-backed buffer)"]
    C --> D["Indexers process at their<br/>sustainable rate,<br/>draining the backlog gradually"]
    D --> E["Search sees slightly<br/>increased indexing lag<br/>(logs take longer to become searchable)<br/>— but NOTHING IS LOST"]

    F["Without Kafka buffering"] --> G["Direct agent-to-Elasticsearch<br/>would overwhelm ES during spikes,<br/>causing indexing failures or<br/>agent-side blocking —<br/>risk of losing exactly the logs<br/>you need most during an incident"]
```

*This is precisely why logs shouldn't be shipped directly to the search index — an incident (the exact moment you need logs most) is also exactly when log volume spikes hardest, and a direct-write architecture would be most likely to fail at the worst possible time.*

---

## 8. Storage Tiering & Retention

```mermaid
flowchart TB
    A["Log ingested"] --> B["Hot tier: Elasticsearch<br/>(fully indexed, fast search)<br/>Retention: 7-14 days"]
    B --> C{"Index age exceeds<br/>hot retention window"}
    C -- Yes --> D["Snapshot index to<br/>cold archive (S3, compressed)"]
    D --> E["Delete from hot Elasticsearch<br/>(frees expensive indexed storage)"]
    E --> F["Cold archive retained<br/>1+ years, compressed,<br/>NOT actively indexed"]

    G["Need to search old logs<br/>(rare, e.g., compliance/audit)"] --> H["Restore relevant archive<br/>segment temporarily,<br/>re-index on demand"]
```

*Full-text indexing is expensive to maintain indefinitely — the vast majority of log queries target the last few hours/days. Cold storage keeps older data available for rare compliance/audit needs at a fraction of the cost, accepting slower access in exchange.*

---

## 9. Alerting on Log Patterns

```mermaid
sequenceDiagram
    participant K as Kafka (log stream)
    participant Alert as Alert Engine
    participant Rules as Alert Rules
    participant Notif as Notification Channel

    K->>Alert: Consume log stream (parallel to indexing)
    Alert->>Rules: Evaluate against active rules<br/>e.g., "ERROR rate > 100/min<br/>for service=payment"

    Alert->>Alert: Maintain sliding window<br/>count of matching logs

    alt Threshold breached
        Alert->>Notif: Fire alert with<br/>sample matching log lines
    else Below threshold
        Alert->>Alert: Continue monitoring
    end
```

*The alert engine consumes directly from the Kafka log stream in parallel with indexing — this means alerting doesn't wait for the (potentially lagging) search index to catch up, giving faster incident detection than "query Elasticsearch every minute" would.*

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Log Aggregation HLD))
    Log Shipper Agent
      Local tailing/buffering
      At-least-once delivery
      Never blocks application
    Kafka Buffer
      Absorbs volume spikes
      Decouples producers from indexers
    Parser/Enricher
      Grok pattern field extraction
      Structured document creation
    Indexer Workers
      Bulk write to Elasticsearch
      Time-partitioned index routing
    Elasticsearch Cluster
      Hot, searchable storage
      Sharded inverted index
    Cold Archive
      Long-term compressed storage
      On-demand restore
    Alert Engine
      Parallel stream consumption
      Sliding window rule evaluation
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Buffering layer | Kafka between agents and indexers | Absorbs volume spikes without blocking applications or losing logs during exactly the moments they matter most (incidents) |
| Log structure | Parse into structured fields at ingestion | Transforms raw text into queryable structured data, enabling precise filtering beyond simple keyword search |
| Index partitioning | Time-based (daily indices) | Makes retention/deletion trivial and keeps shard sizes manageable |
| Storage tiering | Hot (indexed) + Cold (compressed archive) | Full indexing is expensive to maintain indefinitely; most queries only need recent data |
| Alerting architecture | Parallel stream consumption, not query-based polling | Faster detection — doesn't wait for the (potentially lagging) search index |
| Agent-side failure handling | Local disk buffering on shipper agents | Application logging must never block or crash on downstream unavailability |

---

## 12. Bottlenecks & Scaling Considerations

- **Elasticsearch indexing throughput** — the indexing layer is often the bottleneck under sustained high volume; horizontal scaling (more shards, more indexer workers) and bulk-write batching are essential.
- **Hot shard/index sizing** — too few shards per daily index limits parallelism; too many adds per-shard overhead — needs tuning based on actual daily volume per service/environment.
- **Cardinality in structured fields** — similar to metrics systems, indexing high-cardinality fields (e.g., raw user_id as a keyword field) can bloat index size and slow queries; requires thoughtful field mapping decisions.
- **Cost of full retention at full fidelity** — most organizations can't afford to keep everything hot forever; tiering strategy (and sometimes sampling of high-volume, low-value logs) is essential for cost control.
- **Multi-tenancy/noisy neighbor** — one service suddenly logging at 100x its normal rate (e.g., a misbehaving retry loop) can degrade indexing performance for all other services sharing the pipeline; per-service rate limiting or quotas protect against this.
- **Search query complexity at scale** — expensive queries (e.g., broad wildcard searches across weeks of data) can overwhelm the cluster; query complexity limits and query result caching help contain this.
- **Log format inconsistency** — different services/teams often log in inconsistent formats, making universal Grok patterns fragile; encouraging structured logging (e.g., JSON) at the application source dramatically simplifies the parsing layer.
