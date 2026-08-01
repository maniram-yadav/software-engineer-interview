# Section 1-1: Simple DFS/BFS

---

## 1. Evaluate Division

**Problem:** You're given equations like `a/b = 2.0`, `b/c = 3.0` as pairs `equations = [[a,b],[b,c]]` with `values = [2.0, 3.0]`. Answer queries like `a/c = ?` by treating each equation as a weighted, directed edge in a graph. If a query variable doesn't exist in the graph, or no path connects the two, return `-1.0`.

**Example:**
```
equations = [["a","b"],["b","c"]], values = [2.0,3.0]
queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
Output: [6.0, 0.5, -1.0, 1.0, -1.0]
```
`a/c = (a/b)*(b/c) = 2.0*3.0 = 6.0`

### Approach
Build a weighted graph: `a -> b` with weight `2.0`, and `b -> a` with weight `1/2.0`. For each query, DFS/BFS from source to destination, multiplying edge weights along the way.

**Brute Force = Optimal here** — there's no meaningfully "worse" brute force beyond DFS/BFS over the graph (trying to enumerate all paths would just be a variant of DFS with no memoization, which is essentially the same). I'll show the standard DFS solution, and also the Union-Find (weighted DSU) alternative as a second approach.

### Solution 1: DFS (Graph traversal)

```java
import java.util.*;

class Solution {
    public double[] calcEquation(List<List<String>> equations, double[] values, List<List<String>> queries) {
        // Build weighted adjacency list
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
**Complexity:** Building graph `O(E)`. Each query DFS is `O(V + E)` worst case. Total: `O(E + Q*(V+E))`. Space: `O(V + E)` for graph + recursion stack.

### Solution 2: Weighted Union-Find (alternative optimal approach)

```java
import java.util.*;

class Solution {
    private Map<String, String> parent = new HashMap<>();
    private Map<String, Double> weight = new HashMap<>(); // ratio to parent

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
        // weight[a]/weight[rootA] combined with val to relate rootA to rootB
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
**Complexity:** Nearly `O(1)` per operation with path compression (amortized inverse-Ackermann). Total `O((V+Q) α(V))`. Better when there are many queries.

---

## 2. Keys and Rooms

**Problem:** `n` rooms numbered `0` to `n-1`. Room `0` is unlocked initially. `rooms[i]` is a list of keys found in room `i`, each key `rooms[i][j]` opens room `rooms[i][j]`. Return `true` if you can visit every room.

**Example:**
```
rooms = [[1],[2],[3],[]]
Output: true
Explanation: Start in room 0, get key 1 -> go to room 1, get key 2 -> room 2, get key 3 -> room 3.
```

### Approach
This is just reachability from node `0` in a directed graph. Brute force and optimal are essentially the same (DFS/BFS) — the only "brute force" alternative is repeatedly re-scanning all rooms until no new room is unlockable (a fixed-point iteration), which is worse.

### Brute Force: Repeated scanning (fixed-point iteration)

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
**Complexity:** Each full pass is `O(n)` rooms, and could take up to `O(n)` passes to converge → `O(n^2)` worst case. Space `O(n)`.

### Optimal: DFS

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
**Complexity:** `O(V + E)` where V = rooms, E = total keys. Space: `O(V)` recursion stack + visited array.

### Optimal: BFS (iterative, avoids recursion stack overflow)

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

## 3. Get Watched Videos by Your Friends

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

### Approach
BFS from `id` to find the exact set of people at distance `level`. Then collect all their watched videos, count frequencies, sort.

### Brute Force: DFS collecting all distances, then filter

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

    // Note: plain DFS doesn't guarantee shortest distance without extra care;
    // this brute-force version explores all paths and keeps the minimum found.
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
**Complexity:** Since edges are unweighted, this "DFS with relaxation" can revisit nodes multiple times — worst case `O(V*E)`. Space `O(V)`.

### Optimal: BFS (guarantees shortest distance in one pass)

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

        Map<String, Integer> freq = new TreeMap<>(); // TreeMap gives alphabetical order for free on ties
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

## 4. Find if Path Exists in Graph

**Problem:** Given `n` vertices and bidirectional `edges`, determine if there is a path from `source` to `destination`.

**Example:**
```
n = 3, edges = [[0,1],[1,2],[2,0]], source = 0, destination = 2
Output: true
```

### Approach
Simple connectivity check. Brute force = repeatedly union/merge components until no changes; optimal = single DFS/BFS or Union-Find pass.

### Brute Force: Iterative merging of connected sets

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
**Complexity:** `O(E*n)` due to linear scans for each edge merge, plus `O(n)` for final check. Very inefficient — shown only for contrast.

### Optimal: BFS

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

### Optimal: Union-Find

```java
class Solution {
    private int[] parent, rank_;

    private int find(int x) {
        while (parent[x] != x) {
            parent[x] = parent[parent[x]]; // path compression
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
**Complexity:** `O(E α(n))` ≈ `O(E)`, essentially constant per operation. Space `O(n)`.

---

## 5. Detonate the Maximum Bombs

**Problem:** `bombs[i] = [x, y, r]` — position and blast radius. If bomb `i` detonates, it detonates every bomb whose center lies within radius `r` of `(x,y)` — this can chain (asymmetric: A can trigger B even if B's radius wouldn't reach A). Find the maximum number of bombs that can be detonated by choosing to detonate exactly one initially.

**Example:**
```
bombs = [[2,1,3],[6,1,4]]
Output: 2
Explanation: Bomb 0's radius covers bomb 1 (distance = 4 <= 3? Actually check: distance between centers = 4, bomb0 radius=3 doesn't reach... 
```
(Use the well-known example instead:)
```
bombs = [[1,1,5],[10,10,5]]
Output: 1
Explanation: Neither bomb's radius reaches the other, so detonating either only sets off itself.
```

### Approach
Build a **directed** graph: edge `i -> j` if bomb `i`'s blast reaches bomb `j`'s center (distance ≤ radius_i). For each bomb, run DFS/BFS to count how many bombs get chain-detonated if it's the starting bomb. Take the max over all starting bombs.

### Brute Force: For each bomb, simulate detonation via DFS, O(n) per node reachability check redone (this IS essentially the standard approach — true "brute force" would be trying all subsets, which is exponential and impractical, so the reasonable brute force is "graph + naive DFS per source" which is what's shown below; note complexity is inherently O(n^2) building + O(n) per traversal).

```java
import java.util.*;

class Solution {
    public int maximumDetonation(int[][] bombs) {
        int n = bombs.length;
        List<List<Integer>> graph = new ArrayList<>();
        for (int i = 0; i < n; i++) graph.add(new ArrayList<>());

        // Build directed reachability graph: O(n^2)
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
**Complexity:** Building graph `O(n^2)`. For each of `n` starting bombs, DFS is `O(V+E) = O(n^2)` worst case (dense graph). Total: `O(n^3)`. Space: `O(n^2)` for graph.

### Optimal: Same graph-build (unavoidable `O(n^2)` since every pair must be checked), but BFS with early bit-set optimizations — in practice the "optimal" for this problem *is* `O(n^3)` (LeetCode's accepted complexity, since n ≤ 100). We can slightly optimize with **BFS + boolean array reuse** to avoid recursion overhead, and prune impossible pairs early using squared distance (avoiding `sqrt`, already done above).

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
            if (maxCount == n) break; // early exit — can't do better than detonating all
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
**Complexity:** `O(n^2)` graph build + `O(n)` sources × `O(n^2)` BFS worst case = `O(n^3)` time, `O(n^2)` space. The early-exit when `maxCount == n` is a practical speedup but doesn't change worst-case complexity.

---

That covers **1-1 Simple DFS/BFS** in full. Want me to move on to **1-2 Count Degrees** next, or jump to one of the other sections (Topological Sort, Union-Find, Dijkstra)?