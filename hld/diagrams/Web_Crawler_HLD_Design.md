# Design a Web Crawler — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Given seed URLs, discover and crawl linked pages recursively
- Extract and store page content for downstream indexing
- Respect `robots.txt` and crawl-delay directives (politeness)
- Avoid re-crawling the same URL redundantly within a short window
- Detect and handle duplicate content (different URLs, same content)
- Prioritize re-crawling frequently-changing pages more often than static ones

### Non-Functional Requirements
- **Scale:** Crawl billions of pages across the web
- **Politeness:** Never overwhelm any single web server with concurrent requests
- **Freshness:** Important/frequently-changing pages should be re-crawled more often
- **Efficiency:** Avoid wasted work — don't recrawl unchanged content, don't crawl duplicate/low-value pages
- **Fault tolerance:** Individual page failures (timeouts, 404s, malformed HTML) must not halt the overall crawl

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Total pages to crawl | ~10B+ |
| Pages crawled/sec (platform-wide) | ~10,000-50,000 |
| Avg page size | ~100KB (HTML) |
| URL frontier size | Billions of URLs queued at any time |
| Distinct domains | ~100M+ |
| Politeness delay per domain | Configurable, often 1+ seconds between requests |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    subgraph Frontier["URL Frontier (Priority Queue System)"]
        PriorityQueue["Priority Queues<br/>(by crawl priority/freshness need)"]
        DomainQueues["Per-Domain Sub-Queues<br/>(politeness enforcement)"]
    end

    subgraph Fetching["Fetching Layer"]
        FetcherPool["Fetcher Worker Pool<br/>(distributed, many instances)"]
        RobotsCache["robots.txt Cache<br/>(per domain)"]
        DNSCache["DNS Resolution Cache"]
    end

    subgraph Processing["Processing Layer"]
        Parser["HTML Parser<br/>(extract links, content)"]
        DupDetector["Duplicate Content Detector<br/>(content hashing)"]
        URLFilter["URL Filter/Normalizer"]
    end

    subgraph Storage["Storage Layer"]
        SeenURLs[("Seen URL Store<br/>(Bloom filter + DB)")]
        ContentStore[("Crawled Content Store<br/>(raw HTML)")]
        URLGraph[("URL Graph / Metadata DB<br/>(crawl history, priority scores)")]
    end

    Downstream["Downstream: Indexing Pipeline"]

    PriorityQueue --> DomainQueues --> FetcherPool
    FetcherPool --> RobotsCache
    FetcherPool --> DNSCache
    FetcherPool --> Parser

    Parser --> DupDetector
    Parser --> URLFilter
    DupDetector --> ContentStore

    URLFilter --> SeenURLs
    SeenURLs -->|"New URL"| PriorityQueue
    URLFilter --> URLGraph

    ContentStore --> Downstream
```

**Key idea:** The entire system revolves around the **URL Frontier** — a distributed priority queue system that must balance three competing concerns simultaneously: crawl priority (important pages first), politeness (don't hammer any one domain), and freshness (recrawl changing content more often). Everything downstream (fetching, parsing, storage) exists to feed URLs back into this frontier.

---

## 3. Data Model

```mermaid
erDiagram
    URL_METADATA {
        string url PK
        string domain
        string status "pending/crawled/failed"
        float priority_score
        timestamp last_crawled_at
        timestamp next_crawl_at
        string content_hash
        int crawl_count
    }
    DOMAIN_METADATA {
        string domain PK
        string robots_txt_content
        timestamp robots_fetched_at
        int crawl_delay_ms
        timestamp last_request_at
    }
    CRAWLED_CONTENT {
        string url FK
        string content_hash PK
        bytes raw_html
        timestamp crawled_at
    }
    LINK_GRAPH {
        string source_url FK
        string target_url FK
    }
```

---

## 4. URL Frontier Design (The Hard Problem)

```mermaid
flowchart TB
    A["URL Frontier"] --> B["Front Queues<br/>(prioritization)"]
    A --> C["Back Queues<br/>(politeness)"]

    B --> B1["Priority 1: High-value pages<br/>(news homepages, frequently updated)"]
    B --> B2["Priority 2: Normal pages"]
    B --> B3["Priority 3: Low-value/rarely-changing"]

    B1 & B2 & B3 --> D["Biased random selection<br/>from front queues<br/>(higher priority = more often selected)"]
    D --> C

    C --> E["One back queue PER DOMAIN<br/>(or per small group of domains)"]
    E --> F["Fetcher pulls from a back queue<br/>ONLY IF that domain's<br/>politeness delay has elapsed"]
```

*This two-tier front/back queue design (from the classic Mercator crawler architecture) elegantly decouples prioritization from politeness — the front queues decide *what's important*, while the back queues (one per domain) enforce *how fast we're allowed to hit any single server*, regardless of how urgently we want to crawl its content.*

---

## 5. Politeness Enforcement — Detailed Sequence

```mermaid
sequenceDiagram
    participant Frontier as URL Frontier
    participant DomainQ as Domain Back-Queue
    participant Fetcher as Fetcher Worker
    participant RobotsCache as robots.txt Cache
    participant Site as example.com

    Fetcher->>DomainQ: Request next URL for example.com
    DomainQ->>DomainQ: Check: has crawl_delay elapsed<br/>since last request to this domain?

    alt Delay not yet elapsed
        DomainQ-->>Fetcher: Not ready, wait
    else Delay elapsed
        DomainQ-->>Fetcher: Here's the next URL

        Fetcher->>RobotsCache: Check robots.txt rules for this path
        alt robots.txt not cached or expired
            Fetcher->>Site: GET /robots.txt
            Site-->>Fetcher: robots.txt content
            Fetcher->>RobotsCache: Cache it
        end

        RobotsCache-->>Fetcher: Allowed/Disallowed

        alt Path disallowed
            Fetcher->>Fetcher: Skip this URL entirely
        else Path allowed
            Fetcher->>Site: GET requested page
            Site-->>Fetcher: HTML content
            Fetcher->>DomainQ: Update last_request_at = now()
        end
    end
```

---

## 6. Page Fetch & Processing Pipeline

```mermaid
sequenceDiagram
    participant Fetcher as Fetcher Worker
    participant Parser as HTML Parser
    participant Dup as Dup Content Detector
    participant Filter as URL Filter/Normalizer
    participant Seen as Seen URL Store
    participant Frontier as URL Frontier
    participant Store as Content Store

    Fetcher->>Fetcher: Fetch page (with timeout)
    Fetcher->>Parser: Raw HTML

    Parser->>Parser: Extract: text content, outbound links,<br/>metadata (title, canonical URL)

    Parser->>Dup: Compute content hash<br/>(e.g., SimHash for near-dup detection)
    Dup->>Dup: Compare against known hashes
    alt Duplicate/near-duplicate content
        Dup-->>Parser: Skip storage,<br/>but still process links
    else Unique content
        Dup->>Store: Store content
    end

    loop For each extracted link
        Parser->>Filter: Normalize URL<br/>(resolve relative paths, strip<br/>tracking params, lowercase domain)
        Filter->>Filter: Apply exclusion rules<br/>(file types, blocked domains)
        Filter->>Seen: Check Bloom filter:<br/>have we seen this URL before?
        alt New URL
            Seen-->>Filter: Not seen
            Filter->>Frontier: Add to frontier<br/>with computed priority
        else Already seen
            Seen-->>Filter: Already seen — skip
        end
    end
```

---

## 7. Seen-URL Detection at Scale (Bloom Filter)

```mermaid
flowchart TB
    A["Billions of URLs to<br/>check for 'have we seen this?'"] --> B{"Naive approach:<br/>DB lookup per URL"}
    B --> C["Too slow at this scale —<br/>billions of point queries"]

    A --> D["Bloom Filter approach"]
    D --> E["Probabilistic set membership,<br/>compact in-memory structure"]
    E --> F["Fast O(1) check:<br/>'definitely not seen' OR<br/>'possibly seen'"]
    F --> G{"Possibly seen?"}
    G -- "Definitely not seen" --> H["New URL — safe to add<br/>without further checking"]
    G -- "Possibly seen<br/>(could be false positive)" --> I["Confirm against<br/>authoritative DB<br/>(rare — only for<br/>positive bloom hits)"]
```

**Why a Bloom filter:** At billions of URLs, storing and querying a traditional index for "have I seen this exact URL" is prohibitively expensive at the required throughput. A Bloom filter trades a small, tunable false-positive rate for massive space and speed savings — the rare false positives just mean an occasional unnecessary DB double-check, never a missed duplicate.

---

## 8. Crawl Priority & Freshness Scheduling

```mermaid
flowchart TB
    A["Page priority score"] --> B["Factors:"]
    B --> C["Inbound link count<br/>(PageRank-style importance)"]
    B --> D["Historical change frequency<br/>(how often has this page<br/>actually changed on recrawl?)"]
    B --> E["Domain authority/trust score"]
    B --> F["Content type<br/>(news = high freshness need,<br/>static archive = low)"]

    C & D & E & F --> G["Composite priority score"]
    G --> H["Determines: which front queue,<br/>and next_crawl_at timestamp"]

    I["Page hasn't changed<br/>on last 5 recrawls"] --> J["Exponentially increase<br/>next_crawl_at interval<br/>(adaptive freshness)"]
    K["Page changes every recrawl"] --> L["Keep recrawl interval short"]
```

*This adaptive scheduling (inspired by how real search engine crawlers operate) avoids wasting crawl budget on static pages that never change, while ensuring frequently-updated pages (news sites, forums) are recrawled often enough to stay fresh in the index.*

---

## 9. Distributed Crawling Coordination

```mermaid
flowchart TB
    A["Global URL Frontier<br/>(logically one, physically sharded)"] --> B["Shard by domain hash"]
    B --> C["Fetcher Pool Region 1<br/>handles domain shard A"]
    B --> D["Fetcher Pool Region 2<br/>handles domain shard B"]
    B --> E["Fetcher Pool Region 3<br/>handles domain shard C"]

    Note1["Sharding by domain (not URL)<br/>ensures politeness enforcement<br/>for a given domain stays<br/>within a single shard —<br/>no cross-shard coordination<br/>needed to avoid hammering<br/>the same server twice"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Web Crawler HLD))
    URL Frontier
      Front queues for priority
      Back queues for politeness
      Domain-sharded distribution
    Fetcher Workers
      HTTP fetch with timeout
      robots.txt compliance
      DNS caching
    HTML Parser
      Content extraction
      Link discovery
    Dup Content Detector
      SimHash/content fingerprinting
      Avoids storing redundant pages
    URL Filter/Normalizer
      Canonicalization
      Exclusion rule enforcement
    Seen URL Store
      Bloom filter for fast checks
      Authoritative DB for confirmation
    Priority Scheduler
      PageRank-style importance
      Adaptive freshness intervals
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Frontier architecture | Front/back queue split (Mercator-style) | Cleanly separates "what's important to crawl" from "how fast are we allowed to crawl it" |
| Politeness enforcement | Per-domain queue with delay tracking | Prevents overwhelming any single server, regardless of that domain's crawl priority |
| Duplicate URL detection | Bloom filter + authoritative DB fallback | Achieves near-constant-time checks at billions-of-URLs scale, at the cost of a tunable, rare false-positive rate |
| Duplicate content detection | Content hashing (SimHash for near-duplicates) | Prevents wasted storage/indexing on mirrored or near-identical pages across different URLs |
| Recrawl scheduling | Adaptive interval based on observed change frequency | Concentrates crawl budget on genuinely changing content instead of uniformly recrawling everything |
| Distribution strategy | Shard by domain, not by URL | Keeps politeness enforcement for a given domain within one shard, avoiding cross-shard coordination overhead |

---

## 12. Bottlenecks & Scaling Considerations

- **Politeness vs throughput tension** — the fundamental crawler tradeoff: going faster means crawling more content sooner, but politeness constraints per domain cap how fast any single domain can be crawled — overall throughput scales by increasing the *number of distinct domains* crawled in parallel, not by crawling any one domain faster.
- **URL frontier as a potential bottleneck** — with billions of URLs, the frontier itself must be a distributed system (sharded, likely backed by a distributed queue/DB), not a single in-memory structure.
- **Malicious/infinite crawl traps** — some sites generate infinite URLs dynamically (e.g., calendar pages with "next month" links forever); needs safeguards like max-depth limits, per-domain URL count caps, and pattern detection for URL-generating traps.
- **Fetch timeouts and slow servers** — a single unresponsive server shouldn't tie up fetcher resources indefinitely; aggressive timeouts and circuit-breaking per domain prevent slow sites from degrading overall crawl throughput.
- **DNS resolution overhead** — resolving domains repeatedly at this scale is expensive; a dedicated DNS cache layer significantly reduces redundant resolution work across the fetcher fleet.
- **Storage growth** — raw HTML for 10B+ pages is enormous; typically compressed heavily and/or only diffs stored for recrawled-but-unchanged pages, with old snapshots eventually archived to cold storage.
- **Freshness/coverage tradeoff** — crawl budget is always finite; prioritizing freshness (recrawling known pages) trades off against coverage (discovering new pages) — this balance is a continuous tuning decision, not a fixed architectural answer.
