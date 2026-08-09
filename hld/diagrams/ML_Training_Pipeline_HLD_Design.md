# Design a Large-Scale ML Training Pipeline — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Train large models (potentially billions of parameters) across many machines/GPUs in parallel
- Support checkpointing so long-running training jobs can resume after failure without starting over
- Support both data parallelism (same model, different data shards) and, for very large models, model parallelism (model itself split across devices)
- Track experiment metadata (hyperparameters, metrics, resulting model artifacts) for reproducibility

### Non-Functional Requirements
- **Fault tolerance:** Training runs spanning days/weeks across hundreds of machines WILL experience individual hardware failures — this must be an expected, handled case, not an exceptional one
- **Efficient resource utilization:** GPU time is extremely expensive — idle/wasted compute directly translates to real cost
- **Scalability:** Must scale from a single-GPU experiment to a training run spanning thousands of GPUs, ideally with minimal code changes
- **Reproducibility:** Given the same code, data, and configuration, training should be able to produce consistent, auditable results

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Training run duration | Days to weeks for large models |
| GPUs involved | Tens to thousands, depending on model/data scale |
| Checkpoint frequency | Every N minutes or N training steps |
| Expected hardware failure rate | Non-trivial at thousand-GPU scale — failures are a statistical CERTAINTY over a multi-week run |

---

## 2. The Core Problem — Why Failures Are Inevitable at This Scale

```mermaid
flowchart TB
    A["Single GPU has some small<br/>probability P of failure<br/>per day (hardware fault,<br/>network blip, host crash)"] --> B["Training job spans 1,000 GPUs<br/>simultaneously, over 2 weeks"]

    B --> C["Probability of AT LEAST ONE<br/>failure somewhere in the<br/>fleet during this run<br/>approaches CERTAINTY —<br/>even a small per-GPU failure<br/>rate compounds dramatically<br/>across enough machines and<br/>enough time"]

    C --> D["Design consequence: the<br/>system CANNOT be built<br/>assuming failures are rare<br/>exceptions — it must be<br/>architected from the ground<br/>up to expect, detect, and<br/>recover from partial failures<br/>as ROUTINE, ongoing events<br/>during a single training run"]
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    subgraph DataLayer["Data Layer"]
        DataStore[("Training Dataset<br/>(sharded, distributed<br/>object storage)")]
    end

    subgraph Orchestration["Training Orchestration"]
        Scheduler["Job Scheduler<br/>(allocates GPU resources)"]
        Coordinator["Training Coordinator<br/>(manages worker lifecycle)"]
    end

    subgraph Workers["Distributed Training Workers"]
        Worker1["Worker 1<br/>(GPU shard 1,<br/>data shard 1)"]
        Worker2["Worker 2<br/>(GPU shard 2,<br/>data shard 2)"]
        WorkerN["Worker N..."]
    end

    subgraph Sync["Gradient Synchronization"]
        AllReduce["All-Reduce<br/>Communication Layer"]
    end

    subgraph Persistence["Checkpointing & Tracking"]
        CheckpointStore[("Checkpoint Storage<br/>— periodic model state snapshots")]
        ExperimentTracker[("Experiment Tracking<br/>— metrics, hyperparameters,<br/>artifacts")]
    end

    DataStore --> Worker1
    DataStore --> Worker2
    DataStore --> WorkerN

    Scheduler --> Coordinator
    Coordinator --> Worker1
    Coordinator --> Worker2
    Coordinator --> WorkerN

    Worker1 <--> AllReduce
    Worker2 <--> AllReduce
    WorkerN <--> AllReduce

    Coordinator --> CheckpointStore
    Worker1 --> ExperimentTracker
```

**Key idea:** Each worker independently processes its own shard of the training data, computes gradients locally, and then all workers synchronize via an "all-reduce" operation — averaging gradients across every worker before each one applies the update to its local copy of the model. This is the essence of data-parallel distributed training: the SAME model exists on every worker, but each sees DIFFERENT data, and they periodically synchronize to stay consistent.

---

## 4. Data Parallelism — How Gradient Synchronization Works

```mermaid
flowchart TB
    A["Training batch split across<br/>N workers, each gets<br/>1/N of the batch"] --> B["Worker 1: forward + backward<br/>pass on its data shard →<br/>local gradient G1"]
    A --> C["Worker 2: forward + backward<br/>pass on its data shard →<br/>local gradient G2"]
    A --> D["Worker N: → local gradient GN"]

    B & C & D --> E["ALL-REDUCE: average<br/>ALL workers' gradients<br/>into a single consensus<br/>gradient: (G1+G2+...+GN)/N"]

    E --> F["EVERY worker applies this<br/>SAME averaged gradient to<br/>update its LOCAL copy of<br/>the model — ensuring all<br/>workers' models stay<br/>identical after each step"]
```

```mermaid
sequenceDiagram
    participant W1 as Worker 1
    participant W2 as Worker 2
    participant W3 as Worker 3
    participant AllReduce as All-Reduce Layer

    par Parallel forward/backward pass
        W1->>W1: Compute gradient G1<br/>on its data shard
    and
        W2->>W2: Compute gradient G2
    and
        W3->>W3: Compute gradient G3
    end

    W1->>AllReduce: Contribute G1
    W2->>AllReduce: Contribute G2
    W3->>AllReduce: Contribute G3

    AllReduce->>AllReduce: Compute average:<br/>(G1+G2+G3)/3

    AllReduce-->>W1: Averaged gradient
    AllReduce-->>W2: Averaged gradient
    AllReduce-->>W3: Averaged gradient

    par Each worker updates independently
        W1->>W1: Apply update to<br/>local model copy
    and
        W2->>W2: Apply update
    and
        W3->>W3: Apply update
    end

    Note over W1,W3: All three workers now<br/>have IDENTICAL model weights,<br/>ready for the next training step
```

---

## 5. Data Model

```mermaid
erDiagram
    TRAINING_RUN {
        string run_id PK
        string model_architecture
        map hyperparameters
        string status "running/completed/failed"
        timestamp started_at
    }
    CHECKPOINT {
        string checkpoint_id PK
        string run_id FK
        int training_step
        string storage_location
        timestamp created_at
    }
    WORKER_STATUS {
        string worker_id PK
        string run_id FK
        string status "healthy/failed/replaced"
        int last_completed_step
        timestamp last_heartbeat
    }
    METRIC_ENTRY {
        string run_id FK
        int training_step
        string metric_name "loss/accuracy/etc"
        float value
        timestamp recorded_at
    }
```

---

## 6. Checkpointing Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Coordinator as Training Coordinator
    participant Workers as All Workers
    participant CheckpointStore as Checkpoint Storage

    loop Every N training steps (e.g., every 500 steps)
        Coordinator->>Workers: Signal: pause for checkpoint

        Workers->>Workers: Ensure all workers have<br/>completed the same step<br/>(synchronization barrier)

        par Each worker saves its shard of state
            Workers->>CheckpointStore: Save model weights,<br/>optimizer state, current<br/>step number
        end

        CheckpointStore-->>Coordinator: All shards confirmed saved
        Coordinator->>Workers: Resume training
    end
```

**Why checkpointing must include optimizer state, not just model weights:** Modern optimizers (like Adam) maintain their own internal running statistics (momentum, variance estimates) that are essential for training to continue smoothly from where it left off — restoring only the model weights without this optimizer state would effectively restart the optimizer's "memory," often causing a training quality regression at the resume point even though the model weights themselves were preserved correctly.

---

## 7. Fault Recovery Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant Coordinator as Training Coordinator
    participant FailedWorker as Worker 47 (fails)
    participant HealthyWorkers as Remaining Workers
    participant Scheduler as Job Scheduler
    participant NewWorker as Replacement Worker
    participant CheckpointStore as Checkpoint Storage

    Note over FailedWorker: Hardware failure —<br/>stops sending heartbeats

    Coordinator->>Coordinator: Detect missed heartbeats<br/>from Worker 47<br/>(same pattern as the<br/>Network Partition Detection<br/>design's failure detection)

    Coordinator->>HealthyWorkers: Pause training<br/>(cannot proceed with a<br/>missing data-parallel shard)

    Coordinator->>Scheduler: Request replacement<br/>GPU resource
    Scheduler->>NewWorker: Provision new worker

    Coordinator->>CheckpointStore: Fetch LATEST checkpoint
    CheckpointStore-->>Coordinator: Checkpoint at step 48,500

    Coordinator->>NewWorker: Load checkpoint,<br/>assume Worker 47's<br/>data shard assignment
    Coordinator->>HealthyWorkers: ALSO reload from the<br/>same checkpoint<br/>(ensures ALL workers are<br/>synchronized to the exact<br/>same step, even the ones<br/>that didn't fail)

    Coordinator->>Coordinator: Resume training from<br/>step 48,500 across the<br/>full worker set
```

**Why even the HEALTHY workers must also reload from the checkpoint:** Since data-parallel training requires every worker to be at the EXACT SAME training step for the all-reduce synchronization to be meaningful, a single worker's failure and replacement means the entire fleet must realign to a common, consistent state — you can't have some workers at step 48,700 and a freshly-restored replacement at step 48,500; the whole ensemble rewinds together.

---

## 8. Elastic Scaling During Training (Advanced)

```mermaid
flowchart TB
    A["Training job initially<br/>allocated 500 GPUs"] --> B{"Cluster-wide resource<br/>availability changes"}

    B --> C["MORE capacity becomes<br/>available (e.g., another<br/>job completed, freeing<br/>GPUs)"]
    C --> D["Elastic scale-UP:<br/>add more workers,<br/>redistribute data shards<br/>across the larger worker<br/>set, adjust effective<br/>batch size/learning rate<br/>accordingly"]

    B --> E["Capacity is being<br/>RECLAIMED (e.g., higher-<br/>priority job needs<br/>resources, spot instance<br/>preemption)"]
    E --> F["Elastic scale-DOWN:<br/>checkpoint current state,<br/>gracefully release some<br/>workers, continue with<br/>fewer resources"]

    G["This capability lets<br/>training jobs OPPORTUNISTICALLY<br/>utilize available cluster<br/>capacity rather than requiring<br/>a fixed, dedicated allocation<br/>for the entire run duration —<br/>significantly improving overall<br/>cluster utilization efficiency"] -.-> D
```

---

## 9. Experiment Tracking & Reproducibility

```mermaid
sequenceDiagram
    participant Researcher as ML Researcher
    participant Coordinator as Training Coordinator
    participant Tracker as Experiment Tracker

    Researcher->>Coordinator: Launch training run<br/>{config: hyperparameters,<br/>dataset_version, code_version}

    Coordinator->>Tracker: Record run metadata<br/>(full configuration snapshot)

    loop Throughout training
        Coordinator->>Tracker: Log metrics<br/>(loss, accuracy, learning rate,<br/>gradient norms, etc.)
    end

    Coordinator->>Tracker: Register final model<br/>artifact + which checkpoint<br/>it came from

    Note over Tracker: Later, ANY researcher can<br/>query: "show me exactly<br/>what config/data/code<br/>produced model version X" —<br/>essential for debugging,<br/>comparison, and audit
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((ML Training Pipeline HLD))
    Training Coordinator
      Orchestrates worker lifecycle
      Manages checkpoint cycles
      Handles failure recovery
    Workers
      Local forward/backward pass
      Participate in all-reduce
    All-Reduce Layer
      Gradient synchronization
      Ensures consistent model state
    Checkpoint Storage
      Periodic full state snapshots
      Includes optimizer state
    Job Scheduler
      GPU resource allocation
      Elastic scaling support
    Experiment Tracker
      Configuration and metrics
      Reproducibility record
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Parallelism strategy | Data parallelism (with model parallelism for very large models) | Matches most training workloads' actual bottleneck; simpler to implement and reason about than full model parallelism when the model fits on a single device |
| Failure handling philosophy | Expected, routine, always-recoverable | At thousand-GPU, multi-week scale, failures are a statistical certainty, not an edge case — the entire system must be architected around this reality |
| Checkpoint content | Full state: model weights + optimizer state + step number | Partial checkpointing (weights only) causes subtle training quality regressions on resume, even though it appears to "work" |
| Recovery scope | Entire worker fleet reloads on any single failure | Data-parallel synchronization requires all workers at the same step; partial-fleet recovery would break this invariant |
| Resource allocation | Elastic scaling support | Improves cluster-wide utilization by allowing training jobs to opportunistically use available capacity, rather than requiring a fixed dedicated allocation |
| Experiment tracking | Mandatory metadata capture for every run | Reproducibility is a hard requirement for ML research/production — without this, "why does this model behave differently" becomes unanswerable |

---

## 12. Bottlenecks & Scaling Considerations

- **All-reduce communication overhead grows with worker count** — as more workers participate, the gradient synchronization step itself becomes an increasingly significant fraction of total training time; efficient all-reduce implementations (e.g., ring-based algorithms) and high-bandwidth interconnects (InfiniBand, NVLink) are essential investments at large scale, not optional infrastructure.
- **Checkpoint size and frequency tradeoff** — checkpointing very large models (many GB of weights + optimizer state) takes real time and I/O bandwidth; too frequent checkpointing wastes valuable training time, too infrequent risks losing more progress on failure — this mirrors the same tradeoff explored in the WAL & Recovery System design, applied to model training instead of database durability.
- **Straggler workers** — even without outright failure, some workers may run measurably slower than others (hardware variance, network congestion) — since all-reduce synchronization means the WHOLE fleet waits for the slowest worker each step, a single consistent straggler can bottleneck the entire training run's throughput.
- **Data loading pipeline bottleneck** — if data loading/preprocessing can't keep pace with GPU compute speed, expensive GPUs sit idle waiting for data — the data pipeline itself often needs its own dedicated scaling and optimization effort, separate from the model computation.
- **Model parallelism complexity for extremely large models** — when a single model no longer fits in one device's memory (common for today's largest models), the model itself must be split across devices, introducing significantly more complex communication patterns than data parallelism's relatively simple gradient averaging — this is a substantially harder engineering problem, often requiring specialized frameworks.
- **Cost management at scale** — given the enormous expense of large GPU fleets running for days/weeks, proactive monitoring of training efficiency (GPU utilization, time lost to failures/stragglers) directly translates to real cost savings — this operational visibility is as important as the training correctness itself for organizations running training at this scale.
