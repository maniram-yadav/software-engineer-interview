# Design Twitter/X — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Users can post tweets (text, images, video, up to 280 chars for text)
- Users can follow/unfollow other users
- Users see a home timeline of tweets from people they follow
- Users can like, retweet, reply
- Users can search tweets
- Trending topics / hashtags

### Non-Functional Requirements
- **Scale:** ~500M DAU, ~5,000 tweets/sec average, ~50,000/sec peak
- **Read-heavy:** Read:Write ratio ~1000:1 (timeline reads dominate)
- **Low latency:** Timeline load < 200ms p99
- **Availability > Consistency** for timeline (eventual consistency acceptable)
- **Durability:** Tweets must never be lost once acknowledged

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| DAU | 500M |
| Avg tweets/user/day | 2 |
| Tweets/day | ~1B |
| Writes/sec (avg) | ~5,000 |
| Timeline reads/sec | ~500,000+ |
| Avg followers/user | 200 (with celebrity outliers at 100M+) |
| Storage/tweet (with metadata) | ~300 bytes |
| Daily storage growth | ~300 GB/day (text only, media separate) |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Client["Client Apps<br/>(Web / iOS / Android)"]
    CDN["CDN<br/>(static + media)"]
    LB["Load Balancer"]
    Gateway["API Gateway<br/>(Auth, Rate Limiting, Routing)"]

    subgraph Services["Core Services"]
        TweetSvc["Tweet Service"]
        TimelineSvc["Timeline Service"]
        UserSvc["User / Graph Service"]
        SearchSvc["Search Service"]
        NotifSvc["Notification Service"]
        MediaSvc["Media Service"]
    end

    subgraph Async["Async / Streaming Layer"]
        Kafka["Kafka<br/>(Event Bus: TweetCreated, Follow, Like)"]
        FanoutWorker["Fanout Workers"]
    end

    subgraph Storage["Storage Layer"]
        TweetDB[("Tweet Store<br/>(Cassandra/DynamoDB)")]
        GraphDB[("Social Graph DB<br/>(Follower/Following)")]
        TimelineCache[("Timeline Cache<br/>(Redis)")]
        SearchIndex[("Search Index<br/>(Elasticsearch)")]
        ObjectStore[("Object Storage<br/>(S3 - media)")]
    end

    Client --> CDN
    Client --> LB --> Gateway
    Gateway --> TweetSvc
    Gateway --> TimelineSvc
    Gateway --> UserSvc
    Gateway --> SearchSvc
    Gateway --> MediaSvc

    TweetSvc --> TweetDB
    TweetSvc --> Kafka
    MediaSvc --> ObjectStore
    MediaSvc --> CDN

    Kafka --> FanoutWorker
    Kafka --> SearchIndex
    Kafka --> NotifSvc

    FanoutWorker --> TimelineCache
    TimelineSvc --> TimelineCache
    TimelineSvc --> TweetDB

    UserSvc --> GraphDB
    SearchSvc --> SearchIndex
```

**Key idea:** Writes (tweet creation) go through a fast synchronous path to durable storage, then fan out asynchronously via Kafka to pre-compute timelines — decoupling the slow "push to all followers" work from the user-facing write latency.

---

## 3. Data Model (ER Diagram)

```mermaid
erDiagram
    USER ||--o{ TWEET : posts
    USER ||--o{ FOLLOW : "follows (as follower)"
    USER ||--o{ FOLLOW : "followed by (as followee)"
    TWEET ||--o{ LIKE : receives
    TWEET ||--o{ RETWEET : receives
    TWEET ||--o{ REPLY : receives
    TWEET ||--o{ MEDIA : contains

    USER {
        string user_id PK
        string username
        string bio
        int follower_count
        int following_count
        timestamp created_at
    }
    TWEET {
        string tweet_id PK
        string user_id FK
        string text
        string media_id FK
        int like_count
        int retweet_count
        timestamp created_at
    }
    FOLLOW {
        string follower_id FK
        string followee_id FK
        timestamp followed_at
    }
    LIKE {
        string user_id FK
        string tweet_id FK
        timestamp liked_at
    }
    MEDIA {
        string media_id PK
        string s3_url
        string type
    }
```

---

## 4. Feed Generation: Fanout-on-Write vs Fanout-on-Read

This is the **core hard problem** of Twitter's design. Two strategies, blended in practice.

### 4.1 Fanout-on-Write (Push Model) — used for normal users

```mermaid
sequenceDiagram
    participant U as User (Author)
    participant TS as Tweet Service
    participant DB as Tweet DB
    participant K as Kafka
    participant FW as Fanout Worker
    participant GDB as Graph DB
    participant TC as Timeline Cache (Redis)

    U->>TS: POST /tweet
    TS->>DB: Persist tweet
    TS-->>U: 200 OK (fast ack)
    TS->>K: Publish TweetCreated event

    K->>FW: Consume event
    FW->>GDB: Get list of followers
    loop For each follower
        FW->>TC: Push tweet_id into follower's timeline cache (list)
    end
```

**Why:** Precomputing means a follower's `GET /timeline` is just a cheap Redis `LRANGE` — O(1) fast read. Since reads vastly outnumber writes, we pay the fanout cost once at write time instead of on every read.

**Problem:** Celebrities with 100M followers — a single tweet would trigger 100M cache writes. This is the **"hot key" / "celebrity problem."**

### 4.2 Fanout-on-Read (Pull Model) — used for celebrities

```mermaid
sequenceDiagram
    participant U as User (Reader)
    participant TS as Timeline Service
    participant TC as Timeline Cache
    participant GDB as Graph DB
    participant DB as Tweet DB

    U->>TS: GET /timeline
    TS->>TC: Fetch pre-computed timeline (regular follows)
    TS->>GDB: Get list of celebrities user follows
    TS->>DB: Fetch recent tweets directly from those celebrities
    TS->>TS: Merge + rank both sets by recency/relevance
    TS-->>U: Return merged timeline
```

### 4.3 Hybrid Model (What Twitter Actually Does)

```mermaid
flowchart LR
    A["New Tweet"] --> B{"Author follower count<br/>&gt; threshold?<br/>(e.g. 1M)"}
    B -- "No (normal user)" --> C["Fanout-on-Write<br/>Push to all followers' caches"]
    B -- "Yes (celebrity)" --> D["Skip fanout<br/>Mark as 'celebrity tweet'"]

    E["User requests timeline"] --> F["Read pre-computed<br/>timeline from cache"]
    F --> G{"User follows<br/>any celebrities?"}
    G -- Yes --> H["Fetch celebrity tweets<br/>on-demand (pull)"]
    G -- No --> I["Return cached timeline as-is"]
    H --> J["Merge + rank by time"]
    I --> K["Return to user"]
    J --> K
```

---

## 5. Timeline Ranking Pipeline

```mermaid
flowchart TB
    A["Candidate Generation<br/>(cached timeline + celebrity pulls)"] --> B["Feature Enrichment<br/>(recency, engagement, author affinity)"]
    B --> C["ML Ranking Model<br/>(predicted engagement score)"]
    C --> D["Filtering<br/>(remove blocked/muted/seen)"]
    D --> E["Diversity/Ads Injection"]
    E --> F["Final Ranked Timeline<br/>returned to client"]
```

---

## 6. Write Path (Tweet Creation) — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client
    participant GW as API Gateway
    participant TS as Tweet Service
    participant M as Media Service
    participant S3 as Object Storage
    participant DB as Tweet DB (Cassandra)
    participant K as Kafka

    C->>M: Upload media (if any)
    M->>S3: Store image/video
    S3-->>M: media_url
    M-->>C: media_id

    C->>GW: POST /tweet {text, media_id}
    GW->>TS: Forward request (authenticated)
    TS->>TS: Validate + generate tweet_id (Snowflake ID)
    TS->>DB: Write tweet (partitioned by user_id)
    DB-->>TS: Ack
    TS-->>C: 201 Created

    TS->>K: Emit TweetCreated event
    Note over K: Consumed async by:<br/>Fanout Workers, Search Indexer,<br/>Notification Service, Analytics
```

---

## 7. Read Path (Home Timeline) — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client
    participant GW as API Gateway
    participant TL as Timeline Service
    participant Cache as Redis (Timeline Cache)
    participant DB as Tweet DB
    participant Rank as Ranking Service

    C->>GW: GET /home_timeline
    GW->>TL: Forward request
    TL->>Cache: LRANGE timeline:{user_id} 0 100
    Cache-->>TL: List of tweet_ids

    TL->>DB: Batch GET tweet content by IDs
    DB-->>TL: Tweet objects (hydrated)

    TL->>Rank: Send candidates for ranking
    Rank-->>TL: Ranked + filtered list

    TL-->>C: Return final timeline JSON
```

---

## 8. Social Graph Storage

```mermaid
flowchart LR
    subgraph GraphDB["Social Graph DB"]
        direction TB
        A["follower_id → [followee_ids]<br/>(Who I follow)"]
        B["followee_id → [follower_ids]<br/>(My followers)"]
    end

    Note1["Read pattern 1:<br/>'Who does user X follow?'<br/>→ used at write-time fanout"]
    Note2["Read pattern 2:<br/>'Who follows user X?'<br/>→ used to build follower list for fanout"]

    A -.-> Note1
    B -.-> Note2
```

*Stored as a dedicated graph/wide-column store (e.g., FlockDB-style or Cassandra wide rows) since it needs to support both directions efficiently at massive fan-out scale.*

---

## 9. Component Responsibilities Summary

```mermaid
mindmap
  root((Twitter HLD))
    Tweet Service
      Write path
      Snowflake ID generation
      Validation
    Timeline Service
      Read path
      Merge push/pull results
      Pagination
    Fanout Workers
      Consume Kafka events
      Push to follower caches
      Celebrity threshold logic
    Graph Service
      Follow/unfollow
      Follower/following lookups
    Search Service
      Elasticsearch indexing
      Hashtag/trend detection
    Media Service
      Upload handling
      S3 storage
      CDN distribution
    Notification Service
      Push/email fanout
      Mentions, likes, replies
```

---

## 10. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Timeline generation | Hybrid push/pull | Avoids celebrity fanout explosion while keeping normal-user reads O(1) |
| Tweet ID generation | Snowflake (time-sortable, distributed) | No central counter bottleneck, IDs sortable by time |
| Tweet storage | Wide-column store (Cassandra/DynamoDB) | High write throughput, partition by user_id |
| Timeline cache | Redis sorted sets/lists | Fast O(1) reads, TTL-based eviction for inactive users |
| Consistency model | Eventual consistency for timelines | Users tolerate a tweet appearing a few seconds late; availability matters more |
| Event backbone | Kafka | Decouples write path from fanout, search indexing, notifications |
| Media delivery | CDN + Object Storage | Offload heavy bandwidth from core services |

---

## 11. Bottlenecks & Scaling Considerations

- **Celebrity fanout** → solved via hybrid pull model + threshold-based routing.
- **Hot partitions in Tweet DB** → shard by `user_id` with consistent hashing; watch for celebrity accounts creating hot shards.
- **Timeline cache memory pressure** → TTL-evict timelines for inactive users; only keep top N (e.g., 800) tweet_ids per user, not full history.
- **Kafka consumer lag during fanout spikes** (e.g., breaking news) → auto-scale fanout worker pool, apply backpressure.
- **Search indexing lag** → acceptable to be near-real-time (few seconds) rather than synchronous.
- **Cross-region replication** → async replication with conflict resolution for global availability; accept eventual consistency across regions.
