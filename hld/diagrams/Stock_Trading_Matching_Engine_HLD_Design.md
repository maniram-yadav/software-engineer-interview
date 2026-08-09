# Design a Stock Trading Matching Engine — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Accept buy/sell orders for a given security and match compatible orders together to execute trades
- Maintain an order book (all currently outstanding, unmatched orders) per security
- Support multiple order types: market orders (execute immediately at best price), limit orders (execute only at a specified price or better)
- Enforce price-time priority: at the same price, earlier orders execute before later ones

### Non-Functional Requirements
- **Ultra-low latency (the defining requirement):** Matching decisions often need to happen in microseconds, not milliseconds — this shapes literally every architectural decision
- **Strict ordering/fairness:** The exact sequence in which orders are matched has direct financial consequences and regulatory scrutiny — determinism is non-negotiable
- **Extreme throughput during volatility:** Order volume can spike enormously during market volatility, precisely when correctness matters most
- **Zero tolerance for lost or duplicated trades:** A financial matching error has direct, immediate monetary and legal consequences

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Orders/sec (per security, peak) | Tens of thousands during high volatility |
| Matching latency target | Single-digit to low double-digit MICROSECONDS |
| Order book depth | Thousands of outstanding orders per active security |
| Securities traded | Thousands, each needing independent, isolated matching |

---

## 2. The Core Architectural Principle — Single-Threaded, In-Memory Matching Per Security

```mermaid
flowchart TB
    A["Naive approach: distribute<br/>order matching across MULTIPLE<br/>threads/machines for a given<br/>security, for 'scalability'"] --> A1["Problem: matching REQUIRES a<br/>strict, deterministic ORDER<br/>of operations (price-time<br/>priority) — introducing<br/>concurrency/distribution for<br/>a SINGLE security's order<br/>book creates race conditions<br/>and non-deterministic matching<br/>outcomes, which is<br/>UNACCEPTABLE for a financial<br/>system with strict fairness<br/>and regulatory requirements"]

    B["Correct approach: EACH<br/>security's order book is<br/>matched by a SINGLE-THREADED,<br/>IN-MEMORY process — this<br/>eliminates concurrency-related<br/>ordering ambiguity ENTIRELY<br/>for that security, since<br/>only one thread ever touches<br/>that specific order book"] --> B1["Scalability is instead<br/>achieved HORIZONTALLY —<br/>DIFFERENT securities are<br/>matched by DIFFERENT,<br/>completely independent<br/>single-threaded engines,<br/>running in parallel across<br/>many machines/cores"]
```

**Why this single-threaded-per-security design is the industry-standard approach:** This is precisely how real production exchanges achieve both ultra-low latency AND strict correctness simultaneously — rather than trying to parallelize matching WITHIN one security's order book (which would require complex, latency-adding locking/coordination), the system parallelizes ACROSS securities, where each security's matching is embarrassingly independent of every other security's.

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph Clients["Trading Clients"]
        Trader1["Trading Firm 1"]
        Trader2["Trading Firm 2"]
    end

    subgraph Gateway["Order Gateway"]
        OrderValidator["Order Validation<br/>(risk checks, format)"]
        OrderRouter["Order Router<br/>(routes to correct<br/>security's engine)"]
    end

    subgraph MatchingEngines["Matching Engines<br/>(one per security, isolated)"]
        EngineAAPL["Matching Engine: AAPL<br/>(single-threaded, in-memory<br/>order book)"]
        EngineGOOG["Matching Engine: GOOG<br/>(single-threaded, in-memory<br/>order book)"]
    end

    subgraph Persistence["Durability Layer"]
        SequencedLog[("Sequenced Order Log<br/>— append-only, WAL-style,<br/>same pattern as the WAL<br/>and Recovery System design")]
    end

    subgraph Distribution["Market Data Distribution"]
        TradeFeed["Trade Execution Feed"]
        MarketDataFeed["Order Book Update Feed"]
    end

    Trader1 --> OrderValidator
    Trader2 --> OrderValidator
    OrderValidator --> OrderRouter

    OrderRouter --> EngineAAPL
    OrderRouter --> EngineGOOG

    EngineAAPL --> SequencedLog
    EngineGOOG --> SequencedLog

    EngineAAPL --> TradeFeed
    EngineAAPL --> MarketDataFeed
    EngineGOOG --> TradeFeed
    EngineGOOG --> MarketDataFeed
```

**Key idea:** Every order, before reaching a matching engine, passes through validation and routing — but once inside a specific security's matching engine, it's processed by a SINGLE thread operating purely on IN-MEMORY data structures, with durability achieved via writing to a sequenced, append-only log (the same WAL principle from the dedicated WAL & Recovery System design) rather than synchronous database writes that would introduce unacceptable latency.

---

## 4. Data Model

```mermaid
erDiagram
    ORDER {
        string order_id PK
        string security_symbol
        string side "buy/sell"
        string order_type "market/limit"
        float limit_price "null for market orders"
        int quantity
        int remaining_quantity
        long sequence_number "strict arrival order"
        timestamp submitted_at
    }
    TRADE_EXECUTION {
        string trade_id PK
        string buy_order_id FK
        string sell_order_id FK
        float execution_price
        int quantity
        timestamp executed_at
    }
    ORDER_BOOK_LEVEL {
        string security_symbol FK
        float price_level
        string side
        int total_quantity
        list order_ids "in strict time priority order"
    }
```

---

## 5. Order Book Structure — Price-Time Priority

```mermaid
flowchart TB
    A["Order Book for AAPL"] --> B["BID side (buyers,<br/>highest price = best)"]
    A --> C["ASK side (sellers,<br/>lowest price = best)"]

    B --> B1["$150.05: [Order A (10:00:01),<br/>Order D (10:00:03)]"]
    B --> B2["$150.00: [Order B (10:00:02)]"]

    C --> C1["$150.10: [Order C (10:00:01)]"]
    C --> C2["$150.15: [Order E (10:00:04)]"]

    D["Price-Time Priority Rule:<br/>at the SAME price level,<br/>orders are matched in the<br/>EXACT order they arrived —<br/>this is why the sequence_number<br/>(strict arrival order) is<br/>such a critical, carefully-<br/>guarded field"] -.-> B1
```

**Why this data structure is typically implemented as a price-indexed sorted structure (not a simple list):** The matching engine needs to instantly find "the best available price" on each side — a structure like a sorted tree or price-level array indexed by price, with a time-ordered list AT each price level, gives O(log n) or better access to the best price while still preserving strict time-priority within that price level.

---

## 6. Order Matching Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Router as Order Router
    participant Engine as Matching Engine (AAPL)
    participant OrderBook as In-Memory Order Book
    participant Log as Sequenced Log

    Router->>Engine: New order: BUY 100 shares<br/>AAPL @ $150.10 (limit)

    Engine->>Engine: Assign strict sequence<br/>number (monotonic, this<br/>engine's own counter)

    Engine->>OrderBook: Check ASK side for<br/>matching sell orders<br/>at price <= $150.10

    OrderBook-->>Engine: Best ask: $150.10,<br/>Order C, 50 shares available<br/>(earliest at this price)

    Engine->>Engine: MATCH: 50 shares @ $150.10<br/>between incoming buy order<br/>and Order C

    Engine->>Log: Append TRADE_EXECUTION event<br/>(durability BEFORE<br/>considering the match final —<br/>same WAL principle)

    Engine->>OrderBook: Update: Order C fully<br/>filled, remove from book;<br/>incoming order has<br/>50 shares remaining

    Engine->>OrderBook: Continue matching remaining<br/>50 shares against NEXT<br/>best ask level, or if<br/>none available at<br/>acceptable price, REST<br/>the remainder in the<br/>book as a new resting<br/>limit order

    Engine-->>Router: Trade confirmations
```

---

## 7. Why Durability Logging Must Not Add Latency to the Matching Decision

```mermaid
flowchart TB
    A["Matching decision itself:<br/>PURELY in-memory, microsecond-<br/>scale — this is the<br/>time-critical hot path"] --> B["The MATCH is considered<br/>DECIDED the instant the<br/>in-memory logic determines<br/>it — but durability logging<br/>(writing to the sequenced<br/>log) happens in a carefully<br/>engineered, HIGHLY OPTIMIZED<br/>WRITE PATH designed to add<br/>MINIMAL additional latency"]

    C["Techniques used: writing to<br/>a pre-allocated, memory-mapped<br/>log file (avoiding syscall<br/>overhead), batching multiple<br/>sequential log writes,<br/>and using extremely fast<br/>persistent storage (e.g.,<br/>NVMe) — the durability<br/>write happens essentially<br/>IN PARALLEL with, not<br/>BLOCKING, the matching<br/>engine's progression to<br/>the next order"] --> D["This is the same fundamental<br/>WAL principle as the general<br/>WAL & Recovery System design,<br/>but engineered with FAR more<br/>extreme latency optimization<br/>given this domain's<br/>microsecond-scale requirements"]
```

---

## 8. Handling Matching Engine Failure & Recovery

```mermaid
sequenceDiagram
    participant Engine as Matching Engine<br/>(crashes)
    participant Restart as Restart Process
    participant Log as Sequenced Order Log
    participant NewEngine as Recovered Engine Instance

    Note over Engine: Process crashes<br/>(hardware fault, etc.)

    Restart->>Log: Read the COMPLETE sequenced<br/>log for this security from<br/>the beginning (or the<br/>most recent snapshot,<br/>same checkpoint principle<br/>as the WAL design)

    Restart->>NewEngine: Replay every logged order<br/>and match decision IN THE<br/>EXACT SEQUENCE they<br/>originally occurred

    NewEngine->>NewEngine: Rebuild the EXACT in-memory<br/>order book state that<br/>existed at the moment of<br/>the crash — deterministic<br/>replay guarantees this<br/>reconstruction is EXACT,<br/>not approximate

    Note over NewEngine: Only AFTER full, verified<br/>replay does the recovered<br/>engine begin accepting<br/>NEW orders — resuming<br/>EXACTLY where the crashed<br/>instance left off, with<br/>zero ambiguity about which<br/>trades genuinely executed
```

**Why deterministic replay is essential here specifically (beyond general WAL recovery):** Because matching decisions have direct financial and regulatory consequences, recovery cannot produce even a SLIGHTLY different outcome than what would have naturally occurred — the sequenced log combined with fully deterministic matching logic (same inputs in the same order ALWAYS produce the same matches) is what guarantees recovery reconstructs the EXACT correct state, not just a reasonable approximation.

---

## 9. Market Data Distribution (Low-Latency Fan-Out)

```mermaid
flowchart TB
    A["Every trade execution and<br/>order book change must be<br/>broadcast to potentially<br/>THOUSANDS of subscribed<br/>trading clients/systems,<br/>with minimal latency"] --> B["Multicast-based distribution<br/>(not point-to-point per<br/>subscriber) — the matching<br/>engine publishes ONCE, and<br/>network-level multicast<br/>replication delivers to ALL<br/>subscribers simultaneously,<br/>rather than the engine<br/>itself iterating through<br/>and sending to each<br/>subscriber individually"]

    B --> C["This avoids the matching<br/>engine's own precious<br/>processing time being spent<br/>on FAN-OUT logic — that<br/>responsibility is offloaded<br/>to specialized, separate<br/>network infrastructure,<br/>keeping the matching<br/>engine's core loop laser-<br/>focused purely on matching<br/>decisions"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Trading Matching Engine HLD))
    Order Gateway
      Validation and risk checks
      Routes to correct security engine
    Matching Engine
      Single-threaded per security
      Pure in-memory order book
      Price-time priority enforcement
    Sequenced Log
      WAL-style durability
      Deterministic replay source
    Order Book Structure
      Price-indexed, time-ordered
      O(log n) best-price access
    Market Data Distribution
      Multicast fan-out
      Offloaded from matching hot path
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Concurrency model | Single-threaded per security, parallel across securities | Eliminates race conditions and non-deterministic matching WITHIN a security entirely, while still achieving overall system scalability horizontally |
| Processing location | Purely in-memory | Database-speed persistence is far too slow for microsecond-scale matching decisions — durability is achieved via optimized logging, not synchronous DB writes |
| Durability mechanism | Highly optimized sequenced append-only log | Same WAL principle as general database recovery, engineered specifically for extreme low-latency requirements unique to this domain |
| Recovery approach | Deterministic full replay from sequenced log | Guarantees EXACT, not approximate, state reconstruction — essential given the direct financial/regulatory consequences of matching decisions |
| Market data distribution | Multicast, offloaded from the matching hot path | Keeps the matching engine's core loop focused purely on matching, not burdened with fan-out responsibilities |

---

## 12. Bottlenecks & Scaling Considerations

- **Single-security throughput ceiling** — because a security's matching is deliberately single-threaded, its maximum throughput is bounded by that ONE thread's processing speed; for extremely high-volume securities during extreme volatility, this ceiling is a genuine, accepted architectural constraint (not a bug), requiring careful engine implementation optimization rather than adding concurrency, which would compromise correctness.
- **Hardware and network optimization as a first-class concern** — at microsecond-scale latency requirements, factors invisible to typical system design (CPU cache locality, network interface card configuration, even physical proximity of servers to reduce cable-length propagation delay) become genuine, significant engineering considerations — this domain pushes optimization down to a level most other systems in this document series never need to consider.
- **Order gateway validation latency** — while the matching engine itself is optimized to the extreme, the PRE-matching validation/risk-check step (Order Gateway) must also be carefully latency-optimized, since it sits directly in the critical path before an order even reaches the matching engine.
- **Handling extreme volatility spikes** — order volume can spike dramatically during major market events, precisely when matching correctness and speed matter most; the system must be provisioned with substantial headroom above typical average load specifically for these predictable-in-nature-if-not-in-timing volatility events.
- **Regulatory audit requirements** — beyond basic durability, financial matching engines typically face STRICT regulatory requirements for complete, immutable audit trails of every order and match decision — this connects directly to the same tamper-evident logging principles covered in the Tamper-Evident Audit Log design, applied to trading activity specifically.
- **Cross-security dependencies (rare but real)** — while this design assumes securities are matched independently, some real-world scenarios (e.g., circuit breakers halting trading across an entire market during extreme volatility) require SOME cross-security coordination — this is typically handled as a separate, higher-level control layer rather than compromising the core per-security matching engine's independence.
- **Testing determinism rigorously** — given the zero-tolerance requirement for matching correctness, testing must verify BOTH that the matching logic is genuinely deterministic (same input sequence always produces the same output) AND that recovery-via-replay genuinely reconstructs identical state — this requires the same rigorous fault-injection and property-based testing discipline emphasized throughout the more correctness-critical designs in this series (WAL Recovery, Exactly-Once Stream Processing).
