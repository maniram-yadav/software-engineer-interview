# The Complete DSA Interview Guide
### Topics → Patterns → Approach → Complexity (Easy → Very Hard)

---

## 📋 Table of Contents (All DSA Topics)

1. [Big-O / Complexity Primer](#big-o-primer)
2. [Arrays & Strings](#1-arrays--strings)
3. [Two Pointers](#2-two-pointers)
4. [Sliding Window](#3-sliding-window)
5. [Hashing (HashMap/HashSet)](#4-hashing)
6. [Sorting](#5-sorting)
7. [Binary Search](#6-binary-search)
8. [Recursion & Backtracking](#7-recursion--backtracking)
9. [Linked List](#8-linked-list)
10. [Stack](#9-stack)
11. [Queue & Monotonic Deque](#10-queue--monotonic-deque)
12. [Trees (Binary Tree / BST)](#11-trees)
13. [Heap / Priority Queue](#12-heap--priority-queue)
14. [Graphs](#13-graphs)
15. [Dynamic Programming](#14-dynamic-programming)
16. [Greedy](#15-greedy)
17. [Trie](#16-trie)
18. [Union-Find (Disjoint Set)](#17-union-find)
19. [Segment Tree / Fenwick Tree (BIT)](#18-segment-tree--fenwick-tree)
20. [String Algorithms](#19-string-algorithms)
21. [Bit Manipulation](#20-bit-manipulation)
22. [Math & Number Theory](#21-math--number-theory)
23. [Divide and Conquer](#22-divide-and-conquer)
24. [Design Problems](#23-design-problems)

---

<a id="big-o-primer"></a>
## 🧮 Big-O Primer (cheat sheet used throughout)

| Complexity | Name | Example |
|---|---|---|
| O(1) | Constant | array index access |
| O(log n) | Logarithmic | binary search |
| O(n) | Linear | single loop scan |
| O(n log n) | Linearithmic | merge sort, heap sort |
| O(n²) | Quadratic | nested loops, brute-force pairs |
| O(n³) | Cubic | triple nested loops, Floyd-Warshall |
| O(2ⁿ) | Exponential | subsets, brute-force recursion |
| O(n!) | Factorial | permutations |

**Difficulty tiers used below:** 🟢 Easy · 🟡 Medium · 🔴 Hard · ⚫ Very Hard

---

## 1. Arrays & Strings

Foundational topic — most other patterns (two pointers, sliding window, prefix sums) operate on top of arrays.

### Patterns

**🟢 Prefix Sum**
- *Problem type:* Range sum queries, subarray sum equals K.
- *Approach:* Precompute `prefix[i] = prefix[i-1] + arr[i]`. Range sum(l,r) = `prefix[r] - prefix[l-1]`. For "subarray sum = K", use a hashmap of prefix sums seen so far.
- *Complexity:* Build O(n), Query O(1); Space O(n).

**🟢 Kadane's Algorithm**
- *Problem type:* Maximum subarray sum.
- *Approach:* Track `currentSum = max(arr[i], currentSum + arr[i])`, update `maxSum` globally.
- *Complexity:* O(n) time, O(1) space.

**🟡 Rotate / In-place Array Manipulation**
- *Problem type:* Rotate array by k, move zeroes, Dutch national flag (sort 0s,1s,2s).
- *Approach:* Reversal algorithm for rotation (reverse whole, reverse parts). Three-pointer partition (low/mid/high) for Dutch flag.
- *Complexity:* O(n) time, O(1) space.

**🟡 Matrix Traversal**
- *Problem type:* Spiral matrix, rotate image, set matrix zeroes.
- *Approach:* Use 4 boundary pointers (top,bottom,left,right) and shrink; for rotate: transpose + reverse rows.
- *Complexity:* O(n×m) time, O(1) extra space (in-place variants).

**🔴 Next Permutation / Trapping Rain Water**
- *Problem type:* Next lexicographic permutation, trapping rain water, product of array except self.
- *Approach:* Next permutation — find pivot from right where arr[i] < arr[i+1], swap with next greater, reverse suffix. Trapping rain water — precompute leftMax/rightMax arrays or two-pointer.
- *Complexity:* O(n) time, O(1) space (two-pointer version).

**⚫ Median of Two Sorted Arrays**
- *Problem type:* Find median of two sorted arrays.
- *Approach:* Binary search on the smaller array's partition point so that left-half max ≤ right-half min across both arrays.
- *Complexity:* O(log(min(n,m))) time, O(1) space.

---

## 2. Two Pointers

### Patterns

**🟢 Pair Sum in Sorted Array**
- *Problem type:* Two Sum II, valid palindrome.
- *Approach:* One pointer from start, one from end; move based on comparison to target.
- *Complexity:* O(n) time, O(1) space.

**🟡 Three Sum / Four Sum**
- *Problem type:* 3Sum, 3Sum closest, 4Sum.
- *Approach:* Sort array, fix one (or two) elements, use two-pointer for the rest; skip duplicates.
- *Complexity:* O(n²) for 3Sum, O(n³) for 4Sum; O(1) extra space (excluding sort).

**🟡 Fast & Slow Pointers**
- *Problem type:* Cycle detection, find middle of linked list, happy number.
- *Approach:* Floyd's cycle detection — slow moves 1 step, fast moves 2 steps; they meet if a cycle exists.
- *Complexity:* O(n) time, O(1) space.

**🔴 Container With Most Water / Trapping Rain Water (2-pointer)**
- *Problem type:* Max area between two lines.
- *Approach:* Start from widest boundaries, move the pointer at the shorter line inward (greedy — moving the taller one can't improve area).
- *Complexity:* O(n) time, O(1) space.

**🔴 Merge Intervals via Pointers / Partition Problems**
- *Problem type:* Sort colors, partition labels.
- *Approach:* Track last-seen index of each element to determine partition boundaries; extend pointer greedily.
- *Complexity:* O(n) time, O(1)–O(26) space.

---

## 3. Sliding Window

### Patterns

**🟢 Fixed-size Window**
- *Problem type:* Max sum subarray of size K, average of subarrays.
- *Approach:* Maintain window sum; add new element, remove element leaving window.
- *Complexity:* O(n) time, O(1) space.

**🟡 Variable-size Window (Shrinkable)**
- *Problem type:* Smallest subarray with sum ≥ target, longest substring without repeating characters.
- *Approach:* Expand right pointer; while condition violated/satisfied, shrink left pointer; track best result.
- *Complexity:* O(n) time (each pointer moves ≤ n times), O(k) space for window state.

**🔴 Longest Substring with At Most K Distinct Characters**
- *Problem type:* Character frequency window problems, minimum window substring.
- *Approach:* HashMap of char frequencies inside window; shrink when distinct count > k or when all target chars satisfied.
- *Complexity:* O(n) time, O(k) space.

**🔴 Sliding Window Maximum**
- *Problem type:* Max/min in every window of size K.
- *Approach:* Monotonic deque storing indices in decreasing value order; pop from front when out of window, pop from back when smaller than new element.
- *Complexity:* O(n) time (amortized), O(k) space.

**⚫ Minimum Window Substring with Multiple Constraints**
- *Problem type:* Minimum window covering all characters of another string with counts, substring with concatenation of all words.
- *Approach:* Two hashmaps (need vs window), track `formed` count of satisfied unique chars; shrink while still valid.
- *Complexity:* O(n + m) time, O(m) space.

---

## 4. Hashing

### Patterns

**🟢 Frequency Counting**
- *Problem type:* Anagram check, first unique character, majority element (Boyer-Moore alternative).
- *Approach:* HashMap/array counts of elements; compare or query.
- *Complexity:* O(n) time, O(k) space (k = alphabet/key size).

**🟢 Two Sum**
- *Problem type:* Find pair summing to target.
- *Approach:* Single pass, store `value → index` in hashmap, check `target - value` exists.
- *Complexity:* O(n) time, O(n) space.

**🟡 Grouping (Anagrams, Isomorphic Strings)**
- *Problem type:* Group anagrams, group by pattern.
- *Approach:* Use a canonical key (sorted string or char-count signature) as hashmap key, append to bucket.
- *Complexity:* O(n·k log k) time (k = avg string length), O(n·k) space.

**🟡 Subarray Sum Equals K (Prefix Sum + Hashing)**
- *Problem type:* Count subarrays with sum K, longest subarray with equal 0s/1s.
- *Approach:* Store prefix sum frequency in hashmap; at each step check if `prefixSum - K` exists.
- *Complexity:* O(n) time, O(n) space.

**🔴 Longest Consecutive Sequence**
- *Problem type:* Longest run of consecutive integers.
- *Approach:* Put all numbers in a hashset; only start counting from numbers whose `num-1` is absent (sequence start); expand forward.
- *Complexity:* O(n) time, O(n) space.

**🔴 LRU / LFU Cache (Hashing + Linked List)**
- *Problem type:* Design LRU cache.
- *Approach:* HashMap (key → node) + doubly linked list for O(1) get/put with recency ordering. LFU adds a frequency-bucket structure.
- *Complexity:* O(1) time per operation, O(n) space.

---

## 5. Sorting

### Patterns

**🟢 Comparison Sorts**
- *Problem type:* Sort an array, custom comparator sorting.
- *Approach:* Merge sort (divide & conquer, stable), Quick sort (partition, in-place), Heap sort.
- *Complexity:* Merge sort O(n log n) time / O(n) space; Quick sort O(n log n) avg / O(n²) worst / O(log n) space; Heap sort O(n log n) / O(1) space.

**🟢 Non-Comparison Sorts**
- *Problem type:* Sort integers in known range, sort large datasets of fixed-width keys.
- *Approach:* Counting sort (frequency array), Bucket sort, Radix sort (digit by digit).
- *Complexity:* Counting/Radix O(n + k) time, O(n + k) space.

**🟡 Merge Intervals**
- *Problem type:* Merge overlapping intervals, insert interval, meeting rooms.
- *Approach:* Sort by start time, iterate and merge when `current.start ≤ prev.end`.
- *Complexity:* O(n log n) time, O(n) space.

**🟡 K Closest / Sort by Custom Key**
- *Problem type:* Sort array by frequency, largest number formed by array.
- *Approach:* Custom comparator (e.g., `(a+b) vs (b+a)` string comparison for largest number).
- *Complexity:* O(n log n) time.

**🔴 Merge Sort Applications**
- *Problem type:* Count inversions, count of smaller numbers after self.
- *Approach:* Modify merge step to count cross-inversions while merging two sorted halves.
- *Complexity:* O(n log n) time, O(n) space.

**🔴 Meeting Rooms II (Interval + Heap)**
- *Problem type:* Minimum meeting rooms required.
- *Approach:* Sort start times; min-heap of end times; if earliest end ≤ current start, reuse room, else allocate new.
- *Complexity:* O(n log n) time, O(n) space.

---

## 6. Binary Search

### Patterns

**🟢 Classic Search**
- *Problem type:* Search in sorted array, first/last occurrence.
- *Approach:* Standard `lo/hi/mid` binary search; for first/last occurrence, continue narrowing after a match instead of returning immediately.
- *Complexity:* O(log n) time, O(1) space.

**🟡 Search in Rotated Sorted Array**
- *Problem type:* Search in rotated sorted array (with/without duplicates).
- *Approach:* At each mid, determine which half is sorted, then decide which half to discard based on target's range.
- *Complexity:* O(log n) time (O(n) worst case with duplicates), O(1) space.

**🟡 Binary Search on Answer**
- *Problem type:* Koko eating bananas, capacity to ship packages in D days, minimum days to make m bouquets.
- *Approach:* Binary search over the *answer space* (not the array); write a `feasible(x)` check function (usually O(n)), binary search for smallest/largest feasible x.
- *Complexity:* O(n log(range)) time, O(1) space.

**🔴 Median of Two Sorted Arrays / K-th Element Across Arrays**
- *Problem type:* Median of two sorted arrays, kth smallest element in two sorted arrays.
- *Approach:* Binary search on partition index of the smaller array to balance left/right halves across both arrays.
- *Complexity:* O(log(min(n,m))) time, O(1) space.

**🔴 Binary Search on 2D Matrix**
- *Problem type:* Search a 2D matrix (row & column sorted).
- *Approach:* Treat matrix as flattened 1D array with index mapping `mid → (mid/cols, mid%cols)`, OR start from top-right corner and eliminate row/col each step.
- *Complexity:* O(log(n·m)) or O(n + m) depending on approach.

**⚫ Split Array Largest Sum / Aggressive Cows**
- *Problem type:* Minimize the maximum subarray sum after splitting into k parts; place cows to maximize minimum distance.
- *Approach:* Binary search on answer with greedy feasibility check counting partitions/placements.
- *Complexity:* O(n log(sum)) time.

---

## 7. Recursion & Backtracking

### Patterns

**🟢 Basic Recursion**
- *Problem type:* Factorial, power, sum of digits, reverse a string.
- *Approach:* Define base case + recursive relation.
- *Complexity:* O(n) time, O(n) space (call stack).

**🟡 Subsets / Combinations**
- *Problem type:* Subsets, combinations, letter combinations of a phone number.
- *Approach:* At each index, branch into "include" and "exclude"; build via DFS + backtrack (undo choice after recursive call).
- *Complexity:* O(2ⁿ) time for subsets, O(n·2ⁿ) with copying; O(n) recursion depth.

**🟡 Permutations**
- *Problem type:* Generate all permutations.
- *Approach:* Swap-based in-place generation or "used[]" boolean array marking visited elements.
- *Complexity:* O(n! · n) time, O(n) space.

**🔴 Constraint Satisfaction Backtracking**
- *Problem type:* N-Queens, Sudoku solver, word search.
- *Approach:* DFS placing one unit at a time, prune branches violating constraints early (pruning is critical for performance), backtrack on failure.
- *Complexity:* Exponential worst case, e.g., N-Queens ~O(n!), pruning drastically reduces practical runtime; space O(n) recursion + O(n²) board.

**🔴 Partitioning Problems**
- *Problem type:* Palindrome partitioning, combination sum (with repetition).
- *Approach:* DFS trying every valid cut/element at current position, backtrack, use memoized "is palindrome" table to speed checks.
- *Complexity:* O(2ⁿ) time worst case, O(n) space.

**⚫ Backtracking + Memoization Hybrid**
- *Problem type:* Word break II (all sentences), expression add operators.
- *Approach:* Backtracking to enumerate, memoize sub-results (map from index/state → list of valid results) to avoid recomputation across overlapping subproblems.
- *Complexity:* Exponential in worst case but memoization cuts redundant subtree exploration significantly.

---

## 8. Linked List

### Patterns

**🟢 Traversal / Basic Manipulation**
- *Problem type:* Reverse a linked list, find middle, delete a node.
- *Approach:* Iterative pointer rewiring (`prev, curr, next`) or recursive reversal.
- *Complexity:* O(n) time, O(1) space (iterative) / O(n) space (recursive).

**🟡 Fast & Slow Pointers**
- *Problem type:* Detect cycle, find cycle start, find middle, palindrome linked list.
- *Approach:* Floyd's algorithm; for cycle start, after meeting point reset one pointer to head, move both 1 step until they meet again.
- *Complexity:* O(n) time, O(1) space.

**🟡 Merge / Reorder**
- *Problem type:* Merge two sorted lists, merge K sorted lists, reorder list, add two numbers.
- *Approach:* Merge two — dummy node + pick smaller. Merge K — min-heap of K heads or divide & conquer pairwise merging.
- *Complexity:* Merge two O(n+m); Merge K lists O(N log k) with heap (N total nodes, k lists); O(k) space.

**🔴 Reverse in Groups / Complex Rewiring**
- *Problem type:* Reverse nodes in k-group, rotate list, swap pairs.
- *Approach:* Reverse sublist of size k iteratively with careful boundary pointer tracking, recurse/iterate for next group.
- *Complexity:* O(n) time, O(1) space (iterative).

**🔴 Copy List with Random Pointer**
- *Problem type:* Deep copy a linked list with random pointers.
- *Approach:* HashMap old→new node mapping in first pass, wire next/random in second pass. (Or O(1) space trick: interleave cloned nodes then split.)
- *Complexity:* O(n) time, O(n) space (or O(1) with interleaving trick).

**⚫ LRU Cache via Linked List + Hashmap**
- *Problem type:* Design LRU/LFU cache.
- *Approach:* Doubly linked list maintains recency order; hashmap gives O(1) node access; move-to-front on access, evict from tail on capacity overflow.
- *Complexity:* O(1) time per operation, O(capacity) space.

---

## 9. Stack

### Patterns

**🟢 Valid Parentheses / Matching**
- *Problem type:* Valid parentheses, remove outermost parentheses.
- *Approach:* Push opening brackets; on closing bracket, pop and check match.
- *Complexity:* O(n) time, O(n) space.

**🟡 Monotonic Stack**
- *Problem type:* Next greater element, daily temperatures, largest rectangle in histogram.
- *Approach:* Maintain stack of indices in increasing/decreasing value order; pop while current breaks monotonic property, computing the answer at pop time.
- *Complexity:* O(n) time (each element pushed/popped once), O(n) space.

**🟡 Evaluate Expressions**
- *Problem type:* Evaluate RPN, basic calculator, infix to postfix.
- *Approach:* Use stack for operands/operators; handle precedence with a precedence map for shunting-yard style conversion.
- *Complexity:* O(n) time, O(n) space.

**🔴 Largest Rectangle in Histogram / Maximal Rectangle**
- *Problem type:* Largest rectangle area, maximal rectangle in binary matrix.
- *Approach:* Monotonic increasing stack of bar indices; when a smaller bar appears, pop and compute area using popped bar as height and current index minus new stack top as width. Maximal rectangle = apply histogram technique per row.
- *Complexity:* O(n) per row, O(n·m) total for matrix version.

**🔴 Min Stack / Stack with getMin O(1)**
- *Problem type:* Design a stack supporting push, pop, top, getMin in O(1).
- *Approach:* Auxiliary stack tracking running minimum, or store (value, currentMin) pairs.
- *Complexity:* O(1) time per operation, O(n) space.

**⚫ Basic Calculator III (Nested Expressions)**
- *Problem type:* Expression evaluation with nested parentheses, +,-,*,/ and precedence.
- *Approach:* Recursive descent parsing using a stack, recursing into sub-expressions on `(`.
- *Complexity:* O(n) time, O(n) space (recursion + stack).

---

## 10. Queue & Monotonic Deque

### Patterns

**🟢 Basic Queue Operations**
- *Problem type:* Implement queue using stacks, circular queue.
- *Approach:* Two stacks (in-stack, out-stack); transfer elements when out-stack empty for amortized O(1) dequeue.
- *Complexity:* Amortized O(1) per operation.

**🟡 BFS Traversal Queue**
- *Problem type:* Level order traversal, shortest path in unweighted graph.
- *Approach:* Standard queue-based BFS, process level by level using queue-size snapshot.
- *Complexity:* O(V + E) time, O(V) space.

**🔴 Monotonic Deque (Sliding Window Max)**
- *Problem type:* Sliding window maximum, shortest subarray with sum at least K.
- *Approach:* Deque holding indices; maintain increasing/decreasing order by popping from back before pushing; pop from front when index out of window.
- *Complexity:* O(n) time, O(k) space.

**🔴 Design Circular Deque / Task Scheduler**
- *Problem type:* Task scheduler with cooldown.
- *Approach:* Max-heap for task frequency + queue to track cooldown release times.
- *Complexity:* O(n log 26) time ≈ O(n), O(1) extra space (bounded alphabet).

---

## 11. Trees

(Binary Trees, BST, N-ary Trees)

### Patterns

**🟢 Traversals**
- *Problem type:* Inorder, preorder, postorder, level order.
- *Approach:* Recursive DFS or iterative with explicit stack; level order uses a queue (BFS).
- *Complexity:* O(n) time, O(h) space recursive (h = height) or O(n) for BFS queue.

**🟢 Height / Depth / Basic Properties**
- *Problem type:* Max depth, balanced tree check, diameter of tree.
- *Approach:* Bottom-up recursion returning height, compute diameter as `leftHeight + rightHeight` at each node while returning height upward.
- *Complexity:* O(n) time, O(h) space.

**🟡 BST Operations**
- *Problem type:* Insert/delete/search in BST, validate BST, kth smallest in BST.
- *Approach:* Exploit BST ordering property to prune half the tree at each step; inorder traversal gives sorted order.
- *Complexity:* O(h) time for search/insert (O(log n) balanced, O(n) skewed), O(n) for full traversal.

**🟡 Lowest Common Ancestor**
- *Problem type:* LCA in binary tree / BST.
- *Approach:* BST — use ordering to go left/right. General tree — recursive DFS returning node if found in both subtrees.
- *Complexity:* O(n) time (O(h) for BST), O(h) space.

**🔴 Path Sum Variants**
- *Problem type:* Path sum II, path sum III (any start/end), binary tree maximum path sum.
- *Approach:* DFS with backtracking for explicit paths; prefix-sum hashmap technique for path sum III (count paths ending at any node); "max path through node" combining left+right gains for max path sum.
- *Complexity:* O(n) time, O(h) space (O(n) with prefix-sum hashmap).

**🔴 Serialize/Deserialize & Tree Construction**
- *Problem type:* Serialize/deserialize binary tree, construct tree from preorder+inorder.
- *Approach:* Preorder DFS with null markers for serialization; recursive construction using index mapping (hashmap of value→index in inorder) for O(1) root lookup.
- *Complexity:* O(n) time, O(n) space.

**⚫ Segment/Tree DP (Binary Tree Cameras, House Robber III)**
- *Problem type:* Minimum cameras to cover tree, max sum with no two adjacent nodes selected.
- *Approach:* Post-order DFS returning multiple states per node (e.g., "covered/not covered/has camera", or "include/exclude" sums), combine at parent.
- *Complexity:* O(n) time, O(h) space.

**⚫ Morris Traversal (O(1) Space)**
- *Problem type:* Inorder traversal without recursion or stack.
- *Approach:* Temporarily thread the tree — link predecessor's right pointer to current node, traverse, then restore.
- *Complexity:* O(n) time, O(1) space (a rare true-constant-space tree traversal).

---

## 12. Heap / Priority Queue

### Patterns

**🟢 Kth Largest/Smallest**
- *Problem type:* Kth largest element in array/stream.
- *Approach:* Min-heap of size k (for kth largest) — push, pop when size > k, top is the answer.
- *Complexity:* O(n log k) time, O(k) space.

**🟡 Top K Frequent Elements**
- *Problem type:* Top K frequent words/elements, sort by frequency.
- *Approach:* Frequency hashmap + heap of size k, OR bucket sort by frequency for O(n).
- *Complexity:* O(n log k) with heap, O(n) with bucket sort; O(n) space.

**🟡 Merge K Sorted Lists/Arrays**
- *Problem type:* Merge k sorted lists, smallest range covering elements from k lists.
- *Approach:* Min-heap holding one element per list with (value, listIndex, elemIndex); pop min, push next from same list.
- *Complexity:* O(N log k) time (N total elements), O(k) space.

**🔴 Two Heaps (Median Finder)**
- *Problem type:* Find median from data stream.
- *Approach:* Max-heap for lower half, min-heap for upper half; balance sizes so they differ by ≤1; median from heap tops.
- *Complexity:* O(log n) insert, O(1) find median; O(n) space.

**🔴 Task Scheduling / Greedy with Heap**
- *Problem type:* Task scheduler, meeting rooms II, IPO (maximize capital).
- *Approach:* Max-heap for greedy selection by priority/profit, combined with sorting for feasibility ordering.
- *Complexity:* O(n log n) time, O(n) space.

**⚫ K-way Merge with Custom Constraints**
- *Problem type:* Find kth smallest pair distance, kth smallest in matrix with sorted rows/cols.
- *Approach:* Heap-based k-way merge OR binary search on answer value combined with counting function — often the binary-search-on-answer variant beats the heap approach asymptotically.
- *Complexity:* Heap: O(k log n); Binary search on value: O(n log(range)).

---

## 13. Graphs

The richest topic — spans traversal, shortest paths, MST, topological ordering, and advanced flow problems.

### Patterns

**🟢 BFS / DFS Traversal**
- *Problem type:* Number of islands, flood fill, connected components.
- *Approach:* Standard BFS (queue) or DFS (recursion/stack), mark visited to avoid revisits.
- *Complexity:* O(V + E) time, O(V) space.

**🟡 Topological Sort**
- *Problem type:* Course schedule, task ordering with dependencies, alien dictionary.
- *Approach:* Kahn's algorithm (BFS using in-degree array) or DFS post-order with reversal.
- *Complexity:* O(V + E) time, O(V) space.

**🟡 Cycle Detection**
- *Problem type:* Detect cycle in directed/undirected graph.
- *Approach:* Directed — DFS with recursion-stack tracking (3-color marking) or Kahn's algorithm (if not all nodes processed, cycle exists). Undirected — DFS/BFS with parent tracking, or Union-Find.
- *Complexity:* O(V + E) time, O(V) space.

**🔴 Shortest Path (Unweighted & Weighted)**
- *Problem type:* Shortest path in weighted graph (Dijkstra), graphs with negative weights (Bellman-Ford), all-pairs shortest path (Floyd-Warshall).
- *Approach:* Dijkstra — min-heap greedy relaxation (no negative weights). Bellman-Ford — relax all edges V-1 times (handles negative weights, detects negative cycles). Floyd-Warshall — DP over all triplets of nodes.
- *Complexity:* Dijkstra O((V+E) log V); Bellman-Ford O(V·E); Floyd-Warshall O(V³); space O(V²) for Floyd-Warshall, O(V) for others.

**🔴 Minimum Spanning Tree**
- *Problem type:* Connect all nodes with minimum total edge weight.
- *Approach:* Kruskal's (sort edges, union-find to avoid cycles) or Prim's (min-heap growing tree from a start node).
- *Complexity:* Kruskal O(E log E); Prim O(E log V); space O(V + E).

**🔴 Union-Find Based Graph Problems**
- *Problem type:* Number of provinces, redundant connection, accounts merge.
- *Approach:* Union-Find with path compression + union by rank/size; union nodes belonging to the same group, count distinct roots.
- *Complexity:* O(E · α(V)) ≈ O(E) time (α = inverse Ackermann, effectively constant), O(V) space.

**⚫ Advanced Graph — Bridges, Articulation Points, SCC**
- *Problem type:* Critical connections in a network (bridges), articulation points, strongly connected components.
- *Approach:* Tarjan's algorithm using DFS discovery time and low-link values; SCC via Tarjan's or Kosaraju's (two-pass DFS with graph transpose).
- *Complexity:* O(V + E) time, O(V) space.

**⚫ Network Flow**
- *Problem type:* Maximum flow, minimum cut, bipartite matching.
- *Approach:* Ford-Fulkerson/Edmonds-Karp (BFS-based augmenting paths), Dinic's algorithm for better complexity on large graphs.
- *Complexity:* Edmonds-Karp O(V·E²); Dinic's O(V²·E) general, O(E√V) for unit-capacity bipartite graphs.

**⚫ Word Ladder / Bidirectional BFS**
- *Problem type:* Word ladder (shortest transformation sequence), bidirectional search optimization.
- *Approach:* BFS from both start and end simultaneously, meeting in the middle to cut search space roughly in half exponentially.
- *Complexity:* O(V + E) worst case but practically much faster than one-directional BFS on large graphs.

---

## 14. Dynamic Programming

The most pattern-rich topic — organized by the *shape* of the recurrence.

### Patterns

**🟢 1D DP (Linear Recurrence)**
- *Problem type:* Climbing stairs, house robber, fibonacci.
- *Approach:* `dp[i] = f(dp[i-1], dp[i-2], ...)`. Can be space-optimized to O(1) by keeping only last few states.
- *Complexity:* O(n) time, O(1) space (optimized) or O(n) space.

**🟡 Knapsack Family**
- *Problem type:* 0/1 knapsack, subset sum, partition equal subset sum, coin change (min coins / ways).
- *Approach:* `dp[i][w] = max/count combining "take item" and "skip item"`. 0/1 knapsack iterates items outer, capacity inner (reverse order for 1D optimization); unbounded knapsack (coin change) iterates capacity outer for reuse.
- *Complexity:* O(n·W) time, O(W) space (1D optimized), W = capacity/target.

**🟡 Longest Common Subsequence Family**
- *Problem type:* LCS, edit distance, longest palindromic subsequence, distinct subsequences.
- *Approach:* 2D `dp[i][j]` comparing two strings/sequences; match → `dp[i-1][j-1]+1`; mismatch → take max/min of neighbors depending on problem (insert/delete/replace for edit distance).
- *Complexity:* O(n·m) time, O(n·m) space (reducible to O(min(n,m)) with rolling array).

**🟡 Longest Increasing Subsequence**
- *Problem type:* LIS, maximum envelopes, box stacking.
- *Approach:* Naive `dp[i] = 1 + max(dp[j]) for j<i, arr[j]<arr[i]` O(n²); optimized — maintain a "tails" array with binary search for patience sorting.
- *Complexity:* O(n²) naive; O(n log n) optimized; O(n) space.

**🔴 Interval DP**
- *Problem type:* Matrix chain multiplication, burst balloons, palindrome partitioning min cuts.
- *Approach:* `dp[i][j]` represents best answer for subarray/substring [i,j]; iterate by increasing interval length, try every split point k.
- *Complexity:* O(n³) time (n² states × n split points), O(n²) space.

**🔴 DP on Grids**
- *Problem type:* Unique paths, minimum path sum, dungeon game, cherry pickup.
- *Approach:* `dp[i][j]` from top/left neighbors (or bottom/right for reverse problems); multi-pointer DP for problems needing two simultaneous paths (cherry pickup).
- *Complexity:* O(n·m) time (O(n³) for dual-path variants), O(n·m) space.

**🔴 Bitmask DP**
- *Problem type:* Traveling salesman problem, assign tasks to workers, shortest path visiting all nodes.
- *Approach:* State = `dp[mask][i]` where mask represents the subset of visited nodes/tasks; transition by adding one more bit to mask.
- *Complexity:* O(n² · 2ⁿ) time, O(n · 2ⁿ) space.

**🔴 Digit DP**
- *Problem type:* Count numbers in range with a digit property (e.g., no repeated digits, digit sum constraint).
- *Approach:* DP over digit positions with state (position, tight-bound flag, other constraint state), process digit by digit.
- *Complexity:* O(d · states) where d = number of digits, typically very fast.

**⚫ Tree DP**
- *Problem type:* House robber III, binary tree cameras, diameter with weighted edges.
- *Approach:* Post-order DFS returning multiple states per subtree (e.g., include/exclude root), combine at parent — this is DP over the tree structure instead of a linear/2D array.
- *Complexity:* O(n) time, O(h) space.

**⚫ DP + Data Structure Optimization**
- *Problem type:* LIS in O(n log n), maximum sum with constraints using segment tree/monotonic deque optimization, DP with convex-hull trick.
- *Approach:* Replace naive O(n) inner-loop DP transitions with O(log n) segment tree / Fenwick tree queries, or monotonic deque for sliding-window DP optimization.
- *Complexity:* Reduces O(n²) DP to O(n log n); space O(n).

---

## 15. Greedy

### Patterns

**🟢 Interval Scheduling**
- *Problem type:* Activity selection, non-overlapping intervals, minimum arrows to burst balloons.
- *Approach:* Sort by end time, greedily pick earliest-ending compatible interval.
- *Complexity:* O(n log n) time, O(1) extra space.

**🟡 Jump Game / Reachability**
- *Problem type:* Jump game I/II, gas station.
- *Approach:* Track farthest reachable index while scanning; for min jumps, use a level-by-level BFS-like greedy expansion.
- *Complexity:* O(n) time, O(1) space.

**🟡 Greedy + Sorting**
- *Problem type:* Assign cookies, task scheduler, minimum platforms.
- *Approach:* Sort one or both arrays, greedily match/assign in order to satisfy the maximum count of constraints.
- *Complexity:* O(n log n) time, O(1) space.

**🔴 Exchange Argument Problems**
- *Problem type:* Job sequencing with deadlines, minimize sum of products, candy distribution.
- *Approach:* Prove greedy choice via exchange argument (swapping any two elements out of greedy order doesn't improve the answer); typically combined with sorting + priority queue.
- *Complexity:* O(n log n) time, O(n) space.

**⚫ Greedy with Proof via Matroid/Exchange (Huffman Coding)**
- *Problem type:* Huffman encoding, minimum cost to connect ropes.
- *Approach:* Min-heap — repeatedly combine two smallest elements, push sum back; correctness proven via exchange argument / matroid theory.
- *Complexity:* O(n log n) time, O(n) space.

---

## 16. Trie

### Patterns

**🟢 Basic Insert/Search**
- *Problem type:* Implement Trie, word search prefix check.
- *Approach:* Tree of characters, each node has up to 26 children + `isEndOfWord` flag.
- *Complexity:* O(L) time per insert/search (L = word length), O(N·L) space (N words).

**🟡 Prefix-based Queries**
- *Problem type:* Autocomplete, longest common prefix, count words with given prefix.
- *Approach:* Traverse trie along prefix path, then DFS from that node to collect matches or count subtree.
- *Complexity:* O(L + results) time.

**🔴 Word Search II (Trie + Backtracking)**
- *Problem type:* Find all words from a dictionary present in a grid.
- *Approach:* Build trie of all dictionary words, DFS/backtrack through grid, pruning branches where no trie path exists.
- *Complexity:* O(N·M·4^L) worst-case bounded heavily by trie pruning in practice; O(N·L_total) trie space.

**⚫ XOR Trie (Bitwise Trie)**
- *Problem type:* Maximum XOR of two numbers in array, maximum XOR with an element from array (queries).
- *Approach:* Build a binary trie over bit representations (MSB to LSB); for each query, greedily walk the trie choosing the opposite bit at each level to maximize XOR.
- *Complexity:* O(n·B) time (B = bit width, typically 32), O(n·B) space.

---

## 17. Union-Find

(Disjoint Set Union)

### Patterns

**🟢 Basic Connectivity**
- *Problem type:* Number of connected components, friend circles.
- *Approach:* `find()` with path compression, `union()` by rank/size.
- *Complexity:* O(α(n)) ≈ O(1) amortized per operation, O(n) space.

**🟡 Cycle Detection in Graph**
- *Problem type:* Redundant connection, detect cycle in undirected graph.
- *Approach:* Union nodes of each edge; if two nodes already share a root, adding that edge creates a cycle.
- *Complexity:* O(E · α(V)) time.

**🔴 Union-Find with Weighted/Ranked Union**
- *Problem type:* Accounts merge, number of islands II (dynamic connectivity), satisfiability of equality equations.
- *Approach:* Union-Find with additional metadata stored per component (e.g., merged account emails); process operations incrementally, merging components on the fly.
- *Complexity:* O(Q · α(n)) for Q operations.

**⚫ Union-Find on Implicit Graphs**
- *Problem type:* Smallest string with swaps, evaluate division (graph + union-find hybrid), most stones removed.
- *Approach:* Map problem elements to DSU nodes creatively (e.g., row+col as combined index for grid problems), union based on relationships, then aggregate per component.
- *Complexity:* O(n · α(n)) time typically.

---

## 18. Segment Tree / Fenwick Tree

### Patterns

**🟡 Range Sum / Range Min-Max Query with Point Updates**
- *Problem type:* Range sum query mutable, range minimum query.
- *Approach:* Build a segment tree where each node stores an aggregate (sum/min/max) of its range; update propagates up O(log n), query combines O(log n) relevant nodes. Fenwick tree (BIT) is a simpler/faster alternative for prefix sums specifically.
- *Complexity:* Build O(n), Update O(log n), Query O(log n); space O(n) (O(4n) for segment tree array).

**🔴 Range Update + Range Query (Lazy Propagation)**
- *Problem type:* Range add, range sum query; range assign updates.
- *Approach:* Segment tree with lazy propagation — defer updates to children, push down lazily only when needed.
- *Complexity:* O(log n) per update/query, O(n) space.

**🔴 Fenwick Tree for Inversions / Order Statistics**
- *Problem type:* Count of smaller numbers after self, count inversions.
- *Approach:* Coordinate-compress values, use BIT indexed by compressed value to count elements seen so far less than current.
- *Complexity:* O(n log n) time, O(n) space.

**⚫ 2D Segment Tree / BIT**
- *Problem type:* 2D range sum query mutable, count points in rectangle.
- *Approach:* Nested BIT/segment tree (tree of trees) or offline processing with sorting + 1D BIT sweep.
- *Complexity:* O(log n · log m) per operation for 2D BIT; O(n·m) space worst case.

**⚫ Merge Sort Tree / Persistent Segment Tree**
- *Problem type:* Kth smallest in range query, count elements less than X in range [l,r] (offline/online), persistent version history queries.
- *Approach:* Each segment tree node stores a sorted list of its range (merge sort tree) enabling binary search per node; persistent segment tree creates new nodes only along the update path, preserving previous versions.
- *Complexity:* Merge sort tree O(log²n) per query, O(n log n) space; persistent segment tree O(log n) per update creating a new version, O(n log n) total space across versions.

---

## 19. String Algorithms

### Patterns

**🟢 Basic String Manipulation**
- *Problem type:* Reverse words, string compression, palindrome check.
- *Approach:* Two-pointer or direct character array manipulation.
- *Complexity:* O(n) time, O(1)–O(n) space.

**🟡 Pattern Matching — Naive vs Optimized**
- *Problem type:* Find substring (needle in haystack).
- *Approach:* Naive O(n·m) sliding comparison; better — KMP (prefix function to avoid re-scanning), Rabin-Karp (rolling hash).
- *Complexity:* KMP O(n + m) time, O(m) space; Rabin-Karp O(n + m) average, O(n·m) worst case (hash collisions).

**🔴 Z-Algorithm / Manacher's Algorithm**
- *Problem type:* Longest palindromic substring, string matching with Z-array, count palindromic substrings.
- *Approach:* Z-algorithm computes, for each position, length of longest substring matching the prefix — useful for pattern matching in O(n). Manacher's algorithm finds longest palindromic substring in linear time by exploiting palindrome symmetry.
- *Complexity:* O(n) time, O(n) space.

**🔴 Suffix Array / Suffix Automaton (concept level)**
- *Problem type:* Longest repeated substring, longest common substring across multiple strings.
- *Approach:* Build suffix array (sorted suffixes) + LCP (longest common prefix) array via O(n log n) construction (or O(n) with advanced algorithms like SA-IS); binary search / sliding window over LCP array for answers.
- *Complexity:* O(n log n) build, O(n) or O(log n) per query depending on structure.

**⚫ String DP + Automaton Hybrid**
- *Problem type:* Regular expression matching, wildcard matching, distinct subsequences II.
- *Approach:* 2D DP `dp[i][j]` representing match state between string and pattern prefixes, handling `*` and `.` transition rules carefully.
- *Complexity:* O(n·m) time, O(n·m) space (reducible to O(m) rolling row).

---

## 20. Bit Manipulation

### Patterns

**🟢 Basic Bit Tricks**
- *Problem type:* Count set bits, check power of two, single number (XOR).
- *Approach:* `n & (n-1)` clears lowest set bit; XOR cancels pairs (`a ^ a = 0`) to find unique elements.
- *Complexity:* O(1) or O(log(max value)) time, O(1) space.

**🟡 Subsets via Bitmask**
- *Problem type:* Generate all subsets, subset sum enumeration.
- *Approach:* Iterate `mask` from 0 to 2ⁿ-1; each bit represents inclusion/exclusion of an element.
- *Complexity:* O(n · 2ⁿ) time, O(1) extra space (excluding output).

**🟡 Single Number II/III (Multiple Unique Elements)**
- *Problem type:* Find element appearing once when others appear twice/thrice, find two unique elements.
- *Approach:* Bit-counting per position mod k (for "others appear k times"), or XOR + partition by distinguishing bit (for exactly-two-unique-elements case).
- *Complexity:* O(n) time, O(1) space.

**🔴 Bitmask DP**
- *Problem type:* Traveling salesman, assignment problems.
- *Approach:* See DP section — state includes a bitmask representing a subset.
- *Complexity:* O(n² · 2ⁿ) time.

**🔴 XOR Trie / Maximum XOR Pair**
- *Problem type:* Maximum XOR of two numbers.
- *Approach:* See Trie section — greedy bit-by-bit trie traversal.
- *Complexity:* O(n · 32) time, O(n · 32) space.

**⚫ Bit DP with State Compression on Grids**
- *Problem type:* Count ways to tile a board (domino/tromino tiling), broken profile DP.
- *Approach:* Represent each column/row's filled state as a bitmask, DP transition processes cell-by-cell updating the profile mask.
- *Complexity:* O(rows · 2^cols · transitions) time; exponential in the smaller dimension, so orient the grid to minimize it.

---

## 21. Math & Number Theory

### Patterns

**🟢 GCD / LCM / Basic Number Properties**
- *Problem type:* GCD of two numbers, check prime, count divisors.
- *Approach:* Euclidean algorithm for GCD; trial division up to √n for primality/divisors.
- *Complexity:* GCD O(log(min(a,b))); primality check O(√n).

**🟡 Sieve of Eratosthenes**
- *Problem type:* Count primes up to n, prime factorization for many queries.
- *Approach:* Mark composites starting from each prime's square; optionally precompute smallest prime factor (SPF) for O(log n) factorization per query.
- *Complexity:* O(n log log n) time, O(n) space.

**🟡 Modular Arithmetic**
- *Problem type:* Large number power/factorial mod p, combinatorics mod p.
- *Approach:* Fast exponentiation (binary exponentiation), modular inverse via Fermat's little theorem (when p is prime) for division under modulo.
- *Complexity:* O(log n) for fast power, O(n) precompute for factorials mod p.

**🔴 Combinatorics**
- *Problem type:* nCr mod p for many queries, Catalan numbers (valid parentheses count, unique BSTs).
- *Approach:* Precompute factorials + modular inverse factorials for O(1) nCr queries; Catalan number recurrence or direct formula `C(2n,n)/(n+1)`.
- *Complexity:* O(n) precompute, O(1) per query.

**🔴 Matrix Exponentiation**
- *Problem type:* Nth Fibonacci in O(log n), linear recurrence relations at large n.
- *Approach:* Represent recurrence as a matrix, raise to the Nth power using fast exponentiation.
- *Complexity:* O(k³ log n) time (k = matrix dimension/recurrence order), O(k²) space.

**⚫ Advanced Number Theory**
- *Problem type:* Chinese remainder theorem, discrete log, primitive roots, FFT-based big integer multiplication.
- *Approach:* CRT combines modular equations with pairwise coprime moduli; FFT/NTT convert coefficient-domain polynomial multiplication to point-domain for O(n log n) multiplication instead of O(n²).
- *Complexity:* CRT O(k log M); FFT-based multiplication O(n log n).

---

## 22. Divide and Conquer

### Patterns

**🟢 Classic D&C**
- *Problem type:* Merge sort, quick sort, binary search.
- *Approach:* Split problem into independent subproblems, solve recursively, combine results.
- *Complexity:* O(n log n) typical (governed by Master Theorem).

**🟡 Divide and Conquer on Arrays**
- *Problem type:* Maximum subarray (D&C version), count inversions, closest pair of points.
- *Approach:* Split array in half, solve each half recursively, handle the "crossing" case explicitly, combine.
- *Complexity:* O(n log n) time typically.

**🔴 Quickselect**
- *Problem type:* Kth largest element, median of unsorted array.
- *Approach:* Like quicksort but recurse only into the partition containing the target index; random pivot for good average performance.
- *Complexity:* O(n) average, O(n²) worst case, O(1) space.

**⚫ D&C with Complex Combine Step**
- *Problem type:* Closest pair of points (geometry), skyline problem, count of range sum.
- *Approach:* Non-trivial merge/combine logic (e.g., strip-based checking near the dividing line for closest pair, merging skylines from two halves).
- *Complexity:* O(n log n) time with a well-designed O(n) combine step, O(n) space.

---

## 23. Design Problems

(Often combine multiple data structures — frequently asked as "hard" system-style DSA questions)

### Patterns

**🟡 Design HashMap / HashSet from Scratch**
- *Approach:* Array of buckets (linked lists) + hash function; resize/rehash when load factor exceeds threshold.
- *Complexity:* O(1) average per operation, O(n) worst case (many collisions).

**🟡 Design a Stack/Queue with Extra Operations**
- *Problem type:* Min stack, max queue, stack using queues.
- *Approach:* Auxiliary structure tracking the needed extra property alongside the primary structure.
- *Complexity:* O(1) amortized per operation typically.

**🔴 LRU / LFU Cache**
- *Approach:* See Hashing & Linked List sections — hashmap + doubly linked list (LRU) or hashmap + frequency buckets of doubly linked lists (LFU).
- *Complexity:* O(1) time per operation, O(capacity) space.

**🔴 Design Twitter / Design Search Autocomplete**
- *Approach:* Combine hashmap (user→tweets/follows) + heap (merge k most-recent feeds) for Twitter; trie + heap/sorted list for autocomplete ranking by frequency.
- *Complexity:* Feed generation O(k log u) (u = followed users); autocomplete O(p + m log m) (p = prefix length, m = matches).

**⚫ Design Rate Limiter / Design File System / Design In-Memory DB**
- *Approach:* Combine trees/tries (hierarchical paths) with hashmaps for O(1) lookups, sliding-window or token-bucket counters for rate limiting, careful concurrency-safe design in some variants.
- *Complexity:* Varies by design; typically O(log n) to O(1) per operation with O(n) space for stored state.

---

## 🗺️ Suggested Learning Order

```
Arrays/Strings → Two Pointers → Sliding Window → Hashing → Sorting
      ↓
Binary Search → Recursion/Backtracking → Linked List
      ↓
Stack → Queue/Deque → Trees → Heap
      ↓
Graphs (BFS/DFS → Topo Sort → Shortest Path → MST → Union-Find)
      ↓
Dynamic Programming (1D → Knapsack → LCS → Interval → Bitmask/Tree DP)
      ↓
Greedy → Trie → Segment/Fenwick Tree → String Algorithms
      ↓
Bit Manipulation → Math/Number Theory → Divide & Conquer → Design Problems
```

## 🎯 How to Approach Any New DSA Problem

1. **Identify the input shape** — array, string, tree, graph, matrix? This narrows the topic bucket immediately.
2. **Look for signal keywords:**
   - "sorted array" → binary search / two pointers
   - "subarray/substring" + "contiguous" → sliding window / prefix sum
   - "all combinations/permutations/subsets" → backtracking
   - "shortest path" → BFS (unweighted) / Dijkstra (weighted) / DP (DAG)
   - "maximum/minimum ... choices" with overlapping subproblems → DP
   - "kth largest/smallest" → heap or quickselect
   - "connected components / grouping" → Union-Find or BFS/DFS
   - "range queries with updates" → segment tree / Fenwick tree
3. **Start with brute force** — establish correctness and a baseline complexity (usually O(n²) or O(2ⁿ)).
4. **Identify redundant work** — repeated subproblems → memoize (DP); repeated scanning → precompute (prefix sum/hashmap); repeated comparisons → sort first.
5. **Pick the right data structure** to cut the redundant work's cost, then re-derive complexity.
6. **Verify with edge cases** — empty input, single element, all duplicates, negative numbers, already sorted/reverse sorted.
