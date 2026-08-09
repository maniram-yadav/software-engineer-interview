# Design a Multi-Tenant SaaS Database Architecture — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Support many independent customer organizations (tenants) sharing the same application
- Strict data isolation — one tenant must never see another tenant's data, even accidentally
- Support tenant-specific customization (custom fields, varying feature sets) without schema chaos
- Onboard new tenants quickly, and support offboarding/data export cleanly

### Non-Functional Requirements
- **Isolation (the paramount property):** A bug or query error must never leak cross-tenant data — this is both a security and contractual (SLA) requirement
- **Cost efficiency:** Running a fully separate database per tenant doesn't scale economically for a large number of small tenants
- **Noisy neighbor protection:** One tenant's heavy usage shouldn't degrade performance for others
- **Scalability:** Must accommodate both many small tenants and a few very large enterprise tenants within the same overall architecture

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Total tenants | ~50,000 (mix of small/medium/large) |
| Largest tenant data size | Could be 1000x+ larger than smallest |
| Total data volume | Multi-terabyte to petabyte scale |
| Queries/sec (platform-wide) | ~100,000+ |
| Tenant onboarding target | Minutes, not hours |

---

## 2. The Three Fundamental Isolation Models

```mermaid
flowchart TB
    A["Multi-Tenancy Data<br/>Isolation Strategy"] --> B["Silo Model<br/>(separate DB per tenant)"]
    A --> C["Bridge Model<br/>(shared DB, separate schema<br/>per tenant)"]
    A --> D["Pool Model<br/>(shared DB, shared schema,<br/>tenant_id column)"]

    B --> B1["Strongest isolation —<br/>physically separate storage"]
    B --> B2["Most expensive — doesn't<br/>scale economically to<br/>thousands of small tenants"]
    B --> B3["Best fit: large enterprise<br/>tenants with strict<br/>compliance requirements"]

    C --> C1["Moderate isolation —<br/>same DB instance,<br/>separate namespace"]
    C --> C2["Schema migrations must<br/>run across ALL tenant<br/>schemas — operationally<br/>heavier as tenant count grows"]
    C --> C3["Best fit: medium tenant count,<br/>moderate customization needs"]

    D --> D1["Weakest isolation<br/>(logical, via tenant_id) —<br/>but most cost-efficient"]
    D --> D2["Every query MUST filter<br/>by tenant_id — a single<br/>missed filter is a<br/>data leak"]
    D --> D3["Best fit: large numbers of<br/>small-to-medium tenants —<br/>most common SaaS choice"]
```

**This design uses a hybrid approach:** pool model (shared schema, `tenant_id` column) as the default for most tenants, with an escape hatch to the silo model (dedicated database) for large enterprise tenants who need it for compliance or performance isolation reasons — a pattern used by most successful large-scale SaaS platforms.

---

## 3. High-Level Architecture (Hybrid Model)

```mermaid
flowchart TB
    Client["Tenant Application Requests"]
    Gateway["API Gateway<br/>(resolves tenant from<br/>subdomain/JWT/API key)"]

    subgraph Routing["Tenant Routing Layer"]
        TenantRegistry[("Tenant Registry<br/>tenant_id → shard/DB location,<br/>tier, isolation model")]
        Router["Query Router"]
    end

    subgraph PoolTier["Pool Tier (shared DB, most tenants)"]
        SharedDB1[("Shared DB Shard 1<br/>tenant_id column,<br/>Row-Level Security")]
        SharedDB2[("Shared DB Shard 2")]
    end

    subgraph SiloTier["Silo Tier (dedicated, large tenants)"]
        DedicatedDBA[("Dedicated DB<br/>— Enterprise Tenant A")]
        DedicatedDBB[("Dedicated DB<br/>— Enterprise Tenant B")]
    end

    Client --> Gateway --> Router
    Router --> TenantRegistry
    TenantRegistry -->|"tenant tier lookup"| Router

    Router -->|"pool-tier tenant"| SharedDB1
    Router -->|"pool-tier tenant"| SharedDB2
    Router -->|"silo-tier tenant"| DedicatedDBA
    Router -->|"silo-tier tenant"| DedicatedDBB
```

**Key idea:** The Query Router is the critical component that decides, per request, where a given tenant's data actually lives. This indirection is what allows tenants to be **migrated** between tiers (e.g., a growing pool-tier tenant graduating to a dedicated silo) without any application code changes — only the registry entry needs updating.

---

## 4. Data Model (Pool Model — Shared Schema)

```mermaid
erDiagram
    TENANT {
        string tenant_id PK
        string name
        string tier "pool/silo"
        string db_shard_location
    }
    ORDER {
        string order_id PK
        string tenant_id FK "REQUIRED on every row"
        string customer_name
        float amount
    }
    CUSTOMER {
        string customer_id PK
        string tenant_id FK "REQUIRED on every row"
        string name
    }
```

**Key modeling rule:** In the pool model, `tenant_id` is a **mandatory, indexed column on every single table** — not an optional field. This isn't just a modeling convention; it's the foundational mechanism that makes logical isolation possible at all in a shared-schema architecture.

---

## 5. Enforcing Isolation — Row-Level Security (Defense in Depth)

```mermaid
flowchart TB
    A["Query issued by application code"] --> B{"Isolation Enforcement Layers"}

    B --> C["Layer 1: Application-level<br/>WHERE tenant_id = ? filter<br/>(added by ORM/query builder)"]
    C --> C1["Risk: a developer forgets<br/>this filter in a new<br/>query — HUMAN ERROR RISK"]

    B --> D["Layer 2: Database-level<br/>Row-Level Security (RLS) policy<br/>— DB engine itself enforces<br/>tenant_id filtering, even if<br/>the application query omits it"]
    D --> D1["This is the CRITICAL<br/>defense-in-depth layer —<br/>even a buggy or malicious<br/>query CANNOT return<br/>cross-tenant rows, because<br/>the database engine itself<br/>refuses to return them"]

    E["Both layers together:<br/>application-level filtering<br/>for query efficiency (avoid<br/>scanning other tenants' data<br/>unnecessarily), PLUS database-level<br/>RLS as the non-bypassable<br/>safety net"] --> D1
```

---

## 6. Tenant-Scoped Query Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client (Tenant A user)
    participant GW as API Gateway
    participant Router as Query Router
    participant Registry as Tenant Registry
    participant DB as Database (with RLS enabled)

    C->>GW: Request (JWT contains tenant_id=A)
    GW->>GW: Extract + validate tenant_id from JWT
    GW->>Router: Forward request with tenant_id=A

    Router->>Registry: Lookup: which shard/DB<br/>hosts tenant A?
    Registry-->>Router: Shard 2, pool tier

    Router->>DB: SET app.current_tenant = 'A'<br/>(session-level context)
    Router->>DB: SELECT * FROM orders<br/>WHERE tenant_id = 'A'<br/>(explicit application filter)

    DB->>DB: RLS policy ALSO enforces:<br/>only return rows WHERE<br/>tenant_id = current_setting('app.current_tenant')

    DB-->>Router: Rows (guaranteed tenant-A-only,<br/>enforced at TWO layers)
    Router-->>C: Response
```

---

## 7. Tenant Sharding Strategy (Distributing Pool-Tier Tenants)

```mermaid
flowchart TB
    A["50,000 pool-tier tenants —<br/>too many for one DB instance"] --> B{"Sharding Strategy"}

    B --> C["Hash-based: tenant_id → shard<br/>via consistent hashing"]
    C --> C1["Even distribution, but a tenant's<br/>data location isn't predictable<br/>from the tenant_id alone —<br/>requires the registry lookup"]

    B --> D["Range/Bucket-based:<br/>group tenants into<br/>size-aware buckets"]
    D --> D1["Allows deliberately packing<br/>many small tenants together<br/>and isolating a few<br/>large ones onto their<br/>own shard, even within<br/>the pool tier"]

    E["This design favors bucket-based<br/>assignment via the Tenant Registry,<br/>since it allows PROACTIVE<br/>rebalancing as tenants grow —<br/>a tenant outgrowing its shard<br/>can be migrated to a less-loaded<br/>one, or promoted to silo tier"] --> D1
```

---

## 8. Noisy Neighbor Mitigation

```mermaid
flowchart TB
    A["Tenant X on a shared shard<br/>suddenly runs expensive queries<br/>(e.g., generating a huge report)"] --> B{"Mitigation Layers"}

    B --> C["Per-tenant query timeout/<br/>resource limits<br/>(e.g., statement_timeout,<br/>connection pool caps<br/>per tenant)"]
    B --> D["Per-tenant rate limiting<br/>at the API Gateway<br/>(same pattern as the<br/>Rate Limiter design)"]
    B --> E["Monitoring: per-tenant<br/>resource usage tracking,<br/>enabling proactive migration<br/>of consistently heavy tenants<br/>to dedicated silo tier"]

    F["Ultimate mitigation:<br/>a tenant that CONSISTENTLY<br/>exceeds pool-tier resource<br/>norms is a candidate for<br/>migration to the silo tier —<br/>the tiering strategy itself<br/>is the long-term noisy<br/>neighbor solution"]
```

---

## 9. Tenant Migration Between Tiers (Pool → Silo)

```mermaid
sequenceDiagram
    participant Ops as Ops/Automated Trigger
    participant Registry as Tenant Registry
    participant PoolDB as Pool Shard (source)
    participant SiloDB as New Dedicated DB (target)
    participant Router as Query Router

    Ops->>Ops: Detect tenant X exceeds<br/>pool-tier resource thresholds<br/>(or enterprise contract requires silo)

    Ops->>SiloDB: Provision new dedicated<br/>database instance
    Ops->>PoolDB: Begin replicating tenant X's<br/>data to new silo DB<br/>(filtered by tenant_id)

    Note over PoolDB,SiloDB: Live replication continues<br/>while tenant remains<br/>fully operational on pool tier

    Ops->>Ops: Verify replication caught up<br/>(minimal lag)
    Ops->>Registry: Update tenant X's routing entry:<br/>tier=silo, location=SiloDB<br/>(atomic cutover)

    Router->>Registry: Next request for tenant X<br/>picks up NEW routing
    Router->>SiloDB: Subsequent requests now<br/>route to dedicated DB

    Ops->>PoolDB: After confirming cutover success,<br/>delete tenant X's data<br/>from pool shard
```

**Why this migration capability matters architecturally:** Designing the routing indirection (Tenant Registry) from day one — even before any tenant actually needs to migrate — is what makes this kind of live, zero-downtime tier migration possible later. Retrofitting this indirection after tenants are hard-wired to specific databases is significantly harder.

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Multi-Tenant SaaS DB HLD))
    Tenant Registry
      tenant_id to location mapping
      Tier classification
      Enables live migration
    Query Router
      Resolves tenant per request
      Routes to correct shard/DB
    Pool Tier
      Shared schema, tenant_id column
      Row-Level Security enforcement
      Cost-efficient for many small tenants
    Silo Tier
      Dedicated DB per tenant
      Strongest isolation
      For large/compliance-sensitive tenants
    RLS Policies
      Database-enforced isolation
      Non-bypassable safety net
    Noisy Neighbor Guards
      Per-tenant resource limits
      Migration triggers
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Isolation model | Hybrid (pool tier default, silo tier for large/enterprise tenants) | Balances cost-efficiency for the long tail of small tenants against strong isolation needs for large/compliance-sensitive ones |
| Isolation enforcement | Application filter + database Row-Level Security | Defense in depth — RLS provides a non-bypassable safety net even if application-level filtering has a bug |
| Tenant location tracking | Central Tenant Registry with indirection | Enables live tier migration and shard rebalancing without hardcoding tenant-to-database mappings in application code |
| Sharding for pool tier | Bucket-based (not pure hash) assignment | Allows deliberate packing/isolation decisions (e.g., keeping consistently heavy tenants separated) rather than purely random distribution |
| Noisy neighbor handling | Per-tenant resource limits + proactive tier migration | Technical limits provide immediate protection; migration provides the long-term structural solution for consistently heavy tenants |

---

## 12. Bottlenecks & Scaling Considerations

- **Schema migrations across shared-schema pool tier** — a schema change must be applied consistently across all pool-tier shards; needs careful migration tooling (e.g., online schema change tools) to avoid downtime affecting many tenants simultaneously.
- **RLS performance overhead** — row-level security policies add a filtering condition to every query at the database engine level; while essential for safety, this must be accounted for in query performance tuning and indexing strategy (tenant_id should always be part of relevant indexes).
- **Hot pool-tier shards** — even with bucket-based assignment, uneven tenant growth can create imbalanced shards over time; requires ongoing monitoring and periodic rebalancing (which itself needs the same live-migration capability described for pool-to-silo moves).
- **Cross-tenant analytics/reporting** — platform-wide analytics (e.g., "total revenue across all tenants") become more complex in a sharded, multi-tier architecture; typically requires a separate data warehouse/ETL pipeline aggregating across all shards and silos, rather than querying operational databases directly.
- **Backup/restore complexity** — silo-tier tenants need independent backup schedules; pool-tier backups must support tenant-level granular restore (e.g., "restore only tenant X's data from this backup") which is harder than a simple full-database restore.
- **Tenant offboarding (GDPR-style deletion)** — cleanly and completely removing one tenant's data from a shared-schema pool database (without affecting others) requires careful, tenant_id-scoped deletion logic and verification — this is directly related to the "right to be forgotten" challenge covered in the GDPR compliance system design.
- **Testing isolation guarantees** — given that isolation is the single most safety-critical property of this entire system, automated tests that deliberately attempt cross-tenant data access (and verify it's blocked) should be a standing, continuously-run part of the test suite, not a one-time verification.
