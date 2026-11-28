# LangGraph Interview Question Set — Concepts, Pipelines, Use Cases & Observability

> A detailed interview guide covering every core LangGraph concept, how to design different kinds of pipelines (with real use cases), and observability and production configuration. Each answer explains the idea, the trade-offs and an example, with Python where code helps.
>
> **Version note:** Written against **LangGraph 1.x** (1.0 went GA in October 2025, and 1.0 kept the core graph APIs unchanged from 0.x). In 1.0 `langgraph.prebuilt.create_react_agent` was deprecated in favour of `langchain.agents.create_agent`. Features marked **(1.2+)** are newer. The core snippets (interrupts, `Send`, deferred nodes, caching, retries, durability, `context_schema`, custom streaming) and the 1.2 imports (`error_handler`, `TimeoutPolicy`, `NodeError`, `DeltaChannel`, `RunControl`) were checked against **langgraph 1.2.24**. Snippets that call external services (LLMs, databases, LangSmith) were not executed, so check them against your installed versions.

---

## Table of Contents

- [Part A — Core Concepts (Q1–Q24)](#part-a--core-concepts-q1q24)
- [Part B — Pipeline Patterns & Use Cases (Q25–Q42)](#part-b--pipeline-patterns--use-cases-q25q42)
- [Part C — Observability & Configuration (Q43–Q55)](#part-c--observability--configuration-q43q55)
- [Part D — Production Scenario Questions (Q56–Q68)](#part-d--production-scenario-questions-q56q68)
- [Part E — Rapid-Fire Questions & Cheat Sheet](#part-e--rapid-fire-questions--cheat-sheet)

### Setup used in the snippets

```bash
pip install -U langgraph langchain langchain-openai langgraph-checkpoint-postgres langsmith
```

```python
# common.py — shared imports for the snippets in this guide
from typing import Annotated, Literal, TypedDict
import operator, json, uuid
from pydantic import BaseModel, Field
from langchain_core.messages import HumanMessage, AIMessage, ToolMessage, RemoveMessage
from langchain.chat_models import init_chat_model
from langgraph.graph import StateGraph, START, END, MessagesState
from langgraph.graph.message import add_messages
from langgraph.types import Command, Send, interrupt, RetryPolicy, CachePolicy
from langgraph.checkpoint.memory import InMemorySaver

llm = init_chat_model("openai:gpt-4.1-mini", temperature=0)   # any provider string works
```

---

# Part A — Core Concepts (Q1–Q24)

## Q1. What is LangGraph, and when would you choose it over a plain LangChain chain or a high-level agent?

**Answer.** LangGraph is a low-level orchestration framework and runtime for building **stateful, long-running, multi-step LLM applications as graphs**. You define:
- a **State** (shared data structure),
- **Nodes** (functions that read state and return updates),
- **Edges** (fixed or conditional transitions, including cycles).

The runtime adds **persistence (checkpointing)**, **human-in-the-loop interrupts**, **streaming**, **durable execution**, **memory**, and **time travel**.

| Choose | When |
|---|---|
| A plain LLM call / LCEL chain | One-shot, linear tasks (prompt → model → parser) with no loops or state |
| `create_agent` (LangChain 1.x, built on LangGraph) | A standard tool-calling agent loop is enough; customise via middleware |
| **LangGraph (Graph API)** | You need explicit control flow such as loops, branches, parallelism, approvals, multi-agent coordination, resumability, or deterministic steps mixed with LLM steps |
| LangGraph **Functional API** | You want LangGraph's persistence/interrupts but prefer ordinary Python control flow (`if`/`for`) to drawing a graph |

**Interview angle:** LangGraph is not an agent abstraction. It is the runtime that agents run on. It lets you decide which parts are deterministic (workflows) and which are LLM-driven (agents).

---

## Q2. Explain the difference between a *workflow* and an *agent* in LangGraph terms.

| | Workflow | Agent |
|---|---|---|
| Control flow | Predefined by the developer (edges are fixed or rule-based) | Decided by the LLM at runtime (which tool or node next, when to stop) |
| Predictability | High; easy to test | Lower; needs loop limits and evals |
| Examples | Prompt chaining, routing, parallelisation, map-reduce, evaluator-optimizer | ReAct tool loop, supervisor multi-agent, handoffs |
| In LangGraph | Normal edges plus conditional edges driven by code | Conditional edges driven by the model's tool calls, or `Command(goto=...)` chosen by the LLM |

Most production systems are **hybrids**: a deterministic workflow skeleton with agentic sub-steps.

---

## Q3. What is *State* in LangGraph, and how do you define it?

State is the shared memory every node reads from and writes to. You define it as a schema:

| Schema type | Pros | Cons |
|---|---|---|
| `TypedDict` | Lightweight, fastest, most common | No runtime validation |
| `dataclass` | Default values | No validation |
| Pydantic `BaseModel` | Runtime validation of **inputs** | Slower; output is still a dict-like state; validation runs on node inputs only |

```python
class ArticleState(TypedDict):
    topic: str
    outline: str
    draft: str
    review_notes: Annotated[list[str], operator.add]   # reducer: append instead of overwrite
    revision: int
```

**Key rule:** nodes **return partial updates** (a dict with only the keys they change). They don't mutate state in place.

---

## Q4. What are *reducers*, and why do they matter? What happens without one?

A reducer defines **how an update merges into the existing value** of a state key: `new_value = reducer(old_value, update)`.

- **No reducer (default):** last write wins, so the value is overwritten.
- `operator.add`: concatenates lists or adds numbers.
- `add_messages`: the message reducer. It appends new messages, **updates in place by message ID**, and handles `RemoveMessage`.
- **Custom reducer:** any `(old, new) -> merged` function.

**Why it matters:** if two **parallel** nodes write the same key in the same step without a reducer, LangGraph raises `InvalidUpdateError`, because it can't tell which write should win.

```python
def merge_dicts(left: dict, right: dict) -> dict:
    return {**(left or {}), **(right or {})}

def keep_top_k(left: list, right: list, k: int = 5) -> list:
    return sorted((left or []) + (right or []), key=lambda d: -d["score"])[:k]

class ResearchState(TypedDict):
    question: str
    findings: Annotated[dict, merge_dicts]          # parallel workers each add a key
    top_docs: Annotated[list[dict], keep_top_k]      # keep only the best 5 across workers
    messages: Annotated[list, add_messages]
```

**Follow-up:** "How do you *reset* a list that has an `operator.add` reducer?" Use a custom reducer that treats a sentinel such as `None` or `{"__reset__": True}` as "clear". For messages, return `RemoveMessage(id=...)` (or `RemoveMessage(id=REMOVE_ALL_MESSAGES)`).

---

## Q5. What is `MessagesState`, and what does `add_messages` do exactly?

`MessagesState` is a prebuilt state with one key, `messages: Annotated[list[AnyMessage], add_messages]`. You usually subclass it.

`add_messages`:
1. Converts dicts or tuples to `BaseMessage` objects.
2. **Appends** messages with new IDs.
3. **Replaces** an existing message when the update has the **same ID** (useful for editing a tool call before approval).
4. **Deletes** messages when given `RemoveMessage(id=...)`.

```python
from langchain_core.messages import HumanMessage, RemoveMessage

class ChatState(MessagesState):
    user_id: str
    summary: str

def trim_old(state: ChatState):
    # delete all but the last 6 messages from persisted state
    return {"messages": [RemoveMessage(id=m.id) for m in state["messages"][:-6]]}
```

---

## Q6. What are nodes, and what can a node function receive and return?

A node is a function or Runnable (sync or async). Its signature can include:

| Parameter | Purpose |
|---|---|
| `state` | The current state (or the node's declared input schema) |
| `config: RunnableConfig` | thread_id, tags, metadata, callbacks, recursion_limit |
| `runtime: Runtime[Context]` | Run-scoped **context** (user_id, db connection, model name), `store`, `stream_writer` |

A node can return:
- a **dict** of partial state updates,
- a **`Command`** (update **and** route in one return),
- **`None`** (no update).

```python
from dataclasses import dataclass
from langgraph.runtime import Runtime

@dataclass
class Ctx:
    user_id: str
    model_name: str = "openai:gpt-4.1-mini"

def answer(state: MessagesState, runtime: Runtime[Ctx]):
    model = init_chat_model(runtime.context.model_name)
    prefs = runtime.store.get(("prefs", runtime.context.user_id), "style") if runtime.store else None
    sys = f"User style preference: {prefs.value if prefs else 'default'}"
    return {"messages": [model.invoke([{"role": "system", "content": sys}, *state["messages"]])]}

builder = StateGraph(MessagesState, context_schema=Ctx)
builder.add_node("answer", answer)
builder.add_edge(START, "answer")
graph = builder.compile()
graph.invoke({"messages": [("user", "hi")]}, context={"user_id": "u-42"})
```

**Context vs State vs config:**
- **State** is data that evolves during the run and is checkpointed.
- **Context** is immutable run-scoped dependencies (who the user is, which model, a DB client). It is not checkpointed.
- **config** holds framework settings (thread_id, callbacks, tags).

Older code put dependencies in `config["configurable"]`. Newer code uses `context_schema` and `Runtime`.

---

## Q7. Explain the types of edges: normal, conditional, entry/finish, and `Command`-based routing.

| Edge type | API | Use |
|---|---|---|
| Normal | `add_edge("a", "b")` | Always go a → b |
| Entry | `add_edge(START, "a")` | First node |
| Finish | `add_edge("z", END)` | Stop |
| Fan-out | multiple `add_edge("a", x)` | Run x, y, z in parallel next step |
| Fan-in | `add_edge(["x", "y"], "join")` | `join` waits for **both** x and y |
| Conditional | `add_conditional_edges("a", router_fn, path_map)` | Branch on state |
| Dynamic fan-out | conditional edge returning `[Send(...), ...]` | Map-reduce over a runtime-sized list |
| `Command(goto=...)` | returned from a node | Node decides where to go **and** updates state at once |
| Sequence helper | `add_sequence([a, b, c])` | Shorthand for a linear chain |

```python
def route(state) -> Literal["billing", "tech", "general"]:
    return state["category"]

builder.add_conditional_edges("classify", route, {"billing": "billing_agent",
                                                  "tech": "tech_agent",
                                                  "general": "faq"})
```

**Conditional edge vs `Command`:**
- Use a **conditional edge** when routing logic is separate from the work. It is cleaner, and the graph structure is visible at compile time.
- Use **`Command`** when the same node computes the update and the destination, for example an LLM supervisor choosing the next agent or an agent handoff. Annotate the return type `-> Command[Literal["a", "b"]]` so the graph can be rendered.

---

## Q8. How does LangGraph execute a graph? Explain *supersteps* and the Pregel model.

LangGraph's runtime is based on **Pregel / Bulk Synchronous Parallel**:
1. Execution proceeds in **supersteps**.
2. In each superstep, every node scheduled to run executes **in parallel**, all reading the same state snapshot.
3. Their updates are applied **together** through reducers at the end of the step.
4. Edges determine which nodes run in the next superstep.
5. A **checkpoint** is saved after each superstep (when a checkpointer is attached).
6. Execution ends when no nodes are scheduled, or it hits the `recursion_limit`.

**Consequences interviewers probe:**
- Parallel nodes **can't see each other's writes** within the same superstep.
- Parallel writes to the same key need a reducer (otherwise `InvalidUpdateError`).
- If one parallel branch fails, the superstep fails. With a checkpointer, writes from the **successful** sibling nodes are saved as *pending writes*, so they don't re-run on resume.
- **Branches of unequal length:** with `a → [b, c]`, `b → d`, `c → c2 → d`, node `d` runs **twice** (once per arriving branch in different supersteps) unless you use `add_edge(["b", "c2"], "d")` or mark `d` as `defer=True`.

---

## Q9. What does `compile()` do, and what can you pass to it?

`compile()` validates the graph (no orphan nodes, valid edges) and returns a runnable `CompiledStateGraph` that supports `invoke`, `stream`, `batch` and their async versions. Options:

| Argument | Purpose |
|---|---|
| `checkpointer` | Persistence (threads, HITL, time travel, fault tolerance) |
| `store` | Long-term cross-thread memory |
| `cache` | Node-level result cache (used with `cache_policy`) |
| `interrupt_before` / `interrupt_after` | Static breakpoints (mainly for debugging) |
| `name` | Graph name (shows in traces; important for subgraphs and multi-agent) |
| `debug` | Verbose execution logs |

---

## Q10. What is a checkpointer, what is a *thread*, and which checkpointers exist?

A **checkpointer** saves a snapshot of state (a `StateSnapshot`) after every superstep, keyed by **`thread_id`**. A **thread** is one conversation or run history.

What checkpointing gives you:
- multi-turn memory (short-term memory),
- human-in-the-loop (pause and resume),
- time travel (replay or fork from any checkpoint),
- fault tolerance (resume after a crash from the last superstep).

| Checkpointer | Package | Use |
|---|---|---|
| `InMemorySaver` | `langgraph` | Tests, notebooks; lost on restart |
| `SqliteSaver` / `AsyncSqliteSaver` | `langgraph-checkpoint-sqlite` | Local apps, single process |
| `PostgresSaver` / `AsyncPostgresSaver` | `langgraph-checkpoint-postgres` | Production standard |
| Redis, MongoDB, other community or cloud savers | various | Existing infra |
| Managed persistence | LangSmith Deployment (formerly LangGraph Platform) | No setup |

```python
from langgraph.checkpoint.postgres import PostgresSaver

DB_URI = "postgresql://user:pass@db:5432/agents"
with PostgresSaver.from_conn_string(DB_URI) as checkpointer:
    checkpointer.setup()                         # creates tables once
    graph = builder.compile(checkpointer=checkpointer)
    cfg = {"configurable": {"thread_id": "customer-123:session-9"}}
    graph.invoke({"messages": [("user", "My order is late")]}, cfg)
    graph.invoke({"messages": [("user", "It's order #1766")]}, cfg)   # remembers turn 1
```

---

## Q11. What are *durability modes*, and how do you choose one?

Durability controls **when checkpoints are written**. Pass it per invocation, for example `graph.invoke(inp, cfg, durability="sync")`.

| Mode | Behaviour | Trade-off |
|---|---|---|
| `"exit"` | Persist only when the graph exits (success, error, or interrupt) | Fastest; intermediate steps can't be recovered after a crash mid-run |
| `"async"` | Persist asynchronously while the next step runs | Good balance (default); a tiny window where a crash may lose the latest step |
| `"sync"` | Persist synchronously before the next step starts | Highest durability; adds latency per step |

**Use case mapping:**
- Short chat turns: `async`.
- Long, expensive batch pipelines (ETL with paid API calls): `sync`.
- High-throughput stateless classification: `exit`, or no checkpointer at all.

---

## Q12. Explain human-in-the-loop with `interrupt()` and `Command(resume=...)`.

`interrupt(value)` **pauses** the graph inside a node, saves a checkpoint, and surfaces `value` to the caller (under the `__interrupt__` key of the result or stream). The caller resumes with `Command(resume=answer)`, and `interrupt()` **returns that answer** inside the node.

Requirements: a **checkpointer** and the same **thread_id** on resume.

```python
class RefundState(TypedDict):
    order_id: str
    amount: float
    decision: str

def propose(state: RefundState):
    return {"amount": 4999.0}

def approval(state: RefundState) -> Command[Literal["execute", "reject"]]:
    answer = interrupt({"question": "Approve refund?", "order": state["order_id"],
                        "amount": state["amount"]})
    if answer["approved"]:
        return Command(goto="execute", update={"amount": answer.get("amount", state["amount"])})
    return Command(goto="reject", update={"decision": "rejected"})

def execute(state): return {"decision": "refunded"}
def reject(state): return {}

b = StateGraph(RefundState)
b.add_sequence([propose, approval])
b.add_node(execute); b.add_node(reject)
b.add_edge(START, "propose"); b.add_edge("execute", END); b.add_edge("reject", END)
g = b.compile(checkpointer=InMemorySaver())

cfg = {"configurable": {"thread_id": "refund-1766"}}
out = g.invoke({"order_id": "1766"}, cfg)
print(out["__interrupt__"])                        # show this to the human reviewer
g.invoke(Command(resume={"approved": True, "amount": 3000}), cfg)   # resumes inside approval()
```

**Gotchas interviewers love:**
1. **The node re-executes from the top on resume.** Code before `interrupt()` runs again, so keep side effects after the interrupt or make them idempotent.
2. **Multiple interrupts in one node are matched by order.** Don't put `interrupt()` calls inside conditionals whose order can change between runs.
3. Don't wrap `interrupt()` in a bare `try/except Exception`. It works by raising a special exception (`GraphInterrupt`).
4. Interrupts in **parallel** branches surface together. You can resume several at once by passing a dict of `{interrupt_id: value}` to `Command(resume=...)`.

Static breakpoints (`interrupt_before=["tool_node"]`) also exist, but they are mainly for debugging. `interrupt()` is the production pattern.

---

## Q13. What is time travel, and how is it implemented?

Every checkpoint has an ID. You can:
- **inspect** with `graph.get_state(cfg)` and `graph.get_state_history(cfg)`,
- **replay** from a past checkpoint: `invoke(None, {"configurable": {"thread_id": t, "checkpoint_id": cid}})` re-executes the steps after it,
- **fork** with `graph.update_state(cfg_at_checkpoint, new_values, as_node="x")`, which creates a new branch from that point with modified state.

```python
cfg = {"configurable": {"thread_id": "t1"}}
history = list(graph.get_state_history(cfg))          # newest first
before_tool = next(s for s in history if s.next == ("tools",))
print(before_tool.values["messages"][-1].tool_calls)

# Fork: edit the tool call arguments, then continue from there
msg = before_tool.values["messages"][-1]
msg.tool_calls[0]["args"]["city"] = "Pune"
new_cfg = graph.update_state(before_tool.config, {"messages": [msg]})   # same id → replaced
graph.invoke(None, new_cfg)
```

**Use cases:** debugging a bad agent decision, "what-if" analysis, letting users edit a previous message and regenerate, and reproducing production incidents locally.

---

## Q14. Short-term vs long-term memory: checkpointer vs Store.

| | Short-term (thread) memory | Long-term memory |
|---|---|---|
| Mechanism | **Checkpointer** | **Store** (`BaseStore`: `InMemoryStore`, `PostgresStore`, managed store) |
| Scope | One `thread_id` | Across threads and sessions, organised by **namespace** tuples, e.g. `("memories", user_id)` |
| Content | Full graph state (messages, intermediate data) | JSON documents: user preferences, facts, past summaries, procedures |
| Search | By thread | Key lookup, filters, and **semantic search** if you configure an embedding index |
| Typical size | Grows with conversation (needs trimming or summarisation) | Curated, small |

```python
from langgraph.store.memory import InMemoryStore
from langchain.embeddings import init_embeddings

store = InMemoryStore(index={"embed": init_embeddings("openai:text-embedding-3-small"), "dims": 1536})

def remember(state: MessagesState, runtime: Runtime[Ctx]):
    ns = ("memories", runtime.context.user_id)
    last = state["messages"][-1].content
    if "i prefer" in last.lower():
        runtime.store.put(ns, key=str(hash(last)), value={"fact": last})
    hits = runtime.store.search(ns, query=last, limit=3)            # semantic recall
    facts = "\n".join(h.value["fact"] for h in hits)
    reply = llm.invoke([{"role": "system", "content": f"Known about user:\n{facts}"}, *state["messages"]])
    return {"messages": [reply]}

b = StateGraph(MessagesState, context_schema=Ctx)
b.add_node(remember); b.add_edge(START, "remember")
g = b.compile(checkpointer=InMemorySaver(), store=store)
```

Memory types to mention:
- **Semantic** memory: facts about the user.
- **Episodic** memory: past experiences or few-shot examples.
- **Procedural** memory: updated instructions or prompts.

You can write memories **in the hot path** (simple, adds latency) or **in the background** (a separate graph or job; no added latency, but eventually consistent).

---

## Q15. What streaming modes does LangGraph support, and when do you use each?

| `stream_mode` | Emits | Use case |
|---|---|---|
| `"values"` | Full state after each step | Debugging, simple UIs |
| `"updates"` | Only the delta each node returned (`{node_name: update}`) | Progress UIs ("Searching… Grading… Writing…") |
| `"messages"` | `(token_chunk, metadata)` for every LLM call **inside any node** | Token-by-token chat streaming |
| `"custom"` | Anything you emit via `get_stream_writer()` / `runtime.stream_writer` | Tool progress ("Fetched 30/120 pages"), partial results |
| `"debug"` | Detailed task and checkpoint events | Deep debugging |
| `"checkpoints"` / `"tasks"` | Checkpoint and task lifecycle events | Building custom monitors |

```python
from langgraph.config import get_stream_writer

def crawl(state):
    w = get_stream_writer()
    for i, url in enumerate(state["urls"], 1):
        ...  # fetch
        w({"progress": f"{i}/{len(state['urls'])}", "url": url})
    return {"pages": ["..."]}

for mode, chunk in graph.stream(inputs, cfg, stream_mode=["updates", "messages", "custom"]):
    if mode == "messages":
        token, meta = chunk
        if meta["langgraph_node"] == "writer":      # only stream the final writer node's tokens
            print(token.content, end="")
    elif mode == "custom":
        print("\n[progress]", chunk)
    elif mode == "updates":
        print("\n[step done]", list(chunk))
```

Add `subgraphs=True` to receive events from nested graphs. Chunks then come as `(namespace, mode, data)`. For an event-level API, `astream_events` is also available. **(1.2+)** adds `version="v3"` (beta) with a content-block-centric protocol.

---

## Q16. What are subgraphs, and what are the two ways to use them?

A subgraph is a compiled graph used as a node of another graph. You use them for **encapsulation, reuse, team ownership, and multi-agent systems**.

| Method | When | How state flows |
|---|---|---|
| **Add the compiled subgraph directly as a node** | Parent and child **share state keys** (e.g. both use `messages`) | Shared keys pass through automatically |
| **Call the subgraph inside a wrapper node** | **Different schemas** (the child has private state) | You transform parent state → child input and child output → parent update |

```python
# Child: a research subgraph with its own private keys
class ResearchState(TypedDict):
    query: str
    docs: list[str]
    summary: str

def search(s): return {"docs": [f"doc about {s['query']}"]}
def summarize(s): return {"summary": llm.invoke(f"Summarise: {s['docs']}").content}

rb = StateGraph(ResearchState)
rb.add_sequence([search, summarize]); rb.add_edge(START, "search")
research_graph = rb.compile(name="research")

# Parent with a different schema → wrapper node transforms state
class ReportState(TypedDict):
    topic: str
    sections: Annotated[list[str], operator.add]

def research_section(state: ReportState):
    out = research_graph.invoke({"query": state["topic"]})
    return {"sections": [out["summary"]]}

pb = StateGraph(ReportState)
pb.add_node(research_section); pb.add_edge(START, "research_section")
report_graph = pb.compile(checkpointer=InMemorySaver())   # the parent's checkpointer propagates to children
```

**Persistence:** subgraphs inherit the parent's checkpointer by default. Compile a subgraph with `checkpointer=True` when it needs its **own persistent memory across invocations** (e.g. each agent in a multi-agent system keeps its own message history).

**Navigating up:** a node inside a subgraph can return `Command(goto="other_agent", graph=Command.PARENT)` to hand control to a sibling node in the parent graph.

---

## Q17. What is `Send`, and how does it enable map-reduce?

`Send(node_name, payload)` schedules a node invocation with a **custom input** (not the full state). Returning a list of `Send`s from a conditional edge fans out **N parallel tasks**, where N is known only at runtime. Results are merged with a **reducer**.

```python
class MapState(TypedDict):
    docs: list[str]
    summaries: Annotated[list[str], operator.add]
    final: str

class DocInput(TypedDict):
    doc: str

def fan_out(state: MapState):
    return [Send("summarize_one", {"doc": d}) for d in state["docs"]]

def summarize_one(inp: DocInput):
    return {"summaries": [llm.invoke(f"Summarise in 2 lines:\n{inp['doc']}").content]}

def reduce(state: MapState):
    return {"final": llm.invoke("Combine these summaries:\n" + "\n".join(state["summaries"])).content}

b = StateGraph(MapState)
b.add_node(summarize_one); b.add_node(reduce)
b.add_conditional_edges(START, fan_out, ["summarize_one"])
b.add_edge("summarize_one", "reduce"); b.add_edge("reduce", END)
graph = b.compile()
```

Control concurrency with `config={"max_concurrency": 10}` to respect provider rate limits.

---

## Q18. What is `Command`, and what are its fields?

`Command` combines **state update + control flow** (and resume values):

| Field | Meaning |
|---|---|
| `update` | State update to apply |
| `goto` | Next node(s), or `Send` objects |
| `graph` | Target graph (`Command.PARENT` routes in the parent from inside a subgraph) |
| `resume` | Value to resume an `interrupt()` with (used as input to `invoke`/`stream`) |

```python
def supervisor(state: MessagesState) -> Command[Literal["researcher", "coder", "__end__"]]:
    decision = llm.with_structured_output(
        {"type": "object", "properties": {"next": {"enum": ["researcher", "coder", "FINISH"]}},
         "required": ["next"]}).invoke(state["messages"])
    nxt = decision["next"]
    return Command(goto=END if nxt == "FINISH" else nxt, update={"messages": []})
```

**Interview tip:** `Command` adds **dynamic** edges. Static edges from that node still apply, so don't combine a `Command`-returning node with a conflicting `add_edge` from the same node.

---

## Q19. How do you handle errors and retries in LangGraph?

| Mechanism | Use |
|---|---|
| `RetryPolicy` on a node (`max_attempts`, `initial_interval`, `backoff_factor`, `max_interval`, `jitter`, `retry_on`) | Transient failures: rate limits, 5xx, timeouts |
| try/except inside the node, writing the error into state | Let the graph route to a fallback or repair node |
| Error-aware conditional edges | e.g. "SQL failed → regenerate query" loops |
| **(1.2+)** `error_handler=` on `add_node` | Runs after retries are exhausted; receives a `NodeError` and can return a `Command` to compensate and reroute (Saga pattern) |
| **(1.2+)** `timeout=TimeoutPolicy(run_timeout=..., idle_timeout=...)` | Per-node timeouts (async nodes) |
| Checkpointer | Resume from the last successful superstep after a crash; successful parallel siblings are not re-run |
| `recursion_limit` | Bounds runaway loops (`GraphRecursionError`) |

```python
import httpx

def call_api(state):
    r = httpx.get("https://api.example.com/rates", timeout=5)
    r.raise_for_status()
    return {"rates": r.json()}

builder.add_node("call_api", call_api,
                 retry_policy=RetryPolicy(max_attempts=4, initial_interval=0.5, backoff_factor=2.0,
                                          retry_on=(httpx.TimeoutException, httpx.HTTPStatusError)))

def run_sql(state):
    try:
        return {"rows": db.execute(state["sql"]), "error": None}
    except Exception as e:                          # recoverable: feed the error back to the LLM
        return {"error": str(e), "attempts": state.get("attempts", 0) + 1}

def after_sql(state) -> Literal["write_sql", "answer", "give_up"]:
    if state["error"] is None: return "answer"
    return "write_sql" if state["attempts"] < 3 else "give_up"
```

---

## Q20. What is node caching (`CachePolicy`), and when is it useful?

Node-level caching skips re-executing a node when its **input is identical** to a previous call, within a TTL.

```python
from langgraph.cache.memory import InMemoryCache

builder.add_node("embed_docs", embed_docs, cache_policy=CachePolicy(ttl=3600))
graph = builder.compile(cache=InMemoryCache())
```

| Pros | Cons |
|---|---|
| Saves cost and latency for deterministic, expensive nodes (embeddings, web fetches, heavy computations) | Wrong for non-deterministic or time-sensitive nodes; key is based on input, so large inputs mean large keys; in-memory cache isn't shared across replicas |

---

## Q21. What are deferred nodes (`defer=True`)?

A deferred node waits until **all pending branches have completed** before running, even when branches have different lengths. It is useful for fan-in after asymmetric parallel paths, or after map-reduce where the number of tasks varies.

```python
builder.add_node("aggregate", aggregate, defer=True)
```

---

## Q22. What is the Functional API (`@entrypoint`, `@task`), and when do you prefer it?

The Functional API gives you LangGraph's runtime features (checkpointing, interrupts, streaming, durable execution) using **ordinary Python control flow** instead of explicit nodes and edges.
- `@task` marks a unit of work whose result is checkpointed, so it doesn't re-run on resume.
- `@entrypoint` marks the workflow.

```python
from langgraph.func import entrypoint, task

@task
def write_essay(topic: str) -> str:
    return llm.invoke(f"Write a short essay on {topic}").content

@task
def grade(essay: str) -> int:
    return int(llm.invoke(f"Score 1-10, digit only:\n{essay}").content.strip())

@entrypoint(checkpointer=InMemorySaver())
def essay_workflow(topic: str) -> dict:
    essay = write_essay(topic).result()
    for _ in range(3):                              # a plain Python loop instead of graph edges
        if grade(essay).result() >= 8:
            break
        essay = write_essay(f"{topic}. Improve on: {essay[:300]}").result()
    approved = interrupt({"essay": essay, "action": "approve?"})
    return {"essay": essay, "approved": approved}
```

| Graph API | Functional API |
|---|---|
| Visualisable structure, explicit state schema, best for complex branching or multi-agent | Less boilerplate, natural Python, easy to retrofit into existing code |
| State shared via the schema and reducers | State is function-scoped; return values flow naturally |
| Easier to reason about parallel supersteps | Parallelism via launching several tasks and calling `.result()` later |

---

## Q23. How do you build a tool-calling agent: `create_agent` vs a hand-built graph?

**Prebuilt (LangChain 1.x):**

```python
from langchain.agents import create_agent
from langchain.tools import tool

@tool
def get_order_status(order_id: str) -> str:
    """Look up an order's shipping status."""
    return f"Order {order_id}: shipped, arriving Tuesday"

agent = create_agent(model="openai:gpt-4.1-mini", tools=[get_order_status],
                     system_prompt="You are a concise support agent.",
                     checkpointer=InMemorySaver())
agent.invoke({"messages": [("user", "Where is order 1766?")]},
             {"configurable": {"thread_id": "c1"}})
```

**Hand-built ReAct loop** (full control over every step):

```python
from langgraph.prebuilt import ToolNode, tools_condition

tools = [get_order_status]
model_with_tools = llm.bind_tools(tools)

def call_model(state: MessagesState):
    return {"messages": [model_with_tools.invoke(state["messages"])]}

b = StateGraph(MessagesState)
b.add_node("agent", call_model)
b.add_node("tools", ToolNode(tools))
b.add_edge(START, "agent")
b.add_conditional_edges("agent", tools_condition)   # → "tools" if tool_calls else END
b.add_edge("tools", "agent")
react = b.compile(checkpointer=InMemorySaver())
```

| `create_agent` | Custom graph |
|---|---|
| Fast to build; middleware for summarisation, HITL, model fallbacks, PII, call limits | Total control: custom state, extra validation nodes, parallel tool policies, non-standard loops |
| Opinionated loop | More code to maintain |

---

## Q24. What are input/output schemas and private state channels?

You can restrict what a graph accepts and returns, and keep internal keys private:

```python
class InputState(TypedDict):
    question: str

class OutputState(TypedDict):
    answer: str

class OverallState(InputState, OutputState):
    docs: list[str]          # internal only
    rewritten: str

b = StateGraph(OverallState, input_schema=InputState, output_schema=OutputState)
```

Benefits:
- a clean API contract (callers can't inject internal keys),
- smaller outputs,
- private scratch data between specific nodes (a node can also declare its **own** input type, which may include keys not in the overall schema).

---

# Part B — Pipeline Patterns & Use Cases (Q25–Q42)

> Each question gives you a use case and asks you to design the pipeline. Every answer follows the same layout: the **use case**, the **graph shape**, **why this pattern fits**, the **trade-offs**, and **code**.

### Pattern selection map

| Pattern | Control | Typical use case |
|---|---|---|
| Sequential / prompt chaining | Fixed | Content generation, document processing |
| Routing | Rule or LLM classifier | Support triage, intent routing |
| Parallelisation (static fan-out) | Fixed | Multi-check moderation, multi-source search |
| Map-reduce (`Send`) | Dynamic N | Summarising many docs, batch extraction |
| Orchestrator-worker | LLM plans, workers execute | Report or code generation by sections |
| Evaluator-optimizer | Loop until pass | Code gen + tests, translation quality |
| ReAct tool agent | LLM-driven loop | Customer support with tools |
| Agentic / Corrective RAG | Workflow + LLM grading | Enterprise Q&A |
| Human-in-the-loop approval | Pause/resume | Refunds, emails, deployments |
| Supervisor multi-agent | LLM supervisor | Research + coding assistants |
| Handoffs / swarm | Agents transfer control | Sales → billing → support |
| Hierarchical teams | Nested supervisors (subgraphs) | Large enterprise assistants |
| Plan-and-execute | Planner + executor + replanner | Multi-step research tasks |
| Durable long-running pipeline | Checkpointed batch | ETL, data enrichment, document ingestion |
| Memory-enabled assistant | Store + checkpointer | Personal assistant |
| SQL / analytics agent | Generate → validate → execute → repair | BI chatbot |
| Saga / compensation | Error handlers | Order booking across services |
| Guardrailed pipeline | Input/output checks around core | Regulated finance/health bots |

---

## Q25. Design a **sequential (prompt-chaining) pipeline**. Use case: a blog generator that creates an outline, drafts, checks SEO, and polishes the result.

**Graph:** `START → outline → draft → seo_check → (pass? polish : draft) → END`

**Why:** the task decomposes into fixed steps. Each step gets a focused prompt, and you can add a **programmatic gate** (word count, keyword presence) between steps.

**Trade-offs:** more LLM calls mean more latency and cost, but each step is simpler, more reliable, and individually testable.

```python
class BlogState(TypedDict):
    topic: str
    keywords: list[str]
    outline: str
    draft: str
    attempts: int
    final: str

def outline(s): return {"outline": llm.invoke(f"Outline a blog on {s['topic']}").content}
def draft(s):
    return {"draft": llm.invoke(f"Write the blog from this outline, include {s['keywords']}:\n{s['outline']}").content,
            "attempts": s.get("attempts", 0) + 1}
def seo_gate(s) -> Literal["polish", "draft"]:
    ok = all(k.lower() in s["draft"].lower() for k in s["keywords"]) and len(s["draft"].split()) > 600
    return "polish" if ok or s["attempts"] >= 3 else "draft"     # programmatic gate, bounded retries
def polish(s): return {"final": llm.invoke(f"Polish tone and grammar:\n{s['draft']}").content}

b = StateGraph(BlogState)
b.add_sequence([outline, draft])
b.add_node(polish)
b.add_edge(START, "outline")
b.add_conditional_edges("draft", seo_gate)
b.add_edge("polish", END)
blog_graph = b.compile()
```

---

## Q26. Design a **routing pipeline**. Use case: a customer-support triage system routing to billing, technical, returns, or a human.

**Graph:** `START → classify → {billing | tech | returns | human} → END`

**Router options:**

| Router | Pros | Cons |
|---|---|---|
| Rules/regex | Free, instant | Brittle |
| LLM with **structured output** | Handles nuance; returns confidence | Extra call |
| Embedding similarity to labelled examples | ~10 ms | Needs example set |
| Fine-tuned classifier | Fast and accurate | Needs labelled data |

```python
from pydantic import BaseModel, Field

class Route(BaseModel):
    category: Literal["billing", "tech", "returns", "human"]
    confidence: float = Field(ge=0, le=1)
    reason: str

class TriageState(MessagesState):
    route: Route | None

def classify(s: TriageState):
    r = llm.with_structured_output(Route).invoke(
        [{"role": "system", "content": "Classify the support request."}, *s["messages"]])
    if r.confidence < 0.6:
        r.category = "human"                      # low confidence → human
    return {"route": r}

def make_agent(name, prompt):
    def node(s: TriageState):
        return {"messages": [llm.invoke([{"role": "system", "content": prompt}, *s["messages"]])]}
    node.__name__ = name
    return node

b = StateGraph(TriageState)
b.add_node(classify)
for name, p in {"billing": "You handle billing.", "tech": "You handle tech issues.",
                "returns": "You handle returns."}.items():
    b.add_node(name, make_agent(name, p)); b.add_edge(name, END)
b.add_node("human", lambda s: {"messages": [("ai", "Connecting you to a human agent.")]})
b.add_edge("human", END)
b.add_edge(START, "classify")
b.add_conditional_edges("classify", lambda s: s["route"].category)
triage = b.compile(checkpointer=InMemorySaver())
```

---

## Q27. Design a **parallelisation pipeline** (static fan-out / fan-in). Use case: content moderation that runs toxicity, PII, and policy checks at the same time.

**Graph:** `START → [toxicity, pii, policy] → aggregate → END`

**Why:** the checks are independent, so running them together keeps total latency close to the slowest check instead of their sum. **Sectioning** (different checks) and **voting** (the same check run N times for robustness) are both parallelisation.

**Trade-offs:** costs N calls at once, and needs reducers for the shared keys.

```python
class ModState(TypedDict):
    text: str
    flags: Annotated[dict, lambda a, b: {**(a or {}), **(b or {})}]
    verdict: str

def toxicity(s): return {"flags": {"toxic": "yes" in llm.invoke(f"Is this toxic? yes/no: {s['text']}").content.lower()}}
def pii(s):
    import re
    return {"flags": {"pii": bool(re.search(r"\b\d{10}\b|@\w+\.\w+", s["text"]))}}
def policy(s): return {"flags": {"policy": "yes" in llm.invoke(f"Violates policy? yes/no: {s['text']}").content.lower()}}
def aggregate(s): return {"verdict": "block" if any(s["flags"].values()) else "allow"}

b = StateGraph(ModState)
for n in (toxicity, pii, policy):
    b.add_node(n); b.add_edge(START, n.__name__)
b.add_node(aggregate)
b.add_edge(["toxicity", "pii", "policy"], "aggregate")      # waits for all three
b.add_edge("aggregate", END)
moderation = b.compile()
```

---

## Q28. Design a **map-reduce pipeline**. Use case: summarise 500 customer reviews, then extract the top themes.

**Graph:** `START → split → (Send × N) → summarize_batch → collapse_if_too_long → reduce_themes → END`

**Key design points:**
- Batch reviews (e.g. 25 per task) to cut the number of calls.
- Use `max_concurrency` to respect rate limits.
- **Recursive collapse**: if the combined summaries exceed the context window, reduce them again in groups.
- Checkpointing means failed batches retry without redoing successful ones.

```python
class ReviewState(TypedDict):
    reviews: list[str]
    batch_summaries: Annotated[list[str], operator.add]
    themes: str

def dispatch(s: ReviewState):
    batches = [s["reviews"][i:i + 25] for i in range(0, len(s["reviews"]), 25)]
    return [Send("summarize_batch", {"batch": b}) for b in batches]

def summarize_batch(inp: dict):
    text = "\n".join(inp["batch"])
    return {"batch_summaries": [llm.invoke(f"List complaints & praises briefly:\n{text}").content]}

def collapse_or_reduce(s: ReviewState) -> Literal["collapse", "reduce_themes"]:
    return "collapse" if sum(len(x) for x in s["batch_summaries"]) > 60_000 else "reduce_themes"

def collapse(s: ReviewState):
    groups = [s["batch_summaries"][i:i + 5] for i in range(0, len(s["batch_summaries"]), 5)]
    merged = [llm.invoke("Merge:\n" + "\n".join(g)).content for g in groups]
    # list reducer is additive, so overwrite by returning a reset marker handled by a custom reducer in real code
    return {"batch_summaries": merged}

def reduce_themes(s: ReviewState):
    return {"themes": llm.invoke("Top 5 themes with counts:\n" + "\n".join(s["batch_summaries"])).content}

b = StateGraph(ReviewState)
b.add_node(summarize_batch); b.add_node(collapse); b.add_node(reduce_themes, defer=True)
b.add_conditional_edges(START, dispatch, ["summarize_batch"])
b.add_conditional_edges("summarize_batch", collapse_or_reduce)
b.add_edge("collapse", "reduce_themes"); b.add_edge("reduce_themes", END)
review_graph = b.compile(checkpointer=InMemorySaver())
review_graph.invoke({"reviews": reviews}, {"configurable": {"thread_id": "reviews-nov"}, "max_concurrency": 8})
```

> **Gotcha:** with an `operator.add` reducer, `collapse` would *append* the merged summaries rather than replace them. In production, use a custom reducer that supports a reset (see Q4).

---

## Q29. Design an **orchestrator-worker pipeline**. Use case: generate a market-research report where the number and type of sections are decided at runtime.

**Difference from map-reduce:** in map-reduce the split is mechanical (by document). Here an **LLM orchestrator plans** the sub-tasks, so their number and nature are dynamic.

**Graph:** `START → orchestrator (plan sections) → Send(worker) × N → synthesizer → END`

```python
class Section(BaseModel):
    title: str
    instructions: str

class Plan(BaseModel):
    sections: list[Section]

class ReportState(TypedDict):
    topic: str
    plan: list[Section]
    completed: Annotated[list[str], operator.add]
    report: str

def orchestrator(s):
    plan = llm.with_structured_output(Plan).invoke(f"Plan 4-7 sections for a report on {s['topic']}")
    return {"plan": plan.sections}

def assign(s):
    return [Send("worker", {"section": sec, "topic": s["topic"]}) for sec in s["plan"]]

def worker(inp):
    sec = inp["section"]
    body = llm.invoke(f"Topic: {inp['topic']}\nWrite section '{sec.title}': {sec.instructions}").content
    return {"completed": [f"## {sec.title}\n{body}"]}

def synthesizer(s):
    order = {sec.title: i for i, sec in enumerate(s["plan"])}
    ordered = sorted(s["completed"], key=lambda t: order.get(t.split("\n")[0][3:], 99))   # keep plan order
    return {"report": "\n\n".join(ordered)}

b = StateGraph(ReportState)
b.add_node(orchestrator); b.add_node(worker); b.add_node(synthesizer)
b.add_edge(START, "orchestrator")
b.add_conditional_edges("orchestrator", assign, ["worker"])
b.add_edge("worker", "synthesizer"); b.add_edge("synthesizer", END)
report_graph = b.compile()
```

**Trade-offs:** flexible and parallel, but the plan quality bounds the output quality, and parallel workers finish in arbitrary order, so restore the planned order as shown.

---

## Q30. Design an **evaluator-optimizer (reflection) loop**. Use case: generate Python code until its unit tests pass.

**Graph:** `START → generate → run_tests → (pass or max_iters ? END : reflect → generate)`

**Why:** quality improves through explicit feedback. Use **objective evaluators** (tests, schema validation, linters) where possible, and fall back to an LLM judge.

**Trade-offs:** more iterations mean more cost. Always cap iterations, and fail gracefully at the cap.

```python
import subprocess, tempfile, textwrap

class CodeState(TypedDict):
    spec: str
    tests: str
    code: str
    feedback: str
    iterations: int
    passed: bool

def generate(s):
    prompt = f"Write Python for:\n{s['spec']}\nPrevious feedback:\n{s.get('feedback', 'none')}\nCode only."
    code = llm.invoke(prompt).content.strip().removeprefix("```python").removesuffix("```")
    return {"code": code, "iterations": s.get("iterations", 0) + 1}

def run_tests(s):
    with tempfile.TemporaryDirectory() as d:
        open(f"{d}/sol.py", "w").write(s["code"])
        open(f"{d}/test_sol.py", "w").write(s["tests"])
        r = subprocess.run(["python", "-m", "pytest", "-q", d], capture_output=True, text=True, timeout=60)
    return {"passed": r.returncode == 0, "feedback": r.stdout[-3000:]}    # run in a sandbox in production

def should_continue(s) -> Literal["generate", "__end__"]:
    return END if s["passed"] or s["iterations"] >= 4 else "generate"

b = StateGraph(CodeState)
b.add_sequence([generate, run_tests])
b.add_edge(START, "generate")
b.add_conditional_edges("run_tests", should_continue)
coder = b.compile()
```

---

## Q31. Design an **agentic / corrective RAG (CRAG) pipeline**. Use case: an enterprise Q&A bot that must not answer from irrelevant documents.

**Graph:**

```
START → rewrite_query → retrieve → grade_docs ─┬─(relevant)──────────────→ generate → check_grounded ─┬─(ok)→ END
                                               └─(none relevant & tries<2)→ rewrite_query          └─(hallucinated)→ generate (max 2)
                                               └─(none & tries≥2) → web_search/fallback → generate
```

**Concepts shown:** loops, LLM-as-grader, bounded retries, fallback sources, and a groundedness check (the core ideas of Self-RAG and CRAG).

```python
class RagState(TypedDict):
    question: str
    query: str
    docs: list[str]
    answer: str
    tries: int
    gen_tries: int

class Grade(BaseModel):
    relevant: bool

def rewrite_query(s):
    q = s["question"] if not s.get("tries") else llm.invoke(
        f"Rewrite to improve retrieval:\n{s['question']}").content
    return {"query": q, "tries": s.get("tries", 0) + 1}

def retrieve(s): return {"docs": retriever.invoke(s["query"])}     # any LangChain retriever

def grade_docs(s):
    grader = llm.with_structured_output(Grade)
    kept = [d for d in s["docs"]
            if grader.invoke(f"Q: {s['question']}\nDoc: {d.page_content}\nRelevant?").relevant]
    return {"docs": kept}

def route_after_grade(s) -> Literal["generate", "rewrite_query", "fallback"]:
    if s["docs"]: return "generate"
    return "rewrite_query" if s["tries"] < 2 else "fallback"

def fallback(s): return {"docs": web_search.invoke(s["question"])}

def generate(s):
    ctx = "\n\n".join(getattr(d, "page_content", str(d)) for d in s["docs"])
    return {"answer": llm.invoke(f"Answer only from context.\n{ctx}\nQ: {s['question']}").content,
            "gen_tries": s.get("gen_tries", 0) + 1}

def check_grounded(s) -> Literal["generate", "__end__"]:
    ok = llm.with_structured_output(Grade).invoke(
        f"Is this answer fully supported by the docs?\nDocs:{s['docs']}\nAnswer:{s['answer']}").relevant
    return END if ok or s["gen_tries"] >= 2 else "generate"

b = StateGraph(RagState)
for n in (rewrite_query, retrieve, grade_docs, generate, fallback):
    b.add_node(n)
b.add_edge(START, "rewrite_query"); b.add_edge("rewrite_query", "retrieve")
b.add_edge("retrieve", "grade_docs")
b.add_conditional_edges("grade_docs", route_after_grade)
b.add_edge("fallback", "generate")
b.add_conditional_edges("generate", check_grounded)
crag = b.compile()
```

**Trade-offs:** much higher answer quality and safety, but 3–8 LLM calls per question. Mitigate by using a small model for grading, grading in parallel with `Send`, or using a cross-encoder re-ranker instead of an LLM grader.

---

## Q32. Design a **human-in-the-loop approval pipeline for tool calls**. Use case: an agent that can send emails, which must be approved, edited, or rejected before sending.

**Pattern:** intercept tool calls for sensitive tools and `interrupt()` with the proposed call. Resume with one of `approve`, `edit` (new args), or `reject` (with feedback for the model).

```python
from langchain_core.messages import ToolMessage

SENSITIVE = {"send_email", "issue_refund"}

class AgentState(MessagesState):
    pass

def human_review(state: AgentState) -> Command[Literal["tools", "agent"]]:
    last = state["messages"][-1]
    risky = [tc for tc in last.tool_calls if tc["name"] in SENSITIVE]
    if not risky:
        return Command(goto="tools")
    decision = interrupt({"tool_calls": risky, "options": ["approve", "edit", "reject"]})
    if decision["action"] == "approve":
        return Command(goto="tools")
    if decision["action"] == "edit":
        edited = last.model_copy(deep=True)
        for tc in edited.tool_calls:
            if tc["name"] in SENSITIVE:
                tc["args"] = decision["args"]
        return Command(goto="tools", update={"messages": [edited]})          # same id → replaces
    rejections = [ToolMessage(content=f"Rejected by human: {decision.get('feedback', '')}",
                              tool_call_id=tc["id"]) for tc in last.tool_calls]
    return Command(goto="agent", update={"messages": rejections})

def route_agent(s) -> Literal["human_review", "__end__"]:
    return "human_review" if s["messages"][-1].tool_calls else END

b = StateGraph(AgentState)
b.add_node("agent", call_model); b.add_node(human_review); b.add_node("tools", ToolNode(tools))
b.add_edge(START, "agent"); b.add_conditional_edges("agent", route_agent); b.add_edge("tools", "agent")
email_agent = b.compile(checkpointer=InMemorySaver())

# Resume from a UI:
# email_agent.invoke(Command(resume={"action": "edit", "args": {"to": "a@x.com", "body": "..."}}), cfg)
```

With `create_agent`, the equivalent is `HumanInTheLoopMiddleware(interrupt_on={"send_email": True})`.

---

## Q33. Design a **supervisor multi-agent pipeline**. Use case: a research assistant with a web researcher, a data analyst (Python), and a writer.

**Graph:** `START → supervisor ⇄ {researcher, analyst, writer} → END` (each worker returns to the supervisor).

**Design points:**
- The supervisor uses **structured output** to choose the next worker or FINISH.
- Each worker is a **subgraph or `create_agent`** with its own tools and prompt.
- Decide what each agent sees: the full shared message history (simple, but long context) vs. only its instructions plus final results (cheaper, more focused).
- Cap supervisor turns.

```python
from langchain.agents import create_agent

researcher = create_agent("openai:gpt-4.1-mini", tools=[web_search], name="researcher",
                          system_prompt="Find facts with sources.")
analyst = create_agent("openai:gpt-4.1-mini", tools=[python_repl], name="analyst",
                       system_prompt="Analyse data with Python.")
writer = create_agent("openai:gpt-4.1-mini", tools=[], name="writer",
                      system_prompt="Write the final report from the team's findings.")

class TeamState(MessagesState):
    turns: int

class Next(BaseModel):
    next: Literal["researcher", "analyst", "writer", "FINISH"]
    instruction: str

def supervisor(s: TeamState) -> Command[Literal["researcher", "analyst", "writer", "__end__"]]:
    if s.get("turns", 0) >= 8:
        return Command(goto=END)
    d = llm.with_structured_output(Next).invoke(
        [{"role": "system", "content": "You manage researcher, analyst, writer. Pick who acts next or FINISH."},
         *s["messages"]])
    if d.next == "FINISH":
        return Command(goto=END)
    return Command(goto=d.next, update={"turns": s.get("turns", 0) + 1,
                                        "messages": [("user", f"[supervisor → {d.next}] {d.instruction}")]})

def wrap(agent, name):
    def node(s: TeamState) -> Command[Literal["supervisor"]]:
        out = agent.invoke({"messages": s["messages"]})
        return Command(goto="supervisor",
                       update={"messages": [("ai", f"[{name}] {out['messages'][-1].content}")]})
    node.__name__ = name
    return node

b = StateGraph(TeamState)
b.add_node(supervisor)
for a, n in ((researcher, "researcher"), (analyst, "analyst"), (writer, "writer")):
    b.add_node(n, wrap(a, n))
b.add_edge(START, "supervisor")
team = b.compile(checkpointer=InMemorySaver())
```

**Trade-offs:** clear control and easy to add agents, but the supervisor is a bottleneck (an extra LLM call per hop), and its routing errors compound. The `langgraph-supervisor` package offers a prebuilt version.

---

## Q34. Design a **handoff / swarm pipeline**. Use case: a sales agent hands the customer to a billing agent, who can hand back.

**Difference from supervisor:** there is no central router. Each agent decides when to **transfer control** using a handoff tool. That means fewer LLM hops, but control flow is less centralised and harder to reason about.

```python
from langchain.tools import tool, InjectedToolCallId
from langgraph.prebuilt import InjectedState

def make_handoff(target: str):
    @tool(f"transfer_to_{target}", description=f"Transfer the customer to the {target} agent.")
    def handoff(state: Annotated[dict, InjectedState],
                tool_call_id: Annotated[str, InjectedToolCallId]) -> Command:
        msg = ToolMessage(content=f"Transferred to {target}", tool_call_id=tool_call_id)
        return Command(goto=target, graph=Command.PARENT,
                       update={"messages": state["messages"] + [msg], "active_agent": target})
    return handoff

sales = create_agent("openai:gpt-4.1-mini", tools=[product_lookup, make_handoff("billing")],
                     name="sales", system_prompt="You sell plans. Billing questions → transfer.")
billing = create_agent("openai:gpt-4.1-mini", tools=[invoice_lookup, make_handoff("sales")],
                       name="billing", system_prompt="You handle invoices. Sales questions → transfer.")

class SwarmState(MessagesState):
    active_agent: str

b = StateGraph(SwarmState)
b.add_node("sales", sales, destinations=("billing",))      # destinations help visualisation
b.add_node("billing", billing, destinations=("sales",))
b.add_conditional_edges(START, lambda s: s.get("active_agent") or "sales", ["sales", "billing"])
swarm = b.compile(checkpointer=InMemorySaver())          # the next turn resumes with the active agent
```

The `langgraph-swarm` package provides a prebuilt swarm with `active_agent` tracking.

---

## Q35. Design a **hierarchical multi-agent system**. Use case: an enterprise assistant with a top supervisor over a "Research team" and a "Document team", each with their own supervisor.

**Why:** a single supervisor with 15 tools or agents makes poor routing decisions. Hierarchy keeps each decision small.

**How:** build each team as its own **compiled supervisor graph (subgraph)** and add the teams as nodes under a top-level supervisor.

```python
from pydantic import create_model

def build_team(name: str, members: dict):
    Pick = create_model("Pick", next=(Literal[tuple(members) + ("FINISH",)], ...))   # dynamic enum

    def sup(s: MessagesState) -> Command:
        d = llm.with_structured_output(Pick).invoke(
            [{"role": "system", "content": f"You lead {name}. Members: {list(members)}. Pick next or FINISH."},
             *s["messages"]])
        return Command(goto=END if d.next == "FINISH" else d.next)

    g = StateGraph(MessagesState)
    g.add_node("supervisor", sup)
    for n, agent in members.items():
        g.add_node(n, lambda s, a=agent, nm=n: {"messages": [("ai", f"[{nm}] " +
                   a.invoke({"messages": s["messages"]})["messages"][-1].content)]})
        g.add_edge(n, "supervisor")
    g.add_edge(START, "supervisor")
    return g.compile(name=name)

research_team = build_team("research_team", {"web": researcher, "analyst": analyst})
doc_team = build_team("doc_team", {"writer": writer, "editor": editor})

top = StateGraph(MessagesState)
top.add_node("research_team", research_team)          # shared `messages` key → add directly
top.add_node("doc_team", doc_team)
top.add_node("top_supervisor", top_supervisor_fn)    # routes between teams or END
top.add_edge(START, "top_supervisor")
top.add_edge("research_team", "top_supervisor"); top.add_edge("doc_team", "top_supervisor")
enterprise = top.compile(checkpointer=InMemorySaver())
```

**Trade-offs:** scales to many agents with clear ownership per team, but adds latency (more supervisor hops), makes debugging harder, and requires observability with `subgraphs=True` streaming and nested traces.

---

## Q36. Design a **plan-and-execute pipeline**. Use case: "Compare the pricing of 3 cloud providers for a 10-node GPU cluster and recommend one."

**Graph:** `START → planner → executor (one step, ReAct with tools) → replanner → (done ? END : executor)`

**Why:** an explicit plan improves long-horizon tasks, keeps the expensive planner calls few, and lets a cheaper model execute the steps.

```python
class PlanState(TypedDict):
    objective: str
    plan: list[str]
    past_steps: Annotated[list[tuple[str, str]], operator.add]
    response: str

class PlanOut(BaseModel):
    steps: list[str]

class Replan(BaseModel):
    done: bool
    response: str = ""
    steps: list[str] = []

def planner(s):
    return {"plan": llm.with_structured_output(PlanOut).invoke(f"Plan steps for: {s['objective']}").steps}

executor_agent = create_agent("openai:gpt-4.1-mini", tools=[web_search, calculator])

def executor(s):
    step = s["plan"][0]
    out = executor_agent.invoke({"messages": [("user", f"Objective: {s['objective']}\nDo step: {step}")]})
    return {"past_steps": [(step, out["messages"][-1].content)]}

def replanner(s):
    r = llm.with_structured_output(Replan).invoke(
        f"Objective: {s['objective']}\nDone so far: {s['past_steps']}\nRemaining plan: {s['plan'][1:]}\n"
        "If finished, set done=true and give the final response; else give the updated remaining steps.")
    return {"response": r.response} if r.done else {"plan": r.steps}

b = StateGraph(PlanState)
b.add_sequence([planner, executor, replanner]); b.add_edge(START, "planner")
b.add_conditional_edges("replanner", lambda s: END if s.get("response") else "executor")
plan_exec = b.compile()
```

---

## Q37. Design a **durable, long-running pipeline**. Use case: nightly enrichment of 50,000 company records using paid APIs, which must resume after crashes without paying twice.

**Concepts:** `PostgresSaver`, `durability="sync"`, idempotent tasks, map-reduce in batches, `RetryPolicy`, `max_concurrency`, resuming with the same `thread_id`, **(1.2+)** graceful drain.

```python
from langgraph.checkpoint.postgres import PostgresSaver

class EnrichState(TypedDict):
    record_ids: list[str]
    results: Annotated[dict, lambda a, b: {**(a or {}), **(b or {})}]
    report: str

def dispatch(s):
    todo = [r for r in s["record_ids"] if r not in s.get("results", {})]     # skip completed
    return [Send("enrich_batch", {"ids": todo[i:i + 100]}) for i in range(0, len(todo), 100)]

def enrich_batch(inp):
    out = {}
    for rid in inp["ids"]:
        out[rid] = paid_api.enrich(rid, idempotency_key=f"enrich:{rid}")   # idempotent on retry
    return {"results": out}

def summarize(s): return {"report": f"Enriched {len(s['results'])} records"}

b = StateGraph(EnrichState)
b.add_node("enrich_batch", enrich_batch, retry_policy=RetryPolicy(max_attempts=5, backoff_factor=2))
b.add_node(summarize, defer=True)
b.add_conditional_edges(START, dispatch, ["enrich_batch"])
b.add_edge("enrich_batch", "summarize"); b.add_edge("summarize", END)

with PostgresSaver.from_conn_string(DB_URI) as cp:
    cp.setup()
    g = b.compile(checkpointer=cp)
    cfg = {"configurable": {"thread_id": "enrich-2026-11-28"}, "max_concurrency": 10}
    try:
        g.invoke({"record_ids": ids}, cfg, durability="sync")
    except Exception:
        g.invoke(None, cfg, durability="sync")   # resume: completed batches are not re-run
```

---

## Q38. Design a **memory-enabled personal assistant**. Use case: an assistant that remembers user preferences across sessions and keeps conversations short.

**Concepts:** checkpointer (per-thread history), Store (cross-thread memories with semantic search), summarisation of old messages, background memory extraction.

```python
class AssistState(MessagesState):
    summary: str

def load_and_answer(s: AssistState, runtime: Runtime[Ctx]):
    ns = ("memories", runtime.context.user_id)
    mems = runtime.store.search(ns, query=s["messages"][-1].content, limit=5)
    system = (f"Facts about the user:\n" + "\n".join(m.value["fact"] for m in mems) +
              f"\nEarlier conversation summary: {s.get('summary', '')}")
    return {"messages": [llm.invoke([{"role": "system", "content": system}, *s["messages"][-8:]])]}

class Facts(BaseModel):
    facts: list[str]

def extract_memories(s: AssistState, runtime: Runtime[Ctx]):
    new = llm.with_structured_output(Facts).invoke(
        f"Extract durable user facts/preferences (or none):\n{s['messages'][-2:]}").facts
    for f in new:
        runtime.store.put(("memories", runtime.context.user_id), key=str(abs(hash(f))), value={"fact": f})
    return {}

def summarize_if_long(s: AssistState):
    if len(s["messages"]) < 20:
        return {}
    old = s["messages"][:-8]
    summ = llm.invoke(f"Update summary.\nOld: {s.get('summary', '')}\nNew: {[m.content for m in old]}").content
    return {"summary": summ, "messages": [RemoveMessage(id=m.id) for m in old]}

b = StateGraph(AssistState, context_schema=Ctx)
b.add_node(load_and_answer); b.add_node(extract_memories); b.add_node(summarize_if_long)
b.add_edge(START, "load_and_answer")
b.add_edge("load_and_answer", "extract_memories")          # could run in the background instead
b.add_edge("load_and_answer", "summarize_if_long")         # runs in parallel with extraction
assistant = b.compile(checkpointer=InMemorySaver(), store=store)
```

---

## Q39. Design a **SQL / analytics agent pipeline**. Use case: "What was revenue by region last quarter?" over a production database.

**Graph:** `START → get_schema → write_sql → validate_sql → (invalid ? write_sql) → [optional human approval for expensive queries] → execute → (error ? write_sql : answer) → END`

**Safety controls:**
- a read-only DB user,
- an allowlist of tables,
- a `LIMIT` on every query,
- `EXPLAIN` cost checks,
- a cap on retries.

```python
import sqlglot

class SQLState(TypedDict):
    question: str
    schema: str
    sql: str
    error: str | None
    attempts: int
    rows: list
    answer: str

ALLOWED = {"orders", "customers", "regions"}

def get_schema(s): return {"schema": db.get_table_info(list(ALLOWED))}

def write_sql(s):
    sql = llm.invoke(f"Schema:\n{s['schema']}\nPrevious error: {s.get('error')}\n"
                     f"Write one PostgreSQL SELECT for: {s['question']}. SQL only.").content.strip("`\n ")
    return {"sql": sql.removeprefix("sql").strip(), "attempts": s.get("attempts", 0) + 1}

def validate_sql(s):
    try:
        tree = sqlglot.parse_one(s["sql"], read="postgres")
        if tree.key != "select":
            return {"error": "Only SELECT allowed"}
        tables = {t.name for t in tree.find_all(sqlglot.exp.Table)}
        if not tables <= ALLOWED:
            return {"error": f"Disallowed tables: {tables - ALLOWED}"}
        if not tree.args.get("limit"):
            return {"sql": tree.limit(1000).sql("postgres"), "error": None}
        return {"error": None}
    except sqlglot.errors.ParseError as e:
        return {"error": str(e)}

def execute(s):
    try:
        return {"rows": db.run(s["sql"]), "error": None}
    except Exception as e:
        return {"error": str(e)}

def answer(s): return {"answer": llm.invoke(f"Q: {s['question']}\nRows: {s['rows'][:50]}\nAnswer.").content}

def retry_or(nxt):
    def r(s): return nxt if not s.get("error") else ("write_sql" if s["attempts"] < 3 else END)
    return r

b = StateGraph(SQLState)
for n in (get_schema, write_sql, validate_sql, execute, answer): b.add_node(n)
b.add_edge(START, "get_schema"); b.add_edge("get_schema", "write_sql"); b.add_edge("write_sql", "validate_sql")
b.add_conditional_edges("validate_sql", retry_or("execute"), ["execute", "write_sql", END])
b.add_conditional_edges("execute", retry_or("answer"), ["answer", "write_sql", END])
b.add_edge("answer", END)
sql_agent = b.compile()
```

---

## Q40. Design a **saga / compensation pipeline**. Use case: a travel booking agent reserves a flight, then a hotel, then charges payment. If payment fails, both reservations must be released.

**Concepts:** ordered side effects, idempotency keys, compensating actions, **(1.2+)** `error_handler` that runs after retries are exhausted, or explicit error routing in older versions.

```python
class TripState(TypedDict):
    trip_id: str
    flight_ref: str | None
    hotel_ref: str | None
    status: str
    error: str | None

def book_flight(s): return {"flight_ref": flights.reserve(s["trip_id"], idem=f"f:{s['trip_id']}")}
def book_hotel(s): return {"hotel_ref": hotels.reserve(s["trip_id"], idem=f"h:{s['trip_id']}")}
def charge(s):
    payments.charge(s["trip_id"], idem=f"p:{s['trip_id']}")
    return {"status": "confirmed"}

def compensate(s):
    if s.get("hotel_ref"): hotels.cancel(s["hotel_ref"])
    if s.get("flight_ref"): flights.cancel(s["flight_ref"])
    return {"status": "rolled_back"}

# Option A (1.2+): declarative error handler after retries are exhausted
from langgraph.errors import NodeError
def on_charge_failed(state: TripState, error: NodeError) -> Command[Literal["compensate"]]:
    return Command(goto="compensate", update={"error": str(error)})

b = StateGraph(TripState)
b.add_sequence([book_flight, book_hotel])
b.add_node("charge", charge, retry_policy=RetryPolicy(max_attempts=3), error_handler=on_charge_failed)
b.add_node(compensate)
b.add_edge(START, "book_flight"); b.add_edge("book_hotel", "charge")
b.add_edge("charge", END); b.add_edge("compensate", END)
saga = b.compile(checkpointer=InMemorySaver())

# Option B (any version): catch inside the node and route with Command
def charge_safe(s) -> Command[Literal["compensate", "__end__"]]:
    try:
        payments.charge(s["trip_id"], idem=f"p:{s['trip_id']}")
        return Command(goto=END, update={"status": "confirmed"})
    except PaymentError as e:
        return Command(goto="compensate", update={"error": str(e)})
```

---

## Q41. Design a **guardrailed pipeline** for a regulated domain. Use case: a stock-broking assistant that must block insider-trading requests and redact PII.

**Graph:** `START → input_guard → (blocked ? safe_reply : core_agent) → output_guard → END`

```python
class GuardState(MessagesState):
    blocked: bool
    category: str

class Verdict(BaseModel):
    blocked: bool
    category: Literal["insider_trading", "manipulation", "pii_request", "none"]

def input_guard(s):
    v = llm.with_structured_output(Verdict).invoke(
        [{"role": "system", "content": "Flag requests to facilitate illegal trading or extract personal data. "
                                       "Educational questions are allowed."}, s["messages"][-1]])
    return {"blocked": v.blocked, "category": v.category}

def safe_reply(s):
    return {"messages": [("ai", "I can't help with that, but I can explain legal investing strategies.")]}

def output_guard(s):
    import re
    last = s["messages"][-1]
    redacted = re.sub(r"\b[A-Z]{5}\d{4}[A-Z]\b", "[PAN REDACTED]", last.content)   # Indian PAN pattern
    return {"messages": [last.model_copy(update={"content": redacted})]}           # same id → replaced

b = StateGraph(GuardState)
for n in (input_guard, safe_reply, output_guard): b.add_node(n)
b.add_node("core_agent", core_agent)                 # any agent / subgraph
b.add_edge(START, "input_guard")
b.add_conditional_edges("input_guard", lambda s: "safe_reply" if s["blocked"] else "core_agent")
b.add_edge("core_agent", "output_guard"); b.add_edge("output_guard", END); b.add_edge("safe_reply", END)
guarded = b.compile()
```

**Tip:** run the input guard **in parallel** with retrieval, and cancel or ignore the retrieval result if the guard blocks the request. This hides the guard's latency.

---

## Q42. How do you decide which pipeline pattern to use? Give a decision framework.

1. **Is the sequence of steps known ahead of time?** Yes → workflow (chain, routing, parallel). No → agent (ReAct, plan-execute).
2. **Is the work splittable into independent parts?** Fixed parts → static parallel. Runtime-sized → `Send` map-reduce. LLM-decided → orchestrator-worker.
3. **Does output quality need iteration?** → evaluator-optimizer with an objective evaluator and an iteration cap.
4. **Are there irreversible side effects?** → HITL `interrupt` before them; idempotency; saga compensation.
5. **Many tools or domains?** → supervisor (central control) or handoffs (fewer hops); go hierarchical beyond about 5–7 agents.
6. **Long-running or expensive?** → Postgres checkpointer, `durability="sync"`, retries, resumability.
7. **Needs personalisation?** → Store-based long-term memory.
8. **Regulated?** → guard nodes, audit traces, PII masking in observability.

**Rule of thumb:** start with the simplest deterministic workflow that works, and add agency only where it measurably improves outcomes.

---

# Part C — Observability & Configuration (Q43–Q55)

## Q43. What does observability mean for a LangGraph application, and what should you capture?

Agent observability is about answering **"what happened, why, how long did it take, and how much did it cost?"** for every run.

| Layer | What to capture |
|---|---|
| **Traces** | One trace per invocation. Nested spans: graph → node → LLM call / tool call / retriever / subgraph |
| **Inputs and outputs** | Prompt messages, model responses, tool args and results, state deltas per node (masked for PII) |
| **Metadata** | thread_id, user_id/tenant, graph version, prompt version, model name, environment, feature flag |
| **Metrics** | Latency (p50/p95 per node; TTFT for streaming), tokens and cost per run and node, error rate, retry count, interrupt count and time-to-approval, recursion-limit hits, steps per run, tool-call counts |
| **Quality signals** | User feedback (👍/👎), online LLM-judge evaluations, groundedness, task success |
| **State history** | Checkpoints per thread (for replay and time travel) |
| **Alerts** | Error spikes, cost anomalies, latency SLO breaches, loop detection |

Tools commonly used:
- **LangSmith** (native and automatic),
- **Langfuse** (open source, via a callback handler),
- **OpenTelemetry** export to Datadog, Grafana Tempo, Jaeger or Honeycomb,
- Arize Phoenix and other OpenInference-based tools,
- **Prometheus** metrics for service-level dashboards.

---

## Q44. How do you enable LangSmith tracing for LangGraph, and which environment variables matter?

LangGraph auto-instruments when tracing is enabled through environment variables. You don't need code changes.

```bash
# .env
LANGSMITH_TRACING=true
LANGSMITH_API_KEY=lsv2_...
LANGSMITH_PROJECT=support-agent-prod          # group traces by app/environment
LANGSMITH_ENDPOINT=https://api.smith.langchain.com   # or EU / self-hosted endpoint
LANGSMITH_TRACING_SAMPLING_RATE=0.25          # trace 25% of runs (high-volume prod)
# Privacy (hide all inputs/outputs — coarse-grained)
LANGSMITH_HIDE_INPUTS=false
LANGSMITH_HIDE_OUTPUTS=false
# OpenTelemetry export
LANGSMITH_OTEL_ENABLED=false
```

| Variable | Purpose |
|---|---|
| `LANGSMITH_TRACING` | Master switch |
| `LANGSMITH_API_KEY` | Auth |
| `LANGSMITH_PROJECT` | Tracing project (use one per env: `-dev`, `-staging`, `-prod`) |
| `LANGSMITH_ENDPOINT` | Region or self-hosted URL |
| `LANGSMITH_TRACING_SAMPLING_RATE` | 0–1 fraction of traces kept |
| `LANGSMITH_HIDE_INPUTS` / `LANGSMITH_HIDE_OUTPUTS` | Drop all inputs or outputs from traces |
| `LANGSMITH_OTEL_ENABLED` | Emit OpenTelemetry spans |

Older `LANGCHAIN_*` names (e.g. `LANGCHAIN_TRACING_V2`) still appear in older code. Newer docs use `LANGSMITH_*`. Recent LangSmith docs also describe sending traces to an "agent" (`LANGSMITH_AGENT_ID`, `LANGSMITH_AGENT_ENVIRONMENT`) as an alternative to `LANGSMITH_PROJECT`. You use one or the other, not both.

---

## Q45. How do you attach run names, tags, and metadata so traces are searchable?

Pass them through `config`. They propagate to every nested node, LLM call, and subgraph.

```python
import uuid

run_id = uuid.uuid4()
config = {
    "configurable": {"thread_id": "cust-881:sess-12"},
    "run_name": "support_agent.turn",
    "run_id": run_id,                                  # lets you attach feedback later
    "tags": ["prod", "support", "graph:v3.2"],
    "metadata": {"user_id": "cust-881", "tenant": "acme", "prompt_version": "triage@7",
                 "model": "gpt-4.1-mini", "channel": "whatsapp"},
    "recursion_limit": 25,
    "max_concurrency": 8,
}
result = graph.invoke({"messages": [("user", "Refund status?")]}, config)

# Or bake defaults into the compiled graph
graph = builder.compile(checkpointer=cp).with_config(run_name="support_agent", tags=["support"])
```

Inside a node, LangGraph also adds metadata such as `langgraph_node`, `langgraph_step`, `langgraph_triggers`, and `checkpoint_ns`. You can filter on these in LangSmith or use them in `stream_mode="messages"` metadata.

**Interview tip:** put **business identifiers** (tenant, user, feature) in metadata, so you can slice cost, latency, and error rate per customer.

---

## Q46. How do you capture user feedback and link it to traces?

```python
from langsmith import Client
ls = Client()

# after the user clicks 👍/👎 in the UI for the response produced by run_id
ls.create_feedback(run_id=run_id, key="user_rating", score=1, comment="Solved my issue")
ls.create_feedback(run_id=run_id, key="resolved", score=0)
```

Use feedback to:
- build datasets of failing runs,
- drive online evaluators,
- compare graph and prompt versions in experiments.

---

## Q47. How do you trace custom code (non-LangChain functions) inside nodes?

Use `@traceable` so plain Python functions, such as API clients, re-rankers or business logic, appear as child spans:

```python
from langsmith import traceable

@traceable(run_type="retriever", name="vector_search")
def vector_search(query: str, k: int = 5) -> list[dict]:
    return qdrant.search(query, k=k)

@traceable(run_type="tool", name="crm_lookup", metadata={"system": "salesforce"})
def crm_lookup(customer_id: str) -> dict:
    return crm.get(customer_id)

def retrieve_node(state):
    return {"docs": vector_search(state["question"])}     # nested under the node span automatically
```

`run_type` options include `chain`, `llm`, `tool`, `retriever`, `embedding`, and `prompt`. They affect how the LangSmith UI renders the span.

---

## Q48. How do you control *what* gets traced: sampling, conditional tracing, and PII masking?

**1. Sampling** (reduce volume and cost):

```bash
export LANGSMITH_TRACING_SAMPLING_RATE=0.1
```

Or set a different rate per client: `Client(tracing_sampling_rate=0.5)`.

**2. Conditional tracing** (always trace some requests, never trace others). `tracing_context` has the highest priority, then `ls.configure(...)`, then environment variables.

```python
from langsmith import tracing_context

def handle(req):
    sensitive = req.tenant in {"hospital-a"}
    debug = req.headers.get("x-debug-trace") == "1"
    with tracing_context(enabled=(debug or not sensitive)):
        return graph.invoke(req.payload, req.config)
```

**3. Masking PII with an anonymizer** (fine-grained, keeps the trace useful):

```python
from langsmith import Client
from langsmith.anonymizer import create_anonymizer
from langchain_core.tracers.langchain import LangChainTracer

anonymizer = create_anonymizer([
    {"pattern": r"[\w\.-]+@[\w\.-]+\.\w+", "replace": "<email>"},
    {"pattern": r"\b\d{10}\b", "replace": "<phone>"},
    {"pattern": r"\b(?:\d[ -]?){13,19}\b", "replace": "<card>"},
])
tracer = LangChainTracer(client=Client(anonymizer=anonymizer))
graph.invoke(inputs, {"callbacks": [tracer], "configurable": {"thread_id": "t1"}})
```

**4. Coarse switches:** `LANGSMITH_HIDE_INPUTS=true` / `LANGSMITH_HIDE_OUTPUTS=true`. These hide everything; to hide individual keys, use an anonymizer or custom input/output processing.

| Approach | Pros | Cons |
|---|---|---|
| Sampling | Cheap; statistically representative | You may miss rare failures (combine it with always-trace-on-error or debug flags) |
| Conditional | Precise control by tenant or flag | Logic to maintain |
| Anonymizer | Keeps traces debuggable while protecting PII | Regex misses; slight overhead |
| Hide all I/O | Simplest compliance option | Traces lose most of their debugging value |

---

## Q49. How do you export LangGraph traces via OpenTelemetry to Datadog, Grafana, or Jaeger?

```bash
pip install "langsmith[otel]" opentelemetry-exporter-otlp
export LANGSMITH_OTEL_ENABLED=true
export LANGSMITH_TRACING=true
# Send to your own collector (which can fan out to LangSmith, Datadog, Tempo, ...)
export OTEL_EXPORTER_OTLP_ENDPOINT=http://otel-collector:4318
export OTEL_SERVICE_NAME=support-agent
```

Or set a global tracer provider in code before building the graph. LangSmith detects and uses it:

```python
from opentelemetry import trace
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor
from opentelemetry.sdk.resources import Resource
from opentelemetry.exporter.otlp.proto.http.trace_exporter import OTLPSpanExporter

provider = TracerProvider(resource=Resource.create({"service.name": "support-agent",
                                                   "deployment.environment": "prod"}))
provider.add_span_processor(BatchSpanProcessor(OTLPSpanExporter(endpoint="http://otel-collector:4318/v1/traces")))
trace.set_tracer_provider(provider)
```

**Why OTel:**
- vendor-neutral,
- correlates LLM spans with your existing HTTP, DB and queue spans in one distributed trace,
- lets you redact centrally in the collector before export.

---

## Q50. How do you integrate Langfuse (open-source) instead of LangSmith?

```python
from langfuse.langchain import CallbackHandler      # Langfuse v3 SDK
# env: LANGFUSE_PUBLIC_KEY, LANGFUSE_SECRET_KEY, LANGFUSE_HOST

handler = CallbackHandler()
graph.invoke(
    {"messages": [("user", "hi")]},
    config={"callbacks": [handler],
            "configurable": {"thread_id": "t1"},
            "metadata": {"langfuse_user_id": "cust-881", "langfuse_session_id": "t1",
                         "langfuse_tags": ["prod", "support"]}},
)
```

| LangSmith | Langfuse |
|---|---|
| Native, zero-code via env vars; tight integration with LangGraph Studio, datasets, evals, deployments | Open source, self-hostable, framework-agnostic |
| Managed SaaS (self-hosted option on enterprise plans) | You operate it if self-hosted |

Both support sessions, users, cost tracking, prompt management, and evaluations.

---

## Q51. How do you emit operational metrics (Prometheus) per node, beyond traces?

Traces are per-request. **Metrics** power dashboards and alerts. Wrap nodes with a decorator, or use a callback handler.

```python
import time, functools
from prometheus_client import Counter, Histogram, start_http_server

NODE_LATENCY = Histogram("lg_node_latency_seconds", "Node latency", ["graph", "node"])
NODE_ERRORS = Counter("lg_node_errors_total", "Node errors", ["graph", "node", "error"])
TOKENS = Counter("lg_llm_tokens_total", "LLM tokens", ["graph", "node", "kind"])

def instrumented(graph_name: str):
    def deco(fn):
        @functools.wraps(fn)
        def wrapper(state, *a, **kw):
            t = time.perf_counter()
            try:
                return fn(state, *a, **kw)
            except Exception as e:
                NODE_ERRORS.labels(graph_name, fn.__name__, type(e).__name__).inc()
                raise
            finally:
                NODE_LATENCY.labels(graph_name, fn.__name__).observe(time.perf_counter() - t)
        return wrapper
    return deco

@instrumented("support")
def call_model(state: MessagesState):
    msg = llm.invoke(state["messages"])
    u = msg.usage_metadata or {}
    TOKENS.labels("support", "call_model", "input").inc(u.get("input_tokens", 0))
    TOKENS.labels("support", "call_model", "output").inc(u.get("output_tokens", 0))
    return {"messages": [msg]}

start_http_server(9100)   # scrape endpoint
```

**Alerts to define:**
- p95 latency per node,
- error rate above X%,
- rate of `GraphRecursionError`,
- token spend per hour above budget,
- age of pending interrupts (approvals stuck),
- retries per run.

---

## Q52. How do you debug a misbehaving graph locally? Mention LangGraph Studio, `langgraph dev`, visualisation, and state inspection.

1. **Visualise the graph:** `print(graph.get_graph().draw_mermaid())` or `graph.get_graph(xray=True)`, which expands subgraphs.
2. **Stream with `debug` or `updates`** to see which node ran and what it returned.
3. **Inspect checkpoints:** `graph.get_state(cfg)` shows `.values`, `.next`, `.tasks` (including errors and interrupts), and `.metadata`.
4. **Time travel** to the bad step, edit state, and replay (Q13).
5. **LangGraph Studio** (via `langgraph dev`): a visual IDE where you watch execution, edit state, step through interrupts, and fork runs.
6. **LangSmith trace view:** nested spans with tokens, latency, and errors per node.

```bash
pip install -U "langgraph-cli[inmem]"
langgraph dev            # local server + Studio UI with hot reload
```

```python
snap = graph.get_state(cfg)
print(snap.next)                     # e.g. ('tools',) → where it's paused / what runs next
for task in snap.tasks:
    print(task.name, task.error, task.interrupts)
print(snap.metadata["step"], snap.metadata["source"])
```

---

## Q53. What goes into `langgraph.json`, and how do you configure a deployment?

`langgraph.json` is the config used by the LangGraph CLI and LangSmith Deployment (formerly LangGraph Platform) to build and serve your graphs as an API: threads, runs, streaming, cron jobs, a task queue, and persistence.

```json
{
  "$schema": "https://langgra.ph/schema.json",
  "python_version": "3.12",
  "dependencies": ["."],
  "graphs": {
    "support_agent": "./src/support/graph.py:graph",
    "research_team": "./src/research/graph.py:make_graph"
  },
  "env": ".env",
  "auth": { "path": "./src/auth.py:auth" },
  "store": {
    "index": { "embed": "openai:text-embedding-3-small", "dims": 1536, "fields": ["$"] },
    "ttl": { "refresh_on_read": true, "default_ttl": 10080, "sweep_interval_minutes": 60 }
  },
  "checkpointer": {
    "ttl": { "strategy": "delete", "default_ttl": 43200, "sweep_interval_minutes": 60 }
  },
  "http": { "cors": { "allow_origins": ["https://app.example.com"] } }
}
```

| Key | Purpose |
|---|---|
| `graphs` | Name → `module:variable` or a factory function (a factory can build the graph per request, e.g. per tenant) |
| `dependencies` | Packages and local paths to install |
| `env` | Env file or variables |
| `auth` | Custom authentication and authorisation handlers (e.g. restrict a user to their own threads) |
| `store.index` | Enables semantic search in the long-term memory store |
| `store.ttl` / `checkpointer.ttl` | Auto-expire old memories and threads (minutes); helps with cost and data-retention compliance |

Commands:
- `langgraph dev` runs locally in memory.
- `langgraph build` builds a Docker image.
- `langgraph up` runs it locally with Postgres and Redis.

**Platform-level features to mention:**
- **double-texting** strategies (`reject`, `enqueue`, `interrupt`, `rollback`) for concurrent messages on the same thread,
- background runs,
- cron jobs,
- webhooks,
- horizontal scaling with a task queue.

> Check exact `langgraph.json` keys against the CLI docs for your version. TTL and auth options have evolved across releases.

---

## Q54. How do you configure a graph at runtime: model choice, prompts, tenants? Context vs `configurable` vs graph factories.

| Mechanism | Use for | Notes |
|---|---|---|
| **`context_schema` + `Runtime`** | Run-scoped dependencies: user_id, tenant, model name, feature flags, DB clients | Typed; not persisted; recommended in 1.x |
| `config["configurable"]` | thread_id, checkpoint_id, older-style custom config | Still used for persistence keys |
| **Graph factory** (function returning a compiled graph) | Structurally different graphs per tenant or plan (e.g. premium tier gets an extra review node) | Built per request; cache them |
| Environment variables | Secrets, endpoints | Not per request |

```python
@dataclass
class AppCtx:
    tenant: str
    model: str = "openai:gpt-4.1-mini"
    strict_mode: bool = False

def respond(state: MessagesState, runtime: Runtime[AppCtx]):
    model = init_chat_model(runtime.context.model, temperature=0)
    prompt = PROMPTS[runtime.context.tenant]                 # tenant-specific prompt
    return {"messages": [model.invoke([{"role": "system", "content": prompt}, *state["messages"]])]}

def make_graph(tier: str):
    b = StateGraph(MessagesState, context_schema=AppCtx)
    b.add_node(respond); b.add_edge(START, "respond")
    if tier == "enterprise":
        b.add_node("compliance_review", compliance_review)
        b.add_edge("respond", "compliance_review")
    return b.compile(checkpointer=cp)

graph = make_graph("enterprise")
graph.invoke({"messages": [("user", "hi")]},
             {"configurable": {"thread_id": "t9"}},
             context=AppCtx(tenant="acme", model="anthropic:claude-sonnet-4-5", strict_mode=True))
```

---

## Q55. How do you secure checkpoints and memory: encryption, retention, and multi-tenancy?

| Concern | Control |
|---|---|
| Sensitive data at rest in checkpoints | **Encrypted serializer** (`EncryptedSerializer.from_pycryptodome_aes()`, key from `LANGGRAPH_AES_KEY`), plus DB-level encryption |
| Retention / right-to-be-forgotten | Checkpointer and store **TTLs**; `checkpointer.delete_thread(thread_id)`; delete user namespaces from the store |
| Tenant isolation | Thread IDs and store namespaces **prefixed by tenant/user**; enforce in a custom auth layer so user A can't load user B's thread |
| What's stored | Avoid putting secrets or raw documents in state; store references (IDs, S3 keys) instead |
| Trace privacy | Anonymizers, sampling, hide I/O (Q48) |

```python
from langgraph.checkpoint.serde.encrypted import EncryptedSerializer
from langgraph.checkpoint.postgres import PostgresSaver

serde = EncryptedSerializer.from_pycryptodome_aes()      # reads LANGGRAPH_AES_KEY env var
with PostgresSaver.from_conn_string(DB_URI, serde=serde) as cp:   # check the signature for your version
    cp.setup()
    graph = builder.compile(checkpointer=cp)

def thread_id_for(user) -> str:
    return f"{user.tenant_id}:{user.id}:{uuid.uuid4()}"   # never accept a raw thread_id from the client without checks
```

---

# Part D — Production Scenario Questions (Q56–Q68)

## Q56. Your agent loops forever (search → summarise → search). How do you prevent and detect this in LangGraph?

- **`recursion_limit`** in config caps supersteps and raises `GraphRecursionError`. Catch it and return a graceful fallback. **Set it explicitly.** Older versions defaulted to 25, but current 1.2.x defaults to 10,007 (overridable via the `LANGGRAPH_DEFAULT_RECURSION_LIMIT` env var). A looping agent can therefore burn thousands of LLM calls before the default stops it.
- A **step counter in state** with explicit routing to a "finalise" node once N steps have passed (better UX than an exception).
- **Repeated-call detection**: hash `(tool, args)` in state and stop when one repeats.
- With `create_agent`: **tool or model call-limit middleware**.
- Observability: an alert on steps per run and on recursion-limit errors.

```python
from langgraph.errors import GraphRecursionError

class LoopState(MessagesState):
    steps: int
    seen_calls: Annotated[list[str], operator.add]

def guard(state: LoopState) -> Literal["tools", "finalize", "__end__"]:
    last = state["messages"][-1]
    if not last.tool_calls:
        return END
    sigs = [f"{tc['name']}:{json.dumps(tc['args'], sort_keys=True)}" for tc in last.tool_calls]
    if state.get("steps", 0) >= 8 or any(s in state.get("seen_calls", []) for s in sigs):
        return "finalize"
    return "tools"

try:
    graph.invoke(inputs, {"recursion_limit": 30, "configurable": {"thread_id": "t"}})
except GraphRecursionError:
    reply = "I couldn't complete this automatically; a human agent will follow up."
```

---

## Q57. A user sends a second message while the first run on the same thread is still executing (double-texting). What happens, and how do you handle it?

Two concurrent runs on the same thread would race on checkpoints. Strategies (built into LangSmith Deployment as `multitask_strategy`; implement them yourself otherwise):

| Strategy | Behaviour | Good for |
|---|---|---|
| `reject` | Refuse the new run while one is active | Strict workflows |
| `enqueue` | Run the new message after the current run finishes | Chat where every message matters |
| `interrupt` | Stop the current run (keep its progress) and start the new one | "Actually, I meant…" corrections |
| `rollback` | Cancel the current run, **revert** its state, start fresh | Users who restart their question |

Self-hosted: use a per-thread distributed lock (e.g. a Redis lock keyed by thread_id) plus a queue.

---

## Q58. You change the graph (add or rename nodes, change state schema) while thousands of threads are mid-flight or paused on interrupts. What breaks, and how do you migrate safely?

**Risks:**
- A paused checkpoint whose `next` points to a node that **no longer exists**.
- A state key whose type changed, so old values fail deserialisation or validation.
- A changed reducer semantics, so merges behave differently.

**Safe practices:**
- **Additive changes** (new optional keys with defaults, new nodes) are generally safe.
- Don't rename or remove nodes that threads may be paused on. Keep them as **compatibility aliases** until the threads drain.
- Version graphs, e.g. `support_v3` and `support_v4` side by side; new threads use v4 and old threads finish on v3.
- Store `graph_version` in metadata and state.
- For long-lived threads, run a **migration job**: load the state, transform it, and `update_state` into the new shape (or fork into a new thread).
- Test with **replayed production checkpoints** in staging before deploying.

---

## Q59. How do you unit-test and integration-test LangGraph applications?

| Level | What | How |
|---|---|---|
| Node unit tests | Pure functions: state in, update out | Call the node directly with a fake state; mock LLMs (`GenericFakeChatModel`, `FakeListChatModel`) |
| Routing tests | Conditional edge functions | Assert the branch for each state |
| Graph integration | Full graph with fake model and `InMemorySaver` | Assert the final state and the visited path (via `stream_mode="updates"`) |
| Interrupt tests | Pause/resume flows | Invoke, assert `__interrupt__`, resume with `Command(resume=...)` |
| Partial execution | Start from a specific node with a given state | `update_state(cfg, values, as_node="x")` then `invoke(None, cfg)` |
| Evals | Quality on datasets | LangSmith datasets + evaluators run in CI; compare experiments across versions |

```python
from langchain_core.language_models.fake_chat_models import GenericFakeChatModel
from langchain_core.messages import AIMessage

def test_refund_requires_approval():
    g = b.compile(checkpointer=InMemorySaver())
    cfg = {"configurable": {"thread_id": "test-1"}}
    out = g.invoke({"order_id": "1766"}, cfg)
    assert out["__interrupt__"][0].value["amount"] == 4999.0
    final = g.invoke(Command(resume={"approved": False}), cfg)
    assert final["decision"] == "rejected"

def test_path_taken():
    fake = GenericFakeChatModel(messages=iter([AIMessage(content="billing")]))
    # inject the fake model via context or monkeypatch, then:
    visited = [list(u)[0] for u in triage.stream(inputs, cfg, stream_mode="updates")]
    assert visited == ["classify", "billing"]
```

---

## Q60. How do you scale a LangGraph service to thousands of concurrent users?

- **Async everywhere:** `ainvoke`/`astream`, async nodes, async checkpointer (`AsyncPostgresSaver`), and async HTTP clients.
- **Stateless API workers** behind a load balancer. All state lives in Postgres (checkpoints and store), so any worker can serve any thread.
- **Connection pooling** for Postgres (e.g. psycopg pool); size it to workers × concurrency.
- **Background task queue** for long runs (Redis/Celery or the platform's queue). Return a `run_id` and let clients stream or poll.
- **`max_concurrency`** for fan-out; per-provider semaphores; a gateway with fallbacks across model providers.
- **Durability tuning** (`async` for chat) and lean state (keep state small; store large blobs elsewhere).
- **Checkpoint growth:** TTLs, summarisation of messages, **(1.2+)** `DeltaChannel` (beta) to write message-list deltas instead of full copies.
- **(1.2+) Graceful shutdown:** drain runs at superstep boundaries during deploys so they resume cleanly.

```python
from psycopg_pool import AsyncConnectionPool
from langgraph.checkpoint.postgres.aio import AsyncPostgresSaver

pool = AsyncConnectionPool(DB_URI, max_size=50, kwargs={"autocommit": True, "prepare_threshold": 0})
checkpointer = AsyncPostgresSaver(pool)
# await checkpointer.setup()  (once, at deploy time)
graph = builder.compile(checkpointer=checkpointer)

async def handle(thread_id: str, text: str):
    async for mode, chunk in graph.astream({"messages": [("user", text)]},
                                           {"configurable": {"thread_id": thread_id}},
                                           stream_mode=["messages", "updates"]):
        yield mode, chunk
```

---

## Q61. Token costs are exploding in a long-running conversational graph. Which LangGraph-level techniques help?

1. **Summarise and delete** old messages with `RemoveMessage` (Q38). Or use `SummarizationMiddleware` with `create_agent`.
2. **Trim at call time** (`trim_messages`) without deleting history.
3. **Don't keep tool outputs or retrieved docs in `messages`.** Store them in separate state keys that get overwritten each turn.
4. **Private state per sub-agent** (subgraphs with their own schema), so each agent sees only what it needs.
5. **Model routing per node:** a small model for grading and routing, a large one only for final synthesis.
6. **Node caching** for repeated deterministic work (`CachePolicy`).
7. **Cost observability:** per-node token metrics and LangSmith cost tracking, sliced by tenant metadata.

---

## Q62. How do you run part of a graph in parallel but cap concurrency because of provider rate limits?

- `config["max_concurrency"]` caps the number of tasks running in parallel within a superstep (including `Send` fan-outs).
- Use `RetryPolicy` with backoff on rate-limit errors.
- Add a shared async semaphore or rate limiter around LLM calls (e.g. LangChain's `InMemoryRateLimiter` passed to the chat model).
- Batch items per `Send` task to reduce the number of calls (Q28).

```python
from langchain_core.rate_limiters import InMemoryRateLimiter
limiter = InMemoryRateLimiter(requests_per_second=5, check_every_n_seconds=0.1, max_bucket_size=10)
llm = init_chat_model("openai:gpt-4.1-mini", rate_limiter=limiter)
graph.invoke(inputs, {"max_concurrency": 10})
```

---

## Q63. How would you stream only the final answer's tokens to the UI, but show step-level progress for the other nodes?

Use multiple stream modes and filter `messages` by `langgraph_node` (or by tags you attach to the model). Q15 shows the code.

Alternatives:
- Tag the final model, e.g. `llm.with_config(tags=["final_answer"])`, and filter on `metadata["tags"]`.
- Use `stream_mode="custom"` to emit friendly status events from inside nodes ("Checking your order…").

---

## Q64. When would you NOT use LangGraph?

- A single prompt or simple RAG chain with no loops, state, or approvals. A plain function or LCEL chain is simpler.
- Hard real-time, ultra-low-latency paths (< 50 ms), where framework overhead and checkpoint writes matter.
- Pure data pipelines with no LLM decisions. Use an orchestrator like Airflow, Dagster, or Temporal (though LangGraph can be *called* from them).
- Teams that need language-agnostic durable execution across many services may prefer **Temporal** or similar, and call LLM logic inside activities.

**Trade-off summary:** LangGraph adds structure, persistence, and observability at the cost of learning curve, extra abstraction, and checkpoint storage and latency.

---

## Q65. Compare LangGraph with other agent frameworks.

| Framework | Model | Strengths | When to prefer |
|---|---|---|---|
| **LangGraph** | Explicit state graph + durable runtime | Fine-grained control, HITL, persistence, time travel, streaming, LangSmith integration | Production agents that need control and durability |
| CrewAI | Role-based crews and tasks | Fast multi-agent prototyping, high-level | Simple role-play collaborations |
| AutoGen / Microsoft Agent Framework | Conversational multi-agent | Agent-to-agent chat patterns | Research, conversational agent teams |
| LlamaIndex Workflows | Event-driven steps | Strong data/RAG ecosystem | RAG-heavy apps |
| OpenAI Agents SDK / provider SDKs | Lightweight agent loop + handoffs | Simple, provider-native | Single-provider apps, simple handoffs |
| Temporal | Durable workflow engine (general-purpose) | Rock-solid durability, polyglot | Mission-critical long workflows; LLM calls as activities |

---

## Q66. How do you implement "edit a previous user message and regenerate" in a chat app built on LangGraph?

Use time travel:
1. Find the checkpoint **just before** the node that processed that message.
2. `update_state` with an edited `HumanMessage` that has the **same message ID** (so `add_messages` replaces it).
3. Continue from the returned config.

The old branch remains in history, which supports "version 1 / version 2" answer switching in the UI.

```python
def edit_and_regenerate(graph, thread_id: str, message_id: str, new_text: str):
    cfg = {"configurable": {"thread_id": thread_id}}
    for snap in graph.get_state_history(cfg):
        msgs = snap.values.get("messages", [])
        if msgs and msgs[-1].id == message_id and snap.next:       # state right after the user msg was added
            edited = HumanMessage(content=new_text, id=message_id)
            fork_cfg = graph.update_state(snap.config, {"messages": [edited]})
            return graph.invoke(None, fork_cfg)
    raise ValueError("message not found")
```

---

## Q67. How do you evaluate a LangGraph agent (offline and online)?

- **Final-response evals:** correctness and helpfulness against a reference (LLM-as-judge or exact checks).
- **Trajectory evals:** did the agent call the right tools in a reasonable order? Compare the visited nodes and tool calls with an expected trajectory (strict, unordered, or subset match).
- **Single-step evals:** test one node's decision (e.g. a router) in isolation over many inputs.
- **Online evals:** run sampled production traces through evaluators; monitor feedback and success rates.
- Version experiments by graph and prompt version metadata. Gate deploys in CI on eval thresholds.

```python
from langsmith import Client
from langsmith.evaluation import evaluate

def target(inputs: dict) -> dict:
    out = triage.invoke({"messages": [("user", inputs["question"])]},
                        {"configurable": {"thread_id": str(uuid.uuid4())}})
    return {"route": out["route"].category, "answer": out["messages"][-1].content}

def correct_route(run, example) -> dict:
    return {"key": "route_correct", "score": int(run.outputs["route"] == example.outputs["route"])}

evaluate(target, data="triage-golden-set", evaluators=[correct_route],
         experiment_prefix="triage-v4", metadata={"graph_version": "v4"})
```

---

## Q68. Walk through the end-to-end architecture of a production LangGraph customer-support agent.

```
Client (web / WhatsApp) ──SSE──▶ API (FastAPI, auth, rate limit, double-text lock)
                                    │ config: thread_id, run_id, tags, metadata, context(tenant, user)
                                    ▼
                    LangGraph app (async, stateless replicas)
  input_guard ─▶ classify ─▶ {billing | tech | returns}_agent (subgraphs w/ tools)
        │                         │  sensitive tools ─▶ interrupt() ─▶ human approval UI
        ▼                         ▼
     safe_reply             output_guard ─▶ memory_extraction (background) ─▶ END
                                    │
   Postgres: checkpoints (encrypted, TTL) · Store: user memories (semantic index, TTL)
   Redis: locks, rate limits, caches   · Vector DB: KB retrieval
                                    │
 Observability: LangSmith / OTel traces (sampled, PII-anonymised, tagged by tenant & version)
                Prometheus metrics (node latency, tokens, errors, pending interrupts)
                Feedback ▶ datasets ▶ offline evals in CI ▶ canary deploy of new graph version
```

Key talking points:
- explicit control where the business needs it (guards, approvals),
- agentic flexibility inside domain sub-agents,
- durable state,
- observable at node granularity,
- versioned and evaluated releases.

---

# Part E — Rapid-Fire Questions & Cheat Sheet

| # | Question | Short answer |
|---|---|---|
| 1 | What's the default behaviour when two nodes write the same key in one step without a reducer? | `InvalidUpdateError` |
| 2 | What does a node return to update state? | A partial dict (or `Command`, or `None`) |
| 3 | `START` / `END`? | Virtual entry and exit nodes |
| 4 | What's required for `interrupt()` to work? | A checkpointer and a `thread_id` |
| 5 | How do you resume an interrupt? | `graph.invoke(Command(resume=value), same_config)` |
| 6 | What re-runs on resume? | The interrupted node, from its start |
| 7 | Checkpointer vs Store? | Per-thread state history vs cross-thread long-term memory |
| 8 | How do you fan out over a runtime-sized list? | Return `[Send(node, payload), ...]` from a conditional edge |
| 9 | How do you route to a sibling agent from inside a subgraph? | `Command(goto="x", graph=Command.PARENT)` |
| 10 | How do you stream LLM tokens from inside nodes? | `stream_mode="messages"` |
| 11 | How do you emit custom progress events? | `get_stream_writer()` / `runtime.stream_writer` + `stream_mode="custom"` |
| 12 | How do you stop infinite loops? | Set `recursion_limit` explicitly (the 1.2.x default is 10,007), plus step counters and repeated-call detection |
| 13 | How do you pass per-request dependencies (user, model)? | `context_schema` + `Runtime` (`context=` at invoke) |
| 14 | How do you see where a paused graph will go next? | `graph.get_state(cfg).next` |
| 15 | How do you replay or fork from history? | `get_state_history` + `checkpoint_id` / `update_state` |
| 16 | How do you make a join wait for uneven branches? | `add_edge([a, b], join)` or `defer=True` |
| 17 | How do you retry flaky nodes? | `RetryPolicy` on `add_node` |
| 18 | How do you cache expensive deterministic nodes? | `CachePolicy` + `compile(cache=...)` |
| 19 | Which durability mode for expensive batch jobs? | `durability="sync"` |
| 20 | How do you visualise a graph? | `graph.get_graph(xray=True).draw_mermaid()` |
| 21 | Enable LangSmith tracing? | `LANGSMITH_TRACING=true` + `LANGSMITH_API_KEY` (+ `LANGSMITH_PROJECT`) |
| 22 | Reduce trace volume? | `LANGSMITH_TRACING_SAMPLING_RATE` |
| 23 | Mask PII in traces? | LangSmith anonymizer, `tracing_context(enabled=False)`, hide-I/O env vars |
| 24 | Export to Datadog or Grafana? | `LANGSMITH_OTEL_ENABLED=true` + `OTEL_EXPORTER_OTLP_ENDPOINT` |
| 25 | Run locally with Studio? | `langgraph dev` |
| 26 | Prebuilt agent in 1.x? | `langchain.agents.create_agent` (replaces `create_react_agent`) |
| 27 | Workflow vs agent? | Developer-defined vs LLM-decided control flow |
| 28 | Graph API vs Functional API? | Explicit nodes/edges vs `@entrypoint`/`@task` with Python control flow |
| 29 | Encrypt checkpoints? | `EncryptedSerializer` (AES key via env) |
| 30 | Handle concurrent messages on one thread? | Double-texting strategy: reject / enqueue / interrupt / rollback |

### Concept checklist (make sure you can explain each one)

State · TypedDict/Pydantic schemas · reducers (`operator.add`, `add_messages`, custom) · `MessagesState` · `RemoveMessage` · nodes · normal/conditional edges · `START`/`END` · `add_sequence` · fan-out/fan-in · `Send` · `Command` (update/goto/graph/resume) · supersteps (Pregel/BSP) · `compile()` · checkpointers (InMemory/Sqlite/Postgres) · threads · `StateSnapshot` · durability modes · `interrupt()` and HITL · static breakpoints · time travel (`get_state_history`, `update_state`, `checkpoint_id`) · Store and namespaces and semantic search · short- vs long-term memory · `context_schema`/`Runtime` · `RunnableConfig` · `recursion_limit` · `max_concurrency` · streaming modes (values/updates/messages/custom/debug) · subgraphs (shared vs transformed state; `Command.PARENT`) · input/output/private schemas · `RetryPolicy` · `CachePolicy` · deferred nodes · error handlers and timeouts (1.2+) · Functional API · `ToolNode`/`tools_condition` · `create_agent` and middleware · multi-agent (supervisor, swarm/handoffs, hierarchical) · LangSmith tracing, tags, metadata, feedback, sampling, anonymizers · OpenTelemetry · Langfuse · Prometheus metrics · LangGraph Studio/`langgraph dev` · `langgraph.json` · TTLs · encryption · double-texting · graph versioning and migration · testing and evals.
