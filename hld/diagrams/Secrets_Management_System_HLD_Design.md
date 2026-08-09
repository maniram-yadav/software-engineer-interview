# Design a Secrets Management System (Vault-style) — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Securely store secrets (API keys, database credentials, certificates, encryption keys)
- Provide fine-grained access control — which applications/users can access which specific secrets
- Support automatic secret rotation without requiring application downtime
- Maintain a complete, tamper-evident audit log of every secret access
- Support dynamic/short-lived credentials (generated on-demand, auto-expiring) in addition to static secrets

### Non-Functional Requirements
- **Confidentiality (paramount):** Secrets must never be exposed in plaintext anywhere they shouldn't be — not in logs, not in transit unencrypted, not to unauthorized callers
- **Availability:** Since applications depend on this system to even START UP (fetching their DB credentials, etc.), an outage here can cascade into a broad platform outage
- **Least privilege:** Every access grant should be as narrowly scoped as possible, both in what secrets and for how long
- **Auditability:** Every single access must be attributable and logged — this is often a compliance/regulatory requirement, not just a nice-to-have

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Total secrets stored | Tens of thousands to millions (across many applications/environments) |
| Secret access requests/sec | Thousands (mostly at application startup, some steady-state) |
| Rotation frequency | Varies — hours (dynamic DB creds) to months (long-lived API keys) |
| Audit log retention | Often years, for compliance |

---

## 2. The Core Security Model — Encryption, Sealing, and the Root of Trust

```mermaid
flowchart TB
    A["Secrets at rest"] --> B["ALWAYS encrypted —<br/>never stored in plaintext<br/>on disk, ever"]

    B --> C["Encryption requires a<br/>MASTER KEY — but where does<br/>THAT key live? This is the<br/>fundamental bootstrapping<br/>problem of any secrets<br/>management system"]

    C --> D["Solution: the master key<br/>itself is SPLIT using<br/>Shamir's Secret Sharing —<br/>divided into N shares, of<br/>which a THRESHOLD (e.g., 3<br/>of 5) must be combined to<br/>reconstruct it"]

    D --> E["No single person/system<br/>ever holds the complete<br/>master key — trust is<br/>distributed across multiple<br/>independent key-holders<br/>(e.g., different senior<br/>engineers/officers), each<br/>possessing only ONE share"]
```

**Why this "unsealing" mechanism is foundational:** Every secrets management system faces the same bootstrapping paradox — you need a key to decrypt your secrets, but that key is itself the most sensitive secret of all. Shamir's Secret Sharing solves this by ensuring no single compromised person or system component can unilaterally decrypt the vault — a meaningful threshold of independent trust holders must cooperate, which is a deliberate, significant operational friction chosen specifically for its security value.

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph Clients["Client Applications"]
        App1["Application 1"]
        App2["Application 2"]
    end

    subgraph VaultCluster["Secrets Management Cluster"]
        AuthLayer["Authentication Layer<br/>(verifies caller identity)"]
        PolicyEngine["Policy Engine<br/>(fine-grained ACL evaluation)"]
        SecretEngine["Secrets Engine<br/>(static + dynamic secrets)"]
        AuditLog["Audit Logger"]
    end

    subgraph Storage["Encrypted Storage Backend"]
        EncryptedStore[("Encrypted Secret Store<br/>never plaintext at rest")]
        UnsealKeys["Master Key<br/>(in-memory ONLY when unsealed,<br/>reconstructed from shares)"]
    end

    subgraph External["Dynamic Secret Backends"]
        DB[("Databases<br/>— create short-lived<br/>credentials on demand")]
        CloudProvider["Cloud IAM<br/>— issue temporary<br/>access tokens"]
    end

    App1 --> AuthLayer
    App2 --> AuthLayer
    AuthLayer --> PolicyEngine
    PolicyEngine --> SecretEngine
    SecretEngine --> EncryptedStore
    SecretEngine <--> UnsealKeys
    SecretEngine --> DB
    SecretEngine --> CloudProvider

    AuthLayer --> AuditLog
    PolicyEngine --> AuditLog
    SecretEngine --> AuditLog
```

**Key idea:** Every single request — successful or denied — passes through authentication, then policy evaluation, before ever touching the secrets engine, with the audit logger capturing the full trail at each stage. This layered gate structure ensures that "who is asking" and "are they allowed" are always resolved BEFORE any secret material is ever retrieved or generated.

---

## 4. Data Model

```mermaid
erDiagram
    SECRET {
        string secret_path PK
        bytes encrypted_value
        int version
        timestamp created_at
        timestamp rotation_due_at
    }
    POLICY {
        string policy_id PK
        string path_pattern "e.g. secret/app1/*"
        list allowed_operations "read/write/list"
    }
    IDENTITY {
        string identity_id PK
        string auth_method "AppRole/K8s/OIDC"
        list attached_policies
    }
    AUDIT_ENTRY {
        string entry_id PK
        string identity_id FK
        string secret_path
        string operation
        string result "allowed/denied"
        timestamp accessed_at
        string request_hash "for tamper detection"
    }
```

---

## 5. Static Secret Retrieval Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant App as Application
    participant Auth as Authentication Layer
    participant Policy as Policy Engine
    participant SecretEng as Secrets Engine
    participant Store as Encrypted Store
    participant Audit as Audit Logger

    App->>Auth: Request secret at path<br/>"secret/app1/db-password"<br/>(with identity credentials —<br/>e.g., a Kubernetes service<br/>account token)

    Auth->>Auth: Verify identity<br/>(validate the auth token<br/>against the configured<br/>auth method)
    Auth->>Audit: Log authentication attempt

    Auth->>Policy: Check: does this identity's<br/>attached policies permit<br/>READ on this exact path?
    Policy->>Audit: Log authorization decision

    alt Authorized
        Policy->>SecretEng: Fetch secret
        SecretEng->>Store: Retrieve encrypted value
        Store-->>SecretEng: Encrypted bytes
        SecretEng->>SecretEng: Decrypt using unsealed<br/>master key (in-memory only)
        SecretEng-->>App: Return plaintext secret<br/>(over TLS, never logged)
        SecretEng->>Audit: Log successful access
    else Not authorized
        Policy-->>App: 403 Forbidden
        Policy->>Audit: Log denied access attempt<br/>(equally important to log<br/>failures as successes)
    end
```

---

## 6. Dynamic Secrets — Short-Lived, On-Demand Credentials

```mermaid
flowchart TB
    A["Traditional static secret<br/>problem: a long-lived DB<br/>password shared across many<br/>application instances — if<br/>compromised, valid until<br/>manually rotated, and hard<br/>to attribute WHICH instance<br/>leaked it"] --> B["Dynamic secrets: instead of<br/>storing a static password,<br/>the secrets system has<br/>PRIVILEGED ACCESS to CREATE<br/>brand new, unique database<br/>credentials on-demand"]

    B --> C["Each application instance<br/>gets its OWN unique,<br/>short-lived credential —<br/>auto-expiring after a<br/>configured TTL (e.g., 1 hour)"]

    C --> D["Benefits: automatic expiry<br/>bounds the damage of any<br/>leak, precise attribution<br/>(each credential traceable<br/>to exactly one request/<br/>instance), and NO long-lived<br/>secret ever needs to be<br/>manually rotated at all"]
```

```mermaid
sequenceDiagram
    participant App as Application
    participant SecretEng as Secrets Engine
    participant DB as Target Database

    App->>SecretEng: Request dynamic DB credential<br/>for role "readonly-app1"

    SecretEng->>SecretEng: Verify auth + policy<br/>(same as static flow)

    SecretEng->>DB: CREATE USER with<br/>randomly generated<br/>username/password,<br/>GRANT readonly permissions,<br/>SET expiry = now+1hr
    DB-->>SecretEng: Credential created

    SecretEng-->>App: Return the newly created,<br/>UNIQUE credential<br/>(valid for 1 hour)

    Note over SecretEng,DB: After TTL expires, a<br/>background lease-revocation<br/>process automatically<br/>REVOKES this specific<br/>credential from the database
```

---

## 7. Lease Management & Automatic Revocation

```mermaid
sequenceDiagram
    participant SecretEng as Secrets Engine
    participant LeaseStore as Lease Tracking Store
    participant Revoker as Lease Revocation Worker
    participant DB as Target Database

    SecretEng->>LeaseStore: Record lease:<br/>{credential_id, expires_at,<br/>revocation_action}

    loop Continuous background sweep
        Revoker->>LeaseStore: Find leases WHERE<br/>expires_at < now()
        LeaseStore-->>Revoker: Expired leases

        loop For each expired lease
            Revoker->>DB: Execute revocation<br/>(e.g., DROP USER)
            Revoker->>LeaseStore: Mark lease as REVOKED
        end
    end
```

**Why automatic lease revocation is essential, not optional:** Dynamic secrets are only as valuable as their expiry enforcement — a credential that's supposed to expire but doesn't (due to a missed revocation) provides false confidence while remaining an active liability. This mirrors the same reservation-expiry pattern from the E-commerce Checkout design, applied to security-critical credentials rather than inventory holds — the consequence of a missed expiry here is considerably higher stakes.

---

## 8. Secret Rotation (For Static Secrets That Must Remain Static)

```mermaid
sequenceDiagram
    participant Scheduler as Rotation Scheduler
    participant SecretEng as Secrets Engine
    participant Store as Encrypted Store
    participant TargetSystem as Target System<br/>(e.g., third-party API provider)
    participant App as Consuming Application

    Note over Scheduler: Some secrets (e.g., a<br/>third-party API key) can't<br/>be made fully dynamic —<br/>they still benefit from<br/>periodic rotation

    Scheduler->>SecretEng: Trigger rotation for<br/>"secret/app1/stripe-api-key"

    SecretEng->>TargetSystem: Generate NEW API key<br/>(via target system's own<br/>key management API)
    TargetSystem-->>SecretEng: New key issued

    SecretEng->>Store: Store new key as<br/>NEW VERSION (old version<br/>retained temporarily, not<br/>immediately deleted)

    Note over App: Applications configured to<br/>periodically re-fetch their<br/>secrets will pick up the<br/>new version on their next<br/>refresh cycle

    SecretEng->>TargetSystem: After a grace period,<br/>REVOKE the old key<br/>(gives time for all app<br/>instances to have picked<br/>up the new version)
```

**Why the grace period before revoking the old version matters:** Rotating a static secret isn't instantaneous across a distributed fleet of application instances — some may still be using the previous version briefly after rotation. Immediately revoking the old key the moment a new one is issued risks breaking still-transitioning application instances; a grace period accepts a brief window of both keys being simultaneously valid in exchange for zero-downtime rotation.

---

## 9. Tamper-Evident Audit Logging

```mermaid
flowchart TB
    A["Every secret access<br/>(successful or denied)<br/>generates an audit entry"] --> B["Entry includes a<br/>cryptographic hash chaining<br/>it to the PREVIOUS entry<br/>(similar principle to a<br/>blockchain's hash-linking)"]

    B --> C["If an attacker (even one<br/>with elevated system access)<br/>attempts to DELETE or MODIFY<br/>a past audit entry to hide<br/>their tracks..."]

    C --> D["...the hash chain BREAKS —<br/>any subsequent verification<br/>of the log's integrity<br/>immediately reveals tampering,<br/>even if the specific modified<br/>entry itself isn't obviously<br/>wrong on its own"]

    E["This tamper-evidence is<br/>critical for compliance and<br/>incident response — the audit<br/>log's INTEGRITY is as<br/>important as its existence"] --> D
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Secrets Management HLD))
    Authentication Layer
      Verifies caller identity
      Multiple auth methods supported
    Policy Engine
      Fine-grained path-based ACLs
      Evaluated before any secret access
    Secrets Engine
      Static and dynamic secret handling
      Encryption/decryption with master key
    Lease Management
      Tracks dynamic credential TTLs
      Automatic background revocation
    Rotation Scheduler
      Periodic static secret rotation
      Grace-period-based cutover
    Audit Logger
      Tamper-evident hash chaining
      Logs both success and denial
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Master key protection | Shamir's Secret Sharing (threshold unsealing) | Ensures no single compromised person/component can unilaterally decrypt the entire vault — distributes trust deliberately |
| Secret types supported | Both static (encrypted storage) and dynamic (on-demand generation) | Dynamic secrets eliminate long-lived credential risk entirely where possible; static secrets remain necessary for systems that can't support on-demand credential creation |
| Dynamic secret lifecycle | Auto-expiring leases with background revocation | Bounds the damage of any credential leak automatically, without requiring manual intervention |
| Static secret rotation | Grace-period-based, dual-valid-version cutover | Enables zero-downtime rotation across a distributed application fleet that can't be updated instantaneously and simultaneously |
| Audit logging | Tamper-evident hash-chained entries | Makes unauthorized log modification detectable, which is essential for meeting compliance and incident-response requirements |
| Access control | Fine-grained, path-based policies evaluated before every access | Enforces least-privilege as a structural property of every request, not an assumed convention |

---

## 12. Bottlenecks & Scaling Considerations

- **Availability is existentially critical** — because applications often depend on this system just to START (fetching their initial DB credentials, API keys), an outage here can cascade into failures across the ENTIRE platform simultaneously; this justifies significant investment in high availability (multi-node clustering, cross-region replication) beyond what a typical internal tool would warrant.
- **The unsealing process itself is a deliberate availability tradeoff** — after any full restart, the vault requires a threshold of key-holders to manually provide their shares before it can serve ANY requests; this is intentional friction for security, but means a full cluster restart isn't as simple as a typical stateless service restart, and needs clear operational runbooks.
- **Dynamic secret backend load** — if dynamic secrets are used extensively (e.g., a new short-lived DB credential per request, not per instance), the load placed on the TARGET system (creating/dropping users constantly) can itself become a bottleneck — needs careful TTL tuning to balance security benefit against target-system overhead.
- **Policy evaluation performance at scale** — with potentially complex, many-layered policies across thousands of identities and secret paths, policy evaluation must remain fast since it sits on the critical path of every single secret access — this often requires efficient policy indexing/caching strategies, not naive linear policy scanning.
- **Audit log storage growth** — given typically multi-year compliance retention requirements and high access-request volume, audit log storage grows substantially over time; needs the same tiered storage/retention strategy discussed in the Log Aggregation design, while preserving the tamper-evident chain's integrity across any archival process.
- **Client-side caching risk** — applications that cache retrieved secrets locally (to avoid re-fetching on every use) reintroduce some of the long-lived-credential risk that dynamic secrets are meant to solve; client libraries need clear guidance and enforced TTL-respecting behavior, not indefinite local caching.
- **Break-glass emergency access** — production incidents sometimes require emergency access that bypasses normal policy flows (e.g., a critical secret needed during an active outage, but the usual approver is unreachable); this requires a carefully designed, heavily audited "break-glass" procedure that's rare, logged extensively, and reviewed after every use — a deliberate exception path, not a backdoor.
