# Design a Service Mesh — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Provide service-to-service communication capabilities (routing, load balancing, retries) WITHOUT requiring each application to implement this logic itself
- Enforce mutual TLS (mTLS) encryption between all services automatically
- Support traffic shaping: canary releases, traffic mirroring, fault injection for testing
- Provide observability: automatic metrics, distributed tracing, and logging for all inter-service calls

### Non-Functional Requirements
- **Transparency to application code:** Services shouldn't need to know the mesh exists — no application-level SDK integration required for baseline functionality
- **Low latency overhead:** Every single service-to-service call now passes through the mesh — this overhead must be minimal
- **Consistent policy enforcement:** Security and traffic policies must be uniformly applied across potentially hundreds of independently-developed services
- **Operational manageability:** Must be centrally configurable without requiring per-service redeployment for policy changes

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Services in mesh | Hundreds to low thousands |
| Inter-service calls/sec (platform-wide) | Millions |
| Added latency per hop (sidecar overhead) | Target: low single-digit milliseconds |
| mTLS handshake overhead | Amortized via connection reuse/pooling |

---

## 2. The Core Architectural Pattern — The Sidecar Proxy

```mermaid
flowchart TB
    A["Traditional approach: EACH<br/>service implements its OWN<br/>retry logic, load balancing,<br/>TLS handling, metrics<br/>collection — typically via a<br/>shared library"] --> A1["Problem: libraries drift out<br/>of sync across services<br/>(different versions, languages,<br/>teams updating at different<br/>paces) — inconsistent behavior<br/>and a maintenance burden<br/>multiplied across every<br/>single service"]

    B["Service mesh approach: deploy<br/>a SIDECAR PROXY container<br/>alongside EVERY service<br/>instance — ALL network traffic<br/>in and out of the service<br/>flows THROUGH this proxy,<br/>which handles retries,<br/>load balancing, TLS, metrics<br/>UNIFORMLY, completely outside<br/>the application's own code"] --> B1["The application code remains<br/>completely unaware — it just<br/>makes a normal local network<br/>call, and the sidecar<br/>transparently intercepts and<br/>manages everything about<br/>HOW that call actually<br/>traverses the network"]
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph PodA["Service A Instance"]
        AppA["Application A"]
        SidecarA["Sidecar Proxy<br/>(Envoy)"]
    end

    subgraph PodB["Service B Instance"]
        AppB["Application B"]
        SidecarB["Sidecar Proxy<br/>(Envoy)"]
    end

    subgraph ControlPlane["Mesh Control Plane"]
        ConfigMgr["Configuration Manager<br/>(routing rules, policies)"]
        CertAuthority["Certificate Authority<br/>(issues mTLS certs)"]
        ServiceRegistry["Service Discovery Registry"]
    end

    AppA -->|"local call<br/>(app unaware of mesh)"| SidecarA
    SidecarA -->|"mTLS-encrypted,<br/>retried, load-balanced<br/>network call"| SidecarB
    SidecarB -->|"local call"| AppB

    ConfigMgr -.->|"push routing/policy config"| SidecarA
    ConfigMgr -.->|"push routing/policy config"| SidecarB
    CertAuthority -.->|"issue/rotate certs"| SidecarA
    CertAuthority -.->|"issue/rotate certs"| SidecarB
    ServiceRegistry -.->|"endpoint discovery"| SidecarA
```

**Key idea:** The sidecar proxies collectively form the "data plane" (where actual traffic flows), while a separate, centralized "control plane" manages configuration, security policy, and certificates — pushing this configuration DOWN to every sidecar. Applications only ever talk to their own local sidecar; the sidecar handles everything about how that call actually reaches its destination.

---

## 4. Data Model (Control Plane Configuration)

```mermaid
erDiagram
    SERVICE {
        string service_name PK
        list healthy_endpoints
        string namespace
    }
    ROUTING_RULE {
        string rule_id PK
        string source_service
        string destination_service
        int traffic_weight_percent
        string destination_version
    }
    SECURITY_POLICY {
        string policy_id PK
        string source_service
        string destination_service
        bool mtls_required
        list allowed_operations
    }
    CERTIFICATE {
        string service_identity PK
        bytes cert
        bytes private_key
        timestamp expires_at
    }
```

---

## 5. Service-to-Service Call Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant AppA as Application A
    participant SidecarA as Sidecar A
    participant Registry as Service Registry
    participant SidecarB as Sidecar B (one of<br/>several B instances)
    participant AppB as Application B

    AppA->>SidecarA: HTTP call to<br/>"service-b" (local,<br/>plaintext — app doesn't<br/>manage TLS itself)

    SidecarA->>Registry: Resolve healthy<br/>instances of service-b<br/>(cached, periodically refreshed)
    Registry-->>SidecarA: [instance_1, instance_2,<br/>instance_3]

    SidecarA->>SidecarA: Apply load balancing<br/>policy (e.g., least-connections)<br/>select instance_2

    SidecarA->>SidecarB: mTLS-encrypted call<br/>to instance_2<br/>(mutual certificate<br/>verification — BOTH sides<br/>authenticate each other)

    SidecarB->>AppB: Forward as local,<br/>plaintext call<br/>(app unaware of the<br/>encryption that happened<br/>over the wire)

    AppB-->>SidecarB: Response
    SidecarB-->>SidecarA: mTLS-encrypted response
    SidecarA-->>AppA: Local response
```

**Why mTLS at the sidecar level, not the application level:** Requiring every application team to correctly implement mutual TLS themselves is both a significant burden and a consistency risk (implementations WILL vary in correctness/rigor). By handling this entirely within the sidecar — using automatically-issued, automatically-rotated certificates from the control plane — encryption between services becomes a MESH-WIDE GUARANTEE rather than a per-team best-effort implementation detail.

---

## 6. Automatic Retries & Circuit Breaking

```mermaid
sequenceDiagram
    participant SidecarA as Sidecar A
    participant Instance1 as Service B Instance 1
    participant Instance2 as Service B Instance 2

    SidecarA->>Instance1: Request
    Note over Instance1: Times out / returns 503

    SidecarA->>SidecarA: Automatic retry policy<br/>(configured at mesh level,<br/>NOT in application code)
    SidecarA->>Instance2: Retry against a<br/>DIFFERENT instance<br/>(avoid retrying the<br/>same possibly-unhealthy one)

    Instance2-->>SidecarA: Success

    Note over SidecarA: If Instance1 continues<br/>failing across multiple<br/>calls from many different<br/>sidecars, CIRCUIT BREAKER<br/>logic temporarily stops<br/>routing ANY traffic to it —<br/>giving it time to recover<br/>without continued load
```

**Why this belongs in the mesh, not application code:** Every single service in the mesh benefits from consistent, well-tuned retry and circuit-breaking behavior without each team needing to correctly implement this resilience pattern themselves — a notoriously easy thing to get subtly wrong (e.g., retrying non-idempotent operations, or retry storms without backoff) if left to inconsistent per-team implementation.

---

## 7. Traffic Shaping — Canary Release via the Mesh

```mermaid
flowchart TB
    A["New version of Service B<br/>(v2) deployed alongside<br/>existing v1"] --> B["Control Plane pushes<br/>ROUTING RULE to ALL<br/>sidecars calling Service B:<br/>90% traffic → v1,<br/>10% traffic → v2"]

    B --> C["Every sidecar mesh-wide<br/>enforces this SAME weighted<br/>split — consistent behavior<br/>regardless of which service<br/>is calling, without ANY of<br/>the calling services' code<br/>needing awareness of the<br/>canary rollout happening"]

    C --> D["Monitor v2's health/metrics<br/>(same principle as the ML<br/>Model Serving design's<br/>canary monitoring, applied<br/>at the infrastructure/service<br/>level instead of the model<br/>level)"]

    D --> E["Gradually shift weight:<br/>90/10 → 50/50 → 0/100,<br/>purely via control-plane<br/>configuration changes —<br/>ZERO application code<br/>changes or redeployment<br/>needed at any calling service"]
```

---

## 8. Automatic Observability (Metrics, Tracing, Logging)

```mermaid
flowchart TB
    A["Every single inter-service<br/>call passes through a sidecar"] --> B["Sidecar automatically<br/>captures: latency, status code,<br/>request/response size, retry<br/>count — for EVERY call,<br/>WITHOUT the application<br/>needing to instrument<br/>anything itself"]

    B --> C["Distributed tracing: sidecar<br/>propagates trace context<br/>headers, allowing a SINGLE<br/>user request's full journey<br/>across DOZENS of services to<br/>be reconstructed and<br/>visualized end-to-end"]

    C --> D["This gives platform-wide,<br/>UNIFORM observability<br/>essentially for free — a<br/>massive advantage over<br/>requiring every team to<br/>correctly and consistently<br/>instrument their own service's<br/>observability"]
```

---

## 9. Certificate Rotation (Automated Security Maintenance)

```mermaid
sequenceDiagram
    participant CA as Certificate Authority<br/>(Control Plane)
    participant SidecarA as Sidecar A
    participant SidecarB as Sidecar B

    loop Periodic rotation (e.g., every 24 hours,<br/>well before expiry)
        CA->>SidecarA: Issue NEW certificate<br/>for Service A's identity
        CA->>SidecarB: Issue NEW certificate<br/>for Service B's identity
    end

    Note over SidecarA,SidecarB: Certificates rotate<br/>AUTOMATICALLY and FREQUENTLY —<br/>dramatically reducing the<br/>blast radius of any single<br/>compromised certificate<br/>compared to traditional,<br/>manually-managed, long-lived<br/>certificates
```

**Why automated short-lived certificates are a major security improvement:** Traditional manually-managed TLS certificates are often long-lived (months to years) simply because manual rotation is operationally painful — meaning a compromised certificate remains dangerous for a long time. Automating rotation via the mesh's control plane makes SHORT-lived certificates (hours to days) operationally trivial, dramatically shrinking the window of risk from any single certificate compromise.

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Service Mesh HLD))
    Sidecar Proxy
      Intercepts all service traffic
      Load balancing, retries, mTLS
      Transparent to application
    Configuration Manager
      Centralized routing/policy rules
      Pushes to all sidecars
    Certificate Authority
      Automated issuance and rotation
      Short-lived certificates
    Service Registry
      Endpoint discovery
      Health-aware routing input
    Observability Pipeline
      Automatic metrics and tracing
      Uniform across all services
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Core pattern | Sidecar proxy per service instance | Provides consistent, uniform networking/security/observability behavior across all services without requiring per-team library integration and maintenance |
| Encryption | Automatic mTLS handled entirely by sidecars | Makes encryption a mesh-wide GUARANTEE rather than a per-team implementation detail subject to inconsistency |
| Configuration | Centralized control plane, pushed to distributed data plane | Enables platform-wide policy changes (canary rollouts, retry tuning) without requiring any application redeployment |
| Resilience patterns | Retries and circuit breaking at the mesh level | Notoriously easy to implement subtly incorrectly at the application level; centralizing ensures consistent, well-tuned behavior everywhere |
| Certificate management | Automated, short-lived, frequently rotated | Dramatically reduces the security blast radius compared to traditional manually-managed long-lived certificates |
| Observability | Automatic, sidecar-level instrumentation | Provides uniform platform-wide visibility without depending on every team correctly instrumenting their own service |

---

## 12. Bottlenecks & Scaling Considerations

- **Sidecar latency overhead compounds across call chains** — a single user request that fans out across many internal service hops (common in microservices architectures) accumulates the sidecar's per-hop latency overhead multiple times; while individually small, this can become meaningful for deeply-chained call graphs, requiring careful architecture-level awareness of call depth, not just per-hop optimization.
- **Control plane as a critical, mesh-wide dependency** — while the DATA plane (sidecars actually routing traffic) can continue operating on cached configuration even if the control plane is briefly unavailable, the control plane's health directly determines how quickly policy changes, certificate rotations, and service discovery updates propagate mesh-wide; needs high availability commensurate with its central role.
- **Resource overhead per service instance** — running a sidecar proxy alongside EVERY service instance means every single pod/container now has additional CPU/memory overhead purely for the mesh infrastructure — at very large scale (thousands of instances), this aggregate resource cost is a genuine, non-trivial consideration in capacity planning.
- **Operational complexity and learning curve** — a service mesh introduces a significant new operational layer that teams must understand to debug effectively (e.g., "is this latency from my application code, or from the sidecar's retry logic?") — this requires investment in team education and mesh-aware debugging tooling, not just the infrastructure itself.
- **Multi-cluster/multi-region mesh federation** — extending mesh capabilities (unified service discovery, cross-cluster mTLS) across MULTIPLE Kubernetes clusters or regions adds substantial additional complexity beyond a single-cluster mesh deployment, often requiring dedicated cross-cluster gateway components not covered in this simplified single-cluster design.
- **Gradual adoption path** — for organizations with many existing services, migrating everything to run with sidecars simultaneously is often impractical; the mesh design needs to gracefully support a MIXED environment (some services meshed, some not) during a gradual migration period, rather than requiring an all-or-nothing cutover.
