# Complete Graph Algorithms — Problems & Implementations
### Brute Force → Optimized · All Difficulty Levels · Java

---

## 📋 Index

| # | Topic | Problem | Difficulty |
|---|---|---|---|
| 1 | Representation | Adjacency List vs Matrix | 🟢 |
| 2 | Traversal | Number of Islands / Connected Components | 🟢 |
| 3 | Traversal | Flood Fill | 🟢 |
| 4 | Traversal | Path Exists in Graph | 🟢 |
| 5 | Traversal | Clone Graph | 🟡 |
| 6 | Topological Sort | Course Schedule I & II | 🟡 |
| 7 | Cycle Detection | Directed & Undirected Graph | 🟡 |
| 8 | Bipartite Check | Is Graph Bipartite | 🟡 |
| 9 | Union-Find | Number of Provinces / Redundant Connection | 🟡 |
| 10 | Shortest Path | Dijkstra's Algorithm | 🟡/🔴 |
| 11 | Shortest Path | Bellman-Ford Algorithm | 🔴 |
| 12 | Shortest Path | Floyd-Warshall (All Pairs) | 🔴 |
| 13 | MST | Kruskal's Algorithm | 🔴 |
| 14 | MST | Prim's Algorithm | 🔴 |
| 15 | Advanced | Bridges in Graph (Tarjan's) | 🔴 |
| 16 | Advanced | Articulation Points (Tarjan's) | 🔴 |
| 17 | Advanced | Strongly Connected Components (Kosaraju's & Tarjan's) | ⚫ |
| 18 | Advanced | Maximum Flow (Ford-Fulkerson / Edmonds-Karp / Dinic's) | ⚫ |
| 19 | Advanced | Bidirectional BFS (Word Ladder) | ⚫ |
| 20 | Advanced | Traveling Salesman (Bitmask DP) | ⚫ |
| 21 | Advanced | Alien Dictionary (Topo Sort on implicit graph) | 🔴 |

> All snippets compile under Java 17+ (`javac`). Helper classes (`DSU`, `Node`, etc.) are defined once and reused — assume they're in scope for later sections.

---

## 1. Graph Representation 🟢

```java
import java.util.*;

// ---------- Adjacency Matrix ----------
// Space: O(V^2) | Edge lookup: O(1) | Good for dense graphs
class GraphMatrix {
    int n;
    int[][] matrix;

    GraphMatrix(int n) {
        this.n = n;
        matrix = new int[n][n];
    }

    void addEdge(int u, int v, int w, boolean directed) {
        matrix[u][v] = w;
        if (!directed) matrix[v][u] = w;
    }
}

// ---------- Adjacency List ----------
// Space: O(V+E) | Edge lookup: O(degree) | Good for sparse graphs (default choice)
class GraphList {
    Map<Integer, List<int[]>> adj = new HashMap<>(); // node -> list of {neighbor, weight}

    void addEdge(int u, int v, int w, boolean directed) {
        adj.computeIfAbsent(u, k -> new ArrayList<>()).add(new int[]{v, w});
        if (!directed) adj.computeIfAbsent(v, k -> new ArrayList<>()).add(new int[]{u, w});
    }
}
```
**Complexity:** Matrix → O(V²) space, O(1) edge check, O(V²) to enumerate all edges.
Adjacency List → O(V+E) space, O(deg(v)) edge check, O(V+E) to enumerate all edges.
**Rule of thumb:** dense graph (E ≈ V²) → matrix; sparse graph (E ≈ V) → list (default choice in 95% of interview problems).

---

## 2. Number of Islands / Connected Components 🟢

**Problem:** Given a grid of `1`s (land) and `0`s (water), count the number of islands (4-directionally connected components).

There's no meaningfully "worse" brute force beyond the standard traversal — the naive correct approach *is* DFS/BFS with a visited marker. We show DFS, BFS, and Union-Find (needed for the streaming follow-up).

### Optimized Solution 1 — DFS (in-place marking, O(1) extra space)
```java
class NumIslands {
    public int numIslandsDFS(char[][] grid) {
        if (grid == null || grid.length == 0) return 0;
        int rows = grid.length, cols = grid[0].length;
        int count = 0;
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (grid[r][c] == '1') {
                    count++;
                    dfs(grid, r, c);
                }
            }
        }
        return count;
    }

    private void dfs(char[][] grid, int r, int c) {
        int rows = grid.length, cols = grid[0].length;
        if (r < 0 || r >= rows || c < 0 || c >= cols || grid[r][c] != '1') return;
        grid[r][c] = '0'; // mark visited in-place
        dfs(grid, r + 1, c);
        dfs(grid, r - 1, c);
        dfs(grid, r, c + 1);
        dfs(grid, r, c - 1);
    }
}
```
**Complexity:** Time O(R·C) — each cell visited once. Space O(R·C) worst case (recursion stack for an all-land grid).

### Optimized Solution 2 — BFS (avoids recursion stack overflow on large grids)
```java
class NumIslandsBFS {
    public int numIslandsBFS(char[][] grid) {
        if (grid == null || grid.length == 0) return 0;
        int rows = grid.length, cols = grid[0].length;
        int count = 0;
        int[][] dirs = {{1,0},{-1,0},{0,1},{0,-1}};

        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (grid[r][c] == '1') {
                    count++;
                    Queue<int[]> q = new LinkedList<>();
                    q.offer(new int[]{r, c});
                    grid[r][c] = '0';
                    while (!q.isEmpty()) {
                        int[] cell = q.poll();
                        for (int[] d : dirs) {
                            int nr = cell[0] + d[0], nc = cell[1] + d[1];
                            if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr][nc] == '1') {
                                grid[nr][nc] = '0';
                                q.offer(new int[]{nr, nc});
                            }
                        }
                    }
                }
            }
        }
        return count;
    }
}
```
**Complexity:** Time O(R·C), Space O(min(R,C)) queue size — better worst-case space than recursive DFS.

### Optimized Solution 3 — Union-Find (needed for streaming/dynamic version: "Number of Islands II")
```java
class DSU {
    int[] parent, rank;
    int count; // number of distinct components

    DSU(int n) {
        parent = new int[n];
        rank = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
    }

    int find(int x) {
        while (parent[x] != x) {
            parent[x] = parent[parent[x]]; // path compression
            x = parent[x];
        }
        return x;
    }

    void union(int x, int y) {
        int rx = find(x), ry = find(y);
        if (rx == ry) return;
        if (rank[rx] < rank[ry]) { int tmp = rx; rx = ry; ry = tmp; }
        parent[ry] = rx;
        if (rank[rx] == rank[ry]) rank[rx]++;
        count--;
    }
}

class NumIslandsII {
    public List<Integer> numIslands2(int rows, int cols, int[][] positions) {
        DSU dsu = new DSU(rows * cols);
        int[][] grid = new int[rows][cols];
        List<Integer> result = new ArrayList<>();
        int[][] dirs = {{1,0},{-1,0},{0,1},{0,-1}};

        for (int[] pos : positions) {
            int r = pos[0], c = pos[1];
            if (grid[r][c] == 1) {
                result.add(dsu.count);
                continue;
            }
            grid[r][c] = 1;
            dsu.count++;
            for (int[] d : dirs) {
                int nr = r + d[0], nc = c + d[1];
                if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr][nc] == 1) {
                    dsu.union(r * cols + c, nr * cols + nc);
                }
            }
            result.add(dsu.count);
        }
        return result;
    }
}
```
**Complexity:** Time O(k · α(R·C)) for k updates (α = inverse Ackermann, practically constant). This beats re-running DFS/BFS from scratch after every update (which would be O(k·R·C)).

---

## 3. Flood Fill 🟢

```java
class FloodFill {
    public int[][] floodFill(int[][] image, int sr, int sc, int newColor) {
        int oldColor = image[sr][sc];
        if (oldColor == newColor) return image;
        dfs(image, sr, sc, oldColor, newColor);
        return image;
    }

    private void dfs(int[][] image, int r, int c, int oldColor, int newColor) {
        int rows = image.length, cols = image[0].length;
        if (r < 0 || r >= rows || c < 0 || c >= cols || image[r][c] != oldColor) return;
        image[r][c] = newColor;
        dfs(image, r + 1, c, oldColor, newColor);
        dfs(image, r - 1, c, oldColor, newColor);
        dfs(image, r, c + 1, oldColor, newColor);
        dfs(image, r, c - 1, oldColor, newColor);
    }
}
```
**Complexity:** O(R·C) time, O(R·C) worst-case recursion space. Direct DFS-graph-traversal application — no better asymptotic approach exists since every reachable pixel must be visited.

---

## 4. Path Exists in Graph 🟢

### Brute Force / Base Method — DFS (fine for a single query, contrasted with Union-Find below)
```java
class ValidPath {
    public boolean validPathDFS(int n, int[][] edges, int source, int destination) {
        Map<Integer, List<Integer>> adj = new HashMap<>();
        for (int[] e : edges) {
            adj.computeIfAbsent(e[0], k -> new ArrayList<>()).add(e[1]);
            adj.computeIfAbsent(e[1], k -> new ArrayList<>()).add(e[0]);
        }
        boolean[] visited = new boolean[n];
        return dfs(adj, visited, source, destination);
    }

    private boolean dfs(Map<Integer, List<Integer>> adj, boolean[] visited, int node, int dest) {
        if (node == dest) return true;
        visited[node] = true;
        for (int nei : adj.getOrDefault(node, Collections.emptyList())) {
            if (!visited[nei] && dfs(adj, visited, nei, dest)) return true;
        }
        return false;
    }
}
```
**Complexity:** O(V+E) time, O(V+E) space.

### Optimized (for MULTIPLE queries) — Union-Find, preprocess once
```java
class ValidPathQueries {
    public boolean[] checkPaths(int n, int[][] edges, int[][] queries) {
        DSU dsu = new DSU(n);
        for (int[] e : edges) dsu.union(e[0], e[1]);
        boolean[] result = new boolean[queries.length];
        for (int i = 0; i < queries.length; i++) {
            result[i] = dsu.find(queries[i][0]) == dsu.find(queries[i][1]);
        }
        return result;
    }
}
```
**Complexity:** Preprocess O(E·α(n)); each query O(α(n)) ≈ O(1). Beats re-running DFS O(V+E) per query when there are many queries.

---

## 5. Clone Graph 🟡

```java
class Node {
    int val;
    List<Node> neighbors;
    Node(int val) { this.val = val; neighbors = new ArrayList<>(); }
}
```

### Solution 1 — DFS with HashMap
```java
class CloneGraphDFS {
    public Node cloneGraph(Node node) {
        if (node == null) return null;
        Map<Node, Node> visited = new HashMap<>();
        return dfs(node, visited);
    }

    private Node dfs(Node node, Map<Node, Node> visited) {
        if (visited.containsKey(node)) return visited.get(node);
        Node clone = new Node(node.val);
        visited.put(node, clone);
        for (Node nei : node.neighbors) {
            clone.neighbors.add(dfs(nei, visited));
        }
        return clone;
    }
}
```

### Solution 2 — BFS with HashMap (avoids deep recursion on large graphs)
```java
class CloneGraphBFS {
    public Node cloneGraph(Node node) {
        if (node == null) return null;
        Map<Node, Node> visited = new HashMap<>();
        visited.put(node, new Node(node.val));
        Queue<Node> q = new LinkedList<>();
        q.offer(node);

        while (!q.isEmpty()) {
            Node curr = q.poll();
            for (Node nei : curr.neighbors) {
                if (!visited.containsKey(nei)) {
                    visited.put(nei, new Node(nei.val));
                    q.offer(nei);
                }
                visited.get(curr).neighbors.add(visited.get(nei));
            }
        }
        return visited.get(node);
    }
}
```
**Complexity (both):** O(V+E) time, O(V) space for the hashmap.

---

## 6. Topological Sort — Course Schedule I & II 🟡

### Brute Force — Repeatedly scan for a node with in-degree 0, remove it, repeat
```java
class CourseScheduleBrute {
    public boolean canFinishBrute(int numCourses, int[][] prerequisites) {
        Map<Integer, List<Integer>> adj = new HashMap<>();
        int[] indegree = new int[numCourses];
        for (int[] p : prerequisites) {
            adj.computeIfAbsent(p[1], k -> new ArrayList<>()).add(p[0]);
            indegree[p[0]]++;
        }

        boolean[] removed = new boolean[numCourses];
        int processed = 0;

        for (int iter = 0; iter < numCourses; iter++) {
            int found = -1;
            for (int i = 0; i < numCourses; i++) {          // O(V) scan every iteration
                if (!removed[i] && indegree[i] == 0) { found = i; break; }
            }
            if (found == -1) return false;                  // cycle detected
            removed[found] = true;
            processed++;
            for (int nei : adj.getOrDefault(found, Collections.emptyList())) {
                indegree[nei]--;
            }
        }
        return processed == numCourses;
    }
}
```
**Complexity:** O(V²+E) time (linear scan for zero-indegree node repeated V times), O(V+E) space.

### Optimized Solution 1 — Kahn's Algorithm (BFS with a queue of zero-indegree nodes)
```java
class CourseScheduleKahn {
    public int[] findOrder(int numCourses, int[][] prerequisites) {
        Map<Integer, List<Integer>> adj = new HashMap<>();
        int[] indegree = new int[numCourses];
        for (int[] p : prerequisites) {
            adj.computeIfAbsent(p[1], k -> new ArrayList<>()).add(p[0]);
            indegree[p[0]]++;
        }

        Queue<Integer> q = new LinkedList<>();
        for (int i = 0; i < numCourses; i++) if (indegree[i] == 0) q.offer(i);

        int[] order = new int[numCourses];
        int idx = 0;
        while (!q.isEmpty()) {
            int node = q.poll();
            order[idx++] = node;
            for (int nei : adj.getOrDefault(node, Collections.emptyList())) {
                if (--indegree[nei] == 0) q.offer(nei);
            }
        }
        return idx == numCourses ? order : new int[0]; // empty => cycle exists
    }
}
```
**Complexity:** O(V+E) time, O(V+E) space.

### Optimized Solution 2 — DFS Post-order + Reverse (3-color cycle detection)
```java
class CourseScheduleDFS {
    private static final int WHITE = 0, GRAY = 1, BLACK = 2;

    public int[] findOrder(int numCourses, int[][] prerequisites) {
        Map<Integer, List<Integer>> adj = new HashMap<>();
        for (int[] p : prerequisites) {
            adj.computeIfAbsent(p[1], k -> new ArrayList<>()).add(p[0]);
        }

        int[] color = new int[numCourses];
        List<Integer> order = new ArrayList<>();

        for (int i = 0; i < numCourses; i++) {
            if (color[i] == WHITE && !dfs(i, adj, color, order)) {
                return new int[0];
            }
        }
        Collections.reverse(order);
        return order.stream().mapToInt(Integer::intValue).toArray();
    }

    private boolean dfs(int node, Map<Integer, List<Integer>> adj, int[] color, List<Integer> order) {
        color[node] = GRAY;
        for (int nei : adj.getOrDefault(node, Collections.emptyList())) {
            if (color[nei] == GRAY) return false;         // back edge => cycle
            if (color[nei] == WHITE && !dfs(nei, adj, color, order)) return false;
        }
        color[node] = BLACK;
        order.add(node);
        return true;
    }
}
```
**Complexity:** O(V+E) time, O(V) space (recursion + color array).

---

## 7. Cycle Detection 🟡

### 7a. Directed Graph — DFS 3-color method
```java
class CycleDirectedDFS {
    private static final int WHITE = 0, GRAY = 1, BLACK = 2;

    public boolean hasCycle(int n, int[][] edges) {
        Map<Integer, List<Integer>> adj = new HashMap<>();
        for (int[] e : edges) adj.computeIfAbsent(e[0], k -> new ArrayList<>()).add(e[1]);
        int[] color = new int[n];
        for (int i = 0; i < n; i++) {
            if (color[i] == WHITE && dfs(i, adj, color)) return true;
        }
        return false;
    }

    private boolean dfs(int node, Map<Integer, List<Integer>> adj, int[] color) {
        color[node] = GRAY;
        for (int nei : adj.getOrDefault(node, Collections.emptyList())) {
            if (color[nei] == GRAY) return true;
            if (color[nei] == WHITE && dfs(nei, adj, color)) return true;
        }
        color[node] = BLACK;
        return false;
    }
}
```
**Complexity:** O(V+E) time, O(V) space.

### Alternative for Directed Graph — Kahn's Algorithm (if fewer nodes processed than V, cycle exists)
```java
class CycleDirectedKahn {
    public boolean hasCycle(int n, int[][] edges) {
        Map<Integer, List<Integer>> adj = new HashMap<>();
        int[] indegree = new int[n];
        for (int[] e : edges) {
            adj.computeIfAbsent(e[0], k -> new ArrayList<>()).add(e[1]);
            indegree[e[1]]++;
        }
        Queue<Integer> q = new LinkedList<>();
        for (int i = 0; i < n; i++) if (indegree[i] == 0) q.offer(i);

        int visitedCount = 0;
        while (!q.isEmpty()) {
            int node = q.poll();
            visitedCount++;
            for (int nei : adj.getOrDefault(node, Collections.emptyList())) {
                if (--indegree[nei] == 0) q.offer(nei);
            }
        }
        return visitedCount != n;
    }
}
```
**Complexity:** O(V+E) time, O(V) space (useful when you already need topo order anyway).

### 7b. Undirected Graph — DFS with Parent Tracking
```java
class CycleUndirectedDFS {
    public boolean hasCycle(int n, int[][] edges) {
        Map<Integer, List<Integer>> adj = new HashMap<>();
        for (int[] e : edges) {
            adj.computeIfAbsent(e[0], k -> new ArrayList<>()).add(e[1]);
            adj.computeIfAbsent(e[1], k -> new ArrayList<>()).add(e[0]);
        }
        boolean[] visited = new boolean[n];
        for (int i = 0; i < n; i++) {
            if (!visited[i] && dfs(i, -1, adj, visited)) return true;
        }
        return false;
    }

    private boolean dfs(int node, int parent, Map<Integer, List<Integer>> adj, boolean[] visited) {
        visited[node] = true;
        for (int nei : adj.getOrDefault(node, Collections.emptyList())) {
            if (!visited[nei]) {
                if (dfs(nei, node, adj, visited)) return true;
            } else if (nei != parent) {
                return true; // visited neighbor that's not parent => cycle
            }
        }
        return false;
    }
}
```
**Complexity:** O(V+E) time, O(V) space.

### Alternative for Undirected Graph — Union-Find (also used for "Redundant Connection")
```java
class CycleUndirectedDSU {
    public boolean hasCycle(int n, int[][] edges) {
        DSU dsu = new DSU(n);
        for (int[] e : edges) {
            if (dsu.find(e[0]) == dsu.find(e[1])) return true; // this edge creates a cycle
            dsu.union(e[0], e[1]);
        }
        return false;
    }

    public int[] findRedundantConnection(int[][] edges) {
        DSU dsu = new DSU(edges.length + 1);
        for (int[] e : edges) {
            if (dsu.find(e[0]) == dsu.find(e[1])) return e;
            dsu.union(e[0], e[1]);
        }
        return new int[0];
    }
}
```
**Complexity:** O(E·α(V)) time, O(V) space — typically preferred over DFS for undirected cycle detection because it naturally identifies the *specific* redundant edge.

---

## 8. Bipartite Check 🟡

### Brute Force — Try all 2^V colorings and validate (exponential, shown only to contrast)
```java
class BipartiteBrute {
    public boolean isBipartiteBrute(int[][] graph) {
        int n = graph.length;
        for (long mask = 0; mask < (1L << n); mask++) {   // 2^n colorings
            boolean valid = true;
            outer:
            for (int u = 0; u < n && valid; u++) {
                int colorU = (int) ((mask >> u) & 1);
                for (int v : graph[u]) {
                    int colorV = (int) ((mask >> v) & 1);
                    if (colorU == colorV) { valid = false; break outer; }
                }
            }
            if (valid) return true;
        }
        return false;
    }
}
```
**Complexity:** O(2ⁿ · V·E) time — intractable for large graphs, included purely for pedagogical contrast.

### Optimized — BFS 2-coloring
```java
class BipartiteBFS {
    public boolean isBipartite(int[][] graph) {
        int n = graph.length;
        int[] color = new int[n];
        Arrays.fill(color, -1);

        for (int start = 0; start < n; start++) {
            if (color[start] != -1) continue;
            color[start] = 0;
            Queue<Integer> q = new LinkedList<>();
            q.offer(start);
            while (!q.isEmpty()) {
                int node = q.poll();
                for (int nei : graph[node]) {
                    if (color[nei] == -1) {
                        color[nei] = 1 - color[node];
                        q.offer(nei);
                    } else if (color[nei] == color[node]) {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}
```
**Complexity:** O(V+E) time, O(V) space.

---

## 9. Union-Find Applications — Number of Provinces 🟡

### Brute Force — DFS/BFS from each unvisited node
```java
class NumProvincesDFS {
    public int findCircleNum(int[][] isConnected) {
        int n = isConnected.length;
        boolean[] visited = new boolean[n];
        int count = 0;
        for (int city = 0; city < n; city++) {
            if (!visited[city]) {
                count++;
                dfs(isConnected, visited, city);
            }
        }
        return count;
    }

    private void dfs(int[][] isConnected, boolean[] visited, int city) {
        visited[city] = true;
        for (int nei = 0; nei < isConnected.length; nei++) {
            if (isConnected[city][nei] == 1 && !visited[nei]) {
                dfs(isConnected, visited, nei);
            }
        }
    }
}
```
**Complexity:** O(V²) time (matrix scan), O(V) space.

### Optimized — Union-Find (better for incremental/dynamic edge addition)
```java
class NumProvincesDSU {
    public int findCircleNum(int[][] isConnected) {
        int n = isConnected.length;
        DSU dsu = new DSU(n);
        dsu.count = n;
        for (int i = 0; i < n; i++) {
            for (int j = i + 1; j < n; j++) {
                if (isConnected[i][j] == 1) dsu.union(i, j);
            }
        }
        return dsu.count;
    }
}
```
**Complexity:** O(V²·α(V)) here (still scanning the matrix), but becomes O(E·α(V)) given an edge list — the real win is when provinces need to be queried/updated incrementally.

---

## 10. Dijkstra's Algorithm — Shortest Path (Non-negative weights) 🟡/🔴

### Brute Force — No priority queue, linear scan for minimum unvisited distance each iteration
```java
class DijkstraBrute {
    public int[] dijkstra(int n, int[][] edges, int src) {
        Map<Integer, List<int[]>> adj = new HashMap<>();
        for (int[] e : edges) adj.computeIfAbsent(e[0], k -> new ArrayList<>()).add(new int[]{e[1], e[2]});

        int[] dist = new int[n];
        Arrays.fill(dist, Integer.MAX_VALUE);
        dist[src] = 0;
        boolean[] visited = new boolean[n];

        for (int iter = 0; iter < n; iter++) {
            int u = -1;
            for (int i = 0; i < n; i++) {                  // O(V) scan for min — the "brute" part
                if (!visited[i] && (u == -1 || dist[i] < dist[u])) u = i;
            }
            if (dist[u] == Integer.MAX_VALUE) break;
            visited[u] = true;
            for (int[] edge : adj.getOrDefault(u, Collections.emptyList())) {
                int v = edge[0], w = edge[1];
                if (dist[u] + w < dist[v]) dist[v] = dist[u] + w;
            }
        }
        return dist;
    }
}
```
**Complexity:** O(V²) time — actually optimal for **dense** graphs (E ≈ V²), O(V) space.

### Optimized — Min-Heap (Priority Queue), best for sparse graphs
```java
class DijkstraHeap {
    public int[] dijkstra(int n, int[][] edges, int src) {
        Map<Integer, List<int[]>> adj = new HashMap<>();
        for (int[] e : edges) adj.computeIfAbsent(e[0], k -> new ArrayList<>()).add(new int[]{e[1], e[2]});

        int[] dist = new int[n];
        Arrays.fill(dist, Integer.MAX_VALUE);
        dist[src] = 0;

        PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[0] - b[0]); // {dist, node}
        pq.offer(new int[]{0, src});

        while (!pq.isEmpty()) {
            int[] curr = pq.poll();
            int d = curr[0], u = curr[1];
            if (d > dist[u]) continue; // stale entry, skip
            for (int[] edge : adj.getOrDefault(u, Collections.emptyList())) {
                int v = edge[0], w = edge[1];
                int nd = d + w;
                if (nd < dist[v]) {
                    dist[v] = nd;
                    pq.offer(new int[]{nd, v});
                }
            }
        }
        return dist;
    }
}
```
**Complexity:** O((V+E) log V) time, O(V+E) space. **Best general-purpose choice** — use this unless the graph is very dense.

### Note on negative weights
Dijkstra is **incorrect** with negative edge weights (a node marked "done" might later find a cheaper path through a negative edge). Use Bellman-Ford instead.

---

## 11. Bellman-Ford Algorithm 🔴

```java
class BellmanFord {
    // returns {dist array, hasNegativeCycle flag}
    public Object[] bellmanFord(int n, int[][] edges, int src) {
        int[] dist = new int[n];
        Arrays.fill(dist, Integer.MAX_VALUE);
        dist[src] = 0;

        for (int i = 0; i < n - 1; i++) {
            boolean updated = false;
            for (int[] e : edges) {
                int u = e[0], v = e[1], w = e[2];
                if (dist[u] != Integer.MAX_VALUE && dist[u] + w < dist[v]) {
                    dist[v] = dist[u] + w;
                    updated = true;
                }
            }
            if (!updated) break; // early termination optimization
        }

        for (int[] e : edges) {
            int u = e[0], v = e[1], w = e[2];
            if (dist[u] != Integer.MAX_VALUE && dist[u] + w < dist[v]) {
                return new Object[]{null, true}; // negative cycle detected
            }
        }
        return new Object[]{dist, false};
    }
}
```
**Complexity:** O(V·E) time, O(V) space. Slower than Dijkstra but handles negative weights and detects negative cycles — Dijkstra cannot do either.

### Optimized variant — SPFA (Shortest Path Faster Algorithm, queue-based Bellman-Ford)
```java
class SPFA {
    public Object[] spfa(int n, Map<Integer, List<int[]>> adj, int src) {
        int[] dist = new int[n];
        Arrays.fill(dist, Integer.MAX_VALUE);
        dist[src] = 0;
        boolean[] inQueue = new boolean[n];
        int[] count = new int[n]; // times a node is enqueued — negative-cycle detection

        Queue<Integer> q = new LinkedList<>();
        q.offer(src);
        inQueue[src] = true;

        while (!q.isEmpty()) {
            int u = q.poll();
            inQueue[u] = false;
            for (int[] edge : adj.getOrDefault(u, Collections.emptyList())) {
                int v = edge[0], w = edge[1];
                if (dist[u] + w < dist[v]) {
                    dist[v] = dist[u] + w;
                    if (!inQueue[v]) {
                        q.offer(v);
                        inQueue[v] = true;
                        if (++count[v] > n) return new Object[]{null, true}; // negative cycle
                    }
                }
            }
        }
        return new Object[]{dist, false};
    }
}
```
**Complexity:** O(V·E) worst case (same as Bellman-Ford) but **average case much faster** in practice.

---

## 12. Floyd-Warshall — All Pairs Shortest Path 🔴

### Brute Force — Run Bellman-Ford from every single source
```java
class AllPairsBrute {
    public int[][] allPairsBrute(int n, int[][] edges) {
        int[][] result = new int[n][];
        BellmanFord bf = new BellmanFord();
        for (int src = 0; src < n; src++) {
            Object[] res = bf.bellmanFord(n, edges, src);
            result[src] = (int[]) res[0];
        }
        return result;
    }
}
```
**Complexity:** O(V² · E) using Bellman-Ford per source, or O(V·(V+E)log V) using Dijkstra per source (only valid without negative weights).

### Optimized — Floyd-Warshall DP (better when graph is dense, or negative edges exist without negative cycles)
```java
class FloydWarshall {
    public int[][] floydWarshall(int n, int[][] edges) {
        int INF = Integer.MAX_VALUE / 2; // avoid overflow on addition
        int[][] dist = new int[n][n];
        for (int[] row : dist) Arrays.fill(row, INF);
        for (int i = 0; i < n; i++) dist[i][i] = 0;
        for (int[] e : edges) dist[e[0]][e[1]] = Math.min(dist[e[0]][e[1]], e[2]);

        for (int k = 0; k < n; k++) {           // intermediate node
            for (int i = 0; i < n; i++) {       // source
                for (int j = 0; j < n; j++) {   // destination
                    if (dist[i][k] + dist[k][j] < dist[i][j]) {
                        dist[i][j] = dist[i][k] + dist[k][j];
                    }
                }
            }
        }

        boolean hasNegativeCycle = false;
        for (int i = 0; i < n; i++) if (dist[i][i] < 0) hasNegativeCycle = true;
        // (hasNegativeCycle can be returned alongside dist as needed)
        return dist;
    }
}
```
**Complexity:** O(V³) time, O(V²) space. **Better than running Bellman-Ford V times** (O(V²E)) when the graph is dense; worse for sparse graphs.

---

## 13. Minimum Spanning Tree — Kruskal's Algorithm 🔴

### Brute Force — Try all subsets of V-1 edges, check validity (combinatorial explosion)
```java
class MSTBrute {
    // Only feasible for tiny graphs — shown for contrast
    public int mstBrute(int n, int[][] edges) {
        int best = Integer.MAX_VALUE;
        int m = edges.length;
        int need = n - 1;
        // iterate all C(m, need) subsets via combinations
        int[] combo = new int[need];
        best = Math.min(best, combine(edges, combo, 0, 0, n));
        return best;
    }

    private int combine(int[][] edges, int[] combo, int start, int depth, int n) {
        if (depth == combo.length) {
            DSU dsu = new DSU(n);
            dsu.count = n;
            int total = 0;
            for (int idx : combo) {
                int[] e = edges[idx];
                if (dsu.find(e[0]) == dsu.find(e[1])) return Integer.MAX_VALUE;
                dsu.union(e[0], e[1]);
                total += e[2];
            }
            return dsu.count == 1 ? total : Integer.MAX_VALUE;
        }
        int best = Integer.MAX_VALUE;
        for (int i = start; i < edges.length; i++) {
            combo[depth] = i;
            best = Math.min(best, combine(edges, combo, i + 1, depth + 1, n));
        }
        return best;
    }
}
```
**Complexity:** O(C(E, V-1) · V) — combinatorial, intractable beyond tiny graphs. Shown purely for contrast.

### Optimized — Kruskal's (Sort edges + Union-Find, greedy)
```java
class Kruskal {
    // edges: {u, v, weight}
    public int kruskalMST(int n, int[][] edges) {
        Arrays.sort(edges, (a, b) -> a[2] - b[2]); // sort by weight ascending
        DSU dsu = new DSU(n);
        int totalWeight = 0, edgesUsed = 0;

        for (int[] e : edges) {
            int u = e[0], v = e[1], w = e[2];
            if (dsu.find(u) != dsu.find(v)) {
                dsu.union(u, v);
                totalWeight += w;
                edgesUsed++;
                if (edgesUsed == n - 1) break;
            }
        }
        return totalWeight;
    }
}
```
**Complexity:** O(E log E) time (dominated by sort), O(V) space. Best when the edge list is given directly and graph is sparse.

---

## 14. Minimum Spanning Tree — Prim's Algorithm 🔴

### Brute Force — Linear scan for minimum edge crossing the cut (no heap)
```java
class PrimBrute {
    public int primBrute(int n, int[][] adjMatrix) {
        int INF = Integer.MAX_VALUE;
        boolean[] inMST = new boolean[n];
        int[] key = new int[n];
        Arrays.fill(key, INF);
        key[0] = 0;
        int total = 0;

        for (int iter = 0; iter < n; iter++) {
            int u = -1;
            for (int i = 0; i < n; i++) {
                if (!inMST[i] && (u == -1 || key[i] < key[u])) u = i;
            }
            inMST[u] = true;
            total += key[u];
            for (int v = 0; v < n; v++) {
                if (adjMatrix[u][v] != 0 && !inMST[v] && adjMatrix[u][v] < key[v]) {
                    key[v] = adjMatrix[u][v];
                }
            }
        }
        return total;
    }
}
```
**Complexity:** O(V²) time — actually **optimal for dense graphs**, O(V) space.

### Optimized — Min-Heap version, best for sparse graphs
```java
class PrimHeap {
    public int primHeap(int n, Map<Integer, List<int[]>> adj) {
        boolean[] visited = new boolean[n];
        PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[0] - b[0]); // {weight, node}
        pq.offer(new int[]{0, 0}); // start from node 0
        int total = 0, edgesUsed = 0;

        while (!pq.isEmpty() && edgesUsed < n) {
            int[] curr = pq.poll();
            int w = curr[0], u = curr[1];
            if (visited[u]) continue;
            visited[u] = true;
            total += w;
            edgesUsed++;
            for (int[] edge : adj.getOrDefault(u, Collections.emptyList())) {
                int v = edge[0], weight = edge[1];
                if (!visited[v]) pq.offer(new int[]{weight, v});
            }
        }
        return total;
    }
}
```
**Complexity:** O(E log V) time, O(V+E) space. **Kruskal vs Prim:** Kruskal is simpler and great when edges are already listed; Prim (heap version) is often faster on very dense graphs represented as adjacency lists.

---

## 15. Bridges in a Graph — Tarjan's Algorithm 🔴

### Brute Force — Remove each edge one at a time, check connectivity via DFS
```java
class BridgesBrute {
    public List<List<Integer>> findBridgesBrute(int n, int[][] edges) {
        List<List<Integer>> bridges = new ArrayList<>();
        for (int[] edge : edges) {
            if (!isConnectedWithoutEdge(n, edges, edge)) {
                bridges.add(Arrays.asList(edge[0], edge[1]));
            }
        }
        return bridges;
    }

    private boolean isConnectedWithoutEdge(int n, int[][] edges, int[] skip) {
        Map<Integer, List<Integer>> adj = new HashMap<>();
        for (int[] e : edges) {
            if (Arrays.equals(e, skip)) continue;
            adj.computeIfAbsent(e[0], k -> new ArrayList<>()).add(e[1]);
            adj.computeIfAbsent(e[1], k -> new ArrayList<>()).add(e[0]);
        }
        Set<Integer> visited = new HashSet<>();
        Deque<Integer> stack = new ArrayDeque<>();
        stack.push(0);
        visited.add(0);
        while (!stack.isEmpty()) {
            int node = stack.pop();
            for (int nei : adj.getOrDefault(node, Collections.emptyList())) {
                if (!visited.contains(nei)) { visited.add(nei); stack.push(nei); }
            }
        }
        return visited.size() == n;
    }
}
```
**Complexity:** O(E · (V+E)) time — recomputing full connectivity for every edge removal.

### Optimized — Tarjan's Bridge-Finding (discovery time + low-link values)
```java
class BridgesTarjan {
    private int timer = 0;

    public List<List<Integer>> findBridges(int n, int[][] edges) {
        Map<Integer, List<Integer>> adj = new HashMap<>();
        for (int[] e : edges) {
            adj.computeIfAbsent(e[0], k -> new ArrayList<>()).add(e[1]);
            adj.computeIfAbsent(e[1], k -> new ArrayList<>()).add(e[0]);
        }
        int[] disc = new int[n], low = new int[n];
        Arrays.fill(disc, -1);
        List<List<Integer>> bridges = new ArrayList<>();

        for (int i = 0; i < n; i++) {
            if (disc[i] == -1) dfs(i, -1, adj, disc, low, bridges);
        }
        return bridges;
    }

    private void dfs(int u, int parent, Map<Integer, List<Integer>> adj, int[] disc, int[] low,
                      List<List<Integer>> bridges) {
        disc[u] = low[u] = timer++;
        for (int v : adj.getOrDefault(u, Collections.emptyList())) {
            if (v == parent) continue;
            if (disc[v] == -1) {
                dfs(v, u, adj, disc, low, bridges);
                low[u] = Math.min(low[u], low[v]);
                if (low[v] > disc[u]) {          // no back edge reaches u or above => bridge
                    bridges.add(Arrays.asList(u, v));
                }
            } else {
                low[u] = Math.min(low[u], disc[v]); // back edge
            }
        }
    }
}
```
**Complexity:** O(V+E) time, O(V) space — a single DFS pass instead of E full traversals.

---

## 16. Articulation Points — Tarjan's Algorithm 🔴

```java
class ArticulationPoints {
    private int timer = 0;

    public List<Integer> findArticulationPoints(int n, int[][] edges) {
        Map<Integer, List<Integer>> adj = new HashMap<>();
        for (int[] e : edges) {
            adj.computeIfAbsent(e[0], k -> new ArrayList<>()).add(e[1]);
            adj.computeIfAbsent(e[1], k -> new ArrayList<>()).add(e[0]);
        }
        int[] disc = new int[n], low = new int[n];
        Arrays.fill(disc, -1);
        boolean[] isAP = new boolean[n];

        for (int i = 0; i < n; i++) {
            if (disc[i] == -1) dfs(i, -1, adj, disc, low, isAP);
        }

        List<Integer> result = new ArrayList<>();
        for (int i = 0; i < n; i++) if (isAP[i]) result.add(i);
        return result;
    }

    private void dfs(int u, int parent, Map<Integer, List<Integer>> adj, int[] disc, int[] low, boolean[] isAP) {
        int children = 0;
        disc[u] = low[u] = timer++;
        for (int v : adj.getOrDefault(u, Collections.emptyList())) {
            if (v == parent) continue;
            if (disc[v] == -1) {
                children++;
                dfs(v, u, adj, disc, low, isAP);
                low[u] = Math.min(low[u], low[v]);
                if (parent != -1 && low[v] >= disc[u]) isAP[u] = true; // non-root case
            } else {
                low[u] = Math.min(low[u], disc[v]);
            }
        }
        if (parent == -1 && children > 1) isAP[u] = true; // root special case
    }
}
```
**Complexity:** O(V+E) time, O(V) space. Same DFS-with-low-link technique as bridges, different comparison condition (`>=` instead of `>`, plus root special-casing).

---

## 17. Strongly Connected Components 🔴/⚫

### Optimized Solution 1 — Kosaraju's Algorithm (two-pass DFS + transpose)
```java
class KosarajuSCC {
    public List<List<Integer>> kosaraju(int n, int[][] edges) {
        Map<Integer, List<Integer>> adj = new HashMap<>();
        Map<Integer, List<Integer>> radj = new HashMap<>(); // reversed/transposed graph
        for (int[] e : edges) {
            adj.computeIfAbsent(e[0], k -> new ArrayList<>()).add(e[1]);
            radj.computeIfAbsent(e[1], k -> new ArrayList<>()).add(e[0]);
        }

        boolean[] visited = new boolean[n];
        Deque<Integer> order = new ArrayDeque<>();
        for (int i = 0; i < n; i++) {
            if (!visited[i]) dfs1(i, adj, visited, order);
        }

        visited = new boolean[n];
        List<List<Integer>> sccs = new ArrayList<>();
        for (int node : order) {
            if (!visited[node]) {
                List<Integer> component = new ArrayList<>();
                dfs2(node, radj, visited, component);
                sccs.add(component);
            }
        }
        return sccs;
    }

    private void dfs1(int u, Map<Integer, List<Integer>> adj, boolean[] visited, Deque<Integer> order) {
        visited[u] = true;
        for (int v : adj.getOrDefault(u, Collections.emptyList())) {
            if (!visited[v]) dfs1(v, adj, visited, order);
        }
        order.push(u); // post-order
    }

    private void dfs2(int u, Map<Integer, List<Integer>> radj, boolean[] visited, List<Integer> component) {
        visited[u] = true;
        component.add(u);
        for (int v : radj.getOrDefault(u, Collections.emptyList())) {
            if (!visited[v]) dfs2(v, radj, visited, component);
        }
    }
}
```
**Complexity:** O(V+E) time, O(V+E) space. Two full DFS passes — simple to reason about.

### Optimized Solution 2 — Tarjan's SCC (single-pass, low-link based, no explicit transpose needed)
```java
class TarjanSCC {
    private int timer = 0;
    private int[] disc, low;
    private boolean[] onStack;
    private Deque<Integer> stack = new ArrayDeque<>();
    private List<List<Integer>> sccs = new ArrayList<>();

    public List<List<Integer>> tarjanSCC(int n, int[][] edges) {
        Map<Integer, List<Integer>> adj = new HashMap<>();
        for (int[] e : edges) adj.computeIfAbsent(e[0], k -> new ArrayList<>()).add(e[1]);

        disc = new int[n];
        low = new int[n];
        onStack = new boolean[n];
        Arrays.fill(disc, -1);

        for (int i = 0; i < n; i++) {
            if (disc[i] == -1) dfs(i, adj);
        }
        return sccs;
    }

    private void dfs(int u, Map<Integer, List<Integer>> adj) {
        disc[u] = low[u] = timer++;
        stack.push(u);
        onStack[u] = true;

        for (int v : adj.getOrDefault(u, Collections.emptyList())) {
            if (disc[v] == -1) {
                dfs(v, adj);
                low[u] = Math.min(low[u], low[v]);
            } else if (onStack[v]) {
                low[u] = Math.min(low[u], disc[v]);
            }
        }

        if (low[u] == disc[u]) {           // u is a root of an SCC
            List<Integer> component = new ArrayList<>();
            while (true) {
                int w = stack.pop();
                onStack[w] = false;
                component.add(w);
                if (w == u) break;
            }
            sccs.add(component);
        }
    }
}
```
**Complexity:** O(V+E) time, O(V) space. **Tarjan's vs Kosaraju's:** Tarjan's needs only one DFS pass (no graph transpose) and is generally preferred in competitive settings; Kosaraju's is often considered easier to understand/prove correct.

---

## 18. Maximum Flow — Ford-Fulkerson / Edmonds-Karp / Dinic's ⚫

### Base Method — Ford-Fulkerson with DFS to find augmenting paths
```java
class FordFulkerson {
    public int maxFlow(int n, int[][] capacity, int source, int sink) {
        int maxFlow = 0;
        while (true) {
            boolean[] visited = new boolean[n];
            int pushed = dfs(capacity, source, sink, visited, Integer.MAX_VALUE);
            if (pushed == 0) break;
            maxFlow += pushed;
        }
        return maxFlow;
    }

    private int dfs(int[][] capacity, int u, int sink, boolean[] visited, int minCap) {
        if (u == sink) return minCap;
        visited[u] = true;
        int n = capacity.length;
        for (int v = 0; v < n; v++) {
            if (!visited[v] && capacity[u][v] > 0) {
                int bottleneck = dfs(capacity, v, sink, visited, Math.min(minCap, capacity[u][v]));
                if (bottleneck > 0) {
                    capacity[u][v] -= bottleneck;
                    capacity[v][u] += bottleneck; // residual back edge
                    return bottleneck;
                }
            }
        }
        return 0;
    }
}
```
**Complexity:** O(E · max_flow) — can be slow if augmenting paths found via DFS are narrow.

### Optimized — Edmonds-Karp (Ford-Fulkerson using BFS = always finds shortest augmenting path)
```java
class EdmondsKarp {
    public int maxFlow(int n, int[][] capacity, int source, int sink) {
        int maxFlow = 0;
        while (true) {
            int[] parent = new int[n];
            Arrays.fill(parent, -1);
            parent[source] = source;
            Queue<Integer> q = new LinkedList<>();
            q.offer(source);

            while (!q.isEmpty() && parent[sink] == -1) {
                int u = q.poll();
                for (int v = 0; v < n; v++) {
                    if (parent[v] == -1 && capacity[u][v] > 0) {
                        parent[v] = u;
                        q.offer(v);
                    }
                }
            }
            if (parent[sink] == -1) break; // no augmenting path left

            int pathFlow = Integer.MAX_VALUE;
            for (int v = sink; v != source; v = parent[v]) {
                pathFlow = Math.min(pathFlow, capacity[parent[v]][v]);
            }
            for (int v = sink; v != source; v = parent[v]) {
                int u = parent[v];
                capacity[u][v] -= pathFlow;
                capacity[v][u] += pathFlow;
            }
            maxFlow += pathFlow;
        }
        return maxFlow;
    }
}
```
**Complexity:** O(V·E²) time — polynomial and predictable (unlike plain Ford-Fulkerson), O(V²) space for the capacity matrix.

### Further Optimized — Dinic's Algorithm (for large graphs)
```java
class Dinic {
    private List<int[]> edges = new ArrayList<>();     // each edge: {to, cap}
    private Map<Integer, List<Integer>> graph = new HashMap<>(); // node -> edge indices

    private void addEdge(int u, int v, int cap) {
        graph.computeIfAbsent(u, k -> new ArrayList<>()).add(edges.size());
        edges.add(new int[]{v, cap});
        graph.computeIfAbsent(v, k -> new ArrayList<>()).add(edges.size());
        edges.add(new int[]{u, 0}); // reverse edge, 0 initial capacity
    }

    public int maxFlow(int n, int[][] edgeList, int source, int sink) {
        for (int[] e : edgeList) addEdge(e[0], e[1], e[2]);

        int maxFlow = 0;
        while (true) {
            int[] level = bfsLevel(n, source);
            if (level[sink] == -1) break;
            int[] it = new int[n]; // iterator per node for DFS
            int pushed;
            while ((pushed = dfsBlockingFlow(source, sink, Integer.MAX_VALUE, level, it)) > 0) {
                maxFlow += pushed;
            }
        }
        return maxFlow;
    }

    private int[] bfsLevel(int n, int source) {
        int[] level = new int[n];
        Arrays.fill(level, -1);
        level[source] = 0;
        Queue<Integer> q = new LinkedList<>();
        q.offer(source);
        while (!q.isEmpty()) {
            int u = q.poll();
            for (int eid : graph.getOrDefault(u, Collections.emptyList())) {
                int v = edges.get(eid)[0], cap = edges.get(eid)[1];
                if (cap > 0 && level[v] == -1) {
                    level[v] = level[u] + 1;
                    q.offer(v);
                }
            }
        }
        return level;
    }

    private int dfsBlockingFlow(int u, int sink, int pushed, int[] level, int[] it) {
        if (u == sink || pushed == 0) return pushed;
        List<Integer> adjList = graph.getOrDefault(u, Collections.emptyList());
        while (it[u] < adjList.size()) {
            int eid = adjList.get(it[u]);
            int v = edges.get(eid)[0], cap = edges.get(eid)[1];
            if (cap > 0 && level[v] == level[u] + 1) {
                int d = dfsBlockingFlow(v, sink, Math.min(pushed, cap), level, it);
                if (d > 0) {
                    edges.get(eid)[1] -= d;
                    edges.get(eid ^ 1)[1] += d;
                    return d;
                }
            }
            it[u]++;
        }
        return 0;
    }
}
```
**Complexity:** O(V²·E) general graphs, O(E√V) for unit-capacity graphs (e.g., bipartite matching) — the go-to choice for large flow networks.

---

## 19. Bidirectional BFS — Word Ladder ⚫

### Brute Force — Standard single-direction BFS
```java
class WordLadderBFS {
    public int ladderLength(String beginWord, String endWord, List<String> wordList) {
        Set<String> wordSet = new HashSet<>(wordList);
        if (!wordSet.contains(endWord)) return 0;

        Queue<String> q = new LinkedList<>();
        q.offer(beginWord);
        Set<String> visited = new HashSet<>();
        visited.add(beginWord);
        int steps = 1;

        while (!q.isEmpty()) {
            int size = q.size();
            for (int i = 0; i < size; i++) {
                String word = q.poll();
                if (word.equals(endWord)) return steps;
                char[] chars = word.toCharArray();
                for (int j = 0; j < chars.length; j++) {
                    char original = chars[j];
                    for (char c = 'a'; c <= 'z'; c++) {
                        chars[j] = c;
                        String newWord = new String(chars);
                        if (wordSet.contains(newWord) && !visited.contains(newWord)) {
                            visited.add(newWord);
                            q.offer(newWord);
                        }
                    }
                    chars[j] = original;
                }
            }
            steps++;
        }
        return 0;
    }
}
```
**Complexity:** O(N · L · 26) time (N = word list size, L = word length), O(N) space. Explores an exponentially growing frontier from one side only.

### Optimized — Bidirectional BFS (search from both ends, meet in the middle)
```java
class WordLadderBidirectional {
    public int ladderLength(String beginWord, String endWord, List<String> wordList) {
        Set<String> wordSet = new HashSet<>(wordList);
        if (!wordSet.contains(endWord)) return 0;

        Set<String> front = new HashSet<>();
        Set<String> back = new HashSet<>();
        front.add(beginWord);
        back.add(endWord);
        int steps = 1;

        while (!front.isEmpty() && !back.isEmpty()) {
            if (front.size() > back.size()) { // always expand the smaller frontier
                Set<String> tmp = front; front = back; back = tmp;
            }

            Set<String> nextFront = new HashSet<>();
            for (String word : front) {
                char[] chars = word.toCharArray();
                for (int i = 0; i < chars.length; i++) {
                    char original = chars[i];
                    for (char c = 'a'; c <= 'z'; c++) {
                        chars[i] = c;
                        String newWord = new String(chars);
                        if (back.contains(newWord)) return steps + 1;
                        if (wordSet.contains(newWord)) {
                            nextFront.add(newWord);
                            wordSet.remove(newWord); // mark visited
                        }
                    }
                    chars[i] = original;
                }
            }
            front = nextFront;
            steps++;
        }
        return 0;
    }
}
```
**Complexity:** Same theoretical worst case O(N · L · 26), but **practically much faster** — two frontiers growing to meet in the middle cover roughly `2·√(branching^depth)` nodes instead of `branching^depth`, a large constant-factor (often orders-of-magnitude) speedup on wide search spaces.

---

## 20. Traveling Salesman Problem — Bitmask DP ⚫

### Brute Force — Try all permutations of nodes
```java
class TSPBrute {
    private int best = Integer.MAX_VALUE;

    public int tspBrute(int n, int[][] dist) {
        int[] nodes = new int[n - 1];
        for (int i = 1; i < n; i++) nodes[i - 1] = i;
        permute(nodes, 0, dist, n);
        return best;
    }

    private void permute(int[] nodes, int k, int[][] dist, int n) {
        if (k == nodes.length) {
            int cost = dist[0][nodes[0]];
            for (int i = 0; i < nodes.length - 1; i++) cost += dist[nodes[i]][nodes[i + 1]];
            cost += dist[nodes[nodes.length - 1]][0];
            best = Math.min(best, cost);
            return;
        }
        for (int i = k; i < nodes.length; i++) {
            swap(nodes, k, i);
            permute(nodes, k + 1, dist, n);
            swap(nodes, k, i);
        }
    }

    private void swap(int[] arr, int i, int j) { int t = arr[i]; arr[i] = arr[j]; arr[j] = t; }
}
```
**Complexity:** O(n!) time — intractable beyond ~10-11 nodes.

### Optimized — Bitmask DP (Held-Karp Algorithm)
```java
class TSPBitmaskDP {
    public int tsp(int n, int[][] dist) {
        int INF = Integer.MAX_VALUE / 2;
        int[][] dp = new int[1 << n][n]; // dp[mask][i] = min cost visiting `mask`, ending at i
        for (int[] row : dp) Arrays.fill(row, INF);
        dp[1][0] = 0; // start at node 0, only node 0 visited

        for (int mask = 0; mask < (1 << n); mask++) {
            for (int u = 0; u < n; u++) {
                if (dp[mask][u] == INF || (mask & (1 << u)) == 0) continue;
                for (int v = 0; v < n; v++) {
                    if ((mask & (1 << v)) != 0) continue; // already visited
                    int newMask = mask | (1 << v);
                    int newCost = dp[mask][u] + dist[u][v];
                    if (newCost < dp[newMask][v]) dp[newMask][v] = newCost;
                }
            }
        }

        int fullMask = (1 << n) - 1;
        int result = INF;
        for (int i = 1; i < n; i++) {
            result = Math.min(result, dp[fullMask][i] + dist[i][0]);
        }
        return result;
    }
}
```
**Complexity:** O(n² · 2ⁿ) time, O(n · 2ⁿ) space. Exponential, but a **massive** improvement over O(n!) — e.g., n=20: n! ≈ 2.4×10¹⁸ vs n²·2ⁿ ≈ 4×10⁸.

---

## 21. Alien Dictionary — Topological Sort on Implicit Graph 🔴

```java
class AlienDictionary {
    public String alienOrder(String[] words) {
        Map<Character, Set<Character>> adj = new HashMap<>();
        Map<Character, Integer> indegree = new HashMap<>();
        for (String word : words) {
            for (char c : word.toCharArray()) indegree.putIfAbsent(c, 0);
        }

        for (int i = 0; i < words.length - 1; i++) {
            String w1 = words[i], w2 = words[i + 1];
            int minLen = Math.min(w1.length(), w2.length());
            if (w1.length() > w2.length() && w1.substring(0, minLen).equals(w2.substring(0, minLen))) {
                return ""; // invalid order (prefix must come first)
            }
            for (int j = 0; j < minLen; j++) {
                char c1 = w1.charAt(j), c2 = w2.charAt(j);
                if (c1 != c2) {
                    adj.computeIfAbsent(c1, k -> new HashSet<>());
                    if (!adj.get(c1).contains(c2)) {
                        adj.get(c1).add(c2);
                        indegree.merge(c2, 1, Integer::sum);
                    }
                    break;
                }
            }
        }

        Queue<Character> q = new LinkedList<>();
        for (char c : indegree.keySet()) if (indegree.get(c) == 0) q.offer(c);

        StringBuilder order = new StringBuilder();
        while (!q.isEmpty()) {
            char c = q.poll();
            order.append(c);
            for (char nei : adj.getOrDefault(c, Collections.emptySet())) {
                indegree.merge(nei, -1, Integer::sum);
                if (indegree.get(nei) == 0) q.offer(nei);
            }
        }

        return order.length() == indegree.size() ? order.toString() : "";
    }
}
```
**Complexity:** O(C) time where C = total characters across all words (building the graph), plus O(V+E) for Kahn's topological sort where V ≤ 26 letters — effectively O(C). Space O(1) (bounded alphabet) + O(C) for input scanning.
**Key insight:** This is topological sort applied to a graph you must first *construct* by inferring edges from pairwise word comparisons — a very common "hard" interview pattern (implicit graph construction + known algorithm).

---

## 🎯 Complexity Summary Table

| Algorithm | Time | Space | Use Case |
|---|---|---|---|
| BFS/DFS Traversal | O(V+E) | O(V) | Connectivity, shortest path (unweighted) |
| Topological Sort (Kahn's/DFS) | O(V+E) | O(V) | Dependency ordering, DAG problems |
| Cycle Detection | O(V+E) | O(V) | Directed: 3-color DFS; Undirected: Union-Find |
| Dijkstra (heap) | O((V+E) log V) | O(V+E) | Shortest path, non-negative weights, sparse graph |
| Dijkstra (array scan) | O(V²) | O(V) | Shortest path, dense graph |
| Bellman-Ford | O(V·E) | O(V) | Negative weights, negative cycle detection |
| SPFA | O(V·E) worst, faster avg | O(V) | Bellman-Ford optimization |
| Floyd-Warshall | O(V³) | O(V²) | All-pairs shortest path, dense graphs |
| Kruskal's MST | O(E log E) | O(V) | MST, sparse graphs, edge list given |
| Prim's MST (heap) | O(E log V) | O(V+E) | MST, adjacency list |
| Prim's MST (array) | O(V²) | O(V) | MST, dense graphs |
| Tarjan's Bridges/APs | O(V+E) | O(V) | Critical connections |
| Kosaraju's / Tarjan's SCC | O(V+E) | O(V+E) | Strongly connected components |
| Edmonds-Karp Max Flow | O(V·E²) | O(V²) | Network flow, polynomial guarantee |
| Dinic's Max Flow | O(V²·E) / O(E√V) | O(V+E) | Large flow networks, bipartite matching |
| Bidirectional BFS | O(b^(d/2)) | O(b^(d/2)) | Shortest path with huge branching factor |
| TSP Bitmask DP | O(n²·2ⁿ) | O(n·2ⁿ) | Exact TSP for n ≤ ~20 |

---

## 🧭 Decision Framework — Which Algorithm to Pick

1. **Unweighted shortest path / connectivity** → BFS/DFS, O(V+E)
2. **Weighted shortest path, no negative edges** → Dijkstra (heap for sparse, array for dense)
3. **Weighted shortest path, negative edges possible** → Bellman-Ford (or SPFA for speed)
4. **All-pairs shortest path** → Floyd-Warshall (dense) or Dijkstra/Bellman-Ford from every node (sparse)
5. **Minimum spanning tree** → Kruskal's (edge list given) or Prim's (adjacency list, dense graph)
6. **Ordering with dependencies** → Topological sort (Kahn's for iterative, DFS for recursive style)
7. **Detecting cut edges/vertices** → Tarjan's bridges/articulation points
8. **Grouping mutually reachable nodes (directed)** → Kosaraju's or Tarjan's SCC
9. **Maximum flow / bipartite matching** → Edmonds-Karp (simple) or Dinic's (large scale)
10. **Huge search space, need shortest transformation** → Bidirectional BFS
11. **Small n, visit-all-nodes optimization** → Bitmask DP (Held-Karp for TSP-style problems)
12. **Dynamic connectivity queries** → Union-Find with path compression + union by rank
