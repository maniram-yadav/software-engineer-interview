# Graph DSA Problem Guide (Java)

A comprehensive reference covering graph problems from simple traversal to advanced algorithms, with problem descriptions, examples, brute-force and optimal Java solutions, approach explanations, and time/space complexity.

---

# Part I — Easy Level Topics

## Section 1-1: Simple DFS/BFS

### 1. Evaluate Division

**LeetCode:** https://leetcode.com/problems/evaluate-division

**Problem:** You're given equations like `a/b = 2.0`, `b/c = 3.0` as pairs `equations = [[a,b],[b,c]]` with `values = [2.0, 3.0]`. Answer queries like `a/c = ?` by treating each equation as a weighted, directed edge in a graph. If a query variable doesn't exist in the graph, or no path connects the two, return `-1.0`.

**Example:**
```
equations = [["a","b"],["b","c"]], values = [2.0,3.0]
queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
Output: [6.0, 0.5, -1.0, 1.0, -1.0]
```
`a/c = (a/b)*(b/c) = 2.0*3.0 = 6.0`

**Approach:** Build a weighted graph: `a -> b` with weight `2.0`, and `b -> a` with weight `1/2.0`. For each query, DFS/BFS from source to destination, multiplying edge weights along the way. There's no meaningfully "worse" brute force beyond DFS/BFS over the graph, so we present the standard DFS solution and the Weighted Union-Find alternative.

#### Solution 1: DFS (Graph traversal)

```java
import java.util.*;

class Solution {
    public double[] calcEquation(List<List<String>> equations, double[] values, List<List<String>> queries) {
        Map<String, Map<String, Double>> graph = new HashMap<>();
        for (int i = 0; i < equations.size(); i++) {
            String a = equations.get(i).get(0);
            String b = equations.get(i).get(1);
            double val = values[i];
            graph.computeIfAbsent(a, k -> new HashMap<>()).put(b, val);
            graph.computeIfAbsent(b, k -> new HashMap<>()).put(a, 1.0 / val);
        }

        double[] results = new double[queries.size()];
        for (int i = 0; i < queries.size(); i++) {
            String src = queries.get(i).get(0);
            String dst = queries.get(i).get(1);
            if (!graph.containsKey(src) || !graph.containsKey(dst)) {
                results[i] = -1.0;
            } else {
                Set<String> visited = new HashSet<>();
                results[i] = dfs(graph, src, dst, 1.0, visited);
            }
        }
        return results;
    }

    private double dfs(Map<String, Map<String, Double>> graph, String cur, String target,
                        double acc, Set<String> visited) {
        if (cur.equals(target)) return acc;
        visited.add(cur);
        for (Map.Entry<String, Double> neighbor : graph.get(cur).entrySet()) {
            if (!visited.contains(neighbor.getKey())) {
                double result = dfs(graph, neighbor.getKey(), target, acc * neighbor.getValue(), visited);
                if (result != -1.0) return result;
            }
        }
        return -1.0;
    }
}
```
**Complexity:** Building graph `O(E)`. Each query DFS is `O(V + E)` worst case. Total: `O(E + Q*(V+E))`. Space: `O(V + E)`.

#### Solution 2: Weighted Union-Find (alternative optimal approach)

```java
import java.util.*;

class Solution {
    private Map<String, String> parent = new HashMap<>();
    private Map<String, Double> weight = new HashMap<>();

    private String find(String x) {
        if (!parent.containsKey(x)) {
            parent.put(x, x);
            weight.put(x, 1.0);
        }
        if (!parent.get(x).equals(x)) {
            String root = find(parent.get(x));
            weight.put(x, weight.get(x) * weight.get(parent.get(x)));
            parent.put(x, root);
        }
        return parent.get(x);
    }

    private void union(String a, String b, double val) {
        String rootA = find(a), rootB = find(b);
        if (rootA.equals(rootB)) return;
        parent.put(rootA, rootB);
        weight.put(rootA, val * weight.get(b) / weight.get(a));
    }

    public double[] calcEquation(List<List<String>> equations, double[] values, List<List<String>> queries) {
        for (int i = 0; i < equations.size(); i++) {
            union(equations.get(i).get(0), equations.get(i).get(1), values[i]);
        }
        double[] results = new double[queries.size()];
        for (int i = 0; i < queries.size(); i++) {
            String a = queries.get(i).get(0), b = queries.get(i).get(1);
            if (!parent.containsKey(a) || !parent.containsKey(b) || !find(a).equals(find(b))) {
                results[i] = -1.0;
            } else {
                results[i] = weight.get(a) / weight.get(b);
            }
        }
        return results;
    }
}
```
**Complexity:** Nearly `O(1)` per operation with path compression. Total `O((V+Q) α(V))`. Better when there are many queries.

---

### 2. Keys and Rooms

**LeetCode:** https://leetcode.com/problems/keys-and-rooms

**Problem:** `n` rooms numbered `0` to `n-1`. Room `0` is unlocked initially. `rooms[i]` is a list of keys found in room `i`, each key `rooms[i][j]` opens room `rooms[i][j]`. Return `true` if you can visit every room.

**Example:**
```
rooms = [[1],[2],[3],[]]
Output: true
Explanation: Start in room 0, get key 1 -> go to room 1, get key 2 -> room 2, get key 3 -> room 3.
```

**Approach:** This is reachability from node `0` in a directed graph. The "brute force" alternative to DFS/BFS is repeatedly re-scanning all rooms until no new room is unlockable (a fixed-point iteration), which is worse.

#### Brute Force: Repeated scanning (fixed-point iteration)

```java
import java.util.*;

class Solution {
    public boolean canVisitAllRooms(List<List<Integer>> rooms) {
        int n = rooms.size();
        boolean[] visited = new boolean[n];
        visited[0] = true;
        Set<Integer> keys = new HashSet<>(rooms.get(0));
        keys.add(0);

        boolean changed = true;
        while (changed) {
            changed = false;
            for (int room : keys) {
                if (!visited[room]) {
                    visited[room] = true;
                    changed = true;
                    keys.addAll(rooms.get(room));
                }
            }
        }
        for (boolean v : visited) if (!v) return false;
        return true;
    }
}
```
**Complexity:** Each full pass is `O(n)`, could take up to `O(n)` passes to converge → `O(n^2)` worst case. Space `O(n)`.

#### Optimal: DFS

```java
import java.util.*;

class Solution {
    public boolean canVisitAllRooms(List<List<Integer>> rooms) {
        int n = rooms.size();
        boolean[] visited = new boolean[n];
        dfs(0, rooms, visited);
        for (boolean v : visited) if (!v) return false;
        return true;
    }

    private void dfs(int room, List<List<Integer>> rooms, boolean[] visited) {
        visited[room] = true;
        for (int key : rooms.get(room)) {
            if (!visited[key]) {
                dfs(key, rooms, visited);
            }
        }
    }
}
```
**Complexity:** `O(V + E)`. Space: `O(V)` recursion stack + visited array.

#### Optimal: BFS (iterative, avoids recursion stack overflow)

```java
import java.util.*;

class Solution {
    public boolean canVisitAllRooms(List<List<Integer>> rooms) {
        int n = rooms.size();
        boolean[] visited = new boolean[n];
        visited[0] = true;
        Deque<Integer> queue = new ArrayDeque<>();
        queue.add(0);
        int count = 1;

        while (!queue.isEmpty()) {
            int room = queue.poll();
            for (int key : rooms.get(room)) {
                if (!visited[key]) {
                    visited[key] = true;
                    count++;
                    queue.add(key);
                }
            }
        }
        return count == n;
    }
}
```
**Complexity:** `O(V + E)` time, `O(V)` space.

---

### 3. Get Watched Videos by Your Friends

**LeetCode:** https://leetcode.com/problems/get-watched-videos-by-your-friends

**Problem:** `n` people, `watchedVideos[i]` = list of videos person `i` watched, `friends[i]` = list of friend indices. Given `id` and `level`, find all videos watched by friends exactly `level` steps away (via BFS layers), sorted by frequency (ascending), then alphabetically for ties.

**Example:**
```
watchedVideos = [["A","B"],["C"],["B","C"],["D"]]
friends = [[1,2],[0,3],[0,3],[1,2]]
id = 0, level = 1
Output: ["B","C"]
Explanation: Person 0's level-1 friends are persons 1 and 2.
Person 1 watched ["C"], person 2 watched ["B","C"]. Combined: B(1), C(2). Sorted by freq then alpha: ["B","C"]
```

**Approach:** BFS from `id` to find the exact set of people at distance `level`. Then collect all their watched videos, count frequencies, sort.

#### Brute Force: DFS with distance relaxation, then filter

```java
import java.util.*;

class Solution {
    public List<String> watchedVideosByFriends(List<List<String>> watchedVideos, int[][] friends,
                                                 int id, int level) {
        int n = friends.length;
        int[] dist = new int[n];
        Arrays.fill(dist, -1);
        dist[id] = 0;
        dfsDistances(id, friends, dist);

        Map<String, Integer> freq = new HashMap<>();
        for (int i = 0; i < n; i++) {
            if (dist[i] == level) {
                for (String video : watchedVideos.get(i)) {
                    freq.merge(video, 1, Integer::sum);
                }
            }
        }
        List<String> result = new ArrayList<>(freq.keySet());
        result.sort((a, b) -> freq.get(a) != freq.get(b) ? freq.get(a) - freq.get(b) : a.compareTo(b));
        return result;
    }

    private void dfsDistances(int start, int[][] friends, int[] dist) {
        Deque<Integer> stack = new ArrayDeque<>();
        stack.push(start);
        while (!stack.isEmpty()) {
            int cur = stack.pop();
            for (int nxt : friends[cur]) {
                if (dist[nxt] == -1 || dist[nxt] > dist[cur] + 1) {
                    dist[nxt] = dist[cur] + 1;
                    stack.push(nxt);
                }
            }
        }
    }
}
```
**Complexity:** Plain DFS-relaxation can revisit nodes multiple times — worst case `O(V*E)`. Space `O(V)`.

#### Optimal: BFS (guarantees shortest distance in one pass)

```java
import java.util.*;

class Solution {
    public List<String> watchedVideosByFriends(List<List<String>> watchedVideos, int[][] friends,
                                                 int id, int level) {
        int n = friends.length;
        boolean[] visited = new boolean[n];
        visited[id] = true;
        Deque<Integer> queue = new ArrayDeque<>();
        queue.add(id);
        int curLevel = 0;

        while (!queue.isEmpty() && curLevel < level) {
            int size = queue.size();
            for (int i = 0; i < size; i++) {
                int cur = queue.poll();
                for (int nxt : friends[cur]) {
                    if (!visited[nxt]) {
                        visited[nxt] = true;
                        queue.add(nxt);
                    }
                }
            }
            curLevel++;
        }

        Map<String, Integer> freq = new TreeMap<>();
        for (int person : queue) {
            for (String video : watchedVideos.get(person)) {
                freq.merge(video, 1, Integer::sum);
            }
        }

        List<String> result = new ArrayList<>(freq.keySet());
        result.sort((a, b) -> freq.get(a) - freq.get(b) != 0 ? freq.get(a) - freq.get(b) : a.compareTo(b));
        return result;
    }
}
```
**Complexity:** `O(V + E)` for BFS, plus `O(K log K)` for sorting the `K` unique videos found. Space `O(V + K)`.

---

### 4. Find if Path Exists in Graph

**LeetCode:** https://leetcode.com/problems/find-if-path-exists-in-graph

**Problem:** Given `n` vertices and bidirectional `edges`, determine if there is a path from `source` to `destination`.

**Example:**
```
n = 3, edges = [[0,1],[1,2],[2,0]], source = 0, destination = 2
Output: true
```

**Approach:** Simple connectivity check. Brute force = repeatedly union/merge components until no changes; optimal = single DFS/BFS or Union-Find pass.

#### Brute Force: Iterative merging of connected sets

```java
import java.util.*;

class Solution {
    public boolean validPath(int n, int[][] edges, int source, int destination) {
        List<Set<Integer>> components = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            Set<Integer> s = new HashSet<>();
            s.add(i);
            components.add(s);
        }
        for (int[] edge : edges) {
            int a = edge[0], b = edge[1];
            Set<Integer> compA = null, compB = null;
            for (Set<Integer> s : components) {
                if (s.contains(a)) compA = s;
                if (s.contains(b)) compB = s;
            }
            if (compA != compB) {
                compA.addAll(compB);
                components.remove(compB);
            }
        }
        for (Set<Integer> s : components) {
            if (s.contains(source) && s.contains(destination)) return true;
        }
        return false;
    }
}
```
**Complexity:** `O(E*n)` due to linear scans for each edge merge, plus `O(n)` for final check.

#### Optimal: BFS

```java
import java.util.*;

class Solution {
    public boolean validPath(int n, int[][] edges, int source, int destination) {
        if (source == destination) return true;
        List<List<Integer>> graph = new ArrayList<>();
        for (int i = 0; i < n; i++) graph.add(new ArrayList<>());
        for (int[] e : edges) {
            graph.get(e[0]).add(e[1]);
            graph.get(e[1]).add(e[0]);
        }

        boolean[] visited = new boolean[n];
        Deque<Integer> queue = new ArrayDeque<>();
        queue.add(source);
        visited[source] = true;

        while (!queue.isEmpty()) {
            int cur = queue.poll();
            if (cur == destination) return true;
            for (int nxt : graph.get(cur)) {
                if (!visited[nxt]) {
                    visited[nxt] = true;
                    queue.add(nxt);
                }
            }
        }
        return false;
    }
}
```
**Complexity:** `O(V + E)` time, `O(V + E)` space.

#### Optimal: Union-Find

```java
class Solution {
    private int[] parent, rank_;

    private int find(int x) {
        while (parent[x] != x) {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    }

    private void union(int a, int b) {
        int ra = find(a), rb = find(b);
        if (ra == rb) return;
        if (rank_[ra] < rank_[rb]) { int t = ra; ra = rb; rb = t; }
        parent[rb] = ra;
        if (rank_[ra] == rank_[rb]) rank_[ra]++;
    }

    public boolean validPath(int n, int[][] edges, int source, int destination) {
        parent = new int[n];
        rank_ = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
        for (int[] e : edges) union(e[0], e[1]);
        return find(source) == find(destination);
    }
}
```
**Complexity:** `O(E α(n))` ≈ `O(E)`. Space `O(n)`.

---

### 5. Detonate the Maximum Bombs

**LeetCode:** https://leetcode.com/problems/detonate-the-maximum-bombs

**Problem:** `bombs[i] = [x, y, r]` — position and blast radius. If bomb `i` detonates, it detonates every bomb whose center lies within radius `r` of `(x,y)` — this can chain (asymmetric). Find the maximum number of bombs that can be detonated by choosing to detonate exactly one initially.

**Example:**
```
bombs = [[1,1,5],[10,10,5]]
Output: 1
Explanation: Neither bomb's radius reaches the other, so detonating either only sets off itself.
```

**Approach:** Build a **directed** graph: edge `i -> j` if bomb `i`'s blast reaches bomb `j`'s center. For each bomb, run DFS/BFS to count chain-detonated bombs. Take the max over all starting bombs.

#### Brute Force / Standard Solution: Graph + DFS per source

```java
import java.util.*;

class Solution {
    public int maximumDetonation(int[][] bombs) {
        int n = bombs.length;
        List<List<Integer>> graph = new ArrayList<>();
        for (int i = 0; i < n; i++) graph.add(new ArrayList<>());

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (i == j) continue;
                long dx = bombs[i][0] - bombs[j][0];
                long dy = bombs[i][1] - bombs[j][1];
                long distSq = dx * dx + dy * dy;
                long rSq = (long) bombs[i][2] * bombs[i][2];
                if (distSq <= rSq) {
                    graph.get(i).add(j);
                }
            }
        }

        int maxCount = 0;
        for (int i = 0; i < n; i++) {
            boolean[] visited = new boolean[n];
            int count = dfs(i, graph, visited);
            maxCount = Math.max(maxCount, count);
        }
        return maxCount;
    }

    private int dfs(int node, List<List<Integer>> graph, boolean[] visited) {
        visited[node] = true;
        int count = 1;
        for (int next : graph.get(node)) {
            if (!visited[next]) {
                count += dfs(next, graph, visited);
            }
        }
        return count;
    }
}
```
**Complexity:** Building graph `O(n^2)`. For each of `n` starting bombs, DFS is `O(n^2)` worst case (dense graph). Total: `O(n^3)`. Space: `O(n^2)`.

#### Optimal: BFS with early exit

```java
import java.util.*;

class Solution {
    public int maximumDetonation(int[][] bombs) {
        int n = bombs.length;
        List<List<Integer>> graph = new ArrayList<>();
        for (int i = 0; i < n; i++) graph.add(new ArrayList<>());

        for (int i = 0; i < n; i++) {
            long xi = bombs[i][0], yi = bombs[i][1], ri = bombs[i][2];
            for (int j = 0; j < n; j++) {
                if (i == j) continue;
                long dx = xi - bombs[j][0];
                long dy = yi - bombs[j][1];
                if (dx * dx + dy * dy <= ri * ri) {
                    graph.get(i).add(j);
                }
            }
        }

        int maxCount = 0;
        for (int i = 0; i < n; i++) {
            maxCount = Math.max(maxCount, bfs(i, graph, n));
            if (maxCount == n) break;
        }
        return maxCount;
    }

    private int bfs(int start, List<List<Integer>> graph, int n) {
        boolean[] visited = new boolean[n];
        Deque<Integer> queue = new ArrayDeque<>();
        queue.add(start);
        visited[start] = true;
        int count = 0;

        while (!queue.isEmpty()) {
            int cur = queue.poll();
            count++;
            for (int next : graph.get(cur)) {
                if (!visited[next]) {
                    visited[next] = true;
                    queue.add(next);
                }
            }
        }
        return count;
    }
}
```
**Complexity:** `O(n^2)` graph build + `O(n^3)` worst case traversal = `O(n^3)` time, `O(n^2)` space.

---

## Section 1-2: Count Degrees

### 1. Find the Town Judge

**LeetCode:** https://leetcode.com/problems/find-the-town-judge

**Problem:** `n` people labeled `1..n`. `trust[i] = [a, b]` means `a` trusts `b`. The town judge trusts nobody but is trusted by everyone else (`n-1` people). Return the judge's label, or `-1` if none exists.

**Example:**
```
n = 3, trust = [[1,3],[2,3]]
Output: 3
```

**Approach:** Track a net score per person: `+1` when trusted, `-1` when trusting someone. The judge is the only person with score `n-1`.

#### Brute Force: Check every candidate against all edges

```java
class Solution {
    public int findJudge(int n, int[][] trust) {
        for (int candidate = 1; candidate <= n; candidate++) {
            boolean trustsSomeone = false;
            int trustedByCount = 0;
            for (int[] t : trust) {
                if (t[0] == candidate) trustsSomeone = true;
                if (t[1] == candidate) trustedByCount++;
            }
            if (!trustsSomeone && trustedByCount == n - 1) return candidate;
        }
        return -1;
    }
}
```
**Complexity:** `O(n * E)`. Space `O(1)`.

#### Optimal: Single-pass degree scoring

```java
class Solution {
    public int findJudge(int n, int[][] trust) {
        int[] score = new int[n + 1];
        for (int[] t : trust) {
            score[t[0]]--;
            score[t[1]]++;
        }
        for (int i = 1; i <= n; i++) {
            if (score[i] == n - 1) return i;
        }
        return -1;
    }
}
```
**Complexity:** `O(n + E)` time, `O(n)` space.

---

### 2. Minimum Number of Vertices to Reach All Nodes

**LeetCode:** https://leetcode.com/problems/minimum-number-of-vertices-to-reach-all-nodes

**Problem:** Given a DAG with `n` nodes and directed `edges`, return the smallest set of nodes from which every node is reachable.

**Example:**
```
n = 6, edges = [[0,1],[0,2],[2,5],[3,4],[4,2]]
Output: [0,3]
```

**Approach:** In a DAG, a node with indegree 0 can never be reached from any other node — it must be a starting point. Every node with indegree ≥ 1 is reachable from its predecessor. Answer = set of indegree-0 nodes.

#### Brute Force: For each node, check reachability from every other node

```java
import java.util.*;

class Solution {
    public List<Integer> findSmallestSetOfVertices(int n, List<List<Integer>> edges) {
        List<List<Integer>> graph = new ArrayList<>();
        for (int i = 0; i < n; i++) graph.add(new ArrayList<>());
        for (List<Integer> e : edges) graph.get(e.get(0)).add(e.get(1));

        List<Integer> result = new ArrayList<>();
        for (int v = 0; v < n; v++) {
            boolean reachable = false;
            for (int u = 0; u < n && !reachable; u++) {
                if (u == v) continue;
                reachable = canReach(u, v, graph, new boolean[n]);
            }
            if (!reachable) result.add(v);
        }
        return result;
    }

    private boolean canReach(int u, int target, List<List<Integer>> graph, boolean[] visited) {
        if (u == target) return true;
        visited[u] = true;
        for (int next : graph.get(u)) {
            if (!visited[next] && canReach(next, target, graph, visited)) return true;
        }
        return false;
    }
}
```
**Complexity:** `O(n * n * (V+E))`.

#### Optimal: Indegree counting

```java
import java.util.*;

class Solution {
    public List<Integer> findSmallestSetOfVertices(int n, List<List<Integer>> edges) {
        int[] indegree = new int[n];
        for (List<Integer> e : edges) indegree[e.get(1)]++;

        List<Integer> result = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            if (indegree[i] == 0) result.add(i);
        }
        return result;
    }
}
```
**Complexity:** `O(V + E)` time, `O(V)` space.

---

### 3. Maximal Network Rank

**LeetCode:** https://leetcode.com/problems/maximal-network-rank

**Problem:** `n` cities, undirected `roads`. Network rank of a pair `(a,b)` = total roads connected to `a` or `b`, counting the direct road between them only once (if it exists). Return the max network rank over all pairs.

**Example:**
```
n = 4, roads = [[0,1],[0,3],[1,2],[1,3]]
Output: 4
Explanation: Pair (0,1): degree(0)=2, degree(1)=3, connected directly -> 2+3-1=4
```

**Approach:** Precompute each city's degree, and store adjacency for O(1) "are they directly connected" lookups. Compute `degree[a] + degree[b] - (1 if connected)` for every pair.

#### Brute Force: Recompute adjacency by scanning roads each time

```java
class Solution {
    public int maximalNetworkRank(int n, int[][] roads) {
        int[] degree = new int[n];
        for (int[] r : roads) {
            degree[r[0]]++;
            degree[r[1]]++;
        }

        int maxRank = 0;
        for (int a = 0; a < n; a++) {
            for (int b = a + 1; b < n; b++) {
                boolean connected = false;
                for (int[] r : roads) {
                    if ((r[0] == a && r[1] == b) || (r[0] == b && r[1] == a)) {
                        connected = true;
                        break;
                    }
                }
                int rank = degree[a] + degree[b] - (connected ? 1 : 0);
                maxRank = Math.max(maxRank, rank);
            }
        }
        return maxRank;
    }
}
```
**Complexity:** `O(n^2 * E)`.

#### Optimal: Adjacency matrix / set for O(1) lookup

```java
class Solution {
    public int maximalNetworkRank(int n, int[][] roads) {
        int[] degree = new int[n];
        boolean[][] connected = new boolean[n][n];
        for (int[] r : roads) {
            degree[r[0]]++;
            degree[r[1]]++;
            connected[r[0]][r[1]] = true;
            connected[r[1]][r[0]] = true;
        }

        int maxRank = 0;
        for (int a = 0; a < n; a++) {
            for (int b = a + 1; b < n; b++) {
                int rank = degree[a] + degree[b] - (connected[a][b] ? 1 : 0);
                maxRank = Math.max(maxRank, rank);
            }
        }
        return maxRank;
    }
}
```
**Complexity:** `O(n^2)` time, `O(n^2)` space.

---

### 4. Minimum Degree of a Connected Trio in a Graph

**LeetCode:** https://leetcode.com/problems/minimum-degree-of-a-connected-trio-in-a-graph

**Problem:** A connected trio is 3 mutually-connected nodes (a triangle). The degree of a trio = sum of the 3 nodes' degrees, minus 6. Return the minimum trio degree, or `-1` if no trio exists.

**Example:**
```
n = 6, edges = [[1,2],[1,3],[3,2],[4,1],[5,2],[3,6]]
Output: 3
Explanation: Trio (1,2,3): degree(1)=3, degree(2)=3, degree(3)=3 -> 3+3+3-6=3
```

**Approach:** Precompute degree per node and an adjacency matrix for O(1) edge checks. Enumerate all triples; if all three edges exist, compute the trio degree.

#### Brute Force: Check edges via linear scan through edge list

```java
class Solution {
    public int minTrioDegree(int n, int[][] edges) {
        int[] degree = new int[n + 1];
        for (int[] e : edges) {
            degree[e[0]]++;
            degree[e[1]]++;
        }

        int minDegree = Integer.MAX_VALUE;
        for (int u = 1; u <= n; u++) {
            for (int v = u + 1; v <= n; v++) {
                if (!hasEdge(edges, u, v)) continue;
                for (int w = v + 1; w <= n; w++) {
                    if (hasEdge(edges, u, w) && hasEdge(edges, v, w)) {
                        int trioDeg = degree[u] + degree[v] + degree[w] - 6;
                        minDegree = Math.min(minDegree, trioDeg);
                    }
                }
            }
        }
        return minDegree == Integer.MAX_VALUE ? -1 : minDegree;
    }

    private boolean hasEdge(int[][] edges, int a, int b) {
        for (int[] e : edges) {
            if ((e[0] == a && e[1] == b) || (e[0] == b && e[1] == a)) return true;
        }
        return false;
    }
}
```
**Complexity:** `O(n^3 * E)`.

#### Optimal: Adjacency matrix lookup

```java
class Solution {
    public int minTrioDegree(int n, int[][] edges) {
        int[] degree = new int[n + 1];
        boolean[][] adj = new boolean[n + 1][n + 1];
        for (int[] e : edges) {
            degree[e[0]]++;
            degree[e[1]]++;
            adj[e[0]][e[1]] = true;
            adj[e[1]][e[0]] = true;
        }

        int minDegree = Integer.MAX_VALUE;
        for (int u = 1; u <= n; u++) {
            for (int v = u + 1; v <= n; v++) {
                if (!adj[u][v]) continue;
                for (int w = v + 1; w <= n; w++) {
                    if (adj[u][w] && adj[v][w]) {
                        int trioDeg = degree[u] + degree[v] + degree[w] - 6;
                        minDegree = Math.min(minDegree, trioDeg);
                    }
                }
            }
        }
        return minDegree == Integer.MAX_VALUE ? -1 : minDegree;
    }
}
```
**Complexity:** `O(n^3)` time, `O(n^2)` space.

---

### 5. Count Pairs of Nodes

**LeetCode:** https://leetcode.com/problems/count-pairs-of-nodes

**Problem:** Undirected `edges` (may have duplicates). For each `query[i]`, count pairs `(a,b)` with `a < b` such that `incident(a,b) = degree[a] + degree[b] - (1 if edge(a,b) exists) > query[i]`.

**Example:**
```
n = 4, edges = [[1,2],[2,4],[1,3],[2,3],[2,1]]
queries = [2,3]
Output: [6,5]
```

**Approach:** Compute degree of every node. Count pairs using a sorted-degree two-pointer scan for `degree[a]+degree[b] > q`, then correct for pairs that share a direct edge (which reduces incident count).

#### Brute Force: Check every pair directly per query

```java
import java.util.*;

class Solution {
    public int[] countPairs(int n, int[][] edges, int[] queries) {
        int[] degree = new int[n + 1];
        Map<Long, Integer> edgeCount = new HashMap<>();
        for (int[] e : edges) {
            int a = Math.min(e[0], e[1]), b = Math.max(e[0], e[1]);
            degree[e[0]]++;
            degree[e[1]]++;
            edgeCount.merge((long) a * 100001 + b, 1, Integer::sum);
        }

        int[] result = new int[queries.length];
        for (int i = 0; i < queries.length; i++) {
            int count = 0;
            for (int a = 1; a <= n; a++) {
                for (int b = a + 1; b <= n; b++) {
                    long key = (long) a * 100001 + b;
                    int shared = edgeCount.getOrDefault(key, 0);
                    int incident = degree[a] + degree[b] - shared;
                    if (incident > queries[i]) count++;
                }
            }
            result[i] = count;
        }
        return result;
    }
}
```
**Complexity:** `O(Q * n^2)`. Space `O(E)`.

#### Optimal: Sorted degrees + two-pointer, with edge correction

```java
import java.util.*;

class Solution {
    public int[] countPairs(int n, int[][] edges, int[] queries) {
        int[] degree = new int[n + 1];
        for (int[] e : edges) {
            degree[e[0]]++;
            degree[e[1]]++;
        }

        int[] sortedDeg = Arrays.copyOfRange(degree, 1, n + 1);
        Arrays.sort(sortedDeg);

        int[] result = new int[queries.length];
        for (int i = 0; i < queries.length; i++) {
            int q = queries[i];
            long count = 0;
            int lo = 0, hi = n - 1;
            while (lo < hi) {
                if (sortedDeg[lo] + sortedDeg[hi] > q) {
                    count += hi - lo;
                    hi--;
                } else {
                    lo++;
                }
            }

            Set<Long> seenPairs = new HashSet<>();
            for (int[] e : edges) {
                int a = Math.min(e[0], e[1]), b = Math.max(e[0], e[1]);
                long key = (long) a * 100001 + b;
                if (seenPairs.contains(key)) continue;
                seenPairs.add(key);
                int sum = degree[a] + degree[b];
                boolean sumCounted = sum > q;
                boolean actualCounted = (sum - 1) > q;
                if (sumCounted && !actualCounted) count--;
            }
            result[i] = (int) count;
        }
        return result;
    }
}
```
**Complexity:** Sorting `O(n log n)`. Each query: `O(n + E)`. Total: `O(n log n + Q*(n+E))`. Space `O(n + E)`.

---

### 6. Find Center of Star Graph

**LeetCode:** https://leetcode.com/problems/find-center-of-star-graph

**Problem:** In a star graph, one central node connects to every other node. Given `edges`, find the center.

**Example:**
```
edges = [[1,2],[2,3],[4,2]]
Output: 2
```

**Approach:** The center node appears in every edge, so it must appear in both of the first two edges.

#### Brute Force: Count degree of every node

```java
import java.util.*;

class Solution {
    public int findCenter(int[][] edges) {
        Map<Integer, Integer> degree = new HashMap<>();
        for (int[] e : edges) {
            degree.merge(e[0], 1, Integer::sum);
            degree.merge(e[1], 1, Integer::sum);
        }
        int n = edges.length + 1;
        for (Map.Entry<Integer, Integer> entry : degree.entrySet()) {
            if (entry.getValue() == n - 1) return entry.getKey();
        }
        return -1;
    }
}
```
**Complexity:** `O(n)` time, `O(n)` space.

#### Optimal: Compare first two edges

```java
class Solution {
    public int findCenter(int[][] edges) {
        int a1 = edges[0][0], b1 = edges[0][1];
        int a2 = edges[1][0], b2 = edges[1][1];
        return (a1 == a2 || a1 == b2) ? a1 : b1;
    }
}
```
**Complexity:** `O(1)` time, `O(1)` space.

---

### 7. Maximum Total Importance of Roads

**LeetCode:** https://leetcode.com/problems/maximum-total-importance-of-roads

**Problem:** `n` cities, undirected `roads`. Assign each city a unique value from `1..n`. Total importance = sum over all roads of `(value[u] + value[v])`. Maximize total importance.

**Example:**
```
n = 5, roads = [[0,1],[1,2],[2,3],[0,2],[1,3],[2,4]]
Output: 48
```

**Approach:** A city's total contribution to the sum is `degree(city) * value(city)`. Assign the largest value to the highest-degree city (greedy / rearrangement inequality).

#### Brute Force: Repeatedly pick the max-degree unassigned city (linear scan)

```java
import java.util.*;

class Solution {
    public long maximumImportance(int n, int[][] roads) {
        int[] degree = new int[n];
        for (int[] r : roads) {
            degree[r[0]]++;
            degree[r[1]]++;
        }

        int[] value = new int[n];
        boolean[] assigned = new boolean[n];
        int currentValue = n;
        for (int step = 0; step < n; step++) {
            int bestCity = -1, bestDeg = -1;
            for (int i = 0; i < n; i++) {
                if (!assigned[i] && degree[i] > bestDeg) {
                    bestDeg = degree[i];
                    bestCity = i;
                }
            }
            value[bestCity] = currentValue--;
            assigned[bestCity] = true;
        }

        long total = 0;
        for (int[] r : roads) {
            total += value[r[0]] + value[r[1]];
        }
        return total;
    }
}
```
**Complexity:** `O(n^2)` for repeated max-scans + `O(E)` for final sum.

#### Optimal: Sort by degree once

```java
import java.util.*;

class Solution {
    public long maximumImportance(int n, int[][] roads) {
        int[] degree = new int[n];
        for (int[] r : roads) {
            degree[r[0]]++;
            degree[r[1]]++;
        }

        Integer[] cities = new Integer[n];
        for (int i = 0; i < n; i++) cities[i] = i;
        Arrays.sort(cities, (a, b) -> degree[b] - degree[a]);

        int[] value = new int[n];
        int currentValue = n;
        for (int city : cities) {
            value[city] = currentValue--;
        }

        long total = 0;
        for (int[] r : roads) {
            total += value[r[0]] + value[r[1]];
        }
        return total;
    }
}
```
**Complexity:** `O(n log n + E)` time, `O(n)` space.

---

### 8. Node with Highest Edge Score

**LeetCode:** https://leetcode.com/problems/node-with-highest-edge-score

**Problem:** `n` nodes, each node `i` has exactly one outgoing edge to `edges[i]`. The edge score of a node = sum of indices of all nodes pointing to it. Return the node with the max score (smallest index on tie).

**Example:**
```
edges = [1,0,0,0,0,7,7,5]
Output: 7
```

**Approach:** Single-pass accumulation: `score[edges[i]] += i`.

#### Brute Force: For each node, scan all edges to sum contributions

```java
class Solution {
    public int edgeScore(int[] edges) {
        int n = edges.length;
        long[] score = new long[n];
        for (int target = 0; target < n; target++) {
            long sum = 0;
            for (int i = 0; i < n; i++) {
                if (edges[i] == target) sum += i;
            }
            score[target] = sum;
        }

        int best = 0;
        for (int i = 1; i < n; i++) {
            if (score[i] > score[best]) best = i;
        }
        return best;
    }
}
```
**Complexity:** `O(n^2)` time, `O(n)` space.

#### Optimal: Single pass

```java
class Solution {
    public int edgeScore(int[] edges) {
        int n = edges.length;
        long[] score = new long[n];
        for (int i = 0; i < n; i++) {
            score[edges[i]] += i;
        }

        int best = 0;
        for (int i = 1; i < n; i++) {
            if (score[i] > score[best]) best = i;
        }
        return best;
    }
}
```
**Complexity:** `O(n)` time, `O(n)` space.

---

### 9. Maximum Star Sum of a Graph

**LeetCode:** https://leetcode.com/problems/maximum-star-sum-of-a-graph

**Problem:** `n` nodes with values `vals[i]`, undirected `edges`, and integer `k`. A star centered at node `u` includes `u` and up to `k` of its neighbors. Star sum = `vals[u]` + sum of the chosen neighbor values (only include positive-value neighbors, up to `k` of them). Return the max star sum over all centers.

**Example:**
```
vals = [1,2,3,4,10,-10,-20], edges = [[0,1],[1,2],[1,3],[3,4],[3,5],[3,6]]
k = 2
Output: 16
```

**Approach:** For each node as center, gather its neighbors' values and greedily take the top `k` positive values.

#### Brute Force: Repeated max-extraction (no sorting) for each center

```java
import java.util.*;

class Solution {
    public long maxStarSum(int[] vals, int[][] edges, int k) {
        int n = vals.length;
        List<List<Integer>> graph = new ArrayList<>();
        for (int i = 0; i < n; i++) graph.add(new ArrayList<>());
        for (int[] e : edges) {
            graph.get(e[0]).add(e[1]);
            graph.get(e[1]).add(e[0]);
        }

        long maxSum = Long.MIN_VALUE;
        for (int center = 0; center < n; center++) {
            List<Integer> neighborVals = new ArrayList<>();
            for (int nb : graph.get(center)) neighborVals.add(vals[nb]);

            long sum = vals[center];
            boolean[] used = new boolean[neighborVals.size()];
            for (int pick = 0; pick < k; pick++) {
                int bestIdx = -1, bestVal = 0;
                for (int j = 0; j < neighborVals.size(); j++) {
                    if (!used[j] && neighborVals.get(j) > bestVal) {
                        bestVal = neighborVals.get(j);
                        bestIdx = j;
                    }
                }
                if (bestIdx == -1) break;
                used[bestIdx] = true;
                sum += bestVal;
            }
            maxSum = Math.max(maxSum, sum);
        }
        return maxSum;
    }
}
```
**Complexity:** `O(V * k * d)` where `d` = average degree, worst case `O(V*d^2)`.

#### Optimal: Sort neighbor values once per center

```java
import java.util.*;

class Solution {
    public long maxStarSum(int[] vals, int[][] edges, int k) {
        int n = vals.length;
        List<List<Integer>> graph = new ArrayList<>();
        for (int i = 0; i < n; i++) graph.add(new ArrayList<>());
        for (int[] e : edges) {
            graph.get(e[0]).add(e[1]);
            graph.get(e[1]).add(e[0]);
        }

        long maxSum = Long.MIN_VALUE;
        for (int center = 0; center < n; center++) {
            List<Integer> neighborVals = new ArrayList<>();
            for (int nb : graph.get(center)) neighborVals.add(vals[nb]);
            neighborVals.sort(Collections.reverseOrder());

            long sum = vals[center];
            for (int i = 0; i < Math.min(k, neighborVals.size()); i++) {
                if (neighborVals.get(i) <= 0) break;
                sum += neighborVals.get(i);
            }
            maxSum = Math.max(maxSum, sum);
        }
        return maxSum;
    }
}
```
**Complexity:** `O(V * d log d)` total, bounded by `O(E log E)`. Space `O(V + E)`.

---

### 10. Add Edges to Make Degrees of All Nodes Even

**LeetCode:** https://leetcode.com/problems/add-edges-to-make-degrees-of-all-nodes-even

**Problem:** Undirected graph with `n` nodes and `edges` (no self-loops or duplicate edges). You may add at most 2 new edges (no self-loops, no duplicates). Determine if it's possible to make every node's degree even.

**Example:**
```
n = 5, edges = [[1,2],[2,3],[3,4],[4,2],[1,4],[2,5]]
Output: true
```

**Approach:** Count nodes with odd degree.
- **0 odd nodes:** already valid → `true`.
- **2 odd nodes** `(a,b)`: try connecting them directly if no existing edge → `true`. Otherwise find a node `c` not already connected to both `a` and `b`, and add `(a,c)` and `(c,b)`.
- **4 odd nodes** `(a,b,c,d)`: try all 3 pairings — if any pairing has both edges non-existent, → `true`.
- **Any other count:** impossible → `false`.

#### Brute Force: Try all possible pairs of new edges exhaustively

```java
import java.util.*;

class Solution {
    public boolean isPossible(int n, List<List<Integer>> edges) {
        Set<Long> edgeSet = new HashSet<>();
        int[] degree = new int[n + 1];
        for (List<Integer> e : edges) {
            int a = e.get(0), b = e.get(1);
            degree[a]++;
            degree[b]++;
            edgeSet.add(key(a, b));
        }

        List<Integer> oddNodes = new ArrayList<>();
        for (int i = 1; i <= n; i++) if (degree[i] % 2 == 1) oddNodes.add(i);

        if (oddNodes.isEmpty()) return true;
        if (oddNodes.size() % 2 != 0 || oddNodes.size() > 4) return false;

        List<int[]> candidates = new ArrayList<>();
        for (int i = 1; i <= n; i++) {
            for (int j = i + 1; j <= n; j++) {
                if (!edgeSet.contains(key(i, j))) candidates.add(new int[]{i, j});
            }
        }

        for (int[] c : candidates) {
            int[] tempDeg = degree.clone();
            tempDeg[c[0]]++; tempDeg[c[1]]++;
            if (allEven(tempDeg, n)) return true;
        }

        for (int i = 0; i < candidates.size(); i++) {
            for (int j = i + 1; j < candidates.size(); j++) {
                int[] c1 = candidates.get(i), c2 = candidates.get(j);
                int[] tempDeg = degree.clone();
                tempDeg[c1[0]]++; tempDeg[c1[1]]++;
                tempDeg[c2[0]]++; tempDeg[c2[1]]++;
                if (allEven(tempDeg, n)) return true;
            }
        }
        return false;
    }

    private boolean allEven(int[] degree, int n) {
        for (int i = 1; i <= n; i++) if (degree[i] % 2 != 0) return false;
        return true;
    }

    private long key(int a, int b) {
        return (long) Math.min(a, b) * 100001 + Math.max(a, b);
    }
}
```
**Complexity:** `O(n^4)` — trying all pairs of candidate edges. Impractical for large `n`, shown for completeness.

#### Optimal: Case analysis on odd-degree node count

```java
import java.util.*;

class Solution {
    public boolean isPossible(int n, List<List<Integer>> edges) {
        Set<Long> edgeSet = new HashSet<>();
        int[] degree = new int[n + 1];
        for (List<Integer> e : edges) {
            int a = e.get(0), b = e.get(1);
            degree[a]++;
            degree[b]++;
            edgeSet.add(key(a, b));
        }

        List<Integer> odd = new ArrayList<>();
        for (int i = 1; i <= n; i++) if (degree[i] % 2 == 1) odd.add(i);

        if (odd.isEmpty()) return true;
        if (odd.size() % 2 != 0 || odd.size() > 4) return false;

        if (odd.size() == 2) {
            int a = odd.get(0), b = odd.get(1);
            if (!edgeSet.contains(key(a, b))) return true;
            for (int c = 1; c <= n; c++) {
                if (c == a || c == b) continue;
                if (!edgeSet.contains(key(a, c)) && !edgeSet.contains(key(b, c))) return true;
            }
            return false;
        }

        int a = odd.get(0), b = odd.get(1), c = odd.get(2), d = odd.get(3);
        int[][][] pairings = {
            {{a, b}, {c, d}},
            {{a, c}, {b, d}},
            {{a, d}, {b, c}}
        };
        for (int[][] pairing : pairings) {
            if (!edgeSet.contains(key(pairing[0][0], pairing[0][1]))
                && !edgeSet.contains(key(pairing[1][0], pairing[1][1]))) {
                return true;
            }
        }
        return false;
    }

    private long key(int a, int b) {
        return (long) Math.min(a, b) * 100001 + Math.max(a, b);
    }
}
```
**Complexity:** `O(n + E)` for building degree/edge sets, plus `O(n)` worst case for the 2-odd-node search. Overall `O(n + E)`. Space `O(n + E)`.

---

### 11. Find Champion II

**LeetCode:** https://leetcode.com/problems/find-champion-ii

**Problem:** DAG with `n` teams; edge `u -> v` means team `u` is stronger than team `v`. A definitive champion exists only if there's exactly one team with indegree 0. Return that team, or `-1`.

**Example:**
```
n = 3, edges = [[0,1],[1,2]]
Output: 0
```

**Approach:** Compute indegree for all nodes. If exactly one node has indegree 0, that's the champion. If more than one node has indegree 0, no definitive comparison exists → `-1`.

#### Brute Force: For each candidate, scan all edges to check if anyone beats them

```java
class Solution {
    public int findChampion(int n, int[][] edges) {
        int championCandidate = -1;
        int candidateCount = 0;

        for (int team = 0; team < n; team++) {
            boolean beaten = false;
            for (int[] e : edges) {
                if (e[1] == team) {
                    beaten = true;
                    break;
                }
            }
            if (!beaten) {
                candidateCount++;
                championCandidate = team;
            }
        }
        return candidateCount == 1 ? championCandidate : -1;
    }
}
```
**Complexity:** `O(n * E)`.

#### Optimal: Single-pass indegree count

```java
class Solution {
    public int findChampion(int n, int[][] edges) {
        int[] indegree = new int[n];
        for (int[] e : edges) {
            indegree[e[1]]++;
        }

        int champion = -1;
        int zeroIndegreeCount = 0;
        for (int i = 0; i < n; i++) {
            if (indegree[i] == 0) {
                zeroIndegreeCount++;
                champion = i;
            }
        }
        return zeroIndegreeCount == 1 ? champion : -1;
    }
}
```
**Complexity:** `O(n + E)` time, `O(n)` space.

---

*More sections (Topological Sorting, Union-Find, Bipartite, Dijkstra, Bellman-Ford, Floyd-Warshall, and advanced topics) to be added as we cover them.*
