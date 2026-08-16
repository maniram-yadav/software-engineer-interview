# System Design Interview Prep Pack

A complete, structured reference covering the most frequently tested system design interview topics — core concepts, classic problems, advanced problems, database internals, real-world architecture challenges, and reliability patterns. Each file uses Mermaid diagrams, comparison tables, worked numeric examples, and explicit trade-off analysis.

## How to use this pack
1. Read `01-core-concepts.md` first — these six ideas (latency/throughput, indexing, queues vs streams, API gateways, caching, rate limiting) are the *building blocks* referenced throughout every design problem in the other files.
2. Work through `02` and `03` (classic and advanced design problems) using the standard framework: **Requirements → Capacity Estimate → High-Level Design → Deep Dive → Trade-offs**. Try to draw the diagram yourself before looking at the given one.
3. `04-database-topics.md` and `06-reliability-topics.md` are the topics interviewers use for **deep-dive follow-up questions** once your high-level design is on the whiteboard — know these cold, since "what happens when X fails" is where most interviews are actually won or lost.
4. `05-real-world-architecture.md` covers operational/production concerns (hot partitions, deployments, migrations) that separate senior/staff-level answers from junior ones — mention these proactively even if not asked directly.

## File index

| File | Topics |
|---|---|
| [01-core-concepts.md](01-core-concepts.md) | Latency vs Throughput · Database Indexing · Message Queues vs Event Streams · API Gateway · Caching Strategies · Rate Limiting Algorithms |
| [02-classic-design-problems.md](02-classic-design-problems.md) | Web Crawler · Key-Value Store · Chat Application · Autocomplete/Typeahead · Job Scheduler · Ticket Booking System |
| [03-advanced-design-problems.md](03-advanced-design-problems.md) | Food Delivery Dispatch · Video Recommendation Engine · Distributed Job Queue (Kafka-like) · Ad Serving & Bidding · Multiplayer Game State Sync |
| [04-database-topics.md](04-database-topics.md) | Partitioning vs Sharding · Leader Election · Read Replicas & Replication Lag · Transaction Isolation Levels |
| [05-real-world-architecture.md](05-real-world-architecture.md) | Handling Hot Partitions · Zero-Downtime Deployments · Backward-Compatible Schema Migrations |
| [06-reliability-topics.md](06-reliability-topics.md) | Graceful Degradation · Bulkhead Isolation · Dead Letter Queues · Health Checks & Self-Healing |

## Cross-cutting themes to internalize (interviewers reward these explicitly)
- **CAP theorem framing**: for every distributed data problem, be ready to say which of consistency/availability you're prioritizing during a partition, and why, given the specific use case.
- **Everything is a trade-off**: never present a design decision without stating what you gave up to get it. This single habit is the biggest differentiator between mid-level and senior-level answers.
- **Numbers matter**: back-of-envelope capacity estimates (QPS, storage, bandwidth) should show up in every design — interviewers want to see you reason quantitatively, not just architecturally.
- **Failure is the default assumption**: for any component you draw, be ready to answer "what happens when this dies/is slow/is partitioned from the rest?" — this is where topics in files 04 and 06 get pulled in as deep-dive follow-ups to files 02 and 03.
- **Scale changes the answer**: a design that's correct at 100 RPS is often wrong at 100K RPS (e.g., single DB → sharded, synchronous → async/queue-based, single-region → multi-region). Show you understand *why* the design evolves with scale, not just the end-state architecture.
