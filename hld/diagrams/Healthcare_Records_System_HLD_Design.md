# Design a Healthcare Records System (Strict Access Control + HL7/FHIR Interoperability) — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Store and retrieve patient medical records (diagnoses, medications, lab results, clinical notes)
- Support fine-grained, role-based access control — different care providers need different levels of access to different parts of a record
- Exchange data with EXTERNAL healthcare systems (other hospitals, labs, pharmacies) using standardized interoperability formats (HL7/FHIR)
- Maintain complete audit trails of every single record access, not just modifications

### Non-Functional Requirements
- **Strict privacy and access control (paramount):** Unauthorized access to patient data has severe legal (HIPAA and similar regulations), ethical, and patient-trust consequences
- **Interoperability:** Must correctly exchange data with a highly heterogeneous ecosystem of external systems, each potentially using slightly different standard versions/implementations
- **Availability for clinical use:** In emergency/clinical settings, the system being unavailable can directly impact patient care — this is a genuinely different availability calculus than most business systems
- **Complete auditability:** Every access (not just writes) must be logged, given the regulatory requirement to detect and investigate unauthorized access

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Patient records | Millions, across a large healthcare network |
| Record accesses/sec | Thousands during peak clinical hours |
| External system integrations | Dozens to hundreds (labs, pharmacies, other hospital systems) |
| Audit log retention | Often 7+ years, per regulatory requirement |

---

## 2. The Core Tension — Access Control Granularity vs Clinical Urgency

```mermaid
flowchart TB
    A["Strict, narrow access<br/>control: a provider can ONLY<br/>access exactly the specific<br/>patient records/sections<br/>they've been EXPLICITLY<br/>authorized for"] --> A1["PRO: minimizes unauthorized<br/>access risk<br/>CON: in a genuine EMERGENCY<br/>(unconscious patient, unknown<br/>treating physician), overly<br/>rigid access control could<br/>directly delay life-critical<br/>care"]

    B["This tension is UNIQUE to<br/>healthcare among the systems<br/>covered in this design series —<br/>most access-control systems<br/>(e.g., the Secrets Management<br/>design) can simply DENY<br/>unauthorized access with no<br/>further consideration. Here,<br/>the design must EXPLICITLY<br/>account for legitimate<br/>emergency-access needs as a<br/>FIRST-CLASS requirement, not<br/>an afterthought"] --> C["Solution: 'Break-glass'<br/>emergency access — a<br/>DELIBERATE, heavily-audited<br/>override mechanism, distinct<br/>from normal role-based<br/>access, covered in detail<br/>below"]
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph Providers["Care Providers"]
        Physician["Physician"]
        Nurse["Nurse"]
        EmergencyStaff["Emergency Room Staff"]
    end

    subgraph AccessLayer["Access Control Layer"]
        AuthNZ["Authentication & Authorization<br/>(role-based + relationship-based)"]
        BreakGlass["Break-Glass Emergency<br/>Access Handler"]
    end

    subgraph CoreSystem["Core Records System"]
        RecordAPI["Patient Record API"]
        RecordStore[("Patient Record Store<br/>— encrypted at rest")]
        AuditLog[("Tamper-Evident Audit Log<br/>— same design as the<br/>dedicated Audit Log system")]
    end

    subgraph Interop["Interoperability Layer"]
        FHIRGateway["FHIR/HL7 Gateway"]
        ExternalSystems["External Systems<br/>(labs, pharmacies,<br/>other hospitals)"]
    end

    Physician --> AuthNZ
    Nurse --> AuthNZ
    EmergencyStaff --> BreakGlass

    AuthNZ --> RecordAPI
    BreakGlass --> RecordAPI
    RecordAPI --> RecordStore
    RecordAPI --> AuditLog
    BreakGlass --> AuditLog

    FHIRGateway <--> RecordAPI
    FHIRGateway <--> ExternalSystems
```

**Key idea:** EVERY path to patient data — normal role-based access, emergency break-glass access, and external system integration via FHIR — funnels through the same Record API, which unconditionally logs to the tamper-evident audit log (the same core design covered in the dedicated Tamper-Evident Audit Log document) before returning any data. There is no access path that bypasses auditing.

---

## 4. Data Model

```mermaid
erDiagram
    PATIENT_RECORD {
        string patient_id PK
        map demographics "encrypted"
        list diagnoses
        list medications
        list lab_results
    }
    ACCESS_GRANT {
        string grant_id PK
        string provider_id
        string patient_id FK
        string access_level "full/limited/emergency"
        string relationship "treating_physician/consulting/etc"
        timestamp granted_at
        timestamp expires_at
    }
    RECORD_ACCESS_LOG {
        string log_id PK
        string provider_id
        string patient_id FK
        string access_type "view/modify/break_glass"
        string justification "required for break_glass"
        timestamp accessed_at
    }
```

---

## 5. Role-Based Access Control Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Physician as Physician
    participant AuthNZ as Auth & Authorization
    participant AccessGrants as Access Grant Store
    participant RecordAPI as Record API
    participant AuditLog as Audit Log

    Physician->>AuthNZ: Request patient_123's record

    AuthNZ->>AccessGrants: Check: does this physician<br/>have an active, valid<br/>ACCESS_GRANT for patient_123?<br/>(e.g., are they the<br/>CURRENTLY TREATING physician,<br/>not just any physician in<br/>the system)

    alt Valid grant exists
        AccessGrants-->>AuthNZ: Authorized, access_level=full
        AuthNZ->>RecordAPI: Fetch record
        RecordAPI->>AuditLog: Log access<br/>(unconditional — even<br/>successful, authorized<br/>access is logged)
        RecordAPI-->>Physician: Return record
    else No valid grant
        AccessGrants-->>AuthNZ: Not authorized
        AuthNZ->>AuditLog: Log DENIED access attempt<br/>(equally important —<br/>repeated denied attempts<br/>could indicate a security<br/>issue worth investigating)
        AuthNZ-->>Physician: Access denied
    end
```

**Why access is tied to an active clinical RELATIONSHIP, not just a general role:** A "physician" role alone is far too broad an access grant — a cardiologist shouldn't have unrestricted access to every patient in the hospital, only those they're actually currently treating or consulting on. Access grants must be tied to genuine, verifiable clinical relationships (treating physician, active consult, care team membership), automatically expiring when that relationship ends (e.g., patient discharge).

---

## 6. Break-Glass Emergency Access Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant EmergencyStaff as Emergency Room Staff
    participant BreakGlass as Break-Glass Handler
    participant RecordAPI as Record API
    participant AuditLog as Audit Log
    participant ComplianceReview as Compliance Review Queue

    EmergencyStaff->>BreakGlass: Emergency access request<br/>for unconscious patient_456<br/>{justification: "Patient<br/>unresponsive, need allergy/<br/>medication history"}

    BreakGlass->>BreakGlass: NO normal access-grant<br/>check required — this is<br/>the DELIBERATE override<br/>mechanism, but requires<br/>MANDATORY justification text

    BreakGlass->>RecordAPI: Grant TEMPORARY, IMMEDIATE<br/>access
    RecordAPI-->>EmergencyStaff: Return record<br/>(access granted WITHOUT<br/>the delay of normal<br/>authorization checks)

    BreakGlass->>AuditLog: Log with SPECIAL FLAG:<br/>break_glass_access=true,<br/>justification captured

    BreakGlass->>ComplianceReview: AUTOMATICALLY queue for<br/>MANDATORY post-hoc review<br/>(every single break-glass<br/>access gets reviewed,<br/>not just suspicious-looking<br/>ones)

    Note over ComplianceReview: Compliance team later<br/>verifies: was this<br/>genuinely a legitimate<br/>emergency, or was the<br/>mechanism misused?<br/>Misuse carries serious<br/>consequences — this<br/>after-the-fact review is<br/>what allows the mechanism<br/>to grant immediate access<br/>WITHOUT immediate<br/>verification, while still<br/>maintaining accountability
```

**Why break-glass access trades upfront verification for guaranteed after-the-fact review, rather than eliminating verification entirely:** In a genuine emergency, delaying access for verification could cost a life — but removing accountability entirely would create an obvious security hole. The solution is to grant access IMMEDIATELY (optimizing for the emergency case) while making EVERY such access automatically and mandatorily subject to compliance review afterward (optimizing for accountability) — nobody can quietly use break-glass access without it being reviewed.

---

## 7. FHIR/HL7 Interoperability Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant ExternalLab as External Lab System
    participant FHIRGateway as FHIR Gateway
    participant RecordAPI as Record API
    participant RecordStore as Record Store

    ExternalLab->>FHIRGateway: Submit lab result<br/>(FHIR-formatted<br/>Observation resource)

    FHIRGateway->>FHIRGateway: Validate against FHIR<br/>schema/profile<br/>(different labs may use<br/>slightly different FHIR<br/>versions/extensions —<br/>gateway must handle this<br/>heterogeneity)

    FHIRGateway->>FHIRGateway: Transform external FHIR<br/>representation into the<br/>system's INTERNAL record<br/>format

    FHIRGateway->>RecordAPI: Submit as an authenticated,<br/>authorized write<br/>(external system integration<br/>ALSO goes through the same<br/>access control and audit<br/>logging as any other write)

    RecordAPI->>RecordStore: Store lab result,<br/>associated with correct<br/>patient record

    RecordAPI-->>FHIRGateway: Confirmed
    FHIRGateway-->>ExternalLab: FHIR-formatted<br/>acknowledgment
```

**Why the FHIR Gateway exists as a distinct translation layer, not direct external access to the Record API:** Real-world healthcare interoperability involves genuine heterogeneity — different external systems implement FHIR/HL7 standards with subtle variations, different resource profiles, sometimes different protocol versions entirely (HL7 v2 messaging vs FHIR REST). The gateway isolates this external-facing complexity and variability from the internal record system's clean, consistent internal data model.

---

## 8. Field-Level Access Restriction (Sensitive Record Sections)

```mermaid
flowchart TB
    A["Not all parts of a patient's<br/>record carry the same<br/>sensitivity — e.g., mental<br/>health notes, substance abuse<br/>treatment history, and HIV<br/>status often carry ADDITIONAL<br/>legal protections beyond<br/>general medical record<br/>privacy requirements"] --> B["Access control must operate<br/>at the FIELD/SECTION level,<br/>not just the whole-record<br/>level — a provider might be<br/>authorized to see a<br/>patient's general medical<br/>history but NOT their<br/>separately-protected mental<br/>health treatment notes,<br/>without EXPLICIT additional<br/>authorization for that<br/>specific category"]

    B --> C["This requires the<br/>ACCESS_GRANT model (Section 4)<br/>to support granular scoping —<br/>not just 'access to patient X'<br/>but 'access to patient X's<br/>GENERAL records, EXCLUDING<br/>specially-protected<br/>categories unless separately<br/>authorized'"]
```

---

## 9. Comprehensive Access Auditing (Beyond Just Writes)

```mermaid
flowchart TB
    A["Unlike many systems where<br/>audit logging focuses primarily<br/>on WRITES/modifications,<br/>healthcare records require<br/>auditing EVERY READ ACCESS<br/>as well"] --> B["Why: unauthorized VIEWING<br/>of a patient's record<br/>(e.g., a curious employee<br/>looking up a celebrity<br/>patient, or an ex-partner's<br/>medical history) is ITSELF<br/>a serious privacy violation<br/>and regulatory breach, even<br/>if nothing was ever modified"]

    B --> C["This means the audit log<br/>volume in a healthcare<br/>system is typically MUCH<br/>higher than in systems<br/>auditing only writes — every<br/>single record view, by every<br/>provider, must be captured<br/>with the SAME tamper-evident<br/>rigor as the dedicated Audit<br/>Log design, sized for this<br/>much higher volume"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Healthcare Records HLD))
    Auth and Authorization
      Relationship-based access grants
      Field-level scoping
    Break-Glass Handler
      Immediate emergency access
      Mandatory post-hoc review
    Record API
      Single funnel for all access paths
      Unconditional audit logging
    FHIR Gateway
      External interoperability translation
      Handles standard heterogeneity
    Audit Log
      Read AND write logging
      Tamper-evident, high volume
    Compliance Review Queue
      Mandatory break-glass verification
      Accountability without upfront delay
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Access control model | Relationship-based, not just role-based | A broad "physician" role is far too permissive; access must be tied to a genuine, verifiable, time-bounded clinical relationship |
| Emergency access | Break-glass with mandatory post-hoc review | Balances the genuine risk of delayed emergency care against the need for accountability, rather than choosing one at the expense of the other |
| External interoperability | Dedicated FHIR Gateway translation layer | Isolates the internal record system from the genuine heterogeneity of external systems' standard implementations |
| Access granularity | Field/section-level, not whole-record only | Certain data categories (mental health, substance abuse) carry additional legal protection requiring finer-grained control than whole-record access |
| Audit scope | Every read AND every write | Unauthorized viewing alone is a serious privacy violation in healthcare, unlike many systems where only modifications warrant audit-level scrutiny |
| Auditing mechanism | Tamper-evident (same design as dedicated Audit Log system) | Regulatory and legal stakes require the same non-repudiation guarantees established in that dedicated design |

---

## 12. Bottlenecks & Scaling Considerations

- **Audit log volume from comprehensive read-tracking** — logging every single view (not just writes) generates substantially higher audit volume than most systems in this document series; needs the same tiered storage/retention strategy as the general Audit Log and Log Aggregation designs, but sized for this healthcare-specific higher baseline.
- **Break-glass review workload scaling** — as break-glass access is used across a large healthcare network, the mandatory compliance review queue needs adequate staffing/tooling to keep pace, similar to the review-queue scaling concerns in the Content Moderation design — a growing unreviewed backlog undermines the accountability the mechanism depends on.
- **Access grant expiration and relationship tracking accuracy** — the system's access control is only as good as its underlying knowledge of WHO is currently treating WHICH patient; this requires reliable integration with hospital admission/discharge/transfer systems to keep access grants accurately time-bounded, a significant real-world integration challenge beyond the pure access-control logic itself.
- **FHIR version and profile heterogeneity** — different external labs, pharmacies, and hospital systems may implement different FHIR versions or custom extensions/profiles; the gateway's translation logic requires ongoing maintenance as new external partners integrate, each potentially bringing their own implementation quirks.
- **Availability requirements during genuine emergencies** — unlike most systems where "the system was briefly unavailable" is an inconvenience, a records system outage during active emergency care has direct patient-safety implications; this justifies substantially higher investment in redundancy and graceful degradation (e.g., cached recent-access data available even during a partial outage) than typical business-system availability requirements would warrant.
- **Cross-institution identity resolution** — when a patient receives care across MULTIPLE healthcare institutions/systems, correctly linking their records as the SAME person (without incorrectly merging different people, or missing that separate records belong to the same person) is a genuinely hard identity-resolution problem with direct patient-safety consequences if done incorrectly — a significant challenge beyond this design's core access-control and interoperability architecture.
- **Regulatory variation across jurisdictions** — a healthcare system operating across multiple regions/countries faces varying specific regulatory requirements (HIPAA in the US, GDPR-health-data provisions in the EU, and others), meaning the field-level protection categories and consent/access requirements from Section 8 may need to be configurable per jurisdiction rather than a single global policy, connecting to similar regulatory-variation considerations as the GDPR Deletion System design.
