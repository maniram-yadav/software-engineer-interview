# Grind 169 — Unique Problems (not in Top Interview 150)

Source list: [Grind 169](https://leetcode.com/problem-list/p810ffa6/) (169 curated interview problems across 22 topics, an extended version of Grind 75). This file contains only the problems from that list **not already covered** in [top_100.md](top_100.md) (the Top Interview 150 set) — 85 problems were duplicates (same underlying LeetCode problem, even when the title differs slightly, e.g. "Merge Two Lists" = "Merge Two Sorted Lists", "N-Queens" is kept since top_100 only has "N-Queens II"). That leaves **84 unique problems** below, each tagged with its official LeetCode problem number.

## Categories

1. [Array](#array) (8)
2. [Binary](#binary) (3)
3. [Dynamic Programming](#dynamic-programming) (5)
4. [Graph](#graph) (13)
5. [Interval](#interval) (4)
6. [Linked List](#linked-list) (6)
7. [Matrix](#matrix) (2)
8. [Stack](#stack) (9)
9. [String](#string) (7)
10. [Binary Search](#binary-search) (4)
11. [Binary Tree](#binary-tree) (8)
12. [Binary Search Tree](#binary-search-tree) (2)
13. [Queue](#queue) (1)
14. [Trie](#trie) (1)
15. [Hash Table](#hash-table) (1)
16. [Heap](#heap) (5)
17. [Recursion](#recursion) (3)
18. [Math](#math) (2)

---

## Array

#### 217. Contains Duplicate `Easy`
Given an integer array `nums`, return true if any value appears at least twice.

**Example:**
```
Input: nums = [1,2,3,1]
Output: true
```

#### 283. Move Zeroes `Easy`
Given an integer array `nums`, move all zeroes to the end while maintaining the relative order of non-zero elements, in place.

**Example:**
```
Input: nums = [0,1,0,3,12]
Output: [1,3,12,0,0]
```

#### 977. Squares of a Sorted Array `Easy`
Given an integer array `nums` sorted in non-decreasing order, return an array of the squares of each number, also sorted in non-decreasing order.

**Example:**
```
Input: nums = [-4,-1,0,3,10]
Output: [0,1,9,16,100]
```

#### 75. Sort Colors `Medium`
Given an array with `n` objects colored red, white, or blue (represented as 0, 1, 2), sort them in place so objects of the same color are adjacent, in the order red, white, blue (Dutch national flag problem).

**Example:**
```
Input: nums = [2,0,2,1,1,0]
Output: [0,0,1,1,2,2]
```

#### 525. Contiguous Array `Medium`
Given a binary array `nums`, return the maximum length of a contiguous subarray with an equal number of 0s and 1s.

**Example:**
```
Input: nums = [0,1,0,1]
Output: 4
```

#### 560. Subarray Sum Equals K `Medium`
Given an integer array `nums` and an integer `k`, return the total number of contiguous subarrays whose sum equals `k`.

**Example:**
```
Input: nums = [1,1,1], k = 2
Output: 2
```

#### 16. 3Sum Closest `Medium`
Given an integer array `nums` and a target, find three integers whose sum is closest to target and return that sum.

**Example:**
```
Input: nums = [-1,2,1,-4], target = 1
Output: 2   (-1 + 2 + 1 = 2)
```

#### 239. Sliding Window Maximum `Hard`
Given an array `nums` and a sliding window of size `k` moving from left to right, return the max value in the window at each position.

**Example:**
```
Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
Output: [3,3,5,5,6,7]
```

---

## Binary

#### 338. Counting Bits `Easy`
Given an integer `n`, return an array `ans` of length `n+1` where `ans[i]` is the number of 1's in the binary representation of `i`.

**Example:**
```
Input: n = 5
Output: [0,1,1,2,1,2]
```

#### 268. Missing Number `Easy`
Given an array `nums` containing `n` distinct numbers in range `[0, n]`, return the one number missing from the range.

**Example:**
```
Input: nums = [3,0,1]
Output: 2
```

#### 287. Find the Duplicate Number `Medium`
Given an array of `n + 1` integers where each value is in `[1, n]`, and exactly one number repeats (possibly multiple times), find the duplicate without modifying the array and using O(1) extra space (Floyd's cycle detection).

**Example:**
```
Input: nums = [1,3,4,2,2]
Output: 2
```

---

## Dynamic Programming

#### 416. Partition Equal Subset Sum `Medium`
Given a non-empty array of positive integers, determine if it can be partitioned into two subsets with equal sum.

**Example:**
```
Input: nums = [1,5,11,5]
Output: true   ([1,5,5] and [11])
```

#### 62. Unique Paths `Medium`
A robot on an `m x n` grid starts at the top-left corner and can only move down or right. Return the number of unique paths to the bottom-right corner.

**Example:**
```
Input: m = 3, n = 7
Output: 28
```

#### 152. Maximum Product Subarray `Medium`
Given an integer array `nums`, find a contiguous subarray with the largest product, and return that product.

**Example:**
```
Input: nums = [2,3,-2,4]
Output: 6   ([2,3])
```

#### 91. Decode Ways `Medium`
A message of digits can be decoded via `'A'->1, ..., 'Z'->26`. Given a digit string `s`, return the number of ways to decode it.

**Example:**
```
Input: s = "12"
Output: 2   ("AB" or "L")
```

#### 377. Combination Sum IV `Medium`
Given an array of distinct positive integers and a target, return the number of possible combinations (order matters, elements reusable) that add up to target.

**Example:**
```
Input: nums = [1,2,3], target = 4
Output: 7
```

---

## Graph

#### 733. Flood Fill `Easy`
Given an image (2D grid of pixel values), a starting pixel `(sr, sc)`, and a new color, perform a flood fill: recolor the starting pixel and all 4-directionally connected pixels of the same original color.

**Example:**
```
Input: image = [[1,1,1],[1,1,0],[1,0,1]], sr = 1, sc = 1, color = 2
Output: [[2,2,2],[2,2,0],[2,0,1]]
```

#### 994. Rotting Oranges `Medium`
Given a grid where cells are empty (0), fresh orange (1), or rotten orange (2), each minute a fresh orange adjacent to a rotten one becomes rotten. Return the minimum minutes until no cell has a fresh orange, or -1 if impossible.

**Example:**
```
Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
Output: 4
```

#### 721. Accounts Merge `Medium`
Given a list of accounts, each with a name and a list of emails, merge accounts that share at least one email (same person), returning merged accounts with sorted emails.

**Example:**
```
Input: accounts = [["John","johnsmith@mail.com","john_newyork@mail.com"],["John","johnsmith@mail.com","john00@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
Output: [["John","john00@mail.com","john_newyork@mail.com","johnsmith@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
```

#### 310. Minimum Height Trees `Medium`
Given a tree (connected, undirected, acyclic graph) with `n` nodes, return all roots that produce minimum-height trees (the "centroids").

**Example:**
```
Input: n = 4, edges = [[1,0],[1,2],[1,3]]
Output: [1]
```

#### 417. Pacific Atlantic Water Flow `Medium`
Given an `m x n` grid of heights, find all cells from which water can flow to both the Pacific (top/left edges) and Atlantic (bottom/right edges) oceans (water flows to equal or lower neighbors).

**Example:**
```
Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
```

#### 1730. Shortest Path to Get Food `Medium`
Given a grid with your position (`*`), food cells (`#`), obstacles (`X`), and empty cells (`O`), return the shortest path length to any food cell, or -1 if unreachable.

**Example:**
```
Input: grid = [["X","X","X","X","X","X"],["X","*","O","O","O","X"],["X","O","O","#","O","X"],["X","X","X","X","X","X"]]
Output: 3
```

#### 261. Graph Valid Tree `Medium`
Given `n` nodes and a list of undirected edges, determine if these edges form a valid tree (connected and acyclic).

**Example:**
```
Input: n = 5, edges = [[0,1],[0,2],[0,3],[1,4]]
Output: true
```

#### 323. Number of Connected Components in an Undirected Graph `Medium`
Given `n` nodes and a list of undirected edges, return the number of connected components.

**Example:**
```
Input: n = 5, edges = [[0,1],[1,2],[3,4]]
Output: 2
```

#### 1197. Minimum Knight Moves `Medium`
On an infinite chessboard, a knight starts at `(0,0)`. Return the minimum number of moves to reach `(x, y)`.

**Example:**
```
Input: x = 2, y = 1
Output: 1
```

#### 787. Cheapest Flights Within K Stops `Medium`
Given `n` cities connected by flights with costs, find the cheapest price from `src` to `dst` using at most `k` stops, or -1 if no such route exists.

**Example:**
```
Input: n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1
Output: 700
```

#### 329. Longest Increasing Path in a Matrix `Hard`
Given an `m x n` integer matrix, return the length of the longest strictly increasing path (moving in any of 4 directions).

**Example:**
```
Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
Output: 4   (path 1->2->6->9)
```

#### 269. Alien Dictionary `Hard`
Given a list of words sorted lexicographically according to an unknown alien language's rules, derive a valid character ordering of that alien alphabet (topological sort). *(LeetCode Premium)*

**Example:**
```
Input: words = ["wrt","wrf","er","ett","rftt"]
Output: "wertf"
```

#### 815. Bus Routes `Hard`
Given bus routes (each a list of stops) and a source/target stop, return the minimum number of buses needed to travel from source to target, or -1.

**Example:**
```
Input: routes = [[1,2,7],[3,6,7]], source = 1, target = 6
Output: 2
```

---

## Interval

#### 252. Meeting Rooms `Easy`
Given an array of meeting time intervals, determine if a person could attend all meetings (no overlaps). *(LeetCode Premium)*

**Example:**
```
Input: intervals = [[0,30],[5,10],[15,20]]
Output: false
```

#### 253. Meeting Rooms II `Medium`
Given an array of meeting time intervals, return the minimum number of conference rooms required. *(LeetCode Premium)*

**Example:**
```
Input: intervals = [[0,30],[5,10],[15,20]]
Output: 2
```

#### 435. Non-overlapping Intervals `Medium`
Given an array of intervals, return the minimum number of intervals to remove so the rest are non-overlapping.

**Example:**
```
Input: intervals = [[1,2],[2,3],[3,4],[1,3]]
Output: 1
```

#### 759. Employee Free Time `Hard`
Given a list of schedules (each employee's list of non-overlapping intervals, sorted), return the list of finite intervals representing common, positive-length free time for all employees. *(LeetCode Premium)*

**Example:**
```
Input: schedule = [[[1,2],[5,6]],[[1,3]],[[4,10]]]
Output: [[3,4]]
```

---

## Linked List

#### 206. Reverse Linked List `Easy`
Given the head of a singly linked list, reverse the list and return the new head.

**Example:**
```
Input: head = [1,2,3,4,5]
Output: [5,4,3,2,1]
```

#### 876. Middle of the Linked List `Easy`
Given the head of a singly linked list, return the middle node (if two middle nodes, return the second one).

**Example:**
```
Input: head = [1,2,3,4,5]
Output: [3,4,5]
```

#### 234. Palindrome Linked List `Easy`
Given the head of a singly linked list, return true if it reads the same forward and backward.

**Example:**
```
Input: head = [1,2,2,1]
Output: true
```

#### 24. Swap Nodes in Pairs `Medium`
Given a linked list, swap every two adjacent nodes and return its head (swap the nodes themselves, not just values).

**Example:**
```
Input: head = [1,2,3,4]
Output: [2,1,4,3]
```

#### 328. Odd Even Linked List `Medium`
Given the head of a singly linked list, group all nodes at odd indices together followed by nodes at even indices (1-indexed), preserving relative order within each group, in O(1) extra space.

**Example:**
```
Input: head = [1,2,3,4,5]
Output: [1,3,5,2,4]
```

#### 143. Reorder List `Medium`
Given the head of a linked list `L0 -> L1 -> ... -> Ln-1 -> Ln`, reorder it in place to `L0 -> Ln -> L1 -> Ln-1 -> L2 -> Ln-2 -> ...`.

**Example:**
```
Input: head = [1,2,3,4]
Output: [1,4,2,3]
```

---

## Matrix

#### 542. 01 Matrix `Medium`
Given an `m x n` binary matrix, return the distance to the nearest 0 for each cell (multi-source BFS).

**Example:**
```
Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
Output: [[0,0,0],[0,1,0],[1,2,1]]
```

#### 37. Sudoku Solver `Hard`
Write a program to solve a Sudoku puzzle by filling the empty cells in place (backtracking).

**Example:**
```
Input: board = partially filled 9x9 Sudoku grid
Output: fully solved 9x9 Sudoku grid
```

---

## Stack

#### 232. Implement Queue using Stacks `Easy`
Implement a first-in-first-out (FIFO) queue using only two stacks, supporting `push`, `pop`, `peek`, and `empty`.

**Example:**
```
MyQueue q = new MyQueue();
q.push(1); q.push(2);
q.peek(); // 1
q.pop();  // 1
q.empty();// false
```

#### 844. Backspace String Compare `Easy`
Given two strings `s` and `t` containing lowercase letters and `#` (backspace), return true if they're equal after applying the backspaces.

**Example:**
```
Input: s = "ab#c", t = "ad#c"
Output: true   (both become "ac")
```

#### 739. Daily Temperatures `Medium`
Given an array of daily temperatures, return an array `answer` where `answer[i]` is the number of days you'd have to wait for a warmer temperature; 0 if none.

**Example:**
```
Input: temperatures = [73,74,75,71,69,72,76,73]
Output: [1,1,4,2,1,1,0,0]
```

#### 394. Decode String `Medium`
Given an encoded string with the pattern `k[encoded_string]` (repeat encoded_string k times), return the fully decoded string.

**Example:**
```
Input: s = "3[a]2[bc]"
Output: "aaabcbc"
```

#### 735. Asteroid Collision `Medium`
Given an array of asteroids (sign = direction, magnitude = size) moving in a row, simulate collisions (larger survives, equal both explode) and return the state after all collisions.

**Example:**
```
Input: asteroids = [5,10,-5]
Output: [5,10]
```

#### 227. Basic Calculator II `Medium`
Given a string expression containing non-negative integers and `+ - * /` (no parentheses), evaluate it following normal operator precedence.

**Example:**
```
Input: s = "3+2*2"
Output: 7
```

#### 84. Largest Rectangle in Histogram `Hard`
Given an array of bar heights of a histogram (width 1 each), return the area of the largest rectangle that fits within it.

**Example:**
```
Input: heights = [2,1,5,6,2,3]
Output: 10
```

#### 895. Maximum Frequency Stack `Hard`
Design a stack-like data structure `FreqStack` where `pop()` removes and returns the most frequent element, breaking ties by most recently pushed.

**Example:**
```
FreqStack fs = new FreqStack();
fs.push(5); fs.push(7); fs.push(5); fs.push(7); fs.push(4); fs.push(5);
fs.pop(); // 5 (most frequent, tie broken by recency... here 5 is most frequent)
```

#### 32. Longest Valid Parentheses `Hard`
Given a string containing just `(` and `)`, find the length of the longest valid (well-formed) parentheses substring.

**Example:**
```
Input: s = ")()())"
Output: 4   ("()()")
```

---

## String

#### 409. Longest Palindrome `Easy`
Given a string of lowercase/uppercase letters, return the length of the longest palindrome that can be built from those letters (case-sensitive, rearrangement allowed).

**Example:**
```
Input: s = "abccccdd"
Output: 7   (e.g. "dccaccd")
```

#### 8. String to Integer (atoi) `Medium`
Implement `atoi` to convert a string to a 32-bit signed integer, following specific whitespace/sign/overflow rules.

**Example:**
```
Input: s = "   -42"
Output: -42
```

#### 438. Find All Anagrams in a String `Medium`
Given strings `s` and `p`, return all start indices of `p`'s anagrams in `s`.

**Example:**
```
Input: s = "cbaebabacd", p = "abc"
Output: [0,6]
```

#### 424. Longest Repeating Character Replacement `Medium`
Given a string `s` and an integer `k`, you can replace up to `k` characters with any other uppercase letter. Return the length of the longest substring containing the same letter after such replacements.

**Example:**
```
Input: s = "ABAB", k = 2
Output: 4
```

#### 179. Largest Number `Medium`
Given a list of non-negative integers, arrange them so they form the largest possible number, returned as a string.

**Example:**
```
Input: nums = [3,30,34,5,9]
Output: "9534330"
```

#### 271. Encode and Decode Strings `Medium`
Design an algorithm to encode a list of strings into one string, and decode it back into the original list of strings. *(LeetCode Premium)*

**Example:**
```
Input: ["lint","code","love","you"]
Output (encoded then decoded): ["lint","code","love","you"]
```

#### 336. Palindrome Pairs `Hard`
Given a list of unique words, return all pairs of indices `(i, j)` such that concatenating `words[i] + words[j]` forms a palindrome.

**Example:**
```
Input: words = ["abcd","dcba","lls","s","sssll"]
Output: [[0,1],[1,0],[3,2],[2,4]]
```

---

## Binary Search

#### 704. Binary Search `Easy`
Given a sorted array of unique integers and a target, return its index using binary search, or -1 if absent.

**Example:**
```
Input: nums = [-1,0,3,5,9,12], target = 9
Output: 4
```

#### 278. First Bad Version `Easy`
You have `n` versions and want to find the first bad one, given an API `isBadVersion(version)`. Minimize the number of calls.

**Example:**
```
Input: n = 5, bad = 4
Output: 4
```

#### 981. Time Based Key-Value Store `Medium`
Design a time-based key-value store: `set(key, value, timestamp)` stores the value, and `get(key, timestamp)` returns the value set at the largest timestamp ≤ the given timestamp.

**Example:**
```
tkv.set("foo","bar",1);
tkv.get("foo",1); // "bar"
tkv.get("foo",3); // "bar"
tkv.set("foo","bar2",4);
tkv.get("foo",4); // "bar2"
```

#### 1235. Maximum Profit in Job Scheduling `Hard`
Given `startTime`, `endTime`, and `profit` arrays for jobs, find the maximum profit achievable by scheduling non-overlapping jobs.

**Example:**
```
Input: startTime = [1,2,3,3], endTime = [3,4,5,6], profit = [50,10,40,70]
Output: 120
```

---

## Binary Tree

#### 110. Balanced Binary Tree `Easy`
Given a binary tree, determine if it is height-balanced (the depth of the two subtrees of every node never differs by more than 1).

**Example:**
```
Input: root = [3,9,20,null,null,15,7]
Output: true
```

#### 543. Diameter of Binary Tree `Easy`
Given the root of a binary tree, return the length (in edges) of the longest path between any two nodes (may or may not pass through the root).

**Example:**
```
Input: root = [1,2,3,4,5]
Output: 3   (path [4,2,1,3] or [5,2,1,3])
```

#### 572. Subtree of Another Tree `Easy`
Given two binary trees `root` and `subRoot`, return true if `subRoot` has the same structure and node values as some subtree of `root`.

**Example:**
```
Input: root = [3,4,5,1,2], subRoot = [4,1,2]
Output: true
```

#### 113. Path Sum II `Medium`
Given the root of a binary tree and `targetSum`, return all root-to-leaf paths where the sum of node values equals `targetSum`.

**Example:**
```
Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
Output: [[5,4,11,2],[5,8,4,5]]
```

#### 662. Maximum Width of Binary Tree `Medium`
Given the root of a binary tree, return the maximum width of any level (distance between leftmost and rightmost non-null nodes, counting nulls in between as if the tree were complete).

**Example:**
```
Input: root = [1,3,2,5,3,null,9]
Output: 4
```

#### 437. Path Sum III `Medium`
Given the root of a binary tree and an integer `targetSum`, return the number of paths (not necessarily root-to-leaf, but must go downward) where the sum equals `targetSum`.

**Example:**
```
Input: root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8
Output: 3
```

#### 863. All Nodes Distance K in Binary Tree `Medium`
Given the root of a binary tree, a target node, and an integer `k`, return the values of all nodes that are exactly distance `k` from the target node.

**Example:**
```
Input: root = [3,5,1,6,2,0,8,null,null,7,4], target = 5, k = 2
Output: [7,4,1]
```

#### 297. Serialize and Deserialize Binary Tree `Hard`
Design an algorithm to serialize a binary tree to a string and deserialize that string back to the original tree structure.

**Example:**
```
Input: root = [1,2,3,null,null,4,5]
Output (round-trip): [1,2,3,null,null,4,5]
```

---

## Binary Search Tree

#### 235. Lowest Common Ancestor of a Binary Search Tree `Medium`
Given a BST and two nodes `p` and `q`, find their lowest common ancestor, using BST ordering properties.

**Example:**
```
Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
Output: 6
```

#### 285. Inorder Successor in BST `Medium`
Given a BST and a node `p`, find the in-order successor of `p` (the node with the smallest value greater than `p.val`), or null if none. *(LeetCode Premium)*

**Example:**
```
Input: root = [2,1,3], p = 1
Output: 2
```

---

## Queue

#### 362. Design Hit Counter `Medium`
Design a hit counter that counts hits received in the past 5 minutes, supporting `hit(timestamp)` and `getHits(timestamp)`, with timestamps monotonically increasing. *(LeetCode Premium)*

**Example:**
```
hc.hit(1); hc.hit(2); hc.hit(3);
hc.getHits(4); // 3
hc.hit(300);
hc.getHits(300); // 4
hc.getHits(301); // 3
```

---

## Trie

#### 588. Design In-Memory File System `Hard`
Design an in-memory file system supporting `ls`, `mkdir`, `addContentToFile`, and `readContentFromFile`, mimicking Unix-style paths.

**Example:**
```
fs.mkdir("/a/b/c");
fs.addContentToFile("/a/b/c/d","hello");
fs.ls("/");        // ["a"]
fs.readContentFromFile("/a/b/c/d"); // "hello"
```

---

## Hash Table

#### 41. First Missing Positive `Hard`
Given an unsorted integer array `nums`, return the smallest missing positive integer, in O(n) time and O(1) extra space.

**Example:**
```
Input: nums = [3,4,-1,1]
Output: 2
```

---

## Heap

#### 973. K Closest Points to Origin `Medium`
Given an array of points on the X-Y plane and an integer `k`, return the `k` closest points to the origin (any order).

**Example:**
```
Input: points = [[1,3],[-2,2]], k = 1
Output: [[-2,2]]
```

#### 621. Task Scheduler `Medium`
Given a list of CPU tasks (letters) and a cooldown `n` between two same tasks, return the minimum number of time units (including idle slots) needed to finish all tasks.

**Example:**
```
Input: tasks = ["A","A","A","B","B","B"], n = 2
Output: 8   ("A B idle A B idle A B")
```

#### 692. Top K Frequent Words `Medium`
Given an array of strings `words` and an integer `k`, return the `k` most frequent words, sorted by frequency (descending) then lexicographically for ties.

**Example:**
```
Input: words = ["i","love","leetcode","i","love","coding"], k = 2
Output: ["i","love"]
```

#### 658. Find K Closest Elements `Medium`
Given a sorted integer array `arr`, and integers `k` and `x`, return the `k` closest integers to `x` in the array, sorted ascending.

**Example:**
```
Input: arr = [1,2,3,4,5], k = 4, x = 3
Output: [1,2,3,4]
```

#### 632. Smallest Range Covering Elements from K Lists `Hard`
Given `k` sorted integer lists, find the smallest range `[a, b]` that includes at least one number from each list.

**Example:**
```
Input: nums = [[4,10,15,24,26],[0,9,12,20],[5,18,22,30]]
Output: [20,24]
```

---

## Recursion

#### 78. Subsets `Medium`
Given an integer array of unique elements, return all possible subsets (the power set).

**Example:**
```
Input: nums = [1,2,3]
Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
```

#### 31. Next Permutation `Medium`
Given an array of integers representing a permutation, rearrange it in place to the next lexicographically greater permutation; if none exists, rearrange to the lowest order (sorted ascending).

**Example:**
```
Input: nums = [1,2,3]
Output: [1,3,2]
```

#### 51. N-Queens `Hard`
Place `n` queens on an `n x n` chessboard so that no two attack each other; return all distinct board configurations.

**Example:**
```
Input: n = 4
Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
```

---

## Math

#### 528. Random Pick with Weight `Medium`
Given an array `w` of positive weights, design a structure that picks an index `i` with probability proportional to `w[i]`.

**Example:**
```
Input: w = [1,3]
Output: pickIndex() returns 0 with probability 1/4, 1 with probability 3/4
```

#### 7. Reverse Integer `Medium`
Given a 32-bit signed integer `x`, return `x` with its digits reversed; return 0 if the reversed value overflows a 32-bit signed integer.

**Example:**
```
Input: x = 123
Output: 321
```
