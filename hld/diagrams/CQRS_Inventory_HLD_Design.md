# Design a CQRS-Based Architecture for a High-Write, High-Read E-commerce Inventory System — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Handle extremely high-frequency inventory WRITES (stock decrements on every sale, restocks, adjustments)
- Serve extremely high-frequency inventory READS (product pages checking "in stock?", search filters, browsing) with very different query patterns than the writes
- Support complex read queries (e.g., "show all in-stock items in category X, sorted by price") that don't map naturally to the write model's structure
- Maintain acceptable consistency between what's written and what's eventually readable

### Non-Functional Requirements
- **Independent scaling of reads and writes:** Read and write traffic patterns and volumes are fundamentally different at this scale — they shouldn't be forced to share the same infrastructure/scaling constraints
- **Write correctness (paramount):** Inventory decrements must never allow overselling — this is a hard business correctness requirement
- **Read performance:** Product browsing/search must be fast, even though the underlying write model isn't optimized for these complex read patterns
- **Acceptable read staleness:** A brief delay between a write and its visibility in read queries is generally tolerable for this domain (unlike, say, a bank balance)

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Inventory writes/sec (sales events) | Thousands, spiking during flash sales |
| Inventory reads/sec (browsing/search) | Hundreds of thousands — orders of magnitude more than writes |
| Read/write ratio | Extremely read-heavy, but WRITES are correctness-critical |
| Read staleness tolerance | Seconds typically acceptable |

---

## 2. The Core Principle — Separate Models for Reading and Writing

```mermaid
flowchart TB
    A["Traditional CRUD approach:<br/>ONE data model serves BOTH<br/>reads and writes — the same<br/>database table/schema handles<br/>'decrement stock on sale'<br/>AND 'show me all in-stock<br/>red shoes under $50,<br/>sorted by popularity'"] --> A1["Problem: these are<br/>FUNDAMENTALLY DIFFERENT access<br/>patterns — writes need strict<br/>correctness on a SINGLE item's<br/>quantity; reads need flexible,<br/>fast querying across MANY<br/>items with rich filtering.<br/>Optimizing a single schema<br/>for BOTH is a genuine<br/>engineering compromise"]

    B["CQRS (Command Query<br/>Responsibility Segregation):<br/>maintain COMPLETELY SEPARATE<br/>models — a WRITE MODEL<br/>optimized purely for correct,<br/>fast inventory updates, and<br/>a READ MODEL optimized purely<br/>for fast, flexible querying —<br/>kept in sync via an<br/>asynchronous propagation<br/>mechanism"] --> B1["Each side can be<br/>independently designed,<br/>scaled, and even use<br/>ENTIRELY DIFFERENT storage<br/>technologies suited to its<br/>specific access pattern"]
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph WriteSide["Write Side (Command Model)"]
        CommandAPI["Command API<br/>(DecrementStock,<br/>RestockItem)"]
        WriteDB[("Write Database<br/>— strongly consistent,<br/>optimized for correct<br/>single-item updates")]
    end

    subgraph Sync["Synchronization Layer"]
        Kafka["Kafka<br/>(InventoryChanged events —<br/>same CDC-style pattern<br/>as the CDC Pipeline design)"]
        Projector["Read Model Projector"]
    end

    subgraph ReadSide["Read Side (Query Model)"]
        SearchIndex[("Search Index<br/>— Elasticsearch,<br/>optimized for filtering/sorting")]
        ReadCache[("Denormalized Read Cache<br/>— fast product page lookups")]
    end

    QueryAPI["Query API<br/>(browse, search, filter)"]

    CommandAPI --> WriteDB
    WriteDB --> Kafka
    Kafka --> Projector
    Projector --> SearchIndex
    Projector --> ReadCache

    QueryAPI --> SearchIndex
    QueryAPI --> ReadCache
```

**Key idea:** The Write Database is a small, simple, strongly-consistent store optimized for exactly one thing — correctly updating a single item's quantity without overselling. The Read Side is entirely separate, denormalized, and optimized for the genuinely different query patterns real users need (browsing, filtering, search) — connected only by an asynchronous event stream, the same CDC-style mechanism covered in the dedicated CDC Pipeline design.

---

## 4. Data Model

```mermaid
erDiagram
    WRITE_MODEL_INVENTORY {
        string sku PK
        int available_quantity
        int reserved_quantity
        int version "optimistic locking"
    }
    READ_MODEL_PRODUCT_VIEW {
        string sku PK
        string product_name
        float price
        string category
        bool in_stock
        int display_quantity "approximate,<br/>e.g. 'only 3 left!'"
        map search_facets "denormalized for<br/>fast filtering"
    }
```

**Why the read model is deliberately DENORMALIZED and even slightly APPROXIMATE:** The write model needs an EXACT quantity for correctness (never oversell). The read model, however, only needs a reasonably fresh "in stock" boolean and a display quantity that might legitimately be a few seconds stale ("only 3 left!" might actually be 2 by the time you click, and that's an acceptable, common e-commerce UX pattern) — this deliberate relaxation of consistency on the read side is precisely what allows it to be optimized purely for query speed and flexibility.

---

## 5. Write Flow (Command Side) — Detailed Sequence

```mermaid
sequenceDiagram
    participant Client as Checkout Service
    participant CommandAPI as Command API
    participant WriteDB as Write Database
    participant Kafka as Event Stream

    Client->>CommandAPI: DecrementStock<br/>{sku: ABC123, quantity: 1}

    CommandAPI->>WriteDB: Conditional UPDATE:<br/>available_quantity -= 1<br/>WHERE sku=ABC123 AND<br/>available_quantity >= 1<br/>(same atomic conditional<br/>update pattern as the<br/>E-commerce Checkout design's<br/>overselling prevention)

    alt Sufficient stock
        WriteDB-->>CommandAPI: Success, new quantity: 41
        CommandAPI->>Kafka: Emit InventoryChanged event<br/>{sku: ABC123, new_quantity: 41}
        CommandAPI-->>Client: Success
    else Insufficient stock
        WriteDB-->>CommandAPI: Failed — out of stock
        CommandAPI-->>Client: 409 Conflict
    end
```

---

## 6. Read Model Projection Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Kafka as Event Stream
    participant Projector as Read Model Projector
    participant SearchIndex as Search Index
    participant ReadCache as Read Cache

    Kafka->>Projector: Consume InventoryChanged<br/>{sku: ABC123, new_quantity: 41}

    Projector->>Projector: Transform into READ-optimized<br/>shape: {sku, in_stock: true,<br/>display_quantity: "40+" —<br/>note the DELIBERATE<br/>imprecision for display purposes}

    par Update both read stores
        Projector->>SearchIndex: Update document<br/>(enables fast filtered search:<br/>"in-stock items in category X")
    and
        Projector->>ReadCache: Update cached product view<br/>(enables fast individual<br/>product page loads)
    end

    Note over SearchIndex,ReadCache: Read side is now updated —<br/>typically within a second or<br/>two of the original write,<br/>an ACCEPTABLE staleness<br/>window for this domain
```

---

## 7. Query Flow (Read Side) — Detailed Sequence

```mermaid
sequenceDiagram
    participant User as Shopper
    participant QueryAPI as Query API
    participant SearchIndex as Search Index

    User->>QueryAPI: Browse: "red shoes,<br/>in stock, under $50,<br/>sorted by popularity"

    QueryAPI->>SearchIndex: Complex filtered/sorted query<br/>(the read model is SPECIFICALLY<br/>structured to make this kind<br/>of query fast — something<br/>the write model was never<br/>designed to support efficiently)

    SearchIndex-->>QueryAPI: Ranked, filtered results

    QueryAPI-->>User: Display results
```

**Why this query would be awkward and slow against the write model directly:** The write model's schema is optimized for "atomically update ONE item's quantity correctly" — it has no denormalized category/price/popularity indexing, no search-optimized structure. Forcing this kind of rich, multi-dimensional filtered query against a schema designed for single-item transactional correctness would require expensive joins/scans that a purpose-built search index avoids entirely.

---

## 8. Handling the Consistency Gap (Read-Your-Writes for Checkout)

```mermaid
flowchart TB
    A["User just completed checkout<br/>(write succeeded), immediately<br/>navigates back to the product<br/>page"] --> B{"Does the READ side<br/>reflect the decrement yet?"}

    B -- "Not yet<br/>(propagation still in-flight)" --> C["Read model briefly shows<br/>the OLD quantity — generally<br/>harmless for browsing, but<br/>could this cause a PROBLEM?"]

    C --> D["Critical distinction: the<br/>WRITE model already correctly<br/>enforced no-overselling at<br/>the moment of purchase —<br/>the read model being briefly<br/>stale doesn't risk ANOTHER<br/>oversold unit, it just<br/>displays slightly outdated<br/>information momentarily"]

    E["For genuinely critical<br/>reads (e.g., the checkout<br/>flow's OWN stock check<br/>immediately before purchase),<br/>route directly to the WRITE<br/>model instead of the read<br/>model — CQRS doesn't mean<br/>EVERY read must go through<br/>the eventually-consistent<br/>read side"] -.-> D
```

**Why this hybrid read-routing matters:** A well-designed CQRS system doesn't dogmatically route ALL reads through the read model — for the specific, narrow case where a read feeds directly into a correctness-critical decision (like the actual stock check during checkout), querying the strongly-consistent write model directly is the right choice, while the vast majority of browsing/search reads happily use the faster, eventually-consistent read model.

---

## 9. Handling Read Model Rebuild (Disaster Recovery / Schema Changes)

```mermaid
sequenceDiagram
    participant Ops as Operations
    participant WriteDB as Write Database<br/>(source of truth)
    participant Kafka as Event Stream
    participant Projector as Read Model Projector
    participant NewSearchIndex as New Search Index<br/>(rebuilt from scratch)

    Note over Ops: Read model needs full<br/>rebuild (e.g., search index<br/>corruption, or a schema<br/>change requiring reindexing)

    Ops->>WriteDB: Full scan of current<br/>write-model state<br/>(bootstrap, similar to the<br/>CDC Pipeline design's<br/>initial snapshot phase)

    Ops->>Projector: Replay full current state<br/>as synthetic events

    Projector->>NewSearchIndex: Rebuild complete read<br/>model from scratch

    Ops->>Kafka: THEN resume consuming<br/>live events from the<br/>point snapshot began<br/>(same snapshot-then-tail<br/>pattern as CDC bootstrapping)
```

**Why this rebuild capability is a genuine advantage of CQRS's architectural separation:** Because the write model remains the unambiguous source of truth and the read model is EXPLICITLY understood to be a derived, rebuildable projection, the system can always recover from read-side corruption or evolve the read model's schema by simply reprocessing from the write model — a capability that's much harder to achieve cleanly in a single-shared-model architecture where "source of truth" and "query interface" are the same tightly-coupled thing.

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((CQRS Inventory HLD))
    Command API
      Write-side entry point
      Enforces correctness rules
    Write Database
      Strongly consistent
      Optimized for atomic updates
    Event Stream
      CDC-style propagation
      Decouples write and read sides
    Read Model Projector
      Transforms events to read shape
      Denormalizes for query speed
    Search Index and Read Cache
      Optimized for browsing/filtering
      Eventually consistent, rebuildable
    Query API
      Read-side entry point
      Can bypass to write model when needed
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Architecture pattern | CQRS — fully separate write and read models | Read and write access patterns are fundamentally different; forcing one shared schema to serve both is a real engineering compromise this pattern eliminates |
| Write model design | Simple, strongly consistent, optimized purely for correctness | The one non-negotiable requirement — never oversell — deserves a purpose-built, uncompromised model |
| Read model design | Denormalized, search-optimized, eventually consistent | Optimized purely for the actual query patterns users need (browse, filter, search), independent of write-side constraints |
| Synchronization | Async event stream (CDC-style) | Decouples the two sides' scaling and technology choices entirely, at the cost of a brief, generally acceptable staleness window |
| Consistency-critical reads | Selectively bypass to the write model | CQRS doesn't mandate ALL reads go through the eventually-consistent side — genuinely critical reads can and should query the source of truth directly |
| Read model recovery | Fully rebuildable from write-model replay | The read model's explicit status as a derived projection (not source of truth) enables clean disaster recovery and schema evolution |

---

## 12. Bottlenecks & Scaling Considerations

- **Propagation lag under write bursts** — during a flash sale with extremely high write volume, the event stream and projector may experience increased lag, widening the read-model staleness window beyond its normal few-seconds baseline; needs monitoring and independently scalable projector capacity, same operational concern as the CDC Pipeline design.
- **Read model rebuild time at scale** — for a catalog with millions of SKUs, a full read-model rebuild (Section 9) can take substantial time; needs efficient batch processing and ideally the ability to rebuild incrementally/in parallel rather than as a single long-running sequential job.
- **Dual-write consistency risk** — the write model update and the event emission (Section 5) must be atomically coupled (both happen, or neither does) — this is the same "transactional outbox" concern noted in the Distributed Transaction Saga design's dual-write problem, requiring careful implementation to avoid a write succeeding while its corresponding event is lost.
- **Multiple read models for different use cases** — beyond the single search-index example shown here, a real system might need SEVERAL distinct read models (e.g., one for search, one for a recommendation engine, one for analytics) all fed from the SAME write-side event stream — each optimized independently for its own specific query pattern, extending the core CQRS principle to multiple specialized read sides rather than just one.
- **Testing consistency guarantees** — because the system deliberately allows brief read-side staleness, testing must clearly distinguish between ACCEPTABLE staleness (browsing shows slightly outdated quantity) and UNACCEPTABLE inconsistency (the write model itself allowing overselling) — conflating these two very different correctness properties during testing/design review is a common and serious mistake.
- **Operational complexity increase** — CQRS introduces genuinely more moving parts (two models, a synchronization pipeline, monitoring for propagation lag) compared to a simple single-model CRUD system; this architectural complexity is a deliberate, worthwhile tradeoff specifically when read and write patterns are genuinely divergent at meaningful scale — applying CQRS to a system without this genuine access-pattern mismatch would add complexity without corresponding benefit.
