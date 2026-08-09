# Design Instagram — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Users can upload photos/videos (posts)
- Users can follow/unfollow other users
- Users see a feed of posts from people they follow
- Users can like, comment on posts
- Users can post Stories (ephemeral, 24h expiry)
- Support image/video filters and processing
- Search users/hashtags

### Non-Functional Requirements
- **Scale:** ~500M DAU, ~100M photos uploaded/day
- **Read-heavy:** Read:Write ratio ~100:1
- **Low latency image serving:** < 200ms globally via CDN
- **High availability** for feed reads; some write latency (processing) is tolerable
- **Durability:** Uploaded media must never be lost

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| DAU | 500M |
| Photos uploaded/day | ~100M |
| Avg photo size | ~2MB (post-compression, multiple resolutions) |
| Daily storage growth | ~200TB/day (raw) before CDN-tier duplication |
| Peak upload QPS | ~5,000/sec |
| Feed read QPS | ~500,000/sec |
| Video content | ~35% of uploads, larger storage/bandwidth footprint |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Client["Client Apps<br/>(Web / iOS / Android)"]
    CDN["Global CDN<br/>(image/video delivery)"]
    LB["Load Balancer"]
    Gateway["API Gateway<br/>(Auth, Rate Limiting)"]

    subgraph Services["Core Services"]
        UploadSvc["Upload Service"]
        FeedSvc["Feed Service"]
        UserSvc["User / Graph Service"]
        StorySvc["Story Service"]
        SearchSvc["Search Service"]
        NotifSvc["Notification Service"]
        CommentSvc["Comment/Like Service"]
    end

    subgraph Processing["Async Media Processing"]
        Kafka["Kafka<br/>(PostUploaded, StoryCreated events)"]
        ImageProcessor["Image/Video Processing Workers<br/>(resize, transcode, filters, thumbnails)"]
        FanoutWorker["Feed Fanout Workers"]
    end

    subgraph Storage["Storage Layer"]
        BlobStore[("Object Storage<br/>(S3 - original + derived media)")]
        PostDB[("Post Metadata DB<br/>(Cassandra/DynamoDB)")]
        GraphDB[("Social Graph DB")]
        FeedCache[("Feed Cache<br/>(Redis)")]
        SearchIndex[("Search Index<br/>(Elasticsearch)")]
        StoryStore[("Story Store<br/>(TTL-based, Redis/DynamoDB)")]
    end

    Client --> CDN
    Client --> LB --> Gateway
    Gateway --> UploadSvc
    Gateway --> FeedSvc
    Gateway --> UserSvc
    Gateway --> StorySvc
    Gateway --> SearchSvc
    Gateway --> CommentSvc

    UploadSvc --> BlobStore
    UploadSvc --> PostDB
    UploadSvc --> Kafka

    Kafka --> ImageProcessor
    ImageProcessor --> BlobStore
    ImageProcessor --> CDN

    Kafka --> FanoutWorker
    FanoutWorker --> FeedCache
    FeedSvc --> FeedCache
    FeedSvc --> PostDB

    StorySvc --> StoryStore
    UserSvc --> GraphDB
    SearchSvc --> SearchIndex
    CommentSvc --> PostDB
```

**Key idea:** Upload is decoupled from processing. A client uploads raw media, gets a fast ack once it's durably stored, and heavy work (resizing, transcoding, filters, thumbnail generation) happens asynchronously via workers before the post appears in feeds.

---

## 3. Data Model (ER Diagram)

```mermaid
erDiagram
    USER ||--o{ POST : creates
    USER ||--o{ FOLLOW : "follows/followed by"
    USER ||--o{ STORY : creates
    POST ||--o{ COMMENT : has
    POST ||--o{ LIKE : receives
    POST ||--o{ MEDIA_ASSET : contains

    USER {
        string user_id PK
        string username
        string profile_pic_url
        int follower_count
        int following_count
    }
    POST {
        string post_id PK
        string user_id FK
        string caption
        int like_count
        int comment_count
        timestamp created_at
    }
    MEDIA_ASSET {
        string asset_id PK
        string post_id FK
        string type
        string original_url
        string thumbnail_url
        string resolution_variant
    }
    STORY {
        string story_id PK
        string user_id FK
        string media_url
        timestamp created_at
        timestamp expires_at
    }
    COMMENT {
        string comment_id PK
        string post_id FK
        string user_id FK
        string text
        timestamp created_at
    }
```

---

## 4. Media Upload & Processing Pipeline

```mermaid
sequenceDiagram
    participant C as Client
    participant GW as API Gateway
    participant US as Upload Service
    participant S3 as Object Storage (raw)
    participant DB as Post DB
    participant K as Kafka
    participant IP as Image Processor Workers
    participant CDN as CDN

    C->>GW: POST /upload (multipart media + caption)
    GW->>US: Forward request
    US->>S3: Store raw media (original resolution)
    S3-->>US: raw_url
    US->>DB: Create post record (status = PROCESSING)
    US-->>C: 202 Accepted (post_id, status: processing)

    US->>K: Emit PostUploaded event
    K->>IP: Consume event

    par Parallel processing
        IP->>IP: Generate thumbnail (150x150)
        IP->>IP: Generate feed resolution (1080p)
        IP->>IP: Apply filters (if requested)
        IP->>IP: Transcode video (multiple bitrates, if video)
    end

    IP->>S3: Store derived assets
    IP->>CDN: Push/invalidate cache for new assets
    IP->>DB: Update post record (status = READY, urls populated)
    IP->>K: Emit PostReady event

    Note over K: Consumed by Feed Fanout Workers<br/>and Search Indexer
```

---

## 5. Feed Generation (Hybrid Push/Pull — Same Core Problem as Twitter)

```mermaid
flowchart TB
    A["Post marked READY"] --> B{"Author follower count<br/>&gt; threshold?"}
    B -- "No (normal user)" --> C["Fanout-on-Write<br/>Push post_id to all followers' feed cache"]
    B -- "Yes (influencer/celebrity)" --> D["Skip fanout<br/>Mark for pull-based inclusion"]

    E["User requests feed"] --> F["Read pre-computed feed<br/>from Redis cache"]
    F --> G{"User follows<br/>influencers?"}
    G -- Yes --> H["Fetch influencer posts<br/>on-demand from Post DB"]
    G -- No --> I["Return cached feed as-is"]
    H --> J["Merge + rank by ML relevance score"]
    I --> K["Return to user"]
    J --> K
```

---

## 6. Feed Read Path — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client
    participant GW as API Gateway
    participant FS as Feed Service
    participant Cache as Redis (Feed Cache)
    participant DB as Post DB
    participant Rank as Ranking Service
    participant CDN as CDN

    C->>GW: GET /feed
    GW->>FS: Forward request
    FS->>Cache: LRANGE feed:{user_id} 0 50
    Cache-->>FS: List of post_ids

    FS->>DB: Batch fetch post metadata
    DB-->>FS: Post objects (captions, counts, media refs)

    FS->>Rank: Send candidates for ranking
    Rank-->>FS: Ranked, filtered list

    FS-->>C: Return feed JSON (media URLs point to CDN)
    C->>CDN: Fetch actual images/video directly
    CDN-->>C: Media bytes (edge-cached)
```

---

## 7. Stories Architecture (Ephemeral Content)

```mermaid
flowchart LR
    A["User posts Story"] --> B["Story Service"]
    B --> C["Store in TTL-based store<br/>(Redis/DynamoDB w/ 24h TTL)"]
    B --> D["Upload media to S3 + CDN"]
    C --> E["Auto-expire after 24h<br/>(TTL eviction, no cleanup job needed)"]

    F["Follower opens app"] --> G["Story Service: Get active stories<br/>from followed users"]
    G --> H["Return story tray<br/>(ordered by recency/unseen status)"]
```

*Stories use a **separate storage path** with native TTL expiry rather than the main Post DB — avoids polluting the permanent feed pipeline with self-deleting content and lets the datastore handle cleanup natively.*

---

## 8. Media Storage & CDN Strategy

```mermaid
flowchart TB
    subgraph Origin["Origin Storage"]
        S3Raw[("S3: Original uploads")]
        S3Derived[("S3: Derived resolutions<br/>(thumbnail, feed, full-res)")]
    end

    subgraph EdgeNetwork["CDN Edge Network"]
        Edge1["Edge PoP - US"]
        Edge2["Edge PoP - EU"]
        Edge3["Edge PoP - APAC"]
    end

    S3Derived -->|"Pull-through cache<br/>on first request"| Edge1
    S3Derived -->|"Pull-through cache"| Edge2
    S3Derived -->|"Pull-through cache"| Edge3

    Client1["User (US)"] --> Edge1
    Client2["User (EU)"] --> Edge2
    Client3["User (APAC)"] --> Edge3

    Note1["Cache-Control headers set long TTL<br/>(media is immutable once processed)<br/>Cache-busting via new URL on edit, not invalidation"]
```

---

## 9. Component Responsibilities Summary

```mermaid
mindmap
  root((Instagram HLD))
    Upload Service
      Accept raw media
      Durable write to S3
      Emit processing event
    Image/Video Processor
      Resize/transcode
      Thumbnail generation
      Filter application
    Feed Service
      Hybrid push/pull read
      Cache-first reads
      Pagination
    Story Service
      TTL-based ephemeral storage
      Separate from main feed pipeline
    Graph Service
      Follow/unfollow
      Follower/following lookups
    Search Service
      Hashtag/user search
      Elasticsearch indexing
    Comment/Like Service
      High-write counters
      Denormalized count updates
```

---

## 10. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Upload flow | Async processing after fast ack | User doesn't wait for resizing/transcoding; better perceived latency |
| Feed generation | Hybrid push/pull | Same celebrity-fanout problem as Twitter; push for normal users, pull for influencers |
| Media storage | Multi-resolution derivatives, not on-the-fly resize | Avoids repeated CPU cost per view; trade storage for compute |
| Stories storage | Separate TTL-based store | Native expiry, keeps main Post DB clean of ephemeral writes |
| CDN strategy | Pull-through caching, long TTL, immutable URLs | Media rarely changes once processed; maximizes cache hit rate |
| Like/comment counts | Denormalized counters on Post record | Avoids COUNT() queries at read time; updated via async counter service |

---

## 11. Bottlenecks & Scaling Considerations

- **Processing pipeline backlog during viral spikes** → auto-scale worker pool horizontally; prioritize thumbnail generation (needed for feed) over full-res processing.
- **Celebrity/influencer fanout** → same hybrid solution as Twitter; threshold-based routing.
- **Hot posts (viral content) overwhelming like/comment counters** → use approximate counters or write-behind aggregation instead of synchronous increments.
- **CDN cold-start for newly uploaded viral content** → pre-warm edge caches for posts from high-follower accounts.
- **Story storage growth** → TTL handles cleanup automatically, but ensure sharding accounts for time-based access patterns (recent stories hot, expiring ones cold).
- **Cross-region media availability** → replicate S3 buckets across regions; CDN handles most of the latency concern regardless.
