# 50 GenAI Topics — Interview Theory Guide

A deep-dive reference covering theory, intuition, math where useful, and practical examples for each topic. Read top to bottom for a full picture, or jump to a topic before an interview.

---

## 1. LLM Fundamentals

A **Large Language Model (LLM)** is a neural network — almost always a **Transformer decoder** — trained on massive text corpora to predict the next token given previous tokens (autoregressive language modeling). The core training objective is:

```
maximize  P(token_t | token_1, token_2, ..., token_{t-1})
```

This simple objective, applied at the scale of trillions of tokens and billions/trillions of parameters, produces emergent capabilities: reasoning, translation, coding, summarization — none of which were explicitly labeled during training.

**Key stages of building an LLM:**
1. **Pre-training** — self-supervised next-token prediction on raw web text, books, code (e.g., GPT-4, Llama, Claude base models). This is where the bulk of "knowledge" is absorbed. Extremely compute-expensive (thousands of GPUs, months).
2. **Supervised Fine-Tuning (SFT)** — train on curated (prompt, ideal response) pairs so the model learns to follow instructions instead of just completing text.
3. **Alignment (RLHF/DPO)** — further tune the model to prefer helpful, harmless, honest responses (see topics 29, 30).

**Why "Large"?** Scale (parameters + data + compute) empirically produces better performance following **scaling laws** (Kaplan et al., Chinchilla) — loss decreases predictably as a power law with compute, until data or model size becomes the bottleneck. Chinchilla's key insight: most early LLMs were *undertrained* relative to their size — optimal training balances parameter count and token count (~20 tokens per parameter).

**Example:** GPT-3 (175B params) was trained on ~300B tokens; Chinchilla (70B params) trained on ~1.4T tokens *outperformed* GPT-3 despite being smaller — the lesson driving modern data-heavy training runs (Llama 3 trained a much smaller model on 15T+ tokens).

**Interview angle:** Be ready to explain *why* next-token prediction alone produces reasoning-like behavior (it forces the model to build internal world models/representations to compress and predict text well), and the distinction between base models vs. instruction-tuned/chat models.

---

## 2. Transformers

Introduced in *"Attention Is All You Need"* (Vaswani et al., 2017), the Transformer replaced RNNs/LSTMs as the dominant sequence architecture because it processes all tokens **in parallel** (not sequentially) and uses **self-attention** to model relationships between any two tokens regardless of distance.

**Architecture overview:**
- **Encoder** — stack of layers that build contextual representations of the input (used in BERT, encoder-only models for classification/embeddings).
- **Decoder** — stack of layers that generate output tokens autoregressively, using masked self-attention so a token can't "see" future tokens (used in GPT, Llama — decoder-only, which is what modern LLMs use).
- **Encoder-Decoder** — used for seq2seq tasks like translation (T5, original Transformer, BART).

**Each Transformer block contains:**
1. Multi-Head Self-Attention (topic 4)
2. Add & LayerNorm (residual connection + normalization for stable gradients)
3. Feed-Forward Network (two linear layers with a nonlinearity, applied position-wise)
4. Add & LayerNorm again

```
x = x + MultiHeadAttention(LayerNorm(x))   # pre-norm variant, common in modern LLMs
x = x + FeedForward(LayerNorm(x))
```

**Why Transformers beat RNNs:**
- **Parallelism** — RNNs process token-by-token sequentially (can't parallelize across time), Transformers process the whole sequence at once during training.
- **Long-range dependencies** — RNNs suffer vanishing gradients over long sequences; self-attention gives direct O(1)-hop connections between any two tokens.
- **Scalability** — Transformers scale better with more data/compute, which is why they underpin every modern LLM.

**Trade-off:** self-attention is O(n²) in sequence length (every token attends to every other token), which is why long-context models need optimizations (FlashAttention, sliding window attention, sparse attention).

**Example:** In "The cat sat on the mat because it was tired," self-attention lets the model directly connect "it" to "cat" (co-reference resolution) in a single step, something an RNN would have to propagate through many time steps to learn.

---

## 3. Attention Mechanism

Attention lets a model decide, for each token, *how much to focus on every other token* when building its representation. It replaced fixed-context or purely recurrent context aggregation.

**Scaled Dot-Product Attention** — the core formula:

```
Attention(Q, K, V) = softmax( Q·Kᵀ / √d_k ) · V
```

- **Q (Query)** — "what am I looking for?" (vector per token)
- **K (Key)** — "what do I contain?" (vector per token)
- **V (Value)** — "what information do I pass along if selected?"
- **Q·Kᵀ** — dot product measures similarity between a query and every key → raw attention scores.
- **/√d_k** — scaling factor to prevent large dot products (as dimension grows) from pushing softmax into regions with vanishing gradients.
- **softmax** — converts scores into a probability distribution (weights summing to 1).
- **· V** — weighted sum of value vectors using those probabilities = the output representation.

**Intuition example:** For the sentence "The animal didn't cross the street because it was too tired," when computing the representation for "it," the query vector for "it" will have a high dot-product with the key vector for "animal," so the output for "it" becomes mostly a blend dominated by "animal"'s value vector — this is how the model resolves coreference without explicit rules.

**Self-attention vs. cross-attention:**
- **Self-attention** — Q, K, V all come from the same sequence (used within encoder or decoder to relate tokens to each other).
- **Cross-attention** — Q comes from one sequence (e.g., decoder), K/V come from another (e.g., encoder output) — used in encoder-decoder models and in some multimodal architectures (image K/V, text Q).

**Causal (masked) attention:** In decoder-only LLMs, a mask sets attention scores to −∞ for future positions before softmax, so token *t* can only attend to tokens ≤ *t* — this enforces autoregressive generation.

---

## 4. Multi-Head Attention

Instead of computing attention once, Multi-Head Attention runs **h parallel attention "heads,"** each with its own learned Q/K/V projection matrices, then concatenates the results and projects back down.

```
head_i = Attention(Q·W_Q_i, K·W_K_i, V·W_V_i)
MultiHead(Q,K,V) = Concat(head_1, ..., head_h) · W_O
```

**Why multiple heads?** A single attention head learns one type of relationship (e.g., "what's the subject of this verb"). Different heads specialize in different relationships — one head might track syntactic dependencies, another might track long-range coreference, another might track positional/local patterns. This is empirically observed via attention visualization in models like BERT.

**Example (concrete numbers):** GPT-3's largest variant has d_model = 12288 and 96 heads, so each head operates in a 128-dim subspace (12288/96). Each head gets its own smaller Q/K/V, letting the model attend to information from different representation subspaces simultaneously, then the concatenation + output projection recombines everything into one unified representation.

**Interview trap to know:** Multi-head attention doesn't increase total compute much vs. single-head with the same total dimension — the total Q/K/V dimension is *split* across heads, not multiplied. The benefit is representational diversity, not more parameters (though W_O and the per-head weights do add some).

**Related optimizations to mention:**
- **Multi-Query Attention (MQA)** — all heads share the same K/V projections (only Q differs), drastically reducing KV-cache memory at inference, slight quality cost.
- **Grouped-Query Attention (GQA)** — middle ground: groups of heads share K/V (used in Llama 2/3, Mistral) — most of MQA's memory savings with less quality loss.

---

## 5. Tokenization

Tokenization converts raw text into a sequence of discrete integer IDs the model can embed and process. LLMs don't see words or characters directly — they see **tokens**, which are subword units.

**Why subword tokenization (not word-level or char-level)?**
- **Word-level** — vocabulary explodes (every inflection, typo, rare word needs its own entry), and out-of-vocabulary words become `<UNK>`, losing information.
- **Character-level** — vocabulary is tiny but sequences become very long (more tokens = more compute, and harder to learn long-range structure).
- **Subword-level** — sweet spot: common words are a single token ("the," "cat"), rare/complex words are split into meaningful pieces ("tokenization" → "token" + "ization"), keeping vocab size manageable (~30K–100K+) while handling any input, including typos and new words, without `<UNK>`.

**Common algorithms:**
- **BPE (Byte-Pair Encoding)** — start with characters/bytes, iteratively merge the most frequent adjacent pair into a new token, repeat until vocab size target is reached. Used by GPT models (GPT-2/3/4 use a byte-level BPE called `tiktoken`).
- **WordPiece** — similar to BPE but merges pairs that maximize likelihood of the training data rather than raw frequency. Used by BERT.
- **SentencePiece / Unigram** — treats tokenization as a probabilistic model, doesn't require pre-splitting by whitespace, works well for languages without clear word boundaries (Japanese, Chinese). Used by Llama, T5.

**Example:** `"unbelievable"` might tokenize as `["un", "believ", "able"]` — three tokens capturing morphology, versus one whole-word token in a huge word-level vocab.

**Practical interview points:**
- Token count ≠ word count. Roughly 1 token ≈ 0.75 words in English; this matters for context-window budgeting and API cost (billed per token).
- Numbers, code, and non-English text often tokenize *less* efficiently (more tokens per character) — a real cost/latency consideration for multilingual or code-heavy applications.
- Tokenization mismatches cause quirky LLM failures — e.g., historically GPT models were bad at character-level tasks (like counting letters in a word, or reversing a string) precisely because they operate on subword tokens, not characters.

---

## 6. Embeddings

An embedding is a dense vector representation of a token, word, sentence, or document in continuous space, such that **semantic similarity corresponds to geometric proximity** (e.g., cosine similarity or Euclidean distance).

**Two related but distinct uses in GenAI:**
1. **Token embeddings inside the model** — the first layer of a Transformer maps each token ID to a learned vector (e.g., 4096-dim for a 7B model). These are trained jointly with the rest of the model via backprop and encode learned semantics/syntax as a side effect of the language-modeling objective.
2. **Standalone embedding models** — models specifically trained (often via contrastive learning) to turn a whole sentence/paragraph/document into a single fixed-size vector for retrieval, clustering, or semantic search (e.g., OpenAI's `text-embedding-3`, Cohere embed, `sentence-transformers`, Voyage AI).

**Classic example (word2vec-era intuition, still illustrative):**
```
vector("king") - vector("man") + vector("woman") ≈ vector("queen")
```
This shows embeddings capture relational/semantic structure as directions in vector space, not just similarity.

**How sentence/document embedding models are trained:** contrastive learning — pull embeddings of semantically similar pairs (e.g., a question and its correct answer, or two paraphrases) closer together, push dissimilar pairs apart, typically using a loss like InfoNCE:

```
L = -log( exp(sim(a,p)/τ) / Σ_i exp(sim(a,n_i)/τ) )
```
where `a` = anchor, `p` = positive pair, `n_i` = negatives, `τ` = temperature.

**Similarity metrics:**
- **Cosine similarity** — most common; angle between vectors, ignores magnitude. `cos(θ) = (A·B) / (‖A‖‖B‖)`
- **Dot product** — used when vectors are normalized or magnitude carries meaning (e.g., some retrieval-optimized embeddings).
- **Euclidean (L2) distance** — less common for text, more common in image embeddings.

**Practical example:** Embed "How do I reset my password?" and a knowledge-base article titled "Password recovery steps" — even though they share almost no exact words, a good embedding model places them close together, enabling semantic search (topic 13) instead of brittle keyword matching.

**Interview point:** Embedding dimensionality is a trade-off — higher dims (e.g., 3072) capture more nuance but cost more storage/compute in a vector DB; many providers now support **Matryoshka embeddings**, which let you truncate a high-dim embedding to a smaller size with graceful quality degradation instead of retraining.

---

## 7. Positional Encoding

Self-attention is **permutation-invariant** — without extra information, `Attention(Q,K,V)` gives the same result regardless of token order ("dog bites man" and "man bites dog" would look identical to raw self-attention). Positional encoding injects order information.

**Sinusoidal positional encoding (original Transformer paper):** a fixed (not learned) function added to token embeddings:

```
PE(pos, 2i)   = sin(pos / 10000^(2i/d_model))
PE(pos, 2i+1) = cos(pos / 10000^(2i/d_model))
```

Different frequencies for different embedding dimensions let the model infer relative positions (since sin/cos of a sum can be expressed as a combination of sin/cos of the parts, enabling the model to learn to attend by relative offset).

**Learned absolute positional embeddings:** a trainable embedding table indexed by position (like BERT/GPT-2 originally used) — simple, but doesn't generalize well beyond the max sequence length seen in training.

**Rotary Positional Embeddings (RoPE)** — the dominant approach in modern LLMs (Llama, Mistral, GPT-NeoX-style models). Instead of *adding* a positional vector, RoPE **rotates** the Q and K vectors by an angle proportional to their position before computing the dot product. This makes the attention score between two tokens a function of their **relative** distance, not absolute position:

```
q_m = R(mθ)·q,  k_n = R(nθ)·k
q_m · k_n  depends only on (m - n)
```

**Why RoPE won out:** it encodes relative position naturally (which matters more linguistically than absolute position), generalizes better to longer sequences than seen during training, and is efficient to compute.

**ALiBi (Attention with Linear Biases)** — another alternative: instead of modifying Q/K, it directly subtracts a distance-proportional penalty from attention scores, biasing attention toward nearby tokens. Enables strong length extrapolation and is simpler/cheaper than RoPE.

**Example / why it matters:** without positional encoding, an LLM couldn't distinguish "The bank of the river" from "The bank approved my loan" *based on word order* — the words themselves would be attended to identically. Positional encoding is what lets the model know "bank" at position 5 relates differently to context than "bank" at position 1.

---

## 8. Prompt Engineering

Prompt engineering is the practice of designing the input to an LLM to reliably elicit the desired output, without changing model weights. It's the cheapest, fastest lever for improving LLM behavior (compared to fine-tuning).

**Core techniques:**
- **Clear task framing** — explicit instructions, role/persona ("You are a senior Python code reviewer..."), explicit output format requirements.
- **Zero-shot** — just the instruction, no examples: `"Classify the sentiment of this review: ..."`
- **Few-shot** — include examples (see topic 9).
- **Delimiters/structure** — using XML tags, markdown headers, or triple-quotes to clearly separate instructions, context, and data so the model doesn't confuse them: e.g. wrapping user-provided text in `<document>...</document>` to prevent prompt injection and ambiguity.
- **Chain-of-Thought prompting** — asking the model to reason step by step (topic 10).
- **System vs. user message separation** — put stable behavioral instructions in the system prompt, put variable/task-specific content in the user turn.
- **Negative/positive framing** — telling the model what *to do* is generally more reliable than only telling it what *not* to do.
- **Output constraints** — asking for JSON, specific length, specific format (paired with structured outputs, topic 25, for reliability).

**Example — bad vs. good prompt:**
```
Bad:  "Summarize this."
Good: "Summarize the following support ticket in 2 sentences, 
       focused on the customer's core issue and desired resolution. 
       Do not include greetings or sign-offs.

       Ticket: {ticket_text}"
```

**Advanced patterns worth naming in an interview:**
- **ReAct (Reason + Act)** — interleave reasoning traces with tool calls/actions (foundational to agents, topic 20).
- **Self-consistency** — sample multiple CoT reasoning paths and take a majority vote on the final answer, improving accuracy on reasoning tasks.
- **Prompt chaining** — break a complex task into multiple sequential prompts/calls rather than one mega-prompt, each step's output feeding the next.
- **Meta-prompting / prompt optimization** — using an LLM to critique and rewrite your prompt (e.g., DSPy automates this).

**Interview point:** prompt engineering is inherently *empirical* — the same instruction can behave differently across model versions/providers, so production systems need prompt versioning + eval suites (topics 43/44), not just "it worked when I tried it once."

---

## 9. Few-Shot Learning

Few-shot learning (a.k.a. **in-context learning**) means giving the model a handful of example input-output pairs *inside the prompt* so it infers the task pattern, without any gradient updates/fine-tuning.

```
Classify the sentiment as Positive, Negative, or Neutral.

Review: "This laptop is amazing, best purchase ever!"
Sentiment: Positive

Review: "It broke after two days, total waste of money."
Sentiment: Negative

Review: "It's okay, does what it says."
Sentiment: Neutral

Review: "{new_review}"
Sentiment:
```

**Why it works:** during pre-training on huge amounts of text, the model has implicitly seen countless "pattern → continuation" structures. Few-shot examples activate an analogous in-context "task vector," effectively letting the model infer the mapping function from examples alone — sometimes described as **implicit meta-learning**, since the model was never explicitly trained to do sentiment classification, but generalizes from pattern completion.

**Zero-shot vs. one-shot vs. few-shot:**
- **Zero-shot** — no examples, relies purely on instruction-following ability (larger/more RLHF-tuned models do this well).
- **One-shot** — a single example, useful when format matters more than variety.
- **Few-shot** — typically 2–10 examples; more examples generally help until diminishing returns or context-window/cost limits kick in.

**Practical considerations:**
- **Example selection matters a lot** — diverse, representative, correctly-labeled examples improve performance; poorly chosen examples (e.g., all one class) can bias the model.
- **Example order can affect output** (recency/primacy effects) — a known instability of ICL.
- **Dynamic few-shot** — retrieving the *most relevant* examples per input (via embedding similarity, i.e., combining topic 6/13 with prompting) generally outperforms a fixed static example set.

**When to prefer few-shot over fine-tuning:** when you need quick iteration, don't have enough labeled data for fine-tuning (few-shot works with just a handful of examples vs. hundreds/thousands needed for fine-tuning), or need to support many tasks with one general model.

---

## 10. Chain-of-Thought (CoT)

Chain-of-Thought prompting elicits step-by-step intermediate reasoning before the final answer, dramatically improving performance on tasks requiring arithmetic, logic, or multi-step reasoning.

**Basic example:**
```
Q: Roger has 5 tennis balls. He buys 2 more cans of tennis balls. 
   Each can has 3 balls. How many tennis balls does he have now?

Standard prompting → A: 11   (sometimes wrong, no reasoning shown)

CoT prompting → 
A: Roger started with 5 balls. 2 cans × 3 balls = 6 new balls. 
   5 + 6 = 11. The answer is 11.
```

For simple problems the answer might coincidentally match, but CoT reliability wins grow sharply as problem complexity increases (multi-step math, logical deduction, planning).

**Why it works:** an LLM generates output token-by-token with fixed compute per token — without CoT, it has to "solve" a multi-step problem in a single forward pass with no scratch space. CoT effectively gives the model **external working memory** — each generated reasoning token becomes part of the context that later tokens can condition on, letting the model decompose a hard problem into a sequence of easier next-token predictions.

**Variants:**
- **Zero-shot CoT** — simply appending `"Let's think step by step"` to the prompt, no examples needed (Kojima et al., 2022) — surprisingly effective.
- **Few-shot CoT** — providing worked examples that include reasoning steps, not just final answers.
- **Self-consistency** — sample several CoT paths (with temperature > 0) and majority-vote the final answers to reduce variance from any single reasoning chain going wrong.
- **Tree-of-Thought** — explore multiple reasoning branches, evaluate/backtrack, more like search than a single linear chain — used for harder planning problems.

**Modern "reasoning models"** (OpenAI o1/o3, Claude extended thinking, DeepSeek-R1) internalize this idea via training: instead of relying on a hand-written "think step by step" prompt, the model is trained (via RL) to generate long internal reasoning traces natively, often allocating variable "thinking time" proportional to problem difficulty.

**Interview point:** CoT isn't free — it increases output tokens (cost + latency), and it isn't guaranteed to reflect the model's *actual* internal computation (research on "unfaithful" CoT shows models sometimes state one reasoning path but arrive at an answer via different internal mechanisms) — relevant to interpretability/safety discussions.

---

## 11. RAG (Retrieval-Augmented Generation)

RAG combines an LLM with an external retrieval system so the model can ground its answers in up-to-date or private information it wasn't trained on, without fine-tuning.

**Basic pipeline:**
1. **Ingest** — split documents into chunks (topic 16), embed each chunk (topic 6), store vectors in a vector DB (topic 12).
2. **Query time** — embed the user's question, retrieve top-K most similar chunks (topic 13), optionally rerank (topic 15).
3. **Augment** — insert retrieved chunks into the LLM prompt as context.
4. **Generate** — the LLM answers using the retrieved context, ideally citing sources.

```
System: Answer using ONLY the context below. If the answer isn't
        in the context, say "I don't know."

Context:
{retrieved_chunk_1}
{retrieved_chunk_2}

Question: {user_question}
```

**Why RAG instead of just fine-tuning or a bigger context window?**
- **Freshness** — update the knowledge base without retraining the model (e.g., new product docs indexed today are queryable immediately).
- **Grounding/reduces hallucination** — the model answers from retrieved evidence rather than relying purely on parametric memory, and can cite sources.
- **Cost** — retrieval + a smaller context is far cheaper than fine-tuning a model per knowledge update, and cheaper per-query than stuffing entire corpora into a huge context window.
- **Access control** — retrieval can be scoped per user/tenant (only search documents that user is permitted to see), which is much harder to enforce with fine-tuned "baked-in" knowledge.

**Failure modes to know for interviews:**
- **Retrieval miss** — relevant chunk never makes it into top-K (bad embeddings, bad chunking, query/document vocabulary mismatch) leads to the model hallucinating or incorrectly saying "I don't know."
- **Context dilution** — irrelevant retrieved chunks crowd out useful ones or confuse the model ("lost in the middle" — LLMs attend better to the start/end of context than the middle).
- **Stale index** — vector DB not resynced with source-of-truth documents.

**Advanced RAG patterns:**
- **Query rewriting/expansion** — rephrase the user's query (e.g., using an LLM) before embedding, to better match document phrasing.
- **HyDE (Hypothetical Document Embeddings)** — have the LLM generate a hypothetical answer first, embed that, and use it to retrieve — often matches document style better than the raw question.
- **Agentic/iterative RAG** — the model decides whether to retrieve, reformulate the query, or retrieve again based on intermediate results (vs. a single fixed retrieval step) — see topics 20/21.
- **GraphRAG** — build a knowledge graph from documents and retrieve via graph traversal for multi-hop questions that plain vector search handles poorly.

---

## 12. Vector Databases

A vector database stores high-dimensional embedding vectors and provides efficient **approximate nearest neighbor (ANN)** search — finding the vectors closest to a query vector without brute-force comparing against every stored vector (which is O(n) and too slow at scale).

**Why ANN, not exact kNN?** Exact nearest-neighbor search is O(n·d) per query — infeasible for millions/billions of vectors at low latency. ANN algorithms trade a small amount of recall for massive speedups.

**Key indexing algorithms:**
- **HNSW (Hierarchical Navigable Small World)** — builds a multi-layer graph where each node connects to its approximate nearest neighbors; search starts at a sparse top layer and descends, narrowing in. Excellent recall/speed trade-off, most widely used (Pinecone, Weaviate, Qdrant, pgvector all support it). Memory-hungry since it keeps the full graph in RAM.
- **IVF (Inverted File Index)** — clusters vectors (e.g., via k-means) into buckets ("cells"); at query time, only search the nearest few clusters instead of the whole dataset. Often combined with **PQ (Product Quantization)** to compress vectors and reduce memory (IVF-PQ), common in FAISS.
- **DiskANN / SPANN** — designed for billion-scale datasets that don't fit in RAM, using SSD-backed graph structures.

**Popular vector DBs and where they fit:**
- **Pinecone** — fully managed, simple API, popular for production RAG.
- **Weaviate / Qdrant / Milvus** — open-source, self-hostable, support hybrid search + metadata filtering.
- **pgvector** — a Postgres extension; great if you already run Postgres and want vectors alongside relational data without a new system.
- **FAISS** — a library (not a full DB) from Meta, used to build custom ANN pipelines, very common in research/offline batch settings.

**Core query mechanics:**
```python
results = index.query(
    vector=query_embedding,
    top_k=5,
    filter={"tenant_id": "acme_corp", "doc_type": "policy"}  # metadata filtering
)
```
Metadata filtering (combining vector similarity with structured filters like date, tenant, category) is essential in production — pure semantic similarity alone often isn't enough.

**Interview points:**
- Trade-off triangle: **recall vs. latency vs. memory** — tuning HNSW's `ef_search`/`M` parameters or IVF's `nprobe` moves you along this trade-off.
- Vector DB choice matters less than **data quality, chunking, and embedding model choice** — a common interview trap is over-focusing on the DB when retrieval quality is usually bottlenecked upstream.

---

## 13. Semantic Search

Semantic search retrieves documents based on **meaning**, not exact keyword overlap, by comparing embedding vectors instead of matching text tokens.

**How it differs from traditional (lexical) search:**
- **Lexical/keyword search (e.g., BM25/TF-IDF)** — matches exact or stemmed words; "car" won't match "automobile" unless explicitly synonym-mapped. Fast, interpretable, great for exact terms (IDs, error codes, names).
- **Semantic search** — embeds the query and documents into the same vector space; "How do I get a refund?" can match a document titled "Return and reimbursement policy" even with zero shared words, because their meanings are close in embedding space.

**Pipeline:**
```
query -> embedding model -> query_vector
                                |
                     cosine similarity search
                                |
              vector DB (topic 12) -> top-K nearest document vectors
```

**Strengths:** handles synonyms, paraphrasing, cross-lingual queries (with multilingual embedding models), conceptual similarity.

**Weaknesses:** can miss exact-match needs — e.g., a query for error code `ERR_504` might retrieve semantically "close" but wrong documents, because embeddings blur precise tokens; struggles with rare/unseen terminology not well represented in the embedding model's training data; less interpretable/debuggable than keyword matching ("why did it retrieve this?" is harder to answer).

**Example:** Query: "my package never arrived" retrieves a doc titled "Lost shipment resolution steps" even though it shares only the word "shipment" conceptually related to "package." A keyword search might miss this entirely or rank it low.

**This complementary weakness is exactly why hybrid search (topic 14) exists** — combining semantic recall with lexical precision.

---

## 14. Hybrid Search

Hybrid search combines **lexical (keyword) search** and **semantic (vector) search**, then merges the two ranked lists into one, to get the precision of exact matching plus the recall of semantic understanding.

**Why combine them:** Semantic search alone can miss exact identifiers, codes, acronyms, or rare proper nouns that embeddings don't represent precisely. Lexical search alone misses paraphrases and synonyms. In practice, production RAG systems almost always use hybrid search because real user queries mix both needs (e.g., "SKU-4471 return policy" needs both the exact SKU match and semantic understanding of "return policy").

**Common fusion technique — Reciprocal Rank Fusion (RRF):**
```
RRF_score(doc) = sum over i of  1 / (k + rank_i(doc))
```
summed over each ranking method `i` (e.g., BM25 rank and vector-search rank), where `k` is a constant (commonly 60) that dampens the influence of very high ranks. RRF is popular because it doesn't require normalizing scores from different systems onto the same scale (BM25 scores and cosine similarities aren't directly comparable) — it only uses rank position.

**Alternative: weighted score fusion:**
```
final_score = alpha * normalize(semantic_score) + (1 - alpha) * normalize(bm25_score)
```
requires score normalization (e.g., min-max scaling) and a tunable `alpha` — more sensitive to tuning than RRF but allows explicit control.

**Example:** Query "Roth IRA contribution limit 2025." BM25 nails "Roth IRA" and "2025" exactly. Semantic search understands "contribution limit" relates to documents phrased as "how much can I put into a Roth IRA." Hybrid search surfaces the document that best satisfies both.

**Interview point:** most production hybrid setups (Weaviate, Elasticsearch with vector support, Qdrant) implement BM25 + dense vector search + RRF out of the box — know the concept and when to reach for it (any RAG system serving real, varied user queries) rather than needing to hand-implement it.

---

## 15. Reranking

Reranking is a second-stage refinement step in a retrieval pipeline: retrieve a larger candidate set cheaply (e.g., top-50 via vector/hybrid search), then use a more expensive, more accurate model to re-score and reorder those candidates down to the final top-K (e.g., top-5) that actually go into the LLM prompt.

**Why two stages?** There's a fundamental trade-off in retrieval models:
- **Bi-encoders** (used for the first-stage embedding search) encode the query and each document independently into vectors, then compare via dot product/cosine similarity. Fast — documents can be pre-embedded and indexed — but less accurate because the query and document never directly interact during encoding.
- **Cross-encoders** (used for reranking) feed the query and document together into a single transformer pass, allowing full token-level attention between them, producing a much more accurate relevance score. Too slow to run over an entire corpus (must run once per query-document pair, no pre-indexing possible), but very affordable when only applied to a small candidate set (e.g., 50 candidates from stage one).

```
Stage 1 (bi-encoder, fast):        query_vec . doc_vec  for 1M docs -> top 50
Stage 2 (cross-encoder, accurate): score(query + doc_i jointly) for 50 docs -> top 5
```

**Popular rerankers:** Cohere Rerank, `bge-reranker`, cross-encoder models from `sentence-transformers`, or using an LLM itself as a reranker (prompting it to score relevance, more expensive but flexible).

**Example impact:** first-stage retrieval might return a document that mentions all the right keywords/topics but isn't actually the best answer (a tangentially related policy doc), while the truly best answer is ranked #8 due to embedding imprecision. A cross-encoder reranker, seeing query and document together, correctly promotes it to #1.

**Interview point:** reranking is one of the highest-ROI additions to a mediocre RAG pipeline — it's often a bigger accuracy win than switching embedding models, because it directly optimizes for the thing that matters (query-document relevance) rather than relying on embedding-space geometry as a proxy.

---

## 16. Chunking Strategies

Chunking splits large documents into smaller pieces before embedding, because (a) embedding models have input length limits, (b) very long chunks dilute the embedding (averaging semantics across too much content reduces retrieval precision), and (c) you want to feed the LLM focused, relevant context rather than entire documents.

**Common strategies:**
- **Fixed-size chunking** — split every N tokens/characters (e.g., 512 tokens), often with overlap (e.g., 50 tokens) so context isn't lost at chunk boundaries. Simple, fast, but can cut sentences/ideas awkwardly.
- **Recursive character/token splitting** — try splitting on paragraph breaks first, then sentences, then words, only falling back to hard cuts when necessary — keeps semantically coherent units together (LangChain's `RecursiveCharacterTextSplitter` is the canonical example).
- **Semantic chunking** — compute embeddings for consecutive sentences and split where semantic similarity drops significantly (topic shift detection) — chunks align with actual content boundaries rather than arbitrary length.
- **Structure-aware chunking** — respect document structure: split by Markdown headers, HTML sections, code function/class boundaries, or table rows — critical for structured content like docs, code, or legal contracts where splitting mid-table or mid-function destroys meaning.
- **Sliding window / hierarchical chunking** — keep small chunks for precise retrieval, but attach a larger surrounding "parent" chunk (or document summary) that gets included in context once the small chunk is matched — sometimes called **small-to-big retrieval** or **parent-document retrieval**: retrieve on a small, focused unit for precision, but generate using more surrounding context.

**Chunk size trade-off:**
- **Too small** — loses context (a chunk might reference "it" or "the policy" without stating what that refers to), retrieval matches on narrow fragments.
- **Too large** — embedding gets "diluted" (represents too many ideas at once, poor discriminative power in vector search), wastes context window, may include irrelevant content alongside the relevant part.

**Example:** For a 50-page technical manual, chunking by section headers (e.g., "3.2 Troubleshooting Network Errors") with ~300-500 token chunks and 10-15% overlap typically retrieves cleanly — a query about network errors matches a focused, self-contained chunk instead of a diluted whole-chapter embedding or a fragment cut off mid-sentence.

**Interview point:** there's no universal "best" chunk size — it depends on document type, query patterns, and the embedding model's effective context; production systems typically A/B test chunking strategies against a retrieval eval set (topic 44) rather than picking one theoretically.

---

## 17. Context Windows

The context window is the maximum number of tokens (input + output combined, for most APIs) an LLM can process in a single request — it's the model's "working memory" for that call.

**Why it's limited:** self-attention is O(n squared) in sequence length in naive implementations — doubling context length roughly quadruples compute/memory for attention, though optimizations (FlashAttention, sparse/sliding-window attention, linear attention variants) have pushed this cost down considerably, enabling the jump from 2K-token windows (early GPT-3) to 128K-1M+ token windows in modern models (GPT-4 Turbo, Claude, Gemini 1.5).

**"Lost in the middle" phenomenon:** research (Liu et al., 2023) showed LLMs tend to attend better to information at the start and end of a long context than content buried in the middle, even when the model's stated context limit is much larger than the input — meaning a bigger context window doesn't guarantee uniformly good use of everything in it. Practical implication: put the most important information (the actual question, critical instructions) near the start or end of the prompt, not sandwiched in the middle of a large context dump.

**Context window vs. RAG — not either/or:** a common interview question is "why use RAG if context windows are huge now?" Answer: even with 1M-token windows, (a) cost scales with tokens processed (every token in context costs money and adds latency on every single call, even if 95% is irrelevant), (b) retrieval keeps only relevant content in context, improving accuracy and reducing "lost in the middle" risk, (c) some corpora (a company's entire document set) still exceed even huge windows, and (d) RAG allows per-query access control that dumping everything into context can't replicate. In practice, large context windows and RAG are often combined — retrieval narrows a huge corpus down to a manageable, still-fits-in-context set of highly relevant documents.

**Context window management techniques in production:**
- **Truncation** — drop oldest turns in a long conversation (crude but simple).
- **Summarization/compaction** — periodically summarize earlier conversation history into a compact form to preserve important information while freeing tokens (used in coding agents, chat memory systems).
- **Sliding window** — keep only the last N turns/tokens verbatim.

**Example:** A 128K context window model can technically hold an entire codebase, but a well-designed coding agent still uses retrieval/search to pull in only relevant files instead of pasting the whole repo into every prompt — for cost, latency, and to avoid "lost in the middle" degrading the model's focus.

---

## 18. Function Calling

Function calling lets an LLM output a structured request to invoke a specific function (with typed arguments) rather than just free-form text, so an external system can execute real code/actions and return results back to the model.

**How it works (mechanically):**
1. Developer defines available functions with a schema (name, description, parameters — typically JSON Schema).
2. The schema is sent alongside the user's prompt.
3. The model, if it decides a function is needed, outputs a structured call instead of (or alongside) natural language.
4. The application (not the model) actually executes the function with those arguments.
5. The result is fed back into the conversation as a new message, and the model produces a final natural-language response using that result.

**Example function schema:**
```json
{
  "name": "get_weather",
  "description": "Get current weather for a given city",
  "parameters": {
    "type": "object",
    "properties": {
      "location": {"type": "string", "description": "City name"},
      "unit": {"type": "string", "enum": ["celsius", "fahrenheit"]}
    },
    "required": ["location"]
  }
}
```

Example call the model would emit:
```json
{ "name": "get_weather", "arguments": { "location": "Bengaluru", "unit": "celsius" } }
```

**Why it matters:** it's the mechanism that turns an LLM from a pure text generator into something that can take real-world actions — query a database, call an API, run a calculation, send an email — grounded in structured, validated arguments instead of asking the model to format everything correctly in free text (unreliable) or asking users to manually copy-paste.

**How the model "knows" the schema:** the model was specifically fine-tuned (during SFT/RLHF) to recognize when a user's intent matches an available function and to emit correctly formatted structured calls — this is a trained capability, not something that works reliably on an arbitrary un-tuned base model.

**Interview point:** function calling underlies both structured outputs (topic 25) and tool calling/agents (topics 19-20) — it's the foundational primitive; "tool calling" is essentially function calling applied specifically to give the model access to external tools/actions rather than just formatting output.

---

## 19. Tool Calling

Tool calling is function calling (topic 18) applied specifically to give an LLM the ability to use external tools — web search, code execution, calculators, APIs, databases — to accomplish tasks it can't (or shouldn't) do purely from its parametric knowledge.

**Why LLMs need tools:**
- **Knowledge cutoff** — a model can't know today's stock price or news; a web search tool fixes this.
- **Precision** — LLMs are unreliable at exact arithmetic on large numbers; a calculator/code-execution tool guarantees correctness.
- **Grounding/side effects** — sending an email, updating a database row, or booking a flight requires actually doing something in the world, not just generating text about it.

**Typical tool-use loop:**
```
1. User asks a question requiring both a calculation and live data.
2. Model emits a tool_call for the calculator with the right arguments.
3. App executes the tool and returns the numeric result.
4. Model emits a tool_call for a weather API.
5. App executes and returns structured weather data.
6. Model combines both results into a final natural-language answer.
```

This loop can run for multiple turns (multi-step tool use) before the model produces a final answer — this iterative loop is exactly what powers AI agents (topic 20).

**Parallel vs. sequential tool calls:** modern models can emit multiple tool calls in one turn when they're independent (e.g., checking weather in 3 cities simultaneously), reducing latency versus calling them one at a time; sequential calls are needed when a later call depends on an earlier result (e.g., search for a restaurant, then check its specific reviews).

**Design considerations that come up in interviews:**
- **Tool descriptions matter enormously** — the model decides whether and how to call a tool based on the natural-language description in the schema; vague descriptions cause wrong or missed tool use, just like a bad docstring confuses a human developer.
- **Error handling** — tools can fail (API timeout, invalid args); the model needs the error message fed back so it can retry, use a different tool, or gracefully inform the user.
- **Security** — tool execution is a real trust boundary; arguments generated by an LLM should be validated/sandboxed before execution (e.g., never directly execute model-generated code without sandboxing; validate SQL generated by a model before running it against production).

---

## 20. AI Agents

An AI agent is an LLM-driven system that autonomously decides a sequence of actions (reasoning, tool calls, reflection) to accomplish a goal, rather than producing a single one-shot response. The defining feature vs. a plain LLM call is a loop: observe, think, act, observe result, repeat, until the goal is achieved or a stopping condition is hit.

**Core loop (the "agent loop"), in pseudocode:**
```
while not done:
    thought = llm.reason(goal, history, observations)
    if thought.needs_tool:
        action = thought.tool_call
        observation = execute(action)
        history.append(action, observation)
    else:
        return thought.final_answer
```

**The ReAct pattern** (Yao et al., 2022) — "Reason + Act" — is the foundational technique: the model interleaves explicit reasoning traces ("I need to check the user's order status first") with actions (tool calls) and observations (tool results), which measurably improves reliability over either pure reasoning or pure action-taking alone, because the reasoning step lets the model plan and self-correct between actions.

**Key components of a real agent system:**
- **Planning** — decomposing a goal into steps (can be explicit up-front planning, or implicit/reactive step-by-step decision making).
- **Tool use** (topic 19) — the actions available to the agent.
- **Memory** (topic 22) — retaining relevant information across steps/sessions (short-term: conversation/scratchpad; long-term: persisted facts/preferences).
- **Reflection/self-correction** — the agent evaluates its own output or intermediate results and revises course (e.g., "that search returned nothing useful, let me try a different query").

**Example — a customer-support agent handling "Cancel my subscription and refund last month's charge":**
The agent first looks up the account from the user's email, then calls a cancel-subscription tool with the returned account ID, then calls a refund tool with that same account ID and the charge amount it observed, and only after all three tool results come back successfully does it produce a final confirmation message to the user — each step's action depends on the previous step's observation.

**Agents vs. plain prompting/workflows:** a fixed pipeline (e.g., a hardcoded RAG chain) always executes the same steps in the same order. An agent dynamically decides which steps to take and in what order based on intermediate results — more flexible/powerful, but less predictable and harder to test/debug, which is the central engineering trade-off (see topic 21).

**Interview point:** be ready to discuss failure modes — agents can loop indefinitely, call the wrong tool, hallucinate tool results if not properly grounded, or accumulate errors across steps (compounding error rate — if each step is 95% reliable, a 10-step agent task succeeds only about 60% of the time: 0.95 to the power of 10 is roughly 0.60), which is why guardrails, step limits, and human-in-the-loop checkpoints matter in production agent design.

---

## 21. Agentic Workflows

An agentic workflow is a system where an LLM's outputs shape control flow — which step runs next, whether to loop, retry, or branch — rather than the developer hardcoding a fixed sequence of calls. It's a spectrum, not a binary: from fully deterministic pipelines to fully autonomous open-ended agents.

**Common workflow patterns (from Anthropic's "Building Effective Agents" framing, useful vocabulary for interviews):**
- **Prompt chaining** — a fixed sequence of LLM calls where each step's output feeds the next (e.g., outline -> draft -> critique -> revise). Deterministic control flow, LLM only fills in content — easiest to test and debug.
- **Routing** — an initial LLM call classifies the input and dispatches it to one of several specialized downstream prompts/paths (e.g., a support router sending billing questions to one prompt and technical questions to another).
- **Parallelization** — run multiple LLM calls simultaneously, either splitting a task into independent subtasks (sectioning) or running the same task multiple times to vote/aggregate (voting) — e.g., running several independent "does this content violate policy" checks and aggregating.
- **Orchestrator-workers** — a central LLM dynamically breaks a task into subtasks and delegates each to worker LLM calls, then synthesizes results — used when subtasks can't be predicted in advance (unlike parallelization's fixed split).
- **Evaluator-optimizer** — one LLM generates a response, another LLM evaluates it against criteria and gives feedback, looping until the evaluator is satisfied — useful when quality criteria are clear but hard to get right in one shot (e.g., iterative code generation with a test-running evaluator).
- **Fully autonomous agent** (topic 20) — the model decides everything: which tools to call, in what order, when to stop — most flexible, least predictable.

**Why this matters for interviews:** a very common design mistake is reaching for a fully autonomous agent when a simpler fixed workflow (prompt chaining, routing) would be more reliable, cheaper, and easier to debug. The engineering principle: use the simplest pattern that reliably solves the task, and only add agentic autonomy where the task genuinely requires open-ended, unpredictable decision-making (e.g., debugging an unfamiliar codebase, where the number and order of steps can't be known in advance).

**Example — routing pattern:** an email-triage system uses one cheap/fast LLM call to classify an incoming email as "sales," "support," or "spam," then routes to a specialized prompt (and possibly a specialized, cheaper or more expensive model) tuned for that category — this is far more reliable than one giant prompt trying to handle all three cases at once.

---

## 22. Memory Systems

Memory in AI agent/LLM systems means retaining and retrieving relevant information across turns or sessions, since the LLM itself is stateless — every call only "knows" what's in its current context window.

**Types of memory:**
- **Short-term / working memory** — the current conversation history or reasoning scratchpad, held directly in the context window for the duration of a session. Simple but bounded by context window size and cost.
- **Long-term memory** — information persisted *outside* the context window (a database, vector store, or file) and selectively retrieved back into context when relevant, across sessions or even across different users' conversations with a shared knowledge base.
- **Episodic memory** — records of specific past interactions/events ("last week the user asked about refund policy and was frustrated about the wait time") — useful for continuity and personalization.
- **Semantic memory** — general facts/knowledge extracted and consolidated from interactions ("this user prefers concise answers," "this user is a Python developer") — usually distilled/summarized rather than stored as raw transcripts.
- **Procedural memory** — learned patterns of *how* to do something (e.g., a coding agent remembering "this repo's tests are run with `pytest -x`") — often stored as reusable instructions/rules rather than facts.

**Implementation approaches:**
- **Summarization-based** — periodically compress conversation history into a shorter summary to fit more turns in the effective context (used when context grows too large — see topic 17).
- **Retrieval-based (RAG-style memory)** — embed and store facts/past interactions in a vector DB, retrieve relevant memories per new query, exactly analogous to RAG over documents but the "documents" are the agent's own memories.
- **Structured memory stores** — explicit key-value or structured records (e.g., a user profile table: preferences, past purchases) rather than unstructured text, when the memory has a clear schema.

**Example:** A coding assistant that remembers, across sessions, that a particular repo uses `pnpm` not `npm`, and that the user prefers tabs over spaces — this gets written to a persistent memory store after being learned once, then retrieved and injected into the system prompt or context at the start of future sessions, so the user never has to repeat that preference.

**Interview point:** the central design challenge is **relevance filtering** — naively dumping all past history into every new context wastes tokens and reintroduces "lost in the middle" problems; good memory systems selectively retrieve only what's relevant to the current task (often via embedding similarity, recency, or explicit importance scoring), similar in spirit to RAG's retrieval step but applied to the agent's own history instead of external documents.

---

## 23. Multi-Agent Systems

A multi-agent system uses multiple LLM-driven agents, often with distinct roles, specialized tools, or separate context windows, that collaborate (or compete) to solve a task that a single agent would struggle with — either due to task complexity, need for specialization, or context-window/focus limitations.

**Why use multiple agents instead of one bigger agent/prompt?**
- **Specialization** — a "researcher" agent, a "coder" agent, and a "reviewer" agent can each have a narrower, more focused system prompt and toolset, which tends to produce more reliable behavior than one agent juggling every responsibility at once.
- **Context isolation** — each sub-agent gets its own clean context window, so a long research task's noisy intermediate steps don't pollute the main "orchestrator" agent's context (which stays focused on the high-level goal) — critical for keeping quality high on long-running tasks.
- **Parallelism** — independent sub-agents can work concurrently on different parts of a problem (e.g., multiple research sub-agents exploring different angles of a question simultaneously), then have results synthesized.

**Common architectures:**
- **Orchestrator/supervisor + workers** — a lead agent decomposes the task, dispatches subtasks to worker agents, and synthesizes their results into a final answer (this is the architecture behind systems like Anthropic's multi-agent research feature).
- **Sequential pipeline of agents** — each agent's output becomes the next agent's input (e.g., a "planner" agent hands off to a "coder" agent, which hands off to a "tester" agent).
- **Debate/critique setups** — two or more agents argue or critique each other's outputs to surface errors or converge on a better answer than either would produce alone.
- **Peer/swarm** — decentralized agents communicating and negotiating without a single central controller (less common in production, more researchy).

**Example:** A "deep research" system uses a lead agent to break a broad question ("compare the top 5 cloud providers' GPU pricing") into sub-questions, spins up parallel sub-agents to research each provider independently (each with its own search tool calls and context), then the lead agent synthesizes their findings into a single coherent report — this finishes faster and produces a more thorough result than one agent sequentially researching all 5 providers in a single context.

**Trade-offs / interview points:**
- **Cost multiplies** — N agents making LLM calls costs roughly N times more tokens than a single agent, so multi-agent systems must be justified by a real quality/speed win, not used by default.
- **Coordination overhead** — sub-agents can duplicate work, miss context the orchestrator had, or produce inconsistent outputs that need careful synthesis/reconciliation logic.
- **Debugging complexity** — failures can originate in any sub-agent or in the orchestration logic itself, making root-causing harder than a single-agent system.

---

## 24. MCP (Model Context Protocol)

MCP is an open protocol (introduced by Anthropic) that standardizes how LLM applications connect to external tools, data sources, and systems — essentially "USB-C for AI applications." Instead of every application writing custom, one-off integration code for every tool/data source it wants an LLM to use, MCP defines a common client-server interface.

**The problem it solves:** before a standard, every AI app (Claude Desktop, an IDE assistant, a custom agent) that wanted to access, say, GitHub, Slack, or a local filesystem had to write bespoke integration code for each combination of app and tool — an M x N integration problem (M apps times N tools). MCP turns this into an M + N problem: any MCP-compatible client can talk to any MCP-compatible server without custom glue code.

**Core architecture:**
- **MCP Server** — exposes a set of capabilities: **tools** (functions the model can call, e.g., `create_github_issue`), **resources** (data the model can read, e.g., a file or database record), and **prompts** (reusable prompt templates). A server wraps a specific system (GitHub, a database, a filesystem, Slack).
- **MCP Client** — lives inside the AI application (e.g., Claude Desktop, an IDE, a custom agent runtime); discovers available servers, lists their capabilities, and routes model requests to the right server.
- **Transport** — communication happens over a defined protocol (JSON-RPC based), locally via stdio or remotely via HTTP/SSE, so servers can run locally (e.g., a local filesystem server) or remotely (e.g., a hosted SaaS integration).

**Example:** A developer wants their coding agent to be able to query their company's internal ticket tracker. Without MCP, they'd write custom code translating the agent's tool-call format into that tracker's specific API. With MCP, if someone has already published an MCP server for that ticket tracker (or a generic one for its API style), the agent's MCP *client* can immediately discover and use it — no custom integration code needed, and the same server works with any other MCP-compatible AI application too.

**How it relates to function/tool calling (topics 18-19):** MCP doesn't replace function calling — it standardizes *how tools are discovered and exposed* to the model. Under the hood, an MCP tool still gets translated into the same kind of structured function-call schema the underlying LLM API expects; MCP's contribution is the standardized, reusable packaging and transport layer around that, plus resources/prompts beyond just tools.

**Interview point:** be ready to contrast MCP (a protocol/integration standard, analogous conceptually to how LSP standardized editor-language integrations) with agent frameworks (LangChain, LlamaIndex, etc., which are code libraries for building agent logic) — they're complementary, not competing: a framework can use MCP servers as one of its tool sources.

---

## 25. Structured Outputs

Structured outputs constrain an LLM's response to conform to a specific schema (most commonly JSON matching a JSON Schema, or a Pydantic/TypeScript-style type definition), so downstream code can reliably parse the model's output without regex hacks or fragile prompt-based formatting requests.

**Why this is hard without special support:** simply asking a model in a prompt ("please respond in JSON") is unreliable — models can add explanatory prose before/after the JSON, produce malformed JSON (trailing commas, unescaped quotes), or drift from the requested schema on edge cases. This unreliability is unacceptable when the output feeds directly into code (e.g., populating a database record, triggering a function call).

**Two main technical approaches providers use:**
- **Constrained decoding / grammar-based sampling** — at each generation step, the model's output vocabulary is masked so it can *only* produce tokens that keep the output valid according to the target schema (a formal grammar derived from the JSON Schema). This makes invalid output structurally impossible, not just unlikely — the API/serving layer intervenes in the token sampling process itself.
- **Fine-tuning for reliable JSON mode** — the model is trained specifically to be very good at emitting clean JSON for a given schema, without hard grammar constraints at inference — more flexible but not a hard guarantee (can still occasionally fail, though rarely with modern implementations).

**Example (using a typed schema):**
```python
from pydantic import BaseModel

class Invoice(BaseModel):
    vendor: str
    total: float
    line_items: list[str]

response = client.chat.completions.create(
    model="...",
    messages=[...],
    response_format=Invoice,   # schema enforced on output
)
invoice = response.parsed  # guaranteed to match Invoice's shape
```

**Where structured outputs matter most:**
- **Function/tool calling** (topics 18-19) — the arguments passed to a function must match its parameter schema exactly.
- **Data extraction pipelines** — pulling structured fields (names, dates, amounts) out of unstructured documents (invoices, resumes, contracts) into a database.
- **Agent decision-making** — having an agent output a structured "next action" (e.g., `{"action": "search", "query": "..."}`) that application code can safely branch on.

**Interview point:** structured outputs improve *format* reliability but don't guarantee *content* correctness — a perfectly schema-valid JSON object can still contain a hallucinated or wrong value (e.g., a correctly formatted `{"total": 4999.00}` that's simply the wrong number). Structured outputs and hallucination detection/evaluation (topics 42-43) are separate concerns that both matter in production data-extraction pipelines.

---

## 26. Fine-Tuning

Fine-tuning further trains a pre-trained model's weights on a smaller, task/domain-specific dataset, adapting its behavior beyond what prompting alone can achieve.

**When fine-tuning is the right lever (vs. prompting/RAG):**
- **Consistent style/format** — you need the model to reliably respond in a very specific tone, format, or persona across thousands of varied inputs, more reliably than few-shot examples can guarantee.
- **Domain-specific behavior/knowledge that's procedural, not factual** — e.g., teaching a model your company's specific classification taxonomy or code style, versus needing it to know a *fact* (which RAG handles better and more updatably).
- **Latency/cost reduction** — fine-tune a smaller, cheaper model to match a larger model's behavior on a narrow task, avoiding the cost of few-shot examples in every prompt and the latency/expense of a bigger model.
- **Behavior that's hard to specify in a prompt** — subtle, example-driven patterns (e.g., "sound like our brand voice") that are more efficiently taught via many examples than described in words.

**When fine-tuning is the *wrong* lever:** injecting new factual/frequently-changing knowledge (use RAG instead — fine-tuning "bakes in" facts as of training time and is expensive to keep current); fixing an occasional error better addressed by better prompting or a guardrail; when you don't have enough high-quality labeled examples (fine-tuning needs at minimum dozens to hundreds of good examples, ideally thousands).

**Full fine-tuning vs. parameter-efficient fine-tuning:** full fine-tuning updates *all* of a model's weights — most accurate potential adaptation, but extremely expensive in compute/memory (must store optimizer states for every parameter) and risks **catastrophic forgetting** (degrading the model's general capabilities while overfitting to the narrow fine-tuning task). This is why parameter-efficient methods like LoRA (topic 27) dominate practical fine-tuning today.

**Typical workflow:**
```
1. Curate a dataset of (input, ideal_output) pairs representative of the target task.
2. Format according to the provider's fine-tuning schema (e.g., chat-style messages).
3. Run the fine-tuning job (managed API, e.g., OpenAI/Anthropic fine-tuning, or self-hosted).
4. Evaluate the fine-tuned model against a held-out test set and against the base model as baseline.
5. Deploy, monitor for regressions in capabilities not covered by the fine-tuning set.
```

**Example:** A legal-tech company fine-tunes a model on thousands of examples of contract clauses labeled with risk categories, so the model reliably classifies new clauses into their exact taxonomy with consistent formatting — something few-shot prompting could approximate but not match in consistency at scale, and something RAG doesn't help with since it's a classification *skill*, not a factual lookup.

---

## 27. LoRA & QLoRA

**LoRA (Low-Rank Adaptation)** is a parameter-efficient fine-tuning technique: instead of updating a model's full weight matrices, it freezes the original pre-trained weights and injects small, trainable **low-rank decomposition matrices** alongside them, drastically reducing the number of trainable parameters.

**The core idea:** for a weight matrix `W` (say, d x d), a full update would be `W' = W + ΔW`, where ΔW is also d x d — huge. LoRA approximates `ΔW` as the product of two much smaller matrices: `ΔW ≈ B · A`, where `A` is r x d and `B` is d x r, with rank `r` much smaller than `d` (e.g., r=8 or 16 vs. d=4096). Only `A` and `B` are trained; the original `W` stays frozen.

```
h = W·x + ΔW·x = W·x + B·A·x
```

Since `r << d`, the number of trainable parameters drops by orders of magnitude — e.g., for a 4096x4096 matrix, full fine-tuning trains ~16.8M params per matrix, while LoRA with r=8 trains only ~65K params (4096×8 + 8×4096) — roughly a 250x reduction for that layer.

**Why this works well in practice:** research (and wide practical adoption) shows the *change* needed to adapt a pre-trained model to a new task tends to have a low "intrinsic rank" — i.e., you don't need to move the weights through a huge number of independent directions to specialize behavior, so a low-rank approximation captures most of the useful adaptation.

**Practical benefits:**
- **Much less GPU memory** — no need to store gradients/optimizer states for the full weight matrices, only for the small A/B matrices.
- **Fast to train, fast to switch** — you can keep one frozen base model in memory and swap in different small LoRA adapters for different tasks/customers, rather than hosting many full fine-tuned model copies.
- **No inference latency cost at merge time** — `B·A` can be added directly into `W` after training (`W_new = W + B·A`), so a merged LoRA model runs exactly as fast as the original at inference.

**QLoRA (Quantized LoRA)** extends this further: the frozen base model is loaded in **4-bit quantized** precision (topic 32) to shrink memory footprint even more, while the small LoRA adapter matrices are still trained in higher precision (e.g., bf16) for stable gradients. This is what makes it possible to fine-tune a 65B-parameter model on a single consumer/prosumer GPU (a technique popularized by the QLoRA paper, Dettmers et al., 2023), by combining 4-bit base-weight storage with LoRA's small trainable footprint.

**Interview point:** know the practical trade-off — LoRA/QLoRA rarely matches full fine-tuning's absolute ceiling on very large, diverse fine-tuning datasets, but the memory/cost savings are so large (often 10-100x less GPU memory) that it's the default choice for the vast majority of real-world fine-tuning use cases.

---

## 28. PEFT (Parameter-Efficient Fine-Tuning)

PEFT is the umbrella category of techniques — of which LoRA/QLoRA (topic 27) is the most popular member — that adapt a pre-trained model to a new task by training only a small subset of parameters (or a small number of newly added parameters), instead of updating the full model.

**Why PEFT matters beyond just LoRA:**
- **Compute/memory efficiency** — training and storing full copies of billion-parameter models per task is often prohibitively expensive; PEFT reduces trainable parameters typically to under 1% of the full model.
- **Mitigates catastrophic forgetting** — since most of the original weights stay frozen, the model's general pre-trained capabilities are much better preserved compared to full fine-tuning, which can overwrite broadly useful representations while overfitting to a narrow task.
- **Modularity/multi-tenancy** — small adapters can be trained per customer/task and swapped in/out against a single shared frozen base model, which is far cheaper to serve at scale than hosting N fully fine-tuned models.

**Other PEFT methods worth naming (beyond LoRA) for interview breadth:**
- **Prompt tuning** — instead of modifying any model weights, learn a small set of continuous "soft prompt" embeddings prepended to the input that are optimized via gradient descent (unlike hand-written discrete prompts) to steer the frozen model's behavior toward the target task.
- **Prefix tuning** — similar idea, but learned continuous vectors are prepended to the keys/values at every attention layer (not just the input embeddings), giving more influence over the model's internal computation than prompt tuning alone.
- **Adapters (bottleneck adapters)** — small trainable feed-forward "bottleneck" modules inserted between existing frozen Transformer layers (down-project, nonlinearity, up-project); only these new modules are trained. Predates LoRA and inspired similar efficiency goals, but adds inference latency (extra layers to run through) in a way LoRA's mergeable weights avoid.
- **(IA)³ (Infused Adapter by Inhibiting and Amplifying Inner Activations)** — learns per-channel scaling vectors applied to activations, even fewer trainable parameters than LoRA, though generally less expressive.

**How to frame this in an interview:** PEFT is the general category/goal ("adapt a large frozen model cheaply"), and LoRA is the dominant, most widely deployed *technique* within that category today because of its favorable accuracy/efficiency trade-off and the fact that it adds zero inference-time overhead once merged — that combination is why it beat out adapters and prompt/prefix tuning as the default choice in most production fine-tuning workflows.

---

## 29. RLHF (Reinforcement Learning from Human Feedback)

RLHF is the technique used to align a pre-trained (and instruction-tuned) LLM's behavior with human preferences — making it more helpful, harmless, and honest — by training it with reinforcement learning against a reward signal derived from human judgments, rather than from a fixed labeled dataset alone.

**Why it's needed beyond SFT (Supervised Fine-Tuning):** SFT teaches a model to imitate example responses, but writing a large, diverse dataset that covers every nuance of "what makes a response good" is impractical, and imitation alone doesn't teach the model to *compare* and prefer better responses over worse ones. RLHF instead teaches the model to optimize directly for a learned notion of quality.

**The classic three-stage RLHF pipeline (as used for models like InstructGPT/GPT-3.5):**
1. **Supervised Fine-Tuning (SFT)** — start from a pre-trained base model, fine-tune on high-quality human-written demonstrations of desired behavior (instruction -> ideal response pairs).
2. **Reward Model (RM) training** — humans rank multiple model outputs for the same prompt from best to worst; a separate model (the reward model) is trained to predict this preference ranking, producing a scalar "quality score" for any (prompt, response) pair.
3. **RL fine-tuning (typically PPO — Proximal Policy Optimization)** — the SFT model is further trained using reinforcement learning, where the reward model's score is the reward signal: the policy (the LLM) is updated to generate responses that the reward model scores highly, while a KL-divergence penalty against the original SFT model keeps it from drifting too far and degenerating (e.g., exploiting the reward model with nonsensical high-scoring text — "reward hacking").

```
reward = RM_score(response) - beta * KL(policy || SFT_reference_policy)
```
The KL penalty term is essential — without it, the policy would over-optimize against the reward model's imperfections rather than genuinely improving response quality (Goodhart's law: "when a measure becomes a target, it ceases to be a good measure").

**Example:** given the prompt "Explain photosynthesis," humans rank several candidate model responses from most to least helpful/accurate/clear. The reward model learns to predict these rankings. Then PPO nudges the LLM's generation policy to produce more responses like the ones the reward model scores highly — over many prompts and iterations, this shapes the model toward being more helpful and better calibrated to human preferences than raw next-token prediction or plain SFT alone would produce.

**Downsides that motivated DPO (topic 30):** RLHF with PPO is complex (requires training and maintaining three separate models: policy, reward model, and reference model), computationally expensive, and can be unstable to tune (RL training is notoriously sensitive to hyperparameters).

---

## 30. DPO (Direct Preference Optimization)

DPO is a simpler alternative to RLHF's RL-based approach that achieves a similar alignment goal — optimizing a model to prefer better responses over worse ones — **without** training a separate reward model or running unstable RL optimization (like PPO).

**Key insight behind DPO:** the RLHF objective (maximize reward model score, subject to a KL penalty against a reference policy) has a closed-form mathematical relationship between the optimal policy and the reward function. DPO exploits this to reframe the *reward modeling + RL* problem as a single, direct **classification-style loss** computed straight from human preference-pair data (chosen response vs. rejected response), optimized with standard supervised learning (just gradient descent on a loss function, no RL loop, no separate reward model, no sampling from the policy during training).

**DPO loss (conceptually):**
```
L_DPO = -log sigma( beta * [ log(pi(y_chosen|x)/pi_ref(y_chosen|x))
                            - log(pi(y_rejected|x)/pi_ref(y_rejected|x)) ] )
```
In words: increase the model's relative log-probability of the *chosen* (human-preferred) response compared to the reference model, and decrease its relative log-probability of the *rejected* response, compared to the same reference model — `beta` controls how strongly to deviate from the reference policy (playing the same role as the KL penalty coefficient in RLHF).

**Why this matters practically:**
- **Simpler pipeline** — just need (prompt, chosen_response, rejected_response) triples and a frozen reference model (usually the SFT checkpoint) — no reward model training stage, no RL rollouts.
- **More stable training** — it's a standard supervised loss (similar complexity to a classification/contrastive loss), avoiding PPO's well-known training instability and sensitivity to hyperparameters.
- **Comparable quality** — the DPO paper (Rafailov et al., 2023) and substantial follow-up work show DPO matches or exceeds PPO-based RLHF on many alignment benchmarks, at much lower engineering complexity/cost, which is why many modern open-weight models (e.g., Llama variants, Zephyr, Mistral-instruct variants) use DPO or DPO-family methods instead of full RLHF.

**Example preference pair used to train with DPO:**
```
Prompt: "How do I pick a lock?"
Chosen:   "I can't help with that, but if you're locked out, 
           here's how to contact a licensed locksmith..."
Rejected: "Here's a step-by-step guide to picking a lock: ..."
```
DPO directly increases the model's preference for the chosen (safe, helpful) response relative to the rejected one, for prompts like this across a large preference dataset.

**Interview point:** be ready to state the trade-off honestly — DPO is offline (trains on a fixed, pre-collected preference dataset) while PPO-based RLHF is online (the policy actively generates new responses that get scored during training), which means RLHF can in principle keep improving by exploring new responses the reward model then scores, while DPO's quality is more tightly bounded by the fixed preference dataset it's given — a nuance worth mentioning if asked "is DPO strictly better than RLHF?"

---
