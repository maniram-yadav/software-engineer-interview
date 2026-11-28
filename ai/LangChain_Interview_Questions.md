# LangChain Interview Question Set — Concepts, Pipelines, Use Cases & Observability

> A detailed interview guide covering every core LangChain concept, how to build different kinds of pipelines (with real use cases), and observability and production configuration. Each answer explains the idea, the trade-offs and an example, with Python where code helps.
>
> **Version note:** Written against **LangChain 1.x**. Import paths were checked against `langchain 1.5.2`, `langchain-core 1.8.5`, `langchain-classic 1.1.0`, `langgraph 1.2.24` and `langsmith 0.16.2`. In 1.x:
> - `create_agent` (built on LangGraph) is the standard agent,
> - **middleware** is the main way to customise agents,
> - legacy chains (`RetrievalQA`, `ConversationChain`, `create_retrieval_chain`, classic retrievers) moved to **`langchain-classic`**,
> - `langchain-community` now emits a **sunset warning**, so prefer standalone integration packages (`langchain-openai`, `langchain-postgres`, `langchain-qdrant`, …).
>
> Snippets that call real models or services were not executed.

---

## Table of Contents

- [Part A — Core Concepts (Q1–Q28)](#part-a--core-concepts-q1q28)
- [Part B — Pipeline Patterns & Use Cases (Q29–Q47)](#part-b--pipeline-patterns--use-cases-q29q47)
- [Part C — Observability & Configuration (Q48–Q60)](#part-c--observability--configuration-q48q60)
- [Part D — Production Scenario Questions (Q61–Q72)](#part-d--production-scenario-questions-q61q72)
- [Part E — Rapid-Fire Questions & Cheat Sheet](#part-e--rapid-fire-questions--cheat-sheet)

### Setup used in the snippets

```bash
pip install -U langchain langchain-core langchain-openai langchain-text-splitters langchain-classic langsmith
```

```python
# common.py — shared imports for snippets in this guide
from typing import Literal
from operator import itemgetter
from pydantic import BaseModel, Field
from langchain.chat_models import init_chat_model
from langchain_core.prompts import ChatPromptTemplate, MessagesPlaceholder
from langchain_core.output_parsers import StrOutputParser
from langchain_core.runnables import (RunnableParallel, RunnablePassthrough, RunnableLambda,
                                      RunnableBranch, RunnableConfig, chain)
from langchain.messages import HumanMessage, AIMessage, SystemMessage, ToolMessage

llm = init_chat_model("openai:gpt-4.1-mini", temperature=0)   # any "provider:model" string
```

---

# Part A — Core Concepts (Q1–Q28)

## Q1. What is LangChain, and how is the ecosystem organised in 1.x?

LangChain is a framework for building LLM applications. It provides **standard interfaces** to models, tools, retrievers and vector stores, plus **composition primitives** (Runnables / LCEL) and a **prebuilt agent** (`create_agent`).

| Package | Contents |
|---|---|
| `langchain-core` | Base abstractions: Runnables/LCEL, messages, prompts, output parsers, tools, callbacks, retrievers, vector store interface, caches, rate limiters |
| `langchain` | High-level APIs: `create_agent`, middleware, `init_chat_model`, `init_embeddings`, structured output strategies |
| `langchain-<provider>` | Standalone integrations (`langchain-openai`, `langchain-anthropic`, `langchain-google-genai`, `langchain-postgres`, `langchain-qdrant`, …) |
| `langchain-text-splitters` | Chunking utilities |
| `langchain-classic` | Legacy chains, retrievers, memory classes, indexing helpers kept for backward compatibility |
| `langchain-community` | Older community integrations (being sunset) |
| `langgraph` | Low-level orchestration runtime that `create_agent` is built on |
| `langsmith` | Tracing, evaluation, prompt management, monitoring |

**Interview angle:**
- LangChain is the **component and agent layer**.
- LangGraph is the **orchestration runtime**.
- LangSmith is the **observability and evaluation layer**.

---

## Q2. LangChain vs LangGraph: when do you use which?

| Need | Use |
|---|---|
| Linear prompt → model → parser pipelines, RAG chains, extraction, classification | **LCEL chains** (LangChain core) |
| A standard tool-calling agent with customisation (summarisation, HITL, PII, retries, routing) | **`create_agent` + middleware** |
| Custom control flow: loops, branches, parallel workers, multi-agent graphs, durable long-running workflows | **LangGraph** directly |

Because `create_agent` returns a compiled LangGraph graph, agents automatically get checkpointing, streaming, interrupts and LangSmith tracing.

---

## Q3. How do you initialise chat models in a provider-agnostic way?

`init_chat_model` creates a chat model from a `"provider:model"` string, so swapping providers doesn't change your code.

```python
gpt = init_chat_model("openai:gpt-4.1-mini", temperature=0, max_tokens=800, timeout=30, max_retries=2)
claude = init_chat_model("anthropic:claude-sonnet-4-5", temperature=0)
local = init_chat_model("ollama:llama3.1", base_url="http://localhost:11434")

# Runtime-configurable model: choose provider/model per request via config
configurable = init_chat_model(configurable_fields=("model", "model_provider", "temperature"),
                               config_prefix="llm")
configurable.invoke("Hi", config={"configurable": {"llm_model": "anthropic:claude-sonnet-4-5"}})
```

Common parameters:
- `temperature`, `max_tokens`, `timeout`, `max_retries`,
- `rate_limiter`, `base_url`, `api_key`,
- provider-specific `model_kwargs`.

---

## Q4. What message types exist, and what are *content blocks*?

| Message | Role |
|---|---|
| `SystemMessage` | Instructions |
| `HumanMessage` | User input (text, images, files, audio) |
| `AIMessage` | Model output: `content`, `tool_calls`, `usage_metadata`, `response_metadata` |
| `ToolMessage` | Tool result, linked by `tool_call_id` |
| `AIMessageChunk` | Streaming piece; chunks can be added with `+` |
| `RemoveMessage` | Deletes a message from agent/graph state |

**Content blocks (1.x):** `message.content_blocks` gives a **provider-neutral, typed view** of content:
- `text`,
- `reasoning`,
- `image` / `file` / `audio`,
- `tool_call`,
- `server_tool_call` / `server_tool_result` (e.g. provider web search),
- `citation`.

The same code then works across OpenAI, Anthropic and Gemini responses.

```python
msg = HumanMessage(content=[
    {"type": "text", "text": "What's in this invoice? Extract the total."},
    {"type": "image", "url": "https://example.com/invoice.png"},
])
resp = llm.invoke([msg])
for block in resp.content_blocks:
    if block["type"] == "reasoning":
        print("thinking:", block.get("reasoning", "")[:200])
    elif block["type"] == "text":
        print("answer:", block["text"])
print(resp.usage_metadata)          # {'input_tokens': ..., 'output_tokens': ..., 'total_tokens': ...}
```

---

## Q5. What are prompt templates, and how do `MessagesPlaceholder` and partials work?

```python
prompt = ChatPromptTemplate.from_messages([
    ("system", "You are a {persona}. Answer in {language}. Today is {today}."),
    MessagesPlaceholder("history", optional=True),          # inject prior chat turns
    ("human", "{question}"),
])
prompt = prompt.partial(today=lambda: __import__("datetime").date.today().isoformat())   # partial: fill later/lazily
messages = prompt.invoke({"persona": "tax advisor", "language": "English",
                          "history": [HumanMessage("hi"), AIMessage("hello")],
                          "question": "Is HRA taxable?"})
```

Key points:
- Templates are **Runnables**, so they compose with `|`.
- **Few-shot**: use `FewShotChatMessagePromptTemplate` with a fixed list, or an **example selector** (semantic similarity) to choose relevant examples per input.
- **Prompt versioning**: store prompts in LangSmith Prompt Hub or your own registry and pull a pinned version at startup (see Q58).

---

## Q6. What is LCEL, and what does the pipe operator actually do?

**LangChain Expression Language (LCEL)** composes Runnables declaratively. `a | b` creates a `RunnableSequence`, where the output of `a` becomes the input of `b`.

Every composed chain automatically supports:
- `invoke` / `ainvoke`,
- `batch` / `abatch` (parallel over inputs, with `max_concurrency`),
- `stream` / `astream` (streams through the chain where components support it),
- `astream_events` (fine-grained event stream),
- callbacks and tracing, `with_retry`, `with_fallbacks`, `with_config`.

```python
summarize = (
    ChatPromptTemplate.from_template("Summarise in 3 bullets:\n{text}")
    | llm
    | StrOutputParser()
)
summarize.invoke({"text": "..."})
summarize.batch([{"text": t} for t in texts], config={"max_concurrency": 5})
for chunk in summarize.stream({"text": "..."}):
    print(chunk, end="")
```

| LCEL pros | LCEL cons |
|---|---|
| Concise, streaming/batching/async for free, automatic tracing | Hard to debug when deeply nested; no loops or state; complex branching gets unreadable, so use LangGraph for that |

---

## Q7. Explain the core Runnable primitives: Sequence, Parallel, Passthrough, `assign`, Lambda, Branch, and `@chain`.

| Primitive | What it does | Example use |
|---|---|---|
| `RunnableSequence` (`a \| b`) | Pipe output to the next step | prompt → model → parser |
| `RunnableParallel` (or a dict literal) | Run branches on the same input concurrently and return a dict | Retrieve context **and** pass the question through |
| `RunnablePassthrough()` | Pass input unchanged | Keep the original question |
| `RunnablePassthrough.assign(k=fn)` | Add new keys to the input dict | Add `context` while keeping the other keys |
| `RunnableLambda(fn)` | Wrap any Python function | Formatting, validation |
| `RunnableBranch((cond, runnable), ..., default)` | if/elif/else routing | Route by classification |
| `@chain` decorator | Turn a function into a Runnable that can call other runnables (traced as one unit) | Custom multi-step logic |
| `itemgetter("key")` | Pick a key from a dict input | Common in RAG |

```python
def format_docs(docs): return "\n\n".join(f"[{i}] {d.page_content}" for i, d in enumerate(docs, 1))

rag = (
    RunnablePassthrough.assign(context=itemgetter("question") | retriever | format_docs)
    | ChatPromptTemplate.from_template("Answer from context with [n] citations.\n{context}\nQ: {question}")
    | llm | StrOutputParser()
)

parallel = RunnableParallel(
    summary=summarize,
    sentiment=ChatPromptTemplate.from_template("Sentiment of: {text}. One word.") | llm | StrOutputParser(),
    keywords=ChatPromptTemplate.from_template("5 keywords for: {text}") | llm | StrOutputParser(),
)
parallel.invoke({"text": review})      # all three LLM calls run concurrently

@chain
def answer_with_check(inputs: dict) -> str:
    draft = rag.invoke(inputs)
    if "[1]" not in draft:                          # custom logic inside a traced runnable
        return "I couldn't find a sourced answer."
    return draft
```

---

## Q8. What are output parsers, and when do you use structured output instead?

| Approach | How | Reliability | When |
|---|---|---|---|
| `StrOutputParser` | Extracts text | — | Free text |
| `JsonOutputParser` / `PydanticOutputParser` | Format instructions in the prompt + parsing | Medium (fences, extra prose, renamed keys) | Models without native structured output |
| `OutputFixingParser` / `RetryWithErrorOutputParser` (`langchain-classic`) | Re-ask an LLM to fix invalid output | Better, costs extra calls | Legacy pipelines |
| **`model.with_structured_output(Schema)`** | Uses the provider's native JSON-schema mode or tool calling | **High** | Default choice for extraction and classification |
| Agent `response_format` (`ToolStrategy` / `ProviderStrategy`) | Structured final answer from an agent | High, with error handling and retry | Agents that must return typed results |

```python
class Ticket(BaseModel):
    category: Literal["billing", "technical", "returns", "other"]
    priority: Literal["low", "medium", "high"]
    summary: str = Field(description="one sentence")
    order_id: str | None = None

extractor = llm.with_structured_output(Ticket)                 # method="json_schema" / "function_calling"
t = extractor.invoke("My order 1766 was charged twice!!! Fix it now")
print(t.category, t.priority, t.order_id)

# include_raw=True returns {"raw": AIMessage, "parsed": Ticket | None, "parsing_error": ...}
safe_extractor = llm.with_structured_output(Ticket, include_raw=True)
```

---

## Q9. How do you define tools, and what makes a good tool?

```python
from langchain.tools import tool, ToolRuntime, ToolException

class RefundArgs(BaseModel):
    order_id: str = Field(description="Order ID like 1766")
    amount: float = Field(gt=0, le=50_000, description="Refund amount in INR")

@tool("issue_refund", args_schema=RefundArgs)
def issue_refund(order_id: str, amount: float) -> str:
    """Issue a refund for an order. Use only after confirming eligibility."""
    if not orders_db.exists(order_id):
        raise ToolException(f"Order {order_id} not found")     # surfaced to the model as an error message
    return payments.refund(order_id, amount)

@tool
def get_my_orders(limit: int, runtime: ToolRuntime) -> list[dict]:
    """List the current user's recent orders."""
    user_id = runtime.context.user_id        # injected, hidden from the model's schema
    runtime.stream_writer({"status": f"Fetching {limit} orders"})
    return orders_db.recent(user_id, limit)
```

**Good tool design:**
- a clear name and a docstring the **model** reads,
- typed args with descriptions and constraints,
- small, single-purpose tools,
- idempotent where possible,
- concise outputs (return IDs and summaries, not 50 KB of JSON),
- errors returned as **actionable** messages,
- permissions enforced **inside** the tool, never trusting the model.

`ToolRuntime` gives a tool access to `state`, `context`, `config`, `store`, `stream_writer` and `tool_call_id` without exposing them in the schema the model sees. Older code used `InjectedState`, `InjectedStore` and `InjectedToolArg` for the same purpose.

---

## Q10. How does tool calling work at the model level (`bind_tools`)?

1. `llm.bind_tools([tools])` sends the tool JSON schemas to the provider.
2. The model returns an `AIMessage` with `tool_calls=[{"name", "args", "id"}]` instead of (or alongside) text.
3. **Your code** executes the tools and sends back `ToolMessage(content, tool_call_id)`.
4. The model produces the final answer, or more tool calls.

```python
model_with_tools = llm.bind_tools([issue_refund], tool_choice="auto", parallel_tool_calls=False)
ai = model_with_tools.invoke("Refund 500 rupees on order 1766")
messages = [HumanMessage("Refund 500 rupees on order 1766"), ai]
for tc in ai.tool_calls:
    result = {"issue_refund": issue_refund}[tc["name"]].invoke(tc["args"])
    messages.append(ToolMessage(content=str(result), tool_call_id=tc["id"]))
final = model_with_tools.invoke(messages)
```

`tool_choice` can be `"auto"`, `"any"`/`"required"`, a specific tool name, or `"none"`. An agent automates this loop (Q11).

---

## Q11. What is `create_agent`, and what are its main parameters?

`create_agent` builds a production-ready tool-calling agent loop (model → tools → model … → final answer) as a **compiled LangGraph graph**.

```python
from langchain.agents import create_agent
from langgraph.checkpoint.memory import InMemorySaver
from dataclasses import dataclass

@dataclass
class Ctx:
    user_id: str
    tier: str = "free"

agent = create_agent(
    model="openai:gpt-4.1-mini",                  # or a BaseChatModel
    tools=[get_my_orders, issue_refund],
    system_prompt="You are Acme's support agent. Be concise. Never promise refunds > ₹50k.",
    middleware=[],                                 # see Q12–Q13
    response_format=None,                          # see Q14
    context_schema=Ctx,                            # run-scoped dependencies
    checkpointer=InMemorySaver(),                  # short-term memory per thread
    store=None,                                    # long-term memory
    name="support_agent",
)
result = agent.invoke(
    {"messages": [{"role": "user", "content": "Where are my last 2 orders?"}]},
    config={"configurable": {"thread_id": "u42-s1"}},
    context=Ctx(user_id="u42"),
)
print(result["messages"][-1].content)
```

| Parameter | Purpose |
|---|---|
| `model` | String id or model instance (static), or chosen dynamically via middleware |
| `tools` | Tools, functions or provider tool dicts |
| `system_prompt` | Static prompt (dynamic via `@dynamic_prompt`) |
| `middleware` | Hooks to customise behaviour |
| `response_format` | Structured final output |
| `state_schema` / `context_schema` | Extend agent state; typed runtime context |
| `checkpointer` / `store` | Memory |
| `interrupt_before/after` | Static breakpoints |
| `cache` | Node caching |

This replaces the deprecated `langgraph.prebuilt.create_react_agent` and the legacy `AgentExecutor` / `initialize_agent`.

---

## Q12. What is agent middleware, and what hooks exist?

Middleware intercepts the agent loop at defined points. It is the main way to do **context engineering** (control exactly what the model sees) and add cross-cutting behaviour.

| Hook | When | Typical use |
|---|---|---|
| `before_agent` / `after_agent` | Once per invocation | Load user profile, final audit log |
| `before_model` | Before each model call | Trim or summarise messages, inject memories, enforce limits |
| `after_model` | After each model call | Validate output, guardrails, stop conditions |
| `wrap_model_call` | **Around** the model call (handler-style) | Dynamic model selection, retries, fallbacks, caching, modifying tools per request |
| `wrap_tool_call` | Around each tool call | Tool retries, error handling, auth checks, timing |
| `dynamic_prompt` | Builds the system prompt per call | Role- or tenant-specific prompts |

```python
from langchain.agents.middleware import (before_model, after_model, wrap_model_call, wrap_tool_call,
                                         dynamic_prompt, AgentState, ModelRequest, ModelResponse)
from langgraph.runtime import Runtime

@dynamic_prompt
def personalised_prompt(request: ModelRequest) -> str:
    tier = request.runtime.context.tier
    return f"You are Acme support. Customer tier: {tier}. {'Offer priority callback.' if tier == 'gold' else ''}"

@wrap_model_call
def pick_model(request: ModelRequest, handler) -> ModelResponse:
    big = len(request.messages) > 12 or request.runtime.context.tier == "gold"
    model = init_chat_model("openai:gpt-4.1" if big else "openai:gpt-4.1-mini")
    return handler(request.override(model=model))              # immutable override, then continue

@wrap_tool_call
def audit_tools(request, handler):
    import time
    t = time.perf_counter()
    result = handler(request)
    log.info("tool", extra={"tool": request.tool_call["name"], "ms": (time.perf_counter() - t) * 1000})
    return result

@after_model
def block_promises(state: AgentState, runtime: Runtime) -> dict | None:
    last = state["messages"][-1]
    if "guarantee" in str(last.content).lower():
        return {"messages": [AIMessage(id=last.id, content="I can't guarantee outcomes, but here's what I can do…")]}
    return None

agent = create_agent("openai:gpt-4.1-mini", tools=[get_my_orders],
                     middleware=[personalised_prompt, pick_model, audit_tools, block_promises],
                     context_schema=Ctx)
```

**Ordering:**
- `before_*` hooks run in list order.
- `after_*` hooks run in reverse order.
- `wrap_*` hooks nest like decorators (the first in the list is the outermost).

For class-based middleware, subclass `AgentMiddleware` and implement the hook methods. This also lets you add **custom state fields** and **extra tools**.

---

## Q13. Which built-in middleware does LangChain provide, and what problem does each solve?

| Middleware | Solves | Key config |
|---|---|---|
| `SummarizationMiddleware` | Context-window overflow in long conversations | `model`, `trigger=("tokens", 4000)` or `("messages", 50)` or `("fraction", 0.8)`, `keep=("messages", 20)` |
| `HumanInTheLoopMiddleware` | Approval before sensitive tool calls | `interrupt_on={"issue_refund": True, "read_x": False}` (plus allowed decisions per tool via `InterruptOnConfig`) |
| `PIIMiddleware` | Detect PII and block, redact, mask or hash it | `pii_type="email"` / `"credit_card"` / custom detector; `strategy`; `apply_to_input/output/tool_results` |
| `ModelCallLimitMiddleware` | Runaway agents and cost | `run_limit`, `thread_limit`, `exit_behavior="end"\|"error"` |
| `ToolCallLimitMiddleware` | Too many calls to a (specific) tool | `tool_name`, `run_limit`, `thread_limit` |
| `ModelFallbackMiddleware` | Provider outages | `ModelFallbackMiddleware("openai:gpt-4.1-mini", "anthropic:claude-...")` |
| `ModelRetryMiddleware` / `ToolRetryMiddleware` | Transient failures | `max_retries`, `backoff_factor`, `retry_on` |
| `ToolErrorMiddleware` | Convert tool exceptions into messages the model can act on | `on_error` |
| `LLMToolSelectorMiddleware` | Agents with many tools (cost and accuracy) | A small model pre-selects the relevant tools; `max_tools`, `always_include` |
| `ContextEditingMiddleware` | Clear old, bulky tool outputs from context | `edits=[ClearToolUsesEdit(...)]` |
| `TodoListMiddleware` | Planning for multi-step tasks | Gives the agent a `write_todos` tool |
| `ModelRouterMiddleware` (newer) | Route each request to a model via a classifier | `choices`, `instructions`, `classifier` |
| `LLMToolEmulator` | Testing with emulated tools | — |
| `ShellToolMiddleware`, `FilesystemFileSearchMiddleware` | Coding and file agents | Execution policy (host, Docker) |

```python
from langchain.agents.middleware import (SummarizationMiddleware, HumanInTheLoopMiddleware, PIIMiddleware,
                                         ModelCallLimitMiddleware, ModelFallbackMiddleware, ToolRetryMiddleware)

agent = create_agent(
    "openai:gpt-4.1-mini",
    tools=[get_my_orders, issue_refund],
    checkpointer=InMemorySaver(),
    middleware=[
        PIIMiddleware("credit_card", strategy="mask", apply_to_input=True, apply_to_output=True),
        PIIMiddleware("email", strategy="redact", apply_to_tool_results=True),
        SummarizationMiddleware(model="openai:gpt-4.1-mini", trigger=("tokens", 6000), keep=("messages", 12)),
        ModelCallLimitMiddleware(run_limit=10, exit_behavior="end"),
        ToolRetryMiddleware(max_retries=2, tools=["get_my_orders"]),
        ModelFallbackMiddleware("anthropic:claude-sonnet-4-5"),
        HumanInTheLoopMiddleware(interrupt_on={"issue_refund": True}),
    ],
)
```

---

## Q14. How do you get structured output from an agent (`response_format`)? `ToolStrategy` vs `ProviderStrategy`.

| Strategy | How | Pros | Cons |
|---|---|---|---|
| `ProviderStrategy(Schema, strict=True)` | Uses the provider's native structured output (JSON schema) | Most reliable, no extra tool | Only for providers/models that support it |
| `ToolStrategy(Schema, handle_errors=True)` | Exposes the schema as a special "respond" tool | Works with any tool-calling model; retries with error feedback on validation failure | Slight prompt overhead |
| Pass the schema directly (`response_format=Schema`) | LangChain picks a strategy automatically | Simple | Less explicit |

```python
from langchain.agents.structured_output import ToolStrategy

class Resolution(BaseModel):
    status: Literal["resolved", "escalated", "needs_info"]
    actions_taken: list[str]
    customer_reply: str

agent = create_agent("openai:gpt-4.1-mini", tools=[get_my_orders, issue_refund],
                     response_format=ToolStrategy(Resolution, handle_errors=True), context_schema=Ctx)
out = agent.invoke({"messages": [("user", "Refund my duplicate charge on 1766")]}, context=Ctx("u42"))
res: Resolution = out["structured_response"]
```

---

## Q15. How does memory work in LangChain 1.x (short-term vs long-term)? What happened to `ConversationBufferMemory`?

- **Short-term (conversation) memory** = the agent's `messages` state persisted by a **LangGraph checkpointer**, keyed by `thread_id`.
- **Long-term memory** = a LangGraph **Store** (namespaced JSON documents with optional semantic search), accessed from tools or middleware.
- **Context-window management** = `SummarizationMiddleware`, `trim_messages`, `ContextEditingMiddleware`.
- Legacy `ConversationBufferMemory`, `ConversationSummaryMemory` and `ConversationChain` live in `langchain-classic`. For LCEL chains, `RunnableWithMessageHistory` still exists in core.

```python
from langgraph.checkpoint.postgres import PostgresSaver
from langgraph.store.postgres import PostgresStore

@tool
def save_preference(pref: str, runtime: ToolRuntime) -> str:
    """Save a durable user preference (e.g. 'prefers WhatsApp updates')."""
    runtime.store.put(("prefs", runtime.context.user_id), pref[:40], {"pref": pref})
    return "saved"

@tool
def recall_preferences(runtime: ToolRuntime) -> list[str]:
    """Recall stored user preferences."""
    return [i.value["pref"] for i in runtime.store.search(("prefs", runtime.context.user_id))]

with PostgresSaver.from_conn_string(DB_URI) as cp, PostgresStore.from_conn_string(DB_URI) as store:
    cp.setup(); store.setup()
    agent = create_agent("openai:gpt-4.1-mini", tools=[save_preference, recall_preferences],
                         checkpointer=cp, store=store, context_schema=Ctx)
```

**LCEL chain with history** (no agent):

```python
from langchain_core.runnables.history import RunnableWithMessageHistory
from langchain_core.chat_history import InMemoryChatMessageHistory

store_ = {}
def get_history(session_id): return store_.setdefault(session_id, InMemoryChatMessageHistory())

chat_prompt = ChatPromptTemplate.from_messages([("system", "Be helpful."), MessagesPlaceholder("history"),
                                                ("human", "{input}")])
chat = RunnableWithMessageHistory(chat_prompt | llm, get_history,
                                  input_messages_key="input", history_messages_key="history")
chat.invoke({"input": "I'm Priya"}, config={"configurable": {"session_id": "s1"}})
```

---

## Q16. What are document loaders, and what metadata should you keep?

Loaders turn sources into `Document(page_content, metadata)`:
- PDFs (PyPDF, PyMuPDF, Unstructured, Docling),
- web pages,
- Notion, Confluence, Google Drive, S3, SQL,
- CSV, JSON.

Use `lazy_load()` for large corpora so you stream documents instead of loading them all into memory.

**Metadata to keep:**
- `source` / URL,
- page number,
- section/heading path,
- `doc_id`, version and `last_modified`,
- **ACL groups**, tenant,
- language,
- document type.

Metadata drives filtering, citations, freshness and security later.

```python
from langchain_community.document_loaders import PyPDFLoader       # or a standalone loader package
for doc in PyPDFLoader("policy_2025.pdf").lazy_load():
    doc.metadata.update({"doc_id": "refund-policy", "version": "2025", "acl": ["all_staff"]})
```

---

## Q17. What text-splitting strategies exist, and how do you choose?

| Splitter | Best for | Trade-off |
|---|---|---|
| `RecursiveCharacterTextSplitter` | General default; tries paragraphs → lines → sentences → words | Size-based; may cut semantic units |
| `.from_tiktoken_encoder` / `TokenTextSplitter` | Exact token budgets | Ignores structure |
| `MarkdownHeaderTextSplitter` / `HTMLHeaderTextSplitter` | Structured docs; keeps headers as metadata | Needs clean structure |
| Language-aware splitter (`RecursiveCharacterTextSplitter.from_language(Language.PYTHON)`) | Code | — |
| `SemanticChunker` (langchain-experimental) | Unstructured prose with topic shifts | Embedding cost at ingestion |
| Parent-document strategy (small chunks for search, big for context) | Precise search with full context | Two stores |

```python
from langchain_text_splitters import RecursiveCharacterTextSplitter, MarkdownHeaderTextSplitter

md_splitter = MarkdownHeaderTextSplitter([("#", "h1"), ("##", "h2"), ("###", "h3")])
sections = md_splitter.split_text(markdown_text)           # each section carries header metadata
splitter = RecursiveCharacterTextSplitter.from_tiktoken_encoder(chunk_size=500, chunk_overlap=60)
chunks = splitter.split_documents(sections)
for c in chunks:                                            # contextual header improves retrieval
    c.page_content = f"{c.metadata.get('h1','')} > {c.metadata.get('h2','')}\n{c.page_content}"
```

---

## Q18. Embeddings: how do you choose them, and how do you avoid re-embedding costs?

- Use `init_embeddings("openai:text-embedding-3-small")` or provider classes.
- Choose based on:
  - domain and language (multilingual models for Indic languages),
  - dimension (storage and speed),
  - max input length,
  - cost,
  - whether it can be self-hosted.
- **Cache** embeddings with `CacheBackedEmbeddings` (`langchain-classic`). The key is the hash of the text plus a namespace (the model), so unchanged chunks aren't re-embedded.

```python
from langchain.embeddings import init_embeddings
from langchain_classic.embeddings import CacheBackedEmbeddings
from langchain_classic.storage import LocalFileStore

base = init_embeddings("openai:text-embedding-3-small")
embeddings = CacheBackedEmbeddings.from_bytes_store(
    base, LocalFileStore("./emb_cache"), namespace="text-embedding-3-small",
    query_embedding_cache=True)      # also cache query embeddings
```

---

## Q19. What is a VectorStore vs a Retriever? Which retrieval strategies does LangChain offer?

- **VectorStore:** stores embeddings and supports `add_documents`, `similarity_search`, `max_marginal_relevance_search`, `delete`, and metadata filters.
- **Retriever:** a Runnable `str → list[Document]`. `vectorstore.as_retriever(...)` is one kind, but retrievers can wrap anything (BM25, SQL, web search).

| Retriever / technique | Fixes | Location |
|---|---|---|
| `as_retriever(search_type="similarity", search_kwargs={"k": 5, "filter": {...}})` | Baseline | core / vector store package |
| `search_type="mmr"` (`fetch_k`, `lambda_mult`) | Redundant near-duplicate chunks | — |
| `search_type="similarity_score_threshold"` | Junk results for off-topic queries | — |
| `BM25Retriever` | Exact keywords, IDs, codes | `langchain-community` |
| `EnsembleRetriever` (RRF fusion of BM25 + dense) | Hybrid search | `langchain-classic` |
| `MultiQueryRetriever` | Short or ambiguous queries (LLM generates variants) | `langchain-classic` |
| `ParentDocumentRetriever` | Small chunks match well but lack context | `langchain-classic` |
| `ContextualCompressionRetriever` + re-ranker / `EmbeddingsFilter` | Precision; fewer tokens | `langchain-classic` |
| Self-query retriever | Natural-language filters ("policies after 2023 for EU") | `langchain-classic` |
| Native hybrid in vector DBs (Qdrant, Weaviate, PGVector, Elasticsearch) | Hybrid search at scale | integration packages |

```python
from langchain_core.vectorstores import InMemoryVectorStore
from langchain_community.retrievers import BM25Retriever
from langchain_classic.retrievers import EnsembleRetriever, MultiQueryRetriever

vs = InMemoryVectorStore.from_documents(chunks, embeddings)
dense = vs.as_retriever(search_type="mmr", search_kwargs={"k": 6, "fetch_k": 30, "lambda_mult": 0.6})
bm25 = BM25Retriever.from_documents(chunks); bm25.k = 6
hybrid = EnsembleRetriever(retrievers=[bm25, dense], weights=[0.4, 0.6])
multi = MultiQueryRetriever.from_llm(retriever=hybrid, llm=llm)
docs = multi.invoke("limit?")
```

---

## Q20. How do you build a custom retriever?

Subclass `BaseRetriever` and implement `_get_relevant_documents` (plus `_aget_relevant_documents` for async). For example, you might wrap an internal search API or enforce ACLs.

```python
from langchain_core.retrievers import BaseRetriever
from langchain_core.documents import Document
from langchain_core.callbacks import CallbackManagerForRetrieverRun

class ACLRetriever(BaseRetriever):
    base: BaseRetriever
    user_groups: list[str]

    def _get_relevant_documents(self, query: str, *, run_manager: CallbackManagerForRetrieverRun) -> list[Document]:
        docs = self.base.invoke(query, config={"callbacks": run_manager.get_child()})
        return [d for d in docs if set(d.metadata.get("acl", [])) & set(self.user_groups)]
```

Prefer pushing the ACL filter **into the vector DB query** when the store supports it. Post-filtering can return fewer than k results.

---

## Q21. How do retries, fallbacks and timeouts work on Runnables?

```python
from langchain_core.rate_limiters import InMemoryRateLimiter

limiter = InMemoryRateLimiter(requests_per_second=4, check_every_n_seconds=0.1, max_bucket_size=8)
primary = init_chat_model("openai:gpt-4.1-mini", timeout=20, max_retries=0, rate_limiter=limiter)
backup = init_chat_model("anthropic:claude-haiku-4-5", timeout=20)

robust_llm = (primary
              .with_retry(stop_after_attempt=3, wait_exponential_jitter=True,
                          retry_if_exception_type=(TimeoutError, ConnectionError))
              .with_fallbacks([backup]))

# Fallbacks work on whole chains too, e.g. a different prompt tuned for the backup model:
chain_a = prompt_a | primary | StrOutputParser()
chain_b = prompt_b | backup | StrOutputParser()
resilient = chain_a.with_fallbacks([chain_b], exceptions_to_handle=(Exception,))
```

**Trade-offs:**
- Don't retry non-retryable errors such as a 400 or context-length error (they burn money). Filter with `retry_if_exception_type`.
- Fallback models need **their own prompt evals**.
- For agents, use `ModelRetryMiddleware` / `ModelFallbackMiddleware`.

---

## Q22. How does LLM response caching work?

```python
from langchain_core.globals import set_llm_cache
from langchain_core.caches import InMemoryCache
from langchain_community.cache import SQLiteCache          # Redis/semantic caches come from integration packages

set_llm_cache(SQLiteCache(database_path=".langchain.db"))  # global, exact-match on (prompt, model params)
llm_nocache = init_chat_model("openai:gpt-4.1-mini", cache=False)   # opt out per model
```

| Cache type | Pros | Cons |
|---|---|---|
| In-memory / SQLite exact cache | Simple; identical prompts return instantly | No hits for paraphrases; not shared across replicas (SQLite) |
| Redis exact cache | Shared across instances | Infra |
| Semantic cache (e.g. Redis vector) | Hits on paraphrases | False-hit risk; needs a similarity threshold and scoping by tenant |
| Provider prompt caching | Cheaper and faster long prefixes | Provider-specific; prefix must be stable |

---

## Q23. What is `RunnableConfig`, and what can you pass in it?

| Key | Purpose |
|---|---|
| `callbacks` | Handlers for events (tracing, metrics) |
| `tags`, `metadata` | Searchable labels propagated to child runs |
| `run_name`, `run_id` | Name and ID of the top-level run (link feedback) |
| `max_concurrency` | Parallelism cap for `batch` and parallel steps |
| `recursion_limit` | Max steps (agents/graphs) |
| `configurable` | Runtime-configurable fields and alternatives; `thread_id` for agents |

```python
cfg: RunnableConfig = {"run_name": "ticket_extractor", "tags": ["prod", "extract"],
                       "metadata": {"tenant": "acme", "prompt_version": "v7"}, "max_concurrency": 8}
extractor.batch(tickets, config=cfg)
traced = extractor.with_config(run_name="ticket_extractor", tags=["extract"])   # bake defaults in
```

---

## Q24. What are `configurable_fields` and `configurable_alternatives`?

They let you change a chain's **parameters or components at runtime** without rebuilding it. This is useful for A/B tests, per-tenant models and premium tiers.

```python
from langchain_core.runnables import ConfigurableField

model = init_chat_model("openai:gpt-4.1-mini").configurable_fields(
    temperature=ConfigurableField(id="temperature", name="LLM temperature")
).configurable_alternatives(
    ConfigurableField(id="llm"), default_key="mini",
    big=init_chat_model("openai:gpt-4.1"),
    claude=init_chat_model("anthropic:claude-sonnet-4-5"),
)
qa = ChatPromptTemplate.from_template("{q}") | model | StrOutputParser()
qa.invoke({"q": "Explain SIP"}, config={"configurable": {"llm": "claude", "temperature": 0.3}})
```

---

## Q25. What streaming options exist, and how do they differ?

| API | Emits | Use |
|---|---|---|
| `chain.stream()` / `astream()` | Output chunks of the final step (tokens for model → `StrOutputParser`) | Simple token streaming |
| `astream_events(version="v2")` | Typed events from **every** component: `on_chat_model_stream`, `on_retriever_end`, `on_tool_start/end`, `on_chain_start/end`, custom events | Rich UIs (show sources, tool progress), debugging |
| Agent `stream(..., stream_mode=["updates", "messages", "custom"])` | LangGraph stream modes | Agent step progress and tokens |
| `adispatch_custom_event` / `ToolRuntime.stream_writer` | Your own progress events | Long tools |

```python
async for ev in rag_chain.astream_events({"question": "refund window?"}, version="v2"):
    kind = ev["event"]
    if kind == "on_retriever_end":
        print("sources:", [d.metadata["source"] for d in ev["data"]["output"]])
    elif kind == "on_chat_model_stream":
        print(ev["data"]["chunk"].content, end="", flush=True)

for mode, chunk in agent.stream({"messages": [("user", "Where's order 1766?")]},
                                stream_mode=["updates", "messages"], context=Ctx("u42")):
    ...
```

**Gotcha:** if a step in the chain doesn't support streaming (e.g. a `RunnableLambda` that consumes its whole input), streaming of the final output starts only after that step finishes. Write generator functions to keep streaming end-to-end.

---

## Q26. How do you use MCP (Model Context Protocol) tools in LangChain?

The `langchain-mcp-adapters` package converts MCP server tools into LangChain tools, so agents can use any MCP server (filesystem, GitHub, databases, internal services).

```python
from langchain_mcp_adapters.client import MultiServerMCPClient

client = MultiServerMCPClient({
    "math": {"command": "python", "args": ["./math_server.py"], "transport": "stdio"},
    "crm": {"url": "https://mcp.internal.example.com/mcp", "transport": "streamable_http",
            "headers": {"Authorization": f"Bearer {token}"}},
})
tools = await client.get_tools()
agent = create_agent("openai:gpt-4.1-mini", tools=tools)
```

Trade-offs:
- standard, reusable tools across apps and teams,
- but there is an extra network hop, and you must **trust and authorise** each server.
- Tool descriptions from servers can carry prompt injection, so review them, and restrict tools per user.

---

## Q27. What are the legacy chains, and how do you migrate them?

| Legacy (now in `langchain-classic`) | Modern replacement |
|---|---|
| `LLMChain` | `prompt \| llm \| parser` (LCEL) |
| `RetrievalQA`, `ConversationalRetrievalChain` | LCEL RAG chain, or an agent with a retriever tool |
| `create_retrieval_chain`, `create_history_aware_retriever`, `create_stuff_documents_chain` | Still usable via `langchain_classic.chains`; or hand-written LCEL |
| `MapReduceDocumentsChain`, `RefineDocumentsChain` | LCEL `batch` + combine, or LangGraph map-reduce with `Send` |
| `ConversationChain` + `ConversationBufferMemory` | Agent + checkpointer, or `RunnableWithMessageHistory` |
| `AgentExecutor`, `initialize_agent`, `create_react_agent` | `create_agent` + middleware |

Migration strategy:
1. Pin versions and run your eval set as a baseline.
2. Install `langchain-classic` for an immediate fix (just change the imports).
3. Rewrite the hot paths to LCEL or `create_agent`, feature by feature, and compare evals.

---

## Q28. How do multimodal inputs work (images, PDFs, audio)?

Pass standard content blocks in a `HumanMessage`. LangChain translates them to each provider's format.

```python
import base64
pdf_b64 = base64.b64encode(open("invoice.pdf", "rb").read()).decode()
msg = HumanMessage(content=[
    {"type": "text", "text": "Extract invoice number and total."},
    {"type": "file", "base64": pdf_b64, "mime_type": "application/pdf"},
])

class Invoice(BaseModel):
    invoice_number: str
    total: float

invoice = llm.with_structured_output(Invoice).invoke([msg])
```

Check model support per modality and the size limits. For large documents, a parse-then-RAG pipeline is cheaper than sending full files every time.

---

# Part B — Pipeline Patterns & Use Cases (Q29–Q47)

> Each question gives you a use case and asks you to design the pipeline. Every answer follows the same layout: the **use case**, the **pipeline shape**, **why it fits**, the **trade-offs**, and **code**.

### Pipeline selection map

| Pipeline | Built with | Typical use case |
|---|---|---|
| Simple LLM chain | LCEL | Rewrite, translate, classify |
| Summarisation (stuff / map-reduce / refine) | LCEL `batch` | Meeting notes, long reports |
| Structured extraction | `with_structured_output` + `batch` | Invoices, tickets, resumes |
| Classification + routing | `RunnableBranch` / router function | Support triage, intent routing |
| Parallel enrichment | `RunnableParallel` | Review analytics (sentiment + topics + summary) |
| Basic RAG with citations | LCEL + retriever | Policy Q&A |
| Conversational RAG | History-aware rewrite + RAG | Chat over docs |
| Hybrid / advanced retrieval RAG | Ensemble + rerank + compression | Technical docs with IDs and codes |
| Ingestion / indexing | Loaders + splitters + `index()` with a RecordManager | Keeping a vector DB in sync |
| Tool-calling agent | `create_agent` | Customer support |
| SQL Q&A | Agent + SQL tools + validation | BI chatbot |
| Agentic RAG | Agent with retriever tool(s) | Multi-source enterprise Q&A |
| HITL agent | `HumanInTheLoopMiddleware` | Refunds, emails |
| Guardrailed agent | PII + custom middleware | Fintech, healthcare |
| Cost-aware routing | Dynamic model middleware / `configurable_alternatives` | High-volume chat |
| Multi-agent (subagents as tools) | `create_agent` inside tools | Research + writing |
| Batch/offline processing | `batch` / `abatch` + rate limiter + cache | Nightly enrichment |
| Streaming chat API | `astream_events` / agent stream + FastAPI SSE | Production chat UI |

---

## Q29. Build a simple **LLM chain** with validation. Use case: rewrite product descriptions in the brand voice under 80 words.

```python
brand_rewrite = (
    ChatPromptTemplate.from_messages([
        ("system", "Rewrite in Acme's voice: friendly, concrete, no superlatives. Max 80 words."),
        ("human", "{description}"),
    ])
    | llm
    | StrOutputParser()
    | RunnableLambda(lambda s: s if len(s.split()) <= 80 else " ".join(s.split()[:80]) + "…")
)
results = brand_rewrite.batch([{"description": d} for d in descriptions], config={"max_concurrency": 10})
```

**Why LCEL:** a linear, stateless task that gets batching, streaming and tracing for free. **Trade-off:** there's no self-check loop. Add an evaluator chain if quality varies.

---

## Q30. Design **summarisation pipelines**: stuff vs map-reduce vs refine. Use case: summarise 300-page board meeting transcripts.

| Strategy | How | Pros | Cons |
|---|---|---|---|
| **Stuff** | Put the whole text in one prompt | 1 call, best coherence | Must fit the context window; "lost in the middle" for huge inputs |
| **Map-reduce** | Summarise chunks in parallel → combine (recursively if needed) | Scales, parallel, fast | May lose cross-chunk links |
| **Refine** | Iterate: summary + next chunk → updated summary | Keeps narrative flow | Sequential and slow; early content can get diluted |
| Hierarchical (sections → chapters → document) | Multi-level map-reduce | Good structure | More calls |

```python
from langchain_text_splitters import RecursiveCharacterTextSplitter

map_chain = ChatPromptTemplate.from_template(
    "Summarise this transcript part. Keep decisions, owners, numbers:\n{chunk}") | llm | StrOutputParser()
reduce_chain = ChatPromptTemplate.from_template(
    "Combine into a board summary with sections Decisions, Action Items (owner, due), Risks:\n{summaries}"
) | llm | StrOutputParser()

def map_reduce_summarize(text: str, max_tokens_for_reduce: int = 12_000) -> str:
    chunks = RecursiveCharacterTextSplitter.from_tiktoken_encoder(chunk_size=3000, chunk_overlap=200).split_text(text)
    summaries = map_chain.batch([{"chunk": c} for c in chunks], config={"max_concurrency": 8})
    while sum(len(s) // 4 for s in summaries) > max_tokens_for_reduce:          # recursive collapse
        groups = ["\n".join(summaries[i:i + 6]) for i in range(0, len(summaries), 6)]
        summaries = map_chain.batch([{"chunk": g} for g in groups])
    return reduce_chain.invoke({"summaries": "\n---\n".join(summaries)})

def refine_summarize(chunks: list[str]) -> str:
    refine = ChatPromptTemplate.from_template(
        "Existing summary:\n{summary}\nNew content:\n{chunk}\nUpdate the summary.") | llm | StrOutputParser()
    summary = ""
    for c in chunks:
        summary = refine.invoke({"summary": summary, "chunk": c})
    return summary
```

**Rule of thumb:**
- Fits comfortably in the context window → stuff.
- Large and needs speed → map-reduce.
- Story-like and order-dependent → refine or hierarchical.

---

## Q31. Design a **structured extraction pipeline**. Use case: extract fields from 20,000 scanned invoices into a database.

**Pipeline:** load/OCR → `with_structured_output(Invoice)` → validate (Pydantic plus business rules) → retry failures with a stronger model → dead-letter queue → DB.

```python
from datetime import date
from pydantic import field_validator

class LineItem(BaseModel):
    description: str
    qty: float
    unit_price: float

class Invoice(BaseModel):
    invoice_number: str
    vendor: str
    invoice_date: date
    currency: Literal["INR", "USD", "EUR"]
    line_items: list[LineItem]
    total: float

    @field_validator("total")
    @classmethod
    def total_matches(cls, v, info):
        items = info.data.get("line_items") or []
        calc = round(sum(i.qty * i.unit_price for i in items), 2)
        if items and abs(calc - v) > 1.0:
            raise ValueError(f"total {v} != sum of items {calc}")
        return v

cheap = init_chat_model("openai:gpt-4.1-mini", temperature=0).with_structured_output(Invoice, include_raw=True)
strong = init_chat_model("openai:gpt-4.1", temperature=0).with_structured_output(Invoice, include_raw=True)

def extract_one(text: str) -> dict:
    for model in (cheap, strong):                            # escalate on failure
        out = model.invoke(f"Extract the invoice. Use null when absent.\n{text}")
        if out["parsed"] is not None:
            return {"ok": True, "invoice": out["parsed"]}
    return {"ok": False, "error": str(out["parsing_error"]), "text": text[:500]}

extract_pipeline = RunnableLambda(extract_one)
results = extract_pipeline.batch(invoice_texts, config={"max_concurrency": 16, "run_name": "invoice_extract"})
```

**Trade-offs:**
- Escalating from a cheap to a strong model keeps cost low while recovering hard cases.
- Business-rule validators catch "valid JSON, wrong numbers" errors.
- Track the parse-failure rate as a metric.

---

## Q32. Design a **classification + routing pipeline**. Use case: route incoming emails to sales, support or legal chains, each with its own prompt.

```python
class Route(BaseModel):
    destination: Literal["sales", "support", "legal", "other"]
    confidence: float

classifier = ChatPromptTemplate.from_template("Classify this email:\n{email}") | llm.with_structured_output(Route)

def specialist(system: str):
    return ChatPromptTemplate.from_messages([("system", system), ("human", "{email}")]) | llm | StrOutputParser()

sales_chain = specialist("Draft a sales reply. Offer a demo.")
support_chain = specialist("Draft a support reply. Ask for order ID if missing.")
legal_chain = RunnableLambda(lambda x: "Forwarded to legal@acme.com for review.")   # no LLM drafting for legal
fallback_chain = specialist("Draft a polite generic acknowledgement.")

router = RunnableBranch(
    (lambda x: x["route"].confidence < 0.6, fallback_chain),
    (lambda x: x["route"].destination == "sales", sales_chain),
    (lambda x: x["route"].destination == "support", support_chain),
    (lambda x: x["route"].destination == "legal", legal_chain),
    fallback_chain,
)
email_pipeline = RunnablePassthrough.assign(route=classifier) | router
email_pipeline.invoke({"email": "Hi, we'd like pricing for 200 seats"})
```

**Alternative:** a plain function returning a runnable (`RunnableLambda(lambda x: routes[x["route"].destination])`). This is often cleaner than `RunnableBranch` when there are many routes.

---

## Q33. Design a **parallel enrichment pipeline**. Use case: for each product review, compute sentiment, topics, a summary and a toxicity flag.

```python
class Sentiment(BaseModel):
    label: Literal["positive", "neutral", "negative"]
    score: float

class Topics(BaseModel):
    topics: list[str]

enrich = RunnableParallel(
    review=itemgetter("text"),
    sentiment=ChatPromptTemplate.from_template("Sentiment of:\n{text}") | llm.with_structured_output(Sentiment),
    topics=ChatPromptTemplate.from_template("Up to 3 topics:\n{text}") | llm.with_structured_output(Topics),
    summary=ChatPromptTemplate.from_template("One-line summary:\n{text}") | llm | StrOutputParser(),
    toxic=RunnableLambda(lambda x: toxicity_model.predict(x["text"]) > 0.8),   # non-LLM step in parallel
)
rows = enrich.batch([{"text": r} for r in reviews], config={"max_concurrency": 6})
```

**Trade-off:** four calls per review run at the same time, so latency is roughly that of a single call, but cost is 4×. Merge them into **one** structured-output call when cost matters more than modularity.

---

## Q34. Build a **basic RAG pipeline with citations**. Use case: HR policy Q&A.

```python
from langchain_core.vectorstores import InMemoryVectorStore

vs = InMemoryVectorStore.from_documents(chunks, embeddings)           # swap for PGVector/Qdrant in prod
retriever = vs.as_retriever(search_kwargs={"k": 5})

def format_docs(docs):
    return "\n\n".join(f"[{i}] (source: {d.metadata.get('source')}, p.{d.metadata.get('page')})\n{d.page_content}"
                       for i, d in enumerate(docs, 1))

rag_prompt = ChatPromptTemplate.from_messages([
    ("system", "Answer ONLY from the context. Cite sources like [1]. If the answer isn't there, say "
               "\"I couldn't find this in the HR policies.\""),
    ("human", "Context:\n{context}\n\nQuestion: {question}"),
])

rag_chain = (
    RunnableParallel(docs=itemgetter("question") | retriever, question=itemgetter("question"))
    | RunnablePassthrough.assign(context=lambda x: format_docs(x["docs"]))
    | RunnableParallel(
        answer=rag_prompt | llm | StrOutputParser(),
        sources=lambda x: [d.metadata.get("source") for d in x["docs"]],
    )
)
rag_chain.invoke({"question": "How many casual leaves do I get?"})
# → {"answer": "... [2]", "sources": ["leave_policy.pdf", ...]}
```

`langchain_classic.chains.create_retrieval_chain` + `create_stuff_documents_chain` is the prebuilt equivalent.

---

## Q35. Build a **conversational RAG pipeline**. Use case: a docs chatbot where users ask follow-ups like "what about for contractors?"

**Pipeline:** (history + follow-up) → condense into a standalone question (skip if there's no history) → retrieve → answer with history → persist history.

```python
condense = ChatPromptTemplate.from_messages([
    ("system", "Rewrite the latest user question as a standalone search query using the chat history. "
               "If it's already standalone, return it unchanged."),
    MessagesPlaceholder("history"), ("human", "{question}"),
]) | llm | StrOutputParser()

def standalone(x):
    return x["question"] if not x.get("history") else condense.invoke(x)

answer_prompt = ChatPromptTemplate.from_messages([
    ("system", "Answer from context with [n] citations.\n\nContext:\n{context}"),
    MessagesPlaceholder("history"), ("human", "{question}"),
])

conv_rag = (
    RunnablePassthrough.assign(search_query=RunnableLambda(standalone))
    | RunnablePassthrough.assign(docs=itemgetter("search_query") | retriever)
    | RunnablePassthrough.assign(context=lambda x: format_docs(x["docs"]))
    | answer_prompt | llm | StrOutputParser()
)

conv_rag_with_memory = RunnableWithMessageHistory(conv_rag, get_history,
                                                  input_messages_key="question", history_messages_key="history")
cfg = {"configurable": {"session_id": "emp-77"}}
conv_rag_with_memory.invoke({"question": "What's the notice period for employees?"}, cfg)
conv_rag_with_memory.invoke({"question": "What about for contractors?"}, cfg)
```

The prebuilt equivalent is `create_history_aware_retriever` (`langchain-classic`). **Agent alternative:** give `create_agent` a retriever tool plus a checkpointer. The model then decides when to search and rewrites queries itself (Q40).

---

## Q36. Build an **advanced retrieval RAG**: hybrid search + re-ranking + compression. Use case: technical support docs full of error codes like `E-4012`.

**Why:** dense embeddings miss exact codes, and top-k contains noise.
- **BM25 + dense** (`EnsembleRetriever`, RRF) fixes recall for exact tokens.
- A **cross-encoder re-ranker** fixes precision.
- **Compression** cuts tokens.

```python
from langchain_classic.retrievers import ContextualCompressionRetriever, EnsembleRetriever
from langchain_classic.retrievers.document_compressors import CrossEncoderReranker
from langchain_community.cross_encoders import HuggingFaceCrossEncoder
from langchain_community.retrievers import BM25Retriever

bm25 = BM25Retriever.from_documents(chunks); bm25.k = 20
dense = vs.as_retriever(search_kwargs={"k": 20})
hybrid = EnsembleRetriever(retrievers=[bm25, dense], weights=[0.5, 0.5])

reranker = CrossEncoderReranker(model=HuggingFaceCrossEncoder(model_name="BAAI/bge-reranker-base"), top_n=4)
precise_retriever = ContextualCompressionRetriever(base_compressor=reranker, base_retriever=hybrid)

def route_retriever(q: str):
    import re
    if re.search(r"\b[A-Z]-\d{3,5}\b", q):           # exact code present → also hard-filter by metadata
        code = re.search(r"\b[A-Z]-\d{3,5}\b", q).group()
        exact = vs.similarity_search(q, k=4, filter=lambda d: code in d.metadata.get("codes", []))
        if exact:
            return exact
    return precise_retriever.invoke(q)

tech_rag = (RunnablePassthrough.assign(context=itemgetter("question") | RunnableLambda(route_retriever) | format_docs)
            | rag_prompt | llm | StrOutputParser())
```

| Component | Gain | Cost |
|---|---|---|
| BM25 | Exact-match recall | Separate index (or use native hybrid in the vector DB) |
| Re-ranker | Big precision gain | +50–300 ms, a model to host or call |
| Compression | Fewer tokens | Extra compute; risk of dropping context |

---

## Q37. Design the **ingestion / indexing pipeline** so the vector store stays in sync without re-embedding everything. Use case: a nightly Confluence sync with edits and deletions.

Use the **indexing API** with a **RecordManager**. It tracks a hash of each document's content and metadata and its source, so you can:
- skip unchanged docs,
- re-embed changed ones,
- delete removed ones.

| `cleanup` mode | Behaviour | Use |
|---|---|---|
| `None` | Only add/skip; never delete | Append-only corpora |
| `"incremental"` | Delete old versions of docs from sources seen in this batch | Continuous partial syncs |
| `"full"` | Delete anything not in this run | Full snapshot syncs (deletes docs removed at the source) |
| `"scoped_full"` | Like `full`, but only for sources in this batch | Large corpora synced in batches |

```python
from langchain_core.indexing import index
from langchain_classic.indexes import SQLRecordManager

record_manager = SQLRecordManager("pgvector/confluence_kb", db_url="postgresql+psycopg://...")
record_manager.create_schema()

def nightly_sync():
    docs = []
    for page in confluence_loader.lazy_load():
        page.metadata["source"] = page.metadata["page_id"]                 # stable source id
        docs.extend(splitter.split_documents([page]))
    stats = index(docs, record_manager, vectorstore, cleanup="full", source_id_key="source",
                  batch_size=200)
    log.info("index_sync", extra=stats)     # {'num_added': 120, 'num_updated': 0, 'num_skipped': 9800, 'num_deleted': 14}
```

Combine this with `CacheBackedEmbeddings` (Q18) to avoid paying for unchanged chunk embeddings when you change chunk settings.

---

## Q38. Build a **tool-calling customer-support agent**. Use case: order status, address change, refunds with limits.

```python
from dataclasses import dataclass
from langchain.agents import create_agent
from langchain.agents.middleware import (HumanInTheLoopMiddleware, ModelCallLimitMiddleware,
                                         SummarizationMiddleware, ToolRetryMiddleware, ToolCallLimitMiddleware)
from langchain.tools import tool, ToolRuntime

@dataclass
class SupportCtx:
    user_id: str
    tier: Literal["free", "gold"] = "free"

@tool
def order_status(order_id: str, runtime: ToolRuntime[SupportCtx]) -> str:
    """Get shipping status of one of the user's orders."""
    order = orders_db.get(order_id)
    if not order or order.user_id != runtime.context.user_id:      # authorisation inside the tool
        return "No such order on your account."
    return f"{order_id}: {order.status}, ETA {order.eta}"

@tool
def change_address(order_id: str, new_address: str, runtime: ToolRuntime[SupportCtx]) -> str:
    """Change delivery address if the order hasn't shipped."""
    return orders_db.change_address(order_id, new_address, user=runtime.context.user_id)

support_agent = create_agent(
    "openai:gpt-4.1-mini",
    tools=[order_status, change_address, issue_refund],
    system_prompt="You are Acme support. Verify order ownership via tools. Refunds above ₹10,000 need approval.",
    context_schema=SupportCtx,
    checkpointer=checkpointer,
    middleware=[
        SummarizationMiddleware("openai:gpt-4.1-mini", trigger=("tokens", 6000), keep=("messages", 10)),
        ToolRetryMiddleware(max_retries=2, tools=["order_status"]),
        ModelCallLimitMiddleware(run_limit=8),
        HumanInTheLoopMiddleware(interrupt_on={"issue_refund": True, "change_address": False}),
    ],
)
```

The HITL resume flow is in Q42.

---

## Q39. Build a **SQL question-answering pipeline**. Use case: "Top 5 cities by revenue last quarter?" on Postgres.

**Two designs:**

| Design | How | Pros | Cons |
|---|---|---|---|
| **Chain** (deterministic) | Schema → write SQL → validate → execute → answer | Predictable, fewer calls | One-shot; weak on complex questions |
| **Agent** with SQL tools (`list_tables`, `get_schema`, `query_checker`, `run_query`) | Model explores schema and retries on errors | Handles complex and ambiguous questions | More calls; needs guardrails |

```python
import sqlglot
from sqlalchemy import create_engine, text

engine = create_engine("postgresql+psycopg://readonly_user:...@db/analytics")   # READ-ONLY role
ALLOWED = {"orders", "customers", "cities"}

@tool
def get_schema(tables: list[str]) -> str:
    """Return CREATE TABLE statements for the given tables."""
    return schema_cache.describe([t for t in tables if t in ALLOWED])

@tool
def run_query(sql: str) -> str:
    """Run a read-only SQL SELECT and return up to 50 rows."""
    tree = sqlglot.parse_one(sql, read="postgres")
    if tree.key != "select":
        return "Error: only SELECT statements are allowed."
    if {t.name for t in tree.find_all(sqlglot.exp.Table)} - ALLOWED:
        return f"Error: allowed tables are {sorted(ALLOWED)}"
    if not tree.args.get("limit"):
        tree = tree.limit(50)
    with engine.connect() as c:
        c.execute(text("SET statement_timeout = 5000"))
        rows = c.execute(text(tree.sql("postgres"))).fetchall()
    return "\n".join(map(str, rows))

sql_agent = create_agent("openai:gpt-4.1", tools=[get_schema, run_query],
                         system_prompt=f"You answer analytics questions. Tables: {sorted(ALLOWED)}. "
                                       "Always inspect schema first. If a query errors, fix and retry (max 3).",
                         middleware=[ToolCallLimitMiddleware(tool_name="run_query", run_limit=4)])
```

---

## Q40. Build an **agentic RAG pipeline** over multiple sources. Use case: an employee assistant over HR policies, the IT wiki and the Jira knowledge base.

**Why an agent:** the model decides **which** source to search, can search several times with refined queries, and can skip retrieval for small talk.

```python
@tool(response_format="content_and_artifact")
def search_hr(query: str):
    """Search HR policies (leave, payroll, benefits)."""
    docs = hr_retriever.invoke(query)
    return format_docs(docs), docs                     # content for the model, artifact (docs) for the app

@tool(response_format="content_and_artifact")
def search_it(query: str):
    """Search the IT wiki (VPN, laptops, access requests)."""
    docs = it_retriever.invoke(query)
    return format_docs(docs), docs

kb_agent = create_agent(
    "openai:gpt-4.1-mini", tools=[search_hr, search_it, search_jira_kb],
    system_prompt="Use the search tools for company-specific questions. Cite sources as [n]. "
                  "If tools return nothing relevant, say you don't know.",
    checkpointer=checkpointer,
    middleware=[ToolCallLimitMiddleware(run_limit=5)],
)
```

| Chain RAG | Agentic RAG |
|---|---|
| 1 retrieval + 1 LLM call; predictable latency and cost | 2–6 calls; adaptive; handles multi-source and multi-hop questions |
| Always retrieves (even for "thanks!") | Can skip retrieval |
| Easier to evaluate | Needs trajectory evals and call limits |

---

## Q41. Design a **guardrailed pipeline** for a regulated domain. Use case: a stock-broking assistant: mask card and PAN numbers, block insider-trading facilitation, add disclaimers.

```python
import re
from langchain.agents.middleware import PIIMiddleware, before_model, after_model, AgentState

PAN = re.compile(r"\b[A-Z]{5}\d{4}[A-Z]\b")

class Policy(BaseModel):
    violation: bool
    category: Literal["insider_trading", "manipulation", "none"]

policy_checker = init_chat_model("openai:gpt-4.1-mini", temperature=0).with_structured_output(Policy)

@before_model(can_jump_to=["end"])
def compliance_gate(state: AgentState, runtime) -> dict | None:
    last_user = next(m for m in reversed(state["messages"]) if m.type == "human")
    v = policy_checker.invoke(f"Does this ask how to commit illegal trading? Education is allowed.\n{last_user.content}")
    if v.violation:
        return {"messages": [AIMessage("I can't help with that — trading on unpublished price-sensitive info "
                                       "is illegal under SEBI rules. I can explain legal investing strategies.")],
                "jump_to": "end"}
    return None

@after_model
def add_disclaimer(state: AgentState, runtime) -> dict | None:
    last = state["messages"][-1]
    if not last.tool_calls and "not investment advice" not in str(last.content):
        return {"messages": [AIMessage(id=last.id, content=f"{last.content}\n\n_This is not investment advice._")]}
    return None

broker_agent = create_agent(
    "openai:gpt-4.1-mini", tools=[portfolio_lookup, market_data],
    middleware=[
        PIIMiddleware("credit_card", strategy="mask", apply_to_input=True, apply_to_output=True),
        PIIMiddleware("pan", detector=lambda text: [{"type": "pan", "value": m.group(), "start": m.start(),
                                                     "end": m.end()} for m in PAN.finditer(text)],
                      strategy="redact", apply_to_input=True, apply_to_output=True),
        compliance_gate, add_disclaimer,
    ])
```

> A custom `detector` returns a list of `PIIMatch` dicts (`type`, `value`, `start`, `end`). Verified against langchain 1.5.2.

---

## Q42. Design a **human-in-the-loop agent pipeline**. Use case: an agent that drafts and sends customer emails, where every send needs approval or edits.

```python
from langgraph.types import Command

email_agent = create_agent(
    "openai:gpt-4.1-mini", tools=[search_crm, send_email],
    checkpointer=checkpointer,                             # required for interrupts
    middleware=[HumanInTheLoopMiddleware(interrupt_on={
        "send_email": {"allowed_decisions": ["approve", "edit", "reject"]},
        "search_crm": False,
    })],
)

cfg = {"configurable": {"thread_id": "campaign-42"}}
result = email_agent.invoke({"messages": [("user", "Email Priya about her renewal discount")]}, cfg)
if "__interrupt__" in result:
    pending = result["__interrupt__"][0].value             # proposed tool call(s) for the reviewer UI
    # Reviewer edits the body and approves:
    result = email_agent.invoke(Command(resume={"decisions": [
        {"type": "edit", "edited_action": {"name": "send_email",
                                           "args": {"to": "priya@x.com", "subject": "Renewal",
                                                    "body": "Hi Priya, ..."}}}]}), cfg)
```

> The resume payload is `{"decisions": [...]}` with one decision per pending tool call: `{"type": "approve"}`, `{"type": "edit", "edited_action": {"name", "args"}}`, `{"type": "reject", ...}`. Verified against langchain 1.5.2. `interrupt_mode="per_call"` raises a separate interrupt for each gated call.

---

## Q43. Design **cost-aware model routing**. Use case: a chatbot with 1M messages/day where most questions are simple.

| Option | How | Trade-off |
|---|---|---|
| `wrap_model_call` dynamic model (Q12) | Rules (length, keywords, tier) pick the model per call | Cheap; rules can misroute |
| `ModelRouterMiddleware` | A classifier chooses among configured models | Extra classification step |
| LCEL `configurable_alternatives` (Q24) | Caller chooses the model per request | Routing logic lives outside |
| Cascade | Small model answers → self-check → escalate if weak | Pays twice on escalations |

```python
@wrap_model_call
def cascade(request: ModelRequest, handler) -> ModelResponse:
    small = handler(request.override(model=init_chat_model("openai:gpt-4.1-mini")))
    text = str(small.result[-1].content)                    # ModelResponse.result = list of messages
    unsure = any(p in text.lower() for p in ("i'm not sure", "i don't know", "cannot determine"))
    if unsure or len(request.messages) > 20:
        return handler(request.override(model=init_chat_model("openai:gpt-4.1")))
    return small
```

> `ModelResponse` has `result` (messages) and `structured_response`. In production, cache the `init_chat_model` instances instead of creating them per call.

---

## Q44. Design a **multi-agent pipeline** with LangChain only (subagents as tools). Use case: a research assistant with a "researcher" and a "writer".

**Pattern:** a **supervisor agent** whose tools call other agents. Each subagent has its own prompt and tools, and returns only its **final result**, which keeps the supervisor's context small.

```python
researcher = create_agent("openai:gpt-4.1-mini", tools=[web_search, fetch_page],
                          system_prompt="Research the question. Return key facts with URLs.")
writer = create_agent("openai:gpt-4.1", tools=[],
                      system_prompt="Write a clear 300-word brief from the given notes. Keep citations.")

@tool
def research(question: str) -> str:
    """Delegate a research question to the research specialist."""
    out = researcher.invoke({"messages": [("user", question)]})
    return out["messages"][-1].content

@tool
def write_brief(notes: str) -> str:
    """Delegate writing a brief from research notes to the writing specialist."""
    return writer.invoke({"messages": [("user", notes)]})["messages"][-1].content

supervisor = create_agent("openai:gpt-4.1-mini", tools=[research, write_brief],
                          system_prompt="Plan, call research (possibly several times), then write_brief.",
                          middleware=[ModelCallLimitMiddleware(run_limit=8)])
```

| Subagents-as-tools | LangGraph supervisor/swarm graph |
|---|---|
| Simple; context isolation; reuses `create_agent` | Explicit routing, shared state, handoffs, per-agent persistence |
| Subagent internals are hidden from the supervisor | More control over which messages each agent sees |

---

## Q45. Design a **batch/offline processing pipeline**. Use case: classify 2M support tickets overnight within rate limits and budget.

```python
from langchain_core.rate_limiters import InMemoryRateLimiter
from langchain_core.callbacks import get_usage_metadata_callback

limiter = InMemoryRateLimiter(requests_per_second=20, check_every_n_seconds=0.05, max_bucket_size=40)
clf_model = init_chat_model("openai:gpt-4.1-mini", temperature=0, rate_limiter=limiter, max_retries=3)
classifier = (ChatPromptTemplate.from_template("Classify:\n{ticket}")
              | clf_model.with_structured_output(Route)).with_retry(stop_after_attempt=3)

def run_batches(tickets, batch=500):
    with get_usage_metadata_callback() as usage:               # token accounting across all calls
        for i in range(0, len(tickets), batch):
            part = tickets[i:i + batch]
            outs = classifier.batch([{"ticket": t["text"]} for t in part],
                                    config={"max_concurrency": 32, "run_name": "nightly_ticket_clf",
                                            "tags": ["batch"]},
                                    return_exceptions=True)      # one failure doesn't kill the batch
            save([(t["id"], o) for t, o in zip(part, outs) if not isinstance(o, Exception)])
            retry_queue([t for t, o in zip(part, outs) if isinstance(o, Exception)])
            checkpoint_progress(i + batch)                       # resume point
        log.info("usage", extra=usage.usage_metadata)
```

**Also consider:**
- **provider batch APIs** (about 50% cheaper, with delayed results) for non-urgent jobs,
- an exact LLM cache for duplicate tickets,
- sampling (`LANGSMITH_TRACING_SAMPLING_RATE`) so 2M traces don't flood observability.

---

## Q46. Build a **streaming chat API** for production. Use case: a FastAPI backend streaming an agent's tokens and tool progress to a web UI.

```python
from fastapi import FastAPI
from fastapi.responses import StreamingResponse
import json

app = FastAPI()

@app.post("/chat/{thread_id}")
async def chat(thread_id: str, body: dict):
    cfg = {"configurable": {"thread_id": thread_id}, "run_name": "support_chat",
           "metadata": {"user_id": body["user_id"]}, "tags": ["api"]}

    async def events():
        async for mode, chunk in support_agent.astream(
                {"messages": [("user", body["message"])]}, cfg,
                stream_mode=["messages", "updates", "custom"],
                context=SupportCtx(user_id=body["user_id"])):
            if mode == "messages":
                token, meta = chunk
                if meta.get("langgraph_node") == "model" and token.content:
                    yield f"event: token\ndata: {json.dumps(token.content if isinstance(token.content, str) else token.text)}\n\n"
            elif mode == "updates" and "tools" in chunk:
                yield f"event: tool\ndata: {json.dumps('tool finished')}\n\n"
            elif mode == "custom":
                yield f"event: progress\ndata: {json.dumps(chunk)}\n\n"
        yield "event: done\ndata: {}\n\n"

    return StreamingResponse(events(), media_type="text/event-stream",
                             headers={"Cache-Control": "no-cache", "X-Accel-Buffering": "no"})
```

---

## Q47. How do you choose between these pipelines? Give a decision framework.

1. **One-shot transformation?** → LCEL chain (Q29). Many items → `batch` with a rate limiter (Q45).
2. **Need typed output?** → `with_structured_output` + validators (Q31).
3. **Different handling per input type?** → classifier + router (Q32).
4. **Independent analyses of the same input?** → `RunnableParallel` (Q33).
5. **Questions over documents?**
   - fixed sources → RAG chain (Q34),
   - follow-ups → conversational RAG (Q35),
   - codes, IDs or precision-critical → hybrid + rerank (Q36),
   - many sources or multi-hop → agentic RAG (Q40).
6. **Actions in external systems?** → `create_agent` with tools (Q38), plus HITL for irreversible actions (Q42) and guardrails for regulated domains (Q41).
7. **Very long or complex tasks, custom loops, durable workflows, many agents?** → step down to **LangGraph**.
8. **Always:** ingestion kept in sync (Q37), tracing, evals and cost controls (Part C).

---

# Part C — Observability & Configuration (Q48–Q60)

## Q48. What should you observe in a LangChain application, and why is it harder than in normal services?

LLM apps are **non-deterministic, multi-step and costly per call**. A "200 OK" can still be a wrong, hallucinated or unsafe answer. You need the following.

| Signal | Examples |
|---|---|
| **Traces** | Nested runs: chain → prompt → model → parser; agent → model → tool → model; retriever spans with returned docs |
| **Inputs and outputs** | Rendered prompts, model responses, tool args and results, retrieved chunks (masked) |
| **Usage and cost** | Input/output/cached/reasoning tokens per call; cost per request, feature and tenant |
| **Latency** | Per step; TTFT (time to first token) for streams; p50/p95/p99 |
| **Errors** | Provider errors (429/5xx), parse failures, tool exceptions, guardrail blocks, call-limit hits |
| **Quality** | User feedback, online LLM-judge scores (faithfulness, relevance), refusal rate, "I don't know" rate |
| **Metadata** | Tenant, user, prompt version, model version, app version, experiment arm |
| **Drift** | Changes in answer length, refusal rate or quality scores over time |

---

## Q49. Explain the callback system. What events exist, and how do you write a custom handler?

Callbacks are hooks fired by every component. Tracing (LangSmith, Langfuse) is itself built on callbacks.

| Event | Fired by |
|---|---|
| `on_chat_model_start`, `on_llm_new_token`, `on_llm_end`, `on_llm_error` | Chat models |
| `on_chain_start`, `on_chain_end`, `on_chain_error` | Runnables and chains (and agent nodes) |
| `on_tool_start`, `on_tool_end`, `on_tool_error` | Tools |
| `on_retriever_start`, `on_retriever_end`, `on_retriever_error` | Retrievers |
| `on_custom_event` | `dispatch_custom_event` from your code |

**Where you attach callbacks matters:**
- **Request-time:** `config={"callbacks": [...]}`. Propagates to all children. This is the **recommended** option.
- **Constructor:** `init_chat_model(..., callbacks=[...])`. Only that object, not its children.

```python
import time
from langchain_core.callbacks import BaseCallbackHandler

PRICES = {"gpt-4.1-mini": (0.40, 1.60)}   # $/1M tokens (example values — use your provider's current prices)

class CostLatencyHandler(BaseCallbackHandler):
    def __init__(self):
        self.t0, self.total_cost, self.calls = {}, 0.0, []

    def on_chat_model_start(self, serialized, messages, *, run_id, **kw):
        self.t0[run_id] = time.perf_counter()

    def on_llm_end(self, response, *, run_id, **kw):
        ms = (time.perf_counter() - self.t0.pop(run_id, time.perf_counter())) * 1000
        msg = response.generations[0][0].message
        usage = msg.usage_metadata or {}
        model = msg.response_metadata.get("model_name", "unknown")
        pin, pout = PRICES.get(model.split("-2")[0], (0, 0))
        cost = (usage.get("input_tokens", 0) * pin + usage.get("output_tokens", 0) * pout) / 1e6
        self.total_cost += cost
        self.calls.append({"model": model, "ms": round(ms), "cost": cost, **usage})

    def on_tool_error(self, error, *, run_id, **kw):
        log.warning("tool_error", extra={"error": str(error)})

h = CostLatencyHandler()
rag_chain.invoke({"question": "leave policy?"}, config={"callbacks": [h]})
print(h.calls, f"${h.total_cost:.5f}")
```

Async apps should use `AsyncCallbackHandler`, so slow handlers don't block the event loop.

---

## Q50. How do you track token usage across a whole request?

```python
from langchain_core.callbacks import get_usage_metadata_callback

with get_usage_metadata_callback() as cb:
    support_agent.invoke({"messages": [("user", "Where is 1766?")]},
                         {"configurable": {"thread_id": "t1"}}, context=SupportCtx("u42"))
print(cb.usage_metadata)
# {'gpt-4.1-mini-2025-04-14': {'input_tokens': 2310, 'output_tokens': 182, 'total_tokens': 2492, ...}}
```

Usage is grouped by the `model_name` in each response's `response_metadata`. Custom or fake models that don't set it won't appear in the totals.

Every `AIMessage` also carries `usage_metadata` (including `input_token_details` such as `cache_read`, and `output_token_details` such as `reasoning`), which you can log per call.

---

## Q51. How do you enable LangSmith tracing, and which environment variables matter?

Tracing is automatic for LangChain and LangGraph once the environment variables are set. No code changes are needed.

```bash
LANGSMITH_TRACING=true
LANGSMITH_API_KEY=lsv2_...
LANGSMITH_PROJECT=hr-assistant-prod        # one project per app + environment
LANGSMITH_ENDPOINT=https://api.smith.langchain.com   # EU region or self-hosted URL if applicable
LANGSMITH_TRACING_SAMPLING_RATE=0.2        # keep 20% of traces (high-volume prod)
LANGSMITH_HIDE_INPUTS=false                # true = drop all inputs from traces
LANGSMITH_HIDE_OUTPUTS=false
LANGSMITH_OTEL_ENABLED=false               # true = emit OpenTelemetry spans
```

| Variable | Purpose |
|---|---|
| `LANGSMITH_TRACING` | Master switch |
| `LANGSMITH_API_KEY` | Auth |
| `LANGSMITH_PROJECT` | Trace grouping (dev/staging/prod) |
| `LANGSMITH_ENDPOINT` | Region or self-hosted instance |
| `LANGSMITH_TRACING_SAMPLING_RATE` | 0–1 sampling |
| `LANGSMITH_HIDE_INPUTS` / `_OUTPUTS` | Coarse privacy switches |
| `LANGSMITH_OTEL_ENABLED` | OpenTelemetry export |

Older `LANGCHAIN_TRACING_V2` / `LANGCHAIN_API_KEY` / `LANGCHAIN_PROJECT` names still appear in older code. Prefer `LANGSMITH_*`.

For **serverless or short-lived** processes, flush before exit so traces aren't lost: call `langsmith.Client().flush()`, or set `LANGCHAIN_CALLBACKS_BACKGROUND=false` so callbacks finish before the function returns.

---

## Q52. How do you make traces searchable and useful: run names, tags, metadata, and feedback?

```python
import uuid
from langsmith import Client

run_id = uuid.uuid4()
answer = rag_chain.invoke(
    {"question": q},
    config={"run_name": "hr_rag.answer", "run_id": run_id,
            "tags": ["prod", "hr", "rag:v4"],
            "metadata": {"tenant": "acme", "user_id": "emp-77", "prompt_version": "hr_rag@12",
                         "retriever": "hybrid+rerank", "app_version": "2.8.1"}},
)

# Name intermediate steps so they're readable in the trace tree
retrieval_step = (itemgetter("question") | retriever).with_config(run_name="retrieve_hr_docs")

# User feedback (👍/👎) attached to the exact run
Client().create_feedback(run_id=run_id, key="user_rating", score=1, comment="accurate")
```

Use metadata to **slice dashboards** (cost per tenant, error rate per prompt version) and to build **datasets** from failing runs.

---

## Q53. How do you trace your own non-LangChain code?

`@traceable` (and `trace` / `RunTree` for manual control) adds spans for plain Python. They nest automatically under the active LangChain run.

```python
from langsmith import traceable

@traceable(run_type="retriever", name="acl_vector_search")
def acl_search(query: str, groups: list[str]) -> list[dict]:
    return qdrant_client.search(query, filter={"acl": groups}, limit=8)

@traceable(run_type="tool", name="payments_api.refund", metadata={"system": "razorpay"})
def call_refund_api(order_id: str, amount: float) -> dict:
    return payments.refund(order_id, amount)

@traceable(name="answer_pipeline")
def answer(question: str, user):
    docs = acl_search(question, user.groups)
    return rag_prompt.pipe(llm).invoke({"context": docs, "question": question})
```

`run_type` values: `chain`, `llm`, `tool`, `retriever`, `embedding`, `prompt`, `parser`. They control how the span is rendered.

---

## Q54. How do you control what gets traced: sampling, conditional tracing, and PII masking?

```python
from langsmith import Client, tracing_context
from langsmith.anonymizer import create_anonymizer
from langchain_core.tracers.langchain import LangChainTracer

# 1) Conditional tracing (highest priority): never trace one tenant, always trace debug requests
def handle(req):
    with tracing_context(enabled=req.headers.get("x-debug") == "1" or req.tenant != "hospital-a"):
        return support_agent.invoke(req.payload, req.config, context=req.ctx)

# 2) Per-client sampling
sampled_client = Client(tracing_sampling_rate=0.1)

# 3) Field-level masking with an anonymizer
anonymizer = create_anonymizer([
    {"pattern": r"[\w\.-]+@[\w\.-]+\.\w+", "replace": "<email>"},
    {"pattern": r"\b[A-Z]{5}\d{4}[A-Z]\b", "replace": "<pan>"},
    {"pattern": r"\b(?:\d[ -]?){13,19}\b", "replace": "<card>"},
])
masked_tracer = LangChainTracer(client=Client(anonymizer=anonymizer), project_name="broker-prod")
broker_agent.invoke(payload, {"callbacks": [masked_tracer]})
```

| Technique | Pros | Cons |
|---|---|---|
| Global sampling | Reduces volume and cost | Misses rare failures (pair it with always-trace-on-error or debug flags) |
| Conditional tracing | Precise; per tenant or feature | Logic to maintain |
| Anonymizer | Traces stay debuggable, PII removed | Regex may miss formats |
| Hide inputs/outputs | Simple compliance | Traces lose most of their debugging value |
| Self-hosted LangSmith or Langfuse | Data stays in your environment | Ops effort |

Also **redact before the model** with `PIIMiddleware`, so sensitive data never reaches the provider *or* the trace.

---

## Q55. How do you export traces via OpenTelemetry to Datadog, Grafana Tempo, Jaeger or Honeycomb?

```bash
pip install "langsmith[otel]" opentelemetry-exporter-otlp
export LANGSMITH_OTEL_ENABLED=true
export LANGSMITH_TRACING=true
export OTEL_EXPORTER_OTLP_ENDPOINT=http://otel-collector:4318   # your collector; it can fan out to many backends
export OTEL_SERVICE_NAME=hr-assistant
export OTEL_RESOURCE_ATTRIBUTES=deployment.environment=prod,service.version=2.8.1
```

Why OTel:
- LLM spans join the **same distributed trace** as your HTTP, DB and queue spans,
- vendor neutrality,
- centralised redaction and sampling in the collector.

Alternatives: OpenInference instrumentation (Arize Phoenix), or OpenLLMetry (Traceloop).

---

## Q56. How do you integrate Langfuse (open-source) instead of LangSmith?

```python
from langfuse.langchain import CallbackHandler      # env: LANGFUSE_PUBLIC_KEY, LANGFUSE_SECRET_KEY, LANGFUSE_HOST

handler = CallbackHandler()
rag_chain.invoke(
    {"question": "notice period?"},
    config={"callbacks": [handler],
            "metadata": {"langfuse_user_id": "emp-77", "langfuse_session_id": "sess-9",
                         "langfuse_tags": ["prod", "hr"]}},
)
```

| LangSmith | Langfuse |
|---|---|
| Native, zero-code via env vars; tight integration with LangGraph Studio, datasets, evals, Prompt Hub, deployments | Open source, self-hostable, framework-agnostic |
| Managed SaaS (self-hosting on enterprise plans) | You operate it if self-hosted |

Both offer sessions, users, cost tracking, prompt management and evaluations. Choose based on data residency, budget, and the tools your stack already uses.

---

## Q57. How do you debug chains locally: `set_debug`, `set_verbose`, `astream_events`, graph visualisation?

```python
from langchain_core.globals import set_debug, set_verbose

set_debug(True)       # logs every event with full inputs/outputs (very noisy — local only)
set_verbose(True)     # lighter, human-readable logging

rag_chain.get_graph().print_ascii()          # visualise the LCEL graph
print(support_agent.get_graph().draw_mermaid())   # agents are LangGraph graphs

async for ev in rag_chain.astream_events({"question": "x"}, version="v2"):
    print(ev["event"], ev.get("name"))     # see each step's start/end and what flowed through it
```

Also useful:
- `langgraph dev` gives you **LangGraph Studio** to step through agents interactively.
- The LangSmith **Playground** re-runs a traced model call with edited prompts.

---

## Q58. How do you manage prompts and configuration as versioned artifacts?

| Option | Pros | Cons |
|---|---|---|
| Prompts in code/YAML (git) | Code review, diffs, CI evals | Non-engineers need PRs |
| **LangSmith Prompt Hub** (`client.push_prompt` / `pull_prompt("name:commit")`) | Versioned, Playground testing, links to traces | Runtime dependency (cache it locally) |
| In-house registry | Full control | You build it |

```python
from langsmith import Client
ls = Client()
ls.push_prompt("hr-rag-answer", object=rag_prompt, tags=["candidate"])          # new commit
prod_prompt = ls.pull_prompt("hr-rag-answer:prod")                             # pinned tag/commit, cache at startup
chain = prod_prompt | llm | StrOutputParser()
chain = chain.with_config(metadata={"prompt": "hr-rag-answer:prod"})           # record the version in traces
```

**Configuration checklist:**
- model id pinned to a dated snapshot,
- temperature, max_tokens, timeouts, retries,
- retriever k / thresholds,
- middleware limits,
- feature flags.

Keep it all in one typed settings object (e.g. `pydantic-settings`) and record it in trace metadata.

---

## Q59. How do you emit service-level metrics (Prometheus) from LangChain?

Traces are per request. **Metrics** drive dashboards and alerts.

```python
from prometheus_client import Counter, Histogram
from langchain_core.callbacks import BaseCallbackHandler

LLM_LAT = Histogram("llm_call_seconds", "LLM call latency", ["model", "feature"])
LLM_TOK = Counter("llm_tokens_total", "Tokens", ["model", "feature", "kind"])
LLM_ERR = Counter("llm_errors_total", "LLM errors", ["model", "feature", "error"])
TOOL_ERR = Counter("tool_errors_total", "Tool errors", ["tool"])

class PromHandler(BaseCallbackHandler):
    def __init__(self, feature: str):
        self.feature, self.start = feature, {}
    def on_chat_model_start(self, serialized, messages, *, run_id, **kw):
        self.start[run_id] = (time.perf_counter(), (kw.get("invocation_params") or {}).get("model", "unknown"))
    def on_llm_end(self, response, *, run_id, **kw):
        t, model = self.start.pop(run_id, (time.perf_counter(), "unknown"))
        LLM_LAT.labels(model, self.feature).observe(time.perf_counter() - t)
        u = response.generations[0][0].message.usage_metadata or {}
        LLM_TOK.labels(model, self.feature, "input").inc(u.get("input_tokens", 0))
        LLM_TOK.labels(model, self.feature, "output").inc(u.get("output_tokens", 0))
    def on_llm_error(self, error, *, run_id, **kw):
        _, model = self.start.pop(run_id, (0, "unknown"))
        LLM_ERR.labels(model, self.feature, type(error).__name__).inc()
    def on_tool_error(self, error, *, run_id, **kw):
        TOOL_ERR.labels(kw.get("name", "unknown")).inc()
```

**Alerts:**
- p95 latency or TTFT above the SLO,
- error rate (429/5xx),
- parse-failure rate,
- tokens or $ per hour above budget,
- call-limit hits,
- 👎 rate spikes.

---

## Q60. How do you evaluate LangChain pipelines (offline and online)?

| Level | What | How |
|---|---|---|
| Component | Retriever recall@k / MRR; extractor field accuracy; router accuracy | Labelled datasets; deterministic metrics |
| End-to-end | Correctness, faithfulness/groundedness, relevance, citation accuracy | LLM-as-judge (`openevals` / custom), human review |
| Agent | Final answer + **trajectory** (right tools, right order, no loops) | Trajectory match evaluators |
| Online | Sampled production traces scored by evaluators; feedback | LangSmith online evaluators / rules |
| Regression | Every prompt, model or retriever change | Experiments compared in CI; block on thresholds |

```python
from langsmith import Client, evaluate

ls = Client()
ds = ls.create_dataset("hr-rag-golden")
ls.create_examples(dataset_id=ds.id, examples=[
    {"inputs": {"question": "Casual leaves per year?"}, "outputs": {"answer": "12 days"}},
])

def target(inputs: dict) -> dict:
    return {"answer": rag_chain.invoke(inputs)["answer"]}

judge = init_chat_model("openai:gpt-4.1", temperature=0)

def correctness(outputs: dict, reference_outputs: dict) -> dict:
    v = judge.invoke(f"Is ANSWER consistent with REFERENCE? yes/no\nANSWER:{outputs['answer']}\n"
                     f"REFERENCE:{reference_outputs['answer']}").content.lower().startswith("yes")
    return {"key": "correct", "score": int(v)}

def has_citation(outputs: dict) -> dict:
    return {"key": "cited", "score": int("[" in outputs["answer"])}

evaluate(target, data="hr-rag-golden", evaluators=[correctness, has_citation],
         experiment_prefix="hr-rag-v4", metadata={"retriever": "hybrid+rerank", "prompt": "hr_rag@12"})
```

---

# Part D — Production Scenario Questions (Q61–Q72)

## Q61. Structured output parsing fails randomly in production (markdown fences, extra prose, renamed keys). Fix it.

1. Switch from `PydanticOutputParser` to **`with_structured_output`** (native JSON schema or tool calling), or to `ProviderStrategy` / `ToolStrategy` for agents.
2. Use `include_raw=True` to capture failures without crashing. Retry with the validation error, then **escalate** to a stronger model (Q31).
3. Add Pydantic validators for business rules, and aliases for known key drift.
4. Use temperature 0; keep schemas small, with field descriptions.
5. Monitor the parse-failure rate per model and prompt version, and dead-letter the remaining failures.

---

## Q62. Your agent loops and burns tokens. What do you do?

- `ModelCallLimitMiddleware(run_limit=…)` and `ToolCallLimitMiddleware(tool_name=…, run_limit=…)`.
- `recursion_limit` in config. **Set it explicitly**, because the LangGraph 1.2.x default is very high (10,007).
- Better tool error messages, so the model doesn't retry blindly. Return "no results for X; try a different query" instead of an empty string.
- `ContextEditingMiddleware` to clear stale tool outputs that confuse the model.
- Trace analysis: find which tool or prompt causes repetition. Add trajectory evals for that case.

---

## Q63. The agent has 60 tools and picks the wrong one, and the prompt is huge. How do you fix it?

| Fix | How |
|---|---|
| **Tool selection** | `LLMToolSelectorMiddleware(max_tools=8, always_include=[...])`, or embedding-based selection in `wrap_model_call` (`request.override(tools=subset)`) |
| **Hierarchy** | Group tools into subagents (Q44); the top agent sees about 5 delegate tools |
| **Better descriptions** | Distinct names, "use when / don't use when" in docstrings, few-shot examples |
| **Per-user filtering** | Only expose tools the user is authorised for (also a security win) |

---

## Q64. Responses are slow (8–12 s). How do you reduce latency in a LangChain pipeline?

1. **Trace first.** Find the slow spans (rewriter, retriever, re-ranker, model, tools).
2. **Stream** tokens (`astream` / agent `stream_mode="messages"`) and show progress events, to cut *perceived* latency.
3. Run independent steps concurrently with `RunnableParallel` (retrieval and guardrails at the same time).
4. Skip unnecessary calls: no query rewrite when there's no history, no retrieval for small talk (agentic RAG).
5. Smaller or faster models for routing and grading; the big model only for synthesis.
6. Fewer, better chunks (re-rank to top 3) to reduce prefill time; use provider prompt caching for static prefixes.
7. Caches (LLM, embedding, semantic); keep-alive HTTP clients; co-locate regions.

---

## Q65. Costs tripled after launch. How do you investigate and reduce them?

- **Attribute** cost using `usage_metadata` plus metadata tags (feature, tenant, prompt version) in LangSmith dashboards or Prometheus.
- Common culprits:
  - growing chat history (fix: `SummarizationMiddleware`),
  - tool outputs bloating context (fix: `ContextEditingMiddleware`, concise tools),
  - retries on non-retryable errors,
  - a big model used for simple tasks,
  - agent loops,
  - re-embedding (fix: `CacheBackedEmbeddings`, indexing API).
- Levers:
  - model routing or cascades,
  - LLM and semantic caching,
  - batch APIs for offline work,
  - output length limits,
  - fewer RAG chunks.
- Guardrails: call limits, budgets per tenant, alerts on $/hour.

---

## Q66. A provider has an outage. How do you keep serving?

- **Chains:** `.with_fallbacks([backup_chain])`, with a prompt tuned for the backup model.
- **Agents:** `ModelFallbackMiddleware(primary, backup1, backup2)` + `ModelRetryMiddleware`.
- Use a circuit breaker or AI gateway (LiteLLM, Portkey) for health-based routing across providers.
- Keep **evals for each fallback model** so quality is known.
- Degrade gracefully: cached answers, or "limited mode" messaging.

---

## Q67. Sensitive data is leaking into traces and prompts. Lock it down.

1. `PIIMiddleware` (redact, mask or block) on inputs, outputs and tool results.
2. Anonymizer on the LangSmith client, conditional tracing per tenant, or self-hosted tracing (Q54).
3. ACL filtering **inside** retrievers; tenant-scoped vector namespaces.
4. Least-privilege tools with authorisation checks using `ToolRuntime.context`.
5. Prompt-injection defences: treat retrieved and tool content as data, review MCP tool descriptions, keep secrets out of prompts.
6. Retention: TTLs on checkpoints and store data; encrypted checkpoint serialisation.

---

## Q68. You must upgrade from LangChain 0.x to 1.x. What's your plan?

1. Inventory the APIs you use: `AgentExecutor`, `initialize_agent`, `LLMChain`, `RetrievalQA`, memory classes, community integrations.
2. Pin versions and capture **baseline evals and traces**.
3. Quick compatibility: install `langchain-classic` and update imports.
4. Migrate agents to `create_agent`, replacing custom callbacks/hacks with middleware (summarisation, HITL, retries, limits).
5. Replace `langchain-community` imports with standalone integration packages where available.
6. Re-run evals, compare experiments in LangSmith, then roll out behind a feature flag or canary.

---

## Q69. How do you unit-test LangChain components without calling real models?

```python
from langchain_core.language_models.fake_chat_models import GenericFakeChatModel, FakeListChatModel
from langchain_core.embeddings import DeterministicFakeEmbedding
from langchain_core.vectorstores import InMemoryVectorStore
from langchain_core.documents import Document

def test_rag_chain_formats_citations():
    fake_llm = FakeListChatModel(responses=["The policy gives 12 days [1]."])
    vs = InMemoryVectorStore.from_documents([Document("12 casual leaves/year", metadata={"source": "leave.pdf"})],
                                            DeterministicFakeEmbedding(size=32))
    chain = (RunnablePassthrough.assign(context=itemgetter("question") | vs.as_retriever(k=1) | format_docs)
             | rag_prompt | fake_llm | StrOutputParser())
    out = chain.invoke({"question": "casual leaves?"})
    assert "[1]" in out

def test_router_picks_support():
    fake_route = RunnableLambda(lambda x: Route(destination="support", confidence=0.9))
    pipeline = RunnablePassthrough.assign(route=fake_route) | router
    assert pipeline.invoke({"email": "my order broke"})       # hits support_chain (mock its LLM too)
```

Test tools as plain functions (`tool.invoke({...})`). Use `LLMToolEmulator` middleware to emulate tools in agent tests. Run LLM-backed evals separately (Q60).

---

## Q70. When should you NOT use LangChain?

- A single provider call with a simple prompt. The provider SDK is enough.
- Teams that want minimal dependencies or full control of HTTP calls in latency-critical paths.
- Complex, long-running, durable multi-agent workflows: use **LangGraph** directly (still in the ecosystem) or a workflow engine like Temporal.

**Trade-off:** LangChain gives standard interfaces, integrations, middleware and observability, at the cost of abstraction layers, version churn and dependency weight.

---

## Q71. Compare LangChain with LlamaIndex, Haystack and raw SDKs.

| Option | Strengths | When |
|---|---|---|
| **LangChain** | Broad integrations, LCEL composition, `create_agent` + middleware, LangGraph/LangSmith ecosystem | General LLM apps and agents |
| LlamaIndex | Data ingestion, indexing and advanced RAG; workflows | RAG-heavy apps over complex data |
| Haystack | Pipeline-centric, production RAG, strong in search | Search/RAG pipelines, enterprise search |
| Provider SDKs (OpenAI Agents SDK, Anthropic SDK) | Thin, fast, provider-native features first | Single-provider apps, simple agents |
| DSPy | Programmatic prompt optimisation | Optimising prompts or pipelines against metrics |

---

## Q72. Walk through a production LangChain architecture end to end (HR assistant).

```
Web/Slack ─▶ FastAPI (SSO auth, rate limit, request_id) ─SSE─▶ client
                │ config: thread_id, run_id, tags, metadata(tenant, user, versions); context(user, groups)
                ▼
     create_agent(model=routed via wrap_model_call,
                  tools=[search_hr (ACL retriever, hybrid+rerank), search_it, create_ticket],
                  middleware=[PIIMiddleware, compliance_gate, SummarizationMiddleware,
                              ToolCallLimitMiddleware, ModelFallbackMiddleware,
                              HumanInTheLoopMiddleware(create_ticket)],
                  checkpointer=PostgresSaver, store=PostgresStore)
                │
   Ingestion: loaders ▶ header-aware splitter ▶ CacheBackedEmbeddings ▶ index(RecordManager, cleanup="full") ▶ PGVector
                │
   Observability: LangSmith traces (sampled 20%, anonymized, always-on for errors/debug)
                  OTel ▶ Datadog (joined with API spans) · Prometheus (latency, tokens, errors, $)
                  Feedback ▶ datasets ▶ CI evals (correctness, faithfulness, trajectory) ▶ canary release
```

---

# Part E — Rapid-Fire Questions & Cheat Sheet

| # | Question | Short answer |
|---|---|---|
| 1 | What does `a \| b` create? | A `RunnableSequence` |
| 2 | Run several runnables on the same input concurrently? | `RunnableParallel` (or a dict literal) |
| 3 | Add a key to the input dict while keeping the others? | `RunnablePassthrough.assign(key=...)` |
| 4 | Wrap a Python function as a runnable? | `RunnableLambda` or the `@chain` decorator |
| 5 | if/else routing in LCEL? | `RunnableBranch`, or a function returning a runnable |
| 6 | Most reliable structured output? | `model.with_structured_output(Schema)` |
| 7 | Structured final answer from an agent? | `response_format=ToolStrategy(...)` / `ProviderStrategy(...)` |
| 8 | Standard agent in 1.x? | `langchain.agents.create_agent` |
| 9 | Customise an agent's prompt, model or tools per request? | Middleware: `@dynamic_prompt`, `@wrap_model_call` |
| 10 | Summarise long chats automatically? | `SummarizationMiddleware` |
| 11 | Require approval before a tool runs? | `HumanInTheLoopMiddleware` + checkpointer; resume with `Command(resume={"decisions": [...]})` |
| 12 | Redact PII before the model sees it? | `PIIMiddleware(..., strategy="redact"/"mask"/"hash"/"block")` |
| 13 | Stop runaway agents? | `ModelCallLimitMiddleware`, `ToolCallLimitMiddleware`, `recursion_limit` |
| 14 | Retries and fallbacks on a chain? | `.with_retry(...)`, `.with_fallbacks([...])` |
| 15 | Rate-limit model calls? | `InMemoryRateLimiter` passed as `rate_limiter=` |
| 16 | Global LLM response cache? | `set_llm_cache(InMemoryCache() / SQLiteCache(...))` |
| 17 | Avoid re-embedding unchanged text? | `CacheBackedEmbeddings` (`langchain-classic`) |
| 18 | Keep a vector store in sync with sources? | `index()` + `RecordManager`, `cleanup="incremental"/"full"` |
| 19 | Hybrid search? | `EnsembleRetriever([BM25Retriever, dense])` or native DB hybrid |
| 20 | Diverse results? | `as_retriever(search_type="mmr")` |
| 21 | Hide injected args (user_id, store) from the model? | `ToolRuntime` parameter in the tool |
| 22 | Return docs from a tool without stuffing them in the message? | `@tool(response_format="content_and_artifact")` |
| 23 | Stream every step's events? | `astream_events(version="v2")` |
| 24 | Total tokens for a request? | `get_usage_metadata_callback()` |
| 25 | Enable LangSmith? | `LANGSMITH_TRACING=true`, `LANGSMITH_API_KEY`, `LANGSMITH_PROJECT` |
| 26 | Reduce trace volume? | `LANGSMITH_TRACING_SAMPLING_RATE` or `Client(tracing_sampling_rate=...)` |
| 27 | Turn tracing off for one request? | `with tracing_context(enabled=False): ...` |
| 28 | Mask PII in traces? | `create_anonymizer([...])` → `Client(anonymizer=...)` → `LangChainTracer` |
| 29 | Export to OTel backends? | `LANGSMITH_OTEL_ENABLED=true` + `OTEL_EXPORTER_OTLP_ENDPOINT` |
| 30 | Trace plain Python functions? | `@traceable(run_type=...)` |
| 31 | Callbacks at request time vs constructor? | Request-time propagates to children; constructor is scoped to that object |
| 32 | Where did `RetrievalQA` / `AgentExecutor` go? | `langchain-classic`; replace with LCEL / `create_agent` |
| 33 | Swap models at runtime without rebuilding the chain? | `configurable_alternatives` / `init_chat_model(configurable_fields=...)` |
| 34 | Use MCP servers as tools? | `langchain-mcp-adapters` → `MultiServerMCPClient.get_tools()` |
| 35 | Test without real LLM calls? | `FakeListChatModel`, `GenericFakeChatModel`, `DeterministicFakeEmbedding` |

### Concept checklist (make sure you can explain each one)

Package layout (core / langchain / integrations / classic / community sunset) · `init_chat_model` · messages and content blocks · `usage_metadata` · prompt templates, `MessagesPlaceholder`, partials, few-shot and example selectors · LCEL and the Runnable interface (invoke/batch/stream/async/astream_events) · Sequence/Parallel/Passthrough/assign/Lambda/Branch/`@chain` · output parsers vs `with_structured_output` · tools (`@tool`, args_schema, `ToolException`, `ToolRuntime`, content_and_artifact) · `bind_tools` and the tool-calling loop · `create_agent` · middleware hooks (before/after agent/model, wrap model/tool, dynamic prompt) · built-in middleware (summarisation, HITL, PII, call limits, retries, fallbacks, tool selection, context editing, todo, router) · `response_format` (Tool/Provider strategy) · memory (checkpointer, store, `RunnableWithMessageHistory`, `trim_messages`) · loaders · splitters · embeddings and caching · vector stores and retrievers (MMR, threshold, BM25, ensemble, multi-query, parent document, compression, self-query, custom) · indexing API and RecordManager · retries, fallbacks, rate limiters, LLM cache · `RunnableConfig` · configurable fields and alternatives · streaming · MCP adapters · multimodal input · legacy-chain migration · callbacks and custom handlers · LangSmith env vars, tags, metadata, run names, feedback, `@traceable`, sampling, conditional tracing, anonymizers · OpenTelemetry · Langfuse · Prometheus metrics · `set_debug` · Prompt Hub · evaluation (component, end-to-end, trajectory, online) · production concerns (latency, cost, reliability, security, upgrades, testing).
