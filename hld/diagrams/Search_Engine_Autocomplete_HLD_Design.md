# Design a Search Engine / Autocomplete System — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Full-text search across a large document/content corpus
- Relevance-ranked results, not just keyword matching
- Autocomplete/typeahead suggestions as the user types
- Typo tolerance (fuzzy matching)
- Faceted filtering (e.g., filter by category, date, price range)

### Non-Functional Requirements
- **Low latency:** Search results < 100-200ms; autocomplete suggestions < 50ms (feels instant while typing)
- **Freshness:** New/updated content should become searchable within seconds to minutes, not hours
- **Scale:** Billions of documents indexed, tens of thousands of queries/sec
- **Relevance quality:** Results must be ranked by genuine relevance, not just presence of keywords
- **High availability:** Search must degrade gracefully (e.g., slightly stale index) rather than hard-fail

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Documents indexed | ~10B |
| Search queries/sec | ~50,000 |
| Autocomplete requests/sec | ~500,000+ (fired per keystroke) |
| Index size | Terabytes, sharded across many nodes |
| Avg document size | ~2KB (text content) |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Client["Client<br/>(Search Box)"]
    Gateway["API Gateway"]

    subgraph QueryPath["Query Path"]
        SearchSvc["Search Service"]
        AutocompleteSvc["Autocomplete Service"]
        RankingSvc["Ranking Service"]
    end

    subgraph IndexPath["Indexing Path"]
        IngestSvc["Content Ingestion Service"]
        Kafka["Kafka<br/>(DocumentCreated/Updated events)"]
        IndexerWorkers["Indexer Workers<br/>(tokenize, build inverted index)"]
    end

    subgraph Storage["Storage Layer"]
        InvertedIndex[("Inverted Index<br/>(sharded — term → doc_ids)")]
        DocStore[("Document Store<br/>(raw content, metadata)")]
        TrieIndex[("Trie/FST Index<br/>(for autocomplete)")]
        PopularityStore[("Query Popularity Store<br/>(for ranking suggestions)")]
    end

    Client -->|"search query"| Gateway --> SearchSvc
    SearchSvc --> InvertedIndex
    SearchSvc --> RankingSvc
    RankingSvc --> DocStore
    SearchSvc --> DocStore

    Client -->|"keystroke"| Gateway --> AutocompleteSvc
    AutocompleteSvc --> TrieIndex
    AutocompleteSvc --> PopularityStore

    IngestSvc --> Kafka --> IndexerWorkers
    IndexerWorkers --> InvertedIndex
    IndexerWorkers --> DocStore
    IndexerWorkers --> TrieIndex
```

**Key idea:** Search and autocomplete are architecturally distinct subsystems sharing the same content pipeline. Search relies on an **inverted index** (term → matching documents) for full-text relevance matching, while autocomplete relies on a completely different structure — a **trie/FST (finite state transducer)** — optimized for prefix matching at extremely low latency.

---

## 3. The Inverted Index — Core Data Structure

```mermaid
flowchart TB
    A["Document 1: 'the quick brown fox'"] --> B["Tokenize + normalize<br/>(lowercase, remove stopwords,<br/>stem: 'running' → 'run')"]
    C["Document 2: 'quick delivery service'"] --> B

    B --> D["Inverted Index"]
    D --> E["'quick' → [doc1, doc2]"]
    D --> F["'brown' → [doc1]"]
    D --> G["'fox' → [doc1]"]
    D --> H["'delivery' → [doc2]"]
    D --> I["'service' → [doc2]"]

    J["Query: 'quick'"] --> K["Direct lookup:<br/>O(1) → [doc1, doc2]"]
```

*The inverted index is what makes full-text search fast — instead of scanning every document for a query term (O(N) documents), you do a direct lookup of the term to get its matching document list (O(1) lookup, then merge/rank).*

---

## 4. Document Indexing Pipeline

```mermaid
sequenceDiagram
    participant Src as Content Source<br/>(new/updated document)
    participant Ingest as Ingestion Service
    participant K as Kafka
    participant Idx as Indexer Worker
    participant DocStore as Document Store
    participant InvIdx as Inverted Index Shards
    participant Trie as Trie Index

    Src->>Ingest: New/updated document
    Ingest->>DocStore: Store raw document + metadata
    Ingest->>K: Emit DocumentIndexRequested event

    K->>Idx: Consume event
    Idx->>Idx: Tokenize text<br/>(lowercase, stopword removal, stemming)
    Idx->>Idx: Compute term frequencies (for ranking)

    loop For each term
        Idx->>InvIdx: Determine shard: hash(term) % N
        Idx->>InvIdx: Append doc_id to term's posting list
    end

    Idx->>Trie: Update prefix structures<br/>(for titles/common query terms)
    Idx->>DocStore: Mark document status = INDEXED
```

---

## 5. Search Query Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client
    participant SS as Search Service
    participant InvIdx as Inverted Index Shards
    participant Rank as Ranking Service
    participant DocStore as Document Store

    C->>SS: GET /search?q=quick+delivery
    SS->>SS: Parse + tokenize query<br/>→ ["quick", "delivery"]

    par Query all relevant shards in parallel
        SS->>InvIdx: Lookup "quick" (shard A)
        InvIdx-->>SS: [doc1, doc2, doc5, ...]
    and
        SS->>InvIdx: Lookup "delivery" (shard B)
        InvIdx-->>SS: [doc2, doc9, ...]
    end

    SS->>SS: Merge/intersect posting lists<br/>(AND logic, or union for OR queries)
    SS->>Rank: Score candidates<br/>(TF-IDF/BM25 + other signals)
    Rank-->>SS: Ranked doc_ids with scores

    SS->>DocStore: Fetch document details<br/>for top N results
    DocStore-->>SS: Hydrated results
    SS-->>C: Return ranked results
```

---

## 6. Relevance Ranking (TF-IDF / BM25)

```mermaid
flowchart TB
    A["Candidate document<br/>matches query terms"] --> B["Term Frequency (TF)<br/>How often does the term<br/>appear in THIS document?"]
    A --> C["Inverse Document Frequency (IDF)<br/>How RARE is this term<br/>across ALL documents?<br/>(common words score lower)"]
    A --> D["Field weighting<br/>(match in title > match in body)"]
    A --> E["Freshness/recency boost<br/>(optional, use-case dependent)"]
    A --> F["Popularity/engagement signal<br/>(click-through rate, if available)"]

    B & C & D & E & F --> G["BM25 Scoring Function<br/>(TF-IDF variant with<br/>saturation + length normalization)"]
    G --> H["Final relevance score<br/>per document"]
    H --> I["Sort descending,<br/>return top N"]
```

*BM25 improves on naive TF-IDF by preventing a document from scoring disproportionately high just by repeating a term many times (diminishing returns via saturation) and normalizing for document length so long documents don't unfairly dominate.*

---

## 7. Autocomplete — Trie-Based Prefix Matching

```mermaid
flowchart TB
    A["Trie structure built from<br/>popular queries/titles"] --> B["Root"]
    B --> C["'q'"]
    C --> D["'qu'"]
    D --> E["'qui'"]
    E --> F["'quic'"]
    F --> G["'quick' (complete word,<br/>marked with popularity score)"]
    G --> H["'quicken'"]
    G --> I["'quickly'"]

    J["User types 'qui'"] --> K["Walk trie to node 'qui'"]
    K --> L["Collect all completions<br/>in subtree below 'qui'"]
    L --> M["Sort by popularity score"]
    M --> N["Return top 5-10 suggestions"]
```

```mermaid
sequenceDiagram
    participant C as Client
    participant AC as Autocomplete Service
    participant Trie as Trie/FST Index
    participant Pop as Popularity Store

    loop On every keystroke
        C->>AC: GET /autocomplete?prefix=qui
        AC->>Trie: Traverse to prefix node "qui"
        Trie-->>AC: Subtree of possible completions
        AC->>Pop: Get popularity scores for candidates
        Pop-->>AC: Scores
        AC->>AC: Rank by popularity, truncate to top 10
        AC-->>C: Suggestions: [quick, quickly, quicken, ...]
    end
```

**Why a trie/FST instead of the inverted index for autocomplete:** Autocomplete needs prefix matching at extremely low latency on every keystroke — a trie gives O(prefix_length) lookup regardless of dataset size, far faster than any inverted-index-based text search for this specific access pattern. An FST (finite state transducer) is often used in production as a more memory-compact variant of the same idea.

---

## 8. Typo Tolerance (Fuzzy Matching)

```mermaid
flowchart TB
    A["User query: 'qwick'<br/>(typo for 'quick')"] --> B{"Fuzzy Matching Strategy"}
    B --> C["Edit Distance<br/>(Levenshtein) within threshold"]
    B --> D["N-gram based matching<br/>(index 3-character shingles)"]

    C --> C1["Compare 'qwick' against<br/>index terms within edit distance 1-2"]
    C1 --> C2["'quick' is edit-distance 1 away<br/>(one substitution) — matches"]

    D --> D1["'qwick' → shingles: qwi, wic, ick"]
    D1 --> D2["Find index terms sharing<br/>most shingles — 'quick' shares 'ick'"]

    C2 & D2 --> E["Include as fuzzy-matched candidate,<br/>ranked lower than exact matches"]
```

---

## 9. Index Sharding Strategy

```mermaid
flowchart TB
    A["Inverted Index<br/>(too large for one machine)"] --> B{"Sharding Strategy"}
    B --> C["Term-based partitioning<br/>(each shard owns a subset of terms)"]
    B --> D["Document-based partitioning<br/>(each shard has full index<br/>for a subset of documents)"]

    C --> C1["Query for term X hits<br/>exactly ONE shard"]
    C --> C2["Downside: some terms are<br/>far more popular — shard skew"]

    D --> D1["Query must fan out to<br/>ALL shards (each may have<br/>matching docs), then merge"]
    D --> D2["More balanced load,<br/>but more network overhead per query"]

    E["Most production search engines<br/>(Elasticsearch, Solr) use<br/>document-based partitioning"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Search Engine HLD))
    Ingestion Service
      Receives new/updated content
      Triggers async indexing
    Indexer Workers
      Tokenization, stemming
      Inverted index construction
      Trie index maintenance
    Search Service
      Query parsing
      Multi-shard fan-out
      Result merging
    Ranking Service
      TF-IDF/BM25 scoring
      Multi-signal relevance
    Autocomplete Service
      Trie/FST prefix lookup
      Popularity-based ranking
    Document Store
      Raw content + metadata
      Hydration for final results
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Core search structure | Inverted index | Transforms full-text search from O(N documents) scanning into O(1) term lookup + merge |
| Autocomplete structure | Separate trie/FST index | Prefix matching has fundamentally different performance characteristics than full-text search; needs its own optimized structure |
| Ranking algorithm | BM25 (TF-IDF variant) | Handles term saturation and document length normalization better than naive term-frequency counting |
| Index sharding | Document-based partitioning | More balanced load distribution than term-based, despite requiring fan-out to all shards per query |
| Typo tolerance | Edit distance + n-gram matching | Combines precision (edit distance) with query speed (n-gram pre-filtering) rather than relying on one technique alone |
| Indexing latency | Async pipeline via Kafka | Decouples content ingestion from the (relatively) expensive tokenization/indexing work, allowing independent scaling |

---

## 12. Bottlenecks & Scaling Considerations

- **Query fan-out latency** — document-based sharding means every query hits every shard; the overall query latency is bounded by the slowest shard's response time, requiring careful timeout/tail-latency management (e.g., "good enough" partial results if one shard is slow).
- **Index update latency vs read load** — frequent re-indexing (for freshness) competes for resources with serving search queries; production systems often separate indexing and query-serving nodes, with periodic index snapshot handoff.
- **Autocomplete cache-ability** — since many users type overlapping prefixes ("a", "ap", "app"...), aggressive caching of common prefix results dramatically reduces trie traversal load — worth layering a cache in front of the trie for the most common prefixes.
- **Popularity score staleness** — autocomplete ranking based on historical query popularity can lag behind rapidly emerging trends (breaking news); needs a real-time signal blended with the historical baseline for trending terms.
- **Hot shard from popular terms** — even with document-based sharding, extremely common query terms can create uneven load across shards during query time; monitor and consider dedicated caching for top queries.
- **Relevance tuning is never "done"** — unlike most infra problems, ranking quality requires continuous evaluation (A/B testing, click-through analysis) rather than a fixed correct architecture — the system needs to be designed for iterative relevance tuning, not just raw performance.
- **Large-scale reindexing** — full corpus re-indexing (e.g., after a ranking algorithm change) at billions of documents is a massive batch operation; needs to run without disrupting live query serving, typically via a blue-green index swap.
