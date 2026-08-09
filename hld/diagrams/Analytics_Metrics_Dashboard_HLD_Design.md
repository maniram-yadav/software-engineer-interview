# Design an Analytics/Metrics Dashboard System — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Ingest high-volume event/metric data from many sources (app servers, clients, IoT devices)
- Support flexible aggregations (sum, count, avg, percentiles) over arbitrary time windows
- Real-time dashboards (near-live metrics, e.g., "requests in the last 5 minutes")
- Historical trend analysis (metrics over days/months/years)
- Alerting based on threshold breaches

### Non-Functional Requirements
- **High write throughput:** Millions of data points/sec ingested
- **Query latency:** Dashboard queries should render in < 1-2 seconds, even over large time ranges
- **Storage efficiency:** Raw granular data doesn't need to be retained forever — older data can be downsampled
- **Availability over strict consistency:** A slightly delayed metric is fine; a dashboard outage during an incident is not
- **Cardinality handling:** Must gracefully handle metrics with many unique label combinations (e.g., per-user, per-endpoint)

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Data points ingested/sec | ~5M |
| Unique metric series (cardinality) | ~10M+ |
| Retention: raw resolution | 24-48 hours |
| Retention: downsampled (1-min) | 30 days |
| Retention: downsampled (1-hour) | 2 years |
| Dashboard queries/sec | ~10,000 |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    subgraph Sources["Data Sources"]
        AppServers["Application Servers<br/>(metrics/logs)"]
        Clients["Client SDKs<br/>(mobile/web events)"]
        Infra["Infrastructure Metrics<br/>(CPU, memory, network)"]
    end

    subgraph Ingestion["Ingestion Layer"]
        Collector["Metrics Collector<br/>(lightweight agents)"]
        Kafka["Kafka<br/>(buffering, backpressure)"]
    end

    subgraph Processing["Stream Processing Layer"]
        StreamProc["Stream Processor<br/>(Flink/Kafka Streams —<br/>real-time aggregation)"]
        Downsampler["Downsampling Workers<br/>(batch, periodic rollups)"]
    end

    subgraph Storage["Storage Layer"]
        TSDB[("Time-Series Database<br/>(sharded, compressed)")]
        HotStore[("Hot Store — raw resolution<br/>(in-memory/SSD, short retention)")]
        ColdStore[("Cold Store — downsampled<br/>(long retention, cheaper storage)")]
    end

    subgraph QueryPath["Query Layer"]
        QueryAPI["Query API<br/>(PromQL-style query language)"]
        DashboardSvc["Dashboard Service"]
        AlertSvc["Alerting Service"]
    end

    AppServers --> Collector
    Clients --> Collector
    Infra --> Collector

    Collector --> Kafka --> StreamProc
    StreamProc --> HotStore
    StreamProc --> AlertSvc

    HotStore --> Downsampler --> ColdStore
    HotStore --> TSDB
    ColdStore --> TSDB

    TSDB --> QueryAPI --> DashboardSvc
```

**Key idea:** Metrics data has a **temporal value gradient** — recent, high-resolution data is queried constantly (live dashboards) but only needs short retention, while old data is queried rarely but must be retained cheaply for long-term trends. The architecture explicitly tiers storage around this: hot (raw, recent), and cold (downsampled, long-retained) — rather than storing everything at full resolution forever.

---

## 3. Time-Series Data Model

```mermaid
erDiagram
    METRIC_SERIES {
        string series_id PK "hash of metric_name + labels"
        string metric_name
        map labels "e.g. {host: web-1, endpoint: /api/orders}"
    }
    DATA_POINT {
        string series_id FK
        timestamp ts
        double value
    }
```

```mermaid
flowchart LR
    A["Raw metric emission:<br/>http_requests_total{host='web-1', endpoint='/api/orders', status='200'} = 1"] --> B["series_id = hash(metric_name + sorted labels)"]
    B --> C["Time-series: append-only list<br/>of (timestamp, value) pairs<br/>for this exact series_id"]
```

**Key modeling concept — cardinality:** Each unique combination of metric name + label values creates a distinct time series. `http_requests_total` with labels for `host`, `endpoint`, and `status` across 1,000 hosts × 50 endpoints × 5 status codes = 250,000 distinct series just for one metric — this is why cardinality control is a first-class design concern, not an afterthought.

---

## 4. Ingestion & Real-Time Aggregation Flow

```mermaid
sequenceDiagram
    participant App as Application Server
    participant Collector as Metrics Collector Agent
    participant K as Kafka
    participant Stream as Stream Processor
    participant Hot as Hot Store

    loop Continuous
        App->>Collector: Emit metric<br/>(counter increment, gauge, histogram)
        Collector->>Collector: Local buffering + batching<br/>(reduce network overhead)
        Collector->>K: Flush batch every few seconds
    end

    K->>Stream: Consume metric batch
    Stream->>Stream: Windowed aggregation<br/>(e.g., sum requests per 10s window,<br/>per series_id)
    Stream->>Hot: Write aggregated data point

    Note over Stream: Late-arriving data handled via<br/>watermarks — small grace period<br/>before window is finalized
```

---

## 5. Downsampling Pipeline (Storage Tiering)

```mermaid
flowchart TB
    A["Raw data: 1-second resolution<br/>Retention: 48 hours"] --> B["Downsampling Worker<br/>(runs periodically)"]
    B --> C["1-minute rollups<br/>(avg/min/max/sum per minute)<br/>Retention: 30 days"]
    C --> D["Downsampling Worker"]
    D --> E["1-hour rollups<br/>Retention: 2 years"]

    F["Query for 'last 6 hours'"] -.->|"reads from"| A
    G["Query for 'last 2 weeks'"] -.->|"reads from"| C
    H["Query for 'last year'"] -.->|"reads from"| E
```

**Why this matters:** A dashboard showing a year-long trend doesn't need per-second granularity — the human eye can't distinguish it, and rendering millions of raw points would be both slow and visually meaningless. Downsampling trades granularity for storage efficiency and query speed exactly where it doesn't cost anything perceptually.

---

## 6. Query Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client (Dashboard)
    participant QAPI as Query API
    participant Router as Resolution Router
    participant Hot as Hot Store (raw)
    participant Cold as Cold Store (downsampled)
    participant TSDB as Time-Series DB

    C->>QAPI: Query: avg(cpu_usage) by host,<br/>range=last_7_days
    QAPI->>Router: Determine appropriate<br/>resolution tier for range

    alt Range fits in raw retention window
        Router->>Hot: Query raw data
        Hot-->>Router: Data points
    else Range requires downsampled data
        Router->>Cold: Query 1-hour rollups
        Cold-->>Router: Aggregated data points
    end

    Router->>TSDB: Merge results if range<br/>spans multiple tiers
    TSDB-->>QAPI: Combined time series
    QAPI->>QAPI: Apply final aggregation<br/>(group by host, compute avg)
    QAPI-->>C: Return chart-ready data
```

---

## 7. Handling High Cardinality

```mermaid
flowchart TB
    A["Metric with unbounded label values<br/>(e.g., labeling by raw user_id —<br/>millions of unique series)"] --> B{"Cardinality Control Strategy"}

    B --> C["Reject/warn at ingestion time<br/>if series count for a metric<br/>exceeds a configured limit"]
    B --> D["Encourage bucketing<br/>(e.g., user_tier instead of<br/>raw user_id) at the SDK level"]
    B --> E["Separate high-cardinality data<br/>into an events/logging system,<br/>NOT the metrics TSDB"]

    F["Why this matters:<br/>Each new label combination =<br/>a new time series to index and store.<br/>Uncontrolled cardinality is the<br/>#1 cause of metrics system outages."]
```

---

## 8. Alerting Pipeline

```mermaid
sequenceDiagram
    participant Stream as Stream Processor
    participant Alert as Alerting Service
    participant Rules as Alert Rule Store
    participant Notif as Notification Channel<br/>(PagerDuty/Slack)

    Stream->>Alert: New aggregated data point<br/>(error_rate = 8%)
    Alert->>Rules: Check active rules for this metric
    Rules-->>Alert: Rule: error_rate > 5% for 3 consecutive<br/>evaluations → trigger

    Alert->>Alert: Track evaluation history<br/>(sliding window of recent breaches)
    alt Threshold breached for required duration
        Alert->>Notif: Fire alert
        Alert->>Alert: Enter COOLDOWN state<br/>(avoid repeat alert spam)
    else Not yet sustained
        Alert->>Alert: Continue monitoring
    end
```

*Alerting requires "sustained breach" logic (not firing on a single noisy spike) — a naive threshold check on every data point would generate constant false-positive pages from normal metric jitter.*

---

## 9. Query Language / Aggregation Model (PromQL-style)

```mermaid
flowchart LR
    A["sum(rate(http_requests_total[5m]))<br/>by (endpoint)"] --> B["Step 1: rate() —<br/>compute per-second rate<br/>from counter over 5-min window"]
    B --> C["Step 2: sum() by (endpoint) —<br/>aggregate across all hosts,<br/>grouped per endpoint"]
    C --> D["Result: time series showing<br/>request rate per endpoint,<br/>ready to plot"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Analytics Dashboard HLD))
    Collector Agents
      Local buffering
      Batch flush to Kafka
    Stream Processor
      Real-time windowed aggregation
      Watermark-based late data handling
    Downsampling Workers
      Periodic rollup jobs
      Storage tier transitions
    Hot Store
      Raw resolution, short retention
      Backs live dashboards
    Cold Store
      Downsampled, long retention
      Backs historical trend queries
    Query API
      Resolution-aware routing
      PromQL-style aggregation
    Alerting Service
      Sustained-breach detection
      Cooldown/dedup logic
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Storage tiering | Hot (raw, short retention) + Cold (downsampled, long retention) | Matches storage cost/query patterns to actual value of data at different ages |
| Aggregation timing | Stream processing (real-time) + batch downsampling (periodic) | Real-time dashboards need live aggregation; long-term rollups don't need to be instant |
| Cardinality control | Enforced limits + architectural separation from logging systems | Uncontrolled label cardinality is the most common cause of metrics system failure at scale |
| Consistency model | Eventual, with brief ingestion delay | A metrics dashboard being a few seconds stale is acceptable; blocking on strict consistency would hurt availability for no real benefit |
| Alert evaluation | Sustained-breach windows, not single-point triggers | Prevents alert fatigue from normal metric noise/spikes |
| Query resolution routing | Automatic tier selection based on time range | Keeps queries fast by never pulling more granularity than the range/chart actually needs |

---

## 12. Bottlenecks & Scaling Considerations

- **Cardinality explosion** — the single biggest operational risk; needs proactive limits at ingestion (reject or aggregate away high-cardinality labels) rather than discovering the problem after storage/query performance degrades.
- **Hot store write throughput** — millions of data points/sec requires a write-optimized storage engine (LSM-tree based, like many purpose-built TSDBs) rather than a general-purpose relational database.
- **Late-arriving data** — mobile/IoT clients with intermittent connectivity may submit data well after its actual timestamp; stream processing watermarks must balance waiting for stragglers against not delaying window finalization indefinitely.
- **Downsampling job backlog** — if downsampling workers fall behind, hot-store retention windows can fill up before older data is safely rolled up and archived; needs monitoring and priority scaling of this pipeline stage specifically.
- **Query fan-out for wide aggregations** — a query aggregating "by host" across 10,000 hosts touches many underlying series; needs efficient parallel scan/merge within the TSDB rather than sequential series-by-series querying.
- **Dashboard query storms** — many users loading the same popular dashboard simultaneously (e.g., during an incident, when everyone checks the same graphs) benefits heavily from query result caching with short TTLs.
- **Alert flapping** — metrics oscillating right around a threshold can cause repeated alert fire/resolve cycles; hysteresis (different thresholds for firing vs resolving) helps stabilize this beyond just the sustained-window requirement.
