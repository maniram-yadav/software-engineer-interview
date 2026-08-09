# Design a Graph Database — High-Level Design Document

## 1. Requirements

### Functional Requirements
- Store nodes (entities) and edges (relationships) with associated properties
- Efficient traversal queries: "find all friends-of-friends," "shortest path between A and B"
- Support both shallow (1-2 hop) and deep (multi-hop) graph traversals
- Support property-based filtering during traversal (e.g., "friends who live in NYC")
- Handle highly-connected "supernodes" (e.g., a celebrity with millions of followers) gracefully

### Non-Functional Requirements
- **Traversal performance:** Multi-hop queries must remain fast even as the graph grows — this is the defining challenge, since naive approaches degrade exponentially with hop depth
- **Scale:** Billions of nodes, tens of billions of edges
- **Write throughput:** Support continuous edge creation (e.g., new social connections, transactions)
- **Flexible schema:** Different node/edge types with varying properties, without rigid upfront schema design

### Back-of-Envelope Estimation
| Metric | Value |
|---|---|
| Nodes | ~1B (e.g., users) |
| Edges | ~50B (e.g., social connections, avg 50 edges/node) |
| Traversal queries/sec | ~50,000 |
| Typical query depth | 1-3 hops (deeper queries are rare and expensive) |
| Supernode edge count | Can reach millions (celebrity accounts) |

---

## 2. Why Graph Databases Exist — The Core Problem With Relational Joins

```mermaid
flowchart TB
    A["Query: 'Find friends-of-friends<br/>of User X' in a RELATIONAL DB"] --> B["Requires a SELF-JOIN on<br/>the friendships table"]
    B --> C["1-hop: 1 join<br/>(manageable)"]
    B --> D["2-hop: 2 joins<br/>(getting expensive)"]
    B --> E["3-hop: 3 joins<br/>(often prohibitively slow —<br/>join cost compounds<br/>with each additional hop)"]

    F["Graph databases solve this by<br/>making relationship traversal<br/>a FIRST-CLASS, INDEX-FREE<br/>operation — each node directly<br/>stores POINTERS to its<br/>adjacent edges, so hopping<br/>to a neighbor is a direct<br/>memory/disk reference,<br/>not a search/join operation"] --> G["This is called<br/>'index-free adjacency' —<br/>the single most important<br/>architectural difference<br/>from relational storage"]
```

---

## 3. High-Level Architecture

```mermaid
flowchart TB
    Client["Client / Query"]
    QueryEngine["Graph Query Engine<br/>(Cypher/Gremlin-style)"]

    subgraph Storage["Storage Layer"]
        NodeStore[("Node Store<br/>node_id → properties")]
        EdgeStore[("Edge Store<br/>index-free adjacency lists")]
        PropIndex[("Property Index<br/>(for filtered lookups,<br/>e.g., 'find node WHERE<br/>city=NYC')")]
    end

    subgraph Partitioning["Graph Partitioning Layer"]
        Shard1["Partition 1<br/>(node subgraph A)"]
        Shard2["Partition 2<br/>(node subgraph B)"]
        Shard3["Partition 3<br/>(node subgraph C)"]
    end

    Client --> QueryEngine
    QueryEngine --> PropIndex
    QueryEngine --> NodeStore
    QueryEngine --> EdgeStore

    NodeStore --> Shard1
    NodeStore --> Shard2
    NodeStore --> Shard3
    EdgeStore --> Shard1
    EdgeStore --> Shard2
    EdgeStore --> Shard3
```

---

## 4. Storage Model — Index-Free Adjacency

```mermaid
flowchart TB
    A["Node: User_A"] --> B["Adjacency List<br/>(stored DIRECTLY with the node,<br/>not in a separate join table)"]
    B --> C["Edge → User_B (type: FRIEND,<br/>since: 2020, direct pointer<br/>to User_B's storage location)"]
    B --> D["Edge → User_C (type: FRIEND,<br/>direct pointer)"]
    B --> E["Edge → Post_123 (type: LIKED,<br/>direct pointer)"]

    F["Traversing from User_A<br/>to User_B is a DIRECT<br/>pointer dereference —<br/>O(1) relative to graph size,<br/>NOT a search through a<br/>separate index or join table"] --> G["This is what allows<br/>multi-hop traversal to remain<br/>fast: each hop is proportional<br/>only to that node's OWN<br/>degree (number of edges),<br/>never to the TOTAL size<br/>of the graph"]
```

```mermaid
erDiagram
    NODE {
        string node_id PK
        string label "e.g. User, Product"
        map properties
        list outgoing_edge_pointers
        list incoming_edge_pointers
    }
    EDGE {
        string edge_id PK
        string from_node_id FK
        string to_node_id FK
        string edge_type
        map properties
    }
```

---

## 5. Traversal Query Flow — Detailed Sequence

```mermaid
sequenceDiagram
    participant C as Client
    participant QE as Query Engine
    participant NodeStore as Node Store
    participant EdgeStore as Edge Store

    C->>QE: Query: MATCH (a:User {id:'X'})<br/>-[:FRIEND]->(b)-[:FRIEND]->(c)<br/>RETURN c (friends-of-friends)

    QE->>NodeStore: Lookup starting node X
    NodeStore-->>QE: Node X's adjacency list

    QE->>EdgeStore: Follow FRIEND edges from X<br/>(direct pointer traversal,<br/>NOT an index search)
    EdgeStore-->>QE: Set of direct friends {B1, B2, B3...}

    loop For each direct friend
        QE->>EdgeStore: Follow FRIEND edges<br/>from this friend<br/>(again, direct pointers)
        EdgeStore-->>QE: Their friends (2-hop results)
    end

    QE->>QE: Deduplicate, exclude X<br/>and X's direct friends<br/>from final result set
    QE-->>C: Return friends-of-friends
```

**Why this stays fast at 2-3 hops but gets expensive beyond that:** Each hop's cost is proportional to the AVERAGE DEGREE (number of edges) of nodes at that level — for a typical social graph with ~50 average friends, 2 hops touches ~50×50 = 2,500 nodes, which is very manageable. But this branches multiplicatively with each additional hop — this is precisely why deep multi-hop traversals are rare in practice and often require specialized handling (see Section 7).

---

## 6. Graph Partitioning Strategy

```mermaid
flowchart TB
    A["Graph too large for<br/>a single machine —<br/>must be partitioned"] --> B{"Partitioning Strategy"}

    B --> C["Random/Hash Partitioning<br/>(simple, but ignores<br/>graph structure)"]
    C --> C1["Problem: friends often<br/>land on DIFFERENT partitions,<br/>making most traversals<br/>require expensive<br/>cross-partition network hops"]

    B --> D["Locality-Aware Partitioning<br/>(e.g., METIS-style graph<br/>partitioning algorithms)"]
    D --> D1["Groups densely-connected<br/>subgraphs onto the SAME<br/>partition — minimizes edges<br/>that cross partition boundaries"]
    D --> D2["Most traversals for a<br/>'community' of connected<br/>users stay within one<br/>partition — fast, local hops"]

    E["Fundamental tradeoff:<br/>perfect partitioning is<br/>NP-hard for large graphs —<br/>production systems use<br/>heuristic algorithms and<br/>accept some cross-partition<br/>traversal cost as unavoidable"]
```

---

## 7. Handling Supernodes (Celebrity Problem, Graph Edition)

```mermaid
flowchart TB
    A["A celebrity node has<br/>10 million follower edges"] --> B["Naive traversal:<br/>'get all followers'<br/>returns 10 million results —<br/>expensive regardless of<br/>storage efficiency"]

    B --> C{"Mitigation Strategies"}
    C --> D["Edge count limits on<br/>full traversal — paginate<br/>results instead of returning<br/>the entire adjacency list<br/>at once"]
    C --> E["Precompute/cache common<br/>aggregate queries for<br/>supernodes (e.g., 'follower<br/>count' as a stored property,<br/>not a live traversal count)"]
    C --> F["Special-case supernode<br/>partitioning — sometimes<br/>replicated or specially<br/>distributed rather than<br/>pinned to one partition,<br/>to avoid a single-partition<br/>hotspot"]

    G["This is structurally the<br/>SAME celebrity/hot-key problem<br/>seen in the Twitter and<br/>Uber designs — graph databases<br/>aren't immune to it just<br/>because relationships are<br/>modeled explicitly"]
```

---

## 8. Property-Filtered Traversal

```mermaid
sequenceDiagram
    participant C as Client
    participant QE as Query Engine
    participant PropIdx as Property Index
    participant EdgeStore as Edge Store

    C->>QE: MATCH (a:User {id:'X'})-[:FRIEND]->(b:User {city:'NYC'})<br/>RETURN b

    QE->>EdgeStore: Get direct friends of X<br/>(index-free adjacency)
    EdgeStore-->>QE: Candidate set {B1, B2, B3, ...}

    QE->>PropIdx: Filter candidates WHERE city='NYC'
    PropIdx-->>QE: Subset matching the property filter

    QE-->>C: Return filtered friends
```

*Note that the traversal itself (finding direct friends) uses index-free adjacency, while the property filter (city='NYC') uses a separate property index — these are two distinct access patterns within the same query, each using the storage structure best suited to it.*

---

## 9. Shortest Path Query (Common Graph Algorithm)

```mermaid
flowchart TB
    A["Find shortest path<br/>between Node A and Node Z"] --> B["Bidirectional BFS<br/>(Breadth-First Search)"]
    B --> C["Expand outward from A<br/>AND from Z simultaneously"]
    C --> D["Stop as soon as the<br/>two expanding frontiers<br/>meet at a common node"]

    E["Why bidirectional over<br/>single-direction BFS?"] --> F["Single-direction BFS from A<br/>to depth D explores ~degree^D nodes.<br/>Bidirectional BFS only needs<br/>each side to expand to depth D/2 —<br/>since degree^(D/2) × 2 is<br/>vastly smaller than degree^D<br/>for any meaningfully connected<br/>graph, this is a dramatic<br/>practical speedup"]
```

---

## 10. Component Responsibilities Summary

```mermaid
mindmap
  root((Graph Database HLD))
    Node Store
      Entity storage
      Adjacency list pointers
    Edge Store
      Index-free adjacency
      Direct pointer traversal
    Property Index
      Filtered lookup support
      Separate from traversal path
    Query Engine
      Cypher/Gremlin-style parsing
      Multi-hop traversal execution
    Graph Partitioner
      Locality-aware sharding
      Minimizes cross-partition hops
    Supernode Handler
      Pagination/caching
      Hotspot mitigation
```

---

## 11. Key Design Decisions & Tradeoffs

| Decision | Choice | Why |
|---|---|---|
| Core storage principle | Index-free adjacency (direct pointers between connected nodes) | Makes traversal cost proportional to node degree, not total graph size — the fundamental advantage over relational joins for relationship-heavy queries |
| Partitioning | Locality-aware (community-preserving) over naive hash | Minimizes expensive cross-partition network hops for typical traversal patterns, at the cost of harder (NP-hard, heuristic) partitioning computation |
| Supernode handling | Pagination + precomputed aggregates + special partitioning | The same celebrity/hot-key problem recurs in graph form; naive full-traversal doesn't scale for extremely high-degree nodes |
| Shortest path algorithm | Bidirectional BFS | Dramatically reduces the search space compared to single-direction BFS, exploiting the exponential blowup of naive traversal |
| Property filtering | Separate property index, combined with adjacency traversal | Uses the right access structure for each part of a query — traversal via pointers, filtering via index |
| Query depth | Optimized for 1-3 hops; deeper queries treated as special cases | Reflects the practical reality that traversal cost compounds multiplicatively with hop depth |

---

## 12. Bottlenecks & Scaling Considerations

- **Cross-partition traversal cost** — even with locality-aware partitioning, some fraction of edges will inevitably cross partition boundaries (especially in graphs with genuinely global connectivity, like social networks); these hops require network round trips instead of local pointer dereferences, and are the dominant latency cost for queries that touch them.
- **Supernodes remain a structural challenge regardless of mitigation** — no amount of clever engineering makes "return all 10 million followers" fast in an absolute sense; the real solution is usually re-framing the product requirement (e.g., "show a sample" or "show aggregate stats" rather than the full list).
- **Deep traversal queries are fundamentally expensive** — multi-hop queries beyond 3-4 hops in a densely connected graph explore an enormous number of nodes; production systems often impose hard depth limits or require these queries to run as offline batch jobs (e.g., using a graph processing framework like Pregel/GraphX) rather than live online queries.
- **Write amplification for highly-connected nodes** — adding a new edge means updating the adjacency lists of BOTH connected nodes; for a supernode, even a single new edge touches a node whose adjacency structure may already be under heavy read/write contention.
- **Graph algorithm complexity beyond simple traversal** — more sophisticated queries (community detection, PageRank-style centrality, recommendation via graph embeddings) often can't run efficiently as live online queries at all — these typically run as separate offline batch computations (see the Recommendation System design's collaborative filtering approach) with results periodically materialized back into the graph or a separate serving store.
- **Schema flexibility vs query optimization tension** — the flexible, evolving-schema nature that makes graph databases attractive for complex relationship modeling also makes query planning/optimization harder than in a rigid relational schema, where the query planner has much more upfront structural information to exploit.
