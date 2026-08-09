# Design YouTube/Netflix — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Users can upload videos
- Videos are processed into multiple resolutions/bitrates for adaptive streaming
- Users can stream/watch videos with minimal buffering
- Support seeking, resuming playback
- Recommendations, search, comments, likes
- Support for live streaming (stretch goal)

### Non-Functional Requirements
- **Scale:** ~2B users, ~500 hours of video uploaded/minute (YouTube-scale)
- **Streaming latency:** Video should start playing < 1-2 seconds after click
- **Adaptive bitrate:** Playback quality should adjust to network conditions in real time
- **High availability:** Streaming must gracefully degrade, never hard-fail
- **Durability:** Uploaded video must never be lost once processing confirms success

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Hours uploaded/minute | ~500 hours |
| Avg video size (raw) | ~1GB for 10 min HD |
| Daily storage growth (raw + all transcoded variants) | Petabytes/day |
| Concurrent streams (peak) | ~50M+ |
| CDN egress bandwidth | Dominant cost driver — Tbps scale |
| Transcoding compute | Massive parallel compute farm, GPU-accelerated |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Client["Client Apps<br/>(Web / Mobile / TV / Consoles)"]
    CDN["Global CDN / Edge Cache Network"]
    LB["Load Balancer"]
    Gateway["API Gateway"]

    subgraph Services["Core Services"]
        UploadSvc["Upload Service"]
        StreamingSvc["Streaming/Playback Service"]
        MetadataSvc["Video Metadata Service"]
        RecoSvc["Recommendation Service"]
        SearchSvc["Search Service"]
        CommentSvc["Comment/Engagement Service"]
    end

    subgraph Pipeline["Async Video Processing Pipeline"]
        Kafka["Kafka<br/>(VideoUploaded events)"]
        TranscodeQueue["Transcoding Job Queue"]
        TranscodeWorkers["Transcoding Worker Fleet<br/>(multi-resolution, multi-codec)"]
        ThumbGen["Thumbnail Generator"]
    end

    subgraph Storage["Storage Layer"]
        RawStore[("Raw Upload Storage<br/>(S3/Blob - staging)")]
        SegmentStore[("Segmented Video Storage<br/>(HLS/DASH chunks)")]
        MetadataDB[("Metadata DB<br/>(video info, view counts)")]
        SearchIndex[("Search Index<br/>(Elasticsearch)")]
    end

    Client --> CDN
    Client --> LB --> Gateway
    Gateway --> UploadSvc
    Gateway --> StreamingSvc
    Gateway --> MetadataSvc
    Gateway --> RecoSvc
    Gateway --> SearchSvc

    UploadSvc --> RawStore
    UploadSvc --> Kafka
    Kafka --> TranscodeQueue --> TranscodeWorkers
    TranscodeWorkers --> RawStore
    TranscodeWorkers --> SegmentStore
    TranscodeWorkers --> ThumbGen
    TranscodeWorkers --> MetadataDB

    SegmentStore --> CDN
    StreamingSvc --> CDN
    StreamingSvc --> MetadataDB
    RecoSvc --> MetadataDB
    SearchSvc --> SearchIndex
```

**Key idea:** Upload and playback are entirely decoupled by an async transcoding pipeline. A video isn't watchable until workers have produced multiple resolution/bitrate variants, chunked into small segments for adaptive streaming, and pushed to CDN-friendly storage.

---

## 3. Data Model (ER Diagram)

```mermaid
erDiagram
    USER ||--o{ VIDEO : uploads
    VIDEO ||--o{ VIDEO_VARIANT : "has (per resolution/bitrate)"
    VIDEO ||--o{ COMMENT : has
    VIDEO ||--o{ WATCH_HISTORY : "watched by"
    USER ||--o{ WATCH_HISTORY : watches

    USER {
        string user_id PK
        string username
        string channel_name
    }
    VIDEO {
        string video_id PK
        string user_id FK
        string title
        string description
        int duration_sec
        string status
        timestamp uploaded_at
        long view_count
    }
    VIDEO_VARIANT {
        string variant_id PK
        string video_id FK
        string resolution
        string codec
        string manifest_url
        string segment_base_path
    }
    COMMENT {
        string comment_id PK
        string video_id FK
        string user_id FK
        string text
    }
    WATCH_HISTORY {
        string user_id FK
        string video_id FK
        int watch_position_sec
        timestamp watched_at
    }
```

---

## 4. Upload & Transcoding Pipeline

```mermaid
sequenceDiagram
    participant C as Creator (Client)
    participant GW as API Gateway
    participant US as Upload Service
    participant Raw as Raw Storage
    participant DB as Metadata DB
    participant K as Kafka
    participant TW as Transcode Workers
    participant Seg as Segment Storage
    participant CDN as CDN

    C->>GW: Upload video (chunked/resumable upload)
    GW->>US: Forward chunks
    US->>Raw: Store raw file
    US->>DB: Create video record (status = UPLOADED)
    US-->>C: 202 Accepted (video_id, status: processing)

    US->>K: Emit VideoUploaded event
    K->>TW: Consume event

    par Parallel transcoding jobs
        TW->>TW: Transcode to 240p/480p/720p/1080p/4K
        TW->>TW: Encode multiple codecs (H.264, VP9, AV1)
        TW->>TW: Segment into HLS/DASH chunks (2-10s each)
    end

    TW->>Seg: Store segmented chunks + manifest files
    TW->>DB: Update video record (status = READY, variants populated)
    TW->>CDN: Push manifests + initial segments to origin
    TW->>K: Emit VideoReady event

    Note over K: Consumed by Search Indexer,<br/>Recommendation Service,<br/>Notification Service (subscribers)
```

---

## 5. Adaptive Bitrate Streaming (Playback)

```mermaid
flowchart TB
    A["Client requests to play video"] --> B["Fetch Master Manifest<br/>(lists available resolutions/bitrates)"]
    B --> C["Client Player: Measure network bandwidth"]
    C --> D{"Select initial<br/>bitrate variant"}
    D --> E["Request video segments<br/>(2-10 sec chunks) from CDN"]
    E --> F["Play segment"]
    F --> G["Continuously monitor buffer health<br/>+ network throughput"]
    G --> H{"Bandwidth changed?"}
    H -- "Degraded" --> I["Switch to lower bitrate variant<br/>for next segment"]
    H -- "Improved" --> J["Switch to higher bitrate variant<br/>for next segment"]
    H -- "Stable" --> E
    I --> E
    J --> E
```

*This is the core of HLS/DASH adaptive streaming: video is pre-cut into small independently-decodable segments at multiple quality levels. The player switches quality **between segments**, not mid-segment, based on real-time network conditions.*

---

## 6. Streaming Playback — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client Player
    participant CDN as CDN Edge
    participant SS as Streaming Service
    participant Origin as Origin (Segment Storage)
    participant DB as Metadata DB

    C->>SS: GET /watch?video_id=X
    SS->>DB: Fetch video metadata + manifest URL
    DB-->>SS: Metadata
    SS-->>C: Return player config + manifest URL

    C->>CDN: GET manifest.m3u8
    alt Manifest cached at edge
        CDN-->>C: Return manifest (cache hit)
    else Cache miss
        CDN->>Origin: Fetch manifest
        Origin-->>CDN: Manifest
        CDN-->>C: Return manifest (cached for next request)
    end

    loop For each segment during playback
        C->>CDN: GET segment_N.ts (at selected bitrate)
        alt Segment cached at edge
            CDN-->>C: Return segment (cache hit, low latency)
        else Cache miss
            CDN->>Origin: Fetch segment
            Origin-->>CDN: Segment
            CDN-->>C: Return segment (cached for future viewers)
        end
    end

    C->>SS: POST /watch_progress (periodic heartbeat)
    SS->>DB: Update watch history / view count
```

---

## 7. CDN & Content Delivery Strategy

```mermaid
flowchart TB
    subgraph Origin["Origin Infrastructure"]
        SegStore[("Segment Storage<br/>All resolutions/codecs")]
    end

    subgraph Tier1["CDN: Regional PoPs"]
        Regional1["Regional Cache - NA"]
        Regional2["Regional Cache - EU"]
        Regional3["Regional Cache - APAC"]
    end

    subgraph Tier2["CDN: Edge/ISP-level Caches"]
        Edge1["ISP Edge Cache 1"]
        Edge2["ISP Edge Cache 2"]
        Edge3["ISP Edge Cache 3"]
    end

    SegStore -->|"Popular content<br/>pre-pushed"| Regional1
    SegStore -->|"Popular content<br/>pre-pushed"| Regional2
    SegStore -->|"Popular content<br/>pre-pushed"| Regional3

    Regional1 --> Edge1
    Regional2 --> Edge2
    Regional3 --> Edge3

    Users["End Users"] --> Edge1
    Users --> Edge2
    Users --> Edge3

    Note1["Tiered caching:<br/>~95%+ of requests served from<br/>Edge/ISP tier — never touch origin"]
```

*This is exactly how Netflix's "Open Connect" and YouTube's edge network operate — placing caching appliances as close to end-users (often inside ISP networks) as possible, since video bandwidth dominates infrastructure cost.*

---

## 8. Recommendation Pipeline (Simplified)

```mermaid
flowchart LR
    A["User Watch History +<br/>Engagement Signals"] --> B["Candidate Generation<br/>(collaborative filtering,<br/>content embeddings)"]
    B --> C["Ranking Model<br/>(predicted watch-time/engagement)"]
    C --> D["Business Rules<br/>(diversity, freshness, dedup)"]
    D --> E["Personalized Homepage/Up-Next"]
```

---

## 9. Component Responsibilities Summary

```mermaid
mindmap
  root((YouTube/Netflix HLD))
    Upload Service
      Resumable/chunked upload
      Raw storage
      Emit processing event
    Transcode Workers
      Multi-resolution encoding
      Multi-codec support
      Segmentation into chunks
    Streaming Service
      Manifest generation
      Playback session management
      Watch progress tracking
    CDN Layer
      Tiered edge caching
      Adaptive bitrate delivery
      Origin shield
    Metadata Service
      Video info, view counts
      Status tracking
    Recommendation Service
      Candidate generation
      Ranking by predicted engagement
    Search Service
      Title/description/tag indexing
```

---

## 10. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Upload/playback decoupling | Async transcoding pipeline | Video isn't watchable until processed; keeps upload path fast and simple |
| Streaming protocol | HLS/DASH (segmented adaptive streaming) | Enables per-segment quality switching without interrupting playback |
| Multi-resolution encoding | Transcode into several fixed resolutions/bitrates upfront | Avoids expensive real-time transcoding per viewer; pay compute cost once |
| CDN architecture | Tiered caching, ISP-embedded edge nodes | Video bandwidth is the dominant cost; minimizing origin egress is critical |
| Segment size | Small chunks (2-10s) | Enables fast quality switches and quick playback start (only need first segment) |
| Metadata vs segment storage | Separated | Metadata DB is small/hot (frequent reads); segment storage is massive/cold-ish (served mostly via CDN) |

---

## 11. Bottlenecks & Scaling Considerations

- **Transcoding compute cost** — the single largest infra cost after CDN egress; use spot/preemptible compute fleets, GPU acceleration, and prioritize popular-format transcodes first (e.g., transcode 1080p before 4K if a video is likely low-traffic).
- **CDN cache misses for long-tail content** — unpopular videos won't be pre-warmed at edge; accept higher origin fetch latency for cold content, optimize only for the popularity curve's head.
- **Thundering herd on new viral content** — many simultaneous requests for the same just-uploaded segment; use CDN request coalescing (single origin fetch serves many waiting edge requests).
- **Live streaming** (if supported) — fundamentally different pipeline: can't pre-transcode; needs real-time encoding ladders and much smaller segment/latency budgets (sub-second to few-second glass-to-glass latency).
- **Storage cost for all variants** — storing every resolution/codec combination for every video is expensive; can lazily transcode rarely-requested combinations (e.g., AV1 for a video with 10 lifetime views) rather than upfront.
- **Global consistency of view counts** — exact real-time counts aren't critical; use approximate/eventually-consistent counters aggregated asynchronously rather than synchronous increments per view.
