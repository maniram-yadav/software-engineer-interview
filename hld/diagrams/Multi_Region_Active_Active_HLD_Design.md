# Design a Multi-Region Active-Active Deployment with Automated Failover — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Run the platform simultaneously "live" (active) across multiple geographic regions, not just one primary with passive standbys
- Serve user traffic from the nearest/best-performing region under normal conditions
- Automatically detect a regional failure and reroute affected traffic to healthy regions
- Support data consistency across regions appropriate to each data type's actual requirements

### Non-Functional Requirements
- **RTO (Recovery Time Objective):** How quickly must the system recover full functionality after a regional failure? Target: seconds to low minutes, not hours
- **RPO (Recovery Point Objective):** How much data loss is acceptable in the worst case? Target: near-zero for critical data, small bounded window for less-critical data
- **True active-active, not active-passive:** ALL regions handle real production traffic continuously, not sitting idle as pure backups — this maximizes infrastructure utilization but requires solving harder consistency problems
- **Graceful degradation:** A regional failure should degrade capacity/performance, not cause a complete platform outage

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Active regions | 3+ (allows continued majority/quorum operation even if one fails) |
| RTO target | < 60 seconds for automated failover |
| RPO target | Near-zero for financial/critical data, seconds for less-critical data |
| Cross-region replication latency | 50-150ms depending on geographic distance |

---

## 2. Active-Active vs Active-Passive — The Fundamental Choice

```mermaid
flowchart TB
    A["Active-Passive:<br/>ONE region handles ALL<br/>production traffic; OTHER<br/>region(s) sit idle,<br/>continuously replicating<br/>data, ready to take over"] --> A1["PRO: simpler consistency<br/>model — only one region<br/>ever accepts writes at a time<br/>CON: wastes significant<br/>infrastructure capacity<br/>(idle standby regions);<br/>failover still requires<br/>PROMOTING the standby,<br/>which takes real time"]

    B["Active-Active:<br/>MULTIPLE regions<br/>simultaneously serve REAL<br/>production traffic,<br/>continuously"] --> B1["PRO: fully utilizes all<br/>regional infrastructure;<br/>failover is often just<br/>REROUTING traffic away<br/>from the failed region —<br/>the remaining regions are<br/>ALREADY live and serving,<br/>no promotion delay<br/>CON: requires solving<br/>genuinely harder<br/>multi-writer consistency<br/>problems, since MULTIPLE<br/>regions accept writes<br/>simultaneously"]

    C["This design targets true<br/>active-active — the harder<br/>but more resilient and<br/>resource-efficient approach"] -.-> B1
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph GlobalLayer["Global Traffic Management"]
        GlobalDNS["Global DNS<br/>(health-based routing,<br/>as in the dedicated design)"]
    end

    subgraph RegionUS["Region: US-East (Active)"]
        LBUSEast["Regional Load Balancer"]
        AppUSEast["Application Servers"]
        DataUSEast[("Regional Data Store<br/>(active writes accepted)")]
    end

    subgraph RegionEU["Region: EU-West (Active)"]
        LBEU["Regional Load Balancer"]
        AppEU["Application Servers"]
        DataEU[("Regional Data Store<br/>(active writes accepted)")]
    end

    subgraph RegionAPAC["Region: APAC (Active)"]
        LBAPAC["Regional Load Balancer"]
        AppAPAC["Application Servers"]
        DataAPAC[("Regional Data Store<br/>(active writes accepted)")]
    end

    UserUS["User (US)"] --> GlobalDNS --> LBUSEast --> AppUSEast --> DataUSEast
    UserEU["User (EU)"] --> GlobalDNS --> LBEU --> AppEU --> DataEU
    UserAPAC["User (APAC)"] --> GlobalDNS --> LBAPAC --> AppAPAC --> DataAPAC

    DataUSEast <-->|"cross-region<br/>replication"| DataEU
    DataEU <-->|"cross-region<br/>replication"| DataAPAC
    DataUSEast <-->|"cross-region<br/>replication"| DataAPAC
```

**Key idea:** Global DNS (the earlier dedicated design) directs each user to their geographically nearest healthy region under normal conditions — but critically, EVERY region is simultaneously capable of handling ANY user's traffic, with data continuously replicated across all regions. This is what makes failover fast: rerouting traffic doesn't require "waking up" a dormant region, since all regions are already actively serving traffic and holding replicated data.

---

## 4. Data Consistency Strategy — Matching Model to Data Type

```mermaid
flowchart TB
    A["Not all data needs the<br/>SAME consistency guarantee<br/>across regions — this is the<br/>SAME core insight explored<br/>in the Linearizability vs<br/>Eventual Consistency design,<br/>applied specifically to<br/>MULTI-REGION architecture"] --> B{"Data Type"}

    B --> C["Financial/critical data<br/>(account balances, orders)"]
    C --> C1["Requires STRONG consistency —<br/>use the approach from the<br/>Multi-Region Strong Consistency<br/>design: designate ONE region<br/>as the writer for a given<br/>data shard, even in an<br/>otherwise active-active<br/>deployment"]

    B --> D["User-generated content,<br/>engagement data (likes,<br/>views, comments)"]
    D --> D1["Eventual consistency<br/>acceptable — EVERY region<br/>can accept writes locally,<br/>using CRDT-based or<br/>conflict-resolution<br/>strategies (as in the CRDT<br/>Counter and Vector Clock<br/>designs) to reconcile<br/>across regions asynchronously"]

    B --> E["Session/cache data"]
    E --> E1["Region-local only — no<br/>cross-region replication<br/>needed at all; a user's<br/>session simply gets recreated<br/>if they're rerouted to a<br/>different region"]

    F["A real active-active system<br/>is NOT one single consistency<br/>model applied uniformly —<br/>it's a DELIBERATE portfolio<br/>of different strategies,<br/>chosen per data type"] -.-> E1
```

---

## 5. Regional Failure Detection & Failover — Detailed Sequence

```mermaid
sequenceDiagram
    participant HealthCheck as Global Health Monitoring
    participant RegionUS as US-East Region<br/>(experiences failure)
    participant DNS as Global DNS
    participant DataStores as Other Regions'<br/>Data Stores
    participant Users as Affected Users

    Note over RegionUS: Regional infrastructure<br/>failure (e.g., datacenter<br/>network issue)

    HealthCheck->>RegionUS: Health check<br/>(multiple vantage points,<br/>same consensus principle<br/>as the Global DNS design)
    Note over RegionUS: No response

    HealthCheck->>HealthCheck: Confirm failure<br/>(multiple consecutive<br/>failed checks, avoiding<br/>false positives)

    HealthCheck->>DNS: Mark US-East as UNHEALTHY

    Note over Users: Users' cached DNS answers<br/>(short TTL) expire naturally
    Users->>DNS: Fresh DNS query
    DNS-->>Users: Reroute to next-nearest<br/>HEALTHY region (e.g., EU-West)

    Users->>RegionUS: (New requests go to<br/>EU-West instead)

    Note over DataStores: EU-West already has<br/>REPLICATED data from<br/>US-East (via ongoing<br/>cross-region replication)<br/>— no data "recovery" step<br/>needed, just continued<br/>service from already-current<br/>replicated state
```

**Why this failover is fast compared to active-passive:** Because EU-West was ALREADY actively serving its own regional traffic and ALREADY had continuously-replicated data, there's no "promote standby to primary" step required — rerouting is purely a traffic-direction change (handled by the Global DNS design's health-based routing), not an infrastructure activation process.

---

## 6. Handling Writes for Strongly-Consistent Data During Normal Operation

```mermaid
sequenceDiagram
    participant UserEU as User (in EU, but their<br/>account data's designated<br/>writer region is US-East)
    participant AppEU as EU Application Server
    participant DataEU as EU Data Store<br/>(read replica for this shard)
    participant DataUS as US-East Data Store<br/>(designated writer for this shard)

    UserEU->>AppEU: Update account balance

    AppEU->>DataEU: Check: is this data's<br/>writer region US-East?
    DataEU-->>AppEU: Yes — forward write<br/>cross-region

    AppEU->>DataUS: Forward write request<br/>(cross-region hop,<br/>same tradeoff as the<br/>Multi-Region Strong<br/>Consistency design)
    DataUS->>DataUS: Process write,<br/>replicate to quorum
    DataUS-->>AppEU: Write confirmed

    AppEU-->>UserEU: Success<br/>(higher latency due to<br/>cross-region write, but<br/>CORRECT — same explicit<br/>tradeoff documented in the<br/>Multi-Region Strong<br/>Consistency design)
```

*This directly reuses the sharded-leader approach from the Multi-Region Strong Consistency design — even within an overall "active-active" deployment philosophy, individual pieces of STRONGLY CONSISTENT data still need a single designated writer region at any given time, accepting the cross-region latency cost for users outside that data's home region.*

---

## 7. Regional Capacity Planning for Failover

```mermaid
flowchart TB
    A["3 active regions, EACH<br/>normally handling ~33% of<br/>total traffic"] --> B["If ONE region fails, the<br/>remaining TWO regions must<br/>absorb the FULL platform<br/>traffic load — meaning each<br/>surviving region needs to<br/>handle ~50% of total traffic,<br/>not just its normal ~33%"]

    B --> C["Capacity planning<br/>implication: EVERY region<br/>must be provisioned with<br/>HEADROOM beyond its normal<br/>load — specifically enough<br/>to absorb a failover<br/>scenario, not just sized<br/>for steady-state traffic"]

    D["This is a genuine, often<br/>underappreciated COST<br/>tradeoff of true active-active<br/>architecture — the resilience<br/>benefit requires deliberately<br/>NOT running every region at<br/>full utilization during<br/>normal operation"] -.-> C
```

---

## 8. Data Replication Conflict Resolution

```mermaid
flowchart TB
    A["User in APAC and user in<br/>EU BOTH modify a shared,<br/>eventually-consistent piece<br/>of data (e.g., a shared<br/>document, a collaborative<br/>list) at nearly the same<br/>time, each hitting their<br/>OWN regional active data<br/>store"] --> B["Same conflict detection<br/>challenge as the Vector<br/>Clock / Causal Ordering<br/>design — need to determine:<br/>are these genuinely CONCURRENT<br/>writes, or does one<br/>causally depend on the other?"]

    B --> C["Apply the SAME resolution<br/>toolkit covered in that<br/>design: CRDT-based automatic<br/>merge where the data type<br/>supports it, or explicit<br/>application-level conflict<br/>surfacing where it doesn't"]
```

---

## 9. Automated Failback (Region Recovery)

```mermaid
sequenceDiagram
    participant RecoveredRegion as US-East<br/>(infrastructure recovered)
    participant HealthCheck as Global Health Monitoring
    participant DNS as Global DNS
    participant DataStores as Other Regions

    Note over RecoveredRegion: Underlying infrastructure<br/>issue resolved

    RecoveredRegion->>DataStores: Begin catching up on<br/>replication lag (data<br/>changes that occurred<br/>elsewhere DURING the outage)

    HealthCheck->>RecoveredRegion: Health checks<br/>begin passing again

    Note over HealthCheck: Wait for SUSTAINED health<br/>(not just one passing check)<br/>AND confirm replication<br/>has caught up before<br/>resuming traffic — avoids<br/>prematurely sending live<br/>traffic to a region that's<br/>still catching up on data

    HealthCheck->>DNS: Mark US-East as<br/>HEALTHY again
    DNS->>DNS: Resume including<br/>US-East in normal<br/>geo-routing decisions

    Note over RecoveredRegion: Traffic gradually resumes<br/>flowing to US-East as<br/>users' DNS caches<br/>naturally refresh
```

**Why replication catch-up verification matters before resuming traffic:** A region that JUST recovered infrastructure-wise may still be behind on replicating changes that occurred in OTHER regions during its downtime — sending live traffic to it before this catch-up completes risks serving genuinely stale data to users, which is why health checks for failback should verify data currency, not just basic service availability.

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Multi-Region Active-Active HLD))
    Global DNS
      Health-based traffic routing
      Automatic failover/failback
    Regional Infrastructure
      Full application stack per region
      All actively serving real traffic
    Sharded-Leader Data Stores
      Strong consistency where required
      Region-designated writers per shard
    Eventually Consistent Stores
      CRDT/conflict-resolution based
      Every region accepts local writes
    Health Monitoring
      Multi-vantage-point consensus
      Failover AND failback verification
    Capacity Planning
      Headroom for N-1 region operation
      Deliberate under-utilization tradeoff
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Deployment model | True active-active (not active-passive) | Maximizes infrastructure utilization and enables fast failover (rerouting, not promotion), at the cost of solving harder multi-writer consistency problems |
| Consistency strategy | Per-data-type, not uniform | Different data has genuinely different consistency needs — financial data needs strong consistency (single-writer sharding); engagement data can be eventually consistent |
| Failover mechanism | Global DNS health-based rerouting | Directly reuses the dedicated Global DNS design's health-routing capability — traffic redirection, not infrastructure activation |
| Capacity provisioning | Headroom beyond steady-state for N-1 region operation | A genuine, deliberate cost tradeoff — true resilience requires NOT running every region at full utilization normally |
| Failback | Verified replication catch-up before resuming traffic | Prevents serving stale data from a region that's infrastructure-healthy but not yet data-current |

---

## 12. Bottlenecks & Scaling Considerations

- **Cross-region write latency for strongly-consistent data** — this remains the same fundamental, unavoidable cost explored in the Multi-Region Strong Consistency design; active-active doesn't eliminate this physics-bound tradeoff for data genuinely requiring strong consistency, it only optimizes the deployment/failover MODEL around it.
- **Capacity headroom cost** — provisioning every region to absorb a failover scenario (not just steady-state load) is a real, ongoing infrastructure cost — this must be weighed against the business cost of reduced resilience if headroom is trimmed to save money, a genuine business/engineering tradeoff decision.
- **Conflict resolution complexity growth** — as MORE regions actively accept writes simultaneously (versus a simpler 2-region setup), the space of possible concurrent-write conflict scenarios grows, requiring more thorough testing and potentially more sophisticated CRDT/merge strategies.
- **Testing failover reliability** — the entire value proposition of this architecture depends on failover actually working correctly when a REAL regional failure occurs — this requires regular, deliberate failover drills/chaos engineering exercises (simulating actual regional outages) rather than only trusting the mechanism theoretically, since failover logic that's never genuinely exercised risks having accumulated subtle bugs by the time it's actually needed.
- **Partial regional degradation (not full outage)** — real-world failures aren't always clean "region fully down" events; a region might be PARTIALLY degraded (e.g., elevated latency, a subset of services failing) — health checking and failover logic designed only for binary healthy/unhealthy states may not handle these partial-degradation scenarios gracefully, requiring more nuanced health signals than simple pass/fail.
- **Coordination and monitoring complexity** — operating genuinely active-active infrastructure across multiple regions, each independently serving real production traffic, requires unified cross-region monitoring/alerting/deployment tooling — the operational complexity of managing N fully-active regions is substantially higher than managing one active region with passive standbys, a real organizational and tooling investment beyond the architecture itself.
