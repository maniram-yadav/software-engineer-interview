# Design an LLM Inference Serving Platform at Scale — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Serve text generation requests from large language models, supporting streaming token-by-token output
- Support many concurrent users/requests against a shared pool of expensive GPU resources
- Support multiple model sizes/versions simultaneously
- Support variable-length inputs and outputs efficiently

### Non-Functional Requirements
- **GPU memory is the primary scarce resource:** Unlike typical inference (bounded by compute), LLM serving is heavily bounded by GPU memory capacity — this shapes nearly every architectural decision
- **Latency characteristics are unusual:** Time-to-first-token matters for perceived responsiveness, while total generation time scales with output length — these are DIFFERENT metrics requiring different optimization
- **Throughput vs latency tradeoff:** Serving many users simultaneously efficiently (throughput) can conflict with making any single user's response as fast as possible (latency)
- **Cost efficiency:** GPU compute for LLM serving is extremely expensive — utilization efficiency has direct, major cost impact

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Concurrent requests (per GPU cluster) | Hundreds to low thousands, depending on model size |
| Model size | Billions of parameters, tens of GB of GPU memory just to load |
| Time-to-first-token target | Hundreds of ms |
| Tokens/sec generation rate (per request) | Tens of tokens/sec typical |
| KV-cache memory per request | Grows linearly with context length — can be substantial |

---

## 2. The Core Problem — Why LLM Serving Is Fundamentally Different

```mermaid
flowchart TB
    A["Traditional ML model<br/>inference (e.g., the ML Model<br/>Serving design's classification/<br/>ranking models): single<br/>forward pass, fixed compute<br/>cost, sub-millisecond to<br/>low-millisecond latency"] --> A1["Batching multiple requests<br/>together is straightforward —<br/>similar input shapes,<br/>predictable compute"]

    B["LLM inference: AUTOREGRESSIVE<br/>generation — producing output<br/>ONE TOKEN AT A TIME, where<br/>EACH new token requires<br/>another full forward pass<br/>through the model, conditioned<br/>on ALL previously generated<br/>tokens"] --> B1["A single request might<br/>generate 500 tokens — that's<br/>500 SEQUENTIAL forward passes,<br/>not one — fundamentally<br/>different computational<br/>pattern requiring specialized<br/>serving infrastructure"]

    C["Additionally: to avoid<br/>RECOMPUTING the entire<br/>attention context from<br/>scratch at every single token,<br/>LLMs maintain a 'KV-cache' —<br/>a growing block of GPU memory<br/>storing intermediate<br/>computation results PER<br/>REQUEST, PER TOKEN generated<br/>so far"] --> D["This KV-cache is what makes<br/>GPU MEMORY (not just compute)<br/>the dominant scarce resource<br/>and scaling bottleneck for<br/>LLM serving specifically"]
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    Client["Client Requests<br/>(streaming text generation)"]

    subgraph Serving["Inference Serving Layer"]
        Router["Request Router<br/>(model + GPU assignment)"]
        Scheduler["Continuous Batching<br/>Scheduler"]
        KVCacheMgr["KV-Cache Manager"]
    end

    subgraph GPUCluster["Multi-GPU Cluster"]
        GPU1["GPU 1<br/>(model shard / replica)"]
        GPU2["GPU 2<br/>(model shard / replica)"]
        GPUN["GPU N..."]
    end

    subgraph ModelMgmt["Model Management"]
        ModelStore[("Model Weight Storage")]
        ModelLoader["Model Loader"]
    end

    Client --> Router
    Router --> Scheduler
    Scheduler --> KVCacheMgr
    Scheduler --> GPU1
    Scheduler --> GPU2
    Scheduler --> GPUN

    ModelStore --> ModelLoader
    ModelLoader --> GPU1
    ModelLoader --> GPU2
    ModelLoader --> GPUN

    GPU1 -.->|"streamed tokens"| Client
    GPU2 -.->|"streamed tokens"| Client
```

**Key idea:** The Continuous Batching Scheduler and KV-Cache Manager are the two components that don't exist in this form in traditional ML serving — they exist specifically to solve LLM serving's unique challenges: efficiently batching requests that are AT DIFFERENT STAGES of generation (not uniform, single-shot requests), and carefully managing the scarce GPU memory consumed by each request's growing context cache.

---

## 4. Continuous (In-Flight) Batching — The Core Serving Innovation

```mermaid
flowchart TB
    A["Naive static batching:<br/>wait for a fixed batch of<br/>requests, process ALL of<br/>them until the SLOWEST one<br/>(longest output) finishes,<br/>THEN start the next batch"] --> A1["Massively inefficient — a<br/>request that finishes after<br/>10 tokens sits IDLE, wasting<br/>GPU capacity, while waiting<br/>for a batch-mate generating<br/>500 tokens to finish"]

    B["Continuous batching:<br/>as SOON as any request in<br/>the current batch finishes,<br/>IMMEDIATELY slot in a NEW<br/>waiting request to fill that<br/>now-empty batch slot —<br/>the batch composition changes<br/>continuously, token by token"] --> B1["Dramatically improves GPU<br/>utilization — no request<br/>ever blocks capacity waiting<br/>for an unrelated request's<br/>unrelated completion"]
```

```mermaid
sequenceDiagram
    participant R1 as Request 1<br/>(short, 10 tokens)
    participant R2 as Request 2<br/>(long, 500 tokens)
    participant R3 as Request 3<br/>(waiting in queue)
    participant Scheduler as Continuous Batching<br/>Scheduler
    participant GPU as GPU

    Scheduler->>GPU: Batch: [R1, R2]<br/>generate next token for each

    loop Token-by-token generation
        GPU->>GPU: Generate token for R1
        GPU->>GPU: Generate token for R2
    end

    Note over R1: R1 completes after<br/>10 tokens (hits stop token)

    Scheduler->>Scheduler: Detect R1 slot now free
    Scheduler->>GPU: IMMEDIATELY replace R1<br/>with waiting R3 in the batch

    Note over GPU: Batch is now [R3, R2] —<br/>NO idle GPU capacity,<br/>R2 continues uninterrupted
```

---

## 5. Data Model

```mermaid
erDiagram
    INFERENCE_REQUEST {
        string request_id PK
        string model_id
        string prompt
        int max_tokens
        string status "queued/generating/completed"
        int tokens_generated_so_far
    }
    KV_CACHE_BLOCK {
        string block_id PK
        string request_id FK
        int gpu_memory_offset
        int sequence_position_range
    }
    GPU_ALLOCATION {
        string gpu_id PK
        int total_memory_gb
        int used_memory_gb
        list active_request_ids
    }
```

---

## 6. KV-Cache Memory Management

```mermaid
flowchart TB
    A["Each request being generated<br/>needs GPU memory to store its<br/>growing KV-cache (attention<br/>context for every token<br/>generated so far)"] --> B["Problem: naive approach<br/>PRE-ALLOCATES the MAXIMUM<br/>possible context length worth<br/>of memory for every request<br/>upfront — massively wasteful<br/>for requests that end up<br/>being short"]

    B --> C["Solution: PagedAttention-style<br/>memory management — allocate<br/>KV-cache in small, FIXED-SIZE<br/>BLOCKS (like OS virtual memory<br/>pages) ON DEMAND as generation<br/>progresses, not upfront"]

    C --> D["Benefits: dramatically reduces<br/>memory WASTE from<br/>over-allocation, and enables<br/>MORE concurrent requests to<br/>fit in the same GPU memory<br/>budget — directly translating<br/>to higher serving throughput"]
```

```mermaid
sequenceDiagram
    participant Request as New Request
    participant KVMgr as KV-Cache Manager
    participant GPUMem as GPU Memory Pool

    Request->>KVMgr: Begin generation<br/>(prompt processed)
    KVMgr->>GPUMem: Allocate ONE small block<br/>(not the max possible size)

    loop Each new token generated
        KVMgr->>KVMgr: Check: does current block<br/>have room for this token's<br/>KV entries?
        alt Room available
            KVMgr->>KVMgr: Append to current block
        else Block full
            KVMgr->>GPUMem: Allocate ANOTHER new block<br/>on demand
        end
    end

    Note over Request: Request completes
    KVMgr->>GPUMem: IMMEDIATELY free all blocks<br/>used by this request —<br/>available for the NEXT<br/>incoming request right away
```

**Why this block-based approach is analogous to OS virtual memory paging:** Just as an operating system doesn't reserve a process's ENTIRE potential memory footprint upfront, but allocates physical memory pages on demand as the process actually uses them, this system allocates KV-cache memory incrementally as generation actually proceeds — avoiding the massive waste of reserving worst-case memory for every request regardless of how much it actually ends up needing.

---

## 7. Multi-GPU Routing Strategies

```mermaid
flowchart TB
    A["Model too large for a<br/>single GPU's memory, OR<br/>need to serve high request<br/>volume across many GPUs"] --> B{"Multi-GPU Strategy"}

    B --> C["Data Parallelism<br/>(replica-based)"]
    C --> C1["Full model copy on EACH<br/>GPU — different REQUESTS<br/>routed to different GPU<br/>replicas independently"]
    C --> C2["Simple, scales throughput<br/>linearly with GPU count —<br/>but each GPU must have<br/>enough memory for a FULL<br/>model copy"]

    B --> D["Tensor Parallelism<br/>(model-split)"]
    D --> D1["The model ITSELF is split<br/>across GPUs — a SINGLE<br/>request's forward pass<br/>requires ALL participating<br/>GPUs to cooperate"]
    D --> D2["Necessary when the model<br/>is too large to fit on ANY<br/>single GPU — but introduces<br/>significant inter-GPU<br/>communication overhead per<br/>token generated"]

    E["Production systems often<br/>use a HYBRID: tensor<br/>parallelism to fit a large<br/>model across a small group<br/>of GPUs, then data<br/>parallelism REPLICATING that<br/>group multiple times for<br/>overall throughput"] -.-> C2
```

---

## 8. Request Routing & Load Balancing — Detailed Sequence

```mermaid
sequenceDiagram
    participant Client as Client
    participant Router as Request Router
    participant GPUGroup1 as GPU Group 1<br/>(model replica A)
    participant GPUGroup2 as GPU Group 2<br/>(model replica B)

    Client->>Router: New generation request

    Router->>Router: Check current load/queue<br/>depth across GPU groups

    alt GPU Group 1 has capacity
        Router->>GPUGroup1: Route request
    else GPU Group 1 saturated, Group 2 has capacity
        Router->>GPUGroup2: Route request
    end

    Note over Router: Load-based routing, not<br/>simple round-robin — since<br/>request COMPLETION TIME<br/>varies wildly (short vs long<br/>generations), naive<br/>round-robin can create<br/>uneven load distribution
```

---

## 9. Streaming Response Delivery

```mermaid
sequenceDiagram
    participant Client as Client
    participant GPU as GPU (generating)
    participant Router as Request Router

    Client->>Router: Request (streaming=true)
    Router->>GPU: Forward request

    loop Token-by-token generation
        GPU->>GPU: Generate next token
        GPU-->>Router: Stream this token immediately
        Router-->>Client: Forward token to client<br/>(Server-Sent Events or<br/>WebSocket)
    end

    Note over Client: User sees text appearing<br/>progressively, word by word —<br/>rather than waiting for the<br/>ENTIRE response to complete<br/>before seeing ANYTHING
```

**Why streaming matters enormously for perceived latency:** Given that full generation of a long response can take several seconds, waiting for complete generation before showing anything would feel sluggish — streaming tokens as they're generated lets the user start reading almost immediately (time-to-first-token, often under a second), dramatically improving perceived responsiveness even though the TOTAL generation time is unchanged.

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((LLM Inference Serving HLD))
    Request Router
      Load-aware GPU group assignment
      Streaming response forwarding
    Continuous Batching Scheduler
      Dynamic batch composition
      Eliminates idle GPU capacity
    KV-Cache Manager
      Block-based, on-demand allocation
      PagedAttention-style memory efficiency
    GPU Cluster
      Data and/or tensor parallelism
      Hosts model weights and active generations
    Model Loader
      Loads weights onto GPU memory
      Supports multiple model versions
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Batching strategy | Continuous (in-flight) batching | Eliminates GPU idle time caused by waiting for the slowest request in a static batch — directly addresses LLM serving's uniquely variable per-request completion times |
| KV-cache allocation | Block-based, on-demand (PagedAttention-style) | Avoids massive memory waste from pre-allocating worst-case context length for every request, directly increasing achievable concurrency |
| Multi-GPU strategy | Hybrid tensor + data parallelism | Tensor parallelism handles models too large for one GPU; data parallelism replicates for throughput — neither alone is sufficient at scale |
| Response delivery | Token-by-token streaming | Dramatically improves perceived latency (time-to-first-token) despite total generation time remaining unchanged |
| Load balancing | Load-aware routing, not round-robin | Request completion times vary enormously (short vs long generations), making naive round-robin routing produce uneven actual load |

---

## 12. Bottlenecks & Scaling Considerations

- **GPU memory as the fundamental ceiling** — unlike most serving systems where compute is the primary constraint, LLM serving is frequently memory-bound; the number of concurrent requests a GPU can serve is directly limited by how much KV-cache memory is available, making memory management efficiency (Section 6) the single highest-leverage optimization area.
- **Long-context requests disproportionately consume resources** — a request with a very long input prompt or requesting a very long generation consumes proportionally more KV-cache memory throughout its lifetime, potentially crowding out capacity for many shorter concurrent requests — may need separate handling/quotas for exceptionally long-context requests.
- **Tensor parallelism communication overhead** — splitting a model across GPUs requires significant inter-GPU communication for EVERY token generated (not just periodically, unlike the gradient synchronization in the ML Training Pipeline design which happens once per training step) — this makes high-bandwidth GPU interconnects (NVLink) essential infrastructure, not a nice-to-have, for tensor-parallel serving.
- **Cold-start model loading time** — loading a large model's weights onto GPU memory can take significant time; auto-scaling GPU capacity up in response to demand spikes faces this same warm-up latency challenge covered in the general ML Model Serving design, often more pronounced given LLM model sizes.
- **Fairness and starvation** — under high load, the scheduler must ensure long-running generations don't perpetually starve newly-arriving requests of batch slots (or vice versa) — this requires deliberate fairness policies within the continuous batching scheduler, not just greedy capacity-filling.
- **Cost optimization through request batching economics** — because GPU cost is so significant, maximizing genuine throughput (tokens generated per GPU-second across ALL concurrent requests) rather than optimizing any single request's latency in isolation is often the primary cost-driving metric this entire architecture is designed to optimize.
- **Speculative decoding and other emerging optimizations** — beyond the core architecture described here, advanced techniques like speculative decoding (using a smaller, faster model to draft multiple tokens that the large model then verifies in parallel) represent an active, rapidly evolving area for further throughput improvements — worth acknowledging as the frontier of this space continues advancing beyond the foundational architecture covered in this design.
