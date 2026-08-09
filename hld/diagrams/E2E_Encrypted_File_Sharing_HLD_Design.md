# Design an End-to-End Encrypted File Sharing System — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Users can upload files that are encrypted such that even the storage provider cannot read the content
- Users can share encrypted files with specific other users, granting them (and only them) decryption capability
- Support revoking a previously-shared user's access
- Support large file uploads/downloads efficiently despite the encryption overhead

### Non-Functional Requirements
- **True end-to-end encryption:** The server must NEVER have access to plaintext file content or the keys needed to decrypt it — this is the core, non-negotiable security property
- **Usability despite complexity:** Key management is inherently complex; the system must make this invisible/manageable for ordinary users, not just cryptography experts
- **Performance:** Encryption/decryption overhead should be imperceptible for typical file sizes
- **Revocation effectiveness:** Once access is revoked, the revoked user should not be able to decrypt NEWLY shared updates (though already-downloaded plaintext copies are inherently outside the system's control)

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Files stored | Billions |
| Avg file size | Highly variable, KB to GB |
| Sharing operations/sec | Thousands |
| Key operations (encrypt/decrypt) | Client-side, scales with user activity not server load |

---

## 2. The Core Principle — The Server Is a Blind Storage Relay

```mermaid
flowchart TB
    A["Traditional cloud storage:<br/>server encrypts data at rest,<br/>but the SERVER holds the<br/>encryption keys — meaning the<br/>provider (or anyone who<br/>compromises the provider)<br/>CAN technically access<br/>plaintext content"] --> A1["This is 'encryption at rest,'<br/>NOT end-to-end encryption —<br/>an important distinction"]

    B["True end-to-end encryption:<br/>ALL encryption/decryption<br/>happens on the CLIENT device,<br/>using keys the server NEVER<br/>possesses — the server only<br/>ever stores and transmits<br/>opaque CIPHERTEXT it cannot<br/>itself decrypt"] --> B1["Same foundational principle<br/>as the WhatsApp/Messenger<br/>design's 'server as blind<br/>relay' — applied here to<br/>file storage instead of<br/>real-time messages"]
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph ClientA["Uploader's Device"]
        FileA["Original File"]
        EncEngineA["Client-Side Encryption Engine"]
        KeyStoreA["Local Private Key Store<br/>(never leaves device)"]
    end

    subgraph Server["Server (Blind Storage Relay)"]
        UploadSvc["Upload Service"]
        BlobStore[("Encrypted Blob Storage<br/>— server CANNOT decrypt")]
        KeyDirectory[("Public Key Directory<br/>— per-user public keys only")]
        ShareMetadata[("Share Metadata<br/>— WHO has access,<br/>not the actual keys")]
    end

    subgraph ClientB["Recipient's Device"]
        KeyStoreB["Local Private Key Store<br/>(never leaves device)"]
        EncEngineB["Client-Side Decryption Engine"]
        FileB["Decrypted File<br/>(only ever exists<br/>on recipient's device)"]
    end

    FileA --> EncEngineA
    EncEngineA -->|"encrypted ciphertext"| UploadSvc
    UploadSvc --> BlobStore
    EncEngineA --> KeyDirectory
    EncEngineA --> ShareMetadata

    KeyStoreA -.->|"used locally only,<br/>NEVER transmitted"| EncEngineA

    BlobStore -->|"encrypted ciphertext<br/>(server never decrypts)"| EncEngineB
    ShareMetadata --> EncEngineB
    KeyStoreB -.->|"used locally only"| EncEngineB
    EncEngineB --> FileB
```

**Key idea:** Note precisely what the server DOES and DOESN'T store: encrypted file blobs (opaque to the server), public keys (safe to share by definition), and share metadata (WHO should have access — a list of user IDs, not decryption capability itself). The actual private keys capable of decryption never leave the client devices that generated them.

---

## 4. Data Model

```mermaid
erDiagram
    USER ||--o| KEY_PAIR : "has public key registered"
    FILE ||--o{ FILE_KEY_WRAP : "has encryption key wrapped for"
    FILE ||--o{ SHARE_GRANT : "shared with"

    USER {
        string user_id PK
        bytes public_key "safe to store server-side"
    }
    FILE {
        string file_id PK
        string owner_id FK
        bytes encrypted_blob_ref "pointer to encrypted storage"
        bytes encrypted_metadata "filename, etc, also encrypted"
    }
    FILE_KEY_WRAP {
        string file_id FK
        string recipient_user_id FK
        bytes wrapped_file_key "the file's symmetric key,<br/>encrypted specifically FOR<br/>this recipient's public key"
    }
    SHARE_GRANT {
        string file_id FK
        string user_id FK
        timestamp granted_at
        timestamp revoked_at "nullable"
    }
```

**Key modeling concept — "key wrapping":** Rather than encrypting the entire (potentially huge) file separately for every recipient, the file is encrypted ONCE with a randomly generated symmetric key (fast, efficient for large data). That symmetric key is then itself encrypted ("wrapped") individually for each authorized recipient using their public key — a small, cheap operation repeated per-recipient, while the expensive full-file encryption happens only once.

---

## 5. File Upload & Initial Encryption Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant User as Uploader (Client)
    participant EncEngine as Client Encryption Engine
    participant Server as Server (Upload Service)
    participant BlobStore as Encrypted Blob Storage

    User->>EncEngine: Select file to upload

    EncEngine->>EncEngine: Generate a random<br/>SYMMETRIC key<br/>(unique per file,<br/>e.g., AES-256 key)
    EncEngine->>EncEngine: Encrypt the FULL FILE<br/>content using this<br/>symmetric key<br/>(fast — symmetric<br/>encryption scales well<br/>to large files)

    EncEngine->>EncEngine: Wrap (encrypt) the<br/>symmetric key using the<br/>UPLOADER's OWN public key<br/>(so the owner can decrypt<br/>their own file later)

    EncEngine->>Server: Upload: encrypted file blob<br/>+ wrapped key (for owner)<br/>+ encrypted filename/metadata

    Server->>BlobStore: Store encrypted blob<br/>(server has NO ability<br/>to decrypt any of this)
    Server-->>User: Upload complete
```

---

## 6. Sharing With Another User — Detailed Sequence

```mermaid
sequenceDiagram
    participant Owner as File Owner (Client)
    participant OwnerEnc as Owner's Encryption Engine
    participant Server as Server
    participant KeyDir as Public Key Directory
    participant Recipient as Recipient (Client, later)
    participant RecipEnc as Recipient's Decryption Engine

    Owner->>Server: "Share file X with user Bob"

    Server->>KeyDir: Lookup Bob's PUBLIC key<br/>(safe — public keys are<br/>meant to be shared)
    KeyDir-->>Server: Bob's public key
    Server-->>Owner: Return Bob's public key

    Owner->>OwnerEnc: Unwrap the file's symmetric<br/>key using OWNER's OWN<br/>private key (decrypting<br/>the key they wrapped<br/>for themselves at upload time)
    OwnerEnc->>OwnerEnc: Re-wrap (encrypt) that SAME<br/>symmetric key using<br/>BOB's public key

    Owner->>Server: Store: new FILE_KEY_WRAP<br/>entry {file_id, recipient=Bob,<br/>wrapped_key_for_bob}<br/>+ SHARE_GRANT record

    Note over Server: Server stores this wrapped<br/>key but STILL cannot decrypt<br/>it — it's encrypted<br/>specifically for Bob's<br/>private key, which the<br/>server never possesses

    Note over Recipient: LATER, when Bob wants<br/>to access the file
    Recipient->>Server: Request file X
    Server-->>Recipient: Encrypted blob +<br/>Bob's specific wrapped key

    Recipient->>RecipEnc: Unwrap the symmetric key<br/>using Bob's OWN private key<br/>(only Bob can do this —<br/>his private key never<br/>left his device)
    RecipEnc->>RecipEnc: Decrypt the file blob<br/>using the now-unwrapped<br/>symmetric key
    RecipEnc-->>Recipient: Plaintext file<br/>(exists ONLY on Bob's device)
```

**Why re-wrapping (not re-encrypting the whole file) makes sharing efficient:** The expensive operation — encrypting the entire file — happened exactly once, at upload time. Sharing with a new recipient only requires the cheap operation of encrypting the small symmetric key for that recipient's public key — this is what makes end-to-end encrypted sharing practical even for very large files shared with many people, since the per-recipient cost stays small and constant regardless of file size.

---

## 7. Revoking Access

```mermaid
flowchart TB
    A["Owner revokes Bob's<br/>access to File X"] --> B["Server marks Bob's<br/>SHARE_GRANT as revoked"]
    B --> C["Server will no longer<br/>serve the encrypted blob<br/>OR Bob's wrapped key<br/>to Bob's future requests"]

    D["CRITICAL LIMITATION —<br/>must be clearly understood:"] --> E["If Bob ALREADY downloaded<br/>and decrypted the file<br/>BEFORE revocation, he has<br/>a plaintext copy on HIS<br/>device — this is fundamentally<br/>OUTSIDE the system's control.<br/>Revocation prevents FUTURE<br/>access, it cannot retroactively<br/>un-decrypt data already<br/>decrypted"]

    F["For content requiring true<br/>retroactive revocation (e.g.,<br/>time-limited access), the file<br/>would need to be RE-ENCRYPTED<br/>with a NEW symmetric key,<br/>and only re-shared (re-wrapped)<br/>with STILL-authorized users —<br/>this is a fundamentally<br/>different, heavier operation"] -.-> D
```

**Why this limitation is worth stating explicitly in any real design discussion:** A common misconception is that "revoke access" in an E2E encrypted system works like revoking a database permission — instantly and completely. In reality, once plaintext has been decrypted on a device, no cryptographic mechanism can reach into that device and destroy it. Being explicit about this boundary is a sign of genuine security understanding, not a design flaw to hide.

---

## 8. Key Rotation for True Revocation (When Needed)

```mermaid
sequenceDiagram
    participant Owner as File Owner
    participant OwnerEnc as Owner's Encryption Engine
    participant Server as Server
    participant RemainingUsers as Still-Authorized Users

    Note over Owner: Owner wants to ensure<br/>Bob (revoked) cannot decrypt<br/>even a FUTURE snapshot<br/>of this file

    OwnerEnc->>OwnerEnc: Generate a BRAND NEW<br/>symmetric key
    OwnerEnc->>OwnerEnc: Re-encrypt file content<br/>with the NEW key

    Owner->>Server: Upload re-encrypted blob<br/>(replaces old version)

    loop For each STILL-authorized user (excluding Bob)
        OwnerEnc->>OwnerEnc: Wrap the NEW symmetric key<br/>with that user's public key
        Owner->>Server: Store new FILE_KEY_WRAP<br/>for this user
    end

    Note over Server: Bob's OLD wrapped key still<br/>exists but is now USELESS —<br/>it unwraps to the OLD<br/>symmetric key, which no<br/>longer decrypts the<br/>CURRENT file blob
```

---

## 9. Multi-Device Support (Same User, Multiple Devices)

```mermaid
flowchart TB
    A["User has a Private Key<br/>ONLY on their original device —<br/>how do they access files<br/>from a NEW device?"] --> B{"Multi-Device Key Strategy"}

    B --> C["Option 1: Generate a<br/>NEW key pair per device,<br/>requiring existing devices<br/>to individually re-wrap<br/>keys for the new device<br/>(similar to sharing with<br/>a new 'recipient')"]

    B --> D["Option 2: Securely export/<br/>sync the private key<br/>itself across the user's<br/>own devices (e.g., encrypted<br/>with a user-chosen<br/>passphrase, similar to the<br/>Signal Protocol's device<br/>linking approach)"]

    E["This mirrors the same<br/>multi-device complexity<br/>noted in the WhatsApp/<br/>Messenger design — E2E<br/>encrypted systems inherently<br/>make 'just add a new device'<br/>significantly more involved<br/>than in a system where the<br/>server can freely manage<br/>keys on the user's behalf"] -.-> C
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((E2E Encrypted File Sharing HLD))
    Client Encryption Engine
      Generates symmetric file keys
      Encrypts/decrypts locally
      Wraps keys for recipients
    Local Private Key Store
      Never transmitted to server
      Root of all decryption capability
    Server (Blind Relay)
      Stores encrypted blobs only
      Stores public keys and wrapped keys
      Cannot decrypt anything
    Public Key Directory
      Safe, shareable public keys
      Enables key wrapping for new recipients
    Share Metadata
      Access control list
      Does not grant decryption itself
    Key Rotation Process
      True revocation mechanism
      Re-encrypts and re-wraps for remaining users
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Encryption architecture | Client-side only, server as blind relay | The defining, non-negotiable property of true end-to-end encryption — server compromise cannot expose plaintext |
| File encryption approach | Hybrid: symmetric key for file content, asymmetric wrapping per recipient | Symmetric encryption scales efficiently to large files; per-recipient asymmetric wrapping is cheap and avoids re-encrypting the whole file for each share |
| Sharing mechanism | Re-wrap the existing symmetric key for new recipients | Avoids the expensive full-file re-encryption cost when adding a new authorized viewer |
| Standard revocation | Access-list based (prevents future server-mediated access) | Simple and sufficient for most use cases, with the explicit, honestly-communicated limitation that already-decrypted plaintext is outside the system's control |
| True/retroactive revocation | Key rotation + full re-encryption when required | The only cryptographically meaningful way to ensure a revoked user cannot decrypt content going forward, at the cost of a heavier re-encryption operation |
| Multi-device support | Explicit key-sharing/linking process between devices | Inherent complexity tradeoff of true E2E encryption — the server cannot transparently manage keys across a user's devices the way it could in a non-E2E system |

---

## 12. Bottlenecks & Scaling Considerations

- **Client-side computational cost** — encryption/decryption happens entirely on user devices, which is a deliberate security property but does mean very large files or low-powered devices (older phones) may experience meaningfully slower upload/download compared to a non-encrypted equivalent — this cost is fundamental to the security model, not an implementation inefficiency to be optimized away.
- **Sharing with many recipients** — while re-wrapping is cheap per recipient, sharing a file with thousands of people (e.g., a large team) still means thousands of individual wrap operations and stored FILE_KEY_WRAP records; this remains far more efficient than full re-encryption per recipient, but isn't entirely free at extreme scale.
- **Key loss = permanent data loss** — since the server never has access to plaintext or unwrapped keys, if a user loses their private key (device lost/reset without backup) with no other authorized party able to re-share, that file's content is genuinely, cryptographically unrecoverable — this is the direct, unavoidable cost of true end-to-end encryption and must be clearly communicated to users, often necessitating a carefully-designed (and separately security-reviewed) key backup/recovery mechanism.
- **Search and preview functionality limitations** — since the server cannot see file content, server-side features common in non-encrypted systems (full-text search across file contents, automatic thumbnail generation, virus scanning) become impossible or require entirely different approaches (e.g., client-side search index generation, encrypted separately) — this is a genuine, inherent product tradeoff of choosing true E2E encryption.
- **Metadata leakage** — even with content fully encrypted, some metadata (file sizes, sharing patterns, access timing, who-shares-with-whom social graph) may still be visible to the server unless specifically designed to also be protected/obscured — a thorough security design must explicitly decide what metadata protection guarantees are and aren't being made, rather than assuming "encrypted" means "fully private in every dimension."
- **Verifying public key authenticity (trust establishment)** — the entire security model depends on users genuinely having the CORRECT public key for their intended recipient, not an attacker-substituted one (a man-in-the-middle risk); production systems need mechanisms like key fingerprint verification or a trusted key transparency log to give users genuine assurance they're encrypting for the right person, not just trusting whatever public key the server happens to hand back.
