# Design a Real-Time ML Model Serving System — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Serve real-time predictions from trained ML models via a low-latency API
- Support multiple model versions running simultaneously (for A/B testing and gradual rollout)
- Support canary/gradual rollout of new model versions with automated rollback on regression
- Support multiple model types/frameworks (not locked to a single ML framework)

### Non-Functional Requirements
- **Low inference latency:** Must fit within the caller's overall latency budget (often tens of milliseconds)
- **High availability:** Model serving is often on the critical path of user-facing features (recommendations, fraud checks, ranking) — cannot be a fragile single point of failure
- **Safe rollout:** A newly deployed model with a subtle bug (degraded accuracy, or an outright bug) must be caught and rolled back BEFORE it causes wide business impact
- **Resource efficiency:** GPU/specialized inference hardware is expensive — must be utilized efficiently, not provisioned wastefully

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Inference requests/sec (platform-wide) | Tens of thousands to millions, depending on use case |
| Latency budget per inference | 10-50ms typical for synchronous use cases |
| Model versions active simultaneously | Multiple (production + canary + A/B variants) |
| Model size | KBs (simple models) to GBs (large deep learning models) |

---

## 2. High-Level Architecture

```mermaid
flowchart TB
    Client["Calling Application<br/>(e.g., recommendation service,<br/>fraud check)"]

    subgraph Serving["Model Serving Layer"]
        LB["Load Balancer /<br/>Traffic Router"]
        ModelServerA["Model Server<br/>(Version A - production)"]
        ModelServerB["Model Server<br/>(Version B - canary)"]
    end

    subgraph Registry["Model Management"]
        ModelRegistry[("Model Registry<br/>— versions, metadata,<br/>artifacts")]
        DeploymentCtrl["Deployment Controller"]
    end

    subgraph Monitoring["Monitoring & Rollback"]
        MetricsCollector["Metrics Collector<br/>(latency, error rate,<br/>prediction distribution)"]
        RollbackEngine["Automated Rollback Engine"]
    end

    Client --> LB
    LB -->|"95% of traffic"| ModelServerA
    LB -->|"5% of traffic<br/>(canary)"| ModelServerB

    ModelServerA --> MetricsCollector
    ModelServerB --> MetricsCollector
    MetricsCollector --> RollbackEngine
    RollbackEngine -->|"triggers rollback<br/>if regression detected"| LB

    DeploymentCtrl --> ModelRegistry
    DeploymentCtrl -->|"deploys new versions"| ModelServerB
```

**Key idea:** Multiple model versions run SIMULTANEOUSLY behind a traffic-splitting router — this is what enables canary deployment (a small traffic percentage validates a new model in production before full rollout) and A/B testing (comparing model variants' real business impact), rather than the risky all-or-nothing approach of deploying a new model directly to 100% of traffic.

---

## 3. Data Model

```mermaid
erDiagram
    MODEL_VERSION {
        string model_id PK
        string version
        string framework "tensorflow/pytorch/xgboost"
        string artifact_location
        string status "canary/production/deprecated"
        timestamp deployed_at
    }
    TRAFFIC_ROUTE {
        string model_id FK
        string version FK
        float traffic_percentage
    }
    PREDICTION_LOG {
        string request_id PK
        string model_id FK
        string version FK
        string input_features_hash
        string prediction_output
        float latency_ms
        timestamp predicted_at
    }
```

---

## 4. Real-Time Inference Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant App as Calling Application
    participant LB as Traffic Router
    participant FS as Feature Store<br/>(as in the dedicated design)
    participant Model as Model Server
    participant Logger as Prediction Logger

    App->>LB: Predict request<br/>{entity_id, context}
    LB->>LB: Determine routing<br/>(production vs canary,<br/>based on configured split)

    LB->>FS: Fetch feature vector<br/>for entity_id<br/>(same online feature store<br/>as covered in that design)
    FS-->>LB: Feature vector

    LB->>Model: Run inference<br/>{feature_vector}
    Model->>Model: Execute forward pass<br/>through the model

    Model-->>LB: Prediction output
    LB->>Logger: Async log: request details,<br/>prediction, latency<br/>(doesn't block the response)
    LB-->>App: Return prediction
```

---

## 5. Canary Deployment & Automated Rollback — Detailed Sequence

```mermaid
sequenceDiagram
    participant DeployCtrl as Deployment Controller
    participant LB as Traffic Router
    participant CanaryModel as Canary Model Server<br/>(new version)
    participant ProdModel as Production Model Server<br/>(current version)
    participant Metrics as Metrics Collector
    participant Rollback as Rollback Engine

    DeployCtrl->>CanaryModel: Deploy new model version
    DeployCtrl->>LB: Route 5% of traffic to canary,<br/>95% remains on production

    loop Continuous monitoring during canary period
        CanaryModel->>Metrics: Report latency, error rate,<br/>prediction distribution
        ProdModel->>Metrics: Report same metrics<br/>(baseline for comparison)

        Metrics->>Rollback: Compare canary vs<br/>production metrics
        Rollback->>Rollback: Check: latency regression?<br/>Error rate spike?<br/>Prediction distribution<br/>shift (potential accuracy<br/>issue)?
    end

    alt Canary metrics healthy
        Rollback->>DeployCtrl: Approve gradual ramp
        DeployCtrl->>LB: Increase canary traffic:<br/>5% → 25% → 50% → 100%
        Note over LB: Full rollout, old version<br/>deprecated
    else Regression detected
        Rollback->>LB: IMMEDIATE rollback:<br/>route 100% back to<br/>production version
        Rollback->>DeployCtrl: Alert on-call,<br/>halt rollout
    end
```

**Why prediction distribution shift matters as a monitoring signal, not just latency/errors:** A buggy model can be perfectly fast and never throw errors, while still producing SYSTEMATICALLY WRONG predictions (e.g., a fraud model that suddenly scores everything as low-risk due to a feature pipeline bug) — monitoring the STATISTICAL DISTRIBUTION of predictions (not just operational health metrics) is essential for catching this class of silent correctness regression that pure infrastructure monitoring would completely miss.

---

## 6. Multi-Framework Model Serving

```mermaid
flowchart TB
    A["Different teams/models use<br/>different ML frameworks<br/>(TensorFlow, PyTorch, XGBoost,<br/>scikit-learn)"] --> B{"Serving Strategy"}

    B --> C["Framework-specific<br/>serving runtimes<br/>(e.g., TensorFlow Serving,<br/>TorchServe)"]
    C --> C1["PRO: optimized performance<br/>for that specific framework<br/>CON: operational complexity<br/>of running multiple different<br/>serving systems"]

    B --> D["Unified model format<br/>(e.g., ONNX — models exported<br/>to a common intermediate<br/>representation)"]
    D --> D1["PRO: single serving runtime<br/>handles models from ANY<br/>source framework<br/>CON: export process can<br/>introduce subtle numerical<br/>differences, requires<br/>validation"]

    E["Most large-scale platforms<br/>use a HYBRID: standardize on<br/>a unified format where<br/>feasible, while supporting<br/>framework-native serving for<br/>models with specialized<br/>requirements that don't<br/>export cleanly"] -.-> D1
```

---

## 7. Handling High-Throughput via Batching

```mermaid
flowchart TB
    A["Individual requests arrive<br/>continuously, one at a time"] --> B["Naive: run inference<br/>separately for EACH request"]
    B --> B1["Inefficient — GPU/specialized<br/>hardware achieves much<br/>higher THROUGHPUT when<br/>processing requests in<br/>BATCHES rather than one<br/>at a time, due to hardware<br/>parallelism characteristics"]

    A --> C["Dynamic batching:<br/>accumulate incoming requests<br/>for a SHORT window<br/>(e.g., a few milliseconds),<br/>then run inference on the<br/>accumulated BATCH together"]
    C --> D["Tradeoff: adds a small<br/>amount of latency (waiting<br/>to accumulate the batch)<br/>in exchange for dramatically<br/>higher throughput per unit<br/>of expensive hardware"]
```

```mermaid
sequenceDiagram
    participant R1 as Request 1
    participant R2 as Request 2
    participant R3 as Request 3
    participant Batcher as Dynamic Batcher
    participant Model as Model (GPU)

    R1->>Batcher: Inference request
    Batcher->>Batcher: Start batch window (e.g., 5ms)
    R2->>Batcher: Inference request<br/>(arrives during window)
    R3->>Batcher: Inference request<br/>(arrives during window)

    Note over Batcher: Window closes (5ms elapsed,<br/>or batch size limit reached)

    Batcher->>Model: Run inference on<br/>BATCHED input [R1, R2, R3]<br/>(single, efficient GPU call)
    Model-->>Batcher: Batched predictions

    Batcher-->>R1: Individual result
    Batcher-->>R2: Individual result
    Batcher-->>R3: Individual result
```

**Why this specific tradeoff (small added latency for large throughput gain) is usually worth it:** For GPU-backed inference specifically, the overhead of launching a single-item computation is often comparable to launching a much larger batch — meaning batching can improve throughput by 10-50x+ for only a few milliseconds of added latency, a highly favorable tradeoff when the per-request latency budget has room to spare.

---

## 8. Model Warm-Up (Avoiding Cold-Start Latency)

```mermaid
flowchart TB
    A["New model server instance<br/>starts up (e.g., during<br/>auto-scaling, or a fresh<br/>canary deployment)"] --> B["Problem: the FIRST few<br/>inference requests to a<br/>freshly-loaded model can<br/>be significantly slower<br/>(JIT compilation, GPU memory<br/>allocation, framework<br/>initialization overhead)"]

    B --> C["Solution: Warm-up phase —<br/>before accepting real<br/>traffic, the new instance<br/>runs several SYNTHETIC<br/>inference requests internally<br/>to 'prime' the model<br/>(trigger JIT compilation,<br/>allocate memory, etc.)"]

    C --> D["Only AFTER warm-up completes<br/>does the load balancer begin<br/>routing real traffic to this<br/>instance — avoiding exposing<br/>real users to the cold-start<br/>latency spike"]
```

---

## 9. A/B Testing Integration

```mermaid
flowchart TB
    A["Beyond safety-focused canary<br/>deployment, model serving<br/>also supports genuine<br/>A/B TESTING — comparing<br/>two model variants'<br/>real business impact,<br/>not just technical health"] --> B["Traffic split: 50% Model A,<br/>50% Model B (sustained,<br/>not a gradually-ramping<br/>canary)"]

    B --> C["Each prediction tagged<br/>with which model version<br/>served it"]

    C --> D["Downstream business metrics<br/>(click-through rate, conversion,<br/>revenue) are analyzed SEPARATELY<br/>per model version — this<br/>connects to the same<br/>experimentation/analytics<br/>infrastructure used for<br/>the broader News Feed<br/>Ranking design's model<br/>rollout process"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((ML Model Serving HLD))
    Traffic Router
      Version-aware request routing
      Canary and A/B split management
    Model Server
      Loads and executes model
      Dynamic batching
      Warm-up handling
    Model Registry
      Version and artifact tracking
      Deployment source of truth
    Deployment Controller
      Orchestrates rollout stages
      Triggers canary promotion
    Metrics Collector
      Operational and distribution monitoring
      Feeds rollback decisions
    Rollback Engine
      Automated regression detection
      Immediate traffic reversion
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Deployment strategy | Canary with gradual ramp, automated rollback | Catches regressions on a small fraction of traffic before they cause wide business impact, without requiring risky all-or-nothing deployment |
| Monitoring scope | Both operational metrics AND prediction distribution | Operational health alone misses silent correctness regressions (fast, error-free, but systematically wrong predictions) |
| Throughput optimization | Dynamic batching | Achieves dramatically higher inference throughput on specialized hardware for a small, usually-acceptable latency cost |
| Framework support | Hybrid — unified format where feasible, framework-native where needed | Balances operational simplicity against supporting the full range of model types teams actually need to deploy |
| Cold-start handling | Explicit warm-up phase before accepting traffic | Prevents new instances (from scaling or deployment) from exposing real users to elevated first-request latency |
| Rollout vs experimentation | Same infrastructure serves both canary safety checks and genuine A/B testing | Traffic-splitting and per-version tracking are the shared underlying mechanism for both distinct use cases |

---

## 12. Bottlenecks & Scaling Considerations

- **GPU/specialized hardware cost and utilization** — inference hardware is expensive; dynamic batching, careful instance sizing, and potentially multi-model serving on shared hardware (for models too small to justify dedicated GPU allocation) are all important levers for cost efficiency at scale.
- **Feature fetching latency as a hidden bottleneck** — as shown in the inference flow, the feature store lookup often takes as long as or longer than the model inference itself; overall serving latency optimization must consider the ENTIRE request path, not just the model execution step in isolation.
- **Rollback decision latency vs statistical confidence** — detecting a genuine regression (vs normal statistical noise) requires enough canary traffic volume to be confident; for low-traffic models, this can mean either accepting slower rollback decisions or extending canary duration, a real tradeoff between deployment velocity and statistical rigor.
- **Model size and loading time** — very large models (multi-GB deep learning models) can take significant time to load into memory/GPU during deployment or auto-scaling events; needs to be factored into both deployment planning and auto-scaling responsiveness expectations.
- **Multi-model resource contention** — running many different models simultaneously on shared infrastructure (common for cost efficiency) risks one model's traffic spike degrading performance for co-located models; requires careful resource isolation/quotas, similar in spirit to the noisy-neighbor concerns in the Multi-Tenant SaaS Database design.
- **Prediction logging volume** — logging every single prediction (for monitoring, debugging, and potential future retraining data) at high request volume generates substantial data; needs the same tiered storage and sampling considerations as other high-volume logging systems covered elsewhere (e.g., the Log Aggregation design).
- **Version proliferation over time** — without active lifecycle management, the number of "still technically deployed" model versions can grow indefinitely as new versions are continuously shipped; requires clear deprecation policies and cleanup processes to avoid unbounded operational complexity and resource waste from long-abandoned versions still consuming serving capacity.
