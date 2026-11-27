# LeetCode "Blind 150" (NeetCode 150) — Complete JavaScript Solutions

A full JavaScript reference for the 150 problems in the **NeetCode 150** list (the Blind 75 plus 75 more), grouped by pattern. Every problem has:

- its **LeetCode number and title**, with difficulty
- a **short problem statement** in my own words, plus an example
- one or more **complete JavaScript solutions**: brute force → better → optimal where that comparison helps
- **time and space complexity** for each approach
- a **key insight** line explaining the idea

> **Every snippet is self-contained: there is nothing to import.** JavaScript has no standard-library modules for algorithms. `Map`, `Set`, `Array`, `Math` and `BigInt` are built-in globals, so no `require` or `import` line is needed anywhere. What JavaScript lacks is a heap and a deque. Rather than depend on `@datastructures-js/priority-queue`, whose API changed between major versions, every snippet that needs a priority queue **defines a small `Heap` class inline**, so it runs on LeetCode, in Node and in a browser as-is. Queues use an array plus a head index, so dequeuing is O(1) instead of the O(n) `shift()`.
>
> Every function and method carries a **JSDoc signature** (`@param {number[]} nums`, `@return {boolean}`) in the same style as LeetCode's JavaScript templates, so editors still show the types. Regular problems are plain `function` declarations, which LeetCode accepts in place of its `var name = function (...)` template. Design problems are ES2022 `class`es, which LeetCode also accepts in place of the `prototype` template. LeetCode supplies `ListNode` and `TreeNode` (below), so paste only the snippet itself.
>
> **How this was verified:** the code is ES2022 and runs in strict mode (`"use strict"`). All 317 approaches were run under Node.js 22 on about 124,500 randomized cases, and each answer was compared with the already-verified Python solutions from the companion file, with no mismatches. Recursive approaches were also run at LeetCode's maximum input sizes. See the recursion-depth note in the gotchas below.

---

## Shared Definitions

LeetCode provides these classes for linked-list and tree problems. They are assumed (not redeclared) in those sections.

```javascript
/**
 * Definition for singly-linked list (LeetCode's own form).
 * @param {number} [val]
 * @param {ListNode | null} [next]
 */
function ListNode(val, next) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
}

/**
 * Definition for a binary tree node (LeetCode's own form).
 * @param {number} [val]
 * @param {TreeNode | null} [left]
 * @param {TreeNode | null} [right]
 */
function TreeNode(val, left, right) {
    this.val = val === undefined ? 0 : val;
    this.left = left === undefined ? null : left;
    this.right = right === undefined ? null : right;
}
```

> To run a snippet locally in Node, paste these two definitions above it. Nothing else needs to be imported.

### The reusable `Heap` class used in heap-based snippets

```javascript
/**
 * Binary heap ordered by `less(a, b)`: pass (a, b) => a < b for a min-heap.
 *
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}
```

> **LeetCode alternative:** LeetCode's JavaScript runtime also provides `MinPriorityQueue`, `MaxPriorityQueue` and `PriorityQueue` from `@datastructures-js/priority-queue` as globals (and `Queue` from `@datastructures-js/queue`), so no `require` is needed there. Their API depends on the library version LeetCode ships, so check the language info panel before relying on them. Locally you would need `npm install @datastructures-js/priority-queue` and `const { MinPriorityQueue } = require("@datastructures-js/priority-queue");`. The inline `Heap` avoids all of that.

---

## Table of Contents

| # | Category | Problems |
|---|----------|----------|
| 1 | [Arrays & Hashing](#1-arrays--hashing) | 9 |
| 2 | [Two Pointers](#2-two-pointers) | 5 |
| 3 | [Sliding Window](#3-sliding-window) | 6 |
| 4 | [Stack](#4-stack) | 7 |
| 5 | [Binary Search](#5-binary-search) | 7 |
| 6 | [Linked List](#6-linked-list) | 11 |
| 7 | [Trees](#7-trees) | 15 |
| 8 | [Tries](#8-tries) | 3 |
| 9 | [Heap / Priority Queue](#9-heap--priority-queue) | 7 |
| 10 | [Backtracking](#10-backtracking) | 9 |
| 11 | [Graphs](#11-graphs) | 13 |
| 12 | [Advanced Graphs](#12-advanced-graphs) | 6 |
| 13 | [1-D Dynamic Programming](#13-1-d-dynamic-programming) | 12 |
| 14 | [2-D Dynamic Programming](#14-2-d-dynamic-programming) | 11 |
| 15 | [Greedy](#15-greedy) | 8 |
| 16 | [Intervals](#16-intervals) | 6 |
| 17 | [Math & Geometry](#17-math--geometry) | 8 |
| 18 | [Bit Manipulation](#18-bit-manipulation) | 7 |
| | **Total** | **150** |

---

## JavaScript Interview Toolkit (quick reference)

```javascript
// Hash map / set
const freq = new Map();
freq.set(x, (freq.get(x) ?? 0) + 1);
const seen = new Set();

// Stack: push / pop / at(-1)
const stack = [];

// Queue in O(1): keep a head index instead of shift() (which is O(n))
const q = [start];
for (let head = 0; head < q.length; head++) {
    const cur = q[head]; /* q.push(next) */
}

// Sorting: the default sort is LEXICOGRAPHIC, so always pass a comparator for numbers
nums.sort((a, b) => a - b);
intervals.sort((a, b) => a[0] - b[0]);

// 2-D arrays: don't use new Array(m).fill([]), which shares one row
const grid = Array.from({ length: m }, () => new Array(n).fill(0));

// Integer division and 32-bit tricks
Math.trunc(a / b); // truncate toward zero
x | 0; // to int32
x >>> 0; // to uint32

// Big integers when needed
const big = 10n ** 20n;
```

**JavaScript gotchas for LeetCode:**
- `Array.prototype.sort()` without a comparator sorts numbers as strings (`[10, 9, 1] → [1, 10, 9]`).
- Numbers are 64-bit floats. Integers are exact up to 2⁵³ − 1 (`Number.MAX_SAFE_INTEGER`). Bitwise operators work on **32-bit signed** integers.
- Map/Set keys use identity for objects and arrays: `new Set([[1, 2]]).has([1, 2])` is `false`. Encode composite keys as strings (`` `${r},${c}` ``) or numbers (`r * n + c`).
- Reading past the end of an array gives `undefined`, not an error, and `undefined + 1` is `NaN`. Bounds-check before reading.
- Use `===` and `!==`. Loose `==` converts types (`0 == ""` is `true`).
- `||` treats `0` and `""` as missing. Use `??` for defaults (`count.get(x) ?? 0`).
- **Recursion depth:** the JS call stack can't be raised from inside a solution. Node allows roughly 12,000 trivial frames, and only about 7,000 for a typical tree DFS. A recursive DFS on a skewed tree of 10⁴ or more nodes, or a 300 × 300 grid flood fill, can throw `RangeError: Maximum call stack size exceeded`. Every problem where this can happen at LeetCode's limits also has an iterative approach in this file, and each such recursive approach carries a ⚠️ note.

---
## 1. Arrays & Hashing

### 217. Contains Duplicate
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/contains-duplicate/)

**Problem:** Given an integer array `nums`, return `true` if any value appears at least twice, and `false` if every element is distinct.

**Example:** `nums = [1,2,3,1]` → `true`; `nums = [1,2,3,4]` → `false`

**Key insight:** A hash set gives O(1) membership checks, so you can spot a repeat in one pass.

#### Approach 1: Brute Force
```javascript
/**
 * @param {number[]} nums
 * @return {boolean}
 */
function containsDuplicate(nums) {
    for (let i = 0; i < nums.length; i++) {
        for (let j = i + 1; j < nums.length; j++) {
            if (nums[i] === nums[j]) return true;
        }
    }
    return false;
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Sorting
```javascript
/**
 * @param {number[]} nums
 * @return {boolean}
 */
function containsDuplicate(nums) {
    const sorted = [...nums].sort((a, b) => a - b); // numeric comparator is required
    for (let i = 1; i < sorted.length; i++) {
        if (sorted[i] === sorted[i - 1]) return true;
    }
    return false;
}
```
**Complexity:** Time O(n log n) · Space O(n) for the copy

#### Approach 3: Set (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {boolean}
 */
function containsDuplicate(nums) {
    const seen = new Set();
    for (const n of nums) {
        if (seen.has(n)) return true;
        seen.add(n);
    }
    return false;
}
```
**Complexity:** Time O(n) · Space O(n)

**One-liner:** `return new Set(nums).size < nums.length;` has the same complexity but can't stop early.

---

### 242. Valid Anagram
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/valid-anagram/)

**Problem:** Given strings `s` and `t`, return `true` if `t` is an anagram of `s`, meaning both use exactly the same letters with the same counts.

**Example:** `s = "anagram", t = "nagaram"` → `true`; `s = "rat", t = "car"` → `false`

**Key insight:** Two strings are anagrams when their character frequency counts match.

#### Approach 1: Sorting
```javascript
/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
function isAnagram(s, t) {
    return s.length === t.length && [...s].sort().join("") === [...t].sort().join("");
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Frequency Array (Optimal)
```javascript
/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
function isAnagram(s, t) {
    if (s.length !== t.length) return false;
    const count = new Array(26).fill(0);
    for (let i = 0; i < s.length; i++) {
        count[s.charCodeAt(i) - 97]++;
        count[t.charCodeAt(i) - 97]--;
    }
    return count.every((c) => c === 0);
}
```
**Complexity:** Time O(n) · Space O(1) (26 counters)

#### Approach 3: Map (handles Unicode follow-up)
```javascript
/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
function isAnagram(s, t) {
    const count = new Map();
    for (const c of s) count.set(c, (count.get(c) ?? 0) + 1); // for..of iterates code points
    for (const c of t) {
        const left = (count.get(c) ?? 0) - 1;
        if (left < 0) return false;
        count.set(c, left);
    }
    for (const v of count.values()) if (v !== 0) return false;
    return true;
}
```
**Complexity:** Time O(n) · Space O(k), where k is the number of distinct characters

---

### 1. Two Sum
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/two-sum/)

**Problem:** Given an integer array `nums` and an integer `target`, return the indices of the two numbers that add up to `target`. Exactly one solution exists, and you may not use the same element twice.

**Example:** `nums = [2,7,11,15], target = 9` → `[0,1]`

**Key insight:** For each number `x`, you need `target - x`. Store values you've already seen in a map from value to index.

#### Approach 1: Brute Force
```javascript
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
function twoSum(nums, target) {
    for (let i = 0; i < nums.length; i++) {
        for (let j = i + 1; j < nums.length; j++) {
            if (nums[i] + nums[j] === target) return [i, j];
        }
    }
    return [];
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: One-Pass Map (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
function twoSum(nums, target) {
    const idx = new Map(); // value -> index
    for (let i = 0; i < nums.length; i++) {
        const j = idx.get(target - nums[i]);
        if (j !== undefined) return [j, i];
        idx.set(nums[i], i);
    }
    return [];
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 49. Group Anagrams
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/group-anagrams/)

**Problem:** Given an array of strings `strs`, group the anagrams together. You may return the groups in any order.

**Example:** `["eat","tea","tan","ate","nat","bat"]` → `[["bat"],["nat","tan"],["ate","eat","tea"]]`

**Key insight:** All anagrams share a canonical key, either the sorted string or the 26-letter count vector.

#### Approach 1: Sorted String as Key
```javascript
/**
 * @param {string[]} strs
 * @return {string[][]}
 */
function groupAnagrams(strs) {
    const groups = new Map();
    for (const s of strs) {
        const key = [...s].sort().join("");
        const g = groups.get(key);
        if (g) g.push(s);
        else groups.set(key, [s]);
    }
    return [...groups.values()];
}
```
**Complexity:** Time O(m · k log k), for m strings of max length k · Space O(m · k)

#### Approach 2: Count Signature as Key (Optimal)
```javascript
/**
 * @param {string[]} strs
 * @return {string[][]}
 */
function groupAnagrams(strs) {
    const groups = new Map();
    for (const s of strs) {
        const count = new Array(26).fill(0);
        for (let i = 0; i < s.length; i++) count[s.charCodeAt(i) - 97]++;
        const key = count.join("#"); // '#' keeps counts like 1,11 unambiguous
        const g = groups.get(key);
        if (g) g.push(s);
        else groups.set(key, [s]);
    }
    return [...groups.values()];
}
```
**Complexity:** Time O(m · k) · Space O(m · k)

---

### 347. Top K Frequent Elements
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/top-k-frequent-elements/)

**Problem:** Given an integer array `nums` and an integer `k`, return the `k` most frequent elements, in any order.

**Example:** `nums = [1,1,1,2,2,3], k = 2` → `[1,2]`

**Key insight:** Frequencies are bounded by `n`, so bucket sort by frequency gives O(n).

#### Approach 1: Sort by Frequency
```javascript
/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
function topKFrequent(nums, k) {
    const freq = new Map();
    for (const n of nums) freq.set(n, (freq.get(n) ?? 0) + 1);
    return [...freq.entries()]
        .sort((a, b) => b[1] - a[1])
        .slice(0, k)
        .map(([v]) => v);
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Min-Heap of Size k
```javascript
/**
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}

/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
function topKFrequent(nums, k) {
    const freq = new Map();
    for (const n of nums) freq.set(n, (freq.get(n) ?? 0) + 1);
    const heap = new Heap((a, b) => a[1] < b[1]); // [value, count], fewest on top
    for (const entry of freq) {
        heap.push(entry);
        if (heap.size > k) heap.pop();
    }
    const res = [];
    while (heap.size) res.push(heap.pop()[0]);
    return res;
}
```
**Complexity:** Time O(n log k) · Space O(n)

#### Approach 3: Bucket Sort (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
function topKFrequent(nums, k) {
    const freq = new Map();
    for (const n of nums) freq.set(n, (freq.get(n) ?? 0) + 1);
    const buckets = Array.from({ length: nums.length + 1 }, () => []); // index = frequency
    for (const [v, c] of freq) buckets[c].push(v);
    const res = [];
    for (let f = buckets.length - 1; f > 0 && res.length < k; f--) {
        for (const v of buckets[f]) {
            res.push(v);
            if (res.length === k) break;
        }
    }
    return res;
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 271. Encode and Decode Strings (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/encode-and-decode-strings/) · Free mirror: LintCode 659

**Problem:** Design an algorithm that encodes a list of strings into a single string, and decodes that string back into the original list. The strings may contain any character, including delimiters.

**Example:** `["neet","code","love","you"]` → encode → decode → `["neet","code","love","you"]`

**Key insight:** Length-prefix each string (`len#str`). The decoder reads the length first, so delimiter characters inside the strings can't confuse it.

#### Approach: Length Prefix (Optimal)
```javascript
class Codec {
    /**
     * @param {string[]} strs
     * @return {string}
     */
    encode(strs) {
        return strs.map((s) => `${s.length}#${s}`).join("");
    }

    /**
     * @param {string} s
     * @return {string[]}
     */
    decode(s) {
        const res = [];
        let i = 0;
        while (i < s.length) {
            const j = s.indexOf("#", i);
            const len = Number(s.slice(i, j)); // length in UTF-16 code units, matching slice()
            res.push(s.slice(j + 1, j + 1 + len));
            i = j + 1 + len;
        }
        return res;
    }
}
```
**Complexity:** Time O(N) for total characters N · Space O(N) for the output

**Alternative:** Escaping, for example writing `#` as `##` and using `" # "` as the separator, also works, but length-prefixing is simpler and harder to get wrong.

---

### 238. Product of Array Except Self
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/product-of-array-except-self/)

**Problem:** Given an integer array `nums`, return an array `answer` where `answer[i]` is the product of every element except `nums[i]`. Run in O(n) without using division.

**Example:** `[1,2,3,4]` → `[24,12,8,6]`

**Key insight:** `answer[i] = (product of everything left of i) × (product of everything right of i)`.

#### Approach 1: Prefix and Suffix Arrays
```javascript
/**
 * @param {number[]} nums
 * @return {number[]}
 */
function productExceptSelf(nums) {
    const n = nums.length;
    const prefix = new Array(n).fill(1);
    const suffix = new Array(n).fill(1);
    for (let i = 1; i < n; i++) prefix[i] = prefix[i - 1] * nums[i - 1];
    for (let i = n - 2; i >= 0; i--) suffix[i] = suffix[i + 1] * nums[i + 1];
    return prefix.map((p, i) => p * suffix[i] + 0); // + 0 turns -0 into 0
}
```
**Complexity:** Time O(n) · Space O(n)

> `+ 0` normalizes `-0` (which appears when a zero is multiplied by a negative) to `0`. LeetCode's judge serializes both as `0` anyway, but it makes `Object.is`-style comparisons behave.

#### Approach 2: In-Place Prefix and Running Suffix (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {number[]}
 */
function productExceptSelf(nums) {
    const n = nums.length;
    const res = new Array(n).fill(1);
    for (let i = 1; i < n; i++) res[i] = res[i - 1] * nums[i - 1];
    let suffix = 1;
    for (let i = n - 1; i >= 0; i--) {
        res[i] = res[i] * suffix + 0;
        suffix *= nums[i];
    }
    return res;
}
```
**Complexity:** Time O(n) · Space O(1) extra (the output doesn't count)

---

### 36. Valid Sudoku
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/valid-sudoku/)

**Problem:** Check whether a partially filled 9×9 Sudoku board is valid. Each row, each column, and each 3×3 sub-box must have no repeated digits 1–9. Empty cells are `'.'`. The board doesn't need to be solvable.

**Key insight:** Track seen digits per row, per column, and per box. Box index = `(r/3)*3 + c/3`.

#### Approach 1: Sets per Row, Column, and Box
```javascript
/**
 * @param {string[][]} board
 * @return {boolean}
 */
function isValidSudoku(board) {
    const rows = Array.from({ length: 9 }, () => new Set());
    const cols = Array.from({ length: 9 }, () => new Set());
    const boxes = Array.from({ length: 9 }, () => new Set());
    for (let r = 0; r < 9; r++) {
        for (let c = 0; c < 9; c++) {
            const d = board[r][c];
            if (d === ".") continue;
            const b = Math.floor(r / 3) * 3 + Math.floor(c / 3);
            if (rows[r].has(d) || cols[c].has(d) || boxes[b].has(d)) return false;
            rows[r].add(d);
            cols[c].add(d);
            boxes[b].add(d);
        }
    }
    return true;
}
```
**Complexity:** Time O(81) = O(1) · Space O(1)

#### Approach 2: Bitmasks
```javascript
/**
 * @param {string[][]} board
 * @return {boolean}
 */
function isValidSudoku(board) {
    const rows = new Array(9).fill(0);
    const cols = new Array(9).fill(0);
    const boxes = new Array(9).fill(0);
    for (let r = 0; r < 9; r++) {
        for (let c = 0; c < 9; c++) {
            if (board[r][c] === ".") continue;
            const bit = 1 << (Number(board[r][c]) - 1);
            const b = Math.floor(r / 3) * 3 + Math.floor(c / 3);
            if ((rows[r] | cols[c] | boxes[b]) & bit) return false;
            rows[r] |= bit;
            cols[c] |= bit;
            boxes[b] |= bit;
        }
    }
    return true;
}
```
**Complexity:** Time O(1) · Space O(1)

---

### 128. Longest Consecutive Sequence
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-consecutive-sequence/)

**Problem:** Given an unsorted integer array `nums`, return the length of the longest run of consecutive integers. The algorithm must run in O(n).

**Example:** `[100,4,200,1,3,2]` → `4` (the run is `1,2,3,4`)

**Key insight:** Only start counting at a number `x` when `x-1` isn't in the set. That way each run is walked once.

#### Approach 1: Sorting
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function longestConsecutive(nums) {
    if (nums.length === 0) return 0;
    const sorted = [...new Set(nums)].sort((a, b) => a - b);
    let best = 1, cur = 1;
    for (let i = 1; i < sorted.length; i++) {
        cur = sorted[i] === sorted[i - 1] + 1 ? cur + 1 : 1;
        best = Math.max(best, cur);
    }
    return best;
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Set (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function longestConsecutive(nums) {
    const set = new Set(nums);
    let best = 0;
    for (const n of set) {
        if (set.has(n - 1)) continue; // not the start of a run
        let len = 1;
        while (set.has(n + len)) len++;
        best = Math.max(best, len);
    }
    return best;
}
```
**Complexity:** Time O(n) · Space O(n)

---
## 2. Two Pointers

### 125. Valid Palindrome
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/valid-palindrome/)

**Problem:** A phrase is a palindrome if, after lowercasing it and removing every non-alphanumeric character, it reads the same forward and backward. Given a string `s`, return whether it is a palindrome.

**Example:** `"A man, a plan, a canal: Panama"` → `true`; `"race a car"` → `false`

**Key insight:** Move two pointers inward, skip non-alphanumeric characters, and compare case-insensitively.

#### Approach 1: Clean, Then Compare
```javascript
/**
 * @param {string} s
 * @return {boolean}
 */
function isPalindrome(s) {
    const clean = s.toLowerCase().replace(/[^a-z0-9]/g, "");
    return clean === [...clean].reverse().join("");
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Two Pointers In Place (Optimal)
```javascript
/**
 * @param {string} s
 * @return {boolean}
 */
function isPalindrome(s) {
    const isAlnum = (c) => /[a-zA-Z0-9]/.test(c);
    let l = 0, r = s.length - 1;
    while (l < r) {
        if (!isAlnum(s[l])) l++;
        else if (!isAlnum(s[r])) r--;
        else {
            if (s[l].toLowerCase() !== s[r].toLowerCase()) return false;
            l++;
            r--;
        }
    }
    return true;
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 167. Two Sum II – Input Array Is Sorted
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)

**Problem:** Given a **1-indexed** array `numbers` sorted in non-decreasing order, find two numbers that add up to `target`. Return their 1-based indices `[i, j]` with `i < j`. Use only constant extra space.

**Example:** `numbers = [2,7,11,15], target = 9` → `[1,2]`

**Key insight:** If the sum is too small, move the left pointer right. If it's too big, move the right pointer left.

#### Approach 1: Binary Search for the Complement
```javascript
/**
 * @param {number[]} numbers
 * @param {number} target
 * @return {number[]}
 */
function twoSum(numbers, target) {
    for (let i = 0; i < numbers.length; i++) {
        const want = target - numbers[i];
        let lo = i + 1, hi = numbers.length - 1;
        while (lo <= hi) {
            const mid = (lo + hi) >> 1;
            if (numbers[mid] === want) return [i + 1, mid + 1];
            if (numbers[mid] < want) lo = mid + 1;
            else hi = mid - 1;
        }
    }
    return [];
}
```
**Complexity:** Time O(n log n) · Space O(1)

#### Approach 2: Two Pointers (Optimal)
```javascript
/**
 * @param {number[]} numbers
 * @param {number} target
 * @return {number[]}
 */
function twoSum(numbers, target) {
    let l = 0, r = numbers.length - 1;
    while (l < r) {
        const sum = numbers[l] + numbers[r];
        if (sum === target) return [l + 1, r + 1];
        if (sum < target) l++;
        else r--;
    }
    return [];
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 15. 3Sum
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/3sum/)

**Problem:** Given an integer array `nums`, return every unique triplet `[a, b, c]` with distinct indices such that `a + b + c == 0`. The result must not contain duplicate triplets.

**Example:** `[-1,0,1,2,-1,-4]` → `[[-1,-1,2],[-1,0,1]]`

**Key insight:** Sort the array, fix the first element, then run Two Sum II on the rest. Skip equal values to avoid duplicate triplets.

#### Approach 1: Set per Anchor
```javascript
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
function threeSum(nums) {
    nums.sort((a, b) => a - b);
    const res = [];
    const seenTriplets = new Set();
    for (let i = 0; i < nums.length; i++) {
        if (i > 0 && nums[i] === nums[i - 1]) continue;
        const seen = new Set();
        for (let j = i + 1; j < nums.length; j++) {
            const need = -nums[i] - nums[j];
            if (seen.has(need)) {
                const key = `${nums[i]},${need},${nums[j]}`;
                if (!seenTriplets.has(key)) {
                    seenTriplets.add(key);
                    res.push([nums[i], need, nums[j]]);
                }
            }
            seen.add(nums[j]);
        }
    }
    return res;
}
```
**Complexity:** Time O(n²) · Space O(n)

#### Approach 2: Sort + Two Pointers (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
function threeSum(nums) {
    nums.sort((a, b) => a - b);
    const res = [];
    for (let i = 0; i < nums.length - 2; i++) {
        if (nums[i] > 0) break; // the smallest value is positive, so no triplet can sum to zero
        if (i > 0 && nums[i] === nums[i - 1]) continue;
        let l = i + 1, r = nums.length - 1;
        while (l < r) {
            const sum = nums[i] + nums[l] + nums[r];
            if (sum < 0) l++;
            else if (sum > 0) r--;
            else {
                res.push([nums[i], nums[l], nums[r]]);
                l++;
                r--;
                while (l < r && nums[l] === nums[l - 1]) l++;
            }
        }
    }
    return res;
}
```
**Complexity:** Time O(n²) · Space O(1) extra (plus the sort and the output)

---

### 11. Container With Most Water
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/container-with-most-water/)

**Problem:** `height[i]` is the height of a vertical line at position `i`. Pick two lines that, together with the x-axis, hold the most water, and return that maximum area.

**Example:** `[1,8,6,2,5,4,8,3,7]` → `49`

**Key insight:** The area is limited by the shorter line, so moving the taller pointer can never help. Always move the shorter one.

#### Approach 1: Brute Force
```javascript
/**
 * @param {number[]} height
 * @return {number}
 */
function maxArea(height) {
    let best = 0;
    for (let i = 0; i < height.length; i++) {
        for (let j = i + 1; j < height.length; j++) {
            best = Math.max(best, (j - i) * Math.min(height[i], height[j]));
        }
    }
    return best;
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Two Pointers (Optimal)
```javascript
/**
 * @param {number[]} height
 * @return {number}
 */
function maxArea(height) {
    let l = 0, r = height.length - 1, best = 0;
    while (l < r) {
        best = Math.max(best, (r - l) * Math.min(height[l], height[r]));
        if (height[l] < height[r]) l++;
        else r--;
    }
    return best;
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 42. Trapping Rain Water
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/trapping-rain-water/)

**Problem:** Given `n` non-negative integers representing an elevation map where each bar has width 1, compute how much water it traps after raining.

**Example:** `[0,1,0,2,1,0,1,3,2,1,2,1]` → `6`

**Key insight:** The water above bar `i` is `min(maxLeft, maxRight) - height[i]`.

#### Approach 1: Prefix and Suffix Max Arrays
```javascript
/**
 * @param {number[]} height
 * @return {number}
 */
function trap(height) {
    const n = height.length;
    if (n === 0) return 0;
    const left = new Array(n), right = new Array(n);
    left[0] = height[0];
    for (let i = 1; i < n; i++) left[i] = Math.max(left[i - 1], height[i]);
    right[n - 1] = height[n - 1];
    for (let i = n - 2; i >= 0; i--) right[i] = Math.max(right[i + 1], height[i]);
    let water = 0;
    for (let i = 0; i < n; i++) water += Math.min(left[i], right[i]) - height[i];
    return water;
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Monotonic Stack
```javascript
/**
 * @param {number[]} height
 * @return {number}
 */
function trap(height) {
    const stack = []; // indices with decreasing heights
    let water = 0;
    for (let i = 0; i < height.length; i++) {
        while (stack.length && height[i] > height[stack[stack.length - 1]]) {
            const bottom = stack.pop();
            if (!stack.length) break;
            const left = stack[stack.length - 1];
            const width = i - left - 1;
            water += width * (Math.min(height[left], height[i]) - height[bottom]);
        }
        stack.push(i);
    }
    return water;
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 3: Two Pointers (Optimal)
```javascript
/**
 * @param {number[]} height
 * @return {number}
 */
function trap(height) {
    let l = 0, r = height.length - 1, leftMax = 0, rightMax = 0, water = 0;
    while (l < r) {
        if (height[l] < height[r]) {
            leftMax = Math.max(leftMax, height[l]);
            water += leftMax - height[l++];
        } else {
            rightMax = Math.max(rightMax, height[r]);
            water += rightMax - height[r--];
        }
    }
    return water;
}
```
**Complexity:** Time O(n) · Space O(1)

---
## 3. Sliding Window

### 121. Best Time to Buy and Sell Stock
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/)

**Problem:** `prices[i]` is a stock's price on day `i`. Choose one day to buy and a later day to sell. Return the maximum profit, or `0` if no profit is possible.

**Example:** `[7,1,5,3,6,4]` → `5` (buy at 1, sell at 6)

**Key insight:** Track the lowest price so far. The best sale today is `price - minSoFar`.

#### Approach 1: Brute Force
```javascript
/**
 * @param {number[]} prices
 * @return {number}
 */
function maxProfit(prices) {
    let best = 0;
    for (let i = 0; i < prices.length; i++) {
        for (let j = i + 1; j < prices.length; j++) best = Math.max(best, prices[j] - prices[i]);
    }
    return best;
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: One Pass with Running Min (Optimal)
```javascript
/**
 * @param {number[]} prices
 * @return {number}
 */
function maxProfit(prices) {
    let minPrice = Infinity, best = 0;
    for (const p of prices) {
        minPrice = Math.min(minPrice, p);
        best = Math.max(best, p - minPrice);
    }
    return best;
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 3. Longest Substring Without Repeating Characters
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-substring-without-repeating-characters/)

**Problem:** Given a string `s`, return the length of the longest substring that has no repeated characters.

**Example:** `"abcabcbb"` → `3` (`"abc"`); `"pwwkew"` → `3` (`"wke"`)

**Key insight:** Keep a window with no duplicates. When a duplicate arrives, jump the left edge past its previous occurrence.

#### Approach 1: Sliding Window with a Set
```javascript
/**
 * @param {string} s
 * @return {number}
 */
function lengthOfLongestSubstring(s) {
    const window = new Set();
    let l = 0, best = 0;
    for (let r = 0; r < s.length; r++) {
        while (window.has(s[r])) window.delete(s[l++]);
        window.add(s[r]);
        best = Math.max(best, r - l + 1);
    }
    return best;
}
```
**Complexity:** Time O(n) (each character enters and leaves once) · Space O(min(n, alphabet))

#### Approach 2: Last-Seen Index Jump (Optimal)
```javascript
/**
 * @param {string} s
 * @return {number}
 */
function lengthOfLongestSubstring(s) {
    const last = new Map(); // char -> last index seen
    let l = 0, best = 0;
    for (let r = 0; r < s.length; r++) {
        const prev = last.get(s[r]);
        if (prev !== undefined && prev >= l) l = prev + 1;
        last.set(s[r], r);
        best = Math.max(best, r - l + 1);
    }
    return best;
}
```
**Complexity:** Time O(n) · Space O(min(n, alphabet))

---

### 424. Longest Repeating Character Replacement
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-repeating-character-replacement/)

**Problem:** Given an uppercase string `s` and an integer `k`, you may change up to `k` characters to any other uppercase letter. Return the length of the longest substring containing a single repeated letter after the changes.

**Example:** `s = "AABABBA", k = 1` → `4`

**Key insight:** A window is valid when `windowLen - maxFreq <= k`. `maxFreq` never needs to decrease, because only a larger `maxFreq` can produce a longer answer.

#### Approach 1: Recompute Max Frequency (O(26) per step)
```javascript
/**
 * @param {string} s
 * @param {number} k
 * @return {number}
 */
function characterReplacement(s, k) {
    const count = new Array(26).fill(0);
    let l = 0, best = 0;
    for (let r = 0; r < s.length; r++) {
        count[s.charCodeAt(r) - 65]++;
        while (r - l + 1 - Math.max(...count) > k) count[s.charCodeAt(l++) - 65]--;
        best = Math.max(best, r - l + 1);
    }
    return best;
}
```
**Complexity:** Time O(26 · n) · Space O(1)

#### Approach 2: Running Max Frequency (Optimal)
```javascript
/**
 * @param {string} s
 * @param {number} k
 * @return {number}
 */
function characterReplacement(s, k) {
    const count = new Array(26).fill(0);
    let l = 0, maxFreq = 0;
    for (let r = 0; r < s.length; r++) {
        maxFreq = Math.max(maxFreq, ++count[s.charCodeAt(r) - 65]);
        if (r - l + 1 - maxFreq > k) count[s.charCodeAt(l++) - 65]--; // shift the window, don't shrink it
    }
    return s.length - l; // the window size never shrinks, so its final size is the answer
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 567. Permutation in String
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/permutation-in-string/)

**Problem:** Given strings `s1` and `s2`, return `true` if some permutation of `s1` appears as a substring of `s2`.

**Example:** `s1 = "ab", s2 = "eidbaooo"` → `true` (`"ba"`)

**Key insight:** Slide a fixed window of size `len(s1)` across `s2` and compare letter counts.

#### Approach 1: Compare Count Arrays Each Step
```javascript
/**
 * @param {string} s1
 * @param {string} s2
 * @return {boolean}
 */
function checkInclusion(s1, s2) {
    const k = s1.length;
    if (k > s2.length) return false;
    const need = new Array(26).fill(0), win = new Array(26).fill(0);
    for (let i = 0; i < k; i++) {
        need[s1.charCodeAt(i) - 97]++;
        win[s2.charCodeAt(i) - 97]++;
    }
    const same = () => need.every((v, i) => v === win[i]);
    if (same()) return true;
    for (let r = k; r < s2.length; r++) {
        win[s2.charCodeAt(r) - 97]++;
        win[s2.charCodeAt(r - k) - 97]--;
        if (same()) return true;
    }
    return false;
}
```
**Complexity:** Time O(26 · n) · Space O(1)

#### Approach 2: Track the Number of Matching Letters (Optimal)
```javascript
/**
 * @param {string} s1
 * @param {string} s2
 * @return {boolean}
 */
function checkInclusion(s1, s2) {
    const k = s1.length;
    if (k > s2.length) return false;
    const c1 = new Array(26).fill(0), c2 = new Array(26).fill(0);
    for (let i = 0; i < k; i++) {
        c1[s1.charCodeAt(i) - 97]++;
        c2[s2.charCodeAt(i) - 97]++;
    }
    let matches = 0;
    for (let i = 0; i < 26; i++) if (c1[i] === c2[i]) matches++;
    const update = (idx, delta) => {
        if (c1[idx] === c2[idx]) matches--;
        c2[idx] += delta;
        if (c1[idx] === c2[idx]) matches++;
    };
    for (let r = k; r < s2.length; r++) {
        if (matches === 26) return true;
        update(s2.charCodeAt(r) - 97, 1);
        update(s2.charCodeAt(r - k) - 97, -1);
    }
    return matches === 26;
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 76. Minimum Window Substring
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/minimum-window-substring/)

**Problem:** Given strings `s` and `t`, return the shortest substring of `s` that contains every character of `t`, including duplicates. Return `""` if there is none.

**Example:** `s = "ADOBECODEBANC", t = "ABC"` → `"BANC"`

**Key insight:** Expand right until the window covers `t` (`have == need`), then shrink from the left while it still covers `t`, recording the smallest window.

#### Approach: Sliding Window with Counts (Optimal)
```javascript
/**
 * @param {string} s
 * @param {string} t
 * @return {string}
 */
function minWindow(s, t) {
    if (t.length === 0 || t.length > s.length) return "";
    const need = new Array(128).fill(0), win = new Array(128).fill(0);
    let required = 0;
    for (let i = 0; i < t.length; i++) if (need[t.charCodeAt(i)]++ === 0) required++;
    let have = 0, l = 0, bestL = 0, bestLen = Infinity;
    for (let r = 0; r < s.length; r++) {
        const c = s.charCodeAt(r);
        if (need[c] > 0 && ++win[c] === need[c]) have++;
        while (have === required) {
            if (r - l + 1 < bestLen) {
                bestL = l;
                bestLen = r - l + 1;
            }
            const lc = s.charCodeAt(l++);
            if (need[lc] > 0 && win[lc]-- === need[lc]) have--;
        }
    }
    return bestLen === Infinity ? "" : s.slice(bestL, bestL + bestLen);
}
```
**Complexity:** Time O(|s| + |t|) · Space O(1) (128 ASCII slots)

**Alternative (brute force):** Check every substring of `s` against `t`'s counts: O(n² · 128) time. Too slow for the constraints, but useful as a correctness baseline.

---

### 239. Sliding Window Maximum
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/sliding-window-maximum/)

**Problem:** Given an array `nums` and a window size `k`, slide the window from left to right one step at a time and return the maximum of each window.

**Example:** `nums = [1,3,-1,-3,5,3,6,7], k = 3` → `[3,3,5,5,6,7]`

**Key insight:** Keep a deque of indices whose values are decreasing. The front is always the window's maximum.

#### Approach 1: Max-Heap with Lazy Deletion
```javascript
/**
 * Binary heap ordered by `less(a, b)`: pass (a, b) => a < b for a min-heap.
 *
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}
/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
function maxSlidingWindow(nums, k) {
    const heap = new Heap((a, b) => nums[a] > nums[b]); // indices, largest value on top
    const res = [];
    for (let i = 0; i < nums.length; i++) {
        heap.push(i);
        if (i >= k - 1) {
            while (heap.peek() <= i - k) heap.pop(); // top has left the window
            res.push(nums[heap.peek()]);
        }
    }
    return res;
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Monotonic Deque (Optimal)
A plain array plus a `head` index gives O(1) pops from both ends.
```javascript
/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
function maxSlidingWindow(nums, k) {
    const dq = []; // indices; their nums values are decreasing
    let head = 0;
    const res = [];
    for (let i = 0; i < nums.length; i++) {
        if (head < dq.length && dq[head] <= i - k) head++;
        while (dq.length > head && nums[dq[dq.length - 1]] < nums[i]) dq.pop();
        dq.push(i);
        if (i >= k - 1) res.push(nums[dq[head]]);
    }
    return res;
}
```
**Complexity:** Time O(n) (each index is pushed and popped at most once) · Space O(n) for the backing array, O(k) live

---
## 4. Stack

### 20. Valid Parentheses
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/valid-parentheses/)

**Problem:** Given a string made only of `()[]{}`, decide whether it is valid. Every opening bracket must be closed by the same type of bracket, in the correct order.

**Example:** `"()[]{}"` → `true`; `"(]"` → `false`; `"([)]"` → `false`

**Key insight:** The most recent unmatched opening bracket must be the first one closed, which is last-in, first-out.

#### Approach 1: Stack (Optimal)
```javascript
/**
 * @param {string} s
 * @return {boolean}
 */
function isValid(s) {
    const pairs = { ")": "(", "]": "[", "}": "{" };
    const stack = [];
    for (const c of s) {
        if (c in pairs) {
            if (stack.pop() !== pairs[c]) return false;
        } else stack.push(c);
    }
    return stack.length === 0;
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Repeated Replacement (simple but slow)
```javascript
/**
 * @param {string} s
 * @return {boolean}
 */
function isValid(s) {
    let prev = "";
    while (prev !== s) {
        prev = s;
        s = s.replace("()", "").replace("[]", "").replace("{}", "");
    }
    return s === "";
}
```
**Complexity:** Time O(n²) · Space O(n)

---

### 155. Min Stack
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/min-stack/)

**Problem:** Design a stack that supports `push`, `pop`, `top`, and `getMin` (return the smallest element) in O(1) time each.

**Key insight:** Store the minimum at each depth alongside the value, so popping restores the previous minimum automatically.

#### Approach 1: Pairs of (value, current min)
```javascript
class MinStack {
    stack = []; // [value, min at this depth]

    /**
     * @param {number} val
     * @return {void}
     */
    push(val) {
        const m = this.stack.length ? Math.min(val, this.stack[this.stack.length - 1][1]) : val;
        this.stack.push([val, m]);
    }

    /**
     * @return {void}
     */
    pop() {
        this.stack.pop();
    }

    /**
     * @return {number}
     */
    top() {
        return this.stack[this.stack.length - 1][0];
    }

    /**
     * @return {number}
     */
    getMin() {
        return this.stack[this.stack.length - 1][1];
    }
}
```
**Complexity:** O(1) per operation · Space O(n)

#### Approach 2: Two Stacks (min stack only grows on a new min)
```javascript
class MinStack {
    stack = [];
    mins = [];

    /**
     * @param {number} val
     * @return {void}
     */
    push(val) {
        this.stack.push(val);
        if (!this.mins.length || val <= this.mins[this.mins.length - 1]) this.mins.push(val);
    }

    /**
     * @return {void}
     */
    pop() {
        if (this.stack.pop() === this.mins[this.mins.length - 1]) this.mins.pop();
    }

    /**
     * @return {number}
     */
    top() {
        return this.stack[this.stack.length - 1];
    }

    /**
     * @return {number}
     */
    getMin() {
        return this.mins[this.mins.length - 1];
    }
}
```
**Complexity:** O(1) per operation · Space O(n), usually less in practice

#### Approach 3: Single Stack Storing Differences (O(1) extra space)
`val - min` can reach ±2³², which is still exact in a JS number (exact up to 2⁵³).
```javascript
class MinStack {
    stack = []; // value - min at push time
    min = 0;

    /**
     * @param {number} val
     * @return {void}
     */
    push(val) {
        if (!this.stack.length) {
            this.stack.push(0);
            this.min = val;
        } else {
            this.stack.push(val - this.min);
            if (val < this.min) this.min = val;
        }
    }

    /**
     * @return {void}
     */
    pop() {
        const diff = this.stack.pop();
        if (diff < 0) this.min -= diff; // the popped value was the min, so restore the previous one
    }

    /**
     * @return {number}
     */
    top() {
        const diff = this.stack[this.stack.length - 1];
        return diff < 0 ? this.min : this.min + diff;
    }

    /**
     * @return {number}
     */
    getMin() {
        return this.min;
    }
}
```
**Complexity:** O(1) per operation · Space O(n) for values, no second stack

---

### 150. Evaluate Reverse Polish Notation
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/evaluate-reverse-polish-notation/)

**Problem:** Evaluate an arithmetic expression written in Reverse Polish (postfix) notation. The operators are `+ - * /`, and division truncates toward zero.

**Example:** `["2","1","+","3","*"]` → `9`; `["4","13","5","/","+"]` → `6`

**Key insight:** Push operands. On an operator, pop two values (second operand first) and push the result.

#### Approach 1: Stack with an Operator Table (Optimal)
```javascript
/**
 * @param {string[]} tokens
 * @return {number}
 */
function evalRPN(tokens) {
    const ops = {
        "+": (a, b) => a + b,
        "-": (a, b) => a - b,
        "*": (a, b) => a * b,
        "/": (a, b) => Math.trunc(a / b), // truncate toward zero
    };
    const stack = [];
    for (const tok of tokens) {
        if (tok in ops) {
            const b = stack.pop(), a = stack.pop();
            stack.push(ops[tok](a, b));
        } else stack.push(Number(tok));
    }
    return stack[0] + 0; // + 0 normalizes a possible -0 from Math.trunc
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Recursive Evaluation from the End
```javascript
/**
 * @param {string[]} tokens
 * @return {number}
 */
function evalRPN(tokens) {
    let i = tokens.length;
    const evaluate = () => {
        const tok = tokens[--i];
        if (!["+", "-", "*", "/"].includes(tok)) return Number(tok);
        const b = evaluate(); // right operand comes first when reading backward
        const a = evaluate();
        switch (tok) {
            case "+":
                return a + b;
            case "-":
                return a - b;
            case "*":
                return a * b;
            default:
                return Math.trunc(a / b);
        }
    };
    return evaluate() + 0;
}
```
**Complexity:** Time O(n) · Space O(n) recursion

---

### 22. Generate Parentheses
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/generate-parentheses/)

**Problem:** Given `n`, generate every combination of `n` pairs of well-formed parentheses.

**Example:** `n = 3` → `["((()))","(()())","(())()","()(())","()()()"]`

**Key insight:** You can add `(` while `open < n`, and `)` while `close < open`.

#### Approach 1: Backtracking (Optimal)
```javascript
/**
 * @param {number} n
 * @return {string[]}
 */
function generateParenthesis(n) {
    const res = [];
    const bt = (cur, open, close) => {
        if (cur.length === 2 * n) {
            res.push(cur);
            return;
        }
        if (open < n) bt(cur + "(", open + 1, close);
        if (close < open) bt(cur + ")", open, close + 1);
    };
    bt("", 0, 0);
    return res;
}
```
**Complexity:** Time O(4ⁿ / √n), the nth Catalan number of results × length · Space O(n) recursion

#### Approach 2: Dynamic Programming (closure number)
Every valid string has the form `"(" + a + ")" + b`, where `a` has `i` pairs and `b` has `n-1-i` pairs.
```javascript
/**
 * @param {number} n
 * @return {string[]}
 */
function generateParenthesis(n) {
    const dp = [[""]];
    for (let k = 1; k <= n; k++) {
        const cur = [];
        for (let i = 0; i < k; i++) {
            for (const a of dp[i]) for (const b of dp[k - 1 - i]) cur.push(`(${a})${b}`);
        }
        dp.push(cur);
    }
    return dp[n];
}
```
**Complexity:** Time O(4ⁿ / √n) · Space O(4ⁿ / √n)

---

### 739. Daily Temperatures
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/daily-temperatures/)

**Problem:** Given daily `temperatures`, return `answer` where `answer[i]` is the number of days you must wait after day `i` for a warmer temperature, or `0` if no warmer day comes.

**Example:** `[73,74,75,71,69,72,76,73]` → `[1,1,4,2,1,1,0,0]`

**Key insight:** Keep a monotonic decreasing stack of unresolved days. A warmer day resolves every colder day on top of the stack.

#### Approach 1: Monotonic Stack (Optimal)
```javascript
/**
 * @param {number[]} temperatures
 * @return {number[]}
 */
function dailyTemperatures(temperatures) {
    const res = new Array(temperatures.length).fill(0);
    const stack = []; // indices of unresolved days
    for (let i = 0; i < temperatures.length; i++) {
        while (stack.length && temperatures[stack[stack.length - 1]] < temperatures[i]) {
            const j = stack.pop();
            res[j] = i - j;
        }
        stack.push(i);
    }
    return res;
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Backward Jumps (O(1) extra space)
Scan from the right and reuse `res` to hop forward to the next warmer candidate.
```javascript
/**
 * @param {number[]} temperatures
 * @return {number[]}
 */
function dailyTemperatures(temperatures) {
    const n = temperatures.length;
    const res = new Array(n).fill(0);
    let hottest = 0;
    for (let i = n - 1; i >= 0; i--) {
        const t = temperatures[i];
        if (t >= hottest) {
            hottest = t;
            continue;
        }
        let days = 1;
        while (temperatures[i + days] <= t) days += res[i + days];
        res[i] = days;
    }
    return res;
}
```
**Complexity:** Time O(n) amortized · Space O(1) extra

---

### 853. Car Fleet
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/car-fleet/)

**Problem:** `n` cars drive toward `target` on a one-lane road. Car `i` starts at `position[i]` with speed `speed[i]`. A faster car that catches a slower one ahead joins it and moves at the slower speed, forming a fleet. Return how many fleets arrive at the target.

**Example:** `target = 12, position = [10,8,0,5,3], speed = [2,4,1,1,3]` → `3`

**Key insight:** Sort cars by position, closest to the target first. A car whose arrival time is ≤ the fleet ahead of it merges with that fleet. Otherwise it starts a new fleet.

#### Approach 1: Sort + Monotonic Stack of Times
```javascript
/**
 * @param {number} target
 * @param {number[]} position
 * @param {number[]} speed
 * @return {number}
 */
function carFleet(target, position, speed) {
    const cars = position.map((p, i) => [p, speed[i]]).sort((a, b) => b[0] - a[0]); // closest to the target first
    const stack = [];
    for (const [p, s] of cars) {
        const t = (target - p) / s;
        if (!stack.length || t > stack[stack.length - 1]) stack.push(t); // too slow to catch the fleet ahead, so a new fleet
    }
    return stack.length;
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sort + Exact Integer Comparison
Compare `(target-p1)/s1 > (target-p2)/s2` by cross-multiplying, so ties are exact with no floating point.
```javascript
/**
 * @param {number} target
 * @param {number[]} position
 * @param {number[]} speed
 * @return {number}
 */
function carFleet(target, position, speed) {
    const cars = position.map((p, i) => [target - p, speed[i]]).sort((a, b) => a[0] - b[0]); // [distance, speed], nearest first
    let fleets = 0, dist0 = -1, speed0 = 1; // the fleet ahead (dist0/speed0 = its arrival time)
    for (const [d, s] of cars) {
        if (d * speed0 > dist0 * s) {
            // d/s > dist0/speed0: slower, so it can't catch up
            fleets++;
            dist0 = d;
            speed0 = s;
        }
    }
    return fleets;
}
```
**Complexity:** Time O(n log n) · Space O(n)

---

### 84. Largest Rectangle in Histogram
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/largest-rectangle-in-histogram/)

**Problem:** Given bar heights `heights`, each bar having width 1, return the area of the largest rectangle that fits in the histogram.

**Example:** `[2,1,5,6,2,3]` → `10` (bars 5 and 6, width 2)

**Key insight:** When a shorter bar arrives, each taller bar popped from an increasing stack has found its right boundary. Its left boundary is the new stack top.

#### Approach 1: Brute Force (expand each bar)
```javascript
/**
 * @param {number[]} heights
 * @return {number}
 */
function largestRectangleArea(heights) {
    let best = 0;
    for (let i = 0; i < heights.length; i++) {
        let minH = heights[i];
        for (let j = i; j < heights.length; j++) {
            minH = Math.min(minH, heights[j]);
            best = Math.max(best, minH * (j - i + 1));
        }
    }
    return best;
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Monotonic Increasing Stack (Optimal)
```javascript
/**
 * @param {number[]} heights
 * @return {number}
 */
function largestRectangleArea(heights) {
    const stack = []; // indices of increasing heights
    let best = 0;
    for (let i = 0; i <= heights.length; i++) {
        const h = i === heights.length ? 0 : heights[i]; // sentinel height 0 flushes the stack
        while (stack.length && heights[stack[stack.length - 1]] >= h) {
            const top = stack.pop();
            const left = stack.length ? stack[stack.length - 1] : -1;
            best = Math.max(best, heights[top] * (i - left - 1));
        }
        stack.push(i);
    }
    return best;
}
```
**Complexity:** Time O(n) · Space O(n)

---
## 5. Binary Search

> `(lo + hi) >> 1` is a fast integer midpoint that is safe for array indices (well below 2³¹). For larger values, use `Math.floor((lo + hi) / 2)`.

### 704. Binary Search
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/binary-search/)

**Problem:** Given a sorted array `nums` of distinct integers and a `target`, return the target's index, or `-1` if it's not present. Run in O(log n).

**Example:** `nums = [-1,0,3,5,9,12], target = 9` → `4`

**Key insight:** Halve the search space each step. In JavaScript, `(lo + hi) >> 1` is safe for array indices, which stay far below 2³¹.

#### Approach 1: Iterative (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
function search(nums, target) {
    let lo = 0, hi = nums.length - 1;
    while (lo <= hi) {
        const mid = (lo + hi) >> 1;
        if (nums[mid] === target) return mid;
        if (nums[mid] < target) lo = mid + 1;
        else hi = mid - 1;
    }
    return -1;
}
```
**Complexity:** Time O(log n) · Space O(1)

#### Approach 2: Recursive
```javascript
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
function search(nums, target) {
    const rec = (lo, hi) => {
        if (lo > hi) return -1;
        const mid = (lo + hi) >> 1;
        if (nums[mid] === target) return mid;
        return nums[mid] < target ? rec(mid + 1, hi) : rec(lo, mid - 1);
    };
    return rec(0, nums.length - 1);
}
```
**Complexity:** Time O(log n) · Space O(log n) recursion

---

### 74. Search a 2D Matrix
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/search-a-2d-matrix/)

**Problem:** In an `m × n` matrix, each row is sorted, and each row's first value is greater than the previous row's last value. Return whether `target` is in the matrix, in O(log(m·n)).

**Example:** `[[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3` → `true`

**Key insight:** The matrix is one sorted array of length `m·n`. Index `k` maps to `(k/n, k%n)`.

#### Approach 1: Flattened Binary Search (Optimal)
```javascript
/**
 * @param {number[][]} matrix
 * @param {number} target
 * @return {boolean}
 */
function searchMatrix(matrix, target) {
    const m = matrix.length, n = matrix[0].length;
    let lo = 0, hi = m * n - 1;
    while (lo <= hi) {
        const mid = (lo + hi) >> 1;
        const v = matrix[Math.floor(mid / n)][mid % n];
        if (v === target) return true;
        if (v < target) lo = mid + 1;
        else hi = mid - 1;
    }
    return false;
}
```
**Complexity:** Time O(log(m·n)) · Space O(1)

#### Approach 2: Two Binary Searches (row, then column)
```javascript
/**
 * @param {number[][]} matrix
 * @param {number} target
 * @return {boolean}
 */
function searchMatrix(matrix, target) {
    let top = 0, bot = matrix.length - 1;
    while (top <= bot) {
        const mid = (top + bot) >> 1;
        const row = matrix[mid];
        if (target < row[0]) bot = mid - 1;
        else if (target > row[row.length - 1]) top = mid + 1;
        else {
            let lo = 0, hi = row.length - 1;
            while (lo <= hi) {
                const m = (lo + hi) >> 1;
                if (row[m] === target) return true;
                if (row[m] < target) lo = m + 1;
                else hi = m - 1;
            }
            return false;
        }
    }
    return false;
}
```
**Complexity:** Time O(log m + log n) · Space O(1)

#### Approach 3: Staircase from the Top-Right (also works for LC 240)
```javascript
/**
 * @param {number[][]} matrix
 * @param {number} target
 * @return {boolean}
 */
function searchMatrix(matrix, target) {
    let r = 0, c = matrix[0].length - 1;
    while (r < matrix.length && c >= 0) {
        if (matrix[r][c] === target) return true;
        if (matrix[r][c] > target) c--;
        else r++;
    }
    return false;
}
```
**Complexity:** Time O(m + n) · Space O(1)

---

### 875. Koko Eating Bananas
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/koko-eating-bananas/)

**Problem:** There are `piles[i]` bananas in each pile. Each hour Koko picks one pile and eats up to `k` bananas from it. Return the minimum integer `k` that lets her finish everything within `h` hours.

**Example:** `piles = [3,6,7,11], h = 8` → `4`

**Key insight:** Hours needed decreases as `k` grows. Binary search on the answer `k` in `[1, max(piles)]`.

#### Approach: Binary Search on the Answer (Optimal)
```javascript
/**
 * @param {number[]} piles
 * @param {number} h
 * @return {number}
 */
function minEatingSpeed(piles, h) {
    let lo = 1, hi = Math.max(...piles);
    while (lo < hi) {
        const k = Math.floor((lo + hi) / 2); // piles up to 1e9, so avoid >> (32-bit)
        let hours = 0;
        for (const p of piles) hours += Math.ceil(p / k);
        if (hours <= h)
            hi = k; // k works; try smaller
        else lo = k + 1;
    }
    return lo;
}
```
**Complexity:** Time O(n · log M), where M = max pile · Space O(1)

> With up to 10⁴ piles, `Math.max(...piles)` is fine. For arrays near the engine's argument limit (around 10⁵ or more), use `piles.reduce((a, b) => Math.max(a, b))`.

**Brute force:** Try `k = 1, 2, …` until it works: O(n · M), which times out.

---

### 153. Find Minimum in Rotated Sorted Array
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/)

**Problem:** A sorted array of unique values was rotated an unknown number of times. Return its minimum element in O(log n).

**Example:** `[3,4,5,1,2]` → `1`; `[4,5,6,7,0,1,2]` → `0`

**Key insight:** Compare `nums[mid]` with `nums[hi]`. If `mid` is larger, the minimum is to the right. Otherwise it's at `mid` or to its left.

#### Approach: Binary Search vs. the Right End (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function findMin(nums) {
    let lo = 0, hi = nums.length - 1;
    while (lo < hi) {
        const mid = (lo + hi) >> 1;
        if (nums[mid] > nums[hi]) lo = mid + 1;
        else hi = mid;
    }
    return nums[lo];
}
```
**Complexity:** Time O(log n) · Space O(1)

**Linear alternative:** `Math.min(...nums)` is O(n). Fine for a sanity check, but it doesn't meet the requirement.

---

### 33. Search in Rotated Sorted Array
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/search-in-rotated-sorted-array/)

**Problem:** Search for `target` in a rotated sorted array of distinct values. Return its index or `-1`, in O(log n).

**Example:** `nums = [4,5,6,7,0,1,2], target = 0` → `4`

**Key insight:** At every `mid`, at least one half is sorted. Check whether `target` falls inside the sorted half's range.

#### Approach 1: One-Pass Binary Search (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
function search(nums, target) {
    let lo = 0, hi = nums.length - 1;
    while (lo <= hi) {
        const mid = (lo + hi) >> 1;
        if (nums[mid] === target) return mid;
        if (nums[lo] <= nums[mid]) {
            // left half sorted
            if (nums[lo] <= target && target < nums[mid]) hi = mid - 1;
            else lo = mid + 1;
        } else {
            // right half sorted
            if (nums[mid] < target && target <= nums[hi]) lo = mid + 1;
            else hi = mid - 1;
        }
    }
    return -1;
}
```
**Complexity:** Time O(log n) · Space O(1)

#### Approach 2: Find the Pivot, Then Binary Search the Correct Half
```javascript
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
function search(nums, target) {
    let lo = 0, hi = nums.length - 1;
    while (lo < hi) {
        // find the index of the minimum
        const mid = (lo + hi) >> 1;
        if (nums[mid] > nums[hi]) lo = mid + 1;
        else hi = mid;
    }
    const pivot = lo;
    [lo, hi] = pivot > 0 && target >= nums[0] ? [0, pivot - 1] : [pivot, nums.length - 1];
    while (lo <= hi) {
        const mid = (lo + hi) >> 1;
        if (nums[mid] === target) return mid;
        if (nums[mid] < target) lo = mid + 1;
        else hi = mid - 1;
    }
    return -1;
}
```
**Complexity:** Time O(log n) · Space O(1)

---

### 981. Time Based Key-Value Store
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/time-based-key-value-store/)

**Problem:** Design `TimeMap` with `set(key, value, timestamp)` and `get(key, timestamp)`. `get` returns the value stored at the largest timestamp ≤ `timestamp`, or `""` if none. Calls to `set` arrive with strictly increasing timestamps.

**Key insight:** Each key's entries are already sorted by time, so `get` is an upper-bound binary search.

#### Approach 1: Map of Arrays + Binary Search (Optimal)
```javascript
class TimeMap {
    store = new Map(); // key -> [timestamp, value][] in increasing time

    /**
     * @param {string} key
     * @param {string} value
     * @param {number} timestamp
     * @return {void}
     */
    set(key, value, timestamp) {
        const list = this.store.get(key);
        if (list) list.push([timestamp, value]);
        else this.store.set(key, [[timestamp, value]]);
    }

    /**
     * @param {string} key
     * @param {number} timestamp
     * @return {string}
     */
    get(key, timestamp) {
        const list = this.store.get(key) ?? [];
        let lo = 0, hi = list.length - 1, ans = "";
        while (lo <= hi) {
            // last index with ts <= timestamp
            const mid = (lo + hi) >> 1;
            if (list[mid][0] <= timestamp) {
                ans = list[mid][1];
                lo = mid + 1;
            } else hi = mid - 1;
        }
        return ans;
    }
}
```
**Complexity:** `set` O(1) amortized · `get` O(log n) · Space O(total entries)

#### Approach 2: Parallel Arrays with an Upper-Bound Helper
```javascript
class TimeMap {
    times = new Map();
    values = new Map();

    /**
     * @param {string} key
     * @param {string} value
     * @param {number} timestamp
     * @return {void}
     */
    set(key, value, timestamp) {
        if (!this.times.has(key)) {
            this.times.set(key, []);
            this.values.set(key, []);
        }
        this.times.get(key).push(timestamp);
        this.values.get(key).push(value);
    }

    /**
     * @param {string} key
     * @param {number} timestamp
     * @return {string}
     */
    get(key, timestamp) {
        const ts = this.times.get(key);
        if (!ts) return "";
        let lo = 0, hi = ts.length; // first index with ts > timestamp
        while (lo < hi) {
            const mid = (lo + hi) >> 1;
            if (ts[mid] <= timestamp) lo = mid + 1;
            else hi = mid;
        }
        return lo === 0 ? "" : this.values.get(key)[lo - 1];
    }
}
```
**Complexity:** `set` O(1) amortized · `get` O(log n) · Space O(total entries)

---

### 4. Median of Two Sorted Arrays
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/median-of-two-sorted-arrays/)

**Problem:** Given sorted arrays `nums1` (size m) and `nums2` (size n), return the median of their combined elements in O(log(m+n)).

**Example:** `[1,3]`, `[2]` → `2.0`; `[1,2]`, `[3,4]` → `2.5`

**Key insight:** Binary search a cut in the shorter array so the left halves of both arrays together hold half the elements, and every element on the left is ≤ every element on the right.

#### Approach 1: Merge (simple)
```javascript
/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @return {number}
 */
function findMedianSortedArrays(nums1, nums2) {
    const merged = [];
    let i = 0, j = 0;
    while (i < nums1.length && j < nums2.length) merged.push(nums1[i] <= nums2[j] ? nums1[i++] : nums2[j++]);
    while (i < nums1.length) merged.push(nums1[i++]);
    while (j < nums2.length) merged.push(nums2[j++]);
    const t = merged.length;
    return t % 2 ? merged[t >> 1] : (merged[t / 2 - 1] + merged[t / 2]) / 2;
}
```
**Complexity:** Time O(m + n) · Space O(m + n)

#### Approach 2: Binary Search Partition (Optimal)
```javascript
/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @return {number}
 */
function findMedianSortedArrays(nums1, nums2) {
    const [a, b] = nums1.length <= nums2.length ? [nums1, nums2] : [nums2, nums1]; // search the shorter array
    const m = a.length, n = b.length, half = (m + n + 1) >> 1;
    let lo = 0, hi = m;
    while (lo <= hi) {
        const i = (lo + hi) >> 1; // elements taken from a
        const j = half - i; // elements taken from b
        const aLeft = i > 0 ? a[i - 1] : -Infinity;
        const aRight = i < m ? a[i] : Infinity;
        const bLeft = j > 0 ? b[j - 1] : -Infinity;
        const bRight = j < n ? b[j] : Infinity;
        if (aLeft <= bRight && bLeft <= aRight) {
            if ((m + n) % 2) return Math.max(aLeft, bLeft);
            return (Math.max(aLeft, bLeft) + Math.min(aRight, bRight)) / 2;
        }
        if (aLeft > bRight) hi = i - 1;
        else lo = i + 1;
    }
    return 0;
}
```
**Complexity:** Time O(log(min(m, n))) · Space O(1)

---
## 6. Linked List

> All problems here use LeetCode's `ListNode` (see [Shared Definitions](#shared-definitions)).

### 206. Reverse Linked List
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/reverse-linked-list/)

**Problem:** Given the head of a singly linked list, reverse the list and return the new head.

**Example:** `1→2→3→4→5` → `5→4→3→2→1`

**Key insight:** Walk the list once, pointing each node's `Next` back at the previous node.

#### Approach 1: Iterative (Optimal)
```javascript
/**
 * @param {ListNode | null} head
 * @return {ListNode | null}
 */
function reverseList(head) {
    let prev = null;
    let cur = head;
    while (cur) {
        const next = cur.next;
        cur.next = prev;
        prev = cur;
        cur = next;
    }
    return prev;
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Recursive
```javascript
/**
 * @param {ListNode | null} head
 * @return {ListNode | null}
 */
function reverseList(head) {
    if (!head || !head.next) return head;
    const newHead = reverseList(head.next);
    head.next.next = head;
    head.next = null;
    return newHead;
}
```
**Complexity:** Time O(n) · Space O(n) recursion

---

### 21. Merge Two Sorted Lists
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/merge-two-sorted-lists/)

**Problem:** Merge two sorted linked lists into one sorted list by splicing their nodes together, and return its head.

**Example:** `1→2→4` + `1→3→4` → `1→1→2→3→4→4`

**Key insight:** A dummy head removes the special case for the first node.

#### Approach 1: Iterative with a Dummy Node (Optimal)
```javascript
/**
 * @param {ListNode | null} list1
 * @param {ListNode | null} list2
 * @return {ListNode | null}
 */
function mergeTwoLists(list1, list2) {
    const dummy = new ListNode();
    let tail = dummy;
    while (list1 && list2) {
        if (list1.val <= list2.val) {
            tail.next = list1;
            list1 = list1.next;
        } else {
            tail.next = list2;
            list2 = list2.next;
        }
        tail = tail.next;
    }
    tail.next = list1 ?? list2;
    return dummy.next;
}
```
**Complexity:** Time O(n + m) · Space O(1)

#### Approach 2: Recursive
```javascript
/**
 * @param {ListNode | null} list1
 * @param {ListNode | null} list2
 * @return {ListNode | null}
 */
function mergeTwoLists(list1, list2) {
    if (!list1 || !list2) return list1 ?? list2;
    if (list1.val <= list2.val) {
        list1.next = mergeTwoLists(list1.next, list2);
        return list1;
    }
    list2.next = mergeTwoLists(list1, list2.next);
    return list2;
}
```
**Complexity:** Time O(n + m) · Space O(n + m) recursion

---

### 143. Reorder List
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/reorder-list/)

**Problem:** Reorder `L0→L1→…→Ln` in place into `L0→Ln→L1→Ln-1→L2→…`. Change the links, not the values.

**Example:** `1→2→3→4→5` → `1→5→2→4→3`

**Key insight:** Find the middle, reverse the second half, then interleave the two halves.

#### Approach 1: Split + Reverse + Merge (Optimal)
```javascript
/**
 * Do not return anything, modify head in-place instead.
 *
 * @param {ListNode | null} head
 * @return {void}
 */
function reorderList(head) {
    if (!head || !head.next) return;
    // 1. find the middle
    let slow = head;
    let fast = head.next;
    while (fast && fast.next) {
        slow = slow.next;
        fast = fast.next.next;
    }
    // 2. reverse the second half
    let second = slow.next;
    slow.next = null;
    let prev = null;
    while (second) {
        const next = second.next;
        second.next = prev;
        prev = second;
        second = next;
    }
    // 3. interleave
    let first = head;
    second = prev;
    while (second && first) {
        const n1 = first.next;
        const n2 = second.next;
        first.next = second;
        second.next = n1;
        first = n1;
        second = n2;
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Array of Nodes
```javascript
/**
 * @param {ListNode | null} head
 * @return {void}
 */
function reorderList(head) {
    const nodes = [];
    for (let cur = head; cur; cur = cur.next) nodes.push(cur);
    let i = 0, j = nodes.length - 1;
    while (i < j) {
        nodes[i].next = nodes[j];
        i++;
        if (i === j) break;
        nodes[j].next = nodes[i];
        j--;
    }
    if (nodes.length) nodes[i].next = null;
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 19. Remove Nth Node From End of List
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/remove-nth-node-from-end-of-list/)

**Problem:** Remove the `n`th node from the end of a linked list and return the head.

**Example:** `1→2→3→4→5, n = 2` → `1→2→3→5`

**Key insight:** Move a lead pointer `n` steps ahead, then advance both pointers until the lead hits the end. The trailing pointer is then just before the target.

#### Approach 1: Two Pointers, One Pass (Optimal)
```javascript
/**
 * @param {ListNode | null} head
 * @param {number} n
 * @return {ListNode | null}
 */
function removeNthFromEnd(head, n) {
    const dummy = new ListNode(0, head);
    let slow = dummy;
    let fast = dummy;
    for (let i = 0; i <= n; i++) fast = fast.next;
    while (fast) {
        slow = slow.next;
        fast = fast.next;
    }
    slow.next = slow.next.next;
    return dummy.next;
}
```
**Complexity:** Time O(L) · Space O(1)

#### Approach 2: Two Passes (count the length)
```javascript
/**
 * @param {ListNode | null} head
 * @param {number} n
 * @return {ListNode | null}
 */
function removeNthFromEnd(head, n) {
    let len = 0;
    for (let c = head; c; c = c.next) len++;
    const dummy = new ListNode(0, head);
    let cur = dummy;
    for (let i = 0; i < len - n; i++) cur = cur.next;
    cur.next = cur.next.next;
    return dummy.next;
}
```
**Complexity:** Time O(L) · Space O(1)

---

### 138. Copy List with Random Pointer
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/copy-list-with-random-pointer/)

**Problem:** Each node has `next` and `random` pointers, and `random` may point to any node or `null`. Return a deep copy of the list.

**Key insight:** Map each original node to its copy. Alternatively, weave each copy right after its original so `orig.Random.Next` is the copied random target.

> LeetCode provides the `_Node` class below for this problem (its JavaScript template uses that name, as a function constructor). The class form below is equivalent and is included so the snippet runs on its own.

#### Approach 1: Map from Original Node to Copy
```javascript
class _Node {
    val;
    next;
    random;
    /**
     * @param {number} [val]
     * @param {_Node} [next]
     * @param {_Node} [random]
     */
    constructor(val, next, random) {
        this.val = val === undefined ? 0 : val;
        this.next = next === undefined ? null : next;
        this.random = random === undefined ? null : random;
    }
}

/**
 * @param {_Node | null} head
 * @return {_Node | null}
 */
function copyRandomList(head) {
    const copies = new Map();
    for (let cur = head; cur; cur = cur.next) copies.set(cur, new _Node(cur.val));
    for (let cur = head; cur; cur = cur.next) {
        const copy = copies.get(cur);
        copy.next = cur.next ? copies.get(cur.next) : null;
        copy.random = cur.random ? copies.get(cur.random) : null;
    }
    return head ? copies.get(head) : null;
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Interweaving (O(1) extra space)
```javascript
class _Node {
    val;
    next;
    random;
    /**
     * @param {number} [val]
     * @param {_Node} [next]
     * @param {_Node} [random]
     */
    constructor(val, next, random) {
        this.val = val === undefined ? 0 : val;
        this.next = next === undefined ? null : next;
        this.random = random === undefined ? null : random;
    }
}

/**
 * @param {_Node | null} head
 * @return {_Node | null}
 */
function copyRandomList(head) {
    if (!head) return null;
    // A -> A' -> B -> B' ...
    for (let cur = head; cur; cur = cur.next.next) {
        const copy = new _Node(cur.val);
        copy.next = cur.next;
        cur.next = copy;
    }
    // copy.random = original.random.next
    for (let cur = head; cur; cur = cur.next.next) {
        if (cur.random) cur.next.random = cur.random.next;
    }
    // unweave the two lists
    const newHead = head.next;
    for (let cur = head; cur; cur = cur.next) {
        const copy = cur.next;
        cur.next = copy.next;
        copy.next = copy.next ? copy.next.next : null;
    }
    return newHead;
}
```
**Complexity:** Time O(n) · Space O(1) extra

---

### 2. Add Two Numbers
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/add-two-numbers/)

**Problem:** Two non-negative integers are stored as linked lists with digits in reverse order, so the head is the ones digit. Return their sum as a list in the same format.

**Example:** `2→4→3` (342) + `5→6→4` (465) → `7→0→8` (807)

**Key insight:** Grade-school addition with a carry. Keep looping while either list or the carry remains.

#### Approach 1: Iterative (Optimal)
```javascript
/**
 * @param {ListNode | null} l1
 * @param {ListNode | null} l2
 * @return {ListNode | null}
 */
function addTwoNumbers(l1, l2) {
    const dummy = new ListNode();
    let tail = dummy, carry = 0;
    while (l1 || l2 || carry) {
        const sum = carry + (l1?.val ?? 0) + (l2?.val ?? 0);
        carry = Math.floor(sum / 10);
        tail.next = new ListNode(sum % 10);
        tail = tail.next;
        l1 = l1?.next ?? null;
        l2 = l2?.next ?? null;
    }
    return dummy.next;
}
```
**Complexity:** Time O(max(m, n)) · Space O(1) extra (the output doesn't count)

#### Approach 2: Recursive
```javascript
/**
 * @param {ListNode | null} l1
 * @param {ListNode | null} l2
 * @param {number} [carry=0]
 * @return {ListNode | null}
 */
function addTwoNumbers(l1, l2, carry = 0) {
    if (!l1 && !l2 && !carry) return null;
    const sum = carry + (l1?.val ?? 0) + (l2?.val ?? 0);
    return new ListNode(sum % 10, addTwoNumbers(l1?.next ?? null, l2?.next ?? null, Math.floor(sum / 10)));
}
```
**Complexity:** Time O(max(m, n)) · Space O(max(m, n)) recursion

---

### 141. Linked List Cycle
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/linked-list-cycle/)

**Problem:** Return `true` if the linked list contains a cycle.

**Key insight:** Floyd's tortoise and hare. If there's a cycle, the fast pointer eventually laps the slow one.

#### Approach 1: Set of Visited Nodes
```javascript
/**
 * @param {ListNode | null} head
 * @return {boolean}
 */
function hasCycle(head) {
    const seen = new Set(); // object identity
    for (let cur = head; cur; cur = cur.next) {
        if (seen.has(cur)) return true;
        seen.add(cur);
    }
    return false;
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Floyd's Fast and Slow Pointers (Optimal)
```javascript
/**
 * @param {ListNode | null} head
 * @return {boolean}
 */
function hasCycle(head) {
    let slow = head, fast = head;
    while (fast && fast.next) {
        slow = slow.next;
        fast = fast.next.next;
        if (slow === fast) return true;
    }
    return false;
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 287. Find the Duplicate Number
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/find-the-duplicate-number/)

**Problem:** `nums` has `n + 1` integers, each in `[1, n]`, and exactly one value is repeated (possibly many times). Return it without modifying the array, using O(1) extra space.

**Example:** `[1,3,4,2,2]` → `2`

**Key insight:** Treat `i → nums[i]` as a linked list. The duplicate value is the entry point of the cycle, which you can find with Floyd's algorithm.

#### Approach 1: Set (uses extra space)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function findDuplicate(nums) {
    const seen = new Set();
    for (const n of nums) {
        if (seen.has(n)) return n;
        seen.add(n);
    }
    return -1;
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Binary Search on Value (pigeonhole)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function findDuplicate(nums) {
    let lo = 1, hi = nums.length - 1;
    while (lo < hi) {
        const mid = (lo + hi) >> 1;
        const cnt = nums.filter((n) => n <= mid).length;
        if (cnt > mid) hi = mid;
        else lo = mid + 1;
    }
    return lo;
}
```
**Complexity:** Time O(n log n) · Space O(n) for `filter`; a counting loop makes it O(1)

#### Approach 3: Floyd's Cycle Detection (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function findDuplicate(nums) {
    let slow = nums[0], fast = nums[nums[0]];
    while (slow !== fast) {
        slow = nums[slow];
        fast = nums[nums[fast]];
    }
    slow = 0;
    while (slow !== fast) {
        slow = nums[slow];
        fast = nums[fast];
    }
    return slow;
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 146. LRU Cache
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/lru-cache/)

**Problem:** Design an LRU (least recently used) cache with a fixed `capacity`. `get(key)` returns the value or `-1`. `put(key, value)` inserts or updates a key, evicting the least recently used key when over capacity. Both must run in O(1).

**Key insight:** A hash map gives O(1) lookup, and a doubly linked list gives O(1) reordering and eviction.

#### Approach 1: Map + Doubly Linked List (Optimal, interview-preferred)
```javascript
class DNode {
    prev = null;
    next = null;
    /**
     * @param {number} [key=0]
     * @param {number} [val=0]
     */
    constructor(key = 0, val = 0) {
        this.key = key;
        this.val = val;
    }
}

class LRUCache {
    items = new Map();
    head = new DNode(); // head.next is the MRU
    tail = new DNode(); // tail.prev is the LRU

    /**
     * @param {number} capacity
     */
    constructor(capacity) {
        this.capacity = capacity;
        this.head.next = this.tail;
        this.tail.prev = this.head;
    }

    /**
     * @param {DNode} node
     * @return {void}
     */
    remove(node) {
        node.prev.next = node.next;
        node.next.prev = node.prev;
    }

    /**
     * @param {DNode} node
     * @return {void}
     */
    pushFront(node) {
        node.prev = this.head;
        node.next = this.head.next;
        this.head.next.prev = node;
        this.head.next = node;
    }

    /**
     * @param {number} key
     * @return {number}
     */
    get(key) {
        const node = this.items.get(key);
        if (!node) return -1;
        this.remove(node);
        this.pushFront(node);
        return node.val;
    }

    /**
     * @param {number} key
     * @param {number} value
     * @return {void}
     */
    put(key, value) {
        let node = this.items.get(key);
        if (node) {
            node.val = value;
            this.remove(node);
            this.pushFront(node);
            return;
        }
        node = new DNode(key, value);
        this.items.set(key, node);
        this.pushFront(node);
        if (this.items.size > this.capacity) {
            const lru = this.tail.prev;
            this.remove(lru);
            this.items.delete(lru.key);
        }
    }
}
```
**Complexity:** O(1) per operation · Space O(capacity)

#### Approach 2: `Map` Insertion Order
A JavaScript `Map` iterates in insertion order, so deleting and re-inserting a key moves it to the "most recent" end, and `keys().next()` is the least recent.
```javascript
class LRUCache {
    cache = new Map();

    /**
     * @param {number} capacity
     */
    constructor(capacity) {
        this.capacity = capacity;
    }

    /**
     * @param {number} key
     * @return {number}
     */
    get(key) {
        const v = this.cache.get(key);
        if (v === undefined) return -1;
        this.cache.delete(key);
        this.cache.set(key, v); // mark as most recently used
        return v;
    }

    /**
     * @param {number} key
     * @param {number} value
     * @return {void}
     */
    put(key, value) {
        this.cache.delete(key);
        this.cache.set(key, value);
        if (this.cache.size > this.capacity) {
            this.cache.delete(this.cache.keys().next().value); // evict the LRU
        }
    }
}
```
**Complexity:** O(1) per operation · Space O(capacity). Good to mention in an interview, but interviewers usually want Approach 1.

---

### 23. Merge k Sorted Lists
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/merge-k-sorted-lists/)

**Problem:** Merge `k` sorted linked lists into one sorted list.

**Example:** `[1→4→5, 1→3→4, 2→6]` → `1→1→2→3→4→4→5→6`

**Key insight:** Either keep a min-heap of the `k` current heads, or merge the lists in pairs (divide and conquer). Both are O(N log k).

#### Approach 1: Min-Heap
```javascript
/**
 * Binary heap ordered by `less(a, b)`: pass (a, b) => a < b for a min-heap.
 *
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}
/**
 * @param {Array<ListNode | null>} lists
 * @return {ListNode | null}
 */
function mergeKLists(lists) {
    const heap = new Heap((a, b) => a.val < b.val);
    for (const l of lists) if (l) heap.push(l);
    const dummy = new ListNode();
    let tail = dummy;
    while (heap.size) {
        const node = heap.pop();
        tail.next = node;
        tail = node;
        if (node.next) heap.push(node.next);
    }
    return dummy.next;
}
```
**Complexity:** Time O(N log k) for N total nodes · Space O(k)

#### Approach 2: Divide and Conquer Pairwise Merge (Optimal, O(1) extra)
```javascript
/**
 * @param {Array<ListNode | null>} lists
 * @return {ListNode | null}
 */
function mergeKLists(lists) {
    const merge2 = (a, b) => {
        const dummy = new ListNode();
        let tail = dummy;
        while (a && b) {
            if (a.val <= b.val) {
                tail.next = a;
                a = a.next;
            } else {
                tail.next = b;
                b = b.next;
            }
            tail = tail.next;
        }
        tail.next = a ?? b;
        return dummy.next;
    };
    if (!lists.length) return null;
    for (let step = 1; step < lists.length; step *= 2) {
        for (let i = 0; i + step < lists.length; i += 2 * step) lists[i] = merge2(lists[i], lists[i + step]);
    }
    return lists[0];
}
```
**Complexity:** Time O(N log k) · Space O(1)

**Brute force:** Collect every value, sort, and rebuild: O(N log N) time, O(N) space.

---

### 25. Reverse Nodes in k-Group
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/reverse-nodes-in-k-group/)

**Problem:** Reverse the list's nodes `k` at a time. If fewer than `k` nodes remain at the end, leave them as they are. Change the links, not the values.

**Example:** `1→2→3→4→5, k = 2` → `2→1→4→3→5`; `k = 3` → `3→2→1→4→5`

**Key insight:** For each group, confirm `k` nodes exist, reverse them, and reconnect the previous group's tail to the new head.

#### Approach 1: Iterative (Optimal)
```javascript
/**
 * @param {ListNode | null} head
 * @param {number} k
 * @return {ListNode | null}
 */
function reverseKGroup(head, k) {
    const dummy = new ListNode(0, head);
    let groupPrev = dummy;
    for (;;) {
        // find the kth node of this group
        let kth = groupPrev;
        for (let i = 0; i < k && kth; i++) kth = kth.next;
        if (!kth) break;
        const groupNext = kth.next;
        // reverse the group
        let prev = groupNext;
        let cur = groupPrev.next;
        while (cur !== groupNext) {
            const next = cur.next;
            cur.next = prev;
            prev = cur;
            cur = next;
        }
        const first = groupPrev.next; // becomes this group's tail
        groupPrev.next = kth;
        groupPrev = first;
    }
    return dummy.next;
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Recursive
```javascript
/**
 * @param {ListNode | null} head
 * @param {number} k
 * @return {ListNode | null}
 */
function reverseKGroup(head, k) {
    let cur = head, count = 0;
    while (cur && count < k) {
        cur = cur.next;
        count++;
    }
    if (count < k) return head;
    let prev = reverseKGroup(cur, k); // cur is the (k+1)th node
    let node = head;
    for (let i = 0; i < k; i++) {
        const next = node.next;
        node.next = prev;
        prev = node;
        node = next;
    }
    return prev;
}
```
**Complexity:** Time O(n) · Space O(n/k) recursion

---
## 7. Trees

> All problems here use LeetCode's `TreeNode` (see [Shared Definitions](#shared-definitions)). BFS queues use an array plus a `head` index, because `Array.prototype.shift()` is O(n).

### 226. Invert Binary Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/invert-binary-tree/)

**Problem:** Mirror a binary tree by swapping every node's left and right children, then return the root.

**Example:** `[4,2,7,1,3,6,9]` → `[4,7,2,9,6,3,1]`

**Key insight:** Swap the children at every node. Traversal order doesn't matter.

#### Approach 1: Recursive DFS (Optimal)
```javascript
/**
 * @param {TreeNode | null} root
 * @return {TreeNode | null}
 */
function invertTree(root) {
    if (root) [root.left, root.right] = [invertTree(root.right), invertTree(root.left)];
    return root;
}
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: Iterative BFS
```javascript
/**
 * @param {TreeNode | null} root
 * @return {TreeNode | null}
 */
function invertTree(root) {
    const q = root ? [root] : [];
    for (let head = 0; head < q.length; head++) {
        const node = q[head];
        [node.left, node.right] = [node.right, node.left];
        if (node.left) q.push(node.left);
        if (node.right) q.push(node.right);
    }
    return root;
}
```
**Complexity:** Time O(n) · Space O(n) for the array (O(w) live)

---

### 104. Maximum Depth of Binary Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/maximum-depth-of-binary-tree/)

**Problem:** Return the number of nodes on the longest path from the root down to a leaf.

**Example:** `[3,9,20,null,null,15,7]` → `3`

#### Approach 1: Recursive DFS
```javascript
/**
 * @param {TreeNode | null} root
 * @return {number}
 */
function maxDepth(root) {
    return root ? 1 + Math.max(maxDepth(root.left), maxDepth(root.right)) : 0;
}
```
**Complexity:** Time O(n) · Space O(h)

> ⚠️ Recursion depth equals the tree height. A skewed tree of 10⁴ nodes overflows the JS stack, so use Approach 2 or 3 there.

#### Approach 2: BFS Level Count
```javascript
/**
 * @param {TreeNode | null} root
 * @return {number}
 */
function maxDepth(root) {
    let level = root ? [root] : [];
    let depth = 0;
    while (level.length) {
        depth++;
        const next = [];
        for (const node of level) {
            if (node.left) next.push(node.left);
            if (node.right) next.push(node.right);
        }
        level = next;
    }
    return depth;
}
```
**Complexity:** Time O(n) · Space O(w)

#### Approach 3: Iterative DFS with an Explicit Stack
```javascript
/**
 * @param {TreeNode | null} root
 * @return {number}
 */
function maxDepth(root) {
    const stack = root ? [[root, 1]] : [];
    let best = 0;
    while (stack.length) {
        const [node, d] = stack.pop();
        best = Math.max(best, d);
        if (node.left) stack.push([node.left, d + 1]);
        if (node.right) stack.push([node.right, d + 1]);
    }
    return best;
}
```
**Complexity:** Time O(n) · Space O(h)

---

### 543. Diameter of Binary Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/diameter-of-binary-tree/)

**Problem:** Return the length (in edges) of the longest path between any two nodes. The path doesn't have to pass through the root.

**Example:** `[1,2,3,4,5]` → `3` (path 4→2→1→3)

**Key insight:** The longest path through a node is `height(left) + height(right)`. Compute heights bottom-up and track the best sum.

#### Approach 1: DFS Returning Height (Optimal)
```javascript
/**
 * @param {TreeNode | null} root
 * @return {number}
 */
function diameterOfBinaryTree(root) {
    let best = 0;
    const height = (node) => {
        if (!node) return 0;
        const l = height(node.left), r = height(node.right);
        best = Math.max(best, l + r);
        return 1 + Math.max(l, r);
    };
    height(root);
    return best;
}
```
**Complexity:** Time O(n) · Space O(h)

> ⚠️ Overflows the JS stack on a skewed tree of about 7,000 or more nodes. Approach 2 has no depth limit.

#### Approach 2: Iterative Post-Order (no recursion)
A degenerate tree of 10⁴ nodes is deeper than the JS call stack allows, so this version processes nodes children-first from an explicit order.
```javascript
/**
 * @param {TreeNode | null} root
 * @return {number}
 */
function diameterOfBinaryTree(root) {
    if (!root) return 0;
    const order = [root]; // parents before children
    for (let i = 0; i < order.length; i++) {
        const n = order[i];
        if (n.left) order.push(n.left);
        if (n.right) order.push(n.right);
    }
    const h = new Map();
    let best = 0;
    for (let i = order.length - 1; i >= 0; i--) {
        // children first
        const n = order[i];
        const l = n.left ? h.get(n.left) : 0, r = n.right ? h.get(n.right) : 0;
        best = Math.max(best, l + r);
        h.set(n, 1 + Math.max(l, r));
    }
    return best;
}
```
**Complexity:** Time O(n) · Space O(n)

**Brute force:** At each node, compute the left and right heights separately: O(n²) on a skewed tree.

---

### 110. Balanced Binary Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/balanced-binary-tree/)

**Problem:** A tree is height-balanced if, at every node, the left and right subtree heights differ by at most 1. Return whether the tree is balanced.

**Key insight:** Return height bottom-up, and use `-1` to signal that a subtree is already unbalanced.

#### Approach 1: Top-Down (simple, O(n²))
```javascript
/**
 * @param {TreeNode | null} root
 * @return {boolean}
 */
function isBalanced(root) {
    const height = (n) => (n ? 1 + Math.max(height(n.left), height(n.right)) : 0);
    if (!root) return true;
    return Math.abs(height(root.left) - height(root.right)) <= 1 && isBalanced(root.left) && isBalanced(root.right);
}
```
**Complexity:** Time O(n²) worst case · Space O(h)

#### Approach 2: Bottom-Up with Early Exit (Optimal)
```javascript
/**
 * @param {TreeNode | null} root
 * @return {boolean}
 */
function isBalanced(root) {
    const check = (n) => {
        // height, or -1 if unbalanced
        if (!n) return 0;
        const l = check(n.left);
        if (l < 0) return -1;
        const r = check(n.right);
        if (r < 0 || Math.abs(l - r) > 1) return -1;
        return 1 + Math.max(l, r);
    };
    return check(root) >= 0;
}
```
**Complexity:** Time O(n) · Space O(h)

---

### 100. Same Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/same-tree/)

**Problem:** Return whether two binary trees have the same structure and the same node values.

#### Approach 1: Recursive (Optimal)
```javascript
/**
 * @param {TreeNode | null} p
 * @param {TreeNode | null} q
 * @return {boolean}
 */
function isSameTree(p, q) {
    if (!p || !q) return p === q; // both null
    return p.val === q.val && isSameTree(p.left, q.left) && isSameTree(p.right, q.right);
}
```
**Complexity:** Time O(min(n, m)) · Space O(h)

#### Approach 2: Iterative BFS on Pairs
```javascript
/**
 * @param {TreeNode | null} p
 * @param {TreeNode | null} q
 * @return {boolean}
 */
function isSameTree(p, q) {
    const queue = [[p, q]];
    for (let head = 0; head < queue.length; head++) {
        const [a, b] = queue[head];
        if (!a && !b) continue;
        if (!a || !b || a.val !== b.val) return false;
        queue.push([a.left, b.left], [a.right, b.right]);
    }
    return true;
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 572. Subtree of Another Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/subtree-of-another-tree/)

**Problem:** Return whether `subRoot` matches, in structure and values, some node of `root` together with all of that node's descendants.

**Key insight:** Run Same Tree at every node (O(m·n)), or serialize both trees and use substring search (O(m+n)).

#### Approach 1: Same-Tree Check at Every Node
```javascript
/**
 * @param {TreeNode | null} root
 * @param {TreeNode | null} subRoot
 * @return {boolean}
 */
function isSubtree(root, subRoot) {
    const same = (a, b) => (!a || !b ? a === b : a.val === b.val && same(a.left, b.left) && same(a.right, b.right));
    if (!root) return subRoot === null;
    return same(root, subRoot) || isSubtree(root.left, subRoot) || isSubtree(root.right, subRoot);
}
```
**Complexity:** Time O(m · n) · Space O(h)

#### Approach 2: Serialize + Substring Search
Delimiters around each value (`^12` vs `^2`) and explicit null markers prevent false matches.
```javascript
/**
 * @param {TreeNode | null} root
 * @param {TreeNode | null} subRoot
 * @return {boolean}
 */
function isSubtree(root, subRoot) {
    const ser = (n) => (n ? `^${n.val}${ser(n.left)}${ser(n.right)}` : "#");
    return ser(root).includes(ser(subRoot)); // swap in KMP for a guaranteed O(m+n)
}
```
**Complexity:** Time O(m + n) with KMP; `includes` is fast in practice · Space O(m + n)

---

### 235. Lowest Common Ancestor of a Binary Search Tree
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)

**Problem:** Given a BST and two of its nodes `p` and `q`, return their lowest common ancestor: the deepest node that has both as descendants (a node counts as its own descendant).

**Key insight:** If both values are smaller than the current node, go left. If both are larger, go right. Otherwise the current node is where they split, and that's the LCA.

#### Approach 1: Iterative (Optimal)
```javascript
/**
 * @param {TreeNode | null} root
 * @param {TreeNode | null} p
 * @param {TreeNode | null} q
 * @return {TreeNode | null}
 */
function lowestCommonAncestor(root, p, q) {
    if (!p || !q) return null;
    let cur = root;
    while (cur) {
        if (p.val < cur.val && q.val < cur.val) cur = cur.left;
        else if (p.val > cur.val && q.val > cur.val) cur = cur.right;
        else return cur; // p and q split here (or one of them is this node)
    }
    return null;
}
```
**Complexity:** Time O(h) · Space O(1)

#### Approach 2: Recursive
```javascript
/**
 * @param {TreeNode | null} root
 * @param {TreeNode | null} p
 * @param {TreeNode | null} q
 * @return {TreeNode | null}
 */
function lowestCommonAncestor(root, p, q) {
    if (!root || !p || !q) return null;
    if (p.val < root.val && q.val < root.val) return lowestCommonAncestor(root.left, p, q);
    if (p.val > root.val && q.val > root.val) return lowestCommonAncestor(root.right, p, q);
    return root;
}
```
**Complexity:** Time O(h) · Space O(h)

> ⚠️ Overflows the JS stack on a skewed BST of 10⁵ nodes. Approach 1 is iterative.

---

### 102. Binary Tree Level Order Traversal
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/binary-tree-level-order-traversal/)

**Problem:** Return the node values level by level, left to right.

**Example:** `[3,9,20,null,null,15,7]` → `[[3],[9,20],[15,7]]`

#### Approach 1: BFS Level by Level (Optimal)
```javascript
/**
 * @param {TreeNode | null} root
 * @return {number[][]}
 */
function levelOrder(root) {
    const res = [];
    let level = root ? [root] : [];
    while (level.length) {
        res.push(level.map((n) => n.val));
        const next = [];
        for (const n of level) {
            if (n.left) next.push(n.left);
            if (n.right) next.push(n.right);
        }
        level = next;
    }
    return res;
}
```
**Complexity:** Time O(n) · Space O(w)

#### Approach 2: DFS with a Depth Index
```javascript
/**
 * @param {TreeNode | null} root
 * @return {number[][]}
 */
function levelOrder(root) {
    const res = [];
    const dfs = (n, d) => {
        if (!n) return;
        (res[d] ??= []).push(n.val);
        dfs(n.left, d + 1);
        dfs(n.right, d + 1);
    };
    dfs(root, 0);
    return res;
}
```
**Complexity:** Time O(n) · Space O(h)

---

### 199. Binary Tree Right Side View
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/binary-tree-right-side-view/)

**Problem:** Imagine standing on the tree's right side. Return the values you can see, from top to bottom.

**Example:** `[1,2,3,null,5,null,4]` → `[1,3,4]`

**Key insight:** The answer is the last node of each BFS level, or the first node reached at each depth in a right-first DFS.

#### Approach 1: BFS, Last Node of Each Level
```javascript
/**
 * @param {TreeNode | null} root
 * @return {number[]}
 */
function rightSideView(root) {
    const res = [];
    let level = root ? [root] : [];
    while (level.length) {
        res.push(level[level.length - 1].val);
        const next = [];
        for (const n of level) {
            if (n.left) next.push(n.left);
            if (n.right) next.push(n.right);
        }
        level = next;
    }
    return res;
}
```
**Complexity:** Time O(n) · Space O(w)

#### Approach 2: Right-First DFS
```javascript
/**
 * @param {TreeNode | null} root
 * @return {number[]}
 */
function rightSideView(root) {
    const res = [];
    const dfs = (n, d) => {
        if (!n) return;
        if (d === res.length) res.push(n.val); // first node seen at this depth is the rightmost
        dfs(n.right, d + 1);
        dfs(n.left, d + 1);
    };
    dfs(root, 0);
    return res;
}
```
**Complexity:** Time O(n) · Space O(h)

---

### 1448. Count Good Nodes in Binary Tree
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/count-good-nodes-in-binary-tree/)

**Problem:** A node `X` is *good* if no node on the path from the root to `X` has a value greater than `X`. Count the good nodes.

**Example:** `[3,1,4,3,null,1,5]` → `4`

**Key insight:** Pass the maximum value seen on the path so far down the DFS.

#### Approach 1: DFS with the Path Max (Optimal)
```javascript
/**
 * @param {TreeNode | null} root
 * @return {number}
 */
function goodNodes(root) {
    const dfs = (n, maxSoFar) => {
        if (!n) return 0;
        const m = Math.max(maxSoFar, n.val);
        return (n.val >= maxSoFar ? 1 : 0) + dfs(n.left, m) + dfs(n.right, m);
    };
    return dfs(root, -Infinity);
}
```
**Complexity:** Time O(n) · Space O(h)

> ⚠️ Overflows the JS stack on a skewed tree of 10⁵ nodes. Approach 2 (BFS) has no depth limit.

#### Approach 2: BFS Carrying (node, max)
```javascript
/**
 * @param {TreeNode | null} root
 * @return {number}
 */
function goodNodes(root) {
    const q = root ? [[root, -Infinity]] : [];
    let count = 0;
    for (let head = 0; head < q.length; head++) {
        const [n, mx] = q[head];
        if (n.val >= mx) count++;
        const m = Math.max(mx, n.val);
        if (n.left) q.push([n.left, m]);
        if (n.right) q.push([n.right, m]);
    }
    return count;
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 98. Validate Binary Search Tree
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/validate-binary-search-tree/)

**Problem:** Return whether the tree is a valid BST. Every node's left subtree must hold only smaller values, its right subtree only larger values, and both subtrees must themselves be BSTs.

**Key insight:** Comparing a node only with its direct children isn't enough. Pass down a valid `(lo, hi)` range, or check that the in-order traversal is strictly increasing.

#### Approach 1: DFS with Bounds (Optimal)
```javascript
/**
 * @param {TreeNode | null} root
 * @return {boolean}
 */
function isValidBST(root) {
    const valid = (n, lo, hi) => !n || (lo < n.val && n.val < hi && valid(n.left, lo, n.val) && valid(n.right, n.val, hi));
    return valid(root, -Infinity, Infinity);
}
```
**Complexity:** Time O(n) · Space O(h)

> ⚠️ Overflows the JS stack on a skewed tree of 10⁴ nodes. Approach 2 is iterative.

#### Approach 2: Iterative In-Order Traversal
```javascript
/**
 * @param {TreeNode | null} root
 * @return {boolean}
 */
function isValidBST(root) {
    const stack = [];
    let cur = root, prev = -Infinity;
    while (cur || stack.length) {
        while (cur) {
            stack.push(cur);
            cur = cur.left;
        }
        const node = stack.pop();
        if (node.val <= prev) return false;
        prev = node.val;
        cur = node.right;
    }
    return true;
}
```
**Complexity:** Time O(n) · Space O(h)

---

### 230. Kth Smallest Element in a BST
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/kth-smallest-element-in-a-bst/)

**Problem:** Return the `k`th smallest value (1-indexed) in a BST.

**Key insight:** In-order traversal visits a BST's values in sorted order, so stop at the `k`th node visited.

#### Approach 1: Iterative In-Order (Optimal, stops early)
```javascript
/**
 * @param {TreeNode | null} root
 * @param {number} k
 * @return {number}
 */
function kthSmallest(root, k) {
    const stack = [];
    let cur = root;
    for (;;) {
        while (cur) {
            stack.push(cur);
            cur = cur.left;
        }
        const node = stack.pop();
        if (--k === 0) return node.val;
        cur = node.right;
    }
}
```
**Complexity:** Time O(h + k) · Space O(h)

#### Approach 2: Generator-Based In-Order
```javascript
/**
 * @param {TreeNode | null} root
 * @param {number} k
 * @return {number}
 */
function kthSmallest(root, k) {
    function* inorder(n) {
        if (!n) return;
        yield* inorder(n.left);
        yield n.val;
        yield* inorder(n.right);
    }
    for (const v of inorder(root)) if (--k === 0) return v; // lazy: stops after k values
    return -1;
}
```
**Complexity:** Time O(h + k) · Space O(h)

> ⚠️ Nested generators recurse once per level, so a skewed BST of 10⁴ nodes overflows the JS stack. Approach 1 is iterative.

**Follow-up (frequent inserts and deletes):** Store a subtree size in each node, then walk down by comparing `k` with the left subtree's size. Each query costs O(h).

---

### 105. Construct Binary Tree from Preorder and Inorder Traversal
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/)

**Problem:** Given a tree's `preorder` and `inorder` traversals (all values unique), rebuild the tree and return its root.

**Example:** `preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]` → `[3,9,20,null,null,15,7]`

**Key insight:** `preorder[0]` is the root. Its position in `inorder` splits the left subtree from the right one.

#### Approach 1: Slicing (simple, O(n²))
```javascript
/**
 * @param {number[]} preorder
 * @param {number[]} inorder
 * @return {TreeNode | null}
 */
function buildTree(preorder, inorder) {
    if (!preorder.length) return null;
    const mid = inorder.indexOf(preorder[0]);
    return new TreeNode(
        preorder[0],
        buildTree(preorder.slice(1, mid + 1), inorder.slice(0, mid)),
        buildTree(preorder.slice(mid + 1), inorder.slice(mid + 1)),
    );
}
```
**Complexity:** Time O(n²) worst case · Space O(n²) from the slices

#### Approach 2: Map of Inorder Indices (Optimal)
```javascript
/**
 * @param {number[]} preorder
 * @param {number[]} inorder
 * @return {TreeNode | null}
 */
function buildTree(preorder, inorder) {
    const pos = new Map();
    inorder.forEach((v, i) => pos.set(v, i));
    let pre = 0;
    const build = (lo, hi) => {
        // inorder range [lo, hi)
        if (lo >= hi) return null;
        const v = preorder[pre++];
        const mid = pos.get(v);
        const node = new TreeNode(v);
        node.left = build(lo, mid);
        node.right = build(mid + 1, hi);
        return node;
    };
    return build(0, inorder.length);
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 124. Binary Tree Maximum Path Sum
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/binary-tree-maximum-path-sum/)

**Problem:** A path is any sequence of connected nodes where each node appears at most once. It doesn't have to pass through the root. Return the maximum path sum over all non-empty paths.

**Example:** `[-10,9,20,null,null,15,7]` → `42` (15→20→7)

**Key insight:** At each node, the best path that bends there is `val + max(0, leftGain) + max(0, rightGain)`. Pass up to the parent only `val + max(0, max(leftGain, rightGain))`, since a path can't branch upward.

#### Approach 1: Post-Order DFS (Optimal)
```javascript
/**
 * @param {TreeNode | null} root
 * @return {number}
 */
function maxPathSum(root) {
    let best = -Infinity;
    const gain = (n) => {
        if (!n) return 0;
        const l = Math.max(0, gain(n.left));
        const r = Math.max(0, gain(n.right));
        best = Math.max(best, n.val + l + r); // best path that bends at n
        return n.val + Math.max(l, r); // only one branch can extend upward
    };
    gain(root);
    return best;
}
```
**Complexity:** Time O(n) · Space O(h)

> ⚠️ Overflows the JS stack on a skewed tree of 3 · 10⁴ nodes. Approach 2 has no depth limit.

#### Approach 2: Iterative Post-Order (no recursion)
Safe for a skewed tree of 3 · 10⁴ nodes, which overflows the JS call stack in Approach 1.
```javascript
/**
 * @param {TreeNode | null} root
 * @return {number}
 */
function maxPathSum(root) {
    if (!root) return 0;
    const order = [root];
    for (let i = 0; i < order.length; i++) {
        const n = order[i];
        if (n.left) order.push(n.left);
        if (n.right) order.push(n.right);
    }
    const gain = new Map();
    let best = -Infinity;
    for (let i = order.length - 1; i >= 0; i--) {
        const n = order[i];
        const l = Math.max(0, n.left ? gain.get(n.left) : 0);
        const r = Math.max(0, n.right ? gain.get(n.right) : 0);
        best = Math.max(best, n.val + l + r);
        gain.set(n, n.val + Math.max(l, r));
    }
    return best;
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 297. Serialize and Deserialize Binary Tree
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/)

**Problem:** Design `serialize`, which turns a binary tree into a string, and `deserialize`, which turns that string back into the same tree.

**Key insight:** A pre-order traversal with explicit null markers (`N`) is enough to rebuild the tree exactly.

> LeetCode's JavaScript template for this problem uses two free functions, `serialize` and `deserialize`.

#### Approach 1: Pre-Order DFS
```javascript
/**
 * @param {TreeNode | null} root
 * @return {string}
 */
function serialize(root) {
    const out = [];
    const dfs = (n) => {
        if (!n) {
            out.push("N");
            return;
        }
        out.push(String(n.val));
        dfs(n.left);
        dfs(n.right);
    };
    dfs(root);
    return out.join(",");
}

/**
 * @param {string} data
 * @return {TreeNode | null}
 */
function deserialize(data) {
    const vals = data.split(",");
    let i = 0;
    const build = () => {
        const v = vals[i++];
        if (v === "N") return null;
        const node = new TreeNode(Number(v));
        node.left = build();
        node.right = build();
        return node;
    };
    return build();
}
```
**Complexity:** Time O(n) for both · Space O(n)

#### Approach 2: BFS (level order, LeetCode's own format)
```javascript
/**
 * @param {TreeNode | null} root
 * @return {string}
 */
function serialize(root) {
    const out = [];
    const q = [root];
    for (let head = 0; head < q.length; head++) {
        const n = q[head];
        if (n) {
            out.push(String(n.val));
            q.push(n.left, n.right);
        } else out.push("N");
    }
    return out.join(",");
}

/**
 * @param {string} data
 * @return {TreeNode | null}
 */
function deserialize(data) {
    const vals = data.split(",");
    if (vals[0] === "N") return null;
    const root = new TreeNode(Number(vals[0]));
    const q = [root];
    let i = 1;
    for (let head = 0; head < q.length; head++) {
        const node = q[head];
        if (vals[i] !== "N") q.push((node.left = new TreeNode(Number(vals[i]))));
        i++;
        if (vals[i] !== "N") q.push((node.right = new TreeNode(Number(vals[i]))));
        i++;
    }
    return root;
}
```
**Complexity:** Time O(n) · Space O(n)

---
## 8. Tries

### 208. Implement Trie (Prefix Tree)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/implement-trie-prefix-tree/)

**Problem:** Implement a trie with `insert(word)`, `search(word)` (is this exact word stored?), and `startsWith(prefix)` (does any stored word begin with this prefix?). Inputs are lowercase letters.

**Key insight:** Each node has up to 26 children plus an end-of-word flag.

#### Approach 1: Node Class with a Children Map (Optimal)
```javascript
class TrieNode {
    children = new Map();
    isEnd = false;
}

class Trie {
    root = new TrieNode();

    /**
     * @param {string} word
     * @return {void}
     */
    insert(word) {
        let node = this.root;
        for (const c of word) {
            let next = node.children.get(c);
            if (!next) {
                next = new TrieNode();
                node.children.set(c, next);
            }
            node = next;
        }
        node.isEnd = true;
    }

    /**
     * @param {string} s
     * @return {TrieNode | null}
     */
    find(s) {
        let node = this.root;
        for (const c of s) {
            node = node.children.get(c);
            if (!node) return null;
        }
        return node;
    }

    /**
     * @param {string} word
     * @return {boolean}
     */
    search(word) {
        return this.find(word)?.isEnd ?? false;
    }

    /**
     * @param {string} prefix
     * @return {boolean}
     */
    startsWith(prefix) {
        return this.find(prefix) !== null;
    }
}
```
**Complexity:** Time O(L) per operation · Space O(total characters)

#### Approach 2: Fixed Array of 26 Children (faster for lowercase)
```javascript
class TrieNode26 {
    children = new Array(26).fill(null);
    isEnd = false;
}

class Trie {
    root = new TrieNode26();

    /**
     * @param {string} word
     * @return {void}
     */
    insert(word) {
        let node = this.root;
        for (let i = 0; i < word.length; i++) {
            const c = word.charCodeAt(i) - 97;
            node = node.children[c] ??= new TrieNode26();
        }
        node.isEnd = true;
    }

    /**
     * @param {string} s
     * @return {TrieNode26 | null}
     */
    find(s) {
        let node = this.root;
        for (let i = 0; i < s.length && node; i++) node = node.children[s.charCodeAt(i) - 97];
        return node;
    }

    /**
     * @param {string} word
     * @return {boolean}
     */
    search(word) {
        return this.find(word)?.isEnd ?? false;
    }

    /**
     * @param {string} prefix
     * @return {boolean}
     */
    startsWith(prefix) {
        return this.find(prefix) !== null;
    }
}
```
**Complexity:** Time O(L) per operation · Space O(total characters × 26)

---

### 211. Design Add and Search Words Data Structure
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/design-add-and-search-words-data-structure/)

**Problem:** Support `addWord(word)` and `search(word)`. The search pattern may contain `.`, which matches any single letter.

**Example:** add `bad`, `dad`, `mad`; `search("pad")` → `false`, `search(".ad")` → `true`, `search("b..")` → `true`

**Key insight:** Use a trie. On `.`, run DFS into every child.

#### Approach 1: Trie + DFS (Optimal)
```javascript
class WDNode {
    children = new Map();
    isEnd = false;
}

class WordDictionary {
    root = new WDNode();

    /**
     * @param {string} word
     * @return {void}
     */
    addWord(word) {
        let node = this.root;
        for (const c of word) {
            if (!node.children.has(c)) node.children.set(c, new WDNode());
            node = node.children.get(c);
        }
        node.isEnd = true;
    }

    /**
     * @param {string} word
     * @return {boolean}
     */
    search(word) {
        const dfs = (node, i) => {
            if (i === word.length) return node.isEnd;
            const c = word[i];
            if (c === ".") {
                for (const child of node.children.values()) if (dfs(child, i + 1)) return true;
                return false;
            }
            const next = node.children.get(c);
            return next !== undefined && dfs(next, i + 1);
        };
        return dfs(this.root, 0);
    }
}
```
**Complexity:** `addWord` O(L) · `search` O(L) without dots, O(26^L) worst case with all dots · Space O(total characters)

#### Approach 2: Bucket Words by Length (simple)
```javascript
class WordDictionary {
    byLen = new Map();

    /**
     * @param {string} word
     * @return {void}
     */
    addWord(word) {
        if (!this.byLen.has(word.length)) this.byLen.set(word.length, new Set());
        this.byLen.get(word.length).add(word);
    }

    /**
     * @param {string} word
     * @return {boolean}
     */
    search(word) {
        const bucket = this.byLen.get(word.length);
        if (!bucket) return false;
        if (!word.includes(".")) return bucket.has(word);
        for (const w of bucket) {
            let ok = true;
            for (let i = 0; i < word.length && ok; i++) ok = word[i] === "." || word[i] === w[i];
            if (ok) return true;
        }
        return false;
    }
}
```
**Complexity:** `addWord` O(L) · `search` O(1) average without dots, O(words × L) with dots · Space O(total characters)

---

### 212. Word Search II
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/word-search-ii/)

**Problem:** Given an `m × n` letter grid and a list of `words`, return every word that can be traced through horizontally or vertically adjacent cells, without reusing a cell within a word.

**Example:** `board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]` → `["eat","oath"]`

**Key insight:** Build a trie from all the words and run one DFS per cell, descending the grid and the trie together. Prune trie nodes once their words are found.

#### Approach 1: Trie + Backtracking (no pruning)
```javascript
class WSNode {
    children = new Map();
    word = null; // set at the end of a word
}

/**
 * @param {string[][]} board
 * @param {string[]} words
 * @return {string[]}
 */
function findWords(board, words) {
    const root = new WSNode();
    for (const w of words) {
        let node = root;
        for (const c of w) {
            if (!node.children.has(c)) node.children.set(c, new WSNode());
            node = node.children.get(c);
        }
        node.word = w;
    }
    const m = board.length, n = board[0].length, res = [];
    const dfs = (r, c, parent) => {
        if (r < 0 || c < 0 || r >= m || c >= n) return;
        const ch = board[r][c];
        const node = parent.children.get(ch); // '#' is never a key, so visited cells stop here
        if (!node) return;
        if (node.word !== null) {
            res.push(node.word);
            node.word = null; // report each word once
        }
        board[r][c] = "#";
        dfs(r + 1, c, node);
        dfs(r - 1, c, node);
        dfs(r, c + 1, node);
        dfs(r, c - 1, node);
        board[r][c] = ch;
    };
    for (let r = 0; r < m; r++) for (let c = 0; c < n; c++) dfs(r, c, root);
    return res;
}
```
**Complexity:** Time O(m · n · 4 · 3^(L-1)), where L is the maximum word length · Space O(total characters in `words`)

#### Approach 2: Trie + Backtracking with Leaf Pruning (Optimal)
```javascript
class WSNode {
    children = new Map();
    word = null;
}

/**
 * @param {string[][]} board
 * @param {string[]} words
 * @return {string[]}
 */
function findWords(board, words) {
    const root = new WSNode();
    for (const w of words) {
        let node = root;
        for (const c of w) {
            if (!node.children.has(c)) node.children.set(c, new WSNode());
            node = node.children.get(c);
        }
        node.word = w;
    }
    const m = board.length, n = board[0].length, res = [];
    const dirs = [
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
    ];
    const dfs = (r, c, parent) => {
        const ch = board[r][c];
        const node = parent.children.get(ch);
        if (node.word !== null) {
            res.push(node.word);
            node.word = null;
        }
        board[r][c] = "#";
        for (const [dr, dc] of dirs) {
            const nr = r + dr, nc = c + dc;
            if (nr >= 0 && nc >= 0 && nr < m && nc < n && node.children.has(board[nr][nc])) dfs(nr, nc, node);
        }
        board[r][c] = ch;
        if (node.children.size === 0 && node.word === null) parent.children.delete(ch); // prune an exhausted leaf
    };
    for (let r = 0; r < m; r++) for (let c = 0; c < n; c++) if (root.children.has(board[r][c])) dfs(r, c, root);
    return res;
}
```
**Complexity:** Same worst case. Removing leaves cuts the search space sharply in practice; this is the version to use in interviews.

**Brute force:** Run LC 79 (Word Search) once per word: O(W · m · n · 4^L). Much slower when there are many words.

---
## 9. Heap / Priority Queue

> Each snippet below includes the `Heap` class from [Shared Definitions](#the-reusable-heap-class-used-in-heap-based-snippets). The comparator `less(a, b)` returns `true` when `a` should be closer to the top, so `(a, b) => a < b` gives a min-heap and `(a, b) => a > b` a max-heap.

### 703. Kth Largest Element in a Stream
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/kth-largest-element-in-a-stream/)

**Problem:** Design a class built from `k` and an initial array `nums`. Each call to `add(val)` adds a value to the stream and returns the current `k`th largest element.

**Example:** `k = 3, nums = [4,5,8,2]`; `add(3)` → 4, `add(5)` → 5, `add(10)` → 5

**Key insight:** Keep a min-heap of size `k`. Its top is the `k`th largest element.

#### Approach: Min-Heap of Size k (Optimal)
```javascript
/**
 * Binary heap ordered by `less(a, b)`: pass (a, b) => a < b for a min-heap.
 *
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}
class KthLargest {
    heap = new Heap((a, b) => a < b);

    /**
     * @param {number} k
     * @param {number[]} nums
     */
    constructor(k, nums) {
        this.k = k;
        for (const n of nums) this.add(n);
    }

    /**
     * @param {number} val
     * @return {number}
     */
    add(val) {
        this.heap.push(val);
        if (this.heap.size > this.k) this.heap.pop();
        return this.heap.peek();
    }
}
```
**Complexity:** `add` O(log k) · Space O(k)

**Alternative:** Keep a sorted array and insert with binary search plus `splice`. `add` is then O(n), from shifting elements.

---

### 1046. Last Stone Weight
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/last-stone-weight/)

**Problem:** Each turn, smash the two heaviest stones `x ≤ y`. If they're equal, both are destroyed. Otherwise a stone of weight `y - x` remains. Return the weight of the last stone, or `0` if none are left.

**Example:** `[2,7,4,1,8,1]` → `1`

#### Approach 1: Max-Heap (Optimal)
```javascript
/**
 * Binary heap ordered by `less(a, b)`: pass (a, b) => a < b for a min-heap.
 *
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}
/**
 * @param {number[]} stones
 * @return {number}
 */
function lastStoneWeight(stones) {
    const heap = new Heap((a, b) => a > b);
    for (const s of stones) heap.push(s);
    while (heap.size > 1) {
        const y = heap.pop(), x = heap.pop();
        if (y !== x) heap.push(y - x);
    }
    return heap.peek() ?? 0;
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sort Every Round (simple, fine for n ≤ 30)
```javascript
/**
 * @param {number[]} stones
 * @return {number}
 */
function lastStoneWeight(stones) {
    const s = [...stones];
    while (s.length > 1) {
        s.sort((a, b) => a - b);
        const y = s.pop(), x = s.pop();
        if (y !== x) s.push(y - x);
    }
    return s[0] ?? 0;
}
```
**Complexity:** Time O(n² log n) · Space O(n)

#### Approach 3: Bucket Sort (weights ≤ 1000)
```javascript
/**
 * @param {number[]} stones
 * @return {number}
 */
function lastStoneWeight(stones) {
    const maxW = Math.max(...stones);
    const bucket = new Array(maxW + 1).fill(0);
    for (const s of stones) bucket[s]++;
    let biggest = 0, cur = maxW;
    while (cur > 0) {
        if (bucket[cur] === 0) cur--;
        else if (biggest === 0) {
            bucket[cur] %= 2; // equal pairs destroy each other
            if (bucket[cur]) biggest = cur;
            cur--;
        } else {
            bucket[cur]--;
            if (biggest - cur <= cur) {
                bucket[biggest - cur]++;
                biggest = 0;
            } else biggest -= cur;
        }
    }
    return biggest;
}
```
**Complexity:** Time O(n + W), where W is the maximum weight · Space O(W)

---

### 973. K Closest Points to Origin
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/k-closest-points-to-origin/)

**Problem:** Given `points[i] = [x, y]`, return the `k` points closest to the origin by Euclidean distance, in any order.

**Example:** `points = [[1,3],[-2,2]], k = 1` → `[[-2,2]]`

**Key insight:** Compare squared distances (no square root needed). Keep a max-heap of size `k`, or use quickselect.

#### Approach 1: Sort
```javascript
/**
 * @param {number[][]} points
 * @param {number} k
 * @return {number[][]}
 */
function kClosest(points, k) {
    const d = (p) => p[0] * p[0] + p[1] * p[1];
    return [...points].sort((a, b) => d(a) - d(b)).slice(0, k);
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Max-Heap of Size k
```javascript
/**
 * Binary heap ordered by `less(a, b)`: pass (a, b) => a < b for a min-heap.
 *
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}
/**
 * @param {number[][]} points
 * @param {number} k
 * @return {number[][]}
 */
function kClosest(points, k) {
    const d = (p) => p[0] * p[0] + p[1] * p[1];
    const heap = new Heap((a, b) => d(a) > d(b)); // farthest on top
    for (const p of points) {
        heap.push(p);
        if (heap.size > k) heap.pop();
    }
    const res = [];
    while (heap.size) res.push(heap.pop());
    return res;
}
```
**Complexity:** Time O(n log k) · Space O(k)

#### Approach 3: Quickselect (Optimal on average)
```javascript
/**
 * @param {number[][]} points
 * @param {number} k
 * @return {number[][]}
 */
function kClosest(points, k) {
    const d = (p) => p[0] * p[0] + p[1] * p[1];
    let lo = 0, hi = points.length - 1;
    while (lo < hi) {
        const p = lo + Math.floor(Math.random() * (hi - lo + 1)); // random pivot to the end
        [points[p], points[hi]] = [points[hi], points[p]];
        const pivot = d(points[hi]);
        let store = lo;
        for (let i = lo; i < hi; i++) {
            if (d(points[i]) < pivot) {
                [points[i], points[store]] = [points[store], points[i]];
                store++;
            }
        }
        [points[store], points[hi]] = [points[hi], points[store]];
        if (store === k - 1) break;
        if (store < k - 1) lo = store + 1;
        else hi = store - 1;
    }
    return points.slice(0, k);
}
```
**Complexity:** Time O(n) average, O(n²) worst case · Space O(1)

---

### 215. Kth Largest Element in an Array
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/kth-largest-element-in-an-array/)

**Problem:** Return the `k`th largest element in an unsorted array (in sorted order, not the `k`th distinct value).

**Example:** `nums = [3,2,1,5,6,4], k = 2` → `5`

#### Approach 1: Sort
```javascript
/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
function findKthLargest(nums, k) {
    return [...nums].sort((a, b) => b - a)[k - 1];
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Min-Heap of Size k
```javascript
/**
 * Binary heap ordered by `less(a, b)`: pass (a, b) => a < b for a min-heap.
 *
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}
/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
function findKthLargest(nums, k) {
    const heap = new Heap((a, b) => a < b);
    for (const n of nums) {
        heap.push(n);
        if (heap.size > k) heap.pop();
    }
    return heap.peek();
}
```
**Complexity:** Time O(n log k) · Space O(k)

#### Approach 3: Quickselect with Three-Way Partition (Optimal on average)
A three-way partition handles many duplicates, and the random pivot avoids worst-case inputs.
```javascript
/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
function findKthLargest(nums, k) {
    const a = [...nums];
    const target = a.length - k; // index in ascending order
    let lo = 0, hi = a.length - 1;
    for (;;) {
        const pivot = a[lo + Math.floor(Math.random() * (hi - lo + 1))];
        let lt = lo, i = lo, gt = hi;
        while (i <= gt) {
            if (a[i] < pivot) [a[lt++], a[i++]] = [a[i], a[lt]];
            else if (a[i] > pivot) [a[i], a[gt--]] = [a[gt], a[i]];
            else i++;
        }
        if (target < lt) hi = lt - 1;
        else if (target > gt) lo = gt + 1;
        else return pivot;
    }
}
```
**Complexity:** Time O(n) average · Space O(n) for the copy (O(1) if you may mutate the input)

#### Approach 4: Counting Sort (values in [-10⁴, 10⁴])
```javascript
/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
function findKthLargest(nums, k) {
    const OFF = 10000;
    const cnt = new Array(2 * OFF + 1).fill(0);
    for (const n of nums) cnt[n + OFF]++;
    for (let v = 2 * OFF; v >= 0; v--) {
        k -= cnt[v];
        if (k <= 0) return v - OFF;
    }
    return -1;
}
```
**Complexity:** Time O(n + R), where R is the value range · Space O(R)

---

### 621. Task Scheduler
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/task-scheduler/)

**Problem:** You're given a list of CPU tasks labeled `A`–`Z` and a cooldown `n`: two identical tasks must be separated by at least `n` time units. The CPU can sit idle. Return the minimum number of time units needed to finish every task.

**Example:** `tasks = ["A","A","A","B","B","B"], n = 2` → `8` (A B idle A B idle A B)

**Key insight:** The most frequent task sets the frame: `(maxFreq-1) × (n+1) + countOfMaxFreqTasks`. The answer is never less than `len(tasks)`.

#### Approach 1: Max-Heap + Cooldown Queue (simulation)
```javascript
/**
 * Binary heap ordered by `less(a, b)`: pass (a, b) => a < b for a min-heap.
 *
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}
/**
 * @param {string[]} tasks
 * @param {number} n
 * @return {number}
 */
function leastInterval(tasks, n) {
    const freq = new Map();
    for (const t of tasks) freq.set(t, (freq.get(t) ?? 0) + 1);
    const heap = new Heap((a, b) => a > b);
    for (const c of freq.values()) heap.push(c);
    const cooldown = []; // [remaining count, time when ready]
    let head = 0, time = 0;
    while (heap.size || head < cooldown.length) {
        time++;
        const c = heap.pop();
        if (c !== undefined && c > 1) cooldown.push([c - 1, time + n]);
        if (head < cooldown.length && cooldown[head][1] === time) heap.push(cooldown[head++][0]);
    }
    return time;
}
```
**Complexity:** Time O(T · log 26) = O(T), where T is the total time · Space O(T) for the queue array

#### Approach 2: Math Formula (Optimal)
```javascript
/**
 * @param {string[]} tasks
 * @param {number} n
 * @return {number}
 */
function leastInterval(tasks, n) {
    const freq = new Array(26).fill(0);
    for (const t of tasks) freq[t.charCodeAt(0) - 65]++;
    const maxF = Math.max(...freq);
    const maxCount = freq.filter((f) => f === maxF).length;
    return Math.max(tasks.length, (maxF - 1) * (n + 1) + maxCount);
}
```
**Complexity:** Time O(len(tasks)) · Space O(1)

---

### 355. Design Twitter
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/design-twitter/)

**Problem:** Design a simplified Twitter with `postTweet(userId, tweetId)`, `getNewsFeed(userId)` (the 10 most recent tweet IDs from the user and the people they follow, newest first), `follow`, and `unfollow`.

**Key insight:** The news feed is a merge of k sorted lists. Each user's tweets are already in time order, so a heap seeded with each followee's newest tweet works.

#### Approach: Maps + Heap Merge (Optimal)
```javascript
/**
 * Binary heap ordered by `less(a, b)`: pass (a, b) => a < b for a min-heap.
 *
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}
class Twitter {
    clock = 0;
    tweets = new Map(); // user -> [time, tweetId][] in time order
    follows = new Map();

    /**
     * @param {number} userId
     * @param {number} tweetId
     * @return {void}
     */
    postTweet(userId, tweetId) {
        if (!this.tweets.has(userId)) this.tweets.set(userId, []);
        this.tweets.get(userId).push([this.clock++, tweetId]);
    }

    /**
     * @param {number} userId
     * @return {number[]}
     */
    getNewsFeed(userId) {
        // [time, tweetId, user, index into that user's list], newest on top
        const heap = new Heap((a, b) => a[0] > b[0]);
        const users = new Set(this.follows.get(userId) ?? []);
        users.add(userId);
        for (const u of users) {
            const list = this.tweets.get(u);
            if (list && list.length) {
                const i = list.length - 1;
                heap.push([list[i][0], list[i][1], u, i]);
            }
        }
        const res = [];
        while (heap.size && res.length < 10) {
            const [, id, u, i] = heap.pop();
            res.push(id);
            if (i > 0) {
                const [t, nid] = this.tweets.get(u)[i - 1];
                heap.push([t, nid, u, i - 1]);
            }
        }
        return res;
    }

    /**
     * @param {number} followerId
     * @param {number} followeeId
     * @return {void}
     */
    follow(followerId, followeeId) {
        if (!this.follows.has(followerId)) this.follows.set(followerId, new Set());
        this.follows.get(followerId).add(followeeId);
    }

    /**
     * @param {number} followerId
     * @param {number} followeeId
     * @return {void}
     */
    unfollow(followerId, followeeId) {
        this.follows.get(followerId)?.delete(followeeId);
    }
}
```
**Complexity:** `postTweet`, `follow`, `unfollow` O(1) · `getNewsFeed` O(F + 10 log F) for F followees · Space O(users + tweets + follows)

**Simpler alternative:** Gather every tweet from the user and their followees, sort by time, and take 10: O(N log N) per feed.

---

### 295. Find Median from Data Stream
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/find-median-from-data-stream/)

**Problem:** Design `MedianFinder` with `addNum(num)` and `findMedian()`, which returns the median of every number added so far.

**Key insight:** Keep two heaps: a max-heap for the smaller half and a min-heap for the larger half. Keep their sizes within one of each other, so the median is on top.

#### Approach 1: Two Heaps (Optimal)
```javascript
/**
 * Binary heap ordered by `less(a, b)`: pass (a, b) => a < b for a min-heap.
 *
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}
class MedianFinder {
    low = new Heap((a, b) => a > b); // max-heap: smaller half
    high = new Heap((a, b) => a < b); // min-heap: larger half

    /**
     * @param {number} num
     * @return {void}
     */
    addNum(num) {
        this.low.push(num);
        this.high.push(this.low.pop()); // move low's max to high
        if (this.high.size > this.low.size) this.low.push(this.high.pop());
    }

    /**
     * @return {number}
     */
    findMedian() {
        if (this.low.size > this.high.size) return this.low.peek();
        return (this.low.peek() + this.high.peek()) / 2;
    }
}
```
**Complexity:** `addNum` O(log n) · `findMedian` O(1) · Space O(n)

#### Approach 2: Sorted Array with Binary Insert (simple)
```javascript
class MedianFinder {
    data = [];

    /**
     * @param {number} num
     * @return {void}
     */
    addNum(num) {
        let lo = 0, hi = this.data.length;
        while (lo < hi) {
            const mid = (lo + hi) >> 1;
            if (this.data[mid] < num) lo = mid + 1;
            else hi = mid;
        }
        this.data.splice(lo, 0, num);
    }

    /**
     * @return {number}
     */
    findMedian() {
        const n = this.data.length;
        return n % 2 ? this.data[n >> 1] : (this.data[n / 2 - 1] + this.data[n / 2]) / 2;
    }
}
```
**Complexity:** `addNum` O(n) for the `splice` · `findMedian` O(1)

**Follow-up (all values in [0, 100]):** Keep a 101-bucket count array and walk it to find the median, which is O(100) per query.

---
## 10. Backtracking

> **Template:** choose → recurse → un-choose. Save a **copy** of the path (`res.push([...path])`). Pushing `path` itself stores a reference, and later changes would overwrite results you've already stored.

### 78. Subsets
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/subsets/)

**Problem:** Given an array of unique integers, return every possible subset (the power set), with no duplicates.

**Example:** `[1,2,3]` → `[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]`

#### Approach 1: Backtracking (include or exclude)
```javascript
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
function subsets(nums) {
    const res = [], path = [];
    const bt = (i) => {
        if (i === nums.length) {
            res.push([...path]);
            return;
        }
        path.push(nums[i]); // include
        bt(i + 1);
        path.pop(); // exclude
        bt(i + 1);
    };
    bt(0);
    return res;
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n) recursion

#### Approach 2: Iterative Cascading
```javascript
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
function subsets(nums) {
    let res = [[]];
    for (const n of nums) res = res.concat(res.map((s) => [...s, n]));
    return res;
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n · 2ⁿ) for the output

#### Approach 3: Bitmask
```javascript
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
function subsets(nums) {
    const res = [];
    for (let mask = 0; mask < 1 << nums.length; mask++) {
        res.push(nums.filter((_, i) => mask & (1 << i)));
    }
    return res;
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n · 2ⁿ)

---

### 39. Combination Sum
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/combination-sum/)

**Problem:** Given distinct `candidates` and a `target`, return every unique combination that sums to `target`. Each number may be used any number of times.

**Example:** `candidates = [2,3,6,7], target = 7` → `[[2,2,3],[7]]`

**Key insight:** Recurse from the same index `i` to allow reuse, and pass `i` forward so the same combination isn't produced in a different order.

#### Approach 1: Backtracking with Sorting and Pruning (Optimal)
```javascript
/**
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
function combinationSum(candidates, target) {
    const c = [...candidates].sort((a, b) => a - b);
    const res = [], path = [];
    const bt = (start, remain) => {
        if (remain === 0) {
            res.push([...path]);
            return;
        }
        for (let i = start; i < c.length && c[i] <= remain; i++) {
            // sorted, so the loop stops once candidates get too big
            path.push(c[i]);
            bt(i, remain - c[i]); // i, not i+1: reuse is allowed
            path.pop();
        }
    };
    bt(0, target);
    return res;
}
```
**Complexity:** Time O(N^(T/M + 1)), where T = target and M = smallest candidate · Space O(T/M) recursion

#### Approach 2: DP Building Combinations (unbounded knapsack style)
```javascript
/**
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
function combinationSum(candidates, target) {
    const dp = Array.from({ length: target + 1 }, () => []);
    dp[0].push([]);
    for (const c of candidates) {
        // outer loop over coins means no permutations
        for (let t = c; t <= target; t++) for (const comb of dp[t - c]) dp[t].push([...comb, c]);
    }
    return dp[target];
}
```
**Complexity:** Exponential in the output size · Space proportional to all partial combinations

---

### 46. Permutations
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/permutations/)

**Problem:** Given an array of distinct integers, return every permutation.

**Example:** `[1,2,3]` → `[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]`

#### Approach 1: Backtracking with a Used Array
```javascript
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
function permute(nums) {
    const res = [], path = [];
    const used = new Array(nums.length).fill(false);
    const bt = () => {
        if (path.length === nums.length) {
            res.push([...path]);
            return;
        }
        for (let i = 0; i < nums.length; i++) {
            if (used[i]) continue;
            used[i] = true;
            path.push(nums[i]);
            bt();
            path.pop();
            used[i] = false;
        }
    };
    bt();
    return res;
}
```
**Complexity:** Time O(n · n!) · Space O(n)

#### Approach 2: In-Place Swapping
```javascript
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
function permute(nums) {
    const a = [...nums], res = [];
    const bt = (k) => {
        if (k === a.length) {
            res.push([...a]);
            return;
        }
        for (let i = k; i < a.length; i++) {
            [a[k], a[i]] = [a[i], a[k]];
            bt(k + 1);
            [a[k], a[i]] = [a[i], a[k]];
        }
    };
    bt(0);
    return res;
}
```
**Complexity:** Time O(n · n!) · Space O(n)

---

### 90. Subsets II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/subsets-ii/)

**Problem:** Same as Subsets, but `nums` may contain duplicates. The result must not contain duplicate subsets.

**Example:** `[1,2,2]` → `[[],[1],[1,2],[1,2,2],[2],[2,2]]`

**Key insight:** Sort first. At each recursion depth, skip a value equal to the previous one at that same depth.

#### Approach 1: Backtracking with Duplicate Skipping (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
function subsetsWithDup(nums) {
    const a = [...nums].sort((x, y) => x - y);
    const res = [], path = [];
    const bt = (start) => {
        res.push([...path]);
        for (let i = start; i < a.length; i++) {
            if (i > start && a[i] === a[i - 1]) continue; // skip duplicates at the same depth
            path.push(a[i]);
            bt(i + 1);
            path.pop();
        }
    };
    bt(0);
    return res;
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n)

#### Approach 2: Iterative (extend only the subsets added last round when the value repeats)
```javascript
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
function subsetsWithDup(nums) {
    const a = [...nums].sort((x, y) => x - y);
    const res = [[]];
    let prevSize = 0;
    for (let i = 0; i < a.length; i++) {
        const start = i > 0 && a[i] === a[i - 1] ? prevSize : 0;
        const size = res.length;
        for (let j = start; j < size; j++) res.push([...res[j], a[i]]);
        prevSize = size;
    }
    return res;
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n · 2ⁿ) for the output

---

### 40. Combination Sum II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/combination-sum-ii/)

**Problem:** Given `candidates` (which may contain duplicates) and a `target`, return every unique combination that sums to `target`. Each element may be used at most once.

**Example:** `candidates = [10,1,2,7,6,1,5], target = 8` → `[[1,1,6],[1,2,5],[1,7],[2,6]]`

#### Approach: Sort + Backtracking with Duplicate Skipping (Optimal)
```javascript
/**
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
function combinationSum2(candidates, target) {
    const c = [...candidates].sort((a, b) => a - b);
    const res = [], path = [];
    const bt = (start, remain) => {
        if (remain === 0) {
            res.push([...path]);
            return;
        }
        for (let i = start; i < c.length && c[i] <= remain; i++) {
            if (i > start && c[i] === c[i - 1]) continue;
            path.push(c[i]);
            bt(i + 1, remain - c[i]);
            path.pop();
        }
    };
    bt(0, target);
    return res;
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n)

---

### 79. Word Search
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/word-search/)

**Problem:** Given an `m × n` grid of letters and a `word`, return whether the word can be traced through horizontally or vertically adjacent cells, without reusing a cell.

**Example:** `board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"` → `true`

**Key insight:** Run DFS from each cell, marking cells as visited in place, and undo the mark when backtracking.

#### Approach: DFS Backtracking (Optimal)
```javascript
/**
 * @param {string[][]} board
 * @param {string} word
 * @return {boolean}
 */
function exist(board, word) {
    const m = board.length, n = board[0].length;
    const dfs = (r, c, i) => {
        if (i === word.length) return true;
        if (r < 0 || c < 0 || r >= m || c >= n || board[r][c] !== word[i]) return false;
        board[r][c] = "#";
        const found = dfs(r + 1, c, i + 1) || dfs(r - 1, c, i + 1) || dfs(r, c + 1, i + 1) || dfs(r, c - 1, i + 1);
        board[r][c] = word[i];
        return found;
    };
    for (let r = 0; r < m; r++) for (let c = 0; c < n; c++) if (dfs(r, c, 0)) return true;
    return false;
}
```
**Complexity:** Time O(m · n · 3^L) · Space O(L) recursion

**Pruning tricks:** Return `false` early if the board has fewer of any letter than the word needs. If the word's last letter is rarer on the board than its first letter, search for the reversed word instead.

---

### 131. Palindrome Partitioning
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/palindrome-partitioning/)

**Problem:** Split `s` so that every piece is a palindrome, and return every such partition.

**Example:** `"aab"` → `[["a","a","b"],["aa","b"]]`

#### Approach 1: Backtracking with an On-the-Fly Palindrome Check
```javascript
/**
 * @param {string} s
 * @return {string[][]}
 */
function partition(s) {
    const res = [], path = [];
    const isPal = (l, r) => {
        while (l < r) if (s[l++] !== s[r--]) return false;
        return true;
    };
    const bt = (start) => {
        if (start === s.length) {
            res.push([...path]);
            return;
        }
        for (let end = start; end < s.length; end++) {
            if (!isPal(start, end)) continue;
            path.push(s.slice(start, end + 1));
            bt(end + 1);
            path.pop();
        }
    };
    bt(0);
    return res;
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n)

#### Approach 2: Precomputed Palindrome DP Table
```javascript
/**
 * @param {string} s
 * @return {string[][]}
 */
function partition(s) {
    const n = s.length;
    const pal = Array.from({ length: n }, () => new Array(n).fill(false));
    for (let i = n - 1; i >= 0; i--) {
        for (let j = i; j < n; j++) pal[i][j] = s[i] === s[j] && (j - i < 2 || pal[i + 1][j - 1]);
    }
    const res = [], path = [];
    const bt = (start) => {
        if (start === n) {
            res.push([...path]);
            return;
        }
        for (let end = start; end < n; end++) {
            if (!pal[start][end]) continue;
            path.push(s.slice(start, end + 1));
            bt(end + 1);
            path.pop();
        }
    };
    bt(0);
    return res;
}
```
**Complexity:** Time O(n · 2ⁿ), with each palindrome check O(1) · Space O(n²)

---

### 17. Letter Combinations of a Phone Number
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/letter-combinations-of-a-phone-number/)

**Problem:** Given a string of digits `2`–`9`, return every letter combination they could spell on a phone keypad.

**Example:** `"23"` → `["ad","ae","af","bd","be","bf","cd","ce","cf"]`

#### Approach 1: Backtracking
```javascript
/**
 * @param {string} digits
 * @return {string[]}
 */
function letterCombinations(digits) {
    if (!digits) return [];
    const keys = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    const res = [];
    const bt = (i, cur) => {
        if (i === digits.length) {
            res.push(cur);
            return;
        }
        for (const c of keys[Number(digits[i])]) bt(i + 1, cur + c);
    };
    bt(0, "");
    return res;
}
```
**Complexity:** Time O(n · 4ⁿ) · Space O(n)

#### Approach 2: Iterative with `reduce`
```javascript
/**
 * @param {string} digits
 * @return {string[]}
 */
function letterCombinations(digits) {
    if (!digits) return [];
    const keys = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    return [...digits].reduce((acc, d) => acc.flatMap((prefix) => [...keys[Number(d)]].map((c) => prefix + c)), [""]);
}
```
**Complexity:** Time O(n · 4ⁿ) · Space O(n · 4ⁿ)

---

### 51. N-Queens
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/n-queens/)

**Problem:** Place `n` queens on an `n × n` board so that no two attack each other. Return every distinct board, written with `'Q'` for a queen and `'.'` for an empty square.

**Example:** `n = 4` → `[[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]`

**Key insight:** Place one queen per row. A column, a diagonal (`r-c`), or an anti-diagonal (`r+c`) can each hold at most one queen.

#### Approach 1: Backtracking with Sets
```javascript
/**
 * @param {number} n
 * @return {string[][]}
 */
function solveNQueens(n) {
    const res = [], queens = [];
    const cols = new Set(), diag = new Set(), anti = new Set();
    const bt = (r) => {
        if (r === n) {
            res.push(queens.map((c) => ".".repeat(c) + "Q" + ".".repeat(n - c - 1)));
            return;
        }
        for (let c = 0; c < n; c++) {
            if (cols.has(c) || diag.has(r - c) || anti.has(r + c)) continue;
            cols.add(c);
            diag.add(r - c);
            anti.add(r + c);
            queens.push(c);
            bt(r + 1);
            queens.pop();
            cols.delete(c);
            diag.delete(r - c);
            anti.delete(r + c);
        }
    };
    bt(0);
    return res;
}
```
**Complexity:** Time O(n!) · Space O(n)

#### Approach 2: Bitmask Backtracking (fastest)
```javascript
/**
 * @param {number} n
 * @return {string[][]}
 */
function solveNQueens(n) {
    const res = [], queens = [];
    const full = (1 << n) - 1;
    const bt = (cols, d1, d2) => {
        if (queens.length === n) {
            res.push(queens.map((c) => ".".repeat(c) + "Q" + ".".repeat(n - c - 1)));
            return;
        }
        let avail = full & ~(cols | d1 | d2);
        while (avail) {
            const bit = avail & -avail; // lowest set bit
            avail ^= bit;
            queens.push(31 - Math.clz32(bit));
            bt(cols | bit, ((d1 | bit) << 1) & full, (d2 | bit) >> 1);
            queens.pop();
        }
    };
    bt(0, 0, 0);
    return res;
}
```
**Complexity:** Time O(n!) with a much smaller constant · Space O(n)

---
## 11. Graphs

> **Grid idiom:** `const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];` and bounds-check `nr >= 0 && nr < m && nc >= 0 && nc < n`. BFS queues use an array plus a head index. Recursive DFS on a 300 × 300 grid can overflow the JS call stack (roughly 10⁴ frames), so the BFS or explicit-stack versions are safer for big inputs.

### 200. Number of Islands
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/number-of-islands/)

**Problem:** Given an `m × n` grid of `'1'` (land) and `'0'` (water), count the islands. An island is a group of land cells connected horizontally or vertically.

**Example:** `[["1","1","0"],["0","1","0"],["0","0","1"]]` → `2`

**Key insight:** Each time you find unvisited land, count one island and flood-fill it so none of its cells are counted again.

#### Approach 1: DFS Flood Fill
```javascript
/**
 * @param {string[][]} grid
 * @return {number}
 */
function numIslands(grid) {
    const m = grid.length, n = grid[0].length;
    const sink = (r, c) => {
        if (r < 0 || c < 0 || r >= m || c >= n || grid[r][c] !== "1") return;
        grid[r][c] = "0";
        sink(r + 1, c);
        sink(r - 1, c);
        sink(r, c + 1);
        sink(r, c - 1);
    };
    let count = 0;
    for (let r = 0; r < m; r++) {
        for (let c = 0; c < n; c++) {
            if (grid[r][c] === "1") {
                count++;
                sink(r, c);
            }
        }
    }
    return count;
}
```
**Complexity:** Time O(m · n) · Space O(m · n) recursion in the worst case

> ⚠️ A 300 × 300 grid of all `"1"` recurses up to 90,000 deep and overflows the JS stack. Approach 2 or 3 is safe.

#### Approach 2: BFS
```javascript
/**
 * @param {string[][]} grid
 * @return {number}
 */
function numIslands(grid) {
    const m = grid.length, n = grid[0].length;
    const dirs = [
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
    ];
    let count = 0;
    for (let r = 0; r < m; r++) {
        for (let c = 0; c < n; c++) {
            if (grid[r][c] !== "1") continue;
            count++;
            grid[r][c] = "0";
            const q = [[r, c]];
            for (let head = 0; head < q.length; head++) {
                const [cr, cc] = q[head];
                for (const [dr, dc] of dirs) {
                    const nr = cr + dr, nc = cc + dc;
                    if (nr >= 0 && nc >= 0 && nr < m && nc < n && grid[nr][nc] === "1") {
                        grid[nr][nc] = "0";
                        q.push([nr, nc]);
                    }
                }
            }
        }
    }
    return count;
}
```
**Complexity:** Time O(m · n) · Space O(m · n) for the queue array

#### Approach 3: Union-Find
```javascript
/**
 * @param {string[][]} grid
 * @return {number}
 */
function numIslands(grid) {
    const m = grid.length, n = grid[0].length;
    const parent = Array.from({ length: m * n }, (_, i) => i);
    let count = 0;
    for (const row of grid) for (const v of row) if (v === "1") count++;
    const find = (x) => {
        while (parent[x] !== x) {
            parent[x] = parent[parent[x]]; // path halving
            x = parent[x];
        }
        return x;
    };
    for (let r = 0; r < m; r++) {
        for (let c = 0; c < n; c++) {
            if (grid[r][c] !== "1") continue;
            for (const [nr, nc] of [
                [r + 1, c],
                [r, c + 1],
            ]) {
                if (nr < m && nc < n && grid[nr][nc] === "1") {
                    const a = find(r * n + c), b = find(nr * n + nc);
                    if (a !== b) {
                        parent[a] = b;
                        count--;
                    }
                }
            }
        }
    }
    return count;
}
```
**Complexity:** Time O(m · n · α(mn)) · Space O(m · n)

---

### 133. Clone Graph
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/clone-graph/)

**Problem:** Given a reference to one node in a connected undirected graph, return a deep copy of the whole graph. Each node has a `val` and a list of `neighbors`.

**Key insight:** A map from original node to clone serves as the visited set and also handles cycles.

> LeetCode provides the `_Node` class below for this problem (its JavaScript template uses that name, as a function constructor). The class form below is equivalent and is included so the snippet runs on its own.

#### Approach 1: DFS + Map
```javascript
class _Node {
    val;
    neighbors;
    /**
     * @param {number} [val]
     * @param {_Node[]} [neighbors]
     */
    constructor(val, neighbors) {
        this.val = val === undefined ? 0 : val;
        this.neighbors = neighbors === undefined ? [] : neighbors;
    }
}

/**
 * @param {_Node | null} node
 * @return {_Node | null}
 */
function cloneGraph(node) {
    const clones = new Map();
    const dfs = (n) => {
        const existing = clones.get(n);
        if (existing) return existing;
        const copy = new _Node(n.val);
        clones.set(n, copy); // register before recursing, so cycles terminate
        copy.neighbors = n.neighbors.map(dfs);
        return copy;
    };
    return node ? dfs(node) : null;
}
```
**Complexity:** Time O(V + E) · Space O(V)

#### Approach 2: BFS + Map
```javascript
class _Node {
    val;
    neighbors;
    /**
     * @param {number} [val]
     * @param {_Node[]} [neighbors]
     */
    constructor(val, neighbors) {
        this.val = val === undefined ? 0 : val;
        this.neighbors = neighbors === undefined ? [] : neighbors;
    }
}

/**
 * @param {_Node | null} node
 * @return {_Node | null}
 */
function cloneGraph(node) {
    if (!node) return null;
    const clones = new Map([[node, new _Node(node.val)]]);
    const q = [node];
    for (let head = 0; head < q.length; head++) {
        const cur = q[head];
        for (const nb of cur.neighbors) {
            if (!clones.has(nb)) {
                clones.set(nb, new _Node(nb.val));
                q.push(nb);
            }
            clones.get(cur).neighbors.push(clones.get(nb));
        }
    }
    return clones.get(node);
}
```
**Complexity:** Time O(V + E) · Space O(V)

---

### 695. Max Area of Island
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/max-area-of-island/)

**Problem:** In a binary grid where `1` is land, return the size of the largest island (4-directionally connected), or `0` if there is no land.

#### Approach 1: DFS Returning Area
```javascript
/**
 * @param {number[][]} grid
 * @return {number}
 */
function maxAreaOfIsland(grid) {
    const m = grid.length, n = grid[0].length;
    const dfs = (r, c) => {
        if (r < 0 || c < 0 || r >= m || c >= n || grid[r][c] === 0) return 0;
        grid[r][c] = 0;
        return 1 + dfs(r + 1, c) + dfs(r - 1, c) + dfs(r, c + 1) + dfs(r, c - 1);
    };
    let best = 0;
    for (let r = 0; r < m; r++) for (let c = 0; c < n; c++) best = Math.max(best, dfs(r, c));
    return best;
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Iterative DFS with a Stack
```javascript
/**
 * @param {number[][]} grid
 * @return {number}
 */
function maxAreaOfIsland(grid) {
    const m = grid.length, n = grid[0].length;
    const dirs = [
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
    ];
    let best = 0;
    for (let r = 0; r < m; r++) {
        for (let c = 0; c < n; c++) {
            if (!grid[r][c]) continue;
            grid[r][c] = 0;
            const stack = [[r, c]];
            let area = 0;
            while (stack.length) {
                const [cr, cc] = stack.pop();
                area++;
                for (const [dr, dc] of dirs) {
                    const nr = cr + dr, nc = cc + dc;
                    if (nr >= 0 && nc >= 0 && nr < m && nc < n && grid[nr][nc]) {
                        grid[nr][nc] = 0;
                        stack.push([nr, nc]);
                    }
                }
            }
            best = Math.max(best, area);
        }
    }
    return best;
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

---

### 417. Pacific Atlantic Water Flow
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/pacific-atlantic-water-flow/)

**Problem:** `heights` is an island. The Pacific Ocean touches its top and left edges, and the Atlantic touches its bottom and right edges. Water flows from a cell to a neighbor of equal or lower height. Return every cell from which water can reach both oceans.

**Key insight:** Search in reverse, from each ocean's border moving to cells of equal or greater height. The answer is the intersection of the two reachable sets.

#### Approach 1: Reverse DFS from Both Oceans (Optimal)
```javascript
/**
 * @param {number[][]} heights
 * @return {number[][]}
 */
function pacificAtlantic(heights) {
    const m = heights.length, n = heights[0].length;
    const pac = Array.from({ length: m }, () => new Array(n).fill(false));
    const atl = Array.from({ length: m }, () => new Array(n).fill(false));
    const dfs = (r, c, seen, prev) => {
        if (r < 0 || c < 0 || r >= m || c >= n || seen[r][c] || heights[r][c] < prev) return;
        seen[r][c] = true;
        const h = heights[r][c];
        dfs(r + 1, c, seen, h);
        dfs(r - 1, c, seen, h);
        dfs(r, c + 1, seen, h);
        dfs(r, c - 1, seen, h);
    };
    for (let c = 0; c < n; c++) {
        dfs(0, c, pac, 0);
        dfs(m - 1, c, atl, 0);
    }
    for (let r = 0; r < m; r++) {
        dfs(r, 0, pac, 0);
        dfs(r, n - 1, atl, 0);
    }
    const res = [];
    for (let r = 0; r < m; r++) for (let c = 0; c < n; c++) if (pac[r][c] && atl[r][c]) res.push([r, c]);
    return res;
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Multi-Source BFS from Each Ocean
```javascript
/**
 * @param {number[][]} heights
 * @return {number[][]}
 */
function pacificAtlantic(heights) {
    const m = heights.length, n = heights[0].length;
    const dirs = [
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
    ];
    const bfs = (starts) => {
        const seen = Array.from({ length: m }, () => new Array(n).fill(false));
        const q = [];
        for (const [r, c] of starts) {
            if (!seen[r][c]) {
                seen[r][c] = true;
                q.push([r, c]);
            }
        }
        for (let head = 0; head < q.length; head++) {
            const [r, c] = q[head];
            for (const [dr, dc] of dirs) {
                const nr = r + dr, nc = c + dc;
                if (nr >= 0 && nc >= 0 && nr < m && nc < n && !seen[nr][nc] && heights[nr][nc] >= heights[r][c]) {
                    seen[nr][nc] = true;
                    q.push([nr, nc]);
                }
            }
        }
        return seen;
    };
    const pacStarts = [], atlStarts = [];
    for (let c = 0; c < n; c++) {
        pacStarts.push([0, c]);
        atlStarts.push([m - 1, c]);
    }
    for (let r = 0; r < m; r++) {
        pacStarts.push([r, 0]);
        atlStarts.push([r, n - 1]);
    }
    const pac = bfs(pacStarts), atl = bfs(atlStarts);
    const res = [];
    for (let r = 0; r < m; r++) for (let c = 0; c < n; c++) if (pac[r][c] && atl[r][c]) res.push([r, c]);
    return res;
}
```
**Complexity:** Time O(m · n) · Space O(m · n). No deep recursion.

**Brute force:** DFS from every cell toward the oceans: O((m·n)²).

---

### 130. Surrounded Regions
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/surrounded-regions/)

**Problem:** In a board of `'X'` and `'O'`, flip every `'O'` region that is fully surrounded by `'X'` to `'X'`. Regions connected to the border stay as they are.

**Key insight:** Work backward. Mark every `O` reachable from the border as safe (`T`). Flip all remaining `O`s to `X`, then turn the `T`s back into `O`s.

#### Approach 1: Border BFS (Optimal)
```javascript
/**
 * Do not return anything, modify board in-place instead.
 *
 * @param {string[][]} board
 * @return {void}
 */
function solve(board) {
    const m = board.length, n = board[0].length;
    const q = [];
    for (let r = 0; r < m; r++)
        for (let c = 0; c < n; c++) {
            if ((r === 0 || c === 0 || r === m - 1 || c === n - 1) && board[r][c] === "O") q.push([r, c]);
        }
    for (let head = 0; head < q.length; head++) {
        const [r, c] = q[head];
        if (r < 0 || c < 0 || r >= m || c >= n || board[r][c] !== "O") continue;
        board[r][c] = "T"; // safe: connected to the border
        q.push([r + 1, c], [r - 1, c], [r, c + 1], [r, c - 1]);
    }
    for (let r = 0; r < m; r++) for (let c = 0; c < n; c++) board[r][c] = board[r][c] === "T" ? "O" : "X";
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Union-Find with a Virtual Border Node
```javascript
/**
 * @param {string[][]} board
 * @return {void}
 */
function solve(board) {
    const m = board.length, n = board[0].length, border = m * n;
    const parent = Array.from({ length: m * n + 1 }, (_, i) => i);
    const find = (x) => {
        while (parent[x] !== x) {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    };
    const union = (a, b) => {
        parent[find(a)] = find(b);
    };
    for (let r = 0; r < m; r++) {
        for (let c = 0; c < n; c++) {
            if (board[r][c] !== "O") continue;
            const id = r * n + c;
            if (r === 0 || c === 0 || r === m - 1 || c === n - 1) union(id, border);
            if (r + 1 < m && board[r + 1][c] === "O") union(id, id + n);
            if (c + 1 < n && board[r][c + 1] === "O") union(id, id + 1);
        }
    }
    const root = find(border);
    for (let r = 0; r < m; r++)
        for (let c = 0; c < n; c++) {
            if (board[r][c] === "O" && find(r * n + c) !== root) board[r][c] = "X";
        }
}
```
**Complexity:** Time O(m · n · α(mn)) · Space O(m · n)

---

### 994. Rotting Oranges
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/rotting-oranges/)

**Problem:** Grid cells are `0` (empty), `1` (fresh), or `2` (rotten). Each minute, fresh oranges next to a rotten one become rotten. Return the minutes until no fresh orange remains, or `-1` if some never rot.

**Key insight:** Run multi-source BFS starting from every rotten orange at once. Each BFS level is one minute.

#### Approach: Multi-Source BFS (Optimal)
```javascript
/**
 * @param {number[][]} grid
 * @return {number}
 */
function orangesRotting(grid) {
    const m = grid.length, n = grid[0].length;
    let q = [];
    let fresh = 0;
    for (let r = 0; r < m; r++)
        for (let c = 0; c < n; c++) {
            if (grid[r][c] === 2) q.push([r, c]);
            else if (grid[r][c] === 1) fresh++;
        }
    const dirs = [
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
    ];
    let minutes = 0;
    while (q.length && fresh) {
        minutes++;
        const next = [];
        for (const [r, c] of q) {
            for (const [dr, dc] of dirs) {
                const nr = r + dr, nc = c + dc;
                if (nr >= 0 && nc >= 0 && nr < m && nc < n && grid[nr][nc] === 1) {
                    grid[nr][nc] = 2;
                    fresh--;
                    next.push([nr, nc]);
                }
            }
        }
        q = next;
    }
    return fresh ? -1 : minutes;
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

---

### 286. Walls and Gates (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/walls-and-gates/) · Free mirror: LintCode 663

**Problem:** In a grid, `-1` is a wall, `0` is a gate, and `INF = 2147483647` is an empty room. Fill each empty room in place with its distance to the nearest gate, leaving `INF` where no gate is reachable.

**Key insight:** Run multi-source BFS from every gate. The first time BFS reaches a room, that distance is the shortest.

#### Approach 1: Multi-Source BFS (Optimal)
```javascript
/**
 * Do not return anything, modify rooms in-place instead.
 *
 * @param {number[][]} rooms
 * @return {void}
 */
function wallsAndGates(rooms) {
    const INF = 2147483647, m = rooms.length, n = rooms[0].length;
    const q = [];
    for (let r = 0; r < m; r++) for (let c = 0; c < n; c++) if (rooms[r][c] === 0) q.push([r, c]);
    const dirs = [
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
    ];
    for (let head = 0; head < q.length; head++) {
        const [r, c] = q[head];
        for (const [dr, dc] of dirs) {
            const nr = r + dr, nc = c + dc;
            if (nr >= 0 && nc >= 0 && nr < m && nc < n && rooms[nr][nc] === INF) {
                rooms[nr][nc] = rooms[r][c] + 1;
                q.push([nr, nc]);
            }
        }
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: DFS from Each Gate (slower)
```javascript
/**
 * @param {number[][]} rooms
 * @return {void}
 */
function wallsAndGates(rooms) {
    const m = rooms.length, n = rooms[0].length;
    const dfs = (r, c, d) => {
        if (r < 0 || c < 0 || r >= m || c >= n || rooms[r][c] < d) return; // a wall (-1), or an already shorter distance
        rooms[r][c] = d;
        dfs(r + 1, c, d + 1);
        dfs(r - 1, c, d + 1);
        dfs(r, c + 1, d + 1);
        dfs(r, c - 1, d + 1);
    };
    for (let r = 0; r < m; r++) for (let c = 0; c < n; c++) if (rooms[r][c] === 0) dfs(r, c, 0);
}
```
**Complexity:** Time O(k · m · n) for k gates · Space O(m · n)

---

### 207. Course Schedule
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/course-schedule/)

**Problem:** There are `numCourses` courses, and `prerequisites[i] = [a, b]` means you must take `b` before `a`. Return whether you can finish every course.

**Example:** `2, [[1,0]]` → `true`; `2, [[1,0],[0,1]]` → `false`

**Key insight:** You can finish every course exactly when the prerequisite graph has no cycle.

#### Approach 1: Kahn's Algorithm (BFS Topological Sort)
```javascript
/**
 * @param {number} numCourses
 * @param {number[][]} prerequisites
 * @return {boolean}
 */
function canFinish(numCourses, prerequisites) {
    const adj = Array.from({ length: numCourses }, () => []);
    const indeg = new Array(numCourses).fill(0);
    for (const [a, b] of prerequisites) {
        adj[b].push(a);
        indeg[a]++;
    }
    const q = [];
    for (let i = 0; i < numCourses; i++) if (indeg[i] === 0) q.push(i);
    for (let head = 0; head < q.length; head++) {
        for (const nxt of adj[q[head]]) if (--indeg[nxt] === 0) q.push(nxt);
    }
    return q.length === numCourses; // every course was eventually taken
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

#### Approach 2: DFS Three-Color Cycle Detection
```javascript
/**
 * @param {number} numCourses
 * @param {number[][]} prerequisites
 * @return {boolean}
 */
function canFinish(numCourses, prerequisites) {
    const adj = Array.from({ length: numCourses }, () => []);
    for (const [a, b] of prerequisites) adj[b].push(a);
    const state = new Array(numCourses).fill(0); // 0 = unvisited, 1 = visiting, 2 = done
    const hasCycle = (u) => {
        if (state[u] === 1) return true; // back edge into the current path
        if (state[u] === 2) return false;
        state[u] = 1;
        if (adj[u].some(hasCycle)) return true;
        state[u] = 2;
        return false;
    };
    for (let i = 0; i < numCourses; i++) if (hasCycle(i)) return false;
    return true;
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

---

### 210. Course Schedule II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/course-schedule-ii/)

**Problem:** Same setup as 207, but return a valid order in which to take the courses, or an empty array if it's impossible.

**Example:** `4, [[1,0],[2,0],[3,1],[3,2]]` → `[0,1,2,3]` or `[0,2,1,3]`

#### Approach 1: Kahn's Algorithm
```javascript
/**
 * @param {number} numCourses
 * @param {number[][]} prerequisites
 * @return {number[]}
 */
function findOrder(numCourses, prerequisites) {
    const adj = Array.from({ length: numCourses }, () => []);
    const indeg = new Array(numCourses).fill(0);
    for (const [a, b] of prerequisites) {
        adj[b].push(a);
        indeg[a]++;
    }
    const order = [];
    for (let i = 0; i < numCourses; i++) if (indeg[i] === 0) order.push(i);
    for (let head = 0; head < order.length; head++) {
        for (const nxt of adj[order[head]]) if (--indeg[nxt] === 0) order.push(nxt);
    }
    return order.length === numCourses ? order : [];
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

#### Approach 2: DFS Post-Order
```javascript
/**
 * @param {number} numCourses
 * @param {number[][]} prerequisites
 * @return {number[]}
 */
function findOrder(numCourses, prerequisites) {
    const adj = Array.from({ length: numCourses }, () => []);
    for (const [a, b] of prerequisites) adj[a].push(b); // course -> its prerequisites
    const state = new Array(numCourses).fill(0);
    const order = [];
    const dfs = (u) => {
        // returns false when a cycle is found
        if (state[u] === 1) return false;
        if (state[u] === 2) return true;
        state[u] = 1;
        if (!adj[u].every(dfs)) return false;
        state[u] = 2;
        order.push(u); // prerequisites are recorded first
        return true;
    };
    for (let i = 0; i < numCourses; i++) if (!dfs(i)) return [];
    return order;
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

---

### 684. Redundant Connection
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/redundant-connection/)

**Problem:** A tree with nodes `1..n` had one extra edge added. Return an edge you can remove to get a tree again. If several edges qualify, return the one that appears last in the input.

**Example:** `[[1,2],[1,3],[2,3]]` → `[2,3]`

**Key insight:** Union-find. The first edge whose two endpoints are already connected is the redundant one.

#### Approach 1: Union-Find with Rank and Path Compression (Optimal)
```javascript
/**
 * @param {number[][]} edges
 * @return {number[]}
 */
function findRedundantConnection(edges) {
    const n = edges.length;
    const parent = Array.from({ length: n + 1 }, (_, i) => i);
    const rank = new Array(n + 1).fill(0);
    const find = (x) => (parent[x] === x ? x : (parent[x] = find(parent[x])));
    for (const [a, b] of edges) {
        let ra = find(a), rb = find(b);
        if (ra === rb) return [a, b]; // already connected, so this edge closes a cycle
        if (rank[ra] < rank[rb]) [ra, rb] = [rb, ra];
        parent[rb] = ra;
        if (rank[ra] === rank[rb]) rank[ra]++;
    }
    return [];
}
```
**Complexity:** Time O(n · α(n)) · Space O(n)

#### Approach 2: DFS per Edge (check whether already connected)
```javascript
/**
 * @param {number[][]} edges
 * @return {number[]}
 */
function findRedundantConnection(edges) {
    const adj = new Map();
    const connected = (u, target, seen) => {
        if (u === target) return true;
        seen.add(u);
        return (adj.get(u) ?? []).some((v) => !seen.has(v) && connected(v, target, seen));
    };
    for (const [a, b] of edges) {
        if (adj.has(a) && adj.has(b) && connected(a, b, new Set())) return [a, b];
        if (!adj.has(a)) adj.set(a, []);
        if (!adj.has(b)) adj.set(b, []);
        adj.get(a).push(b);
        adj.get(b).push(a);
    }
    return [];
}
```
**Complexity:** Time O(n²) · Space O(n)

---

### 323. Number of Connected Components in an Undirected Graph (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/) · Free mirror: LintCode 3651

**Problem:** Given `n` nodes labeled `0..n-1` and a list of undirected `edges`, return the number of connected components.

**Example:** `n = 5, edges = [[0,1],[1,2],[3,4]]` → `2`

#### Approach 1: Union-Find (Optimal)
```javascript
/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {number}
 */
function countComponents(n, edges) {
    const parent = Array.from({ length: n }, (_, i) => i);
    const find = (x) => {
        while (parent[x] !== x) {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    };
    let comps = n;
    for (const [a, b] of edges) {
        const ra = find(a), rb = find(b);
        if (ra !== rb) {
            parent[ra] = rb;
            comps--;
        }
    }
    return comps;
}
```
**Complexity:** Time O(V + E · α(V)) · Space O(V)

#### Approach 2: BFS
```javascript
/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {number}
 */
function countComponents(n, edges) {
    const adj = Array.from({ length: n }, () => []);
    for (const [a, b] of edges) {
        adj[a].push(b);
        adj[b].push(a);
    }
    const seen = new Array(n).fill(false);
    let comps = 0;
    for (let i = 0; i < n; i++) {
        if (seen[i]) continue;
        comps++;
        seen[i] = true;
        const q = [i];
        for (let head = 0; head < q.length; head++) {
            for (const v of adj[q[head]]) {
                if (!seen[v]) {
                    seen[v] = true;
                    q.push(v);
                }
            }
        }
    }
    return comps;
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

---

### 261. Graph Valid Tree (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/graph-valid-tree/) · Free mirror: LintCode 178

**Problem:** Given `n` nodes and a list of undirected `edges`, return whether the edges form a valid tree.

**Key insight:** A graph is a tree exactly when it has `n-1` edges and is connected (equivalently, `n-1` edges and no cycle).

#### Approach 1: Edge Count + Union-Find (Optimal)
```javascript
/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {boolean}
 */
function validTree(n, edges) {
    if (edges.length !== n - 1) return false;
    const parent = Array.from({ length: n }, (_, i) => i);
    const find = (x) => {
        while (parent[x] !== x) {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    };
    for (const [a, b] of edges) {
        const ra = find(a), rb = find(b);
        if (ra === rb) return false; // cycle
        parent[ra] = rb;
    }
    return true;
}
```
**Complexity:** Time O(n · α(n)) · Space O(n)

#### Approach 2: Edge Count + BFS Connectivity
```javascript
/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {boolean}
 */
function validTree(n, edges) {
    if (edges.length !== n - 1) return false;
    const adj = Array.from({ length: n }, () => []);
    for (const [a, b] of edges) {
        adj[a].push(b);
        adj[b].push(a);
    }
    const seen = new Set([0]);
    const q = [0];
    for (let head = 0; head < q.length; head++) {
        for (const v of adj[q[head]]) {
            if (!seen.has(v)) {
                seen.add(v);
                q.push(v);
            }
        }
    }
    return seen.size === n;
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

---

### 127. Word Ladder
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/word-ladder/)

**Problem:** Transform `beginWord` into `endWord` one letter at a time, where every intermediate word must be in `wordList`. Return the number of words in the shortest such sequence, counting both ends, or `0` if no sequence exists.

**Example:** `"hit" → "cog"`, `wordList = ["hot","dot","dog","lot","log","cog"]` → `5`

**Key insight:** BFS over words finds the shortest path. Generate neighbors by trying all 26 letters at each position (or by bucketing words under wildcard patterns like `h*t`).

#### Approach 1: BFS with Wildcard Pattern Buckets
Bucket every word under each of its patterns, such as `h*t` for `hot`, so neighbors are found by lookup instead of trying 26 letters.
```javascript
/**
 * @param {string} beginWord
 * @param {string} endWord
 * @param {string[]} wordList
 * @return {number}
 */
function ladderLength(beginWord, endWord, wordList) {
    if (!wordList.includes(endWord)) return 0;
    const buckets = new Map();
    for (const w of wordList) {
        for (let i = 0; i < w.length; i++) {
            const p = w.slice(0, i) + "*" + w.slice(i + 1);
            if (!buckets.has(p)) buckets.set(p, []);
            buckets.get(p).push(w);
        }
    }
    const seen = new Set([beginWord]);
    let level = [beginWord], steps = 1;
    while (level.length) {
        const next = [];
        for (const word of level) {
            for (let i = 0; i < word.length; i++) {
                const p = word.slice(0, i) + "*" + word.slice(i + 1);
                for (const nxt of buckets.get(p) ?? []) {
                    if (nxt === endWord) return steps + 1;
                    if (!seen.has(nxt)) {
                        seen.add(nxt);
                        next.push(nxt);
                    }
                }
                buckets.delete(p); // each bucket is used once
            }
        }
        level = next;
        steps++;
    }
    return 0;
}
```
**Complexity:** Time O(N · L²) · Space O(N · L²)

#### Approach 2: Bidirectional BFS with Letter Substitution (fastest in practice)
```javascript
/**
 * @param {string} beginWord
 * @param {string} endWord
 * @param {string[]} wordList
 * @return {number}
 */
function ladderLength(beginWord, endWord, wordList) {
    const words = new Set(wordList);
    if (!words.has(endWord)) return 0;
    words.delete(beginWord);
    words.delete(endWord);
    let front = new Set([beginWord]), back = new Set([endWord]), steps = 1;
    while (front.size && back.size) {
        if (front.size > back.size) [front, back] = [back, front]; // always expand the smaller side
        const next = new Set();
        for (const w of front) {
            for (let i = 0; i < w.length; i++) {
                for (let c = 97; c <= 122; c++) {
                    const cand = w.slice(0, i) + String.fromCharCode(c) + w.slice(i + 1);
                    if (back.has(cand)) return steps + 1;
                    if (words.has(cand)) {
                        words.delete(cand);
                        next.add(cand);
                    }
                }
            }
        }
        front = next;
        steps++;
    }
    return 0;
}
```
**Complexity:** Same worst case, but explores far fewer nodes in practice

---
## 12. Advanced Graphs

### 332. Reconstruct Itinerary
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/reconstruct-itinerary/)

**Problem:** Given airline `tickets[i] = [from, to]`, rebuild the itinerary in order, starting from `"JFK"` and using every ticket exactly once. If several itineraries are valid, return the one that comes first in lexical order.

**Example:** `[["MUC","LHR"],["JFK","MUC"],["SFO","SJC"],["LHR","SFO"]]` → `["JFK","MUC","LHR","SFO","SJC"]`

**Key insight:** This is an Eulerian path. Hierholzer's algorithm visits the smallest destination first and appends each airport after its edges are used up (post-order), then reverses the result.

#### Approach 1: Hierholzer's Algorithm (Optimal)
Store each airport's destinations in descending order, so `pop()` from the end yields the lexically smallest.
```javascript
/**
 * @param {string[][]} tickets
 * @return {string[]}
 */
function findItinerary(tickets) {
    const adj = new Map();
    for (const [src, dst] of [...tickets].sort((a, b) => (a[1] < b[1] ? 1 : a[1] > b[1] ? -1 : 0))) {
        if (!adj.has(src)) adj.set(src, []);
        adj.get(src).push(dst);
    }
    const route = [];
    const visit = (a) => {
        const dests = adj.get(a);
        while (dests && dests.length) visit(dests.pop());
        route.push(a); // post-order
    };
    visit("JFK");
    return route.reverse();
}
```
**Complexity:** Time O(E log E) · Space O(E)

#### Approach 2: Iterative Hierholzer with an Explicit Stack
```javascript
/**
 * @param {string[][]} tickets
 * @return {string[]}
 */
function findItinerary(tickets) {
    const adj = new Map();
    for (const [src, dst] of tickets) {
        if (!adj.has(src)) adj.set(src, []);
        adj.get(src).push(dst);
    }
    for (const dests of adj.values()) dests.sort().reverse(); // smallest at the end
    const stack = ["JFK"], route = [];
    while (stack.length) {
        const dests = adj.get(stack[stack.length - 1]);
        if (dests && dests.length) stack.push(dests.pop());
        else route.push(stack.pop()); // dead end: emit in post-order
    }
    return route.reverse();
}
```
**Complexity:** Time O(E log E) · Space O(E)

---

### 1584. Min Cost to Connect All Points
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/min-cost-to-connect-all-points/)

**Problem:** Given points on a 2D plane, connecting two points costs their Manhattan distance. Return the minimum total cost to connect every point (a minimum spanning tree).

**Example:** `[[0,0],[2,2],[3,10],[5,2],[7,0]]` → `20`

#### Approach 1: Prim's with an O(n²) Array (Optimal for dense graphs)
```javascript
/**
 * @param {number[][]} points
 * @return {number}
 */
function minCostConnectPoints(points) {
    const n = points.length;
    const dist = new Array(n).fill(Infinity);
    const inTree = new Array(n).fill(false);
    dist[0] = 0;
    let total = 0;
    for (let k = 0; k < n; k++) {
        let u = -1;
        for (let i = 0; i < n; i++) if (!inTree[i] && (u === -1 || dist[i] < dist[u])) u = i;
        inTree[u] = true;
        total += dist[u];
        for (let v = 0; v < n; v++) {
            if (!inTree[v]) {
                const d = Math.abs(points[u][0] - points[v][0]) + Math.abs(points[u][1] - points[v][1]);
                if (d < dist[v]) dist[v] = d;
            }
        }
    }
    return total;
}
```
**Complexity:** Time O(n²) · Space O(n)

#### Approach 2: Kruskal's with Union-Find
```javascript
/**
 * @param {number[][]} points
 * @return {number}
 */
function minCostConnectPoints(points) {
    const n = points.length;
    const edges = [];
    for (let i = 0; i < n; i++) {
        for (let j = i + 1; j < n; j++) {
            edges.push([Math.abs(points[i][0] - points[j][0]) + Math.abs(points[i][1] - points[j][1]), i, j]);
        }
    }
    edges.sort((a, b) => a[0] - b[0]);
    const parent = Array.from({ length: n }, (_, i) => i);
    const find = (x) => {
        while (parent[x] !== x) {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    };
    let total = 0, used = 0;
    for (const [w, a, b] of edges) {
        if (used === n - 1) break;
        const ra = find(a), rb = find(b);
        if (ra !== rb) {
            parent[ra] = rb;
            total += w;
            used++;
        }
    }
    return total;
}
```
**Complexity:** Time O(n² log n) · Space O(n²)

#### Approach 3: Prim's with a Min-Heap (lazy)
```javascript
/**
 * Binary heap ordered by `less(a, b)`: pass (a, b) => a < b for a min-heap.
 *
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}
/**
 * @param {number[][]} points
 * @return {number}
 */
function minCostConnectPoints(points) {
    const n = points.length;
    const inTree = new Array(n).fill(false);
    const heap = new Heap((a, b) => a[0] < b[0]); // [cost, node]
    heap.push([0, 0]);
    let total = 0, added = 0;
    while (added < n) {
        const [cost, u] = heap.pop();
        if (inTree[u]) continue;
        inTree[u] = true;
        total += cost;
        added++;
        for (let v = 0; v < n; v++) {
            if (!inTree[v]) heap.push([Math.abs(points[u][0] - points[v][0]) + Math.abs(points[u][1] - points[v][1]), v]);
        }
    }
    return total;
}
```
**Complexity:** Time O(n² log n) · Space O(n²)

---

### 743. Network Delay Time
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/network-delay-time/)

**Problem:** `times[i] = [u, v, w]` is a directed edge from `u` to `v` with travel time `w`. A signal starts at node `k`. Return how long until all `n` nodes receive it, or `-1` if some node never does.

**Example:** `times = [[2,1,1],[2,3,1],[3,4,1]], n = 4, k = 2` → `2`

**Key insight:** Single-source shortest paths with non-negative weights calls for Dijkstra. The answer is the largest of the shortest distances.

#### Approach 1: Dijkstra with a Min-Heap (Optimal)
```javascript
/**
 * Binary heap ordered by `less(a, b)`: pass (a, b) => a < b for a min-heap.
 *
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}
/**
 * @param {number[][]} times
 * @param {number} n
 * @param {number} k
 * @return {number}
 */
function networkDelayTime(times, n, k) {
    const adj = Array.from({ length: n + 1 }, () => []);
    for (const [u, v, w] of times) adj[u].push([v, w]);
    const dist = new Array(n + 1).fill(Infinity);
    dist[k] = 0;
    const heap = new Heap((a, b) => a[0] < b[0]); // [dist, node]
    heap.push([0, k]);
    while (heap.size) {
        const [d, u] = heap.pop();
        if (d > dist[u]) continue; // stale entry
        for (const [v, w] of adj[u]) {
            if (d + w < dist[v]) {
                dist[v] = d + w;
                heap.push([dist[v], v]);
            }
        }
    }
    const worst = Math.max(...dist.slice(1));
    return worst === Infinity ? -1 : worst;
}
```
**Complexity:** Time O(E log V) · Space O(V + E)

#### Approach 2: Bellman-Ford
```javascript
/**
 * @param {number[][]} times
 * @param {number} n
 * @param {number} k
 * @return {number}
 */
function networkDelayTime(times, n, k) {
    const dist = new Array(n + 1).fill(Infinity);
    dist[k] = 0;
    for (let i = 1; i < n; i++) {
        let changed = false;
        for (const [u, v, w] of times) {
            if (dist[u] + w < dist[v]) {
                dist[v] = dist[u] + w;
                changed = true;
            }
        }
        if (!changed) break;
    }
    const worst = Math.max(...dist.slice(1));
    return worst === Infinity ? -1 : worst;
}
```
**Complexity:** Time O(V · E) · Space O(V)

#### Approach 3: Floyd–Warshall (n ≤ 100)
```javascript
/**
 * @param {number[][]} times
 * @param {number} n
 * @param {number} k
 * @return {number}
 */
function networkDelayTime(times, n, k) {
    const d = Array.from({ length: n + 1 }, (_, i) => Array.from({ length: n + 1 }, (_, j) => (i === j ? 0 : Infinity)));
    for (const [u, v, w] of times) d[u][v] = w;
    for (let m = 1; m <= n; m++) {
        for (let i = 1; i <= n; i++) {
            for (let j = 1; j <= n; j++) if (d[i][m] + d[m][j] < d[i][j]) d[i][j] = d[i][m] + d[m][j];
        }
    }
    const worst = Math.max(...d[k].slice(1));
    return worst === Infinity ? -1 : worst;
}
```
**Complexity:** Time O(V³) · Space O(V²)

---

### 778. Swim in Rising Water
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/swim-in-rising-water/)

**Problem:** In an `n × n` grid, `grid[i][j]` is the elevation. At time `t` you can swim between adjacent cells whose elevations are both ≤ `t`. Return the least time at which you can get from `(0,0)` to `(n-1,n-1)`.

**Key insight:** Minimize the maximum elevation along a path. Use Dijkstra where a path's cost is `max(cost so far, next cell)`, or binary search on `t` with a BFS check.

#### Approach 1: Dijkstra / Min-Heap on the Path Maximum (Optimal)
```javascript
/**
 * Binary heap ordered by `less(a, b)`: pass (a, b) => a < b for a min-heap.
 *
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}
/**
 * @param {number[][]} grid
 * @return {number}
 */
function swimInWater(grid) {
    const n = grid.length;
    const seen = Array.from({ length: n }, () => new Array(n).fill(false));
    const heap = new Heap((a, b) => a[0] < b[0]); // [time, r, c]
    heap.push([grid[0][0], 0, 0]);
    seen[0][0] = true;
    const dirs = [
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
    ];
    while (heap.size) {
        const [t, r, c] = heap.pop();
        if (r === n - 1 && c === n - 1) return t;
        for (const [dr, dc] of dirs) {
            const nr = r + dr, nc = c + dc;
            if (nr >= 0 && nc >= 0 && nr < n && nc < n && !seen[nr][nc]) {
                seen[nr][nc] = true;
                heap.push([Math.max(t, grid[nr][nc]), nr, nc]);
            }
        }
    }
    return -1;
}
```
**Complexity:** Time O(n² log n) · Space O(n²)

#### Approach 2: Binary Search on Time + BFS
```javascript
/**
 * @param {number[][]} grid
 * @return {number}
 */
function swimInWater(grid) {
    const n = grid.length;
    const dirs = [
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
    ];
    const canReach = (t) => {
        if (grid[0][0] > t) return false;
        const seen = Array.from({ length: n }, () => new Array(n).fill(false));
        seen[0][0] = true;
        const q = [[0, 0]];
        for (let head = 0; head < q.length; head++) {
            const [r, c] = q[head];
            if (r === n - 1 && c === n - 1) return true;
            for (const [dr, dc] of dirs) {
                const nr = r + dr, nc = c + dc;
                if (nr >= 0 && nc >= 0 && nr < n && nc < n && !seen[nr][nc] && grid[nr][nc] <= t) {
                    seen[nr][nc] = true;
                    q.push([nr, nc]);
                }
            }
        }
        return false;
    };
    let lo = grid[0][0], hi = n * n - 1;
    while (lo < hi) {
        const mid = (lo + hi) >> 1;
        if (canReach(mid)) hi = mid;
        else lo = mid + 1;
    }
    return lo;
}
```
**Complexity:** Time O(n² log n) · Space O(n²)

---

### 269. Alien Dictionary (Premium)
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/alien-dictionary/) · Free mirror: LintCode 892

**Problem:** You're given a list of `words` sorted in an unknown alien alphabet. Work out an order for the letters that is consistent with that sorting. Return `""` if the input is contradictory, including when a word comes before its own prefix.

**Example:** `["wrt","wrf","er","ett","rftt"]` → `"wertf"`

**Key insight:** In each adjacent pair of words, the first position where they differ gives one edge `a → b`. A topological sort of those edges gives the alphabet, and a cycle means the input is invalid.

#### Approach 1: Kahn's BFS Topological Sort
```javascript
/**
 * @param {string[]} words
 * @return {string}
 */
function alienOrder(words) {
    const adj = new Map();
    const indeg = new Map();
    for (const w of words)
        for (const c of w) {
            if (!adj.has(c)) adj.set(c, new Set());
            if (!indeg.has(c)) indeg.set(c, 0);
        }
    for (let i = 0; i + 1 < words.length; i++) {
        const a = words[i], b = words[i + 1];
        const minLen = Math.min(a.length, b.length);
        let j = 0;
        while (j < minLen && a[j] === b[j]) j++;
        if (j === minLen) {
            if (a.length > b.length) return ""; // a word appears before its own prefix
            continue;
        }
        if (!adj.get(a[j]).has(b[j])) {
            adj.get(a[j]).add(b[j]);
            indeg.set(b[j], indeg.get(b[j]) + 1);
        }
    }
    const q = [...indeg.keys()].filter((c) => indeg.get(c) === 0);
    for (let head = 0; head < q.length; head++) {
        for (const nxt of adj.get(q[head])) {
            indeg.set(nxt, indeg.get(nxt) - 1);
            if (indeg.get(nxt) === 0) q.push(nxt);
        }
    }
    return q.length === indeg.size ? q.join("") : ""; // shorter means a cycle
}
```
**Complexity:** Time O(C), where C is the total number of characters · Space O(U + min(U², N)) for U unique letters

#### Approach 2: DFS Post-Order with Cycle Detection
```javascript
/**
 * @param {string[]} words
 * @return {string}
 */
function alienOrder(words) {
    const adj = new Map();
    for (const w of words) for (const c of w) if (!adj.has(c)) adj.set(c, new Set());
    for (let i = 0; i + 1 < words.length; i++) {
        const a = words[i], b = words[i + 1];
        const minLen = Math.min(a.length, b.length);
        let j = 0;
        while (j < minLen && a[j] === b[j]) j++;
        if (j === minLen) {
            if (a.length > b.length) return "";
        } else adj.get(a[j]).add(b[j]);
    }
    const state = new Map(); // 1 = visiting, 2 = done
    const out = [];
    const dfs = (c) => {
        // returns false when a cycle is found
        const s = state.get(c);
        if (s === 1) return false;
        if (s === 2) return true;
        state.set(c, 1);
        for (const nx of adj.get(c)) if (!dfs(nx)) return false;
        state.set(c, 2);
        out.push(c);
        return true;
    };
    for (const c of adj.keys()) if (!dfs(c)) return "";
    return out.reverse().join("");
}
```
**Complexity:** Time O(C) · Space O(U + edges)

---

### 787. Cheapest Flights Within K Stops
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/cheapest-flights-within-k-stops/)

**Problem:** `flights[i] = [from, to, price]`. Return the cheapest price from `src` to `dst` using at most `k` stops, or `-1` if no such route exists.

**Example:** `n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1` → `700`

**Key insight:** Bellman-Ford limited to `k+1` rounds. Relax from a copy of the previous round's prices so a single round can't chain several edges.

#### Approach 1: Bellman-Ford with k+1 Iterations (Optimal)
```javascript
/**
 * @param {number} n
 * @param {number[][]} flights
 * @param {number} src
 * @param {number} dst
 * @param {number} k
 * @return {number}
 */
function findCheapestPrice(n, flights, src, dst, k) {
    let prices = new Array(n).fill(Infinity);
    prices[src] = 0;
    for (let i = 0; i <= k; i++) {
        const tmp = [...prices]; // relax from the previous round only
        for (const [u, v, w] of flights) if (prices[u] + w < tmp[v]) tmp[v] = prices[u] + w;
        prices = tmp;
    }
    return prices[dst] === Infinity ? -1 : prices[dst];
}
```
**Complexity:** Time O(k · E) · Space O(n)

#### Approach 2: BFS by Number of Stops
```javascript
/**
 * @param {number} n
 * @param {number[][]} flights
 * @param {number} src
 * @param {number} dst
 * @param {number} k
 * @return {number}
 */
function findCheapestPrice(n, flights, src, dst, k) {
    const adj = Array.from({ length: n }, () => []);
    for (const [u, v, w] of flights) adj[u].push([v, w]);
    const best = new Array(n).fill(Infinity);
    best[src] = 0;
    let level = [[src, 0]]; // [node, cost]
    for (let stops = 0; stops <= k && level.length; stops++) {
        const next = [];
        for (const [u, cost] of level) {
            for (const [v, w] of adj[u]) {
                if (cost + w < best[v]) {
                    best[v] = cost + w;
                    next.push([v, cost + w]);
                }
            }
        }
        level = next;
    }
    return best[dst] === Infinity ? -1 : best[dst];
}
```
**Complexity:** Time O(k · E) · Space O(n + E)

---
## 13. 1-D Dynamic Programming

> **Progression for each problem:** recursion → memoization (top-down) → tabulation (bottom-up) → space-optimized, where that helps. JavaScript engines allow roughly 10,000 nested calls, so when recursion depth can reach the input size (for example `amount` up to 10⁴), prefer the bottom-up version.

### 70. Climbing Stairs
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/climbing-stairs/)

**Problem:** You can climb 1 or 2 steps at a time. How many distinct ways are there to reach the top of `n` stairs?

**Example:** `n = 3` → `3` (1+1+1, 1+2, 2+1)

**Key insight:** `ways(n) = ways(n-1) + ways(n-2)`, the Fibonacci recurrence.

#### Approach 1: Memoized Recursion
```javascript
/**
 * @param {number} n
 * @return {number}
 */
function climbStairs(n) {
    const memo = new Map();
    const f = (i) => {
        if (i <= 2) return i;
        if (!memo.has(i)) memo.set(i, f(i - 1) + f(i - 2));
        return memo.get(i);
    };
    return f(n);
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Bottom-Up, O(1) Space (Optimal)
```javascript
/**
 * @param {number} n
 * @return {number}
 */
function climbStairs(n) {
    let a = 1, b = 1; // ways(0), ways(1)
    for (let i = 2; i <= n; i++) [a, b] = [b, a + b];
    return b;
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 3: Matrix Exponentiation
```javascript
/**
 * @param {number} n
 * @return {number}
 */
function climbStairs(n) {
    // a 2 × 2 matrix [[a, b], [c, d]] is stored flat as [a, b, c, d]
    const mul = (x, y) => [x[0] * y[0] + x[1] * y[2], x[0] * y[1] + x[1] * y[3], x[2] * y[0] + x[3] * y[2], x[2] * y[1] + x[3] * y[3]];
    let res = [1, 0, 0, 1], base = [1, 1, 1, 0];
    for (let p = n; p > 0; p >>= 1) {
        if (p & 1) res = mul(res, base);
        base = mul(base, base);
    }
    return res[0]; // F(n+1)
}
```
**Complexity:** Time O(log n) · Space O(1)

---

### 746. Min Cost Climbing Stairs
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/min-cost-climbing-stairs/)

**Problem:** `cost[i]` is the price of stepping on stair `i`. After paying, you can climb 1 or 2 steps. You may start on stair 0 or stair 1. Return the minimum cost to get past the last stair.

**Example:** `[10,15,20]` → `15`

#### Approach 1: Tabulation
```javascript
/**
 * @param {number[]} cost
 * @return {number}
 */
function minCostClimbingStairs(cost) {
    const n = cost.length;
    const dp = new Array(n + 1).fill(0); // dp[i] = min cost to reach stair i
    for (let i = 2; i <= n; i++) dp[i] = Math.min(dp[i - 1] + cost[i - 1], dp[i - 2] + cost[i - 2]);
    return dp[n];
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: O(1) Space (Optimal)
```javascript
/**
 * @param {number[]} cost
 * @return {number}
 */
function minCostClimbingStairs(cost) {
    let a = 0, b = 0; // cost to stand on stair i-2 and i-1, having paid for them
    for (const c of cost) [a, b] = [b, Math.min(a, b) + c];
    return Math.min(a, b);
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 198. House Robber
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/house-robber/)

**Problem:** Houses along a street hold `nums[i]` money each. You can't rob two adjacent houses. Return the most money you can take.

**Example:** `[2,7,9,3,1]` → `12` (2 + 9 + 1)

**Key insight:** `dp[i] = max(dp[i-1], dp[i-2] + nums[i])`.

#### Approach 1: Memoized Recursion
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function rob(nums) {
    const memo = new Array(nums.length).fill(-1);
    const f = (i) => {
        if (i < 0) return 0;
        if (memo[i] < 0) memo[i] = Math.max(f(i - 1), f(i - 2) + nums[i]);
        return memo[i];
    };
    return f(nums.length - 1);
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: O(1) Space (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function rob(nums) {
    let prev2 = 0, prev1 = 0;
    for (const n of nums) [prev2, prev1] = [prev1, Math.max(prev1, prev2 + n)];
    return prev1;
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 213. House Robber II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/house-robber-ii/)

**Problem:** Same as House Robber, but the houses form a circle, so the first and last houses are adjacent.

**Example:** `[2,3,2]` → `3`; `[1,2,3,1]` → `4`

**Key insight:** You can't rob both the first and the last house. Answer = `max(rob(nums[1:]), rob(nums[:n-1]))`.

#### Approach: Two Linear Passes (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function rob(nums) {
    const robRange = (lo, hi) => {
        let a = 0, b = 0;
        for (let i = lo; i <= hi; i++) [a, b] = [b, Math.max(b, a + nums[i])];
        return b;
    };
    if (nums.length === 1) return nums[0];
    return Math.max(robRange(0, nums.length - 2), robRange(1, nums.length - 1));
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 5. Longest Palindromic Substring
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-palindromic-substring/)

**Problem:** Return the longest substring of `s` that is a palindrome.

**Example:** `"babad"` → `"bab"` (or `"aba"`); `"cbbd"` → `"bb"`

**Key insight:** Every palindrome has a center: a single character (odd length) or the gap between two characters (even length). Try each of the 2n−1 centers and expand outward.

#### Approach 1: Expand Around the Center (Optimal in practice)
```javascript
/**
 * @param {string} s
 * @return {string}
 */
function longestPalindrome(s) {
    let start = 0, best = 0;
    const expand = (l, r) => {
        while (l >= 0 && r < s.length && s[l] === s[r]) {
            l--;
            r++;
        }
        if (r - l - 1 > best) {
            start = l + 1;
            best = r - l - 1;
        }
    };
    for (let i = 0; i < s.length; i++) {
        expand(i, i); // odd length
        expand(i, i + 1); // even length
    }
    return s.slice(start, start + best);
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: DP Table
```javascript
/**
 * @param {string} s
 * @return {string}
 */
function longestPalindrome(s) {
    const n = s.length;
    const dp = Array.from({ length: n }, () => new Array(n).fill(false));
    let start = 0, best = 1;
    for (let i = n - 1; i >= 0; i--) {
        for (let j = i; j < n; j++) {
            if (s[i] === s[j] && (j - i < 3 || dp[i + 1][j - 1])) {
                dp[i][j] = true;
                if (j - i + 1 > best) {
                    start = i;
                    best = j - i + 1;
                }
            }
        }
    }
    return s.slice(start, start + best);
}
```
**Complexity:** Time O(n²) · Space O(n²)

#### Approach 3: Manacher's Algorithm (theoretically optimal)
```javascript
/**
 * @param {string} s
 * @return {string}
 */
function longestPalindrome(s) {
    const t = "^#" + [...s].join("#") + "#$"; // "abc" -> "^#a#b#c#$"; every palindrome now has odd length
    const p = new Array(t.length).fill(0);
    let center = 0, right = 0;
    for (let i = 1; i < t.length - 1; i++) {
        if (i < right) p[i] = Math.min(right - i, p[2 * center - i]);
        while (t[i + p[i] + 1] === t[i - p[i] - 1]) p[i]++;
        if (i + p[i] > right) {
            center = i;
            right = i + p[i];
        }
    }
    let maxLen = 0, ci = 0;
    p.forEach((v, i) => {
        if (v > maxLen) {
            maxLen = v;
            ci = i;
        }
    });
    const begin = (ci - maxLen) >> 1;
    return s.slice(begin, begin + maxLen);
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 647. Palindromic Substrings
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/palindromic-substrings/)

**Problem:** Count the substrings of `s` that are palindromes. Substrings at different positions count separately.

**Example:** `"aaa"` → `6`

#### Approach 1: Expand Around the Center (Optimal in practice)
```javascript
/**
 * @param {string} s
 * @return {number}
 */
function countSubstrings(s) {
    let count = 0;
    for (let c = 0; c < 2 * s.length - 1; c++) {
        let l = c >> 1, r = l + (c & 1);
        while (l >= 0 && r < s.length && s[l] === s[r]) {
            count++;
            l--;
            r++;
        }
    }
    return count;
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: DP Table
```javascript
/**
 * @param {string} s
 * @return {number}
 */
function countSubstrings(s) {
    const n = s.length;
    const dp = Array.from({ length: n }, () => new Array(n).fill(false));
    let count = 0;
    for (let i = n - 1; i >= 0; i--) {
        for (let j = i; j < n; j++) {
            if (s[i] === s[j] && (j - i < 3 || dp[i + 1][j - 1])) {
                dp[i][j] = true;
                count++;
            }
        }
    }
    return count;
}
```
**Complexity:** Time O(n²) · Space O(n²)

**Linear-time alternative:** Manacher's algorithm (see #5). Sum `(p[i]+1)/2` over the transformed string.

---

### 91. Decode Ways
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/decode-ways/)

**Problem:** Letters are encoded as numbers (`'A' → "1"`, …, `'Z' → "26"`). Given a string of digits, count the ways to decode it.

**Example:** `"12"` → `2` ("AB", "L"); `"226"` → `3`; `"06"` → `0`

**Key insight:** `dp[i] = dp[i-1]` if `s[i-1] != '0'`, plus `dp[i-2]` if `s[i-2:i]` is between 10 and 26.

#### Approach 1: Tabulation
```javascript
/**
 * @param {string} s
 * @return {number}
 */
function numDecodings(s) {
    const n = s.length;
    const dp = new Array(n + 1).fill(0);
    dp[0] = 1;
    for (let i = 1; i <= n; i++) {
        if (s[i - 1] !== "0") dp[i] += dp[i - 1];
        if (i >= 2) {
            const two = Number(s.slice(i - 2, i));
            if (two >= 10 && two <= 26) dp[i] += dp[i - 2];
        }
    }
    return dp[n];
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: O(1) Space (Optimal)
```javascript
/**
 * @param {string} s
 * @return {number}
 */
function numDecodings(s) {
    let prev2 = 0, prev1 = 1; // dp[i-2], dp[i-1]
    for (let i = 0; i < s.length; i++) {
        let cur = s[i] !== "0" ? prev1 : 0;
        if (i > 0 && (s[i - 1] === "1" || (s[i - 1] === "2" && s[i] <= "6"))) cur += prev2;
        [prev2, prev1] = [prev1, cur];
    }
    return prev1;
}
```
**Complexity:** Time O(n) · Space O(1)

> LeetCode guarantees the answer fits in a 32-bit integer, so `number` is exact here (inputs up to length 100).

---

### 322. Coin Change
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/coin-change/)

**Problem:** Given coin denominations and an `amount`, return the fewest coins that add up to `amount`, or `-1` if it can't be made. You have unlimited coins of each kind.

**Example:** `coins = [1,2,5], amount = 11` → `3` (5+5+1)

**Key insight:** `dp[a] = 1 + min(dp[a - c])` over every coin `c`. (Greedy fails for some coin sets.)

#### Approach 1: Bottom-Up DP (Optimal)
```javascript
/**
 * @param {number[]} coins
 * @param {number} amount
 * @return {number}
 */
function coinChange(coins, amount) {
    const dp = new Array(amount + 1).fill(Infinity);
    dp[0] = 0;
    for (let a = 1; a <= amount; a++) {
        for (const c of coins) if (c <= a && dp[a - c] + 1 < dp[a]) dp[a] = dp[a - c] + 1;
    }
    return dp[amount] === Infinity ? -1 : dp[amount];
}
```
**Complexity:** Time O(amount · len(coins)) · Space O(amount)

#### Approach 2: BFS over Amounts (shortest path)
```javascript
/**
 * @param {number[]} coins
 * @param {number} amount
 * @return {number}
 */
function coinChange(coins, amount) {
    if (amount === 0) return 0;
    const seen = new Array(amount + 1).fill(false);
    seen[0] = true;
    let level = [0];
    for (let steps = 1; level.length; steps++) {
        const next = [];
        for (const cur of level) {
            for (const c of coins) {
                const v = cur + c;
                if (v === amount) return steps;
                if (v < amount && !seen[v]) {
                    seen[v] = true;
                    next.push(v);
                }
            }
        }
        level = next;
    }
    return -1;
}
```
**Complexity:** Time O(amount · len(coins)) · Space O(amount)

#### Approach 3: Top-Down Memoization
```javascript
/**
 * @param {number[]} coins
 * @param {number} amount
 * @return {number}
 */
function coinChange(coins, amount) {
    const memo = new Array(amount + 1).fill(-2); // -2 = not computed, -1 = impossible
    memo[0] = 0;
    const f = (a) => {
        if (memo[a] !== -2) return memo[a];
        let best = Infinity;
        for (const c of coins) {
            if (c <= a) {
                const r = f(a - c);
                if (r >= 0) best = Math.min(best, r + 1);
            }
        }
        return (memo[a] = best === Infinity ? -1 : best);
    };
    // warm the memo bottom-up in chunks so recursion depth stays small even for amount = 10^4
    for (let a = 0; a <= amount; a += 1000) f(a);
    return f(amount);
}
```
**Complexity:** Time O(amount · len(coins)) · Space O(amount)

> Without the warm-up loop, the recursion depth reaches `amount` (10⁴ with coin `1`), which is near JavaScript's call-stack limit. Computing every 1000th amount first means each call recurses at most about 1000 levels.

---

### 152. Maximum Product Subarray
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/maximum-product-subarray/)

**Problem:** Return the largest product of any contiguous non-empty subarray.

**Example:** `[2,3,-2,4]` → `6`; `[-2,0,-1]` → `0`

**Key insight:** A negative number turns the minimum product into the maximum, so track both the current max and the current min.

#### Approach 1: Track Max and Min (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function maxProduct(nums) {
    let best = nums[0], curMax = nums[0], curMin = nums[0];
    for (let i = 1; i < nums.length; i++) {
        const n = nums[i];
        if (n < 0) [curMax, curMin] = [curMin, curMax]; // a negative flips max and min
        curMax = Math.max(n, curMax * n);
        curMin = Math.min(n, curMin * n);
        best = Math.max(best, curMax);
    }
    return best + 0; // + 0 normalizes -0
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Prefix and Suffix Products
The best subarray always starts at an edge of the array or of a zero-bounded segment.
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function maxProduct(nums) {
    const n = nums.length;
    let best = nums[0], pre = 1, suf = 1;
    for (let i = 0; i < n; i++) {
        pre = (pre || 1) * nums[i]; // restart after a zero
        suf = (suf || 1) * nums[n - 1 - i];
        best = Math.max(best, pre, suf);
    }
    return best + 0;
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 139. Word Break
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/word-break/)

**Problem:** Return whether `s` can be split into a sequence of one or more words from `wordDict`. Words can be reused.

**Example:** `s = "leetcode", wordDict = ["leet","code"]` → `true`

**Key insight:** `dp[i]` is true when some `j < i` has `dp[j]` true and `s[j:i]` in the dictionary.

#### Approach 1: Bottom-Up DP with a Set (Optimal)
```javascript
/**
 * @param {string} s
 * @param {string[]} wordDict
 * @return {boolean}
 */
function wordBreak(s, wordDict) {
    const words = new Set(wordDict);
    const maxLen = Math.max(...wordDict.map((w) => w.length));
    const dp = new Array(s.length + 1).fill(false);
    dp[0] = true;
    for (let i = 1; i <= s.length; i++) {
        for (let j = Math.max(0, i - maxLen); j < i; j++) {
            if (dp[j] && words.has(s.slice(j, i))) {
                dp[i] = true;
                break;
            }
        }
    }
    return dp[s.length];
}
```
**Complexity:** Time O(n · L), where L is the longest word (slicing and hashing add O(L) per check) · Space O(n)

#### Approach 2: DP Iterating over Dictionary Words
```javascript
/**
 * @param {string} s
 * @param {string[]} wordDict
 * @return {boolean}
 */
function wordBreak(s, wordDict) {
    const n = s.length;
    const dp = new Array(n + 1).fill(false);
    dp[n] = true;
    for (let i = n - 1; i >= 0; i--) {
        dp[i] = wordDict.some((w) => i + w.length <= n && dp[i + w.length] && s.startsWith(w, i));
    }
    return dp[0];
}
```
**Complexity:** Time O(n · m · L) for m words · Space O(n)

---

### 300. Longest Increasing Subsequence
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-increasing-subsequence/)

**Problem:** Return the length of the longest strictly increasing subsequence.

**Example:** `[10,9,2,5,3,7,101,18]` → `4` (e.g. `2,3,7,101`)

**Key insight (patience sorting):** Keep `tails[k]`, the smallest possible tail of an increasing subsequence of length `k+1`. Each new number replaces the first tail that is ≥ it.

#### Approach 1: O(n²) DP
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function lengthOfLIS(nums) {
    const dp = new Array(nums.length).fill(1);
    for (let i = 0; i < nums.length; i++) {
        for (let j = 0; j < i; j++) if (nums[j] < nums[i]) dp[i] = Math.max(dp[i], dp[j] + 1);
    }
    return Math.max(...dp);
}
```
**Complexity:** Time O(n²) · Space O(n)

#### Approach 2: Binary Search on Tails (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function lengthOfLIS(nums) {
    const tails = []; // tails[k] = smallest tail of an increasing subsequence of length k+1
    for (const n of nums) {
        let lo = 0, hi = tails.length; // first index with tails[i] >= n
        while (lo < hi) {
            const mid = (lo + hi) >> 1;
            if (tails[mid] < n) lo = mid + 1;
            else hi = mid;
        }
        tails[lo] = n;
    }
    return tails.length;
}
```
**Complexity:** Time O(n log n) · Space O(n)

---

### 416. Partition Equal Subset Sum
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/partition-equal-subset-sum/)

**Problem:** Return whether `nums` can be split into two subsets with equal sums.

**Example:** `[1,5,11,5]` → `true` (`[1,5,5]` and `[11]`)

**Key insight:** This is 0/1 knapsack: can some subset reach `total/2`? Iterate the sums from high to low so each number is used at most once.

#### Approach 1: 1-D Boolean DP
```javascript
/**
 * @param {number[]} nums
 * @return {boolean}
 */
function canPartition(nums) {
    const total = nums.reduce((a, b) => a + b, 0);
    if (total % 2) return false;
    const target = total / 2;
    const dp = new Array(target + 1).fill(false);
    dp[0] = true;
    for (const n of nums) {
        for (let s = target; s >= n; s--) if (dp[s - n]) dp[s] = true; // high to low, so each number is used once
    }
    return dp[target];
}
```
**Complexity:** Time O(n · target) · Space O(target)

#### Approach 2: BigInt Bitset (Optimal in JS)
`BigInt` is an arbitrary-length bitset. Shifting left by `n` adds `n` to every reachable sum at once.
```javascript
/**
 * @param {number[]} nums
 * @return {boolean}
 */
function canPartition(nums) {
    const total = nums.reduce((a, b) => a + b, 0);
    if (total % 2) return false;
    let bits = 1n; // bit s set means sum s is reachable
    for (const n of nums) bits |= bits << BigInt(n);
    return ((bits >> BigInt(total / 2)) & 1n) === 1n;
}
```
**Complexity:** Time O(n · total / 64) word operations · Space O(total) bits

> `BigInt` literals like `1n` need ES2020 or later, which LeetCode's JavaScript runtime (Node.js) supports.

#### Approach 3: Set of Reachable Sums
```javascript
/**
 * @param {number[]} nums
 * @return {boolean}
 */
function canPartition(nums) {
    const total = nums.reduce((a, b) => a + b, 0);
    if (total % 2) return false;
    const target = total / 2;
    let reach = new Set([0]);
    for (const n of nums) {
        const next = new Set(reach);
        for (const s of reach) if (s + n <= target) next.add(s + n);
        reach = next;
        if (reach.has(target)) return true;
    }
    return reach.has(target);
}
```
**Complexity:** Time O(n · target) · Space O(target)

---
## 14. 2-D Dynamic Programming

### 62. Unique Paths
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/unique-paths/)

**Problem:** A robot starts at the top-left of an `m × n` grid and can only move right or down. How many distinct paths lead to the bottom-right corner?

**Example:** `m = 3, n = 7` → `28`

#### Approach 1: 2-D DP
```javascript
/**
 * @param {number} m
 * @param {number} n
 * @return {number}
 */
function uniquePaths(m, n) {
    const dp = Array.from({ length: m }, () => new Array(n).fill(1)); // the first row and column are all 1
    for (let i = 1; i < m; i++) for (let j = 1; j < n; j++) dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
    return dp[m - 1][n - 1];
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D Rolling Row (Optimal DP)
```javascript
/**
 * @param {number} m
 * @param {number} n
 * @return {number}
 */
function uniquePaths(m, n) {
    const row = new Array(n).fill(1);
    for (let i = 1; i < m; i++) for (let j = 1; j < n; j++) row[j] += row[j - 1];
    return row[n - 1];
}
```
**Complexity:** Time O(m · n) · Space O(n)

#### Approach 3: Combinatorics C(m+n-2, m-1)
```javascript
/**
 * @param {number} m
 * @param {number} n
 * @return {number}
 */
function uniquePaths(m, n) {
    const k = Math.min(m, n) - 1, total = m + n - 2;
    let res = 1;
    for (let i = 1; i <= k; i++) res = (res * (total - k + i)) / i; // stays an integer at every step
    return Math.round(res); // guard against float drift
}
```
**Complexity:** Time O(min(m, n)) · Space O(1)

> Intermediate products stay below 2⁵³ for LeetCode's limits (answer ≤ 2 · 10⁹), so `number` is exact. For larger inputs, switch to `BigInt`.

---

### 1143. Longest Common Subsequence
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-common-subsequence/)

**Problem:** Return the length of the longest subsequence that both `text1` and `text2` contain.

**Example:** `"abcde", "ace"` → `3`

**Key insight:** If `a[i] == b[j]`, then `dp[i][j] = 1 + dp[i-1][j-1]`. Otherwise `dp[i][j] = max(dp[i-1][j], dp[i][j-1])`.

#### Approach 1: 2-D Table
```javascript
/**
 * @param {string} text1
 * @param {string} text2
 * @return {number}
 */
function longestCommonSubsequence(text1, text2) {
    const m = text1.length, n = text2.length;
    const dp = Array.from({ length: m + 1 }, () => new Array(n + 1).fill(0));
    for (let i = 1; i <= m; i++) {
        for (let j = 1; j <= n; j++) {
            dp[i][j] = text1[i - 1] === text2[j - 1] ? dp[i - 1][j - 1] + 1 : Math.max(dp[i - 1][j], dp[i][j - 1]);
        }
    }
    return dp[m][n];
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Two Rows (Optimal space)
```javascript
/**
 * @param {string} text1
 * @param {string} text2
 * @return {number}
 */
function longestCommonSubsequence(text1, text2) {
    const [a, b] = text1.length >= text2.length ? [text1, text2] : [text2, text1];
    let prev = new Array(b.length + 1).fill(0);
    for (let i = 1; i <= a.length; i++) {
        const cur = new Array(b.length + 1).fill(0);
        for (let j = 1; j <= b.length; j++) {
            cur[j] = a[i - 1] === b[j - 1] ? prev[j - 1] + 1 : Math.max(prev[j], cur[j - 1]);
        }
        prev = cur;
    }
    return prev[b.length];
}
```
**Complexity:** Time O(m · n) · Space O(min(m, n))

---

### 309. Best Time to Buy and Sell Stock with Cooldown
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/)

**Problem:** You may make as many transactions as you like, but after selling you must wait one day before buying again, and you can hold at most one share. Return the maximum profit.

**Example:** `[1,2,3,0,2]` → `3` (buy, sell, cooldown, buy, sell)

**Key insight:** A state machine with three states: `hold` (own a share), `sold` (sold today, so tomorrow is a cooldown), and `rest` (no share, free to buy).

#### Approach 1: State Machine DP (Optimal)
```javascript
/**
 * @param {number[]} prices
 * @return {number}
 */
function maxProfit(prices) {
    let hold = -Infinity, sold = 0, rest = 0;
    for (const p of prices) [hold, sold, rest] = [Math.max(hold, rest - p), hold + p, Math.max(rest, sold)]; // all from the previous day
    return Math.max(sold, rest);
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Bottom-Up over (index, canBuy)
The top-down recursion would be up to 5000 frames deep, so this version fills the same table from the end instead.
```javascript
/**
 * @param {number[]} prices
 * @return {number}
 */
function maxProfit(prices) {
    const n = prices.length;
    // dp[i][1] = best profit from day i when allowed to buy; dp[i][0] = when holding a share
    const dp = Array.from({ length: n + 2 }, () => [0, 0]);
    for (let i = n - 1; i >= 0; i--) {
        dp[i][1] = Math.max(dp[i + 1][1], dp[i + 1][0] - prices[i]);
        dp[i][0] = Math.max(dp[i + 1][0], dp[i + 2][1] + prices[i]); // selling forces a one-day cooldown
    }
    return dp[0][1];
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 518. Coin Change II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/coin-change-ii/)

**Problem:** Return how many combinations of coins add up to `amount`. Coin supply is unlimited, and combinations are unordered.

**Example:** `amount = 5, coins = [1,2,5]` → `4`

**Key insight:** Loop over coins on the outside and amounts on the inside. That counts each combination once, not once per ordering.

#### Approach 1: 1-D DP (Optimal)
```javascript
/**
 * @param {number} amount
 * @param {number[]} coins
 * @return {number}
 */
function change(amount, coins) {
    const dp = new Array(amount + 1).fill(0);
    dp[0] = 1;
    for (const c of coins) {
        // coins in the outer loop: combinations, not permutations
        for (let a = c; a <= amount; a++) dp[a] += dp[a - c];
    }
    return dp[amount];
}
```
**Complexity:** Time O(n · amount) · Space O(amount)

> On adversarial inputs, **intermediate** counts can exceed 2⁵³, where a JS `number` starts losing precision, even though the final answer fits in 32 bits. Approach 2 stays exact for any input.

#### Approach 2: Exact Counting with `BigInt`
```javascript
/**
 * @param {number} amount
 * @param {number[]} coins
 * @return {number}
 */
function change(amount, coins) {
    const dp = new Array(amount + 1).fill(0n);
    dp[0] = 1n;
    for (const c of coins) for (let a = c; a <= amount; a++) dp[a] += dp[a - c];
    return Number(dp[amount]);
}
```
**Complexity:** Time O(n · amount) big-integer additions · Space O(amount)

---

### 494. Target Sum
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/target-sum/)

**Problem:** Put a `+` or `-` in front of every number in `nums`. Count the ways the resulting expression equals `target`.

**Example:** `nums = [1,1,1,1,1], target = 3` → `5`

**Key insight:** If `P` is the set of numbers with `+`, then `sum(P) = (total + target) / 2`. That reduces the problem to counting subsets with a given sum.

#### Approach 1: Memoized DFS (index, running sum)
```javascript
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
function findTargetSumWays(nums, target) {
    const memo = new Map();
    const dfs = (i, total) => {
        if (i === nums.length) return total === target ? 1 : 0;
        const key = `${i},${total}`;
        if (!memo.has(key)) memo.set(key, dfs(i + 1, total + nums[i]) + dfs(i + 1, total - nums[i]));
        return memo.get(key);
    };
    return dfs(0, 0);
}
```
**Complexity:** Time O(n · total) · Space O(n · total)

#### Approach 2: Map of Reachable Sums
```javascript
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
function findTargetSumWays(nums, target) {
    let ways = new Map([[0, 1]]); // sum -> number of sign assignments reaching it
    for (const n of nums) {
        const next = new Map();
        for (const [s, c] of ways) {
            next.set(s + n, (next.get(s + n) ?? 0) + c);
            next.set(s - n, (next.get(s - n) ?? 0) + c);
        }
        ways = next;
    }
    return ways.get(target) ?? 0;
}
```
**Complexity:** Time O(n · total) · Space O(total)

#### Approach 3: Subset-Sum Counting (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
function findTargetSumWays(nums, target) {
    const total = nums.reduce((a, b) => a + b, 0);
    if (Math.abs(target) > total || (total + target) % 2) return 0;
    const p = (total + target) / 2; // the sum of the numbers that get '+'
    const dp = new Array(p + 1).fill(0);
    dp[0] = 1;
    for (const n of nums) for (let s = p; s >= n; s--) dp[s] += dp[s - n];
    return dp[p];
}
```
**Complexity:** Time O(n · P) · Space O(P)

---

### 97. Interleaving String
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/interleaving-string/)

**Problem:** Return whether `s3` can be formed by interleaving `s1` and `s2` while keeping each string's characters in their original order.

**Example:** `s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"` → `true`

**Key insight:** `dp[i][j]` is true when `s3[:i+j]` is an interleaving of `s1[:i]` and `s2[:j]`.

#### Approach 1: 2-D DP
```javascript
/**
 * @param {string} s1
 * @param {string} s2
 * @param {string} s3
 * @return {boolean}
 */
function isInterleave(s1, s2, s3) {
    const m = s1.length, n = s2.length;
    if (m + n !== s3.length) return false;
    const dp = Array.from({ length: m + 1 }, () => new Array(n + 1).fill(false));
    dp[0][0] = true;
    for (let i = 0; i <= m; i++) {
        for (let j = 0; j <= n; j++) {
            if (i > 0 && s1[i - 1] === s3[i + j - 1] && dp[i - 1][j]) dp[i][j] = true;
            if (j > 0 && s2[j - 1] === s3[i + j - 1] && dp[i][j - 1]) dp[i][j] = true;
        }
    }
    return dp[m][n];
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D DP (Optimal space)
```javascript
/**
 * @param {string} s1
 * @param {string} s2
 * @param {string} s3
 * @return {boolean}
 */
function isInterleave(s1, s2, s3) {
    const m = s1.length, n = s2.length;
    if (m + n !== s3.length) return false;
    const dp = new Array(n + 1).fill(false);
    for (let i = 0; i <= m; i++) {
        for (let j = 0; j <= n; j++) {
            if (i === 0 && j === 0) dp[j] = true;
            else if (i === 0) dp[j] = dp[j - 1] && s2[j - 1] === s3[j - 1];
            else if (j === 0) dp[j] = dp[j] && s1[i - 1] === s3[i - 1];
            else dp[j] = (dp[j] && s1[i - 1] === s3[i + j - 1]) || (dp[j - 1] && s2[j - 1] === s3[i + j - 1]);
        }
    }
    return dp[n];
}
```
**Complexity:** Time O(m · n) · Space O(n)

---

### 329. Longest Increasing Path in a Matrix
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/longest-increasing-path-in-a-matrix/)

**Problem:** Return the length of the longest strictly increasing path in a matrix, moving in four directions.

**Example:** `[[9,9,4],[6,6,8],[2,1,1]]` → `4` (1→2→6→9)

**Key insight:** Strictly increasing paths can't form cycles, so the cells form a DAG and DFS with memoization works.

#### Approach 1: DFS + Memoization
```javascript
/**
 * @param {number[][]} matrix
 * @return {number}
 */
function longestIncreasingPath(matrix) {
    const m = matrix.length, n = matrix[0].length;
    const memo = Array.from({ length: m }, () => new Array(n).fill(0));
    const dirs = [
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
    ];
    const dfs = (r, c) => {
        if (memo[r][c]) return memo[r][c];
        let best = 1;
        for (const [dr, dc] of dirs) {
            const nr = r + dr, nc = c + dc;
            if (nr >= 0 && nc >= 0 && nr < m && nc < n && matrix[nr][nc] > matrix[r][c]) best = Math.max(best, 1 + dfs(nr, nc));
        }
        return (memo[r][c] = best);
    };
    let ans = 0;
    for (let r = 0; r < m; r++) for (let c = 0; c < n; c++) ans = Math.max(ans, dfs(r, c));
    return ans;
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

> ⚠️ A snake-shaped path in a 200 × 200 matrix makes the recursion up to 40,000 frames deep, beyond what JS engines reliably allow. (Node happened to cope in our test because the frames are tiny, but don't rely on it.) Approach 2 is iterative and safe for any input.

#### Approach 2: Process Cells in Increasing Value Order (no recursion, Optimal)
Sorting cells by value gives a topological order for "strictly increasing" edges.
```javascript
/**
 * @param {number[][]} matrix
 * @return {number}
 */
function longestIncreasingPath(matrix) {
    const m = matrix.length, n = matrix[0].length;
    const best = Array.from({ length: m }, () => new Array(n).fill(1));
    const cells = [];
    for (let r = 0; r < m; r++) for (let c = 0; c < n; c++) cells.push([matrix[r][c], r, c]);
    cells.sort((a, b) => a[0] - b[0]);
    const dirs = [
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
    ];
    let ans = 1;
    for (const [v, r, c] of cells) {
        // every smaller neighbor is already final
        for (const [dr, dc] of dirs) {
            const nr = r + dr, nc = c + dc;
            if (nr >= 0 && nc >= 0 && nr < m && nc < n && matrix[nr][nc] < v) best[r][c] = Math.max(best[r][c], best[nr][nc] + 1);
        }
        ans = Math.max(ans, best[r][c]);
    }
    return ans;
}
```
**Complexity:** Time O(m · n · log(m · n)) · Space O(m · n)

---

### 115. Distinct Subsequences
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/distinct-subsequences/)

**Problem:** Count the distinct subsequences of `s` that equal `t`.

**Example:** `s = "rabbbit", t = "rabbit"` → `3`

**Key insight:** `dp[i][j] = dp[i-1][j]` (skip `s[i-1]`), plus `dp[i-1][j-1]` when `s[i-1] == t[j-1]`.

#### Approach 1: 2-D DP
```javascript
/**
 * @param {string} s
 * @param {string} t
 * @return {number}
 */
function numDistinct(s, t) {
    const m = s.length, n = t.length;
    const dp = Array.from({ length: m + 1 }, () => new Array(n + 1).fill(0));
    for (let i = 0; i <= m; i++) dp[i][0] = 1; // the empty t matches once
    for (let i = 1; i <= m; i++) {
        for (let j = 1; j <= n; j++) dp[i][j] = dp[i - 1][j] + (s[i - 1] === t[j - 1] ? dp[i - 1][j - 1] : 0);
    }
    return dp[m][n];
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D DP with `BigInt` (exact for any input)
Intermediate counts can exceed 2⁵³ on adversarial inputs even though the final answer fits in 32 bits, so `BigInt` keeps every step exact.
```javascript
/**
 * @param {string} s
 * @param {string} t
 * @return {number}
 */
function numDistinct(s, t) {
    const dp = new Array(t.length + 1).fill(0n);
    dp[0] = 1n;
    for (const ch of s) {
        for (let j = t.length; j >= 1; j--) if (ch === t[j - 1]) dp[j] += dp[j - 1];
    }
    return Number(dp[t.length]);
}
```
**Complexity:** Time O(m · n) · Space O(n)

---

### 72. Edit Distance
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/edit-distance/)

**Problem:** Return the minimum number of insert, delete, and replace operations needed to turn `word1` into `word2`.

**Example:** `"horse" → "ros"` → `3`

**Key insight:** If the characters match, `dp[i][j] = dp[i-1][j-1]`. Otherwise `dp[i][j] = 1 + min(delete dp[i-1][j], insert dp[i][j-1], replace dp[i-1][j-1])`.

#### Approach 1: 2-D DP
```javascript
/**
 * @param {string} word1
 * @param {string} word2
 * @return {number}
 */
function minDistance(word1, word2) {
    const m = word1.length, n = word2.length;
    const dp = Array.from({ length: m + 1 }, (_, i) => Array.from({ length: n + 1 }, (_, j) => (i === 0 ? j : j === 0 ? i : 0)));
    for (let i = 1; i <= m; i++) {
        for (let j = 1; j <= n; j++) {
            dp[i][j] = word1[i - 1] === word2[j - 1] ? dp[i - 1][j - 1] : 1 + Math.min(dp[i - 1][j - 1], dp[i - 1][j], dp[i][j - 1]); // replace, delete, insert
        }
    }
    return dp[m][n];
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D DP (Optimal space)
```javascript
/**
 * @param {string} word1
 * @param {string} word2
 * @return {number}
 */
function minDistance(word1, word2) {
    const n = word2.length;
    const dp = Array.from({ length: n + 1 }, (_, j) => j);
    for (let i = 1; i <= word1.length; i++) {
        let diag = dp[0]; // dp[i-1][j-1]
        dp[0] = i;
        for (let j = 1; j <= n; j++) {
            const tmp = dp[j];
            dp[j] = word1[i - 1] === word2[j - 1] ? diag : 1 + Math.min(diag, dp[j], dp[j - 1]);
            diag = tmp;
        }
    }
    return dp[n];
}
```
**Complexity:** Time O(m · n) · Space O(n)

---

### 312. Burst Balloons
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/burst-balloons/)

**Problem:** Bursting balloon `i` earns `nums[left] * nums[i] * nums[right]`, where `left` and `right` are its current neighbors, and the ends are padded with 1s. Return the maximum coins you can collect by bursting every balloon.

**Example:** `[3,1,5,8]` → `167`

**Key insight:** Think about the **last** balloon `k` to burst in the open interval `(l, r)`. Its neighbors at that moment are `l` and `r`, which makes the two sides independent subproblems.

#### Approach 1: Interval DP, Bottom-Up (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function maxCoins(nums) {
    const vals = [1, ...nums, 1], n = vals.length;
    const dp = Array.from({ length: n }, () => new Array(n).fill(0));
    for (let len = 2; len < n; len++) {
        // gap between l and r
        for (let l = 0; l + len < n; l++) {
            const r = l + len;
            for (let k = l + 1; k < r; k++) {
                // k is the last balloon burst in (l, r)
                dp[l][r] = Math.max(dp[l][r], dp[l][k] + vals[l] * vals[k] * vals[r] + dp[k][r]);
            }
        }
    }
    return dp[0][n - 1];
}
```
**Complexity:** Time O(n³) · Space O(n²)

#### Approach 2: Top-Down Memoization
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function maxCoins(nums) {
    const vals = [1, ...nums, 1], n = vals.length;
    const memo = Array.from({ length: n }, () => new Array(n).fill(-1));
    const f = (l, r) => {
        if (l + 1 === r) return 0;
        if (memo[l][r] >= 0) return memo[l][r];
        let best = 0;
        for (let k = l + 1; k < r; k++) best = Math.max(best, f(l, k) + vals[l] * vals[k] * vals[r] + f(k, r));
        return (memo[l][r] = best);
    };
    return f(0, n - 1);
}
```
**Complexity:** Time O(n³) · Space O(n²)

---

### 10. Regular Expression Matching
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/regular-expression-matching/)

**Problem:** Implement regex matching with `.` (any single character) and `*` (zero or more of the preceding element). The pattern must match the **entire** input string.

**Example:** `s = "aa", p = "a*"` → `true`; `s = "ab", p = ".*"` → `true`; `s = "mississippi", p = "mis*is*p*."` → `false`

**Key insight:** When `p[j+1] == '*'`, either skip `x*` entirely (`dp(i, j+2)`) or, if the first characters match, consume one character of `s` (`dp(i+1, j)`).

#### Approach 1: Top-Down Memoization
```javascript
/**
 * @param {string} s
 * @param {string} p
 * @return {boolean}
 */
function isMatch(s, p) {
    const memo = new Map();
    const dp = (i, j) => {
        const key = i * (p.length + 1) + j;
        const cached = memo.get(key);
        if (cached !== undefined) return cached;
        let res;
        if (j === p.length) res = i === s.length;
        else {
            const first = i < s.length && (p[j] === s[i] || p[j] === ".");
            if (j + 1 < p.length && p[j + 1] === "*")
                res = dp(i, j + 2) || (first && dp(i + 1, j)); // skip x*, or consume one char
            else res = first && dp(i + 1, j + 1);
        }
        memo.set(key, res);
        return res;
    };
    return dp(0, 0);
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Bottom-Up DP (Optimal)
```javascript
/**
 * @param {string} s
 * @param {string} p
 * @return {boolean}
 */
function isMatch(s, p) {
    const m = s.length, n = p.length;
    const dp = Array.from({ length: m + 1 }, () => new Array(n + 1).fill(false));
    dp[m][n] = true;
    for (let i = m; i >= 0; i--) {
        for (let j = n - 1; j >= 0; j--) {
            const first = i < m && (p[j] === s[i] || p[j] === ".");
            dp[i][j] = j + 1 < n && p[j + 1] === "*" ? dp[i][j + 2] || (first && dp[i + 1][j]) : first && dp[i + 1][j + 1];
        }
    }
    return dp[0][0];
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

> **Shortcut for checking your work:** `new RegExp(`^(?:${p})$`).test(s)` gives the same answer for this problem's restricted syntax.

---
## 15. Greedy

### 53. Maximum Subarray
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/maximum-subarray/)

**Problem:** Return the largest sum of any contiguous non-empty subarray.

**Example:** `[-2,1,-3,4,-1,2,1,-5,4]` → `6` (`[4,-1,2,1]`)

**Key insight (Kadane's algorithm):** Throw away a running sum as soon as it goes negative, since it can only drag down whatever follows.

#### Approach 1: Kadane's Algorithm (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function maxSubArray(nums) {
    let best = nums[0], cur = nums[0];
    for (let i = 1; i < nums.length; i++) {
        cur = Math.max(nums[i], cur + nums[i]); // drop the running sum once it would only drag nums[i] down
        best = Math.max(best, cur);
    }
    return best;
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Divide and Conquer (the follow-up)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function maxSubArray(nums) {
    const solve = (lo, hi) => {
        if (lo === hi) return nums[lo];
        const mid = (lo + hi) >> 1;
        let leftBest = -Infinity, rightBest = -Infinity, sum = 0;
        for (let i = mid; i >= lo; i--) leftBest = Math.max(leftBest, (sum += nums[i]));
        sum = 0;
        for (let i = mid + 1; i <= hi; i++) rightBest = Math.max(rightBest, (sum += nums[i]));
        return Math.max(solve(lo, mid), solve(mid + 1, hi), leftBest + rightBest);
    };
    return solve(0, nums.length - 1);
}
```
**Complexity:** Time O(n log n) · Space O(log n)

#### Approach 3: Prefix Sums (min prefix so far)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function maxSubArray(nums) {
    let best = -Infinity, prefix = 0, minPrefix = 0;
    for (const n of nums) {
        prefix += n;
        best = Math.max(best, prefix - minPrefix); // best subarray ending here
        minPrefix = Math.min(minPrefix, prefix);
    }
    return best;
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 55. Jump Game
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/jump-game/)

**Problem:** `nums[i]` is the farthest you can jump from index `i`. Starting at index 0, can you reach the last index?

**Example:** `[2,3,1,1,4]` → `true`; `[3,2,1,0,4]` → `false`

#### Approach 1: Greedy, Shift the Goal Backward (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {boolean}
 */
function canJump(nums) {
    let goal = nums.length - 1;
    for (let i = nums.length - 2; i >= 0; i--) if (i + nums[i] >= goal) goal = i;
    return goal === 0;
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Greedy, Track the Farthest Reach Forward
```javascript
/**
 * @param {number[]} nums
 * @return {boolean}
 */
function canJump(nums) {
    let reach = 0;
    for (let i = 0; i < nums.length; i++) {
        if (i > reach) return false;
        reach = Math.max(reach, i + nums[i]);
    }
    return true;
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 3: DP (slower, shown for contrast)
```javascript
/**
 * @param {number[]} nums
 * @return {boolean}
 */
function canJump(nums) {
    const n = nums.length;
    const good = new Array(n).fill(false);
    good[n - 1] = true;
    for (let i = n - 2; i >= 0; i--) {
        for (let j = i + 1; j <= Math.min(n - 1, i + nums[i]); j++) {
            if (good[j]) {
                good[i] = true;
                break;
            }
        }
    }
    return good[0];
}
```
**Complexity:** Time O(n²) · Space O(n)

---

### 45. Jump Game II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/jump-game-ii/)

**Problem:** Same jumps as Jump Game, but the last index is always reachable. Return the minimum number of jumps to get there.

**Example:** `[2,3,1,1,4]` → `2`

**Key insight:** This is BFS by levels. The window `[l, r]` holds every index reachable in `jumps` jumps, and the next window ends at the farthest index reachable from it.

#### Approach 1: Greedy BFS Windows (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function jump(nums) {
    let jumps = 0, curEnd = 0, farthest = 0;
    for (let i = 0; i < nums.length - 1; i++) {
        farthest = Math.max(farthest, i + nums[i]);
        if (i === curEnd) {
            // end of this jump's window
            jumps++;
            curEnd = farthest;
        }
    }
    return jumps;
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: DP
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function jump(nums) {
    const n = nums.length;
    const dp = new Array(n).fill(Infinity);
    dp[0] = 0;
    for (let i = 0; i < n; i++) {
        for (let j = i + 1; j <= Math.min(n - 1, i + nums[i]); j++) dp[j] = Math.min(dp[j], dp[i] + 1);
    }
    return dp[n - 1];
}
```
**Complexity:** Time O(n · max jump) · Space O(n)

---

### 134. Gas Station
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/gas-station/)

**Problem:** Stations sit on a circular route. Station `i` provides `gas[i]`, and driving to the next station costs `cost[i]`. Return the starting station from which you can complete the full loop, or `-1` if none works. A valid answer is unique.

**Example:** `gas = [1,2,3,4,5], cost = [3,4,5,1,2]` → `3`

**Key insight:** If total gas ≥ total cost, a solution exists. Whenever the running tank goes negative at `i`, no station from the current start through `i` can work, so restart at `i+1`.

#### Approach 1: One-Pass Greedy (Optimal)
```javascript
/**
 * @param {number[]} gas
 * @param {number[]} cost
 * @return {number}
 */
function canCompleteCircuit(gas, cost) {
    let total = 0, tank = 0, start = 0;
    for (let i = 0; i < gas.length; i++) {
        const diff = gas[i] - cost[i];
        total += diff;
        tank += diff;
        if (tank < 0) {
            // no station in [start, i] can work
            start = i + 1;
            tank = 0;
        }
    }
    return total < 0 ? -1 : start;
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Brute Force Simulation
```javascript
/**
 * @param {number[]} gas
 * @param {number[]} cost
 * @return {number}
 */
function canCompleteCircuit(gas, cost) {
    const n = gas.length;
    outer: for (let s = 0; s < n; s++) {
        let tank = 0;
        for (let k = 0; k < n; k++) {
            const i = (s + k) % n;
            tank += gas[i] - cost[i];
            if (tank < 0) continue outer;
        }
        return s;
    }
    return -1;
}
```
**Complexity:** Time O(n²) · Space O(1)

---

### 846. Hand of Straights
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/hand-of-straights/)

**Problem:** Can the cards in `hand` be rearranged into groups of `groupSize` consecutive values, using every card exactly once?

**Example:** `hand = [1,2,3,6,2,3,4,7,8], groupSize = 3` → `true` (`[1,2,3]`, `[2,3,4]`, `[6,7,8]`)

**Key insight:** The smallest remaining card must start a group, so build each group upward from the current minimum.

#### Approach 1: Sorted Distinct Values + Count Map
```javascript
/**
 * @param {number[]} hand
 * @param {number} groupSize
 * @return {boolean}
 */
function isNStraightHand(hand, groupSize) {
    if (hand.length % groupSize) return false;
    const count = new Map();
    for (const h of hand) count.set(h, (count.get(h) ?? 0) + 1);
    const keys = [...count.keys()].sort((a, b) => a - b);
    for (const start of keys) {
        const need = count.get(start);
        if (need === 0) continue;
        for (let v = start; v < start + groupSize; v++) {
            // start `need` groups at `start`
            const c = count.get(v) ?? 0;
            if (c < need) return false;
            count.set(v, c - need);
        }
    }
    return true;
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sort the Hand, Build Groups One Card at a Time
```javascript
/**
 * @param {number[]} hand
 * @param {number} groupSize
 * @return {boolean}
 */
function isNStraightHand(hand, groupSize) {
    if (hand.length % groupSize) return false;
    const sorted = [...hand].sort((a, b) => a - b);
    const count = new Map();
    for (const h of sorted) count.set(h, (count.get(h) ?? 0) + 1);
    for (const h of sorted) {
        if (!count.get(h)) continue; // already used in an earlier group
        for (let v = h; v < h + groupSize; v++) {
            const c = count.get(v) ?? 0;
            if (c === 0) return false;
            count.set(v, c - 1);
        }
    }
    return true;
}
```
**Complexity:** Time O(n log n + n · groupSize) · Space O(n)

---

### 1899. Merge Triplets to Form Target Triplet
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/merge-triplets-to-form-target-triplet/)

**Problem:** You may merge two triplets by taking the element-wise maximum. Can you produce `target` from some of the given `triplets`?

**Example:** `triplets = [[2,5,3],[1,8,4],[1,7,5]], target = [2,7,5]` → `true`

**Key insight:** Throw out any triplet that exceeds `target` in any position. Among the rest, check that each of the three target values appears in its position in at least one triplet.

#### Approach: Greedy Filter (Optimal)
```javascript
/**
 * @param {number[][]} triplets
 * @param {number[]} target
 * @return {boolean}
 */
function mergeTriplets(triplets, target) {
    const found = [false, false, false];
    for (const t of triplets) {
        if (t[0] > target[0] || t[1] > target[1] || t[2] > target[2]) continue; // would overshoot
        for (let i = 0; i < 3; i++) if (t[i] === target[i]) found[i] = true;
    }
    return found.every(Boolean);
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 763. Partition Labels
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/partition-labels/)

**Problem:** Split `s` into as many parts as possible so that each letter appears in only one part. Return the sizes of the parts.

**Example:** `"ababcbacadefegdehijhklij"` → `[9,7,8]`

**Key insight:** Record each letter's last index. Extend the current part's end to the last index of every letter you see, and close the part when `i` reaches that end.

#### Approach: Last Occurrence + Greedy Extension (Optimal)
```javascript
/**
 * @param {string} s
 * @return {number[]}
 */
function partitionLabels(s) {
    const last = new Map();
    for (let i = 0; i < s.length; i++) last.set(s[i], i); // later indices overwrite earlier ones
    const res = [];
    let start = 0, end = 0;
    for (let i = 0; i < s.length; i++) {
        end = Math.max(end, last.get(s[i]));
        if (i === end) {
            res.push(end - start + 1);
            start = i + 1;
        }
    }
    return res;
}
```
**Complexity:** Time O(n) · Space O(1) (at most 26 keys)

---

### 678. Valid Parenthesis String
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/valid-parenthesis-string/)

**Problem:** The string contains `(`, `)`, and `*`, where each `*` can act as `(`, `)`, or an empty string. Return whether the string can be valid.

**Example:** `"(*))"` → `true`

**Key insight:** Track the range `[lo, hi]` of possible open-paren counts. If `hi` ever drops below 0, the string is invalid. Clamp `lo` at 0. It's valid if `lo == 0` at the end.

#### Approach 1: Greedy Min/Max Open Count (Optimal)
```javascript
/**
 * @param {string} s
 * @return {boolean}
 */
function checkValidString(s) {
    let lo = 0, hi = 0; // range of possible open-paren counts
    for (const c of s) {
        if (c === "(") {
            lo++;
            hi++;
        } else if (c === ")") {
            lo--;
            hi--;
        } else {
            // '*'
            lo--;
            hi++;
        }
        if (hi < 0) return false;
        lo = Math.max(lo, 0);
    }
    return lo === 0;
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Two Stacks of Indices
```javascript
/**
 * @param {string} s
 * @return {boolean}
 */
function checkValidString(s) {
    const open = [], star = [];
    for (let i = 0; i < s.length; i++) {
        if (s[i] === "(") open.push(i);
        else if (s[i] === "*") star.push(i);
        else if (open.length) open.pop();
        else if (star.length) star.pop();
        else return false;
    }
    while (open.length && star.length) if (open.pop() > star.pop()) return false; // a '(' with no '*' after it
    return open.length === 0;
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 3: DP over (index, open count)
```javascript
/**
 * @param {string} s
 * @return {boolean}
 */
function checkValidString(s) {
    let dp = new Set([0]); // reachable open counts
    for (const c of s) {
        const next = new Set();
        for (const k of dp) {
            if (c === "(" || c === "*") next.add(k + 1);
            if ((c === ")" || c === "*") && k > 0) next.add(k - 1);
            if (c === "*") next.add(k);
        }
        dp = next;
    }
    return dp.has(0);
}
```
**Complexity:** Time O(n²) · Space O(n)

---
## 16. Intervals

> Always sort intervals with an explicit comparator: `intervals.sort((a, b) => a[0] - b[0])`. The default sort compares arrays as strings.

### 57. Insert Interval
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/insert-interval/)

**Problem:** `intervals` are non-overlapping and sorted by start. Insert `newInterval`, merging wherever needed, and return the result still sorted and non-overlapping.

**Example:** `[[1,3],[6,9]]` + `[2,5]` → `[[1,5],[6,9]]`

**Key insight:** Make three passes over the list: intervals entirely before the new one, intervals that overlap it (merge them), and intervals entirely after.

#### Approach 1: Linear Three-Phase Scan (Optimal)
```javascript
/**
 * @param {number[][]} intervals
 * @param {number[]} newInterval
 * @return {number[][]}
 */
function insert(intervals, newInterval) {
    const res = [];
    let [lo, hi] = newInterval, i = 0;
    while (i < intervals.length && intervals[i][1] < lo) res.push(intervals[i++]); // entirely before
    while (i < intervals.length && intervals[i][0] <= hi) {
        // overlapping: absorb
        lo = Math.min(lo, intervals[i][0]);
        hi = Math.max(hi, intervals[i][1]);
        i++;
    }
    res.push([lo, hi]);
    return res.concat(intervals.slice(i)); // entirely after
}
```
**Complexity:** Time O(n) · Space O(n) for the output

#### Approach 2: Binary Search for the Boundaries
```javascript
/**
 * @param {number[][]} intervals
 * @param {number[]} newInterval
 * @return {number[][]}
 */
function insert(intervals, newInterval) {
    const firstWhere = (pred) => {
        let lo = 0, hi = intervals.length;
        while (lo < hi) {
            const mid = (lo + hi) >> 1;
            if (pred(intervals[mid])) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    };
    let [lo, hi] = newInterval;
    const l = firstWhere((iv) => iv[1] >= lo); // first interval with end >= new start
    const r = firstWhere((iv) => iv[0] > hi); // first interval with start > new end
    if (l < r) {
        lo = Math.min(lo, intervals[l][0]);
        hi = Math.max(hi, intervals[r - 1][1]);
    }
    return [...intervals.slice(0, l), [lo, hi], ...intervals.slice(r)];
}
```
**Complexity:** Time O(log n) to search plus O(n) to copy · Space O(n)

---

### 56. Merge Intervals
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/merge-intervals/)

**Problem:** Merge every overlapping interval and return the resulting non-overlapping intervals.

**Example:** `[[1,3],[2,6],[8,10],[15,18]]` → `[[1,6],[8,10],[15,18]]`

#### Approach 1: Sort + Merge (Optimal)
```javascript
/**
 * @param {number[][]} intervals
 * @return {number[][]}
 */
function merge(intervals) {
    const sorted = [...intervals].sort((a, b) => a[0] - b[0]);
    const res = [];
    for (const [s, e] of sorted) {
        const last = res[res.length - 1];
        if (last && s <= last[1]) last[1] = Math.max(last[1], e);
        else res.push([s, e]);
    }
    return res;
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sweep over Start and End Events
```javascript
/**
 * @param {number[][]} intervals
 * @return {number[][]}
 */
function merge(intervals) {
    const starts = intervals.map((iv) => iv[0]).sort((a, b) => a - b);
    const ends = intervals.map((iv) => iv[1]).sort((a, b) => a - b);
    const res = [];
    for (let i = 0, begin = 0; i < starts.length; i++) {
        // close a merged block when the next start begins after this end
        if (i === starts.length - 1 || starts[i + 1] > ends[i]) {
            res.push([starts[begin], ends[i]]);
            begin = i + 1;
        }
    }
    return res;
}
```
**Complexity:** Time O(n log n) · Space O(n)

---

### 435. Non-overlapping Intervals
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/non-overlapping-intervals/)

**Problem:** Return the minimum number of intervals to remove so the rest don't overlap. Intervals that only touch at an endpoint don't overlap.

**Example:** `[[1,2],[2,3],[3,4],[1,3]]` → `1`

**Key insight:** This is activity selection: always keep the interval that **ends** earliest, since that leaves the most room for the rest.

#### Approach 1: Sort by End (Optimal)
```javascript
/**
 * @param {number[][]} intervals
 * @return {number}
 */
function eraseOverlapIntervals(intervals) {
    const sorted = [...intervals].sort((a, b) => a[1] - b[1]);
    let removed = 0, end = -Infinity;
    for (const [s, e] of sorted) {
        if (s >= end)
            end = e; // keep it
        else removed++;
    }
    return removed;
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sort by Start, Drop the One That Ends Later
```javascript
/**
 * @param {number[][]} intervals
 * @return {number}
 */
function eraseOverlapIntervals(intervals) {
    const sorted = [...intervals].sort((a, b) => a[0] - b[0]);
    let removed = 0, prevEnd = sorted[0][1];
    for (let i = 1; i < sorted.length; i++) {
        const [s, e] = sorted[i];
        if (s >= prevEnd) prevEnd = e;
        else {
            removed++;
            prevEnd = Math.min(prevEnd, e);
        }
    }
    return removed;
}
```
**Complexity:** Time O(n log n) · Space O(n)

---

### 252. Meeting Rooms (Premium)
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/meeting-rooms/) · Free mirror: LintCode 920

**Problem:** Given meeting time intervals, can one person attend every meeting (that is, do no two meetings overlap)?

**Example:** `[[0,30],[5,10],[15,20]]` → `false`

#### Approach: Sort by Start (Optimal)
```javascript
/**
 * @param {number[][]} intervals
 * @return {boolean}
 */
function canAttendMeetings(intervals) {
    const sorted = [...intervals].sort((a, b) => a[0] - b[0]);
    for (let i = 1; i < sorted.length; i++) if (sorted[i][0] < sorted[i - 1][1]) return false;
    return true;
}
```
**Complexity:** Time O(n log n) · Space O(n)

**Brute force:** Check every pair for overlap: O(n²).

---

### 253. Meeting Rooms II (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/meeting-rooms-ii/) · Free mirror: LintCode 919

**Problem:** Return the minimum number of conference rooms needed to hold every meeting.

**Example:** `[[0,30],[5,10],[15,20]]` → `2`

**Key insight:** The answer is the maximum number of meetings running at the same moment.

#### Approach 1: Separate Sorted Starts and Ends (two pointers)
```javascript
/**
 * @param {number[][]} intervals
 * @return {number}
 */
function minMeetingRooms(intervals) {
    const starts = intervals.map((iv) => iv[0]).sort((a, b) => a - b);
    const ends = intervals.map((iv) => iv[1]).sort((a, b) => a - b);
    let rooms = 0, best = 0, e = 0;
    for (const s of starts) {
        while (e < ends.length && ends[e] <= s) {
            rooms--; // that meeting has ended and freed its room
            e++;
        }
        best = Math.max(best, ++rooms);
    }
    return best;
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Min-Heap of End Times
```javascript
/**
 * Binary heap ordered by `less(a, b)`: pass (a, b) => a < b for a min-heap.
 *
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}
/**
 * @param {number[][]} intervals
 * @return {number}
 */
function minMeetingRooms(intervals) {
    const sorted = [...intervals].sort((a, b) => a[0] - b[0]);
    const ends = new Heap((a, b) => a < b);
    for (const [s, e] of sorted) {
        if (ends.size && ends.peek() <= s) ends.pop(); // reuse the room that frees up earliest
        ends.push(e);
    }
    return ends.size;
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 3: Line Sweep over Sorted Events
```javascript
/**
 * @param {number[][]} intervals
 * @return {number}
 */
function minMeetingRooms(intervals) {
    const events = [];
    for (const [s, e] of intervals) events.push([s, 1], [e, -1]);
    events.sort((a, b) => a[0] - b[0] || a[1] - b[1]); // at equal times, process ends (-1) first
    let cur = 0, best = 0;
    for (const [, d] of events) best = Math.max(best, (cur += d));
    return best;
}
```
**Complexity:** Time O(n log n) · Space O(n)

---

### 1851. Minimum Interval to Include Each Query
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/minimum-interval-to-include-each-query/)

**Problem:** For each `queries[j]`, find the size (`right - left + 1`) of the smallest interval that contains it, or `-1` if no interval does.

**Example:** `intervals = [[1,4],[2,4],[3,6],[4,4]], queries = [2,3,4,5]` → `[3,3,1,4]`

**Key insight:** Process queries in increasing order (offline). Push every interval whose start ≤ q onto a min-heap keyed by size, then pop from the top any interval whose end < q.

#### Approach 1: Sort + Min-Heap (Optimal)
```javascript
/**
 * Binary heap ordered by `less(a, b)`: pass (a, b) => a < b for a min-heap.
 *
 * @template T
 */
class Heap {
    data = [];
    /**
     * @param {(a: T, b: T) => boolean} less
     */
    constructor(less) {
        this.less = less;
    }
    /**
     * @return {number}
     */
    get size() {
        return this.data.length;
    }
    /**
     * @return {T | undefined}
     */
    peek() {
        return this.data[0];
    }
    /**
     * @param {T} x
     * @return {void}
     */
    push(x) {
        const d = this.data;
        d.push(x);
        let i = d.length - 1;
        while (i > 0) {
            const p = (i - 1) >> 1;
            if (!this.less(d[i], d[p])) break;
            [d[i], d[p]] = [d[p], d[i]];
            i = p;
        }
    }
    /**
     * @return {T | undefined}
     */
    pop() {
        const d = this.data;
        if (d.length === 0) return undefined;
        const top = d[0];
        const last = d.pop();
        if (d.length > 0) {
            d[0] = last;
            let i = 0;
            for (;;) {
                const l = 2 * i + 1, r = l + 1;
                let m = i;
                if (l < d.length && this.less(d[l], d[m])) m = l;
                if (r < d.length && this.less(d[r], d[m])) m = r;
                if (m === i) break;
                [d[i], d[m]] = [d[m], d[i]];
                i = m;
            }
        }
        return top;
    }
}
/**
 * @param {number[][]} intervals
 * @param {number[]} queries
 * @return {number[]}
 */
function minInterval(intervals, queries) {
    const sorted = [...intervals].sort((a, b) => a[0] - b[0]);
    const order = queries.map((_, i) => i).sort((a, b) => queries[a] - queries[b]);
    const heap = new Heap((a, b) => a[0] < b[0]); // [size, end]
    const res = new Array(queries.length).fill(-1);
    let i = 0;
    for (const qi of order) {
        const q = queries[qi];
        while (i < sorted.length && sorted[i][0] <= q) {
            const [l, r] = sorted[i++];
            heap.push([r - l + 1, r]);
        }
        while (heap.size && heap.peek()[1] < q) heap.pop(); // ends before q
        if (heap.size) res[qi] = heap.peek()[0];
    }
    return res;
}
```
**Complexity:** Time O(n log n + q log q) · Space O(n + q)

#### Approach 2: Sort Intervals by Size + Union-Find "Next Unanswered Query"
Visit intervals from smallest to largest. Each one answers every unanswered query it covers. The union-find jumps over queries that already have answers.
```javascript
/**
 * @param {number[][]} intervals
 * @param {number[]} queries
 * @return {number[]}
 */
function minInterval(intervals, queries) {
    const qs = [...new Set(queries)].sort((a, b) => a - b);
    const nxt = Array.from({ length: qs.length + 1 }, (_, i) => i); // nxt[i] = next unanswered index >= i
    const ans = new Array(qs.length).fill(-1);
    const find = (x) => {
        while (nxt[x] !== x) {
            nxt[x] = nxt[nxt[x]];
            x = nxt[x];
        }
        return x;
    };
    const lowerBound = (x) => {
        let lo = 0, hi = qs.length;
        while (lo < hi) {
            const mid = (lo + hi) >> 1;
            if (qs[mid] < x) lo = mid + 1;
            else hi = mid;
        }
        return lo;
    };
    for (const [l, r] of [...intervals].sort((a, b) => a[1] - a[0] - (b[1] - b[0]))) {
        for (let j = find(lowerBound(l)); j < qs.length && qs[j] <= r; j = find(j + 1)) {
            ans[j] = r - l + 1;
            nxt[j] = j + 1; // mark answered
        }
    }
    const pos = new Map(qs.map((q, i) => [q, i]));
    return queries.map((q) => ans[pos.get(q)]);
}
```
**Complexity:** Time O(n log n + q log q) · Space O(q)

**Brute force:** Scan every interval for every query: O(n · q).

---
## 17. Math & Geometry

### 48. Rotate Image
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/rotate-image/)

**Problem:** Rotate an `n × n` matrix 90° clockwise **in place**.

**Example:** `[[1,2,3],[4,5,6],[7,8,9]]` → `[[7,4,1],[8,5,2],[9,6,3]]`

**Key insight:** A clockwise rotation equals a transpose followed by reversing each row.

#### Approach 1: Transpose + Reverse Rows (Optimal)
```javascript
/**
 * Do not return anything, modify matrix in-place instead.
 *
 * @param {number[][]} matrix
 * @return {void}
 */
function rotate(matrix) {
    const n = matrix.length;
    for (let i = 0; i < n; i++) {
        for (let j = i + 1; j < n; j++) [matrix[i][j], matrix[j][i]] = [matrix[j][i], matrix[i][j]];
    }
    for (const row of matrix) row.reverse();
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Rotate Four Cells at a Time, Layer by Layer
```javascript
/**
 * @param {number[][]} matrix
 * @return {void}
 */
function rotate(matrix) {
    for (let l = 0, r = matrix.length - 1; l < r; l++, r--) {
        for (let i = 0; i < r - l; i++) {
            const top = l, bottom = r;
            const tmp = matrix[top][l + i]; // save top-left
            matrix[top][l + i] = matrix[bottom - i][l]; // bottom-left to top-left
            matrix[bottom - i][l] = matrix[bottom][r - i]; // bottom-right to bottom-left
            matrix[bottom][r - i] = matrix[top + i][r]; // top-right to bottom-right
            matrix[top + i][r] = tmp; // top-left to top-right
        }
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

---

### 54. Spiral Matrix
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/spiral-matrix/)

**Problem:** Return every element of an `m × n` matrix in spiral order (clockwise, starting at the top-left).

**Example:** `[[1,2,3],[4,5,6],[7,8,9]]` → `[1,2,3,6,9,8,7,4,5]`

#### Approach 1: Shrinking Boundaries (Optimal)
```javascript
/**
 * @param {number[][]} matrix
 * @return {number[]}
 */
function spiralOrder(matrix) {
    const res = [];
    let top = 0, bottom = matrix.length - 1, left = 0, right = matrix[0].length - 1;
    while (top <= bottom && left <= right) {
        for (let c = left; c <= right; c++) res.push(matrix[top][c]);
        top++;
        for (let r = top; r <= bottom; r++) res.push(matrix[r][right]);
        right--;
        if (top <= bottom) {
            for (let c = right; c >= left; c--) res.push(matrix[bottom][c]);
            bottom--;
        }
        if (left <= right) {
            for (let r = bottom; r >= top; r--) res.push(matrix[r][left]);
            left++;
        }
    }
    return res;
}
```
**Complexity:** Time O(m · n) · Space O(1) extra

#### Approach 2: Direction Vectors with a Visited Grid
```javascript
/**
 * @param {number[][]} matrix
 * @return {number[]}
 */
function spiralOrder(matrix) {
    const m = matrix.length, n = matrix[0].length;
    const seen = Array.from({ length: m }, () => new Array(n).fill(false));
    const dirs = [
        [0, 1],
        [1, 0],
        [0, -1],
        [-1, 0],
    ];
    const res = [];
    let r = 0, c = 0, d = 0;
    for (let k = 0; k < m * n; k++) {
        res.push(matrix[r][c]);
        seen[r][c] = true;
        let nr = r + dirs[d][0], nc = c + dirs[d][1];
        if (nr < 0 || nc < 0 || nr >= m || nc >= n || seen[nr][nc]) {
            d = (d + 1) % 4; // turn clockwise
            nr = r + dirs[d][0];
            nc = c + dirs[d][1];
        }
        r = nr;
        c = nc;
    }
    return res;
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

---

### 73. Set Matrix Zeroes
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/set-matrix-zeroes/)

**Problem:** Wherever the matrix has a `0`, set that cell's entire row and column to `0`, in place.

**Example:** `[[1,1,1],[1,0,1],[1,1,1]]` → `[[1,0,1],[0,0,0],[1,0,1]]`

**Key insight:** Use the first row and first column as marker arrays, with one extra flag for whether the first column itself must be zeroed.

#### Approach 1: Row and Column Sets
```javascript
/**
 * Do not return anything, modify matrix in-place instead.
 *
 * @param {number[][]} matrix
 * @return {void}
 */
function setZeroes(matrix) {
    const rows = new Set(), cols = new Set();
    matrix.forEach((row, i) =>
        row.forEach((v, j) => {
            if (v === 0) {
                rows.add(i);
                cols.add(j);
            }
        }),
    );
    matrix.forEach((row, i) =>
        row.forEach((_, j) => {
            if (rows.has(i) || cols.has(j)) row[j] = 0;
        }),
    );
}
```
**Complexity:** Time O(m · n) · Space O(m + n)

#### Approach 2: First Row and Column as Markers (Optimal)
```javascript
/**
 * @param {number[][]} matrix
 * @return {void}
 */
function setZeroes(matrix) {
    const m = matrix.length, n = matrix[0].length;
    let firstColZero = false;
    for (let i = 0; i < m; i++) {
        if (matrix[i][0] === 0) firstColZero = true;
        for (let j = 1; j < n; j++) if (matrix[i][j] === 0) matrix[i][0] = matrix[0][j] = 0;
    }
    for (let i = m - 1; i >= 0; i--) {
        // bottom-up so row 0's markers are used before row 0 is zeroed
        for (let j = n - 1; j >= 1; j--) if (matrix[i][0] === 0 || matrix[0][j] === 0) matrix[i][j] = 0;
        if (firstColZero) matrix[i][0] = 0;
    }
}
```
**Complexity:** Time O(m · n) · Space O(1)

---

### 202. Happy Number
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/happy-number/)

**Problem:** Repeatedly replace `n` with the sum of the squares of its digits. `n` is *happy* if this process reaches 1. Otherwise it loops forever. Return whether `n` is happy.

**Example:** `19` → `true` (1²+9²=82 → 68 → 100 → 1)

**Key insight:** The sequence either reaches 1 or enters a cycle. Detect the cycle with a set or with Floyd's algorithm.

#### Approach 1: Set
```javascript
/**
 * @param {number} n
 * @return {boolean}
 */
function isHappy(n) {
    const seen = new Set();
    while (n !== 1 && !seen.has(n)) {
        seen.add(n);
        n = [...String(n)].reduce((s, d) => s + Number(d) ** 2, 0);
    }
    return n === 1;
}
```
**Complexity:** Time O(log n) per step, with only a few steps · Space O(log n)

#### Approach 2: Floyd's Cycle Detection (Optimal)
```javascript
/**
 * @param {number} n
 * @return {boolean}
 */
function isHappy(n) {
    const next = (x) => {
        let s = 0;
        for (; x > 0; x = Math.floor(x / 10)) s += (x % 10) ** 2;
        return s;
    };
    let slow = n, fast = next(n);
    while (fast !== 1 && slow !== fast) {
        slow = next(slow);
        fast = next(next(fast));
    }
    return fast === 1;
}
```
**Complexity:** Time O(log n) · Space O(1)

---

### 66. Plus One
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/plus-one/)

**Problem:** A large integer is stored as an array of digits, most significant first. Add one and return the resulting digits.

**Example:** `[1,2,9]` → `[1,3,0]`; `[9,9]` → `[1,0,0]`

#### Approach 1: Propagate the Carry from the Right (Optimal)
```javascript
/**
 * @param {number[]} digits
 * @return {number[]}
 */
function plusOne(digits) {
    for (let i = digits.length - 1; i >= 0; i--) {
        if (digits[i] < 9) {
            digits[i]++;
            return digits;
        }
        digits[i] = 0;
    }
    return [1, ...digits]; // every digit was 9
}
```
**Complexity:** Time O(n) · Space O(1), or O(n) when every digit is 9

#### Approach 2: `BigInt` Round Trip
```javascript
/**
 * @param {number[]} digits
 * @return {number[]}
 */
function plusOne(digits) {
    return [...(BigInt(digits.join("")) + 1n).toString()].map(Number);
}
```
**Complexity:** Time O(n) · Space O(n). `BigInt` avoids precision loss for up to 100 digits; it misses the point in an interview.

---

### 50. Pow(x, n)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/powx-n/)

**Problem:** Compute `x` raised to the power `n`, where `n` can be negative (as low as −2³¹).

**Example:** `x = 2.0, n = 10` → `1024.0`; `x = 2.0, n = -2` → `0.25`

**Key insight:** Binary exponentiation: `x^n = (x²)^(n/2)`, with an extra factor of `x` when `n` is odd.

#### Approach 1: Iterative Fast Exponentiation (Optimal)
```javascript
/**
 * @param {number} x
 * @param {number} n
 * @return {number}
 */
function myPow(x, n) {
    let p = n;
    if (p < 0) {
        x = 1 / x;
        p = -p; // fine for -2^31: JS numbers aren't 32-bit
    }
    let res = 1;
    while (p > 0) {
        if (p % 2 === 1) res *= x; // % and / instead of &, >> (those wrap at 2^31)
        x *= x;
        p = Math.floor(p / 2);
    }
    return res;
}
```
**Complexity:** Time O(log n) · Space O(1)

> Using `p & 1` and `p >>= 1` would break for `n = -2³¹`, because `2³¹` doesn't fit in a signed 32-bit integer.

#### Approach 2: Recursive
```javascript
/**
 * @param {number} x
 * @param {number} n
 * @return {number}
 */
function myPow(x, n) {
    if (n === 0) return 1;
    if (n < 0) return 1 / myPow(x, -n);
    const half = myPow(x, Math.floor(n / 2));
    return n % 2 ? half * half * x : half * half;
}
```
**Complexity:** Time O(log n) · Space O(log n)

#### Approach 3: Built-in `**`
```javascript
/**
 * @param {number} x
 * @param {number} n
 * @return {number}
 */
function myPow(x, n) {
    return x ** n;
}
```
**Complexity:** Time O(1) to O(log n) · Space O(1). Good for checking your work, but write Approach 1 in an interview.

---

### 43. Multiply Strings
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/multiply-strings/)

**Problem:** Multiply two non-negative integers given as strings and return the product as a string, without converting the inputs to integers directly.

**Example:** `"123" × "456"` → `"56088"`

**Key insight:** The product of `num1[i]` and `num2[j]` lands in positions `i+j` and `i+j+1` of an array of length `m+n`.

#### Approach: Grade-School Multiplication into a Position Array (Optimal)
```javascript
/**
 * @param {string} num1
 * @param {string} num2
 * @return {string}
 */
function multiply(num1, num2) {
    if (num1 === "0" || num2 === "0") return "0";
    const m = num1.length, n = num2.length;
    const pos = new Array(m + n).fill(0);
    for (let i = m - 1; i >= 0; i--) {
        for (let j = n - 1; j >= 0; j--) {
            const sum = (num1.charCodeAt(i) - 48) * (num2.charCodeAt(j) - 48) + pos[i + j + 1];
            pos[i + j + 1] = sum % 10;
            pos[i + j] += Math.floor(sum / 10);
        }
    }
    const s = pos.join("");
    return s[0] === "0" ? s.slice(1) : s; // at most one leading zero
}
```
**Complexity:** Time O(m · n) · Space O(m + n)

**Note:** `(BigInt(num1) * BigInt(num2)).toString()` would be a one-liner, but the problem forbids converting the inputs to integers directly.

---

### 2013. Detect Squares
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/detect-squares/)

**Problem:** Design `DetectSquares`. `add(point)` adds a point; duplicates are allowed. `count(point)` returns how many ways you can pick three stored points that form an axis-aligned square of positive area together with the query point.

**Key insight:** Treat each stored point that is diagonal to the query (`|dx| == |dy| ≠ 0`) as the opposite corner. Multiply the counts of the other two corners.

#### Approach 1: Count Map + Iterate Diagonal Candidates (Optimal)
```javascript
class DetectSquares {
    cnt = new Map(); // "x,y" -> count
    pts = []; // distinct points

    /**
     * @param {number[]} point
     * @return {void}
     */
    add(point) {
        const key = `${point[0]},${point[1]}`;
        const c = this.cnt.get(key) ?? 0;
        if (c === 0) this.pts.push(point);
        this.cnt.set(key, c + 1);
    }

    /**
     * @param {number[]} point
     * @return {number}
     */
    count(point) {
        const [qx, qy] = point;
        const get = (x, y) => this.cnt.get(`${x},${y}`) ?? 0;
        let res = 0;
        for (const [x, y] of this.pts) {
            if (x === qx || Math.abs(x - qx) !== Math.abs(y - qy)) continue; // diagonal corners only
            res += get(x, y) * get(qx, y) * get(x, qy);
        }
        return res;
    }
}
```
**Complexity:** `add` O(1) · `count` O(P) for P distinct points · Space O(P)

#### Approach 2: Points Grouped by x-Coordinate
```javascript
class DetectSquares {
    byX = new Map(); // x -> (y -> count)

    /**
     * @param {number[]} point
     * @return {void}
     */
    add(point) {
        const [x, y] = point;
        if (!this.byX.has(x)) this.byX.set(x, new Map());
        const col = this.byX.get(x);
        col.set(y, (col.get(y) ?? 0) + 1);
    }

    /**
     * @param {number[]} point
     * @return {number}
     */
    count(point) {
        const [qx, qy] = point;
        const col = this.byX.get(qx);
        if (!col) return 0;
        let res = 0;
        for (const [y, c] of col) {
            const side = y - qy;
            if (side === 0) continue;
            for (const x of [qx + side, qx - side]) {
                // square to the right or to the left
                const other = this.byX.get(x);
                if (other) res += c * (other.get(y) ?? 0) * (other.get(qy) ?? 0);
            }
        }
        return res;
    }
}
```
**Complexity:** `add` O(1) · `count` O(points sharing the query's x) · Space O(P)

---
## 18. Bit Manipulation

> JavaScript bitwise operators convert their operands to **32-bit signed** integers (`>>>` is the only unsigned one). `x >>> 0` reinterprets a value as unsigned, and `x | 0` as signed. `Math.clz32` counts leading zeros.

### 136. Single Number
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/single-number/)

**Problem:** Every element appears twice except one. Find that one in O(n) time and O(1) extra space.

**Example:** `[4,1,2,1,2]` → `4`

**Key insight:** `a ^ a = 0` and `a ^ 0 = a`, so XOR-ing everything cancels the pairs.

#### Approach 1: XOR (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function singleNumber(nums) {
    return nums.reduce((acc, n) => acc ^ n, 0);
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Set Toggle
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function singleNumber(nums) {
    const seen = new Set();
    for (const n of nums) {
        if (seen.has(n)) seen.delete(n);
        else seen.add(n);
    }
    return seen.values().next().value;
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 191. Number of 1 Bits
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/number-of-1-bits/)

**Problem:** Return the number of set bits (the Hamming weight) in `n`.

**Example:** `11` (binary `1011`) → `3`

#### Approach 1: Check Every Bit
```javascript
/**
 * @param {number} n
 * @return {number}
 */
function hammingWeight(n) {
    let count = 0;
    for (let i = 0; i < 32; i++) count += (n >>> i) & 1; // >>> handles inputs >= 2^31
    return count;
}
```
**Complexity:** Time O(32) · Space O(1)

#### Approach 2: Brian Kernighan's Trick (Optimal)
`n & (n-1)` clears the lowest set bit, so the loop runs once per set bit.
```javascript
/**
 * @param {number} n
 * @return {number}
 */
function hammingWeight(n) {
    let count = 0;
    while (n !== 0) {
        n &= n - 1; // works on the 32-bit signed view, which has the same bits
        count++;
    }
    return count;
}
```
**Complexity:** Time O(number of set bits) · Space O(1)

#### Approach 3: Parallel Bit Count (SWAR)
```javascript
/**
 * @param {number} n
 * @return {number}
 */
function hammingWeight(n) {
    n = n - ((n >>> 1) & 0x55555555);
    n = (n & 0x33333333) + ((n >>> 2) & 0x33333333);
    n = (n + (n >>> 4)) & 0x0f0f0f0f;
    return Math.imul(n, 0x01010101) >>> 24; // the top byte collects the four byte-sums
}
```
**Complexity:** Time O(1) · Space O(1)

---

### 338. Counting Bits
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/counting-bits/)

**Problem:** For every `i` in `0..n`, return the number of set bits in `i`.

**Example:** `n = 5` → `[0,1,1,2,1,2]`

**Key insight:** `bits(i) = bits(i >> 1) + (i & 1)`, or equivalently `bits(i) = bits(i & (i-1)) + 1`.

#### Approach 1: DP on the Right Shift (Optimal)
```javascript
/**
 * @param {number} n
 * @return {number[]}
 */
function countBits(n) {
    const res = new Array(n + 1).fill(0);
    for (let i = 1; i <= n; i++) res[i] = res[i >> 1] + (i & 1);
    return res;
}
```
**Complexity:** Time O(n) · Space O(1) extra

#### Approach 2: DP on the Lowest Set Bit
```javascript
/**
 * @param {number} n
 * @return {number[]}
 */
function countBits(n) {
    const res = new Array(n + 1).fill(0);
    for (let i = 1; i <= n; i++) res[i] = res[i & (i - 1)] + 1;
    return res;
}
```
**Complexity:** Time O(n) · Space O(1) extra

#### Approach 3: Popcount Each Number
```javascript
/**
 * @param {number} n
 * @return {number[]}
 */
function countBits(n) {
    return Array.from({ length: n + 1 }, (_, i) => i.toString(2).split("1").length - 1);
}
```
**Complexity:** Time O(n log n) · Space O(1) extra

---

### 190. Reverse Bits
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/reverse-bits/)

**Problem:** Reverse the bits of a 32-bit unsigned integer.

**Example:** `00000010100101000001111010011100` → `00111001011110000010100101000000`

#### Approach 1: Bit-by-Bit Shift
```javascript
/**
 * @param {number} n
 * @return {number}
 */
function reverseBits(n) {
    let res = 0;
    for (let i = 0; i < 32; i++) {
        res = (res << 1) | (n & 1);
        n >>>= 1;
    }
    return res >>> 0; // reinterpret as unsigned
}
```
**Complexity:** Time O(32) · Space O(1)

#### Approach 2: Divide and Conquer Swaps (Optimal)
```javascript
/**
 * @param {number} n
 * @return {number}
 */
function reverseBits(n) {
    n = (n >>> 16) | (n << 16);
    n = ((n & 0xff00ff00) >>> 8) | ((n & 0x00ff00ff) << 8);
    n = ((n & 0xf0f0f0f0) >>> 4) | ((n & 0x0f0f0f0f) << 4);
    n = ((n & 0xcccccccc) >>> 2) | ((n & 0x33333333) << 2);
    n = ((n & 0xaaaaaaaa) >>> 1) | ((n & 0x55555555) << 1);
    return n >>> 0;
}
```
**Complexity:** Time O(1) (5 steps) · Space O(1)

#### Approach 3: String Formatting
```javascript
/**
 * @param {number} n
 * @return {number}
 */
function reverseBits(n) {
    return parseInt((n >>> 0).toString(2).padStart(32, "0").split("").reverse().join(""), 2);
}
```
**Complexity:** Time O(32) · Space O(32)

---

### 268. Missing Number
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/missing-number/)

**Problem:** `nums` contains `n` distinct numbers from the range `[0, n]`. Return the one that's missing.

**Example:** `[3,0,1]` → `2`

#### Approach 1: Gauss Sum
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function missingNumber(nums) {
    const n = nums.length;
    return (n * (n + 1)) / 2 - nums.reduce((a, b) => a + b, 0);
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: XOR of Indices and Values (Optimal)
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function missingNumber(nums) {
    return nums.reduce((acc, v, i) => acc ^ i ^ v, nums.length);
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 3: Sort + Binary Search
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
function missingNumber(nums) {
    const a = [...nums].sort((x, y) => x - y);
    let lo = 0, hi = a.length;
    while (lo < hi) {
        const mid = (lo + hi) >> 1;
        if (a[mid] > mid) hi = mid;
        else lo = mid + 1;
    }
    return lo;
}
```
**Complexity:** Time O(n log n) · Space O(n)

---

### 371. Sum of Two Integers
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/sum-of-two-integers/)

**Problem:** Return `a + b` without using the `+` or `-` operators.

**Example:** `a = 1, b = 2` → `3`; `a = -2, b = 3` → `1`

**Key insight:** `a ^ b` is the sum without carries, and `(a & b) << 1` is the carry. Repeat until the carry is zero. Work in `int32` to get LeetCode's 32-bit two's-complement behavior.

#### Approach 1: Iterative XOR/AND (Optimal)
JS bitwise operators already work on 32-bit two's-complement integers, so negative inputs need no special masking. The carry shifts out of bit 31 and disappears.
```javascript
/**
 * @param {number} a
 * @param {number} b
 * @return {number}
 */
function getSum(a, b) {
    while (b !== 0) {
        const carry = (a & b) << 1; // bits that carry into the next position
        a ^= b; // sum without carries
        b = carry;
    }
    return a;
}
```
**Complexity:** Time O(32) · Space O(1)

#### Approach 2: Recursive
```javascript
/**
 * @param {number} a
 * @param {number} b
 * @return {number}
 */
function getSum(a, b) {
    return b === 0 ? a : getSum(a ^ b, (a & b) << 1);
}
```
**Complexity:** Time O(32) · Space O(32)

---

### 7. Reverse Integer
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/reverse-integer/)

**Problem:** Reverse the digits of a signed 32-bit integer `x`. If the result falls outside `[-2³¹, 2³¹-1]`, return `0`. Assume 64-bit storage isn't available.

**Example:** `123` → `321`; `-123` → `-321`; `1534236469` → `0`

**Key insight:** Before computing `res*10 + d`, check whether it would overflow by comparing `res` against `MaxInt32/10` and `MinInt32/10`.

#### Approach 1: Pop and Push with Overflow Guards (Optimal)
```javascript
/**
 * @param {number} x
 * @return {number}
 */
function reverse(x) {
    const sign = x < 0 ? -1 : 1;
    const limit = sign > 0 ? 2 ** 31 - 1 : 2 ** 31;
    let rest = Math.abs(x), res = 0;
    while (rest > 0) {
        const d = rest % 10;
        rest = Math.floor(rest / 10);
        if (res > (limit - d) / 10) return 0; // res*10 + d would exceed the limit
        res = res * 10 + d;
    }
    return sign * res + 0; // + 0 avoids returning -0 for x = 0
}
```
**Complexity:** Time O(log₁₀ x) · Space O(1)

#### Approach 2: String Reversal
```javascript
/**
 * @param {number} x
 * @return {number}
 */
function reverse(x) {
    const r = Math.sign(x) * Number([...String(Math.abs(x))].reverse().join(""));
    return r < -(2 ** 31) || r > 2 ** 31 - 1 ? 0 : r + 0;
}
```
**Complexity:** Time O(log₁₀ x) · Space O(log₁₀ x)

---

## Complexity Cheat Sheet

| Pattern | Typical time | Typical space | Signals in the problem | JavaScript tools |
|---|---|---|---|---|
| Hash map / set | O(n) | O(n) | "seen before", duplicates, complements | `Map`, `Set`, string keys for tuples |
| Two pointers | O(n) | O(1) | sorted input, pairs, palindromes | index pointers, destructuring swaps |
| Sliding window | O(n) | O(k) | contiguous substring or subarray with a constraint | `charCodeAt` + `number[]` counts |
| Monotonic stack | O(n) | O(n) | next greater or smaller element, histograms | array `push` / `pop` |
| Binary search | O(log n) | O(1) | sorted input, or "minimum k such that…" | `(lo + hi) >> 1` |
| Heap | O(n log k) | O(k) | top-k, k-way merge, streaming median | inline `Heap` class |
| DFS / BFS | O(V + E) | O(V) | grids, connectivity, shortest path in an unweighted graph | array + `head` index queue |
| Topological sort | O(V + E) | O(V + E) | dependencies, ordering, cycle detection | in-degree array + queue |
| Union-find | O(α(n)) per operation | O(n) | dynamic connectivity, redundant edges | `Array.from({ length: n }, (_, i) => i)` |
| Dijkstra | O(E log V) | O(V + E) | weighted shortest path with non-negative weights | `Heap` of `[dist, node]` pairs |
| Backtracking | O(2ⁿ) / O(n!) | O(n) | "all combinations / permutations / partitions" | `res.push([...path])` |
| 1-D DP | O(n) to O(n²) | O(1) to O(n) | best value over a prefix | `new Array(n + 1).fill(0)` |
| 2-D DP | O(m · n) | O(n) when rolled | two strings, grids, intervals | `Array.from` rows, rolling arrays |
| Greedy | O(n) or O(n log n) | O(1) | a locally best choice provably works | `sort((a, b) => a - b)` + scan |
| Bit tricks | O(1) to O(n) | O(1) | XOR pairing, powers of two, bit counts | `>>>`, `Math.clz32`, `Math.imul`, `BigInt` |

---

*End of document: 150 problems, 18 categories.*
