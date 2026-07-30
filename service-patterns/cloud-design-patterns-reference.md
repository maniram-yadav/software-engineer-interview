# Cloud Design Patterns — Complete Reference Guide

*Based on the Azure Architecture Center catalog (44 patterns), with added architect-level commentary, trade-offs, and practical guidance.*

> **How to read this doc:** For each pattern you get — Short Description, Problem It Solves, Use Case(s), Advantages, Disadvantages/Trade-offs, When to Use / When NOT to Use, and Related Patterns. Patterns are grouped by the problem family they belong to (not strictly alphabetical) so you can see how they relate.

---

## Table of Contents

1. [Communication & Gateway Patterns](#1-communication--gateway-patterns)
2. [Resiliency & Fault-Handling Patterns](#2-resiliency--fault-handling-patterns)
3. [Data Management Patterns](#3-data-management-patterns)
4. [Messaging Patterns](#4-messaging-patterns)
5. [Design & Implementation Patterns](#5-design--implementation-patterns)
6. [Deployment & Scaling Patterns](#6-deployment--scaling-patterns)
7. [Security & Identity Patterns](#7-security--identity-patterns)
8. [Combining Patterns & Anti-patterns](#combining-patterns)

---

## 1. Communication & Gateway Patterns

### 1.1 Ambassador
**Short Description:** A helper service (sidecar) that sits between a client/consumer service and the outside world, sending network requests on the consumer's behalf.

**Problem It Solves:** You need cross-cutting network concerns (retry, logging, TLS, circuit breaking, monitoring) applied consistently without duplicating that logic in every service, especially in polyglot environments.

**Use Case:** A legacy app that can't be modified needs retry/circuit-breaker logic added to its outbound calls; injecting an ambassador container beside it in Kubernetes handles this transparently.

**Advantages:**
- Decouples network/infra concerns from business logic
- Language-agnostic — works even if the main app can't be changed
- Centralizes cross-cutting network policies (auth, retries, monitoring)
- Reusable across services

**Disadvantages:**
- Adds latency (extra network hop)
- Extra operational complexity (another process/container to deploy & monitor)
- Overkill for simple apps

**When to Use:** Polyglot microservices, legacy modernization, service mesh–like needs without a full mesh.
**When NOT to Use:** Simple single-language apps where a shared library achieves the same result more cheaply.
**Related Patterns:** Sidecar (Ambassador is a specialized Sidecar), Gateway Offloading.

---

### 1.2 Backends for Frontends (BFF)
**Short Description:** Create separate backend services tailored to specific frontend applications (web, mobile, desktop) instead of one general-purpose backend.

**Problem It Solves:** A single shared backend API tries to serve very different clients (mobile vs. web vs. third-party) and becomes bloated with client-specific logic, versioning conflicts, and competing requirements.

**Use Case:** A mobile app needs lightweight, aggregated payloads (limited bandwidth), while a web app needs richer data; separate BFFs each optimize for their client.

**Advantages:**
- Each frontend gets an API shaped exactly for its needs
- Teams can evolve/deploy independently
- Reduces "god API" complexity and coordination overhead

**Disadvantages:**
- Code duplication across BFFs if not managed carefully
- More services to build, deploy, and monitor
- Risk of business logic leaking into the BFF layer

**When to Use:** Multiple client types with very different data/interaction needs; independent frontend teams.
**When NOT to Use:** Single client type, or when API needs are nearly identical across clients (adds needless overhead).
**Related Patterns:** Gateway Aggregation, API Gateway (Gateway Routing).

---

### 1.3 Gateway Routing
**Short Description:** Route incoming requests to multiple backend services through a single endpoint, based on path, header, or other rules.

**Problem It Solves:** Clients need one stable entry point, but the system behind it is composed of many services that may change, scale, or version independently.

**Use Case:** `/orders/*` routes to Order Service, `/users/*` routes to User Service, all behind one public API gateway (e.g., Azure API Management, Azure Application Gateway).

**Advantages:**
- Decouples clients from internal service topology
- Simplifies client configuration (single endpoint)
- Enables blue/green or canary routing centrally

**Disadvantages:**
- Gateway becomes a critical single point of failure if not made highly available
- Extra network hop/latency
- Can become a bottleneck or "God object" if it accumulates too much logic

**When to Use:** Microservices architectures needing unified entry point and flexible routing.
**When NOT to Use:** Monoliths, or very small systems where direct routing suffices.
**Related Patterns:** Gateway Aggregation, Gateway Offloading (often combined behind one gateway).

---

### 1.4 Gateway Aggregation
**Short Description:** Use a gateway to combine multiple backend requests into a single client-facing request/response.

**Problem It Solves:** A client (especially mobile, over high-latency networks) would otherwise need to make many chatty round-trips to multiple microservices to render one screen.

**Use Case:** A product details page needs data from Inventory, Pricing, and Reviews services — the gateway fetches all three and returns one combined JSON response.

**Advantages:**
- Reduces number of client round-trips → better performance on high-latency networks
- Simplifies client-side code
- Centralizes fan-out/fan-in logic

**Disadvantages:**
- Gateway must handle partial failures (some services down) gracefully
- Adds coupling between the gateway and the shape of multiple backend APIs
- Can become a performance bottleneck if aggregation is heavy

**When to Use:** Mobile/low-bandwidth clients, dashboard-style pages pulling from multiple services.
**When NOT to Use:** When clients can tolerate multiple parallel calls directly, or when aggregation logic differs wildly per client (consider BFF instead).
**Related Patterns:** Backends for Frontends, Gateway Routing.

---

### 1.5 Gateway Offloading
**Short Description:** Offload shared, specialized functionality (SSL termination, compression, auth, rate limiting) to a gateway proxy so backend services don't each reimplement it.

**Problem It Solves:** Every service reimplementing SSL/TLS, certificate rotation, compression, or authentication independently creates duplicated effort, inconsistency, and security risk.

**Use Case:** TLS termination and JWT validation handled at an API gateway (e.g., Azure Application Gateway/API Management) so backend microservices only handle business logic over plain HTTP internally.

**Advantages:**
- Simplifies backend services (single responsibility)
- Consistent, centrally-managed security/compression policy
- Easier certificate/key management (one place to rotate)

**Disadvantages:**
- Gateway becomes complex and critical — needs high availability
- Less flexibility for services needing custom handling of offloaded concerns
- Potential vendor lock-in to gateway's feature set

**When to Use:** Multiple services sharing common cross-cutting concerns (TLS, auth, rate limiting).
**When NOT to Use:** Very few services, or services genuinely needing bespoke handling of these concerns.
**Related Patterns:** Gateway Routing, Gateway Aggregation, Valet Key.

---

### 1.6 Gatekeeper
**Short Description:** Protect applications/services using a dedicated host that validates and sanitizes all requests before they reach private back-end resources.

**Problem It Solves:** Directly exposing backend/data services to untrusted clients increases the attack surface; you need a low-privilege broker to validate input first.

**Use Case:** A gatekeeper VM/service validates and filters requests from the public internet before forwarding sanitized requests to a backend with elevated data-access privileges.

**Advantages:**
- Reduces attack surface — the exposed component has minimal privileges
- Centralizes input validation and sanitization
- Backend services stay isolated from direct public exposure

**Disadvantages:**
- Extra hop = added latency
- Another component to build, secure, and maintain
- If the gatekeeper logic is weak, false sense of security

**When to Use:** High-security systems (financial, healthcare) where backend must never be directly reachable.
**When NOT to Use:** Low-risk internal systems already behind strong network perimeter controls.
**Related Patterns:** Valet Key, Gateway Offloading.

---

### 1.7 Static Content Hosting
**Short Description:** Serve static content (HTML, CSS, JS, images, videos) directly from cloud storage/CDN instead of through application compute.

**Problem It Solves:** Application servers spend compute cycles and cost serving files that never change per-request, when a storage service/CDN could do it cheaper and faster.

**Use Case:** A SPA's JS/CSS bundle and images are hosted on Azure Blob Storage + Azure CDN instead of an App Service instance.

**Advantages:**
- Much lower cost than compute-hosted static files
- Better performance via CDN edge caching, closer to users
- Frees application tier to focus on dynamic logic
- Scales automatically to huge traffic spikes

**Disadvantages:**
- Cache invalidation complexity on content updates
- Not suitable for dynamic/personalized content
- Requires CORS and cache-control configuration discipline

**When to Use:** Any app with a meaningful static-asset footprint (web/mobile apps, SPAs, downloads).
**When NOT to Use:** Content that's fully dynamic per-request/per-user.
**Related Patterns:** Cache-Aside, CDN-related patterns generally.

---

## 2. Resiliency & Fault-Handling Patterns

### 2.1 Retry
**Short Description:** Enable an application to transparently retry an operation that fails due to a transient fault.

**Problem It Solves:** Transient failures (brief network blips, momentary service unavailability, throttling) cause an operation to fail even though it would succeed if attempted again shortly after.

**Use Case:** A call to a database or downstream API returns a `503`/timeout; the client retries with exponential backoff before surfacing an error.

**Advantages:**
- Significantly improves perceived reliability against transient issues
- Simple to implement (libraries like Polly for .NET)
- Configurable backoff/jitter strategies reduce thundering-herd risk

**Disadvantages:**
- Naive retries can amplify load on an already struggling service ("retry storm")
- Not all failures are transient — retrying a bad request wastes time/resources
- Must be idempotent-safe, or retries can cause duplicate side effects (e.g., double charges)

**When to Use:** Calls to external/remote services or resources prone to transient faults (network calls, databases, APIs).
**When NOT to Use:** Non-idempotent operations without safeguards, or permanent/logical errors (e.g., 400 Bad Request).
**Related Patterns:** Circuit Breaker (pair together), Throttling, Rate Limiting.

---

### 2.2 Circuit Breaker
**Short Description:** Prevent an application from repeatedly attempting an operation likely to fail, by "tripping" a circuit after a failure threshold and failing fast until the dependency recovers.

**Problem It Solves:** Continuously retrying a failing/degraded dependency wastes resources, increases latency, and can cause cascading failures across the system.

**Use Case:** A payment service is down; after N consecutive failures the circuit "opens," subsequent calls fail immediately (or fall back), and periodically a "half-open" test call checks for recovery.

**Advantages:**
- Prevents cascading failures across dependent systems
- Fails fast, improving overall system responsiveness under stress
- Gives failing dependencies breathing room to recover

**Disadvantages:**
- Added complexity (state management: closed/open/half-open)
- Requires careful tuning of thresholds/timeouts — too aggressive causes false trips
- Needs monitoring/alerting so open circuits aren't silently ignored

**When to Use:** Calls to remote services where failure could cascade or degrade performance broadly.
**When NOT to Use:** Non-critical, cheap, purely local operations, or very rare/idempotent one-off calls.
**Related Patterns:** Retry (pairs naturally), Bulkhead, Health Endpoint Monitoring.

---

### 2.3 Bulkhead
**Short Description:** Isolate elements of an application into pools (e.g., separate thread pools, connection pools, or service instances) so failure in one doesn't sink the whole system.

**Problem It Solves:** A shared resource pool means one failing/slow dependency can exhaust resources (threads, connections) and take down unrelated functionality too — like one leaking compartment sinking an entire ship.

**Use Case:** Separate connection pools per downstream dependency, so a slow/failing "Recommendations" service doesn't starve threads needed for "Checkout."

**Advantages:**
- Contains failures — a problem in one area doesn't cascade system-wide
- Improves overall system resiliency and availability
- Allows differentiated resource allocation by criticality

**Disadvantages:**
- Increases resource overhead (multiple pools instead of one shared pool)
- Adds configuration/operational complexity
- Requires careful capacity planning per bulkhead

**When to Use:** Multi-tenant systems, or services calling multiple downstream dependencies with different criticality/reliability profiles.
**When NOT to Use:** Simple systems with a single dependency or low risk of resource contention.
**Related Patterns:** Circuit Breaker, Throttling, Deployment Stamps (bulkhead at a larger scale).

---

### 2.4 Health Endpoint Monitoring
**Short Description:** Expose functional health-check endpoints that external monitoring tools/load balancers can poll to verify an application/service is working correctly.

**Problem It Solves:** Infrastructure-level "is the process running" checks don't reveal whether an app can actually do its job (e.g., DB connection is broken, dependency unreachable).

**Use Case:** `/health` endpoint checks DB connectivity, cache availability, and dependency status; a load balancer removes unhealthy instances from rotation automatically.

**Advantages:**
- Enables automated detection & remediation (auto-restart, remove from LB rotation)
- Improves observability into real application health, not just process liveness
- Supports readiness/liveness distinctions for orchestrators (Kubernetes)

**Disadvantages:**
- Poorly designed health checks can cause false positives/negatives (cascading restarts)
- Checks that are too deep/expensive can themselves become a load risk
- Requires ongoing maintenance as dependencies change

**When to Use:** Any production service behind a load balancer or orchestrator.
**When NOT to Use:** N/A — nearly always beneficial; the risk is in poor implementation, not the pattern itself.
**Related Patterns:** Circuit Breaker, Throttling, Leader Election (health checks inform leader failover).

---

### 2.5 Compensating Transaction
**Short Description:** Undo the work of a sequence of steps that collectively form an eventually-consistent operation, when one of the later steps fails.

**Problem It Solves:** In distributed systems, you typically can't use ACID transactions across multiple services/databases. If step 3 of 5 fails, you need a way to "roll back" the logical effects of steps 1 and 2.

**Use Case:** Booking a trip involves reserving a flight, hotel, and car; if the car reservation fails, compensating actions cancel the flight and hotel reservations already made.

**Advantages:**
- Enables consistency across distributed/heterogeneous systems without 2PC
- Business-logic-driven "undo" is more flexible than rigid distributed transactions
- Works well with the Saga pattern for orchestrating multi-step workflows

**Disadvantages:**
- Compensating logic can be complex to design correctly for every failure mode
- Not all actions are truly reversible (e.g., sent an email, charged a non-refundable fee)
- True "isolation" isn't guaranteed — other transactions may observe intermediate state

**When to Use:** Multi-step distributed business processes needing eventual consistency without distributed locks/2PC.
**When NOT to Use:** Simple single-database transactions where native ACID guarantees suffice.
**Related Patterns:** Saga (Saga is often *built on* Compensating Transaction), Scheduler Agent Supervisor.

---

### 2.6 Saga
**Short Description:** Manage data consistency across microservices by breaking a distributed transaction into a sequence of local transactions, each with a corresponding compensating action.

**Problem It Solves:** Long-running business processes span multiple microservices/databases, and there's no distributed ACID transaction to guarantee all-or-nothing consistency.

**Use Case:** An e-commerce order process: reserve inventory → charge payment → schedule shipping. Implemented as either **choreography** (services react to each other's events) or **orchestration** (a central coordinator drives the sequence).

**Advantages:**
- Maintains data consistency without tightly coupled distributed transactions
- Choreography-based sagas keep services loosely coupled
- Orchestration-based sagas give clear, centralized visibility into workflow state

**Disadvantages:**
- Significantly more complex than a single ACID transaction
- Debugging/tracing across many services is harder
- Compensating logic must be designed for every possible failure point
- Choreography sagas can become hard to trace as event chains grow ("callback hell" for events)

**When to Use:** Multi-service business transactions requiring eventual consistency (order processing, travel booking, financial workflows).
**When NOT to Use:** Single-service or single-database operations; simple two-step interactions better solved with Compensating Transaction alone.
**Related Patterns:** Compensating Transaction, Choreography, Scheduler Agent Supervisor, Event Sourcing.

---

### 2.7 Scheduler Agent Supervisor
**Short Description:** Coordinate a set of distributed actions as a single operation, using a Scheduler to sequence steps, Agents to perform the steps on remote services, and a Supervisor to manage recovery if a step fails.

**Problem It Solves:** A multi-step distributed workflow needs resilient coordination — if a step fails or hangs, the system needs to detect it and retry/compensate/recover automatically rather than leaving the workflow stuck.

**Use Case:** A workflow engine schedules a chain of remote operations (e.g., provisioning cloud resources); a supervisor monitors progress and retries or rolls back failed steps.

**Advantages:**
- Provides structured, resilient coordination of complex distributed workflows
- Clear separation of concerns: scheduling vs. execution vs. failure recovery
- Naturally supports retry/rollback semantics

**Disadvantages:**
- Adds architectural complexity (three cooperating roles instead of one)
- Requires durable state storage to track workflow progress reliably
- Overkill for short, simple workflows

**When to Use:** Long-running, multi-step distributed workflows needing robust failure recovery.
**When NOT to Use:** Short synchronous operations, or workflows simple enough for direct Saga/Retry handling.
**Related Patterns:** Saga, Compensating Transaction, Leader Election.

---

### 2.8 Throttling
**Short Description:** Control the resource consumption of individual users, tenants, or services to protect overall system stability.

**Problem It Solves:** A single noisy tenant/user can consume disproportionate resources (CPU, bandwidth, DB connections), degrading service for everyone else.

**Use Case:** A multi-tenant SaaS API limits each tenant to N requests/second; requests beyond that are delayed, queued, or rejected with `429 Too Many Requests`.

**Advantages:**
- Protects overall system stability and fairness across tenants/users
- Enables predictable capacity planning and cost control
- Can support tiered service levels (e.g., premium vs. free tier limits)

**Disadvantages:**
- Poorly tuned limits frustrate legitimate high-usage customers
- Adds complexity in tracking usage per tenant/user in real time
- Requires clear customer communication (documented limits, headers, error messages)

**When to Use:** Multi-tenant systems, public APIs, or any shared resource at risk of being overwhelmed by a subset of consumers.
**When NOT to Use:** Single-tenant internal systems with predictable, controlled load.
**Related Patterns:** Rate Limiting, Priority Queue, Queue-Based Load Leveling.

---

### 2.9 Rate Limiting
**Short Description:** Avoid or minimize throttling errors by proactively controlling the rate at which a client consumes a resource (client- or server-enforced request caps).

**Problem It Solves:** Even well-behaved clients can accidentally overload a dependent service; you need a deliberate cap on request rate rather than reacting only after overload occurs.

**Use Case:** An API gateway enforces a token-bucket rate limiter per API key, rejecting or delaying requests beyond the allotted rate.

**Advantages:**
- Prevents overload before it happens rather than reacting to it
- Protects both the service and the calling client's own budget/quota
- Algorithms (token bucket, sliding window) allow fine control over burstiness

**Disadvantages:**
- Adds latency for delayed requests
- Wrong limits can throttle legitimate traffic during valid spikes
- Requires distributed rate-limiting infrastructure at scale (shared counters across nodes)

**When to Use:** Public/partner-facing APIs, protecting downstream dependencies with known capacity limits.
**When NOT to Use:** Purely internal low-risk calls with generous headroom.
**Related Patterns:** Throttling, Queue-Based Load Leveling, Circuit Breaker.

---

### 2.10 Queue-Based Load Leveling
**Short Description:** Use a queue as a buffer between a task/producer and a service/consumer to smooth out intermittent heavy load spikes.

**Problem It Solves:** Bursty workloads sent directly to a service can overwhelm it during peaks, even if average load is manageable; a queue decouples arrival rate from processing rate.

**Use Case:** Order submissions during a flash sale are placed on a queue (Azure Service Bus/Storage Queue) and processed by workers at a sustainable rate.

**Advantages:**
- Smooths traffic spikes, protecting downstream services from overload
- Decouples producers from consumers — improves resiliency (consumer downtime doesn't lose requests)
- Enables independent, cost-efficient scaling of consumers

**Disadvantages:**
- Introduces asynchronous processing — not suitable when caller needs an immediate response
- Adds latency between submission and processing
- Requires monitoring queue depth/age to detect backlog issues

**When to Use:** Bursty or unpredictable workloads, batch-friendly processing, decoupling producers from consumers.
**When NOT to Use:** Strictly synchronous, low-latency request/response requirements.
**Related Patterns:** Competing Consumers (natural pairing), Priority Queue, Asynchronous Request-Reply.

---

### 2.11 Competing Consumers
**Short Description:** Enable multiple concurrent consumers to process messages from the same messaging channel, distributing work in parallel.

**Problem It Solves:** A single consumer can't keep up with message volume; you need horizontal scale-out of message processing without duplicating work.

**Use Case:** Multiple worker instances read from the same Service Bus queue; the messaging infrastructure ensures each message is delivered to only one consumer.

**Advantages:**
- Scales message processing horizontally, matching throughput to load
- Improves availability (one consumer crashing doesn't stop processing)
- Naturally pairs with auto-scaling for cost-efficient elasticity

**Disadvantages:**
- Requires careful message ordering handling if order matters (see Sequential Convoy)
- Idempotency needed for duplicate delivery scenarios (at-least-once semantics)
- Poison messages can block a consumer instance if not handled (dead-letter queues needed)

**When to Use:** High-volume queued workloads needing parallel processing.
**When NOT to Use:** Workloads requiring strict per-message ordering across the whole queue (or use Sequential Convoy for partitioned ordering).
**Related Patterns:** Queue-Based Load Leveling, Sequential Convoy, Priority Queue.

---

### 2.12 Sequential Convoy
**Short Description:** Process a set of related messages in a defined order, without blocking processing of unrelated message groups.

**Problem It Solves:** Global FIFO ordering across an entire queue kills parallelism, but many workloads only need ordering *within* a related group (e.g., all events for one customer), not across all messages.

**Use Case:** Order-state-change events for a given `OrderId` must be processed in sequence, but events for different `OrderId`s can be processed fully in parallel (session-enabled queues, e.g., Azure Service Bus sessions).

**Advantages:**
- Preserves necessary per-entity ordering while maintaining overall parallel throughput
- Scales much better than strict global FIFO
- Works well with partition/session-aware messaging systems

**Disadvantages:**
- More complex consumer logic (session/group management)
- Uneven load if some groups are much larger/busier than others ("hot partition")
- Requires messaging infrastructure that supports sessions/grouping

**When to Use:** Event-driven systems where ordering matters per-entity but not globally.
**When NOT to Use:** Fully independent messages with no ordering requirements at all (plain Competing Consumers suffices).
**Related Patterns:** Competing Consumers, Event Sourcing.

---

### 2.13 Leader Election
**Short Description:** Coordinate actions in a distributed system by electing one instance as the leader responsible for managing a task or a set of collaborating instances.

**Problem It Solves:** Some tasks (e.g., a scheduled job, a coordinator role) must run on exactly one instance at a time, even though the application is deployed across multiple redundant instances.

**Use Case:** A cluster of app instances elects one leader (via a distributed lock/lease, e.g., Azure Blob lease, ZooKeeper, etcd) to run a nightly batch job so it doesn't run N times simultaneously.

**Advantages:**
- Ensures singleton-style coordination in a horizontally scaled, redundant system
- Automatic failover — a new leader is elected if the current one fails
- Avoids duplicate/conflicting work across instances

**Disadvantages:**
- Adds coordination complexity and a dependency on a reliable consensus mechanism
- Brief periods without a leader during failover/election
- Risk of split-brain if the election mechanism isn't robust

**When to Use:** Tasks that must run exactly once across a fleet of redundant instances (schedulers, coordinators).
**When NOT to Use:** Stateless, idempotent tasks safe to run redundantly, or single-instance deployments.
**Related Patterns:** Health Endpoint Monitoring, Scheduler Agent Supervisor.

---

## 3. Data Management Patterns

### 3.1 Cache-Aside
**Short Description:** Load data into a cache on demand from the underlying data store, rather than always keeping the cache pre-populated.

**Problem It Solves:** Repeated reads directly against the primary data store are slow/expensive; you want to reduce load and latency without the complexity of a write-through cache for all data.

**Use Case:** A web app checks Redis first; on a cache miss, it reads from the database, stores the result in the cache, and returns it. Writes update the DB and invalidate/update the cache entry.

**Advantages:**
- Improves read performance and reduces DB load
- Cache only holds data actually requested (efficient memory use)
- Resilient to cache failures — app can still fall back to the data store

**Disadvantages:**
- Risk of stale data if invalidation isn't handled correctly
- Cache-stampede risk on a popular key expiring (many requests hit the DB simultaneously)
- Adds complexity: two places to keep (loosely) in sync

**When to Use:** Read-heavy workloads with data that doesn't need instant/strict consistency.
**When NOT to Use:** Write-heavy workloads, or data requiring strong consistency guarantees at every read.
**Related Patterns:** Materialized View, Index Table.

---

### 3.2 CQRS (Command Query Responsibility Segregation)
**Short Description:** Separate the models/interfaces used for reading data (queries) from those used for updating data (commands).

**Problem It Solves:** A single data model optimized for both reads and writes often satisfies neither well — complex domain writes need rich validation/business logic, while reads need fast, denormalized shapes for display. Using one model creates contention and compromises.

**Use Case:** An e-commerce system uses a normalized write model enforcing business rules for order commands, while a separate denormalized read model (possibly a different, replicated data store) powers fast product/catalog queries and reporting.

**Advantages:**
- Read and write sides can be scaled and optimized independently
- Simplifies complex domain models by isolating command validation logic
- Enables using different data stores/schemas per side (e.g., SQL for writes, a document store for reads)

**Disadvantages:**
- Adds significant architectural complexity — not appropriate for simple CRUD apps
- Eventual consistency between write and read models (if physically separated) must be handled by the UI/UX
- More moving parts to build, test, and operate

**When to Use:** Complex domains with very different read vs. write performance/scale needs, or high-contention write models.
**When NOT to Use:** Simple CRUD applications — CQRS here is pure overhead.
**Related Patterns:** Event Sourcing (very commonly paired with CQRS), Materialized View.

---

### 3.3 Event Sourcing
**Short Description:** Instead of storing just current state, store the full sequence of state-changing events as an append-only log; current state is derived by replaying events.

**Problem It Solves:** Traditional CRUD storage loses history — you can't easily see *how* the current state was reached, audit changes, or reconstruct past states, and concurrent updates to the same record can cause conflicts.

**Use Case:** A banking ledger stores every deposit/withdrawal event rather than just a running balance, enabling full audit trails, replay/debugging, and rebuilding read models.

**Advantages:**
- Complete audit trail / history is a natural byproduct
- Enables temporal queries (state "as of" any point in time)
- Decouples write model from read model — natural fit with CQRS
- Facilitates event-driven integration with other systems

**Disadvantages:**
- Steeper learning curve and higher architectural complexity
- Querying "current state" requires replay or maintained projections (materialized views)
- Event schema evolution/versioning is a real long-term challenge
- Storage grows indefinitely unless snapshotting/archival is implemented

**When to Use:** Domains needing full audit history, complex business workflows, or natural event-driven integration (finance, e-commerce order lifecycle).
**When NOT to Use:** Simple CRUD apps with no audit/history requirements — the complexity isn't justified.
**Related Patterns:** CQRS, Materialized View, Saga.

---

### 3.4 Materialized View
**Short Description:** Precompute and store query-optimized "views" over data in one or more stores, when the underlying data's native format is poorly suited to required queries.

**Problem It Solves:** Computing complex joins/aggregations on-the-fly, on every query, against normalized or event-sourced data is expensive and slow.

**Use Case:** An e-commerce dashboard needs "total sales by category by day" — instead of aggregating raw transaction data on every page load, a materialized view is pre-built and refreshed periodically or via events.

**Advantages:**
- Dramatically improves read/query performance for complex aggregations
- Reduces load on the primary transactional data store
- Works well combined with Event Sourcing/CQRS to build read-optimized projections

**Disadvantages:**
- View can become stale depending on refresh strategy (eventual consistency)
- Extra storage cost for duplicated/derived data
- Additional complexity in keeping the view synchronized with source changes

**When to Use:** Read-heavy reporting/dashboard scenarios, or data stored in a format unsuited to frequent query patterns.
**When NOT to Use:** Data that changes so frequently that view refresh overhead outweighs the query savings.
**Related Patterns:** CQRS, Event Sourcing, Cache-Aside, Index Table.

---

### 3.5 Index Table
**Short Description:** Create secondary indexes over fields in a data store that are frequently referenced by queries but aren't part of the primary key.

**Problem It Solves:** NoSQL/partitioned stores often only efficiently query by primary/partition key; querying by other attributes forces a full scan, which is slow and expensive.

**Use Case:** A table partitioned by `CustomerId` also needs fast lookups by `Email` — a separate index table maps `Email → CustomerId` to avoid scanning the whole dataset.

**Advantages:**
- Enables efficient queries on non-primary-key attributes
- Avoids expensive full-table scans
- Can be tailored per query pattern (multiple index tables for multiple access paths)

**Disadvantages:**
- Extra storage and write overhead (every write must update index tables too)
- Consistency between the index and the primary data must be actively managed
- Added complexity in write-path logic

**When to Use:** NoSQL/partitioned data stores where common queries don't align with the partition key.
**When NOT to Use:** Relational databases with native secondary index support, or infrequent alternate-key queries where a scan is acceptable.
**Related Patterns:** Materialized View, Sharding.

---

### 3.6 Sharding
**Short Description:** Divide a data store into horizontal partitions (shards), each holding a subset of the data, typically distributed across multiple servers/nodes.

**Problem It Solves:** A single data store instance can't scale to hold or serve the required data volume/throughput; you need to horizontally distribute both storage and load.

**Use Case:** A multi-tenant SaaS database is sharded by `TenantId`, or a global user database is sharded by geographic region or a hash of `UserId`.

**Advantages:**
- Enables horizontal scalability beyond a single machine's capacity
- Can improve performance by parallelizing across shards
- Supports data residency/locality requirements (regional shards)

**Disadvantages:**
- Cross-shard queries/joins become complex and expensive
- Rebalancing shards as data grows unevenly ("hot shards") is operationally hard
- Increases overall system complexity (routing, shard key selection is critical and hard to change later)

**When to Use:** Very large datasets or high-throughput workloads exceeding single-instance capacity; strong tenant/data isolation needs.
**When NOT to Use:** Data volumes/throughput well within a single instance's capability — sharding prematurely adds needless complexity.
**Related Patterns:** Index Table, Deployment Stamps, Geode.

---

## 4. Messaging Patterns

### 4.1 Publisher-Subscriber (Pub/Sub)
**Short Description:** Enable an application to announce events to multiple interested consumers asynchronously, without the sender needing to know who (or how many) receivers exist.

**Problem It Solves:** Direct point-to-point integration between a producer and every consumer creates tight coupling — adding a new consumer requires changing the producer.

**Use Case:** An "OrderPlaced" event is published to a topic (Azure Service Bus Topics, Event Grid); Inventory, Shipping, and Notification services each subscribe independently without the Order service knowing about them.

**Advantages:**
- Decouples producers from consumers completely
- Easy to add new subscribers without touching the publisher
- Naturally supports event-driven, reactive architectures

**Disadvantages:**
- Harder to trace/debug end-to-end flow across many independent subscribers
- Eventual consistency — subscribers process asynchronously
- Requires careful handling of message delivery guarantees (at-least-once, ordering, duplicates)

**When to Use:** Event-driven architectures, systems needing loose coupling between many independent consumers.
**When NOT to Use:** Simple direct request/response interactions between two known parties.
**Related Patterns:** Choreography, Competing Consumers, Event Sourcing.

---

### 4.2 Choreography
**Short Description:** Let each service decide independently when and how to process part of a business operation by reacting to events, instead of a central orchestrator dictating the sequence.

**Problem It Solves:** A central orchestrator for every business process becomes a bottleneck, a single point of failure, and a tightly-coupled "God" component that knows too much about every service.

**Use Case:** In an order fulfillment flow, the Order service emits "OrderCreated"; Payment reacts and emits "PaymentProcessed"; Shipping reacts to that in turn — no central coordinator.

**Advantages:**
- Highly decoupled — services don't need to know about each other
- No single point of failure/bottleneck from a central orchestrator
- Scales well organically as new services subscribe to relevant events

**Disadvantages:**
- Harder to see/understand the overall process flow (no central definition)
- Debugging and tracing distributed event chains is difficult
- Risk of implicit, hard-to-track coupling through event contracts

**When to Use:** Highly decoupled systems where individual teams/services own their reactions independently.
**When NOT to Use:** Complex workflows needing clear visibility/control over sequencing — consider Orchestration (Scheduler Agent Supervisor / orchestrated Saga) instead.
**Related Patterns:** Publisher-Subscriber, Saga, Scheduler Agent Supervisor.

---

### 4.3 Claim Check
**Short Description:** Split a large message into a small "claim check" (reference/token) sent through the message bus and the actual payload stored separately (e.g., Blob Storage).

**Problem It Solves:** Message brokers have size limits and perform poorly with large payloads; sending large data (files, images, big JSON blobs) directly through the bus is inefficient and costly.

**Use Case:** A message includes a reference/URL to a large file in Blob Storage instead of embedding the file itself; the consumer retrieves the payload using the claim check when needed.

**Advantages:**
- Keeps messages small, improving broker performance and throughput
- Avoids message-size limits imposed by messaging platforms
- Separates storage concerns (cheap blob storage) from messaging concerns (optimized for small messages)

**Disadvantages:**
- Extra round-trip to retrieve the payload adds latency
- Requires managing lifecycle/cleanup of stored payloads (avoid orphaned blobs)
- Added complexity vs. simply sending the payload inline

**When to Use:** Messaging workloads involving large payloads (files, images, big documents).
**When NOT to Use:** Small message payloads well within broker limits — unnecessary indirection otherwise.
**Related Patterns:** Publisher-Subscriber, Queue-Based Load Leveling.

---

### 4.4 Priority Queue
**Short Description:** Prioritize requests so that higher-priority messages are processed ahead of lower-priority ones, rather than strict FIFO.

**Problem It Solves:** Not all requests are equally urgent — treating them all with equal priority means critical operations can be delayed behind less important ones during high load.

**Use Case:** A support ticketing system processes "Critical" severity tickets before "Low" severity ones, using separate queues per priority or a priority field with worker logic that checks high-priority queues first.

**Advantages:**
- Ensures critical/urgent work is processed promptly even under load
- Enables service-tiering (e.g., premium customers get faster processing)
- Flexible: can implement via separate queues per priority or metadata-based sorting

**Disadvantages:**
- Risk of starvation for low-priority messages if high-priority volume is constant
- Added complexity in queue/consumer design
- Requires careful fairness/aging strategies to avoid indefinitely delayed low-priority items

**When to Use:** Systems with clearly differentiated urgency/service tiers among requests.
**When NOT to Use:** All requests are truly equal in importance — added complexity has no payoff.
**Related Patterns:** Queue-Based Load Leveling, Throttling, Competing Consumers.

---

### 4.5 Messaging Bridge
**Short Description:** Build an intermediary component that translates and relays messages between two otherwise-incompatible messaging systems.

**Problem It Solves:** Different systems (legacy vs. modern, different vendors/protocols) use incompatible messaging technologies, but they still need to exchange messages/events reliably.

**Use Case:** A legacy on-premises MSMQ-based system needs to communicate with a modern Azure Service Bus–based cloud system during a phased migration; a bridge component translates and forwards messages both ways.

**Advantages:**
- Enables integration across heterogeneous messaging technologies without rewriting either system
- Useful transitional tool during migrations (pairs well with Strangler Fig)
- Isolates protocol-translation complexity into one component

**Disadvantages:**
- Adds a new component that can become a bottleneck or single point of failure
- Ongoing maintenance burden, especially if intended to be temporary but becomes permanent
- Potential for message format/semantic mismatches if translation logic isn't carefully designed

**When to Use:** Migrations or integrations requiring bridging incompatible messaging systems.
**When NOT to Use:** Both systems already share a common messaging technology/protocol.
**Related Patterns:** Strangler Fig, Anti-Corruption Layer, Publisher-Subscriber.

---

### 4.6 Asynchronous Request-Reply
**Short Description:** Decouple back-end processing (which may be long-running/asynchronous) from a front-end host, while still giving the front end a clear, timely initial response.

**Problem It Solves:** Some operations take too long to complete within a typical synchronous HTTP request timeout, but clients still need immediate acknowledgment and a way to check on/receive the eventual result.

**Use Case:** A client submits a long-running report-generation request; the API immediately returns `202 Accepted` with a status URL, and the client polls (or is notified via webhook/SignalR) when the report is ready.

**Advantages:**
- Avoids client-side timeouts on long-running operations
- Improves perceived responsiveness (immediate ack)
- Decouples front-end request lifecycle from back-end processing time

**Disadvantages:**
- More complex client logic (polling, or handling callback/webhook notifications)
- Requires a mechanism to track and expose operation status (status store, correlation IDs)
- Not appropriate when a synchronous answer is truly required immediately

**When to Use:** Long-running back-end operations invoked from front ends expecting a quick initial response (report generation, batch processing, file conversion).
**When NOT to Use:** Fast operations that comfortably complete within normal request timeouts.
**Related Patterns:** Queue-Based Load Leveling, Compute Resource Consolidation.

---

## 5. Design & Implementation Patterns

### 5.1 Anti-Corruption Layer
**Short Description:** Implement a façade/adapter layer between a modern application and a legacy (or otherwise incompatible) system, so the modern system's model isn't "corrupted" by the legacy system's model.

**Problem It Solves:** Directly integrating with a legacy system's data model, terminology, or APIs pollutes the new system's clean domain model with legacy quirks, workarounds, and inconsistent semantics.

**Use Case:** A new microservices-based order system integrates with a decades-old mainframe inventory system via an anti-corruption layer that translates the mainframe's field codes and quirky data formats into the new system's clean domain model.

**Advantages:**
- Keeps the new system's domain model clean and legacy-free
- Isolates legacy quirks/technical debt in one well-defined boundary
- Facilitates incremental modernization (pairs with Strangler Fig)

**Disadvantages:**
- Adds a translation layer that must be built and maintained
- Can become a performance bottleneck if translation logic is heavy
- Risk of the ACL itself accumulating complexity over time if not disciplined

**When to Use:** Integrating a modern system with legacy or third-party systems that have incompatible/messy domain models.
**When NOT to Use:** Systems with clean, compatible, well-designed APIs already — an ACL would be pure overhead.
**Related Patterns:** Strangler Fig, Messaging Bridge.

---

### 5.2 Strangler Fig
**Short Description:** Incrementally migrate a legacy system by gradually replacing specific pieces of functionality with new applications/services, until the legacy system is "strangled" and can be retired.

**Problem It Solves:** A full rewrite of a large legacy system ("big bang" migration) is extremely risky, expensive, and often fails outright; you need a way to modernize incrementally with lower risk.

**Use Case:** A facade/router sits in front of a legacy monolith; new functionality (or rewritten modules) is built as separate services, and the facade gradually routes more and more traffic to the new services while legacy code is decommissioned piece by piece.

**Advantages:**
- Dramatically lowers migration risk vs. a big-bang rewrite
- Delivers incremental value — new features ship on new architecture right away
- Allows the legacy and new systems to coexist safely during transition

**Disadvantages:**
- Migration can take a long time, requiring sustained organizational commitment
- Running both systems in parallel (temporarily) adds operational overhead
- Routing/facade logic itself needs careful design (often needs an Anti-Corruption Layer)

**When to Use:** Modernizing large, risky legacy systems where a full rewrite is infeasible.
**When NOT to Use:** Small legacy systems where a full rewrite is genuinely lower-risk and faster.
**Related Patterns:** Anti-Corruption Layer, Messaging Bridge, Gateway Routing.

---

### 5.3 Pipes and Filters
**Short Description:** Break down a complex processing task into a series of discrete, reusable, independent steps (filters) connected by channels (pipes), each filter transforming or processing the data before passing it on.

**Problem It Solves:** A monolithic processing task is hard to reuse, test, scale, or modify piece-by-piece; you want composable, independently deployable/scalable processing stages.

**Use Case:** An image-processing pipeline: resize → watermark → compress → upload, each stage implemented as an independent filter that can be reused in other pipelines or scaled independently.

**Advantages:**
- Encourages reuse of individual processing steps across different pipelines
- Each filter can be independently developed, tested, deployed, and scaled
- Improves maintainability by isolating each transformation's logic

**Disadvantages:**
- Adds overhead from data serialization/transport between filters/stages
- End-to-end latency can grow with more stages
- Error handling across a multi-stage pipeline needs careful design

**When to Use:** Complex processing broken into naturally sequential, reusable steps (ETL pipelines, media processing, data transformation).
**When NOT to Use:** Simple, single-step processing where pipeline overhead isn't justified.
**Related Patterns:** Queue-Based Load Leveling, Competing Consumers, Compute Resource Consolidation.

---

### 5.4 Sidecar
**Short Description:** Deploy components of an application into a separate process or container (running alongside the main application) to provide supporting features like monitoring, logging, or configuration, in isolation.

**Problem It Solves:** Cross-cutting concerns (logging, monitoring, config, proxying) tightly coupled into the main app's codebase force it into a single language/runtime and complicate independent updates of these concerns.

**Use Case:** A service mesh proxy (like Envoy in Istio) runs as a sidecar container next to each application container in Kubernetes, handling mTLS, retries, and metrics collection transparently.

**Advantages:**
- Language/runtime-agnostic — sidecar can be written independently of the main app
- Cross-cutting concerns can be updated/deployed independently of the main application
- Encourages single-responsibility within the main app's codebase

**Disadvantages:**
- Adds resource overhead (extra container/process per app instance)
- Slight latency overhead for calls routed through the sidecar
- Requires orchestration platform support (e.g., Kubernetes pods) to deploy effectively

**When to Use:** Microservices/containerized environments needing consistent cross-cutting infrastructure (service mesh, logging agents).
**When NOT to Use:** Simple monolithic deployments, or environments without container/pod-level co-location support.
**Related Patterns:** Ambassador (a specialization of Sidecar), Gateway Offloading.

---

### 5.5 Compute Resource Consolidation
**Short Description:** Consolidate multiple tasks or operations into a single computational unit to increase compute density and reduce cost/overhead.

**Problem It Solves:** Running many small, lightly-loaded workloads on separate dedicated compute instances wastes resources and increases operational/management overhead and cost.

**Use Case:** Several low-traffic microservices are consolidated onto a shared Kubernetes cluster or App Service Plan rather than each having its own dedicated, mostly-idle VM.

**Advantages:**
- Improves compute utilization and lowers infrastructure cost
- Reduces the number of things to patch, monitor, and manage
- Simplifies scaling decisions by pooling resources

**Disadvantages:**
- Reduces isolation — a noisy/faulty workload can affect co-located workloads (mitigate with Bulkhead)
- Harder to reason about per-workload capacity and failure domains
- Requires strong monitoring/quota enforcement to avoid one tenant starving others

**When to Use:** Many small, low-utilization workloads suitable for sharing infrastructure.
**When NOT to Use:** Workloads with strict isolation, security, or compliance requirements demanding dedicated infrastructure.
**Related Patterns:** Bulkhead, Deployment Stamps, Sidecar.

---

## 6. Deployment & Scaling Patterns

### 6.1 Deployment Stamps
**Short Description:** Deploy multiple independent copies ("stamps") of application components — including data stores — often per customer, region, or scale unit.

**Problem It Solves:** A single shared deployment eventually hits scale limits (compute, data store, noisy-neighbor effects) or can't meet per-customer isolation/compliance/data-residency requirements.

**Use Case:** A SaaS provider deploys a full independent "stamp" (app + database) per large enterprise customer, or per geographic region, to isolate load and meet data residency requirements.

**Advantages:**
- Provides strong isolation between customers/regions (blast-radius containment)
- Enables near-linear scaling by adding more stamps as needed
- Supports data residency/compliance requirements naturally

**Disadvantages:**
- Significant operational overhead — many independent deployments to manage, patch, and monitor
- Cross-stamp reporting/analytics becomes harder
- Higher infrastructure cost vs. shared multi-tenant deployment

**When to Use:** SaaS platforms needing strong tenant isolation, data residency compliance, or scale beyond single-deployment limits.
**When NOT to Use:** Small-scale systems where a shared multi-tenant deployment is simpler and cheaper.
**Related Patterns:** Sharding, Geode, Bulkhead.

---

### 6.2 Geode
**Short Description:** Deploy back-end services across multiple geographically distributed nodes ("geodes"), where each node can independently serve client requests from any region.

**Problem It Solves:** A single-region deployment creates high latency for distant users and a single point of regional failure; you need active-active multi-region capability.

**Use Case:** A globally distributed application deploys identical service+data stamps in multiple Azure regions, with traffic routed to the nearest healthy geode (via Azure Traffic Manager/Front Door), and data replicated across geodes.

**Advantages:**
- Reduces latency for globally distributed users (serve from the nearest region)
- Improves resiliency — a regional outage doesn't take down the whole service
- Enables active-active high availability across regions

**Disadvantages:**
- Data replication/consistency across geodes is genuinely hard (conflict resolution, latency)
- Significantly higher cost and operational complexity vs. single-region deployment
- Requires careful global traffic routing and failover design

**When to Use:** Global applications with geographically dispersed users needing low latency and multi-region resiliency.
**When NOT to Use:** Regional/local applications where all users are close to a single deployment region.
**Related Patterns:** Deployment Stamps, Sharding.

---

## 7. Security & Identity Patterns

### 7.1 Federated Identity
**Short Description:** Delegate authentication to an external identity provider (IdP) rather than the application managing its own user credentials/directory.

**Problem It Solves:** Applications managing their own user credential stores carry heavy security burden (password storage, breach risk) and force users to manage yet another set of credentials.

**Use Case:** An enterprise app uses Microsoft Entra ID (Azure AD) for single sign-on (SSO), or a consumer app allows "Sign in with Google/Microsoft" instead of maintaining its own username/password store.

**Advantages:**
- Removes the burden (and risk) of storing/managing credentials directly
- Enables SSO — better user experience across multiple related applications
- Leverages the identity provider's stronger security investment (MFA, breach detection)

**Disadvantages:**
- Creates a dependency on the external identity provider's availability
- Federation protocols (OAuth2/OIDC/SAML) add integration complexity
- Loss of some control over the authentication experience/policy

**When to Use:** Enterprise apps needing SSO, or consumer apps wanting to reduce credential-management burden.
**When NOT to Use:** Extremely simple, isolated internal tools with no need for SSO or external identity integration.
**Related Patterns:** Valet Key, Gatekeeper.

---

### 7.2 Valet Key
**Short Description:** Use a limited-scope, time-bound token/key to give clients restricted, direct access to a specific resource, without routing all traffic through the application.

**Problem It Solves:** Routing every file upload/download through the application server for authorization wastes application compute/bandwidth on operations a storage service could handle directly.

**Use Case:** A client requests a file-upload URL from the app; the app returns a short-lived, scoped SAS (Shared Access Signature) token for Azure Blob Storage, and the client uploads directly to storage — bypassing the app server entirely.

**Advantages:**
- Offloads bandwidth-heavy operations (uploads/downloads) from the application tier
- Reduces cost and improves scalability (storage service handles the heavy lifting)
- Time-limited, scoped tokens minimize security exposure

**Disadvantages:**
- Requires careful token scoping/expiration to avoid overly broad or long-lived access
- Harder to apply custom business logic/validation to the direct resource access
- Revoking access mid-flight can be tricky depending on the token mechanism

**When to Use:** Direct client access to storage/resources (file uploads/downloads, media streaming) where routing through the app adds no value.
**When NOT to Use:** Operations requiring per-request business logic/validation that can't be delegated to the storage layer.
**Related Patterns:** Gatekeeper, Gateway Offloading, Static Content Hosting.

---

### 7.3 Quarantine
**Short Description:** Ensure that external assets (files, data, dependencies) meet an agreed quality/security bar before the workload consumes or trusts them.

**Problem It Solves:** Blindly trusting externally-sourced files/data (user uploads, third-party feeds) exposes the system to malware, corrupted data, or malicious payloads.

**Use Case:** User-uploaded files are placed in a "quarantine" storage container and scanned (e.g., via Microsoft Defender for Storage / antivirus scanning) before being moved to the "trusted" container that application logic actually reads from.

**Advantages:**
- Prevents malicious or malformed external content from directly reaching production systems
- Provides a clear, auditable checkpoint for compliance and security policy
- Can be extended to schema/data-quality validation, not just security scanning

**Disadvantages:**
- Adds processing latency between upload and availability for use
- Requires infrastructure/tooling for scanning and quarantine-to-trusted promotion
- False positives can block legitimate content, requiring an appeals/override process

**When to Use:** Any system accepting external, untrusted content (user uploads, third-party integrations, partner data feeds).
**When NOT to Use:** Fully trusted, internally-generated content with no external input.
**Related Patterns:** Gatekeeper, Anti-Corruption Layer.

---

## Combining Patterns

Patterns rarely stand alone in a real architecture — you typically layer several together to address the multiple concerns a workload faces simultaneously. Common combinations:

| Combination | Why |
|---|---|
| **Retry + Circuit Breaker** | Retry transient faults, but stop retrying (fail fast) once faults persist, protecting the caller and the callee. |
| **Queue-Based Load Leveling + Competing Consumers** | Buffer bursty load in a queue, then scale out consumers to process it in parallel. |
| **Gateway Routing + Gateway Aggregation + Gateway Offloading** | Layer all three behind one gateway: route requests, combine calls, and offload TLS/auth — a full API Gateway. |
| **Saga + Compensating Transaction** | Saga orchestrates/choreographs a multi-step process; each step's rollback is implemented via Compensating Transaction. |
| **CQRS + Event Sourcing** | Event Sourcing provides the write-side event log; CQRS uses projections of that log to build fast, purpose-built read models. |
| **Strangler Fig + Anti-Corruption Layer** | Strangler Fig migrates incrementally; the ACL keeps the new system's domain model clean while talking to the legacy system during the transition. |
| **Sidecar + Ambassador** | Ambassador is a specific, network-focused flavor of the more general Sidecar pattern. |

### A note on Antipatterns
Microsoft's Architecture Center also maintains a companion catalog of **antipatterns** — practices that look reasonable initially (and often work fine at low scale) but degrade reliability, performance, or cost efficiency as load grows (e.g., the "Chatty I/O," "No Caching," "Synchronous I/O," and "Extraneous Fetching" antipatterns). Recognizing these helps you identify *which* pattern above is the right fix for an existing design smell. See: `https://learn.microsoft.com/en-us/azure/architecture/antipatterns/`

### How to actually choose a pattern (architect's checklist)
1. **Start from the problem, not the pattern name.** Identify the specific failure mode or bottleneck (e.g., "our data store can't handle read load," "a slow dependency is cascading failures").
2. **Map to the Well-Architected Framework pillar** most at risk: Reliability, Security, Cost Optimization, Operational Excellence, or Performance Efficiency. Most patterns above are tagged with the pillars they support.
3. **Check the trade-off you're accepting.** Every pattern here trades complexity, latency, storage, or consistency for a benefit — make sure that trade-off is one your team can operate long-term, not just implement once.
4. **Prefer composition over a "silver bullet."** Real systems combine 3–6 of these patterns, not one grand pattern that solves everything.
5. **Don't apply patterns speculatively.** If you don't have the problem yet (e.g., you don't have scale problems), don't add Sharding, CQRS, or Event Sourcing "just in case" — that complexity has a real, ongoing cost.

---

*Reference source: [Azure Architecture Center — Cloud Design Patterns](https://learn.microsoft.com/en-us/azure/architecture/patterns/). Commentary, advantages/disadvantages framing, and the "when to use" / "when not to use" guidance in this document are added architect-level analysis beyond what's in the source catalog.*
