# 80 High-Level Design (HLD) Interview Questions

A curated set of system design questions spanning classic app design, distributed systems theory, infrastructure, security, ML systems, and streaming architectures. Use these to prep for senior/staff-level design interviews.

**How to use this list:** For each question, structure your answer as: (1) clarify requirements — functional & non-functional, (2) back-of-envelope estimation, (3) high-level architecture, (4) deep dive into 1-2 hard parts, (5) bottlenecks & scaling/failure modes.

---

## Part 1: Foundational System Design (Q1–30)

### Social & Content Platforms
1. **Design Twitter/X** — Feed generation, fanout-on-write vs fanout-on-read, timeline ranking.
2. **Design Instagram** — Media storage, CDN strategy, feed + stories architecture.
3. **Design a news feed ranking system** — Personalization, freshness vs relevance tradeoffs.
4. **Design YouTube/Netflix** — Video upload pipeline, transcoding, adaptive streaming, CDN.
5. **Design a comments system at scale** — Nested threads, moderation, real-time updates.

### Messaging & Real-Time
6. **Design WhatsApp/Messenger** — Delivery guarantees, read receipts, end-to-end encryption at scale.
7. **Design a notification system** — Push/email/SMS fan-out, retries, per-user rate limiting.
8. **Design a real-time collaborative document editor** — OT/CRDT, conflict resolution (Google Docs-style).
9. **Design a live chat support system** — Presence indicators, typing status, agent routing.

### Marketplaces & Transactional Systems
10. **Design Uber/Lyft** — Geospatial indexing, matching algorithm, surge pricing consistency.
11. **Design an e-commerce checkout system** — Inventory locking, payment idempotency, order state machine.
12. **Design a ticket booking system** — Ticketmaster-style seat contention handling at scale.
13. **Design a food delivery platform** — Multi-sided marketplace: restaurants, drivers, customers.
14. **Design a payment processing system** — Double-entry ledger, idempotency, reconciliation.

### Infrastructure & Storage
15. **Design a distributed key-value store** — Consistency models, replication, partitioning (DynamoDB-style).
16. **Design a distributed cache** — Eviction policies, cache invalidation, thundering herd mitigation.
17. **Design a URL shortener** — ID generation strategies, redirect latency, analytics.
18. **Design a rate limiter** — Token bucket vs sliding window, distributed rate limiting.
19. **Design a distributed job scheduler / task queue** — Cron-at-scale, exactly-once semantics.
20. **Design a distributed file storage system** — Dropbox/Drive-style chunking, sync conflicts, dedup.
21. **Design a distributed lock manager** — Fencing tokens, lease expiry, deadlock avoidance.
22. **Design a message queue** — Kafka-like partitioning, ordering guarantees, consumer groups.

### Search & Data-Intensive Systems
23. **Design a search engine / autocomplete system** — Indexing, ranking, typo tolerance, trie-based suggestions.
24. **Design a web crawler** — Politeness, dedup, distributed crawling, freshness.
25. **Design an analytics/metrics dashboard system** — Time-series ingestion, aggregation, rollups.
26. **Design a recommendation system** — Collaborative filtering vs content-based, cold start problem.
27. **Design a log aggregation and monitoring system** — ELK/Datadog-style high write throughput.

### Advanced / Cross-Cutting Concepts
28. **Design a multi-region system with strong consistency** — CAP tradeoffs across regions.
29. **Design an ad click aggregation / fraud detection system** — High write volume, near-real-time correctness.
30. **Design a system for idempotent API requests at scale** — Dedup keys, exactly-once vs at-least-once.

---

## Part 2: Advanced, Theory-Heavy & Specialized Design (Q31–80)

### Distributed Systems Theory & Consensus
31. **Design a distributed consensus system** — Implement Raft/Paxos conceptually: leader election, log replication, safety.
32. **Design a distributed transaction system across microservices** — 2PC vs Saga pattern, compensating transactions.
33. **Linearizability vs eventual consistency** — Design a system needing each and justify the tradeoffs with a concrete case.
34. **Design a vector clock / causal ordering system** — For a distributed database.
35. **Design a leader election system for 1000+ nodes** — Split-brain prevention.
36. **Design a distributed counter (CRDT-based)** — Highly available and eventually accurate.
37. **Design network partition detection & resolution** — End-to-end partition tolerance strategy.
38. **Design a globally distributed database (Spanner-style)** — TrueTime, external consistency.

### Databases & Storage Engines
39. **Design a time-series database** — Prometheus/InfluxDB-style compression, downsampling, retention.
40. **Design a graph database** — Storage model for nodes/edges, traversal query optimization.
41. **Design a multi-tenant SaaS database architecture** — Shared DB vs shared schema vs siloed isolation tradeoffs.
42. **Design a sharding strategy for a growing e-commerce platform** — Resharding without downtime.
43. **Design a write-ahead log (WAL) & recovery system** — For a custom database engine.
44. **Design an OLAP system for real-time BI** — Star schema, columnar storage, pre-aggregation.
45. **Design a change data capture (CDC) pipeline** — Debezium-style DB-to-downstream sync.
46. **Design a document versioning/history system** — Efficient diff storage (Google Docs revision history).
47. **Design a secondary index system for a distributed database** — Consistency between primary and index.

### Caching & Performance at Extreme Scale
48. **Design a multi-layer CDN caching architecture** — Edge, regional, origin cache coherence.
49. **Design cache warming & stampede prevention** — For a high-traffic launch event.
50. **Design a "hot key" mitigation system** — Celebrity/viral content skew in distributed caches.
51. **Design client + server caching for a mobile app** — Handling intermittent connectivity.

### Security, Identity & Compliance
52. **Design a single sign-on (SSO) system** — SAML + OAuth2/OIDC across multiple identity providers.
53. **Design a secrets management system** — Vault-style rotation, access control, audit logging.
54. **Design an end-to-end encrypted file sharing system** — Key management and revocation.
55. **Design a real-time credit card fraud detection system** — Feature engineering + low-latency scoring.
56. **Design a GDPR-compliant deletion system** — "Right to be forgotten" propagation across microservices.
57. **Design a tamper-evident audit logging system** — Cryptographic chaining, immutability.
58. **Design a bot detection / CAPTCHA-alternative system** — At scale, low false-positive rate.

### Machine Learning Infrastructure
59. **Design a feature store for ML models** — Online/offline consistency, point-in-time correctness.
60. **Design a real-time ML model serving system** — A/B testing, versioning, canary rollout, low-latency inference.
61. **Design a large-scale ML training pipeline** — Distributed data parallelism, checkpointing, fault tolerance.
62. **Design a personalization/ranking system with online learning** — Bandits, exploration vs exploitation.
63. **Design an LLM inference serving platform** — Batching, KV-cache management, multi-GPU routing.
64. **Design a content moderation system** — Combining ML models with human review queues.

### Networking & Infra-Level Systems
65. **Design a global DNS system** — Health-based routing and failover.
66. **Design a service mesh** — Sidecar proxies, traffic shaping, mTLS between services.
67. **Design an API gateway for 10,000+ backend services** — Auth, rate limiting, routing.
68. **Design a load balancer from scratch** — L4 vs L7, consistent hashing for backend selection.
69. **Design a multi-region active-active deployment** — Automated failover, RTO/RPO targets.
70. **Design a service discovery system** — For a dynamic microservices environment.

### Event-Driven & Streaming Architectures
71. **Design an event sourcing system for banking** — Event store, snapshots, replay.
72. **Design a CQRS-based architecture** — High-write, high-read e-commerce inventory system.
73. **Design a real-time fraud/anomaly detection pipeline** — Stream processing: windowing, watermarks, late data.
74. **Design an exactly-once stream processing pipeline** — Idempotency + checkpointing (Kafka/Flink-style).
75. **Design a distributed saga orchestrator** — For long-running business workflows.

### Specialized / Domain-Specific Systems
76. **Design a stock trading matching engine** — Order book, price-time priority, ultra-low latency.
77. **Design a multiplayer game server architecture** — State sync, lag compensation, authoritative server model.
78. **Design a global inventory management system** — Online + in-store sync for a retailer.
79. **Design a healthcare records system** — Strict access control, HL7/FHIR-like interoperability constraints.
80. **Design an IoT telemetry ingestion system** — Millions of devices: protocol choice, backpressure, edge aggregation.

---

## Suggested Prep Approach
- **Weeks 1–2:** Part 1 (Q1–30) — build fluency with the standard framework.
- **Weeks 3–4:** Part 2 theory sections (Consensus, Databases, Caching) — these test *why*, not just *what*.
- **Weeks 5–6:** Security, ML infra, Networking, Streaming, and Specialized systems — pick based on the role you're targeting (e.g., ML infra questions matter more for ML platform roles; trading engine matters for fintech).
- For every question, time-box yourself to 35–40 minutes end-to-end, as in a real interview.
