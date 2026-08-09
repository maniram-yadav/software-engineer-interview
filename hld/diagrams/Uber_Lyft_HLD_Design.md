# Design Uber/Lyft — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Riders can request a ride; system matches them with a nearby available driver
- Real-time location tracking of driver en route and during trip
- Dynamic/surge pricing based on supply-demand imbalance
- Trip lifecycle management: requested → matched → en route → in progress → completed
- Payment processing on trip completion
- Rating system for drivers and riders

### Non-Functional Requirements
- **Scale:** ~5M drivers, ~100M riders, millions of concurrent active trips globally
- **Low latency matching:** Rider should get matched with a driver within seconds
- **Location update frequency:** Driver apps ping location every 3-4 seconds
- **Geographic partitioning:** Matching only matters locally — no need for global coordination per request
- **High availability:** A regional outage shouldn't take down the whole platform (city-level isolation)

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Active drivers (peak, global) | ~1M concurrently online |
| Location updates/sec | ~1M drivers × (1 update / 4 sec) ≈ 250,000/sec |
| Ride requests/sec (peak) | ~10,000/sec |
| Matching latency target | < 3-5 seconds |
| Geospatial index size | Millions of active driver locations, constantly updating |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    RiderApp["Rider App"]
    DriverApp["Driver App"]

    Gateway["API Gateway"]

    subgraph Core["Core Services"]
        LocationSvc["Location Ingestion Service"]
        MatchingSvc["Matching Service"]
        TripSvc["Trip Management Service"]
        PricingSvc["Pricing/Surge Service"]
        PaymentSvc["Payment Service"]
    end

    subgraph Geo["Geospatial Layer"]
        GeoIndex[("Geospatial Index<br/>(Quadtree/Geohash in Redis)")]
        DriverLocStore[("Driver Location Store<br/>(in-memory, TTL-based)")]
    end

    subgraph Storage["Storage Layer"]
        TripDB[("Trip DB<br/>(trip records, status)")]
        UserDB[("User/Driver Profile DB")]
        PricingCache[("Surge Pricing Cache<br/>(per geo-cell)")]
    end

    RiderApp --> Gateway
    DriverApp --> Gateway

    DriverApp -->|"Location ping<br/>every 3-4s"| Gateway
    Gateway --> LocationSvc
    LocationSvc --> GeoIndex
    LocationSvc --> DriverLocStore

    RiderApp -->|"Request ride"| Gateway
    Gateway --> MatchingSvc
    MatchingSvc --> GeoIndex
    MatchingSvc --> PricingSvc
    PricingSvc --> PricingCache
    MatchingSvc --> TripSvc

    TripSvc --> TripDB
    TripSvc --> PaymentSvc
    Gateway --> UserDB
```

**Key idea:** The entire matching problem is fundamentally **geospatial** — the system needs to answer "which drivers are within N km of this rider, right now?" extremely fast, at massive update frequency. This drives almost every architectural decision: an in-memory geospatial index refreshed continuously by a firehose of location pings.

---

## 3. Data Model

```mermaid
erDiagram
    RIDER ||--o{ TRIP : requests
    DRIVER ||--o{ TRIP : accepts
    DRIVER ||--o{ LOCATION_PING : sends
    TRIP ||--o{ PAYMENT : "settled by"
    TRIP ||--o{ RATING : "rated via"

    RIDER {
        string rider_id PK
        string name
        string payment_method_id
    }
    DRIVER {
        string driver_id PK
        string name
        string vehicle_info
        string status "available/busy/offline"
        float rating
    }
    LOCATION_PING {
        string driver_id FK
        float lat
        float lng
        timestamp ts
    }
    TRIP {
        string trip_id PK
        string rider_id FK
        string driver_id FK
        string status "requested/matched/en_route/in_progress/completed/cancelled"
        float pickup_lat
        float pickup_lng
        float dropoff_lat
        float dropoff_lng
        float surge_multiplier
        float fare
        timestamp requested_at
        timestamp completed_at
    }
    PAYMENT {
        string payment_id PK
        string trip_id FK
        float amount
        string status
    }
    RATING {
        string trip_id FK
        string rater_id
        string ratee_id
        int score
    }
```

---

## 4. Geospatial Indexing (The Core Hard Problem)

```mermaid
flowchart TB
    A["Driver location updates<br/>constantly (every 3-4s)"] --> B["Geospatial Index"]
    B --> C{"Indexing Strategy"}
    C --> D["Geohashing<br/>(divide map into grid cells,<br/>encode lat/lng as string prefix)"]
    C --> E["Quadtree<br/>(recursively subdivide space,<br/>denser subdivision in dense areas)"]

    D --> F["'Nearby drivers' query =<br/>look up drivers in same/adjacent<br/>geohash cells"]
    E --> F

    F --> G["Redis GEOADD/GEORADIUS<br/>or custom quadtree service"]
    G --> H["Returns candidate drivers<br/>within radius, sorted by distance"]
```

**Why geohashing/quadtrees:** Naive "scan all drivers, compute distance" doesn't scale to millions of drivers per query. Geohashing converts 2D lat/lng into a 1D sortable string where nearby locations share string prefixes — so "find nearby drivers" becomes a fast prefix/range lookup instead of a full scan. Quadtrees adaptively subdivide dense areas (city centers) more finely than sparse ones (rural areas), balancing index granularity with data density.

---

## 5. Driver Location Update Flow

```mermaid
sequenceDiagram
    participant D as Driver App
    participant GW as API Gateway
    participant LS as Location Service
    participant GeoIdx as Geospatial Index (Redis)
    participant LocStore as Driver Location Store

    loop Every 3-4 seconds
        D->>GW: POST /location {lat, lng}
        GW->>LS: Forward update
        LS->>GeoIdx: Update driver's geo-cell position<br/>(GEOADD driver_id lng lat)
        LS->>LocStore: Update last-known location + timestamp<br/>(TTL: mark stale if no update in 30s)
    end
```

*Stale detection matters: if a driver's app crashes or loses connectivity, their last-known position must eventually be excluded from matching (via TTL expiry) so riders aren't matched with a driver who's actually offline.*

---

## 6. Ride Matching Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant R as Rider App
    participant GW as API Gateway
    participant MS as Matching Service
    participant GeoIdx as Geospatial Index
    participant PS as Pricing Service
    participant TS as Trip Service
    participant D as Candidate Driver

    R->>GW: Request ride {pickup_lat, pickup_lng, dropoff}
    GW->>MS: Forward request

    MS->>GeoIdx: Query nearby available drivers<br/>(expanding radius search)
    GeoIdx-->>MS: List of candidate drivers, sorted by distance

    MS->>PS: Get current surge multiplier for this area
    PS-->>MS: Surge = 1.5x

    MS->>MS: Rank candidates<br/>(distance, driver rating, ETA)
    MS->>D: Send ride offer to top candidate
    alt Driver accepts (within timeout, e.g. 10s)
        D-->>MS: Accept
        MS->>TS: Create trip record (status = MATCHED)
        TS-->>R: Matched! Driver details + ETA
        MS->>GeoIdx: Mark driver as BUSY (remove from available pool)
    else Driver declines or times out
        MS->>MS: Move to next candidate driver
        MS->>D: Send offer to next candidate
    end
```

**Key design point:** Matching uses an **expanding radius search with sequential offers**, not simultaneous broadcast to all nearby drivers — this avoids the "multiple drivers accept the same ride" race condition and keeps the process deterministic and fair.

---

## 7. Surge Pricing Calculation

```mermaid
flowchart TB
    A["Geo-cell (e.g., downtown area)"] --> B["Real-time metrics per cell"]
    B --> C["Active ride requests<br/>in last N minutes"]
    B --> D["Available drivers<br/>in cell right now"]
    C & D --> E["Demand/Supply Ratio"]
    E --> F{"Ratio &gt; threshold?"}
    F -- Yes --> G["Apply surge multiplier<br/>(e.g., 1.2x - 3x)"]
    F -- No --> H["Base fare, no surge"]
    G --> I["Cache multiplier per cell<br/>(short TTL, e.g., 2-5 min)"]
    H --> I
    I --> J["Applied to all matches<br/>in that cell during window"]
```

*Surge pricing serves a dual purpose: it dynamically balances marketplace supply/demand (higher prices bring more drivers online in high-demand areas) while managing rider expectations transparently at request time.*

---

## 8. Trip Lifecycle State Machine

```mermaid
stateDiagram-v2
    [*] --> Requested
    Requested --> Matched: Driver accepts
    Requested --> Cancelled: No driver found / rider cancels
    Matched --> EnRoute: Driver heading to pickup
    Matched --> Cancelled: Rider or driver cancels
    EnRoute --> InProgress: Driver arrives, trip starts
    EnRoute --> Cancelled: Rider no-show / cancellation
    InProgress --> Completed: Trip ends, dropoff reached
    Completed --> [*]
    Cancelled --> [*]
```

---

## 9. Real-Time Trip Tracking (During Ride)

```mermaid
sequenceDiagram
    participant D as Driver App
    participant LS as Location Service
    participant TS as Trip Service
    participant PubSub as Pub/Sub Channel (per trip_id)
    participant R as Rider App

    loop Every few seconds during trip
        D->>LS: Location update
        LS->>TS: Forward (trip is InProgress)
        TS->>PubSub: Publish location to trip's channel
        PubSub-->>R: Push live location update
        R->>R: Update map with driver's<br/>real-time position
    end
```

*Once a trip is active, location updates are routed through a per-trip pub/sub channel rather than the general geospatial index — the rider only cares about their specific driver's position, not the broader matching pool.*

---

## 10. Regional/City-Level Partitioning

```mermaid
flowchart TB
    subgraph Global["Global Layer"]
        GlobalRouter["Global Request Router<br/>(routes by rider's geo-region)"]
    end

    subgraph RegionUS["US-East Region"]
        MatchUS["Matching Service (US-East)"]
        GeoUS["Geospatial Index (US-East)"]
        TripDBUS[("Trip DB (US-East)")]
    end

    subgraph RegionEU["EU Region"]
        MatchEU["Matching Service (EU)"]
        GeoEU["Geospatial Index (EU)"]
        TripDBEU[("Trip DB (EU)")]
    end

    GlobalRouter --> MatchUS
    GlobalRouter --> MatchEU
    MatchUS --> GeoUS --> TripDBUS
    MatchEU --> GeoEU --> TripDBEU

    Note1["Matching is inherently local —<br/>a rider in NYC never needs to be matched<br/>against a driver in London.<br/>Full regional isolation is natural and safe."]
```

*Since ride matching never needs cross-region coordination, this is one of the cleanest cases for full geographic sharding — each region operates almost as an independent deployment, which also contains blast radius during regional outages.*

---

## 11. Component Responsibilities Summary

```mermaid
mindmap
  root((Uber/Lyft HLD))
    Location Service
      Ingests high-frequency GPS pings
      Updates geospatial index
      Stale-driver detection via TTL
    Matching Service
      Nearby driver search
      Sequential offer dispatch
      Race-condition-free acceptance
    Pricing Service
      Real-time supply/demand ratio
      Surge multiplier calculation
    Trip Service
      Trip lifecycle state machine
      Persists trip records
    Geospatial Index
      Geohash/Quadtree based
      Fast radius queries at scale
    Payment Service
      Fare calculation
      Charge on trip completion
```

---

## 12. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Geospatial indexing | Geohashing/Quadtree in-memory index (Redis) | Naive distance-scan doesn't scale; prefix-based lookups make radius queries fast at millions of drivers |
| Matching dispatch | Sequential offers to ranked candidates | Prevents race conditions from broadcasting to multiple drivers simultaneously |
| Location update frequency | Every 3-4 seconds, TTL-based staleness | Balances real-time accuracy against bandwidth/server load; stale entries auto-expire |
| Regional partitioning | Full geographic sharding, no cross-region matching | Ride matching is inherently local; enables blast-radius containment and independent scaling per city/region |
| Surge pricing | Per-geo-cell real-time ratio, short-TTL cache | Localizes pricing signal to where imbalance actually exists, not a global average |
| Trip tracking | Dedicated per-trip pub/sub channel | Decouples active-trip tracking from the general matching geospatial index |

---

## 13. Bottlenecks & Scaling Considerations

- **Location ping volume** — ~250K updates/sec at global peak is the dominant write load; in-memory geospatial stores (Redis with GEO commands, or custom quadtree services) are essential since disk-backed DBs can't sustain this write rate.
- **Hot geo-cells** — dense urban cores (Manhattan, downtown SF) can have disproportionately high driver density in a small area; adaptive subdivision (quadtree) handles this better than fixed-size geohash cells.
- **Matching race conditions** — must guarantee a driver can't be double-booked; sequential offers with a reservation lock (mark driver BUSY immediately on offer, not just on accept) prevent this, with a timeout-based release if the driver doesn't respond.
- **Surge calculation freshness vs stability** — recalculating surge too frequently causes rider-visible price flicker; too infrequently makes it unresponsive to real demand spikes — typically smoothed with short time windows (1-2 min).
- **Driver app connectivity gaps** (tunnels, poor signal) — location staleness must be handled gracefully; a driver missing a few pings shouldn't be instantly dropped from the matching pool, but should be excluded well before staleness reaches minutes.
- **Cross-region edge cases** — riders near a regional boundary (e.g., near a country border) need careful routing logic to avoid being matched against an out-of-region driver pool or missing valid nearby drivers just across the boundary.
- **Payment processing reliability** — payment must be decoupled from the trip-completion critical path (don't block "trip completed" UX on payment gateway latency); process asynchronously with retry and reconciliation.
