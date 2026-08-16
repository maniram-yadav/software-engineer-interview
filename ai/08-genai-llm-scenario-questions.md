# 30 More Scenario-Based AI / GenAI / LLM Interview Questions

Same pattern as the RAG questions: real production failure scenarios, not textbook definitions. Organized by theme. Each answer: **Why it happens → How to diagnose → How to fix**.

---

## A. Prompt Engineering & Instruction Following

### Q1. Your prompt works perfectly in testing but breaks when users add extra context or phrase things differently. What's going on?
**Why**: The prompt was likely overfit to a narrow set of test phrasings — brittle prompts rely on exact keyword patterns or assume a fixed input structure. **Diagnose**: run the prompt against a diverse adversarial set (rephrased, longer, shorter, multi-intent inputs). **Fix**: make instructions robust to variation — use few-shot examples covering diverse phrasing, add explicit structure/delimiters (e.g., XML tags) so the model reliably distinguishes instructions from user content regardless of what the user writes, and test prompt changes against a regression suite, not just the happy path.

### Q2. Adding more few-shot examples made the model perform *worse*. Why?
**Why**: Possible causes — (a) examples are inconsistent in format/style, confusing the model about the expected output pattern; (b) recency/majority bias — models over-weight the *last* example or the majority pattern among examples, causing miscalibration if examples aren't balanced; (c) context dilution — too many examples push the actual task instruction further from the query, worsening lost-in-the-middle effects; (d) example selection introduces spurious patterns the model latches onto instead of the true task logic. **Fix**: use fewer, carefully curated, format-consistent examples; balance classes/edge cases represented; place the core instruction close to the actual query, not just at the very top.

### Q3. The model ignores your system prompt when the user prompt says something conflicting. How do you enforce instruction priority?
**Why**: LLMs don't have a hard architectural guarantee that system prompts override user prompts — it's a learned tendency, not an absolute rule, and can be broken by adversarial or even accidental conflicting phrasing. **Fix**: use models/APIs with stronger system-prompt adherence (many providers have improved this via RLHF specifically for instruction hierarchy); add explicit repeated reinforcement of critical constraints; add output validation/guardrails as a second layer (don't rely on the prompt alone for anything security/policy-critical) — treat prompt instructions as a strong bias, not an enforceable contract.

---

## B. Embeddings & Vector Search

### Q4. Two semantically very different sentences are getting a high similarity score in your vector search. Why?
**Why**: Embedding models can conflate **topical similarity** with **semantic equivalence** — e.g., "The movie was great" and "The movie was terrible" can score highly similar because they share vocabulary/topic (movies, sentiment-bearing), even though meaning is opposite. This is a known embedding model limitation, not a bug in your pipeline. **Fix**: for tasks sensitive to this (sentiment-critical retrieval), don't rely on embeddings alone — add a re-ranking or classification layer that explicitly checks the distinction the raw embedding conflates; choose an embedding model evaluated well on your specific similarity notion (some are tuned more for semantic entailment vs. topical clustering).

### Q5. You changed your embedding model mid-project. What breaks, and what's your migration plan?
**Why it breaks**: Embeddings from different models are **not comparable** — you can't mix old and new embeddings in the same similarity search; distances become meaningless across model versions. **Migration plan**: re-embed the *entire* corpus with the new model (cannot incrementally migrate a mixed index); run both old and new indexes in parallel during a transition window to A/B validate the new model's retrieval quality before fully cutting over; version your embeddings (store which model version produced each vector) so you can detect and prevent silent mixing; plan for the re-embedding cost/time at scale (this can be a significant batch job for large corpora — factor into any embedding-model-upgrade decision).

### Q6. Your vector DB search is fast for single queries but throughput collapses under concurrent load. What would you check?
**Why**: ANN index structures (especially in-memory HNSW) can have significant per-query CPU cost; under concurrency, if the DB/service isn't horizontally scaled or the index isn't sharded, requests queue up. **Diagnose**: check CPU utilization on the vector DB nodes under load, check if you're running a single-node deployment, check whether the client is using connection pooling or opening a new connection per request. **Fix**: shard the index across multiple nodes, add read replicas for the vector DB (similar pattern to database read replicas), tune index parameters for the speed/accuracy trade-off appropriate at your target QPS, consider a managed/purpose-built vector DB designed for concurrent throughput if you're using a naive in-process library.

---

## C. Fine-Tuning vs Prompting vs RAG

### Q7. Stakeholders ask "why not just fine-tune instead of using RAG?" How do you answer?
**Framework**: 
- **RAG** is better when: knowledge changes frequently, you need traceability/citations, you need to scope answers strictly to a known corpus, or you don't have enough labeled examples to fine-tune effectively.
- **Fine-tuning** is better when: you need to teach a *behavior/style/format* (not facts) — e.g., a specific output structure, tone, or domain-specific reasoning pattern — or you need lower latency (no retrieval hop) and can bake static knowledge directly into weights.
- **They're complementary, not exclusive**: many production systems fine-tune a model to be better at *using* retrieved context (following citation format, refusing when unsupported) while still using RAG for the actual facts — fine-tuning the behavior, RAG for the knowledge.
- **Cost/maintenance trade-off**: RAG's knowledge updates are cheap (just update the index); fine-tuning requires a retraining cycle for every knowledge update, which doesn't scale for frequently-changing information.

### Q8. You fine-tuned a model on your support tickets, and now it performs worse on general questions it used to answer fine. What happened?
**Why**: This is **catastrophic forgetting** — fine-tuning on a narrow dataset can degrade the model's broader pretrained capabilities if the fine-tuning data distribution is too narrow or the learning rate/number of epochs is too aggressive. **Fix**: use parameter-efficient fine-tuning (LoRA/QLoRA) which modifies a small subset of parameters and tends to preserve base capabilities better than full fine-tuning; mix in general-purpose instruction data alongside your domain data during fine-tuning (prevent distribution collapse); use a lower learning rate and fewer epochs, validating against both domain-specific *and* general-capability eval sets after each training run, not just the domain metric.

### Q9. Your fine-tuning dataset has only 200 examples. Is that enough, and what are the risks?
**Answer**: 200 examples can work for **narrow behavioral fine-tuning** (e.g., teaching a specific output format or tone) especially with LoRA-style efficient fine-tuning, but is generally too small to teach new *knowledge* reliably or to fine-tune a model's reasoning on a complex task — high risk of overfitting to the exact phrasing/examples seen, poor generalization. **Mitigation**: augment with synthetic data generation (careful to maintain quality/diversity), start with few-shot prompting instead of fine-tuning if the dataset is this small (often gets you 80% of the benefit with none of the overfitting risk or infra cost), and if fine-tuning anyway, hold out a real validation set to catch overfitting early rather than trusting training loss alone.

---

## D. Hallucination & Factuality (Beyond RAG-specific)

### Q10. The model hallucinates specific numbers/statistics even when asked to only use provided data. How do you reduce this?
**Why**: LLMs are next-token predictors optimized for plausible-sounding text, not verified fact retrieval — numbers are especially prone to hallucination because a "plausible-looking number" is easy to generate and hard for the model to self-verify. **Fix**: for numeric/factual answers, prefer **extractive** rather than generative approaches where possible (have the model point to/quote the exact source number rather than restating it from "memory" of the context); add a post-generation verification step that checks any numbers in the output actually appear in the source context (simple string/regex match as a cheap sanity check); lower temperature to 0 for factual tasks; explicitly instruct the model to quote verbatim rather than paraphrase numeric claims.

### Q11. How do you build an automated hallucination detection system for production LLM outputs?
**Approach layers**:
1. **Entailment/NLI-based checking**: use a smaller NLI model to check if each claim in the output is entailed by the source context — flags unsupported claims.
2. **LLM-as-judge**: prompt a separate (often stronger) model to evaluate "is this response fully supported by the given context? List any unsupported claims" — more flexible than NLI but costs an extra LLM call and inherits the judge model's own imperfections.
3. **Self-consistency checking**: generate the same answer multiple times (with some sampling temperature) and check for consistency — high variance across samples on factual claims is a signal of low confidence/potential hallucination.
4. **Source-grounding score**: for RAG systems specifically, compute overlap/entailment between generated claims and retrieved chunks as a quantitative faithfulness score.
**Trade-off**: more thorough detection = more latency/cost added per response; in practice, apply the heaviest checking to a sampled subset for monitoring, and a cheaper/faster check inline for real-time gating.

---

## E. Model Evaluation

### Q12. Your model scores well on standard benchmarks (MMLU, etc.) but performs poorly on your specific business use case. Why, and what do you do?
**Why**: Public benchmarks measure general capability on tasks that may not resemble your actual use case's distribution, format, or domain vocabulary at all — a model can be an excellent general reasoner and a poor fit for your specific structured extraction task or your specific domain jargon. **Fix**: build a **task-specific eval set** built from real examples of your actual use case (not proxy benchmarks) — this is non-negotiable for production decisions; use the public benchmarks only as a rough initial filter for candidate models, never as the final selection criterion.

### Q13. How would you design an evaluation framework for an LLM-powered feature before it ships to production?
**Framework**:
1. **Define success criteria concretely** — what does "good" mean for this specific feature (accuracy, faithfulness, tone, latency, safety) — often multi-dimensional, not a single score.
2. **Build a golden eval set** — real or realistic examples with known-correct/acceptable outputs, covering both common cases and known edge cases.
3. **Automated metrics** — task-specific (exact match, ROUGE/BLEU for summarization, faithfulness score for RAG, etc.) run on every model/prompt change as a fast regression check.
4. **LLM-as-judge for subjective quality** — for aspects hard to measure with exact-match metrics (helpfulness, tone, coherence), use a calibrated LLM judge with clear rubrics, periodically validated against human judgment for agreement.
5. **Human evaluation** — for a sample, especially before major releases, since automated metrics/judges have known blind spots and biases.
6. **Safety/red-teaming pass** — adversarial testing for harmful outputs, prompt injection, jailbreaks, before any production exposure.
7. **Shadow/canary testing** in production — run the new version against real (or replayed) traffic without exposing it to users, comparing outputs/metrics against the current production version.

### Q14. Your LLM-as-judge evaluation disagrees with human reviewers 30% of the time. Is that acceptable, and what would you do?
**Answer**: 30% disagreement is generally too high to trust the judge as a sole gatekeeper — first check *where* the disagreement concentrates (random noise across all categories vs. a systematic bias, e.g., the judge consistently favoring longer/more verbose answers, or consistently missing a specific type of error). **Fix**: refine the judge's rubric/prompt with clearer, more specific criteria and few-shot examples of correct judgments; consider using a different/stronger model as judge; use the judge for high-volume screening but keep human review in the loop for borderline/high-stakes cases; track judge-human agreement as an ongoing metric, not a one-time check, since model updates can shift this.

---

## F. Agents & Tool Use

### Q15. Your LLM agent gets stuck in a loop, repeatedly calling the same tool without making progress. How do you fix this?
**Why**: The agent's reasoning loop lacks a mechanism to recognize "this approach isn't working" — often because tool results aren't being incorporated into the next decision clearly, or the agent's planning doesn't track prior attempts. **Fix**: add explicit **loop detection** (track recent tool calls + arguments; if a repeat is detected, force a different strategy or escalate to human/fallback); add a **max iteration/step limit** with graceful termination ("I was unable to complete this after N attempts") rather than infinite looping; improve the prompt to explicitly require the agent to summarize what it has learned/tried before deciding the next action; consider a more structured planning approach (e.g., explicit plan-then-execute rather than pure reactive loop) for complex multi-step tasks.

### Q16. An agent with access to tools (database queries, API calls, code execution) is a security risk. How do you design safe tool use?
**Layered defenses**:
1. **Least-privilege tool scoping** — give the agent only the specific, narrow tools/permissions it needs (e.g., a read-only DB query tool, not raw SQL execution) rather than broad access "just in case."
2. **Input validation/sandboxing** — validate and sanitize any parameters the agent generates before executing them (e.g., parameterized queries, not raw string interpolation into SQL — same discipline as traditional injection defense, since prompt-injected or hallucinated tool inputs are a real risk).
3. **Sandboxed execution environments** — code execution tools run in isolated, resource-limited containers with no access to sensitive systems/secrets.
4. **Human-in-the-loop for high-stakes actions** — irreversible or high-risk actions (sending an email, making a purchase, deleting data) require explicit human confirmation before execution, not full autonomy.
5. **Audit logging** — log every tool call the agent makes (inputs, outputs) for post-hoc review and anomaly detection.
6. **Rate limiting/circuit breaking on tool calls** — prevent a misbehaving agent loop from hammering downstream systems (ties directly to the reliability patterns — bulkheads, rate limiting — covered in general system design).

### Q17. How do you handle a tool call that fails (API timeout, invalid response) mid-agent-execution?
**Fix**: the agent needs the failure fed back into its reasoning loop as an observation (not a silent crash) — "Tool X failed with error Y" should be part of the next prompt context so the agent can decide to retry, use a different tool, or inform the user of the limitation; implement retries with backoff at the tool-execution layer for transient failures (network blips) before surfacing to the agent; set a reasonable timeout per tool call so a hanging external API doesn't stall the entire agent loop indefinitely; have a defined fallback/graceful degradation path (same principle as the reliability topic) — e.g., if a live-data tool fails, fall back to cached/approximate data with a caveat to the user, rather than a hard failure.

### Q18. Multiple agents/sub-agents in a pipeline are producing inconsistent or conflicting outputs. How do you debug a multi-agent system?
**Diagnose**: multi-agent failures are notoriously hard to debug because errors compound across steps — you need **per-agent-step tracing/logging** (input, output, reasoning trace for each sub-agent in the pipeline) to isolate exactly which stage introduced the inconsistency, rather than only looking at the final output. **Common root causes**: (a) ambiguous task handoff between agents — sub-agent B doesn't have enough context about what sub-agent A actually did/decided; (b) agents operating on stale/inconsistent shared state; (c) no single source of truth for shared facts, so different agents "remember" or infer different versions of the same information. **Fix**: define clear, structured interfaces/contracts between agents (not free-form text handoff); maintain a shared, explicit state/context object all agents read from rather than each agent re-deriving facts independently; add a final consistency-check/arbitration step before returning results to the user.

---

## G. Context Window & Long Context

### Q19. You increased the context window (more retrieved chunks / longer conversation history) expecting better answers, but quality got worse. Why?
**Why**: This connects to lost-in-the-middle and attention dilution (see RAG Q1/Q4) — more context isn't strictly better; irrelevant or redundant content in a longer context can actively distract the model from the truly relevant parts, and very long contexts also increase latency/cost with often diminishing or negative returns on quality. **Fix**: prioritize relevance over volume — better ranking/filtering of what goes into context beats simply including more; test empirically where your specific model's quality peaks vs. context length (there's often a real quality cliff well before the model's stated max context length); summarize/compress long conversation history rather than including it verbatim once it exceeds a reasonable length.

### Q20. How do you manage conversation memory in a multi-turn chat application without hitting context limits?
**Strategies**:
1. **Sliding window** — keep only the last N turns verbatim, drop older ones — simple but loses long-term context.
2. **Summarization** — periodically compress older turns into a running summary, keeping recent turns verbatim + summary of everything before — balances context retention with token budget.
3. **Retrieval-based memory** — store all past turns in a vector store; retrieve only relevant past turns for the current query rather than keeping the full linear history — scales to very long conversations, at the cost of retrieval accuracy/latency.
4. **Structured memory extraction** — extract and store key facts/entities from the conversation (e.g., "user's name is X, prefers Y") in a structured store rather than raw transcript, injected into future prompts as needed — most token-efficient, but requires reliable extraction.
**Trade-off**: verbatim history is most accurate but least scalable; summarization/extraction is scalable but risks losing nuance or introducing summarization errors that compound over a long conversation.

---

## H. Cost, Latency & Infrastructure

### Q21. Your GenAI feature's API costs are growing faster than usage. What would you investigate?
**Diagnose**: check whether cost growth is from (a) more requests (expected, proportional to usage), (b) longer prompts/context per request (e.g., unbounded conversation history, or retrieval returning increasingly large context — ties to Q19), (c) more output tokens per response (verbose model outputs, or retries appending rather than replacing), (d) inefficient retry logic (retrying full expensive calls on failure instead of cheaper fallback), (e) using an unnecessarily large/expensive model for tasks a smaller model could handle. **Fix**: implement per-request token budgets/caps; route tasks by complexity to appropriately-sized models (a **model cascade** — try a cheap/fast model first, escalate to an expensive model only if needed); cache repeated/similar queries; monitor cost per request as a first-class metric, not just aggregate spend, so regressions are caught early (per-request cost creeping up is a leading indicator of the aggregate problem).

### Q22. How would you design a system to serve both a cheap/fast model and an expensive/accurate model, choosing the right one per request?
**Pattern — model routing / cascade**:
- Classify incoming requests by complexity/risk (a lightweight classifier, or heuristics like query length/ambiguity) and route simple/low-risk queries to a small, fast, cheap model.
- For complex or high-stakes queries, route to the larger model directly, or use a **cascade**: try the small model first, and only escalate to the large model if the small model's confidence is low or a verifier flags the response as insufficient.
- Trade-off: cascading adds latency for the escalated cases (paid the small model's cost/time before also paying the large model's) — a pure classifier-based router avoids this double cost but requires an accurate upfront classifier, which is its own ML problem to get right and maintain.

### Q23. Streaming responses improved perceived latency, but you're now seeing increased infrastructure costs/connection issues at scale. What would you check?
**Why**: Streaming requires holding open long-lived connections per active request (vs. a quick request-response cycle), which multiplies concurrent connection/resource usage on your serving infrastructure — load balancers, proxies, and server processes need to be configured for long-lived connections (timeouts, keep-alive settings), and naive infra defaults (aggressive proxy timeouts) can silently truncate streams mid-response. **Fix**: audit and tune timeout settings across every hop (client, load balancer, application server) for streaming-appropriate values; monitor concurrent open connections as a capacity metric distinct from raw request rate; consider server-sent events or WebSocket infrastructure explicitly designed for many concurrent long-lived connections rather than a naive HTTP request-per-connection model.

---

## I. Safety, Security & Guardrails

### Q24. Users are successfully jailbreaking your LLM feature to bypass content restrictions. How do you defend against this?
**Layered defense (no single layer is sufficient)**:
1. **Input-side guardrails** — a classifier/filter that screens incoming prompts for known jailbreak patterns/injection attempts before they even reach the main model.
2. **System prompt hardening** — clear, firmly-stated boundaries, though as noted in Q3, system prompts alone are not a hard guarantee.
3. **Output-side guardrails** — screen the model's response *before* returning it to the user, catching cases where the input-side filter was bypassed but the output is still policy-violating.
4. **Use a model/API with strong built-in safety training** — don't rely solely on your own prompt-level defenses; foundation model providers invest heavily in this and it's a meaningfully different (stronger) baseline than a naive open model with just a system prompt.
5. **Continuous red-teaming** — jailbreak techniques evolve constantly; this needs to be an ongoing process (internal red team + monitoring for new attack patterns in production logs), not a one-time hardening pass.
6. **Rate limiting and anomaly detection** — flag/throttle accounts showing patterns consistent with systematic jailbreak probing.

### Q25. How is prompt injection different from a jailbreak, and how do you defend against prompt injection specifically in a RAG/agent system?
**Distinction**: a **jailbreak** is the *end user* trying to manipulate the model into violating its own guidelines. **Prompt injection** is malicious instructions hidden inside *retrieved content or tool outputs* (a webpage, a document, an email the agent reads) that attempt to hijack the model's behavior — the attacker isn't the user at all, but a third party who controls content the system will ingest. **Defense**: treat all retrieved/tool-sourced content as **untrusted data, never as instructions** — this needs to be architecturally enforced (e.g., clear delimiters/tagging that mark retrieved content as data-to-analyze, explicit prompt instructions that the model should never follow directives found within retrieved content), not just hoped for; for agents specifically, apply the same tool-use safety principles as Q16 (least privilege, human confirmation for high-stakes actions) as a backstop, since prompt injection defenses at the model level are not currently fully reliable.

### Q26. Your LLM feature occasionally outputs PII (personal identifiable information) it shouldn't have access to or shouldn't reveal. How do you prevent this?
**Diagnose the source**: (a) the PII is present in your training/fine-tuning data and the model memorized/regurgitates it, (b) the PII is present in retrieved RAG context and the model is faithfully (correctly, from a grounding standpoint) surfacing it when it shouldn't be shown to this particular user, (c) the model is hallucinating plausible-looking PII (a different but related risk). **Fix**: PII scrubbing/redaction at the data ingestion layer (before it ever enters a training set or retrievable index) using a dedicated PII detection tool, not manual review at scale; access control at the retrieval layer — ensure retrieval respects the requesting user's actual permissions (don't retrieve documents the user isn't authorized to see, regardless of semantic relevance); output-side PII scanning as a final safety net before returning any response.

---

## J. Deployment, Monitoring & Production Operations

### Q27. You want to update your production prompt template. How do you roll this out safely?
**Approach — treat prompts like code**:
1. Version control every prompt template change (git, not ad-hoc editing in a config UI with no history).
2. Run the new prompt against the full regression eval set before deployment — a "prompt regression suite," directly analogous to a code test suite.
3. **A/B test or canary** the new prompt against a small percentage of live traffic, comparing quality/safety/latency metrics against the current version before full rollout (same canary-deployment principle used for any production software change).
4. Have a fast rollback mechanism — since prompt changes can have subtle, hard-to-predict effects on model behavior, be ready to revert quickly if production metrics degrade.
5. Monitor post-rollout for a defined period, not just at the moment of deployment, since some quality regressions only show up with real traffic diversity over time.

### Q28. How do you detect and respond to "model drift" when using a third-party LLM API (where you don't control retraining)?
**Why this happens**: providers periodically update/deprecate model versions; even a "same" model name can have silently updated weights/behavior; your carefully-tuned prompt can start behaving differently overnight through no code change of your own. **Detection**: run your regression eval suite on a scheduled basis (not just at deploy time) against the live API, so a silent provider-side change is caught by your own metrics dropping, rather than discovered via user complaints; monitor output characteristics over time (average response length, refusal rate, format-compliance rate) for unexpected shifts. **Response**: pin to specific model version strings where the provider offers this option (rather than a "latest" alias) to get advance control over when you adopt changes; maintain the ability to quickly test and roll back to a previous pinned version if a provider update degrades your metrics.

### Q29. How would you set up alerting for an LLM-powered production system — what metrics actually matter?
**Key metrics to track and alert on**:
- **Quality**: automated faithfulness/groundedness score trend (for RAG), refusal rate, user feedback (thumbs down rate) — sudden shifts indicate a regression (prompt change, model drift, data issue).
- **Latency**: p50/p95/p99 end-to-end and per-pipeline-stage (ties to the general Latency vs Throughput system design concept) — LLM latency is often much more variable than typical API latency, so tail latency monitoring matters even more here.
- **Cost**: cost per request trend, token usage per request (catches runaway context growth or verbose outputs before it becomes a large bill).
- **Retrieval health** (RAG-specific): average retrieval confidence score, empty-retrieval rate (queries where nothing relevant was found — a leading indicator of both hallucination risk and knowledge-base gaps).
- **Safety**: rate of outputs flagged by safety/guardrail filters — a spike can indicate either an attack pattern or a legitimate new use case your guardrails are miscategorizing (needs investigation either way).
- **Availability/errors**: standard error rate, timeout rate, upstream API failure rate.

### Q30. How do you A/B test two different RAG configurations (e.g., different chunk sizes, or different embedding models) in production, and what makes this harder than a typical A/B test?
**What makes it harder than a standard product A/B test**: 
- **Delayed/subjective outcome signal** — unlike a click-through-rate test, "was this answer good" isn't always immediately observable; you often need a mix of implicit signals (session behavior) and explicit feedback (thumbs up/down), both noisier than a hard conversion event.
- **Interaction effects** — RAG pipeline changes (chunking, embedding model, re-ranker) interact with each other; testing one variable at a time in isolation may not reflect how it performs combined with other pipeline components.
- **Non-stationary ground truth** — the "correct" answer for a given query can change over time as the underlying knowledge base updates, complicating longitudinal comparison.
**Approach**: define clear primary metrics upfront (e.g., faithfulness score + user feedback rate, not just one), ensure statistically sufficient sample size per arm (LLM-related quality signals tend to be noisier than typical product metrics, requiring larger samples for significance), run both automated eval-set-based comparison (fast, controlled, repeatable) *and* live A/B testing (slower, but captures real query distribution) rather than relying on either alone, and hold the rest of the pipeline constant while testing one meaningful change at a time where possible to keep results interpretable.

---

## Quick-reference: the meta-pattern across all these answers

Every strong scenario-based answer in AI/RAG/LLM interviews follows the same shape, regardless of the specific question:

1. **State the likely root cause(s)**, ranked by real-world frequency — don't just list every theoretical possibility with equal weight.
2. **Describe a concrete diagnostic step** — how you'd actually confirm which cause it is, not just guess.
3. **Give a specific fix**, not a vague direction ("improve the prompt" is weak; "add explicit citation-required instructions and reorder chunks to put the highest-confidence one first" is strong).
4. **Name the trade-off** the fix introduces (nothing is free — tighter refusal thresholds cost false-negatives, re-ranking costs latency, bigger models cost money).
5. **Mention how you'd prevent recurrence** — a regression eval set, a monitoring metric, a logging requirement — showing you think about the *system*, not just the one-off fix.
