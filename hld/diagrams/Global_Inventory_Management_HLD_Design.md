# Design a Global Inventory Management System for a Retailer (Online + In-Store Sync) — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Track inventory levels across thousands of physical stores AND online fulfillment centers simultaneously
- Keep online product availability synchronized with real, physical in-store stock as it changes
- Support "buy online, pick up in store" (BOPIS) and "ship from store" fulfillment models
- Support inventory transfers between locations and periodic physical stock reconciliation

### Non-Functional Requirements
- **Correctness for sellable inventory:** Never show an item as available online if it's genuinely out of stock everywhere — this directly causes cancelled orders and customer trust damage
- **Near-real-time sync between physical and digital:** A sale at a physical register should reflect in online availability within seconds, not hours
- **Resilience to store-level connectivity issues:** Individual store networks can be unreliable; the system must handle this gracefully without losing sales or corrupting inventory counts
- **Scale:** Thousands of stores, millions of SKUs, tracking inventory across all location combinations

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Physical stores | Thousands |
| SKUs tracked | Millions (varying subsets per store) |
| Inventory update events/sec (platform-wide) | Thousands (POS sales, online orders, restocks, transfers) |
| Sync latency target (in-store sale → online reflection) | Seconds |

---

## 2. The Core Challenge — Reconciling Physical Reality With Digital Representation

```mermaid
flowchart TB
    A["Physical inventory reality:<br/>a customer picks up an item<br/>and walks to a register —<br/>this is the GROUND TRUTH<br/>event that must eventually<br/>be reflected everywhere"] --> B["The digital system's job:<br/>capture this event as close<br/>to real-time as possible,<br/>and propagate it to EVERY<br/>place that shows 'is this<br/>item available' — online<br/>storefront, other stores'<br/>transfer-eligibility checks,<br/>fulfillment center allocation<br/>decisions"]

    B --> C["The core difficulty: physical<br/>stores have LESS reliable<br/>connectivity and infrastructure<br/>than a centralized data center<br/>— the system must be resilient<br/>to a store temporarily losing<br/>connection WITHOUT losing<br/>sales data or corrupting<br/>counts once reconnected"]
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph Stores["Physical Stores (thousands)"]
        POS1["POS Terminal<br/>(Store 1)"]
        LocalCache1["Local Store Server<br/>(offline-capable buffer)"]
    end

    subgraph Online["Online Channel"]
        WebApp["E-commerce Storefront"]
        FulfillmentCenter["Fulfillment Center<br/>Inventory Systems"]
    end

    subgraph CentralSystem["Central Inventory Platform"]
        IngestionAPI["Inventory Event<br/>Ingestion API"]
        Kafka["Kafka<br/>(inventory change stream)"]
        InventoryLedger[("Central Inventory Ledger<br/>— per-SKU, per-location<br/>authoritative counts")]
        AvailabilityProjector["Availability Projector<br/>(same CQRS-style pattern<br/>as the Inventory CQRS design)"]
    end

    subgraph ReadSide["Read-Optimized Views"]
        OnlineAvailability[("Online Availability View<br/>— aggregated, sellable stock")]
    end

    POS1 --> LocalCache1
    LocalCache1 -->|"sync when connected,<br/>buffer when offline"| IngestionAPI

    WebApp --> OnlineAvailability
    FulfillmentCenter --> IngestionAPI

    IngestionAPI --> Kafka
    Kafka --> InventoryLedger
    Kafka --> AvailabilityProjector
    AvailabilityProjector --> OnlineAvailability
```

**Key idea:** This combines two patterns established in earlier designs — the offline-resilient local buffering from the Mobile Offline Caching design (applied to store-level connectivity instead of individual mobile devices) and the CQRS-style write/read separation from the Inventory CQRS design (the central ledger is the correctness-critical write side; the online availability view is the fast, denormalized read side).

---

## 4. Data Model

```mermaid
erDiagram
    LOCATION {
        string location_id PK
        string type "store/fulfillment_center"
        string connectivity_status
    }
    SKU_LOCATION_INVENTORY {
        string sku FK
        string location_id FK
        int on_hand_quantity
        int reserved_quantity
        int version "optimistic locking"
    }
    INVENTORY_EVENT {
        string event_id PK
        string sku FK
        string location_id FK
        string event_type "sale/restock/transfer/adjustment"
        int quantity_delta
        timestamp occurred_at "when it PHYSICALLY happened"
        timestamp synced_at "when it reached the central system"
    }
```

**Why both `occurred_at` and `synced_at` are tracked separately:** Given that store connectivity can be unreliable, an event might PHYSICALLY happen at 2:00pm but not reach the central system until 2:15pm due to a temporary network issue — distinguishing these two timestamps is essential for correctly ordering events (by when they actually happened) rather than by when they happened to arrive, the same event-time vs processing-time distinction covered in depth in the Stream Processing Fraud Detection design.

---

## 5. Store-Level Sale Event Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant POS as POS Terminal
    participant LocalServer as Local Store Server<br/>(offline-capable)
    participant IngestionAPI as Central Ingestion API
    participant Kafka as Event Stream
    participant Ledger as Central Inventory Ledger

    POS->>LocalServer: Sale: SKU-123, qty=1<br/>(recorded IMMEDIATELY,<br/>locally — the sale<br/>completes regardless of<br/>connectivity)

    LocalServer->>LocalServer: Buffer event locally<br/>(same outbox-queue pattern<br/>as the Mobile Offline<br/>Caching design)

    alt Store has connectivity
        LocalServer->>IngestionAPI: Sync event
        IngestionAPI->>Kafka: Publish InventoryChanged
        Kafka->>Ledger: Update: on_hand_quantity -= 1<br/>for SKU-123 at this location
    else Store temporarily offline
        Note over LocalServer: Event remains safely<br/>buffered locally
        Note over LocalServer: Connectivity restored later
        LocalServer->>IngestionAPI: Sync ALL buffered events<br/>(in original order,<br/>with original occurred_at<br/>timestamps preserved)
        IngestionAPI->>Kafka: Publish events
        Kafka->>Ledger: Apply updates, correctly<br/>ordered by occurred_at
    end
```

**Why the physical sale must NEVER be blocked by connectivity:** A customer at a physical register cannot be told "please wait, we're checking with headquarters" — the sale must complete locally and instantly regardless of network status, with synchronization happening asynchronously and resiliently whenever connectivity allows, exactly mirroring why the Mobile Offline Caching design treats local writes as always-available and network sync as a background concern.

---

## 6. Multi-Location Availability Aggregation

```mermaid
flowchart TB
    A["Online storefront shows:<br/>'SKU-123: In Stock'"] --> B["What does 'in stock' actually<br/>mean when this SKU exists<br/>across THOUSANDS of<br/>locations?"]

    B --> C{"Aggregation Strategy"}
    C --> D["Simple sum: total available<br/>= SUM of on_hand_quantity<br/>across ALL locations"]
    D --> D1["Risk: an item might show<br/>'in stock' based on units<br/>scattered across many stores<br/>that AREN'T eligible for<br/>online fulfillment (e.g.,<br/>reserved for in-store-only<br/>display, or in a location<br/>too far for reasonable<br/>shipping)"]

    C --> E["Fulfillment-eligible sum:<br/>only count quantity at<br/>locations ACTUALLY capable<br/>of fulfilling an online<br/>order for this specific<br/>customer (shipping distance,<br/>store fulfillment capability<br/>flags)"]

    F["This design uses the<br/>fulfillment-eligible approach —<br/>'in stock' must mean<br/>'genuinely orderable by THIS<br/>customer,' not just 'exists<br/>somewhere in the company'"] -.-> E
```

---

## 7. Reservation During Checkout (Preventing Overselling Across Locations)

```mermaid
sequenceDiagram
    participant Customer as Online Customer
    participant CheckoutSvc as Checkout Service
    participant Ledger as Central Inventory Ledger
    participant FulfillmentEngine as Fulfillment Allocation Engine

    Customer->>CheckoutSvc: Place order for SKU-123

    CheckoutSvc->>FulfillmentEngine: Determine best fulfillment<br/>location (nearest store/FC<br/>with available stock)
    FulfillmentEngine->>Ledger: Reserve 1 unit at<br/>chosen location<br/>(same atomic conditional<br/>update pattern as the<br/>E-commerce Checkout design's<br/>overselling prevention)

    alt Reservation succeeds
        Ledger-->>FulfillmentEngine: Reserved
        FulfillmentEngine-->>CheckoutSvc: Confirmed, fulfilling<br/>from Store #4521
        CheckoutSvc-->>Customer: Order confirmed
    else Reservation fails<br/>(another customer/in-store<br/>sale claimed it first)
        Ledger-->>FulfillmentEngine: Insufficient stock<br/>at this location
        FulfillmentEngine->>FulfillmentEngine: Try NEXT-best<br/>fulfillment location
    end
```

**Why this directly reuses the E-commerce Checkout design's reservation pattern:** The same fundamental overselling-prevention challenge exists here — but with an added dimension: the "stock" being reserved might get claimed not just by another ONLINE customer, but by an in-store customer physically buying the item at that exact moment, making the atomic conditional update pattern even more essential given the additional physical-world race condition.

---

## 8. Handling Physical Inventory Reconciliation (Cycle Counts)

```mermaid
sequenceDiagram
    participant StoreStaff as Store Staff<br/>(periodic physical count)
    participant LocalServer as Local Store Server
    participant Ledger as Central Inventory Ledger
    participant Discrepancy as Discrepancy Handler

    StoreStaff->>LocalServer: Physical count: SKU-123<br/>= 47 units actually on shelf

    LocalServer->>Ledger: Compare against SYSTEM'S<br/>recorded count for SKU-123<br/>at this location

    alt Counts match
        Ledger-->>LocalServer: Confirmed accurate
    else Discrepancy found<br/>(system says 52, physical<br/>count says 47)
        Ledger->>Discrepancy: Flag discrepancy:<br/>5 units unaccounted for<br/>(possible theft, damage,<br/>unrecorded sale, data<br/>sync issue)
        Discrepancy->>Ledger: Adjust ledger to match<br/>PHYSICAL reality (47) —<br/>physical count is the<br/>ultimate ground truth
        Discrepancy->>Discrepancy: Log for loss-prevention/<br/>audit investigation
    end
```

**Why physical count is always treated as ground truth over the system's records:** Despite all the careful event-tracking infrastructure, the actual physical item on a shelf is unambiguous reality — any accumulated small errors (edge cases in sync timing, occasional lost events despite best efforts) must periodically be corrected against this ground truth, which is precisely why regular cycle counts remain an essential operational process even with a well-engineered digital tracking system.

---

## 9. Ship-From-Store Fulfillment Flow

```mermaid
flowchart TB
    A["Online order needs<br/>fulfillment, nearest<br/>available stock happens<br/>to be at a physical STORE<br/>rather than a dedicated<br/>fulfillment center"] --> B["Fulfillment Engine allocates<br/>order to that store"]
    B --> C["Store receives fulfillment<br/>task via Local Store Server"]
    C --> D["Store staff picks the item<br/>from shelf, marks as<br/>SHIPPED in the local system"]
    D --> E["Inventory event propagates:<br/>on_hand_quantity -= 1 at<br/>this store — SAME event<br/>pipeline as an in-store<br/>sale, just a different<br/>event_type"]

    F["This unifies in-store<br/>sales and ship-from-store<br/>fulfillment through the<br/>SAME underlying inventory<br/>event infrastructure —<br/>both are simply 'this<br/>store's on-hand quantity<br/>decreased,' regardless of<br/>WHY"] -.-> E
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Global Inventory Management HLD))
    Local Store Server
      Offline-capable buffering
      Never blocks physical sales
    Central Inventory Ledger
      Per-location authoritative counts
      Correctness-critical write side
    Availability Projector
      CQRS-style read model
      Fulfillment-eligible aggregation
    Fulfillment Allocation Engine
      Best-location selection
      Atomic reservation
    Reconciliation Process
      Physical count as ground truth
      Discrepancy flagging and audit
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Store connectivity handling | Local offline-capable buffering | Physical sales must never be blocked by network issues; mirrors the Mobile Offline Caching design's core philosophy |
| Availability calculation | Fulfillment-eligible aggregation, not simple total sum | "In stock" must genuinely mean orderable by the specific customer, not just existing somewhere in the company |
| Overselling prevention | Atomic conditional reservation at allocation time | Same E-commerce Checkout design pattern, extended to handle the additional race condition of competing in-store physical sales |
| Event ordering | Physical occurred_at timestamp, not sync arrival time | Correctly reconstructs the true sequence of events despite variable store connectivity delays |
| Reconciliation | Periodic physical count as ground truth override | Accumulated small digital-tracking errors are inevitable at this scale; physical reality is the ultimate correctness anchor |
| Architecture pattern | CQRS-style separation (correctness-critical ledger + fast read view) | Matches the fundamentally different needs of accurate inventory tracking versus fast customer-facing availability display |

---

## 12. Bottlenecks & Scaling Considerations

- **Store server buffer capacity during extended outages** — a store with connectivity issues lasting hours or days accumulates a growing backlog of unsynced events; local buffering needs sufficient capacity and the sync process needs efficient batch-catch-up capability, same fundamental concern as the Mobile Offline Caching design's outbox growth handling.
- **Cross-location race conditions at scale** — with thousands of stores and online customers simultaneously competing for the same limited stock of a popular item, the atomic reservation mechanism must handle very high contention on hot SKUs, connecting to the same hot-key mitigation principles covered in the dedicated Hot Key Mitigation design.
- **Fulfillment allocation optimization complexity** — determining the "best" fulfillment location isn't just about proximity; it involves shipping cost, delivery speed, store fulfillment capacity, and inventory balancing goals (e.g., avoiding depleting a store's own walk-in stock for online orders) — this allocation logic can become a genuinely complex optimization problem at scale, beyond simple nearest-location selection.
- **Sync latency variability across store infrastructure quality** — stores in different regions/countries may have meaningfully different baseline connectivity reliability; the system's monitoring must account for this variance rather than applying uniform sync-latency alerting thresholds across a genuinely heterogeneous store network.
- **Discrepancy investigation workflow scaling** — as flagged discrepancies accumulate across thousands of stores, the loss-prevention/audit investigation process needs its own prioritization and workflow tooling (similar in spirit to the Content Moderation design's review queue) to handle this at scale rather than treating each discrepancy as an isolated manual investigation.
- **Seasonal/promotional demand spikes** — major sales events create both massive transaction volume (stressing the event ingestion pipeline) AND intense competition for limited popular-item stock (stressing the reservation/allocation system) simultaneously — this connects directly to the same launch-readiness and stampede-prevention considerations covered in the Cache Warming design, applied to inventory infrastructure rather than caching infrastructure.
- **Multi-national regulatory and tax complexity** — a genuinely global retailer faces varying regulatory requirements (customs, taxes, region-specific fulfillment rules) that add business logic complexity to the fulfillment allocation engine beyond the pure inventory-tracking challenge covered in this design's core architecture.
