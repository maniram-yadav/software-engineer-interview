# Design a Single Sign-On (SSO) System Supporting SAML and OAuth2/OIDC — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Allow users to authenticate once and gain access to multiple independent applications ("service providers") without re-entering credentials
- Support multiple identity provider (IdP) protocols: SAML 2.0 (common in enterprise) and OAuth2/OIDC (common in modern web/mobile)
- Support federation with EXTERNAL identity providers (e.g., "Login with Google," a customer's own Okta/Azure AD)
- Support logout that terminates the session across all connected applications (single logout)

### Non-Functional Requirements
- **Security (paramount):** This system is the literal front door to every connected application — a vulnerability here compromises everything downstream
- **Protocol correctness:** SAML and OAuth2/OIDC have precise, standardized flows; deviations break interoperability with third-party IdPs/SPs
- **Session management at scale:** Must track active sessions across potentially millions of users and many connected applications
- **Availability:** SSO going down effectively locks users out of every connected application simultaneously — extremely high availability bar

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Logins/sec (peak, e.g., Monday morning) | ~10,000 |
| Active sessions | Tens of millions |
| Connected applications (service providers) | Dozens to hundreds per enterprise deployment |
| Token/assertion validation calls/sec | Very high — happens on nearly every authenticated request |

---

## 2. Core Concepts — Identity Provider vs Service Provider

```mermaid
flowchart TB
    A["Identity Provider (IdP)"] --> A1["The system that AUTHENTICATES<br/>the user — verifies username/<br/>password, MFA, etc. — and<br/>issues a token/assertion<br/>proving who they are"]

    B["Service Provider (SP)"] --> B1["The application the user<br/>actually wants to USE<br/>(e.g., Salesforce, an internal<br/>HR tool) — trusts the IdP's<br/>assertion instead of<br/>handling authentication itself"]

    C["This SSO system IS the IdP<br/>— its entire purpose is to<br/>authenticate users ONCE and<br/>issue trusted assertions/tokens<br/>that many different SPs<br/>can independently verify<br/>and trust"] --> A1
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    User["User Browser"]

    subgraph SSOSystem["SSO System (Identity Provider)"]
        AuthSvc["Authentication Service<br/>(credential verification, MFA)"]
        SessionSvc["Session Management Service"]
        SAMLModule["SAML Module<br/>(assertion generation)"]
        OIDCModule["OIDC Module<br/>(token generation)"]
        SessionStore[("Session Store<br/>(distributed, low-latency)")]
    end

    subgraph ExternalIdPs["Federated External IdPs"]
        Google["Google"]
        Okta["Customer's Okta/Azure AD"]
    end

    subgraph ServiceProviders["Connected Applications (SPs)"]
        SP1["SAML-based App<br/>(e.g., Salesforce)"]
        SP2["OIDC-based App<br/>(e.g., internal web app)"]
    end

    User --> AuthSvc
    AuthSvc --> SessionSvc --> SessionStore
    AuthSvc -.->|"delegate authentication"| Google
    AuthSvc -.->|"delegate authentication"| Okta

    SessionSvc --> SAMLModule --> SP1
    SessionSvc --> OIDCModule --> SP2
```

**Key idea:** The SSO system centralizes authentication logic and session state, but exposes it through TWO distinct protocol modules (SAML and OIDC) — because these are genuinely different standards with different message formats and flows, not just two configuration options of the same underlying mechanism. Additionally, the SSO system itself can delegate to further UPSTREAM identity providers (federation), acting as both an IdP to its SPs and a "relying party" to external IdPs simultaneously.

---

## 4. SAML 2.0 Flow (Enterprise-Style) — Detailed Sequence

```mermaid
sequenceDiagram
    participant User as User Browser
    participant SP as Service Provider<br/>(e.g., Salesforce)
    participant IdP as SSO System (IdP)

    User->>SP: Access protected resource<br/>(not yet authenticated)
    SP->>SP: Generate SAML AuthnRequest
    SP-->>User: Redirect to IdP with<br/>AuthnRequest (via browser redirect)

    User->>IdP: Present AuthnRequest
    IdP->>IdP: Check: does user have an<br/>existing valid SSO session?

    alt No existing session
        IdP-->>User: Show login form
        User->>IdP: Submit credentials
        IdP->>IdP: Verify credentials, create session
    else Existing valid session
        Note over IdP: Skip login form entirely —<br/>THIS is the "single" in<br/>single sign-on
    end

    IdP->>IdP: Generate SAML Assertion<br/>(digitally signed, contains<br/>user identity + attributes)
    IdP-->>User: Redirect back to SP with<br/>signed SAML Assertion (POST)

    User->>SP: Present SAML Assertion
    SP->>SP: Verify assertion's digital<br/>signature using IdP's<br/>public certificate
    SP->>SP: Extract user identity,<br/>create local SP session
    SP-->>User: Access granted to<br/>protected resource
```

**Why the assertion is digitally signed:** Since the assertion travels through the user's browser (not a direct server-to-server channel) as it passes from IdP to SP, the SP must be able to verify it hasn't been tampered with en route — the IdP's cryptographic signature, verifiable using its published public certificate, provides this guarantee without requiring a direct trusted network path between IdP and SP.

---

## 5. OAuth2/OIDC Flow (Modern Web/Mobile) — Detailed Sequence

```mermaid
sequenceDiagram
    participant User as User Browser
    participant SP as Service Provider<br/>(OIDC Relying Party)
    participant IdP as SSO System<br/>(OIDC Provider)

    User->>SP: Access protected resource
    SP-->>User: Redirect to IdP's<br/>/authorize endpoint<br/>(with client_id, redirect_uri,<br/>scope, state, PKCE challenge)

    User->>IdP: Present authorization request
    IdP->>IdP: Check existing session<br/>(same as SAML flow — skip<br/>login if already authenticated)

    IdP-->>User: Redirect back to SP with<br/>a short-lived AUTHORIZATION CODE

    User->>SP: Present authorization code<br/>(via redirect_uri callback)

    Note over SP,IdP: Server-to-server exchange<br/>(NOT via user's browser —<br/>this is more secure than<br/>SAML's browser-mediated<br/>assertion transfer)

    SP->>IdP: POST /token<br/>{code, client_secret, PKCE verifier}
    IdP->>IdP: Validate code, client credentials,<br/>PKCE verifier
    IdP-->>SP: ID Token (JWT) + Access Token

    SP->>SP: Verify JWT signature,<br/>extract user identity<br/>from ID Token claims
    SP-->>User: Access granted
```

**Why OIDC's code exchange is considered more secure than SAML's browser-mediated flow:** The actual token exchange happens over a direct, authenticated server-to-server channel (SP backend to IdP backend) rather than passing the sensitive credential material through the user's browser as an intermediary — this reduces the attack surface for token interception/replay significantly, which is part of why OIDC is generally preferred for new implementations over SAML.

---

## 6. Session Management — The Foundation of "Single" Sign-On

```mermaid
flowchart TB
    A["User authenticates ONCE<br/>with the IdP"] --> B["IdP creates a central<br/>SSO SESSION<br/>(separate from any<br/>individual SP's session)"]
    B --> C[("Session Store —<br/>session_id, user_id,<br/>auth_time, MFA_completed,<br/>expires_at")]

    D["User navigates to SP #2<br/>(a DIFFERENT application)"] --> E["SP #2 redirects to IdP<br/>for authentication<br/>(as in the flows above)"]
    E --> F{"IdP checks: does the<br/>browser have a valid<br/>SSO session cookie?"}
    F -- Yes --> G["Skip login form entirely —<br/>immediately issue a NEW<br/>assertion/token for SP #2,<br/>based on the EXISTING<br/>central session"]
    F -- No --> H["Require fresh login"]

    G --> I["This central session,<br/>tied to a browser cookie<br/>scoped to the IdP's domain,<br/>is the actual mechanism<br/>that makes 'log in once,<br/>access many apps' work"]
```

---

## 7. Federation With External Identity Providers

```mermaid
sequenceDiagram
    participant User as User Browser
    participant IdP as SSO System<br/>(acting as IdP to SPs,<br/>but Relying Party to Google)
    participant Google as External IdP<br/>(Google)
    participant SP as Service Provider

    User->>SP: Access protected resource
    SP-->>User: Redirect to internal IdP

    User->>IdP: Present auth request
    IdP->>IdP: Determine: this user's<br/>organization is configured<br/>to federate with Google

    IdP-->>User: Redirect to Google<br/>(delegate authentication)
    User->>Google: Authenticate with Google credentials
    Google-->>User: Redirect back to internal IdP<br/>with Google's assertion/token

    User->>IdP: Present Google's assertion
    IdP->>IdP: Verify Google's signature,<br/>extract identity
    IdP->>IdP: Create INTERNAL SSO session<br/>based on the verified<br/>external identity

    IdP->>IdP: Now proceed with normal<br/>SAML/OIDC flow to the<br/>ORIGINAL requesting SP,<br/>as if the user had<br/>logged in directly
```

**Why this layered federation matters architecturally:** The SSO system acts simultaneously as an Identity Provider (to its connected SPs) and a Relying Party/Service Provider (to upstream identity providers like Google or a customer's Azure AD) — this dual role is what allows a single SSO deployment to support "bring your own identity provider" for enterprise customers while still presenting a unified, consistent authentication experience to all downstream connected applications.

---

## 8. Single Logout (Terminating All Sessions)

```mermaid
sequenceDiagram
    participant User as User
    participant IdP as SSO System
    participant SP1 as Service Provider 1
    participant SP2 as Service Provider 2
    participant SP3 as Service Provider 3

    User->>IdP: Initiate logout

    IdP->>IdP: Look up: which SPs have<br/>an active session tied to<br/>this SSO session?
    IdP->>IdP: Invalidate central<br/>SSO session immediately

    par Notify all connected SPs
        IdP->>SP1: Logout notification<br/>(SAML LogoutRequest or<br/>OIDC back-channel logout)
        SP1->>SP1: Terminate local session
    and
        IdP->>SP2: Logout notification
        SP2->>SP2: Terminate local session
    and
        IdP->>SP3: Logout notification
        SP3->>SP3: Terminate local session
    end

    IdP-->>User: Logout complete
```

*Single logout is notably harder to implement reliably than single sign-on itself — it requires proactively notifying multiple independent applications (each potentially using different mechanisms) rather than the more naturally single-point action of creating one central session at login.*

---

## 9. Multi-Factor Authentication Integration

```mermaid
flowchart TB
    A["User submits primary<br/>credentials (password)"] --> B["IdP validates password"]
    B --> C{"MFA required for<br/>this user/application?<br/>(policy-based, e.g., risk<br/>score, SP sensitivity level)"}

    C -- Yes --> D["Prompt for second factor<br/>(TOTP, push notification,<br/>hardware key)"]
    D --> E["Verify second factor"]
    E --> F["Mark session as<br/>MFA_completed=true"]

    C -- No --> F

    F --> G["Session created —<br/>assertions issued to SPs<br/>can include an<br/>'MFA completed' claim,<br/>letting individual SPs<br/>enforce their OWN<br/>step-up requirements"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((SSO System HLD))
    Authentication Service
      Credential verification
      MFA orchestration
      Delegates to external IdPs
    Session Management Service
      Central SSO session
      Enables skip-login for subsequent SPs
    SAML Module
      Assertion generation and signing
      Browser-mediated flow
    OIDC Module
      Authorization code + token exchange
      Server-to-server token delivery
    Session Store
      Distributed, low-latency
      Tracks all active sessions
    Single Logout Coordinator
      Fans out logout to all connected SPs
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Protocol support | Both SAML and OIDC, as distinct modules | Genuinely different standards serving different ecosystems (enterprise legacy vs modern web/mobile) — not interchangeable |
| Session architecture | Central SSO session, separate from per-SP sessions | This central session is the actual mechanism enabling "log in once, access many" — without it, each SP redirect would require fresh authentication |
| Federation model | SSO system as both IdP and Relying Party simultaneously | Enables "bring your own identity provider" for enterprise customers while presenting a unified experience to connected applications |
| Token transport (OIDC) | Server-to-server code exchange, not browser-only | Reduces attack surface compared to passing sensitive tokens through the browser as an intermediary |
| MFA architecture | Policy-based, with claims passed to SPs | Allows individual SPs to make their own step-up authentication decisions based on the SSO system's verified MFA status |
| Logout | Active fan-out notification to all connected SPs | Single logout requires proactive multi-party coordination, unlike the naturally singular action of session creation |

---

## 12. Bottlenecks & Scaling Considerations

- **Session store as an absolutely critical dependency** — since EVERY authentication flow (to any connected SP) depends on session lookups, this store's availability and latency directly bound the entire SSO system's — and therefore every connected application's — availability; needs to be highly available, distributed, and low-latency, similar criticality to the idempotency store in the Idempotent API Requests design.
- **Assertion/token signing key management** — the cryptographic keys used to sign SAML assertions and OIDC tokens are extraordinarily sensitive; compromise would allow forging valid authentication for ANY user to ANY connected application — this connects directly to the Secrets Management System design for how these keys themselves must be protected, rotated, and audited.
- **Clock skew sensitivity** — both SAML assertions and OIDC tokens include time-bound validity windows (not-before, expiry); significant clock skew between the IdP and SP servers can cause valid tokens to be incorrectly rejected — NTP synchronization across all parties is an operational prerequisite, not just a nice-to-have.
- **Federation dependency risk** — when federating to an external IdP (Google, a customer's Azure AD), an outage of that EXTERNAL system directly blocks authentication for affected users, even though the SSO system itself may be perfectly healthy — this external dependency risk should be clearly understood and communicated in SLA commitments.
- **Single logout reliability** — because it requires successfully notifying multiple independent SPs (each potentially over unreliable network calls), true guaranteed single logout across ALL connected applications is genuinely difficult to achieve with 100% reliability; most production systems document this as "best effort" logout propagation rather than an absolute guarantee.
- **Replay attack prevention** — both protocols need careful handling of nonces/unique identifiers (SAML's assertion ID, OIDC's state/nonce parameters) to prevent a captured, valid assertion or authorization code from being replayed by an attacker — this is security-critical, protocol-specified behavior that must be implemented exactly per specification, not approximated.
- **High-availability requirement amplification** — because SSO failure doesn't just affect one application but potentially ALL connected applications simultaneously, the availability bar for this system is effectively the HIGHEST availability requirement across its entire portfolio of dependent applications — this often justifies significantly more redundancy investment than any single downstream application would independently require.
