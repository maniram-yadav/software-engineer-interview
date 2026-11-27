# LeetCode "Blind 150" (NeetCode 150) — Complete Rust Solutions

A full Rust reference for the 150 problems in the **NeetCode 150** list (the Blind 75 plus 75 more), grouped by pattern. Every problem has:

- its **LeetCode number and title**, with difficulty
- a **short problem statement** in my own words, plus an example
- one or more **complete Rust solutions**: brute force → better → optimal where that comparison helps
- **time and space complexity** for each approach
- a **key insight** line explaining the idea

> **Every snippet includes all the `use` statements it needs** (`HashMap`, `HashSet`, `BinaryHeap`, `Reverse`, `VecDeque`, `BTreeMap`, `Rc`, `RefCell`, …), so it compiles exactly as shown. Each one is LeetCode's `impl Solution { pub fn ... }` with the exact snake_case signature LeetCode's Rust template uses. LeetCode supplies `struct Solution;` itself, so paste just the snippet into the editor.
>
> Every snippet in this file was compiled with **rustc 1.97 (edition 2021)**, and all 150 problems were run on randomized tests that check every alternative approach against the others and, where possible, against an independent reference answer. The code uses only the standard library (no crates) and avoids recently stabilized APIs, so it also builds on the older toolchain LeetCode runs.

---

## Shared Definitions

LeetCode provides these structs for linked-list and tree problems. They are assumed (not redeclared) in those sections; tree snippets still include the `Rc`/`RefCell` imports they use.

```rust
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}
```

> **No Rust template on LeetCode:** problems **133** (Clone Graph), **138** (Copy List with Random Pointer), and **141** (Linked List Cycle) don't offer Rust, because a graph with cycles or shared pointers can't be built from `Box`. Their sections define an `Rc<RefCell<…>>` node and solve the same problem idiomatically, so you can still study and run them locally.

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

## Rust Interview Toolkit (quick reference)

```rust
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// Hash map counting
let mut freq: HashMap<i32, i32> = HashMap::new();
*freq.entry(x).or_insert(0) += 1;

// Stack and queue
let mut stack: Vec<i32> = Vec::new();          // push / pop / last()
let mut q: VecDeque<i32> = VecDeque::new();     // push_back / pop_front

// Heaps: BinaryHeap is a MAX-heap; wrap in Reverse for a min-heap
let mut max_heap = BinaryHeap::new();           // max_heap.push(5); max_heap.pop(); max_heap.peek();
let mut min_heap = BinaryHeap::new();           // min_heap.push(Reverse(5));
// Tuples compare lexicographically: heap.push(Reverse((dist, node)));

// Sorting
v.sort();  v.sort_unstable();  v.sort_by_key(|x| x.1);  v.sort_by(|a, b| b.cmp(a));

// Binary search on a sorted slice
let idx = v.partition_point(|&x| x < target);   // lower bound
match v.binary_search(&target) { Ok(i) => i, Err(insert_at) => insert_at };

// Strings: index bytes, not chars, for ASCII input
let s = s.as_bytes();  // s[i] is u8; (s[i] - b'a') as usize
```

**Rust gotchas for LeetCode:**
- Indices are `usize`. Cast with `as usize` / `as i32`, and watch for underflow when you compute `i - 1` on a `usize`.
- Trees use `Option<Rc<RefCell<TreeNode>>>`. Read with `node.borrow().val`, clone the `Rc` with `Rc::clone(&n)` or `n.clone()`, and mutate with `node.borrow_mut().left = ...`.
- Linked lists use `Option<Box<ListNode>>`. Pattern `while let Some(mut node) = cur { cur = node.next.take(); ... }` moves nodes without cloning.
- `i32` overflow panics in debug builds. Use `i64` for products and sums that can overflow, and `checked_mul` / `checked_add` for overflow-detecting problems.

---
## 1. Arrays & Hashing

### 217. Contains Duplicate
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/contains-duplicate/)

**Problem:** Given an integer array `nums`, return `true` if any value appears at least twice, and `false` if every element is distinct.

**Example:** `nums = [1,2,3,1]` → `true`; `nums = [1,2,3,4]` → `false`

**Key insight:** A hash set gives O(1) membership checks, so you can spot a repeat in one pass.

#### Approach 1: Brute Force
```rust
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] {
                    return true;
                }
            }
        }
        false
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Sorting
```rust
impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        nums.windows(2).any(|w| w[0] == w[1])
    }
}
```
**Complexity:** Time O(n log n) · Space O(1) (in-place unstable sort)

#### Approach 3: HashSet (Optimal)
```rust
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::with_capacity(nums.len());
        // insert() returns false if the value was already present
        !nums.into_iter().all(|n| seen.insert(n))
    }
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 242. Valid Anagram
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/valid-anagram/)

**Problem:** Given strings `s` and `t`, return `true` if `t` is an anagram of `s`, meaning both use exactly the same letters with the same counts.

**Example:** `s = "anagram", t = "nagaram"` → `true`; `s = "rat", t = "car"` → `false`

**Key insight:** Two strings are anagrams when their character frequency counts match.

#### Approach 1: Sorting
```rust
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut a: Vec<char> = s.chars().collect();
        let mut b: Vec<char> = t.chars().collect();
        a.sort_unstable();
        b.sort_unstable();
        a == b
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Frequency Array (Optimal)
```rust
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut count = [0i32; 26];
        for (a, b) in s.bytes().zip(t.bytes()) {
            count[(a - b'a') as usize] += 1;
            count[(b - b'a') as usize] -= 1;
        }
        count.iter().all(|&c| c == 0)
    }
}
```
**Complexity:** Time O(n) · Space O(1) (26 counters)

#### Approach 3: HashMap (handles Unicode follow-up)
```rust
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut count: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *count.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            let e = count.entry(c).or_insert(0);
            *e -= 1;
            if *e < 0 {
                return false;
            }
        }
        count.values().all(|&v| v == 0)
    }
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
```rust
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: One-Pass HashMap (Optimal)
```rust
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut idx: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for (i, &n) in nums.iter().enumerate() {
            if let Some(&j) = idx.get(&(target - n)) {
                return vec![j, i as i32];
            }
            idx.insert(n, i as i32);
        }
        vec![]
    }
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
```rust
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        for s in strs {
            let mut key = s.as_bytes().to_vec();
            key.sort_unstable();
            groups.entry(key).or_default().push(s);
        }
        groups.into_values().collect()
    }
}
```
**Complexity:** Time O(m · k log k), for m strings of max length k · Space O(m · k)

#### Approach 2: Count Array as Key (Optimal)
Fixed-size arrays implement `Hash + Eq`, so `[u8; 26]` works directly as a key (each count is ≤ 100 here).
```rust
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for s in strs {
            let mut key = [0u8; 26];
            for b in s.bytes() {
                key[(b - b'a') as usize] += 1;
            }
            groups.entry(key).or_default().push(s);
        }
        groups.into_values().collect()
    }
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
```rust
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            *freq.entry(n).or_insert(0) += 1;
        }
        let mut pairs: Vec<(i32, i32)> = freq.into_iter().collect();
        pairs.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        pairs.into_iter().take(k as usize).map(|(v, _)| v).collect()
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Min-Heap of Size k
```rust
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            *freq.entry(n).or_insert(0) += 1;
        }
        let mut heap = BinaryHeap::new(); // min-heap on (count, value)
        for (v, c) in freq {
            heap.push(Reverse((c, v)));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        heap.into_iter().map(|Reverse((_, v))| v).collect()
    }
}
```
**Complexity:** Time O(n log k) · Space O(n)

#### Approach 3: Bucket Sort (Optimal)
```rust
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq: HashMap<i32, usize> = HashMap::new();
        for &n in &nums {
            *freq.entry(n).or_insert(0) += 1;
        }
        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1]; // index = frequency
        for (v, c) in freq {
            buckets[c].push(v);
        }
        buckets.into_iter().rev().flatten().take(k as usize).collect()
    }
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
```rust
struct Codec;

impl Codec {
    fn new() -> Self {
        Codec
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let mut out = String::new();
        for s in strs {
            out.push_str(&s.len().to_string()); // byte length, so UTF-8 is safe too
            out.push('#');
            out.push_str(&s);
        }
        out
    }

    fn decode(&self, s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let mut res = Vec::new();
        let mut i = 0;
        while i < bytes.len() {
            let mut j = i;
            while bytes[j] != b'#' {
                j += 1;
            }
            let len: usize = s[i..j].parse().unwrap();
            res.push(s[j + 1..j + 1 + len].to_string());
            i = j + 1 + len;
        }
        res
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
```rust
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let (mut prefix, mut suffix) = (vec![1; n], vec![1; n]);
        for i in 1..n {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }
        for i in (0..n - 1).rev() {
            suffix[i] = suffix[i + 1] * nums[i + 1];
        }
        (0..n).map(|i| prefix[i] * suffix[i]).collect()
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: In-Place Prefix and Running Suffix (Optimal)
```rust
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![1; n];
        for i in 1..n {
            res[i] = res[i - 1] * nums[i - 1];
        }
        let mut suffix = 1;
        for i in (0..n).rev() {
            res[i] *= suffix;
            suffix *= nums[i];
        }
        res
    }
}
```
**Complexity:** Time O(n) · Space O(1) extra (the output doesn't count)

---

### 36. Valid Sudoku
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/valid-sudoku/)

**Problem:** Check whether a partially filled 9×9 Sudoku board is valid. Each row, each column, and each 3×3 sub-box must have no repeated digits 1–9. Empty cells are `'.'`. The board doesn't need to be solvable.

**Key insight:** Track seen digits per row, per column, and per box. Box index = `(r/3)*3 + c/3`.

#### Approach 1: Boolean Tables
```rust
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [[false; 9]; 9];
        let mut cols = [[false; 9]; 9];
        let mut boxes = [[false; 9]; 9];
        for r in 0..9 {
            for c in 0..9 {
                let ch = board[r][c];
                if ch == '.' {
                    continue;
                }
                let d = (ch as u8 - b'1') as usize;
                let b = (r / 3) * 3 + c / 3;
                if rows[r][d] || cols[c][d] || boxes[b][d] {
                    return false;
                }
                rows[r][d] = true;
                cols[c][d] = true;
                boxes[b][d] = true;
            }
        }
        true
    }
}
```
**Complexity:** Time O(81) = O(1) · Space O(1)

#### Approach 2: Bitmasks
```rust
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let (mut rows, mut cols, mut boxes) = ([0u16; 9], [0u16; 9], [0u16; 9]);
        for r in 0..9 {
            for c in 0..9 {
                let ch = board[r][c];
                if ch == '.' {
                    continue;
                }
                let bit = 1u16 << (ch as u8 - b'1');
                let b = (r / 3) * 3 + c / 3;
                if rows[r] & bit != 0 || cols[c] & bit != 0 || boxes[b] & bit != 0 {
                    return false;
                }
                rows[r] |= bit;
                cols[c] |= bit;
                boxes[b] |= bit;
            }
        }
        true
    }
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
```rust
impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        nums.sort_unstable();
        nums.dedup();
        let (mut best, mut cur) = (1, 1);
        for w in nums.windows(2) {
            cur = if w[1] == w[0] + 1 { cur + 1 } else { 1 };
            best = best.max(cur);
        }
        best
    }
}
```
**Complexity:** Time O(n log n) · Space O(1)

#### Approach 2: HashSet (Optimal)
```rust
use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut best = 0;
        for &n in &set {
            if set.contains(&(n - 1)) {
                continue; // not the start of a run
            }
            let mut len = 1;
            while set.contains(&(n + len)) {
                len += 1;
            }
            best = best.max(len);
        }
        best
    }
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
```rust
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let clean: Vec<u8> = s
            .bytes()
            .filter(|b| b.is_ascii_alphanumeric())
            .map(|b| b.to_ascii_lowercase())
            .collect();
        clean.iter().eq(clean.iter().rev())
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Two Pointers In Place (Optimal)
```rust
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let b = s.as_bytes();
        if b.is_empty() {
            return true;
        }
        let (mut l, mut r) = (0usize, b.len() - 1);
        while l < r {
            if !b[l].is_ascii_alphanumeric() {
                l += 1;
            } else if !b[r].is_ascii_alphanumeric() {
                r -= 1;
            } else {
                if b[l].to_ascii_lowercase() != b[r].to_ascii_lowercase() {
                    return false;
                }
                l += 1;
                r -= 1;
            }
        }
        true
    }
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
```rust
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len() {
            let want = target - numbers[i];
            if let Ok(off) = numbers[i + 1..].binary_search(&want) {
                return vec![i as i32 + 1, (i + 1 + off) as i32 + 1];
            }
        }
        vec![]
    }
}
```
**Complexity:** Time O(n log n) · Space O(1)

#### Approach 2: Two Pointers (Optimal)
```rust
use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, numbers.len() - 1);
        while l < r {
            match (numbers[l] + numbers[r]).cmp(&target) {
                Ordering::Equal => return vec![l as i32 + 1, r as i32 + 1],
                Ordering::Less => l += 1,
                Ordering::Greater => r -= 1,
            }
        }
        vec![]
    }
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 15. 3Sum
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/3sum/)

**Problem:** Given an integer array `nums`, return every unique triplet `[a, b, c]` with distinct indices such that `a + b + c == 0`. The result must not contain duplicate triplets.

**Example:** `[-1,0,1,2,-1,-4]` → `[[-1,-1,2],[-1,0,1]]`

**Key insight:** Sort the array, fix the first element, then run Two Sum II on the rest. Skip equal values to avoid duplicate triplets.

#### Approach 1: HashSet per Anchor
```rust
use std::collections::HashSet;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut res = Vec::new();
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut seen = HashSet::new();
            let mut j = i + 1;
            while j < nums.len() {
                let need = -nums[i] - nums[j];
                if seen.contains(&need) {
                    res.push(vec![nums[i], need, nums[j]]);
                    while j + 1 < nums.len() && nums[j] == nums[j + 1] {
                        j += 1;
                    }
                }
                seen.insert(nums[j]);
                j += 1;
            }
        }
        res
    }
}
```
**Complexity:** Time O(n²) · Space O(n)

#### Approach 2: Sort + Two Pointers (Optimal)
```rust
use std::cmp::Ordering;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let n = nums.len();
        let mut res = Vec::new();
        for i in 0..n.saturating_sub(2) {
            if nums[i] > 0 {
                break; // the smallest value is positive, so no triplet can sum to zero
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut l, mut r) = (i + 1, n - 1);
            while l < r {
                match (nums[i] + nums[l] + nums[r]).cmp(&0) {
                    Ordering::Less => l += 1,
                    Ordering::Greater => r -= 1,
                    Ordering::Equal => {
                        res.push(vec![nums[i], nums[l], nums[r]]);
                        l += 1;
                        r -= 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                    }
                }
            }
        }
        res
    }
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
```rust
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut best = 0;
        for i in 0..height.len() {
            for j in i + 1..height.len() {
                best = best.max((j - i) as i32 * height[i].min(height[j]));
            }
        }
        best
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Two Pointers (Optimal)
```rust
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r, mut best) = (0, height.len() - 1, 0);
        while l < r {
            best = best.max((r - l) as i32 * height[l].min(height[r]));
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        best
    }
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
```rust
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n == 0 {
            return 0;
        }
        let (mut left, mut right) = (vec![0; n], vec![0; n]);
        left[0] = height[0];
        for i in 1..n {
            left[i] = left[i - 1].max(height[i]);
        }
        right[n - 1] = height[n - 1];
        for i in (0..n - 1).rev() {
            right[i] = right[i + 1].max(height[i]);
        }
        (0..n).map(|i| left[i].min(right[i]) - height[i]).sum()
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Monotonic Stack
```rust
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new(); // indices with decreasing heights
        let mut water = 0;
        for i in 0..height.len() {
            while let Some(&top) = stack.last() {
                if height[i] <= height[top] {
                    break;
                }
                stack.pop();
                if let Some(&left) = stack.last() {
                    let width = (i - left - 1) as i32;
                    let bounded = height[left].min(height[i]) - height[top];
                    water += width * bounded;
                }
            }
            stack.push(i);
        }
        water
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 3: Two Pointers (Optimal)
```rust
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let (mut l, mut r) = (0, height.len() - 1);
        let (mut left_max, mut right_max, mut water) = (0, 0, 0);
        while l < r {
            if height[l] < height[r] {
                left_max = left_max.max(height[l]);
                water += left_max - height[l];
                l += 1;
            } else {
                right_max = right_max.max(height[r]);
                water += right_max - height[r];
                r -= 1;
            }
        }
        water
    }
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
```rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut best = 0;
        for i in 0..prices.len() {
            for j in i + 1..prices.len() {
                best = best.max(prices[j] - prices[i]);
            }
        }
        best
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: One Pass with Running Min (Optimal)
```rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut best = 0;
        for p in prices {
            min_price = min_price.min(p);
            best = best.max(p - min_price);
        }
        best
    }
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 3. Longest Substring Without Repeating Characters
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-substring-without-repeating-characters/)

**Problem:** Given a string `s`, return the length of the longest substring that has no repeated characters.

**Example:** `"abcabcbb"` → `3` (`"abc"`); `"pwwkew"` → `3` (`"wke"`)

**Key insight:** Keep a window with no duplicates. When a duplicate arrives, jump the left edge past its previous occurrence.

#### Approach 1: Sliding Window with a HashSet
```rust
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let b = s.as_bytes();
        let mut window = HashSet::new();
        let (mut l, mut best) = (0, 0);
        for r in 0..b.len() {
            while window.contains(&b[r]) {
                window.remove(&b[l]);
                l += 1;
            }
            window.insert(b[r]);
            best = best.max(r - l + 1);
        }
        best as i32
    }
}
```
**Complexity:** Time O(n) (each character enters and leaves once) · Space O(min(n, alphabet))

#### Approach 2: Last-Seen Index Jump (Optimal)
```rust
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last = [0usize; 128]; // stores index+1; 0 means not seen
        let (mut l, mut best) = (0usize, 0usize);
        for (r, b) in s.bytes().enumerate() {
            l = l.max(last[b as usize]);
            last[b as usize] = r + 1;
            best = best.max(r + 1 - l);
        }
        best as i32
    }
}
```
**Complexity:** Time O(n) · Space O(1) (128 slots)

---

### 424. Longest Repeating Character Replacement
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-repeating-character-replacement/)

**Problem:** Given an uppercase string `s` and an integer `k`, you may change up to `k` characters to any other uppercase letter. Return the length of the longest substring containing a single repeated letter after the changes.

**Example:** `s = "AABABBA", k = 1` → `4`

**Key insight:** A window is valid when `windowLen - maxFreq <= k`. `maxFreq` never needs to decrease, because only a larger `maxFreq` can produce a longer answer.

#### Approach 1: Recompute Max Frequency (O(26) per step)
```rust
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let b = s.as_bytes();
        let mut count = [0i32; 26];
        let (mut l, mut best) = (0usize, 0usize);
        for r in 0..b.len() {
            count[(b[r] - b'A') as usize] += 1;
            while (r - l + 1) as i32 - *count.iter().max().unwrap() > k {
                count[(b[l] - b'A') as usize] -= 1;
                l += 1;
            }
            best = best.max(r - l + 1);
        }
        best as i32
    }
}
```
**Complexity:** Time O(26 · n) · Space O(1)

#### Approach 2: Running Max Frequency (Optimal)
```rust
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let b = s.as_bytes();
        let mut count = [0i32; 26];
        let (mut l, mut max_freq, mut best) = (0usize, 0i32, 0usize);
        for r in 0..b.len() {
            let c = (b[r] - b'A') as usize;
            count[c] += 1;
            max_freq = max_freq.max(count[c]);
            if (r - l + 1) as i32 - max_freq > k {
                count[(b[l] - b'A') as usize] -= 1; // shift the window, don't shrink it
                l += 1;
            }
            best = best.max(r - l + 1);
        }
        best as i32
    }
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
```rust
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (a, b) = (s1.as_bytes(), s2.as_bytes());
        if a.len() > b.len() {
            return false;
        }
        let (mut need, mut win) = ([0i32; 26], [0i32; 26]);
        for i in 0..a.len() {
            need[(a[i] - b'a') as usize] += 1;
            win[(b[i] - b'a') as usize] += 1;
        }
        if need == win {
            return true;
        }
        for r in a.len()..b.len() {
            win[(b[r] - b'a') as usize] += 1;
            win[(b[r - a.len()] - b'a') as usize] -= 1;
            if need == win {
                return true;
            }
        }
        false
    }
}
```
**Complexity:** Time O(26 · n) · Space O(1)

#### Approach 2: Track the Number of Matching Letters (Optimal)
```rust
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (a, b) = (s1.as_bytes(), s2.as_bytes());
        if a.len() > b.len() {
            return false;
        }
        let (mut c1, mut c2) = ([0i32; 26], [0i32; 26]);
        for i in 0..a.len() {
            c1[(a[i] - b'a') as usize] += 1;
            c2[(b[i] - b'a') as usize] += 1;
        }
        let mut matches = (0..26).filter(|&i| c1[i] == c2[i]).count();
        for r in a.len()..b.len() {
            if matches == 26 {
                return true;
            }
            for (idx, delta) in [((b[r] - b'a') as usize, 1), ((b[r - a.len()] - b'a') as usize, -1)] {
                if c1[idx] == c2[idx] {
                    matches -= 1;
                }
                c2[idx] += delta;
                if c1[idx] == c2[idx] {
                    matches += 1;
                }
            }
        }
        matches == 26
    }
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
```rust
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let (sb, tb) = (s.as_bytes(), t.as_bytes());
        if tb.is_empty() || tb.len() > sb.len() {
            return String::new();
        }
        let (mut need, mut win) = ([0i32; 128], [0i32; 128]);
        let mut required = 0;
        for &c in tb {
            if need[c as usize] == 0 {
                required += 1;
            }
            need[c as usize] += 1;
        }
        let (mut have, mut l) = (0, 0usize);
        let (mut best_l, mut best_len) = (0usize, usize::MAX);
        for r in 0..sb.len() {
            let c = sb[r] as usize;
            win[c] += 1;
            if need[c] > 0 && win[c] == need[c] {
                have += 1;
            }
            while have == required {
                if r - l + 1 < best_len {
                    best_l = l;
                    best_len = r - l + 1;
                }
                let lc = sb[l] as usize;
                win[lc] -= 1;
                if need[lc] > 0 && win[lc] < need[lc] {
                    have -= 1;
                }
                l += 1;
            }
        }
        if best_len == usize::MAX {
            String::new()
        } else {
            s[best_l..best_l + best_len].to_string()
        }
    }
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
```rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut heap = BinaryHeap::new(); // (value, index), largest value on top
        let mut res = Vec::with_capacity(nums.len() - k + 1);
        for (i, &n) in nums.iter().enumerate() {
            heap.push((n, i));
            if i + 1 >= k {
                while heap.peek().unwrap().1 + k <= i {
                    heap.pop(); // top has left the window
                }
                res.push(heap.peek().unwrap().0);
            }
        }
        res
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Monotonic Deque (Optimal)
```rust
use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut dq: VecDeque<usize> = VecDeque::new(); // indices; their nums values are decreasing
        let mut res = Vec::with_capacity(nums.len() - k + 1);
        for i in 0..nums.len() {
            if let Some(&front) = dq.front() {
                if front + k <= i {
                    dq.pop_front();
                }
            }
            while let Some(&back) = dq.back() {
                if nums[back] >= nums[i] {
                    break;
                }
                dq.pop_back();
            }
            dq.push_back(i);
            if i + 1 >= k {
                res.push(nums[*dq.front().unwrap()]);
            }
        }
        res
    }
}
```
**Complexity:** Time O(n) (each index is pushed and popped at most once) · Space O(k)

---
## 4. Stack

> In Rust, a plain `Vec` is the stack: `push`, `pop` (returns an `Option`), and `last()`.

### 20. Valid Parentheses
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/valid-parentheses/)

**Problem:** Given a string made only of `()[]{}`, decide whether it is valid. Every opening bracket must be closed by the same type of bracket, in the correct order.

**Example:** `"()[]{}"` → `true`; `"(]"` → `false`; `"([)]"` → `false`

**Key insight:** The most recent unmatched opening bracket must be the first one closed, which is last-in, first-out.

#### Approach 1: Stack (Optimal)
```rust
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                '(' => stack.push(')'), // push the expected closer
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                _ => {
                    if stack.pop() != Some(c) {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Repeated Replacement (simple but slow)
```rust
impl Solution {
    pub fn is_valid(mut s: String) -> bool {
        loop {
            let next = s.replace("()", "").replace("[]", "").replace("{}", "");
            if next == s {
                return s.is_empty();
            }
            s = next;
        }
    }
}
```
**Complexity:** Time O(n²) · Space O(n)

---

### 155. Min Stack
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/min-stack/)

**Problem:** Design a stack that supports `push`, `pop`, `top`, and `get_min` (return the smallest element) in O(1) time each.

**Key insight:** Store the minimum at each depth alongside the value, so popping restores the previous minimum automatically.

#### Approach 1: Pairs of (value, current min)
```rust
struct MinStack {
    data: Vec<(i32, i32)>, // (value, min at this depth)
}

impl MinStack {
    fn new() -> Self {
        MinStack { data: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        let m = self.data.last().map_or(val, |&(_, m)| m.min(val));
        self.data.push((val, m));
    }

    fn pop(&mut self) {
        self.data.pop();
    }

    fn top(&self) -> i32 {
        self.data.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.data.last().unwrap().1
    }
}
```
**Complexity:** O(1) per operation · Space O(n)

#### Approach 2: Two Stacks (min stack only grows on a new min)
```rust
struct MinStack {
    stack: Vec<i32>,
    mins: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack { stack: Vec::new(), mins: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.mins.last().map_or(true, |&m| val <= m) {
            self.mins.push(val);
        }
    }

    fn pop(&mut self) {
        if self.stack.pop() == self.mins.last().copied() {
            self.mins.pop();
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.mins.last().unwrap()
    }
}
```
**Complexity:** O(1) per operation · Space O(n), usually less in practice

#### Approach 3: Single Stack Storing Differences (O(1) extra space)
```rust
struct MinStack {
    stack: Vec<i64>, // value - min (i64 avoids overflow)
    min: i64,
}

impl MinStack {
    fn new() -> Self {
        MinStack { stack: Vec::new(), min: 0 }
    }

    fn push(&mut self, val: i32) {
        let v = val as i64;
        if self.stack.is_empty() {
            self.stack.push(0);
            self.min = v;
        } else {
            self.stack.push(v - self.min);
            if v < self.min {
                self.min = v;
            }
        }
    }

    fn pop(&mut self) {
        let diff = self.stack.pop().unwrap();
        if diff < 0 {
            self.min -= diff; // the popped value was the min, so restore the previous one
        }
    }

    fn top(&self) -> i32 {
        let diff = *self.stack.last().unwrap();
        (if diff < 0 { self.min } else { self.min + diff }) as i32
    }

    fn get_min(&self) -> i32 {
        self.min as i32
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

#### Approach 1: Stack with `match` (Optimal)
```rust
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        for tok in &tokens {
            match tok.as_str() {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(match tok.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        _ => a / b, // Rust integer division truncates toward zero
                    });
                }
                num => stack.push(num.parse().unwrap()),
            }
        }
        stack[0]
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Recursive Evaluation from the End
```rust
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        fn eval(tokens: &[String], i: &mut usize) -> i32 {
            *i -= 1;
            let tok = tokens[*i].as_str();
            match tok {
                "+" | "-" | "*" | "/" => {
                    let b = eval(tokens, i); // right operand comes first when reading backward
                    let a = eval(tokens, i);
                    match tok {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        _ => a / b,
                    }
                }
                num => num.parse().unwrap(),
            }
        }
        let mut i = tokens.len();
        eval(&tokens, &mut i)
    }
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
```rust
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn bt(open: i32, close: i32, n: i32, buf: &mut String, res: &mut Vec<String>) {
            if buf.len() == 2 * n as usize {
                res.push(buf.clone());
                return;
            }
            if open < n {
                buf.push('(');
                bt(open + 1, close, n, buf, res);
                buf.pop();
            }
            if close < open {
                buf.push(')');
                bt(open, close + 1, n, buf, res);
                buf.pop();
            }
        }
        let mut res = Vec::new();
        bt(0, 0, n, &mut String::new(), &mut res);
        res
    }
}
```
**Complexity:** Time O(4ⁿ / √n), the nth Catalan number of results × length · Space O(n) recursion

#### Approach 2: Dynamic Programming (closure number)
Every valid string has the form `"(" + a + ")" + b`, where `a` has `i` pairs and `b` has `n-1-i` pairs.
```rust
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut dp: Vec<Vec<String>> = vec![vec![String::new()]];
        for k in 1..=n {
            let mut cur = Vec::new();
            for i in 0..k {
                for a in &dp[i] {
                    for b in &dp[k - 1 - i] {
                        cur.push(format!("({}){}", a, b));
                    }
                }
            }
            dp.push(cur);
        }
        dp.pop().unwrap()
    }
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
```rust
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut stack: Vec<usize> = Vec::new(); // indices of unresolved days
        for (i, &t) in temperatures.iter().enumerate() {
            while let Some(&j) = stack.last() {
                if temperatures[j] >= t {
                    break;
                }
                stack.pop();
                res[j] = (i - j) as i32;
            }
            stack.push(i);
        }
        res
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Backward Jumps (O(1) extra space)
Scan from the right and reuse `res` to hop forward to the next warmer candidate.
```rust
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut res = vec![0i32; n];
        let mut hottest = 0;
        for i in (0..n).rev() {
            let t = temperatures[i];
            if t >= hottest {
                hottest = t;
                continue;
            }
            let mut days = 1usize;
            while temperatures[i + days] <= t {
                days += res[i + days] as usize;
            }
            res[i] = days as i32;
        }
        res
    }
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
```rust
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();
        cars.sort_unstable_by(|a, b| b.0.cmp(&a.0)); // closest to the target first
        let mut stack: Vec<f64> = Vec::new();
        for (p, s) in cars {
            let t = (target - p) as f64 / s as f64;
            if stack.last().map_or(true, |&top| t > top) {
                stack.push(t); // too slow to catch the fleet ahead, so a new fleet
            }
        }
        stack.len() as i32
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sort + Single Variable (exact integer comparison)
Compare times `(target-p1)/s1 > (target-p2)/s2` by cross-multiplying, so there's no floating point at all.
```rust
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i64, i64)> = position.iter().zip(&speed).map(|(&p, &s)| (p as i64, s as i64)).collect();
        cars.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        let target = target as i64;
        let mut fleets = 0;
        let mut slowest: Option<(i64, i64)> = None; // (distance, speed) of the fleet ahead
        for (p, s) in cars {
            let dist = target - p;
            let slower = match slowest {
                None => true,
                Some((d0, s0)) => dist * s0 > d0 * s, // dist/s > d0/s0
            };
            if slower {
                fleets += 1;
                slowest = Some((dist, s));
            }
        }
        fleets
    }
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
```rust
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut best = 0;
        for i in 0..heights.len() {
            let mut min_h = heights[i];
            for j in i..heights.len() {
                min_h = min_h.min(heights[j]);
                best = best.max(min_h * (j - i + 1) as i32);
            }
        }
        best
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Monotonic Increasing Stack (Optimal)
```rust
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        let mut stack: Vec<usize> = Vec::new(); // indices of increasing heights
        let mut best = 0;
        for i in 0..=n {
            let h = if i == n { 0 } else { heights[i] }; // sentinel height 0 at i == n flushes the stack
            while let Some(&top) = stack.last() {
                if heights[top] < h {
                    break;
                }
                stack.pop();
                let width = match stack.last() {
                    Some(&left) => i - left - 1,
                    None => i,
                };
                best = best.max(heights[top] * width as i32);
            }
            stack.push(i);
        }
        best
    }
}
```
**Complexity:** Time O(n) · Space O(n)

---
## 5. Binary Search

> Rust indices are `usize`, so `hi = mid - 1` underflows when `mid == 0`. These solutions use the half-open `[lo, hi)` form, or `i32` bounds where that reads more naturally.

### 704. Binary Search
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/binary-search/)

**Problem:** Given a sorted array `nums` of distinct integers and a `target`, return the target's index, or `-1` if it's not present. Run in O(log n).

**Example:** `nums = [-1,0,3,5,9,12], target = 9` → `4`

**Key insight:** Halve the search space each step. Use `let mid = lo + (hi - lo) / 2;` so the midpoint can't overflow.

#### Approach 1: Iterative, Half-Open Interval (Optimal)
```rust
use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0usize, nums.len()); // search [lo, hi)
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Less => lo = mid + 1,
                Ordering::Greater => hi = mid,
            }
        }
        -1
    }
}
```
**Complexity:** Time O(log n) · Space O(1)

#### Approach 2: Recursive
```rust
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        fn rec(nums: &[i32], target: i32, offset: usize) -> i32 {
            if nums.is_empty() {
                return -1;
            }
            let mid = nums.len() / 2;
            if nums[mid] == target {
                (offset + mid) as i32
            } else if nums[mid] < target {
                rec(&nums[mid + 1..], target, offset + mid + 1)
            } else {
                rec(&nums[..mid], target, offset)
            }
        }
        rec(&nums, target, 0)
    }
}
```
**Complexity:** Time O(log n) · Space O(log n) recursion

#### Approach 3: Standard Library
```rust
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target).map_or(-1, |i| i as i32)
    }
}
```
**Complexity:** Time O(log n) · Space O(1)

---

### 74. Search a 2D Matrix
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/search-a-2d-matrix/)

**Problem:** In an `m × n` matrix, each row is sorted, and each row's first value is greater than the previous row's last value. Return whether `target` is in the matrix, in O(log(m·n)).

**Example:** `[[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3` → `true`

**Key insight:** The matrix is one sorted array of length `m·n`. Index `k` maps to `(k/n, k%n)`.

#### Approach 1: Flattened Binary Search (Optimal)
```rust
use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut lo, mut hi) = (0usize, m * n);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match matrix[mid / n][mid % n].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Less => lo = mid + 1,
                Ordering::Greater => hi = mid,
            }
        }
        false
    }
}
```
**Complexity:** Time O(log(m·n)) · Space O(1)

#### Approach 2: Two Binary Searches (row, then column)
```rust
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // the row that could hold target is the last one whose first element is <= target
        let row = matrix.partition_point(|r| r[0] <= target);
        if row == 0 {
            return false;
        }
        matrix[row - 1].binary_search(&target).is_ok()
    }
}
```
**Complexity:** Time O(log m + log n) · Space O(1)

#### Approach 3: Staircase from the Top-Right (also works for LC 240)
```rust
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (mut r, mut c) = (0usize, matrix[0].len() as i32 - 1);
        while r < matrix.len() && c >= 0 {
            let v = matrix[r][c as usize];
            if v == target {
                return true;
            } else if v > target {
                c -= 1;
            } else {
                r += 1;
            }
        }
        false
    }
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
```rust
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut lo, mut hi) = (1i64, *piles.iter().max().unwrap() as i64);
        while lo < hi {
            let k = lo + (hi - lo) / 2;
            // i64: up to 10^4 piles × 10^9 bananas overflows i32
            let hours: i64 = piles.iter().map(|&p| (p as i64 + k - 1) / k).sum(); // ceil(p / k)
            if hours <= h as i64 {
                hi = k; // k works; try smaller
            } else {
                lo = k + 1;
            }
        }
        lo as i32
    }
}
```
**Complexity:** Time O(n · log M), where M = max pile · Space O(1)

**Brute force:** Try `k = 1, 2, …` until it works: O(n · M), which times out.

---

### 153. Find Minimum in Rotated Sorted Array
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/)

**Problem:** A sorted array of unique values was rotated an unknown number of times. Return its minimum element in O(log n).

**Example:** `[3,4,5,1,2]` → `1`; `[4,5,6,7,0,1,2]` → `0`

**Key insight:** Compare `nums[mid]` with `nums[hi]`. If `mid` is larger, the minimum is to the right. Otherwise it's at `mid` or to its left.

#### Approach: Binary Search vs. the Right End (Optimal)
```rust
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] > nums[hi] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        nums[lo]
    }
}
```
**Complexity:** Time O(log n) · Space O(1)

**Linear alternative:** `*nums.iter().min().unwrap()` is O(n). Fine for a sanity check, but it doesn't meet the requirement.

---

### 33. Search in Rotated Sorted Array
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/search-in-rotated-sorted-array/)

**Problem:** Search for `target` in a rotated sorted array of distinct values. Return its index or `-1`, in O(log n).

**Example:** `nums = [4,5,6,7,0,1,2], target = 0` → `4`

**Key insight:** At every `mid`, at least one half is sorted. Check whether `target` falls inside the sorted half's range.

#### Approach 1: One-Pass Binary Search (Optimal)
```rust
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let (l, m, h) = (nums[lo as usize], nums[mid as usize], nums[hi as usize]);
            if m == target {
                return mid;
            }
            if l <= m {
                // left half sorted
                if l <= target && target < m { hi = mid - 1; } else { lo = mid + 1; }
            } else {
                // right half sorted
                if m < target && target <= h { lo = mid + 1; } else { hi = mid - 1; }
            }
        }
        -1
    }
}
```
**Complexity:** Time O(log n) · Space O(1)

#### Approach 2: Find the Pivot, Then Binary Search the Correct Half
```rust
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let (mut lo, mut hi) = (0, n - 1);
        while lo < hi {
            // find the index of the minimum
            let mid = (lo + hi) / 2;
            if nums[mid] > nums[hi] { lo = mid + 1; } else { hi = mid; }
        }
        let pivot = lo;
        let (start, slice) = if pivot > 0 && target >= nums[0] { (0, &nums[..pivot]) } else { (pivot, &nums[pivot..]) };
        slice.binary_search(&target).map_or(-1, |i| (start + i) as i32)
    }
}
```
**Complexity:** Time O(log n) · Space O(1)

---

### 981. Time Based Key-Value Store
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/time-based-key-value-store/)

**Problem:** Design `TimeMap` with `set(key, value, timestamp)` and `get(key, timestamp)`. `get` returns the value stored at the largest timestamp ≤ `timestamp`, or `""` if none. Calls to `set` arrive with strictly increasing timestamps.

**Key insight:** Each key's entries are already sorted by time, so `get` is an upper-bound binary search.

#### Approach 1: HashMap of Vecs + Binary Search (Optimal)
```rust
use std::collections::HashMap;

struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>, // timestamps arrive in increasing order
}

impl TimeMap {
    fn new() -> Self {
        TimeMap { store: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        match self.store.get(&key) {
            None => String::new(),
            Some(list) => {
                let i = list.partition_point(|(t, _)| *t <= timestamp); // first index with t > timestamp
                if i == 0 { String::new() } else { list[i - 1].1.clone() }
            }
        }
    }
}
```
**Complexity:** `set` O(1) amortized · `get` O(log n) · Space O(total entries)

#### Approach 2: `BTreeMap` per Key (works for any timestamp order)
```rust
use std::collections::{BTreeMap, HashMap};

struct TimeMap {
    store: HashMap<String, BTreeMap<i32, String>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap { store: HashMap::new() }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store.entry(key).or_default().insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.store
            .get(&key)
            .and_then(|tm| tm.range(..=timestamp).next_back()) // floor entry
            .map_or(String::new(), |(_, v)| v.clone())
    }
}
```
**Complexity:** `set` O(log n) · `get` O(log n) · Space O(total entries)

---

### 4. Median of Two Sorted Arrays
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/median-of-two-sorted-arrays/)

**Problem:** Given sorted arrays `nums1` (size m) and `nums2` (size n), return the median of their combined elements in O(log(m+n)).

**Example:** `[1,3]`, `[2]` → `2.0`; `[1,2]`, `[3,4]` → `2.5`

**Key insight:** Binary search a cut in the shorter array so the left halves of both arrays together hold half the elements, and every element on the left is ≤ every element on the right.

#### Approach 1: Merge (simple)
```rust
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged = Vec::with_capacity(nums1.len() + nums2.len());
        let (mut i, mut j) = (0, 0);
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] <= nums2[j] { merged.push(nums1[i]); i += 1; } else { merged.push(nums2[j]); j += 1; }
        }
        merged.extend_from_slice(&nums1[i..]);
        merged.extend_from_slice(&nums2[j..]);
        let t = merged.len();
        if t % 2 == 1 {
            merged[t / 2] as f64
        } else {
            (merged[t / 2 - 1] as f64 + merged[t / 2] as f64) / 2.0
        }
    }
}
```
**Complexity:** Time O(m + n) · Space O(m + n)

#### Approach 2: Binary Search Partition (Optimal)
```rust
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() <= nums2.len() { (&nums1, &nums2) } else { (&nums2, &nums1) }; // search the shorter array
        let (m, n) = (a.len(), b.len());
        let half = (m + n + 1) / 2;
        let (mut lo, mut hi) = (0usize, m);
        loop {
            let i = (lo + hi) / 2; // elements taken from a
            let j = half - i;      // elements taken from b
            let a_left = if i > 0 { a[i - 1] } else { i32::MIN };
            let a_right = if i < m { a[i] } else { i32::MAX };
            let b_left = if j > 0 { b[j - 1] } else { i32::MIN };
            let b_right = if j < n { b[j] } else { i32::MAX };
            if a_left <= b_right && b_left <= a_right {
                let left_max = a_left.max(b_left) as f64;
                if (m + n) % 2 == 1 {
                    return left_max;
                }
                return (left_max + a_right.min(b_right) as f64) / 2.0;
            } else if a_left > b_right {
                hi = i - 1;
            } else {
                lo = i + 1;
            }
        }
    }
}
```
**Complexity:** Time O(log(min(m, n))) · Space O(1)

---
## 6. Linked List

> All problems here use LeetCode's `ListNode` with `Option<Box<ListNode>>` links (see [Shared Definitions](#shared-definitions)). Ownership makes some classic pointer tricks awkward in safe Rust. The solutions below use `take()` to move nodes, `as_ref()`/`as_mut()` to borrow, and when needed collect values or nodes into a `Vec` and rebuild.

### 206. Reverse Linked List
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/reverse-linked-list/)

**Problem:** Given the head of a singly linked list, reverse the list and return the new head.

**Example:** `1→2→3→4→5` → `5→4→3→2→1`

**Key insight:** Walk the list once, pointing each node's `Next` back at the previous node.

#### Approach 1: Iterative (Optimal)
```rust
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut prev, mut cur) = (None, head);
        while let Some(mut node) = cur {
            cur = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Recursive (with an accumulator)
```rust
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn rev(cur: Option<Box<ListNode>>, acc: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            match cur {
                None => acc,
                Some(mut node) => {
                    let next = node.next.take();
                    node.next = acc;
                    rev(next, Some(node))
                }
            }
        }
        rev(head, None)
    }
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
```rust
impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        while list1.is_some() && list2.is_some() {
            let take_first = list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val;
            let src = if take_first { &mut list1 } else { &mut list2 };
            let mut node = src.take().unwrap();
            *src = node.next.take();
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = if list1.is_some() { list1 } else { list2 };
        dummy.next
    }
}
```
**Complexity:** Time O(n + m) · Space O(1)

#### Approach 2: Recursive
```rust
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, r) | (r, None) => r,
            (Some(mut a), Some(mut b)) => {
                if a.val <= b.val {
                    a.next = Self::merge_two_lists(a.next.take(), Some(b));
                    Some(a)
                } else {
                    b.next = Self::merge_two_lists(Some(a), b.next.take());
                    Some(b)
                }
            }
        }
    }
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
```rust
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // 1. count, then cut after the first ceil(n/2) nodes
        let mut len = 0;
        let mut p = head.as_ref();
        while let Some(node) = p {
            len += 1;
            p = node.next.as_ref();
        }
        if len < 3 {
            return;
        }
        let mut cur = head.as_mut();
        for _ in 0..(len + 1) / 2 - 1 {
            cur = cur.unwrap().next.as_mut();
        }
        let mut second = cur.unwrap().next.take();
        // 2. reverse the second half
        let mut rev: Option<Box<ListNode>> = None;
        while let Some(mut node) = second {
            second = node.next.take();
            node.next = rev;
            rev = Some(node);
        }
        // 3. interleave: after each node of the first half, splice in one node from rev
        let mut first = head.as_mut();
        while let Some(node) = first {
            if let Some(mut r) = rev {
                rev = r.next.take();
                r.next = node.next.take();
                node.next = Some(r);
                first = node.next.as_mut().unwrap().next.as_mut();
            } else {
                break;
            }
        }
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Collect Nodes into a VecDeque, Then Relink
```rust
use std::collections::VecDeque;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut nodes: VecDeque<Box<ListNode>> = VecDeque::new();
        let mut cur = head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
            nodes.push_back(node);
        }
        // pick front, back, front, back, ... then build the list from the tail backward
        let mut order = Vec::with_capacity(nodes.len());
        let mut front = true;
        while let Some(node) = if front { nodes.pop_front() } else { nodes.pop_back() } {
            order.push(node);
            front = !front;
        }
        let mut next: Option<Box<ListNode>> = None;
        while let Some(mut node) = order.pop() {
            node.next = next;
            next = Some(node);
        }
        *head = next;
    }
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 19. Remove Nth Node From End of List
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/remove-nth-node-from-end-of-list/)

**Problem:** Remove the `n`th node from the end of a linked list and return the head.

**Example:** `1→2→3→4→5, n = 2` → `1→2→3→5`

**Key insight:** Move a lead pointer `n` steps ahead, then advance both pointers until the lead hits the end. The trailing pointer is then just before the target.

#### Approach 1: Count the Length, Then Unlink (Optimal in safe Rust)
```rust
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut p = head.as_ref();
        while let Some(node) = p {
            len += 1;
            p = node.next.as_ref();
        }
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = &mut dummy;
        for _ in 0..len - n {
            cur = cur.next.as_mut().unwrap();
        }
        let removed = cur.next.take();
        cur.next = removed.and_then(|mut node| node.next.take());
        dummy.next
    }
}
```
**Complexity:** Time O(L) · Space O(1)

#### Approach 2: Recursive, Counting Position from the End While Unwinding
A single pass: the recursion reaches the end first, and each frame learns its position from the end as the calls return.
```rust
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // returns (rebuilt rest of the list, this node's position counted from the end)
        fn rec(node: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
            match node {
                None => (None, 0),
                Some(mut nd) => {
                    let (rest, pos) = rec(nd.next.take(), n);
                    let pos = pos + 1;
                    if pos == n {
                        (rest, pos) // drop this node
                    } else {
                        nd.next = rest;
                        (Some(nd), pos)
                    }
                }
            }
        }
        rec(head, n).0
    }
}
```
**Complexity:** Time O(L) · Space O(L) recursion

> The classic "lead pointer n steps ahead" trick needs two live references into one list, and one of them must be mutable. Safe Rust doesn't allow that with `Box` links, so the counting version above is the idiomatic one-pass-equivalent answer.

---

### 138. Copy List with Random Pointer
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/copy-list-with-random-pointer/)

**Problem:** Each node has `next` and `random` pointers, and `random` may point to any node or `None`. Return a deep copy of the list.

**Key insight:** Map each original node to its copy. Alternatively, weave each copy right after its original so `orig.Random.Next` is the copied random target.

> No Rust template on LeetCode. The node below uses `Rc<RefCell<…>>` so that `random` can point anywhere in the list.

#### Approach 1: HashMap from Original Node to Copy
```rust
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub random: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, next: None, random: None }))
    }
}

impl Solution {
    pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        // key each original node by its pointer address
        let mut copies: HashMap<*const RefCell<Node>, Rc<RefCell<Node>>> = HashMap::new();
        let mut cur = head.clone();
        while let Some(n) = cur {
            copies.insert(Rc::as_ptr(&n), Node::new(n.borrow().val));
            cur = n.borrow().next.clone();
        }
        let lookup = |o: &Option<Rc<RefCell<Node>>>| o.as_ref().map(|x| Rc::clone(&copies[&Rc::as_ptr(x)]));
        let mut cur = head.clone();
        while let Some(n) = cur {
            let copy = &copies[&Rc::as_ptr(&n)];
            copy.borrow_mut().next = lookup(&n.borrow().next);
            copy.borrow_mut().random = lookup(&n.borrow().random);
            cur = n.borrow().next.clone();
        }
        head.map(|h| Rc::clone(&copies[&Rc::as_ptr(&h)]))
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Interweaving (O(1) extra space)
```rust
use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
    pub random: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, next: None, random: None }))
    }
}

impl Solution {
    pub fn copy_random_list(head: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let head = head?;
        // 1. A → A' → B → B' ...
        let mut cur = Some(Rc::clone(&head));
        while let Some(n) = cur {
            let copy = Node::new(n.borrow().val);
            copy.borrow_mut().next = n.borrow_mut().next.take();
            cur = copy.borrow().next.clone();
            n.borrow_mut().next = Some(copy);
        }
        // 2. copy.random = original.random.next
        let mut cur = Some(Rc::clone(&head));
        while let Some(n) = cur {
            let copy = n.borrow().next.clone().unwrap();
            let rnd = n.borrow().random.as_ref().map(|r| r.borrow().next.clone().unwrap());
            copy.borrow_mut().random = rnd;
            cur = copy.borrow().next.clone();
        }
        // 3. unweave the two lists
        let new_head = head.borrow().next.clone();
        let mut cur = Some(Rc::clone(&head));
        while let Some(n) = cur {
            let copy = n.borrow().next.clone().unwrap();
            let next_orig = copy.borrow().next.clone();
            n.borrow_mut().next = next_orig.clone();
            copy.borrow_mut().next = next_orig.as_ref().and_then(|o| o.borrow().next.clone());
            cur = next_orig;
        }
        new_head
    }
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
```rust
impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }
            carry = sum / 10;
            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next
    }
}
```
**Complexity:** Time O(max(m, n)) · Space O(1) extra (the output doesn't count)

#### Approach 2: Recursive
```rust
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn add(a: Option<Box<ListNode>>, b: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
            if a.is_none() && b.is_none() && carry == 0 {
                return None;
            }
            let (av, an) = a.map_or((0, None), |n| (n.val, n.next));
            let (bv, bn) = b.map_or((0, None), |n| (n.val, n.next));
            let sum = av + bv + carry;
            Some(Box::new(ListNode { val: sum % 10, next: add(an, bn, sum / 10) }))
        }
        add(l1, l2, 0)
    }
}
```
**Complexity:** Time O(max(m, n)) · Space O(max(m, n)) recursion

---

### 141. Linked List Cycle
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/linked-list-cycle/)

**Problem:** Return `true` if the linked list contains a cycle.

**Key insight:** Floyd's tortoise and hare. If there's a cycle, the fast pointer eventually laps the slow one.

> No Rust template on LeetCode: a `Box` list can't form a cycle. Here the list is built from `Rc<RefCell<…>>` nodes, and identity is compared with `Rc::ptr_eq`.

#### Approach 1: HashSet of Node Addresses
```rust
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub struct RcNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<RcNode>>>,
}

impl Solution {
    pub fn has_cycle(head: Option<Rc<RefCell<RcNode>>>) -> bool {
        let mut seen: HashSet<*const RefCell<RcNode>> = HashSet::new();
        let mut cur = head;
        while let Some(n) = cur {
            if !seen.insert(Rc::as_ptr(&n)) {
                return true;
            }
            cur = n.borrow().next.clone();
        }
        false
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Floyd's Fast and Slow Pointers (Optimal)
```rust
use std::cell::RefCell;
use std::rc::Rc;

pub struct RcNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<RcNode>>>,
}

impl Solution {
    pub fn has_cycle(head: Option<Rc<RefCell<RcNode>>>) -> bool {
        let next = |n: &Option<Rc<RefCell<RcNode>>>| n.as_ref().and_then(|x| x.borrow().next.clone());
        let (mut slow, mut fast) = (head.clone(), head);
        loop {
            fast = next(&next(&fast));
            slow = next(&slow);
            match (&slow, &fast) {
                (Some(s), Some(f)) if Rc::ptr_eq(s, f) => return true,
                (_, None) => return false,
                _ => {}
            }
        }
    }
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 287. Find the Duplicate Number
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/find-the-duplicate-number/)

**Problem:** `nums` has `n + 1` integers, each in `[1, n]`, and exactly one value is repeated (possibly many times). Return it without modifying the array, using O(1) extra space.

**Example:** `[1,3,4,2,2]` → `2`

**Key insight:** Treat `i → nums[i]` as a linked list. The duplicate value is the entry point of the cycle, which you can find with Floyd's algorithm.

#### Approach 1: HashSet (uses extra space)
```rust
use std::collections::HashSet;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        for n in nums {
            if !seen.insert(n) {
                return n;
            }
        }
        -1
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Binary Search on Value (pigeonhole)
```rust
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (1, nums.len() as i32 - 1);
        while lo < hi {
            let mid = (lo + hi) / 2;
            let cnt = nums.iter().filter(|&&n| n <= mid).count() as i32;
            if cnt > mid { hi = mid; } else { lo = mid + 1; }
        }
        lo
    }
}
```
**Complexity:** Time O(n log n) · Space O(1)

#### Approach 3: Floyd's Cycle Detection (Optimal)
```rust
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let f = |i: i32| nums[i as usize];
        let (mut slow, mut fast) = (f(0), f(f(0)));
        while slow != fast {
            slow = f(slow);
            fast = f(f(fast));
        }
        slow = 0;
        while slow != fast {
            slow = f(slow);
            fast = f(fast);
        }
        slow
    }
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 146. LRU Cache
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/lru-cache/)

**Problem:** Design an LRU (least recently used) cache with a fixed `capacity`. `get(key)` returns the value or `-1`. `put(key, value)` inserts or updates a key, evicting the least recently used key when over capacity. Both must run in O(1).

**Key insight:** A hash map gives O(1) lookup, and a doubly linked list gives O(1) reordering and eviction.

#### Approach 1: HashMap + Index-Based Doubly Linked List (Optimal)
Using a `Vec` arena with `usize` links avoids `Rc<RefCell>` overhead and keeps everything in safe Rust.
```rust
use std::collections::HashMap;

struct Entry {
    key: i32,
    val: i32,
    prev: usize,
    next: usize,
}

struct LRUCache {
    cap: usize,
    map: HashMap<i32, usize>, // key → slot in `nodes`
    nodes: Vec<Entry>,        // slots 0 (head) and 1 (tail) are sentinels
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let nodes = vec![
            Entry { key: 0, val: 0, prev: 0, next: 1 },
            Entry { key: 0, val: 0, prev: 0, next: 1 },
        ];
        LRUCache { cap: capacity as usize, map: HashMap::new(), nodes }
    }

    fn unlink(&mut self, i: usize) {
        let (p, n) = (self.nodes[i].prev, self.nodes[i].next);
        self.nodes[p].next = n;
        self.nodes[n].prev = p;
    }

    fn push_front(&mut self, i: usize) {
        let first = self.nodes[0].next;
        self.nodes[i].prev = 0;
        self.nodes[i].next = first;
        self.nodes[first].prev = i;
        self.nodes[0].next = i;
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key).copied() {
            None => -1,
            Some(i) => {
                self.unlink(i);
                self.push_front(i);
                self.nodes[i].val
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(&i) = self.map.get(&key) {
            self.nodes[i].val = value;
            self.unlink(i);
            self.push_front(i);
            return;
        }
        let i = if self.map.len() == self.cap {
            let lru = self.nodes[1].prev; // evict the LRU and reuse its slot
            self.unlink(lru);
            self.map.remove(&self.nodes[lru].key);
            self.nodes[lru].key = key;
            self.nodes[lru].val = value;
            lru
        } else {
            self.nodes.push(Entry { key, val: value, prev: 0, next: 0 });
            self.nodes.len() - 1
        };
        self.map.insert(key, i);
        self.push_front(i);
    }
}
```
**Complexity:** O(1) per operation · Space O(capacity)

#### Approach 2: HashMap + Timestamps in a BTreeMap
Simpler to write, but each operation is O(log n).
```rust
use std::collections::{BTreeMap, HashMap};

struct LRUCache {
    cap: usize,
    tick: u64,
    map: HashMap<i32, (i32, u64)>, // key → (value, last-used tick)
    order: BTreeMap<u64, i32>,     // tick → key; the smallest tick is the LRU
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache { cap: capacity as usize, tick: 0, map: HashMap::new(), order: BTreeMap::new() }
    }

    fn touch(&mut self, key: i32, value: i32) {
        if let Some(&(_, t)) = self.map.get(&key) {
            self.order.remove(&t);
        }
        self.tick += 1;
        self.map.insert(key, (value, self.tick));
        self.order.insert(self.tick, key);
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map.get(&key).copied() {
            None => -1,
            Some((v, _)) => {
                self.touch(key, v);
                v
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.touch(key, value);
        if self.map.len() > self.cap {
            let (&t, &k) = self.order.iter().next().unwrap();
            self.order.remove(&t);
            self.map.remove(&k);
        }
    }
}
```
**Complexity:** O(log n) per operation · Space O(capacity)

---

### 23. Merge k Sorted Lists
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/merge-k-sorted-lists/)

**Problem:** Merge `k` sorted linked lists into one sorted list.

**Example:** `[1→4→5, 1→3→4, 2→6]` → `1→1→2→3→4→4→5→6`

**Key insight:** Either keep a min-heap of the `k` current heads, or merge the lists in pairs (divide and conquer). Both are O(N log k).

#### Approach 1: Min-Heap
`ListNode` derives `Eq` but not `Ord`, so the heap holds `Reverse((val, index))` and the nodes live in a side vector.
```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heads: Vec<Option<Box<ListNode>>> = lists;
        let mut heap = BinaryHeap::new();
        for (i, h) in heads.iter().enumerate() {
            if let Some(node) = h {
                heap.push(Reverse((node.val, i)));
            }
        }
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        while let Some(Reverse((_, i))) = heap.pop() {
            let mut node = heads[i].take().unwrap();
            heads[i] = node.next.take();
            if let Some(next) = &heads[i] {
                heap.push(Reverse((next.val, i)));
            }
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next
    }
}
```
**Complexity:** Time O(N log k) for N total nodes · Space O(k)

#### Approach 2: Divide and Conquer Pairwise Merge (Optimal, O(1) extra)
```rust
impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        fn merge2(a: Option<Box<ListNode>>, b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let (mut a, mut b) = (a, b);
            let mut dummy = Box::new(ListNode::new(0));
            let mut tail = &mut dummy;
            while a.is_some() && b.is_some() {
                let src = if a.as_ref().unwrap().val <= b.as_ref().unwrap().val { &mut a } else { &mut b };
                let mut node = src.take().unwrap();
                *src = node.next.take();
                tail.next = Some(node);
                tail = tail.next.as_mut().unwrap();
            }
            tail.next = if a.is_some() { a } else { b };
            dummy.next
        }
        if lists.is_empty() {
            return None;
        }
        let mut step = 1;
        while step < lists.len() {
            let mut i = 0;
            while i + step < lists.len() {
                let (a, b) = (lists[i].take(), lists[i + step].take());
                lists[i] = merge2(a, b);
                i += 2 * step;
            }
            step *= 2;
        }
        lists[0].take()
    }
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

#### Approach 1: Recursive (Optimal for Box-based lists)
```rust
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // check that k nodes exist
        let mut probe = head.as_ref();
        for _ in 0..k {
            match probe {
                None => return head, // fewer than k left: leave them as they are
                Some(node) => probe = node.next.as_ref(),
            }
        }
        // reverse the first k nodes, linking the tail to the reversed rest
        let mut cur = head;
        let mut group: Vec<Box<ListNode>> = Vec::with_capacity(k as usize);
        for _ in 0..k {
            let mut node = cur.unwrap();
            cur = node.next.take();
            group.push(node);
        }
        let mut prev = Self::reverse_k_group(cur, k);
        for mut node in group {
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}
```
**Complexity:** Time O(n) · Space O(n/k) recursion + O(k) buffer

#### Approach 2: Iterative with a Tail Cursor
```rust
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut rest = head;
        loop {
            // detach the next k nodes (or fewer)
            let mut group = Vec::with_capacity(k as usize);
            while group.len() < k as usize {
                match rest {
                    None => break,
                    Some(mut node) => {
                        rest = node.next.take();
                        group.push(node);
                    }
                }
            }
            let full = group.len() == k as usize;
            if !full {
                // reattach the leftover nodes in their original order
                for node in group {
                    tail.next = Some(node);
                    tail = tail.next.as_mut().unwrap();
                }
                break;
            }
            for node in group.into_iter().rev() {
                tail.next = Some(node);
                tail = tail.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}
```
**Complexity:** Time O(n) · Space O(k)

---
## 7. Trees

> All problems here use LeetCode's `TreeNode` behind `Option<Rc<RefCell<TreeNode>>>` (see [Shared Definitions](#shared-definitions)). Pattern to remember: `if let Some(node) = root { let n = node.borrow(); n.val; n.left.clone(); }`. Cloning an `Rc` is cheap: it bumps a reference count and doesn't copy the subtree.

### 226. Invert Binary Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/invert-binary-tree/)

**Problem:** Mirror a binary tree by swapping every node's left and right children, then return the root.

**Example:** `[4,2,7,1,3,6,9]` → `[4,7,2,9,6,3,1]`

**Key insight:** Swap the children at every node. Traversal order doesn't matter.

#### Approach 1: Recursive DFS (Optimal)
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = &root {
            let mut n = node.borrow_mut();
            let left = n.left.take();
            let right = n.right.take();
            n.left = Self::invert_tree(right);
            n.right = Self::invert_tree(left);
        }
        root
    }
}
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: Iterative BFS
```rust
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        if let Some(r) = &root {
            q.push_back(Rc::clone(r));
        }
        while let Some(node) = q.pop_front() {
            let mut n = node.borrow_mut();
            let n = &mut *n;
            std::mem::swap(&mut n.left, &mut n.right);
            if let Some(l) = &n.left {
                q.push_back(Rc::clone(l));
            }
            if let Some(r) = &n.right {
                q.push_back(Rc::clone(r));
            }
        }
        root
    }
}
```
**Complexity:** Time O(n) · Space O(w), where w is the maximum width

---

### 104. Maximum Depth of Binary Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/maximum-depth-of-binary-tree/)

**Problem:** Return the number of nodes on the longest path from the root down to a leaf.

**Example:** `[3,9,20,null,null,15,7]` → `3`

#### Approach 1: Recursive DFS
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let n = node.borrow();
                1 + Self::max_depth(n.left.clone()).max(Self::max_depth(n.right.clone()))
            }
        }
    }
}
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: BFS Level Count
```rust
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = root.into_iter().collect();
        let mut depth = 0;
        while !q.is_empty() {
            depth += 1;
            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                let n = node.borrow();
                q.extend(n.left.clone());
                q.extend(n.right.clone());
            }
        }
        depth
    }
}
```
**Complexity:** Time O(n) · Space O(w)

#### Approach 3: Iterative DFS with an Explicit Stack
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack: Vec<(Rc<RefCell<TreeNode>>, i32)> = root.into_iter().map(|r| (r, 1)).collect();
        let mut best = 0;
        while let Some((node, d)) = stack.pop() {
            best = best.max(d);
            let n = node.borrow();
            for child in [n.left.clone(), n.right.clone()].into_iter().flatten() {
                stack.push((child, d + 1));
            }
        }
        best
    }
}
```
**Complexity:** Time O(n) · Space O(h)

---

### 543. Diameter of Binary Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/diameter-of-binary-tree/)

**Problem:** Return the length (in edges) of the longest path between any two nodes. The path doesn't have to pass through the root.

**Example:** `[1,2,3,4,5]` → `3` (path 4→2→1→3)

**Key insight:** The longest path through a node is `height(left) + height(right)`. Compute heights bottom-up and track the best sum.

#### Approach: DFS Returning Height (Optimal)
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn height(node: &Option<Rc<RefCell<TreeNode>>>, best: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    let (l, r) = (height(&n.left, best), height(&n.right, best));
                    *best = (*best).max(l + r);
                    1 + l.max(r)
                }
            }
        }
        let mut best = 0;
        height(&root, &mut best);
        best
    }
}
```
**Complexity:** Time O(n) · Space O(h)

**Brute force:** At each node, compute the left and right heights separately: O(n²) on a skewed tree.

---

### 110. Balanced Binary Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/balanced-binary-tree/)

**Problem:** A tree is height-balanced if, at every node, the left and right subtree heights differ by at most 1. Return whether the tree is balanced.

**Key insight:** Return height bottom-up, and use `-1` to signal that a subtree is already unbalanced.

#### Approach 1: Top-Down (simple, O(n²))
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            node.as_ref().map_or(0, |n| {
                let n = n.borrow();
                1 + height(&n.left).max(height(&n.right))
            })
        }
        match &root {
            None => true,
            Some(n) => {
                let n = n.borrow();
                (height(&n.left) - height(&n.right)).abs() <= 1
                    && Self::is_balanced(n.left.clone())
                    && Self::is_balanced(n.right.clone())
            }
        }
    }
}
```
**Complexity:** Time O(n²) worst case · Space O(h)

#### Approach 2: Bottom-Up with Early Exit (Optimal)
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // Some(height) if balanced, None as soon as any subtree is unbalanced
        fn check(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
            match node {
                None => Some(0),
                Some(n) => {
                    let n = n.borrow();
                    let l = check(&n.left)?;
                    let r = check(&n.right)?;
                    if (l - r).abs() > 1 { None } else { Some(1 + l.max(r)) }
                }
            }
        }
        check(&root).is_some()
    }
}
```
**Complexity:** Time O(n) · Space O(h)

---

### 100. Same Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/same-tree/)

**Problem:** Return whether two binary trees have the same structure and the same node values.

#### Approach 1: Derived `PartialEq` (one line)
`TreeNode` derives `PartialEq`, and `Rc<RefCell<T>>` compares by value, so `==` does a full structural comparison.
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        p == q
    }
}
```
**Complexity:** Time O(min(n, m)) · Space O(h)

#### Approach 2: Explicit Recursion (what an interviewer expects)
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                let (a, b) = (a.borrow(), b.borrow());
                a.val == b.val
                    && Self::is_same_tree(a.left.clone(), b.left.clone())
                    && Self::is_same_tree(a.right.clone(), b.right.clone())
            }
            _ => false,
        }
    }
}
```
**Complexity:** Time O(min(n, m)) · Space O(h)

#### Approach 3: Iterative BFS on Pairs
```rust
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::from(vec![(p, q)]);
        while let Some(pair) = queue.pop_front() {
            match pair {
                (None, None) => continue,
                (Some(a), Some(b)) => {
                    let (a, b) = (a.borrow(), b.borrow());
                    if a.val != b.val {
                        return false;
                    }
                    queue.push_back((a.left.clone(), b.left.clone()));
                    queue.push_back((a.right.clone(), b.right.clone()));
                }
                _ => return false,
            }
        }
        true
    }
}
```
**Complexity:** Time O(n) · Space O(w)

---

### 572. Subtree of Another Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/subtree-of-another-tree/)

**Problem:** Return whether `subRoot` matches, in structure and values, some node of `root` together with all of that node's descendants.

**Key insight:** Run Same Tree at every node (O(m·n)), or serialize both trees and use substring search (O(m+n)).

#### Approach 1: Same-Tree Check at Every Node
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match &root {
            None => sub_root.is_none(),
            Some(n) => {
                root == sub_root // derived structural equality, same as LC 100
                    || Self::is_subtree(n.borrow().left.clone(), sub_root.clone())
                    || Self::is_subtree(n.borrow().right.clone(), sub_root.clone())
            }
        }
    }
}
```
**Complexity:** Time O(m · n) · Space O(h)

#### Approach 2: Serialize + Substring Search
Delimiters around each value (`^12` vs `^2`) and explicit null markers prevent false matches.
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn ser(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut String) {
            match node {
                None => out.push('#'),
                Some(n) => {
                    let n = n.borrow();
                    out.push('^');
                    out.push_str(&n.val.to_string());
                    ser(&n.left, out);
                    ser(&n.right, out);
                }
            }
        }
        let (mut a, mut b) = (String::new(), String::new());
        ser(&root, &mut a);
        ser(&sub_root, &mut b);
        a.contains(&b) // swap in KMP for a guaranteed O(m+n)
    }
}
```
**Complexity:** Time O(m + n) with KMP; `str::contains` is fast in practice · Space O(m + n)

---

### 235. Lowest Common Ancestor of a Binary Search Tree
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)

**Problem:** Given a BST and two of its nodes `p` and `q`, return their lowest common ancestor: the deepest node that has both as descendants (a node counts as its own descendant).

**Key insight:** If both values are smaller than the current node, go left. If both are larger, go right. Otherwise the current node is where they split, and that's the LCA.

#### Approach 1: Iterative (Optimal)
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (pv, qv) = (p.unwrap().borrow().val, q.unwrap().borrow().val);
        let mut cur = root;
        while let Some(node) = cur {
            let (val, left, right) = {
                let n = node.borrow();
                (n.val, n.left.clone(), n.right.clone())
            };
            if pv < val && qv < val {
                cur = left;
            } else if pv > val && qv > val {
                cur = right;
            } else {
                return Some(node); // p and q split here (or one of them is this node)
            }
        }
        None
    }
}
```
**Complexity:** Time O(h) · Space O(1)

#### Approach 2: Recursive
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node = root?;
        let val = node.borrow().val;
        let (pv, qv) = (p.as_ref()?.borrow().val, q.as_ref()?.borrow().val);
        if pv < val && qv < val {
            let left = node.borrow().left.clone();
            Self::lowest_common_ancestor(left, p, q)
        } else if pv > val && qv > val {
            let right = node.borrow().right.clone();
            Self::lowest_common_ancestor(right, p, q)
        } else {
            Some(node)
        }
    }
}
```
**Complexity:** Time O(h) · Space O(h)

---

### 102. Binary Tree Level Order Traversal
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/binary-tree-level-order-traversal/)

**Problem:** Return the node values level by level, left to right.

**Example:** `[3,9,20,null,null,15,7]` → `[[3],[9,20],[15,7]]`

#### Approach 1: BFS with a Queue (Optimal)
```rust
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = root.into_iter().collect();
        while !q.is_empty() {
            let mut level = Vec::with_capacity(q.len());
            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                let n = node.borrow();
                level.push(n.val);
                q.extend(n.left.clone());
                q.extend(n.right.clone());
            }
            res.push(level);
        }
        res
    }
}
```
**Complexity:** Time O(n) · Space O(w)

#### Approach 2: DFS with a Depth Index
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, d: usize, res: &mut Vec<Vec<i32>>) {
            if let Some(n) = node {
                let n = n.borrow();
                if d == res.len() {
                    res.push(Vec::new());
                }
                res[d].push(n.val);
                dfs(&n.left, d + 1, res);
                dfs(&n.right, d + 1, res);
            }
        }
        let mut res = Vec::new();
        dfs(&root, 0, &mut res);
        res
    }
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
```rust
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = root.into_iter().collect();
        while !q.is_empty() {
            let size = q.len();
            for i in 0..size {
                let node = q.pop_front().unwrap();
                let n = node.borrow();
                if i == size - 1 {
                    res.push(n.val);
                }
                q.extend(n.left.clone());
                q.extend(n.right.clone());
            }
        }
        res
    }
}
```
**Complexity:** Time O(n) · Space O(w)

#### Approach 2: Right-First DFS
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, d: usize, res: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                if d == res.len() {
                    res.push(n.val); // first node seen at this depth is the rightmost
                }
                dfs(&n.right, d + 1, res);
                dfs(&n.left, d + 1, res);
            }
        }
        let mut res = Vec::new();
        dfs(&root, 0, &mut res);
        res
    }
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
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max_so_far: i32) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    let good = (n.val >= max_so_far) as i32;
                    let m = max_so_far.max(n.val);
                    good + dfs(&n.left, m) + dfs(&n.right, m)
                }
            }
        }
        dfs(&root, i32::MIN)
    }
}
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: BFS Carrying (node, max)
```rust
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut q: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = root.into_iter().map(|r| (r, i32::MIN)).collect();
        let mut count = 0;
        while let Some((node, mx)) = q.pop_front() {
            let n = node.borrow();
            if n.val >= mx {
                count += 1;
            }
            let mx = mx.max(n.val);
            for child in [n.left.clone(), n.right.clone()].into_iter().flatten() {
                q.push_back((child, mx));
            }
        }
        count
    }
}
```
**Complexity:** Time O(n) · Space O(w)

---

### 98. Validate Binary Search Tree
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/validate-binary-search-tree/)

**Problem:** Return whether the tree is a valid BST. Every node's left subtree must hold only smaller values, its right subtree only larger values, and both subtrees must themselves be BSTs.

**Key insight:** Comparing a node only with its direct children isn't enough. Pass down a valid `(lo, hi)` range, or check that the in-order traversal is strictly increasing.

#### Approach 1: DFS with Bounds (Optimal)
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // i64 bounds so nodes holding i32::MIN / i32::MAX are handled correctly
        fn valid(node: &Option<Rc<RefCell<TreeNode>>>, lo: i64, hi: i64) -> bool {
            match node {
                None => true,
                Some(n) => {
                    let n = n.borrow();
                    let v = n.val as i64;
                    lo < v && v < hi && valid(&n.left, lo, v) && valid(&n.right, v, hi)
                }
            }
        }
        valid(&root, i64::MIN, i64::MAX)
    }
}
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: Iterative In-Order Traversal
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut cur = root;
        let mut prev: Option<i32> = None;
        while cur.is_some() || !stack.is_empty() {
            while let Some(node) = cur {
                cur = node.borrow().left.clone();
                stack.push(node);
            }
            let node = stack.pop().unwrap();
            let v = node.borrow().val;
            if prev.map_or(false, |p| v <= p) {
                return false;
            }
            prev = Some(v);
            cur = node.borrow().right.clone();
        }
        true
    }
}
```
**Complexity:** Time O(n) · Space O(h)

---

### 230. Kth Smallest Element in a BST
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/kth-smallest-element-in-a-bst/)

**Problem:** Return the `k`th smallest value (1-indexed) in a BST.

**Key insight:** In-order traversal visits a BST's values in sorted order, so stop at the `k`th node visited.

#### Approach 1: Iterative In-Order (Optimal, stops early)
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let (mut cur, mut k) = (root, k);
        loop {
            while let Some(node) = cur {
                cur = node.borrow().left.clone();
                stack.push(node);
            }
            let node = stack.pop().unwrap();
            k -= 1;
            if k == 0 {
                return node.borrow().val;
            }
            cur = node.borrow().right.clone();
        }
    }
}
```
**Complexity:** Time O(h + k) · Space O(h)

#### Approach 2: Recursive In-Order with a Counter
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, k: &mut i32, ans: &mut i32) {
            if let Some(n) = node {
                if *k == 0 {
                    return;
                }
                let n = n.borrow();
                inorder(&n.left, k, ans);
                *k -= 1;
                if *k == 0 {
                    *ans = n.val;
                    return;
                }
                inorder(&n.right, k, ans);
            }
        }
        let (mut k, mut ans) = (k, 0);
        inorder(&root, &mut k, &mut ans);
        ans
    }
}
```
**Complexity:** Time O(h + k) · Space O(h)

**Follow-up (frequent inserts and deletes):** Store a subtree size in each node, then walk down by comparing `k` with the left subtree's size. Each query costs O(h).

---

### 105. Construct Binary Tree from Preorder and Inorder Traversal
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/)

**Problem:** Given a tree's `preorder` and `inorder` traversals (all values unique), rebuild the tree and return its root.

**Example:** `preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]` → `[3,9,20,null,null,15,7]`

**Key insight:** `preorder[0]` is the root. Its position in `inorder` splits the left subtree from the right one.

#### Approach 1: Slicing (simple, O(n²))
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(pre: &[i32], ino: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if pre.is_empty() {
                return None;
            }
            let mid = ino.iter().position(|&v| v == pre[0]).unwrap();
            let mut node = TreeNode::new(pre[0]);
            node.left = build(&pre[1..mid + 1], &ino[..mid]);
            node.right = build(&pre[mid + 1..], &ino[mid + 1..]);
            Some(Rc::new(RefCell::new(node)))
        }
        build(&preorder, &inorder)
    }
}
```
**Complexity:** Time O(n²) worst case (the `position` scan) · Space O(n)

#### Approach 2: HashMap of Inorder Indices (Optimal)
```rust
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(pre: &[i32], p: &mut usize, pos: &HashMap<i32, usize>, lo: usize, hi: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if lo == hi {
                return None; // empty inorder range [lo, hi)
            }
            let v = pre[*p];
            *p += 1;
            let mid = pos[&v];
            let mut node = TreeNode::new(v);
            node.left = build(pre, p, pos, lo, mid);
            node.right = build(pre, p, pos, mid + 1, hi);
            Some(Rc::new(RefCell::new(node)))
        }
        let pos: HashMap<i32, usize> = inorder.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        let mut p = 0;
        build(&preorder, &mut p, &pos, 0, inorder.len())
    }
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 124. Binary Tree Maximum Path Sum
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/binary-tree-maximum-path-sum/)

**Problem:** A path is any sequence of connected nodes where each node appears at most once. It doesn't have to pass through the root. Return the maximum path sum over all non-empty paths.

**Example:** `[-10,9,20,null,null,15,7]` → `42` (15→20→7)

**Key insight:** At each node, the best path that bends there is `val + max(0, leftGain) + max(0, rightGain)`. Pass up to the parent only `val + max(0, max(leftGain, rightGain))`, since a path can't branch upward.

#### Approach: Post-Order DFS (Optimal)
```rust
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn gain(node: &Option<Rc<RefCell<TreeNode>>>, best: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let n = n.borrow();
                    let l = gain(&n.left, best).max(0);
                    let r = gain(&n.right, best).max(0);
                    *best = (*best).max(n.val + l + r); // best path that bends at this node
                    n.val + l.max(r)                    // only one branch can extend upward
                }
            }
        }
        let mut best = i32::MIN;
        gain(&root, &mut best);
        best
    }
}
```
**Complexity:** Time O(n) · Space O(h)

---

### 297. Serialize and Deserialize Binary Tree
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/)

**Problem:** Design `serialize`, which turns a binary tree into a string, and `deserialize`, which turns that string back into the same tree.

**Key insight:** A pre-order traversal with explicit null markers (`N`) is enough to rebuild the tree exactly.

#### Approach 1: Pre-Order DFS
```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Codec;

impl Codec {
    fn new() -> Self {
        Codec
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn ser(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<String>) {
            match node {
                None => out.push("N".to_string()),
                Some(n) => {
                    let n = n.borrow();
                    out.push(n.val.to_string());
                    ser(&n.left, out);
                    ser(&n.right, out);
                }
            }
        }
        let mut out = Vec::new();
        ser(&root, &mut out);
        out.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn des<'a>(it: &mut impl Iterator<Item = &'a str>) -> Option<Rc<RefCell<TreeNode>>> {
            match it.next()? {
                "N" => None,
                v => {
                    let mut node = TreeNode::new(v.parse().unwrap());
                    node.left = des(it);
                    node.right = des(it);
                    Some(Rc::new(RefCell::new(node)))
                }
            }
        }
        des(&mut data.split(','))
    }
}
```
**Complexity:** Time O(n) for both · Space O(n)

#### Approach 2: BFS (level order, LeetCode's own format)
```rust
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Codec;

impl Codec {
    fn new() -> Self {
        Codec
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut out = Vec::new();
        let mut q: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::from(vec![root]);
        while let Some(slot) = q.pop_front() {
            match slot {
                None => out.push("N".to_string()),
                Some(n) => {
                    let n = n.borrow();
                    out.push(n.val.to_string());
                    q.push_back(n.left.clone());
                    q.push_back(n.right.clone());
                }
            }
        }
        out.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let vals: Vec<&str> = data.split(',').collect();
        let make = |s: &str| -> Option<Rc<RefCell<TreeNode>>> {
            if s == "N" { None } else { Some(Rc::new(RefCell::new(TreeNode::new(s.parse().unwrap())))) }
        };
        let root = make(vals[0]);
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = root.iter().cloned().collect();
        let mut i = 1;
        while let Some(node) = q.pop_front() {
            let left = make(vals[i]);
            let right = make(vals[i + 1]);
            i += 2;
            if let Some(l) = &left {
                q.push_back(Rc::clone(l));
            }
            if let Some(r) = &right {
                q.push_back(Rc::clone(r));
            }
            let mut n = node.borrow_mut();
            n.left = left;
            n.right = right;
        }
        root
    }
}
```
**Complexity:** Time O(n) · Space O(n)

---
## 8. Tries

> In Rust a trie node owns its children: `children: [Option<Box<TrieNode>>; 26]`. `Default` builds an empty node, because arrays of `Option<Box<_>>` implement `Default`.

### 208. Implement Trie (Prefix Tree)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/implement-trie-prefix-tree/)

**Problem:** Implement a trie with `insert(word)`, `search(word)` (is this exact word stored?), and `starts_with(prefix)` (does any stored word begin with this prefix?). Inputs are lowercase letters.

**Key insight:** Each node has up to 26 children plus an end-of-word flag.

#### Approach 1: Fixed Array of Children (Optimal for lowercase)
```rust
#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for b in word.bytes() {
            node = node.children[(b - b'a') as usize].get_or_insert_with(|| Box::new(Trie::default()));
        }
        node.is_end = true;
    }

    fn find(&self, s: &str) -> Option<&Trie> {
        let mut node = self;
        for b in s.bytes() {
            node = node.children[(b - b'a') as usize].as_deref()?;
        }
        Some(node)
    }

    fn search(&self, word: String) -> bool {
        self.find(&word).map_or(false, |n| n.is_end)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find(&prefix).is_some()
    }
}
```
**Complexity:** Time O(L) per operation · Space O(total characters × 26)

#### Approach 2: HashMap Children (any alphabet, less memory when sparse)
```rust
use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.is_end = true;
    }

    fn find(&self, s: &str) -> Option<&Trie> {
        let mut node = self;
        for c in s.chars() {
            node = node.children.get(&c)?;
        }
        Some(node)
    }

    fn search(&self, word: String) -> bool {
        self.find(&word).map_or(false, |n| n.is_end)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find(&prefix).is_some()
    }
}
```
**Complexity:** Time O(L) average · Space O(total characters)

---

### 211. Design Add and Search Words Data Structure
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/design-add-and-search-words-data-structure/)

**Problem:** Support `add_word(word)` and `search(word)`. The search pattern may contain `.`, which matches any single letter.

**Example:** add `bad`, `dad`, `mad`; `search("pad")` → `false`, `search(".ad")` → `true`, `search("b..")` → `true`

**Key insight:** Use a trie. On `.`, run DFS into every child.

#### Approach: Trie + DFS (Optimal)
```rust
#[derive(Default)]
struct WordDictionary {
    children: [Option<Box<WordDictionary>>; 26],
    is_end: bool,
}

impl WordDictionary {
    fn new() -> Self {
        Default::default()
    }

    fn add_word(&mut self, word: String) {
        let mut node = self;
        for b in word.bytes() {
            node = node.children[(b - b'a') as usize].get_or_insert_with(Default::default);
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        fn dfs(node: &WordDictionary, w: &[u8]) -> bool {
            match w.split_first() {
                None => node.is_end,
                Some((&b'.', rest)) => node.children.iter().flatten().any(|ch| dfs(ch, rest)),
                Some((&c, rest)) => node.children[(c - b'a') as usize].as_ref().map_or(false, |ch| dfs(ch, rest)),
            }
        }
        dfs(self, word.as_bytes())
    }
}
```
**Complexity:** `add_word` O(L) · `search` O(L) without dots, O(26^L) worst case with all dots · Space O(total characters)

**Alternative:** Bucket words by length in a `HashMap<usize, Vec<String>>` and compare byte by byte. Simpler, but `search` costs O(words × L).

---

### 212. Word Search II
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/word-search-ii/)

**Problem:** Given an `m × n` letter grid and a list of `words`, return every word that can be traced through horizontally or vertically adjacent cells, without reusing a cell within a word.

**Example:** `board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]` → `["eat","oath"]`

**Key insight:** Build a trie from all the words and run one DFS per cell, descending the grid and the trie together. Prune trie nodes once their words are found.

#### Approach 1: Trie + Backtracking (no pruning)
```rust
#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    word: Option<String>, // Some at the end of a word
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut root = Node::default();
        for w in words {
            let mut node = &mut root;
            for b in w.bytes() {
                node = node.children[(b - b'a') as usize].get_or_insert_with(Default::default);
            }
            node.word = Some(w);
        }
        fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize, parent: &mut Node, res: &mut Vec<String>) {
            let ch = board[r][c];
            if ch == '#' {
                return;
            }
            let node = match parent.children[(ch as u8 - b'a') as usize].as_mut() {
                Some(n) => n,
                None => return,
            };
            if let Some(w) = node.word.take() {
                res.push(w); // take() reports each word once
            }
            board[r][c] = '#';
            if r > 0 { dfs(board, r - 1, c, node, res); }
            if r + 1 < board.len() { dfs(board, r + 1, c, node, res); }
            if c > 0 { dfs(board, r, c - 1, node, res); }
            if c + 1 < board[0].len() { dfs(board, r, c + 1, node, res); }
            board[r][c] = ch;
        }
        let mut res = Vec::new();
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                dfs(&mut board, r, c, &mut root, &mut res);
            }
        }
        res
    }
}
```
**Complexity:** Time O(m · n · 4 · 3^(L-1)), where L is the maximum word length · Space O(total characters in `words`)

#### Approach 2: Trie + Backtracking with Leaf Pruning (Optimal)
```rust
#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    word: Option<String>,
    child_count: usize,
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut root = Node::default();
        for w in words {
            let mut node = &mut root;
            for b in w.bytes() {
                let i = (b - b'a') as usize;
                if node.children[i].is_none() {
                    node.child_count += 1;
                }
                node = node.children[i].get_or_insert_with(Default::default);
            }
            node.word = Some(w);
        }
        fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize, parent: &mut Node, res: &mut Vec<String>) {
            let ch = board[r][c];
            if ch == '#' {
                return;
            }
            let idx = (ch as u8 - b'a') as usize;
            let prune = {
                let node = match parent.children[idx].as_mut() {
                    Some(n) => n,
                    None => return,
                };
                if let Some(w) = node.word.take() {
                    res.push(w);
                }
                board[r][c] = '#';
                if r > 0 { dfs(board, r - 1, c, node, res); }
                if r + 1 < board.len() { dfs(board, r + 1, c, node, res); }
                if c > 0 { dfs(board, r, c - 1, node, res); }
                if c + 1 < board[0].len() { dfs(board, r, c + 1, node, res); }
                board[r][c] = ch;
                node.child_count == 0 && node.word.is_none()
            };
            if prune {
                parent.children[idx] = None; // drop an exhausted leaf
                parent.child_count -= 1;
            }
        }
        let mut res = Vec::new();
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                dfs(&mut board, r, c, &mut root, &mut res);
            }
        }
        res
    }
}
```
**Complexity:** Same worst case. Removing leaves cuts the search space sharply in practice; this is the version to use in interviews.

**Brute force:** Run LC 79 (Word Search) once per word: O(W · m · n · 4^L). Much slower when there are many words.

---
## 9. Heap / Priority Queue

> `std::collections::BinaryHeap` is a **max-heap**. Wrap values in `std::cmp::Reverse` to get a min-heap. Tuples compare lexicographically, so `Reverse((dist, node))` orders by `dist` first. `f64` isn't `Ord`, so compare integers (for example squared distances) instead.

### 703. Kth Largest Element in a Stream
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/kth-largest-element-in-a-stream/)

**Problem:** Design a class built from `k` and an initial array `nums`. Each call to `add(val)` adds a value to the stream and returns the current `k`th largest element.

**Example:** `k = 3, nums = [4,5,8,2]`; `add(3)` → 4, `add(5)` → 5, `add(10)` → 5

**Key insight:** Keep a min-heap of size `k`. Its top is the `k`th largest element.

#### Approach: Min-Heap of Size k (Optimal)
```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kl = KthLargest { k: k as usize, heap: BinaryHeap::new() };
        for n in nums {
            kl.add(n);
        }
        kl
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}
```
**Complexity:** `add` O(log k) · Space O(k)

**Alternative:** Keep a sorted `Vec` and insert at `partition_point`. `add` is then O(n), from shifting elements.

---

### 1046. Last Stone Weight
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/last-stone-weight/)

**Problem:** Each turn, smash the two heaviest stones `x ≤ y`. If they're equal, both are destroyed. Otherwise a stone of weight `y - x` remains. Return the weight of the last stone, or `0` if none are left.

**Example:** `[2,7,4,1,8,1]` → `1`

#### Approach 1: Max-Heap (Optimal)
```rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = stones.into_iter().collect(); // O(n) heapify
        while heap.len() > 1 {
            let y = heap.pop().unwrap();
            let x = heap.pop().unwrap();
            if y != x {
                heap.push(y - x);
            }
        }
        heap.pop().unwrap_or(0)
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Bucket Sort (weights ≤ 1000)
```rust
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let max_w = *stones.iter().max().unwrap() as usize;
        let mut bucket = vec![0i32; max_w + 1];
        for &s in &stones {
            bucket[s as usize] += 1;
        }
        let (mut biggest, mut cur) = (0usize, max_w);
        while cur > 0 {
            if bucket[cur] == 0 {
                cur -= 1;
            } else if biggest == 0 {
                bucket[cur] %= 2; // equal pairs destroy each other
                if bucket[cur] == 1 {
                    biggest = cur;
                }
                cur -= 1;
            } else {
                bucket[cur] -= 1;
                if biggest - cur <= cur {
                    bucket[biggest - cur] += 1;
                    biggest = 0;
                } else {
                    biggest -= cur;
                }
            }
        }
        biggest as i32
    }
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
```rust
impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        points.sort_unstable_by_key(|p| p[0] * p[0] + p[1] * p[1]);
        points.truncate(k as usize);
        points
    }
}
```
**Complexity:** Time O(n log n) · Space O(1)

#### Approach 2: Max-Heap of Size k
```rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new(); // (squared distance, index)
        for (i, p) in points.iter().enumerate() {
            heap.push((p[0] * p[0] + p[1] * p[1], i));
            if heap.len() > k as usize {
                heap.pop(); // drop the farthest
            }
        }
        heap.into_iter().map(|(_, i)| points[i].clone()).collect()
    }
}
```
**Complexity:** Time O(n log k) · Space O(k)

#### Approach 3: Quickselect via `select_nth_unstable_by_key` (Optimal on average)
The standard library ships an introselect: average O(n), and O(n log n) in the worst case.
```rust
impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        if k < points.len() {
            points.select_nth_unstable_by_key(k - 1, |p| p[0] * p[0] + p[1] * p[1]);
        }
        points.truncate(k);
        points
    }
}
```
**Complexity:** Time O(n) average · Space O(1)

---

### 215. Kth Largest Element in an Array
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/kth-largest-element-in-an-array/)

**Problem:** Return the `k`th largest element in an unsorted array (in sorted order, not the `k`th distinct value).

**Example:** `nums = [3,2,1,5,6,4], k = 2` → `5`

#### Approach 1: Sort
```rust
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        nums[nums.len() - k as usize]
    }
}
```
**Complexity:** Time O(n log n) · Space O(1)

#### Approach 2: Min-Heap of Size k
```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for n in nums {
            heap.push(Reverse(n));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        heap.peek().unwrap().0
    }
}
```
**Complexity:** Time O(n log k) · Space O(k)

#### Approach 3: Hand-Written Quickselect with Three-Way Partition
A three-way partition handles many duplicates. The pivot index comes from a tiny xorshift generator, so no `rand` crate is needed.
```rust
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let target = nums.len() - k as usize; // index in ascending order
        let (mut lo, mut hi) = (0usize, nums.len() - 1);
        let mut seed: u64 = 0x9E37_79B9_7F4A_7C15;
        loop {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let pivot = nums[lo + (seed % (hi - lo + 1) as u64) as usize];
            // Dutch national flag over [lo, hi]
            let (mut lt, mut i, mut gt) = (lo, lo, hi + 1);
            while i < gt {
                if nums[i] < pivot {
                    nums.swap(lt, i);
                    lt += 1;
                    i += 1;
                } else if nums[i] > pivot {
                    gt -= 1;
                    nums.swap(i, gt);
                } else {
                    i += 1;
                }
            }
            if target < lt {
                hi = lt - 1;
            } else if target >= gt {
                lo = gt;
            } else {
                return pivot;
            }
        }
    }
}
```
**Complexity:** Time O(n) average · Space O(1)

#### Approach 4: Standard-Library Selection (one line)
```rust
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let idx = nums.len() - k as usize;
        *nums.select_nth_unstable(idx).1
    }
}
```
**Complexity:** Time O(n) average · Space O(1)

#### Approach 5: Counting Sort (values in [-10⁴, 10⁴])
```rust
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        const OFF: i32 = 10_000;
        let mut cnt = vec![0i32; (2 * OFF + 1) as usize];
        for n in nums {
            cnt[(n + OFF) as usize] += 1;
        }
        let mut k = k;
        for v in (0..cnt.len()).rev() {
            k -= cnt[v];
            if k <= 0 {
                return v as i32 - OFF;
            }
        }
        unreachable!()
    }
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
```rust
use std::collections::{BinaryHeap, VecDeque};

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut freq = [0i32; 26];
        for t in tasks {
            freq[(t as u8 - b'A') as usize] += 1;
        }
        let mut heap: BinaryHeap<i32> = freq.iter().copied().filter(|&f| f > 0).collect();
        let mut cooldown: VecDeque<(i32, i32)> = VecDeque::new(); // (remaining count, time when ready)
        let mut time = 0;
        while !heap.is_empty() || !cooldown.is_empty() {
            time += 1;
            if let Some(c) = heap.pop() {
                if c > 1 {
                    cooldown.push_back((c - 1, time + n));
                }
            }
            if cooldown.front().map_or(false, |&(_, ready)| ready == time) {
                heap.push(cooldown.pop_front().unwrap().0);
            }
        }
        time
    }
}
```
**Complexity:** Time O(T · log 26) = O(T), where T is the total time · Space O(26)

#### Approach 2: Math Formula (Optimal)
```rust
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut freq = [0i32; 26];
        for &t in &tasks {
            freq[(t as u8 - b'A') as usize] += 1;
        }
        let max_f = *freq.iter().max().unwrap();
        let max_count = freq.iter().filter(|&&f| f == max_f).count() as i32;
        (tasks.len() as i32).max((max_f - 1) * (n + 1) + max_count)
    }
}
```
**Complexity:** Time O(len(tasks)) · Space O(1)

---

### 355. Design Twitter
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/design-twitter/)

**Problem:** Design a simplified Twitter with `post_tweet(user_id, tweet_id)`, `get_news_feed(user_id)` (the 10 most recent tweet IDs from the user and the people they follow, newest first), `follow`, and `unfollow`.

**Key insight:** The news feed is a merge of k sorted lists. Each user's tweets are already in time order, so a heap seeded with each followee's newest tweet works.

#### Approach: HashMaps + Heap Merge (Optimal)
```rust
use std::collections::{BinaryHeap, HashMap, HashSet};

struct Twitter {
    clock: i32,
    tweets: HashMap<i32, Vec<(i32, i32)>>, // user → [(time, tweet_id)] in time order
    follows: HashMap<i32, HashSet<i32>>,
}

impl Twitter {
    fn new() -> Self {
        Twitter { clock: 0, tweets: HashMap::new(), follows: HashMap::new() }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.clock += 1;
        self.tweets.entry(user_id).or_default().push((self.clock, tweet_id));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        // max-heap of (time, tweet_id, user, index into that user's list)
        let mut heap: BinaryHeap<(i32, i32, i32, usize)> = BinaryHeap::new();
        let mut users: HashSet<i32> = self.follows.get(&user_id).cloned().unwrap_or_default();
        users.insert(user_id);
        for u in users {
            if let Some(list) = self.tweets.get(&u) {
                if let Some(&(t, id)) = list.last() {
                    heap.push((t, id, u, list.len() - 1));
                }
            }
        }
        let mut res = Vec::new();
        while res.len() < 10 {
            let (_, id, u, i) = match heap.pop() {
                Some(top) => top,
                None => break,
            };
            res.push(id);
            if i > 0 {
                let (t, nid) = self.tweets[&u][i - 1];
                heap.push((t, nid, u, i - 1));
            }
        }
        res
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows.entry(follower_id).or_default().insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(set) = self.follows.get_mut(&follower_id) {
            set.remove(&followee_id);
        }
    }
}
```
**Complexity:** `post_tweet`, `follow`, `unfollow` O(1) · `get_news_feed` O(F + 10 log F) for F followees · Space O(users + tweets + follows)

**Simpler alternative:** Gather every tweet from the user and their followees, sort by time, and take 10: O(N log N) per feed.

---

### 295. Find Median from Data Stream
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/find-median-from-data-stream/)

**Problem:** Design `MedianFinder` with `add_num(num)` and `find_median()`, which returns the median of every number added so far.

**Key insight:** Keep two heaps: a max-heap for the smaller half and a min-heap for the larger half. Keep their sizes within one of each other, so the median is on top.

#### Approach 1: Two Heaps (Optimal)
```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    low: BinaryHeap<i32>,           // max-heap: smaller half
    high: BinaryHeap<Reverse<i32>>, // min-heap: larger half
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder { low: BinaryHeap::new(), high: BinaryHeap::new() }
    }

    fn add_num(&mut self, num: i32) {
        self.low.push(num);
        let moved = self.low.pop().unwrap(); // move low's max to high
        self.high.push(Reverse(moved));
        if self.high.len() > self.low.len() {
            let Reverse(back) = self.high.pop().unwrap();
            self.low.push(back);
        }
    }

    fn find_median(&self) -> f64 {
        let lo = *self.low.peek().unwrap() as f64;
        if self.low.len() > self.high.len() {
            lo
        } else {
            (lo + self.high.peek().unwrap().0 as f64) / 2.0
        }
    }
}
```
**Complexity:** `add_num` O(log n) · `find_median` O(1) · Space O(n)

#### Approach 2: Sorted Vec with Binary Insert (simple)
```rust
struct MedianFinder {
    data: Vec<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder { data: Vec::new() }
    }

    fn add_num(&mut self, num: i32) {
        let i = self.data.partition_point(|&x| x < num);
        self.data.insert(i, num);
    }

    fn find_median(&self) -> f64 {
        let n = self.data.len();
        if n % 2 == 1 {
            self.data[n / 2] as f64
        } else {
            (self.data[n / 2 - 1] as f64 + self.data[n / 2] as f64) / 2.0
        }
    }
}
```
**Complexity:** `add_num` O(n) · `find_median` O(1)

**Follow-up (all values in [0, 100]):** Keep a 101-bucket count array and walk it to find the median, which is O(100) per query.

---
## 10. Backtracking

> **Template:** choose → recurse → un-choose. In Rust, pass `&mut Vec` for the path and the results, and save a **copy** with `res.push(path.clone())`. Nested `fn` helpers keep the borrow story simple, because they don't capture anything.

### 78. Subsets
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/subsets/)

**Problem:** Given an array of unique integers, return every possible subset (the power set), with no duplicates.

**Example:** `[1,2,3]` → `[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]`

#### Approach 1: Backtracking (include or exclude)
```rust
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn bt(nums: &[i32], i: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if i == nums.len() {
                res.push(path.clone());
                return;
            }
            path.push(nums[i]); // include
            bt(nums, i + 1, path, res);
            path.pop();         // exclude
            bt(nums, i + 1, path, res);
        }
        let mut res = Vec::new();
        bt(&nums, 0, &mut Vec::new(), &mut res);
        res
    }
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n) recursion

#### Approach 2: Iterative Cascading
```rust
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![vec![]];
        for n in nums {
            let extra: Vec<Vec<i32>> = res
                .iter()
                .map(|s| {
                    let mut t = s.clone();
                    t.push(n);
                    t
                })
                .collect();
            res.extend(extra);
        }
        res
    }
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n · 2ⁿ) for the output

#### Approach 3: Bitmask
```rust
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        (0..1u32 << n)
            .map(|mask| (0..n).filter(|&i| mask >> i & 1 == 1).map(|i| nums[i]).collect())
            .collect()
    }
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
```rust
impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn bt(c: &[i32], start: usize, remain: i32, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if remain == 0 {
                res.push(path.clone());
                return;
            }
            for i in start..c.len() {
                if c[i] > remain {
                    break; // sorted, so every later candidate is too big
                }
                path.push(c[i]);
                bt(c, i, remain - c[i], path, res); // i, not i+1: reuse is allowed
                path.pop();
            }
        }
        candidates.sort_unstable();
        let mut res = Vec::new();
        bt(&candidates, 0, target, &mut Vec::new(), &mut res);
        res
    }
}
```
**Complexity:** Time O(N^(T/M + 1)), where T = target and M = smallest candidate · Space O(T/M) recursion

#### Approach 2: DP Building Combinations (unbounded knapsack style)
```rust
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let t = target as usize;
        let mut dp: Vec<Vec<Vec<i32>>> = vec![Vec::new(); t + 1];
        dp[0].push(Vec::new());
        for &c in &candidates {
            // outer loop over coins means no permutations
            let c = c as usize;
            for s in c..=t {
                let extended: Vec<Vec<i32>> = dp[s - c]
                    .iter()
                    .map(|comb| {
                        let mut v = comb.clone();
                        v.push(c as i32);
                        v
                    })
                    .collect();
                dp[s].extend(extended);
            }
        }
        dp.swap_remove(t)
    }
}
```
**Complexity:** Exponential in the output size · Space proportional to all partial combinations

---

### 46. Permutations
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/permutations/)

**Problem:** Given an array of distinct integers, return every permutation.

**Example:** `[1,2,3]` → `[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]`

#### Approach 1: Backtracking with a Used Array
```rust
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn bt(nums: &[i32], used: &mut Vec<bool>, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if path.len() == nums.len() {
                res.push(path.clone());
                return;
            }
            for i in 0..nums.len() {
                if used[i] {
                    continue;
                }
                used[i] = true;
                path.push(nums[i]);
                bt(nums, used, path, res);
                path.pop();
                used[i] = false;
            }
        }
        let mut res = Vec::new();
        bt(&nums, &mut vec![false; nums.len()], &mut Vec::new(), &mut res);
        res
    }
}
```
**Complexity:** Time O(n · n!) · Space O(n)

#### Approach 2: In-Place Swapping
```rust
impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn bt(nums: &mut Vec<i32>, k: usize, res: &mut Vec<Vec<i32>>) {
            if k == nums.len() {
                res.push(nums.clone());
                return;
            }
            for i in k..nums.len() {
                nums.swap(k, i);
                bt(nums, k + 1, res);
                nums.swap(k, i);
            }
        }
        let mut res = Vec::new();
        bt(&mut nums, 0, &mut res);
        res
    }
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
```rust
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn bt(nums: &[i32], start: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            res.push(path.clone());
            for i in start..nums.len() {
                if i > start && nums[i] == nums[i - 1] {
                    continue; // skip duplicates at the same depth
                }
                path.push(nums[i]);
                bt(nums, i + 1, path, res);
                path.pop();
            }
        }
        nums.sort_unstable();
        let mut res = Vec::new();
        bt(&nums, 0, &mut Vec::new(), &mut res);
        res
    }
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n)

#### Approach 2: Iterative (extend only the subsets added last round when the value repeats)
```rust
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut res: Vec<Vec<i32>> = vec![vec![]];
        let mut prev_size = 0;
        for i in 0..nums.len() {
            let start = if i > 0 && nums[i] == nums[i - 1] { prev_size } else { 0 };
            let size = res.len();
            for j in start..size {
                let mut s = res[j].clone();
                s.push(nums[i]);
                res.push(s);
            }
            prev_size = size;
        }
        res
    }
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n · 2ⁿ) for the output

---

### 40. Combination Sum II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/combination-sum-ii/)

**Problem:** Given `candidates` (which may contain duplicates) and a `target`, return every unique combination that sums to `target`. Each element may be used at most once.

**Example:** `candidates = [10,1,2,7,6,1,5], target = 8` → `[[1,1,6],[1,2,5],[1,7],[2,6]]`

#### Approach: Sort + Backtracking with Duplicate Skipping (Optimal)
```rust
impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn bt(c: &[i32], start: usize, remain: i32, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if remain == 0 {
                res.push(path.clone());
                return;
            }
            for i in start..c.len() {
                if i > start && c[i] == c[i - 1] {
                    continue;
                }
                if c[i] > remain {
                    break;
                }
                path.push(c[i]);
                bt(c, i + 1, remain - c[i], path, res);
                path.pop();
            }
        }
        candidates.sort_unstable();
        let mut res = Vec::new();
        bt(&candidates, 0, target, &mut Vec::new(), &mut res);
        res
    }
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
```rust
impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        fn dfs(b: &mut Vec<Vec<char>>, w: &[char], r: i32, c: i32, i: usize) -> bool {
            if i == w.len() {
                return true;
            }
            if r < 0 || c < 0 || r as usize >= b.len() || c as usize >= b[0].len() {
                return false;
            }
            let (ru, cu) = (r as usize, c as usize);
            if b[ru][cu] != w[i] {
                return false;
            }
            b[ru][cu] = '#';
            let found = dfs(b, w, r + 1, c, i + 1)
                || dfs(b, w, r - 1, c, i + 1)
                || dfs(b, w, r, c + 1, i + 1)
                || dfs(b, w, r, c - 1, i + 1);
            b[ru][cu] = w[i];
            found
        }
        let w: Vec<char> = word.chars().collect();
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                if dfs(&mut board, &w, r as i32, c as i32, 0) {
                    return true;
                }
            }
        }
        false
    }
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
```rust
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn is_pal(b: &[u8]) -> bool {
            b.iter().eq(b.iter().rev())
        }
        fn bt(s: &str, start: usize, path: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
            if start == s.len() {
                res.push(path.clone());
                return;
            }
            for end in start + 1..=s.len() {
                if is_pal(&s.as_bytes()[start..end]) {
                    path.push(s[start..end].to_string());
                    bt(s, end, path, res);
                    path.pop();
                }
            }
        }
        let mut res = Vec::new();
        bt(&s, 0, &mut Vec::new(), &mut res);
        res
    }
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n)

#### Approach 2: Precomputed Palindrome DP Table
```rust
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let b = s.as_bytes();
        let n = b.len();
        let mut pal = vec![vec![false; n]; n];
        for i in (0..n).rev() {
            for j in i..n {
                pal[i][j] = b[i] == b[j] && (j - i < 2 || pal[i + 1][j - 1]);
            }
        }
        fn bt(s: &str, pal: &Vec<Vec<bool>>, start: usize, path: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
            if start == s.len() {
                res.push(path.clone());
                return;
            }
            for end in start..s.len() {
                if pal[start][end] {
                    path.push(s[start..=end].to_string());
                    bt(s, pal, end + 1, path, res);
                    path.pop();
                }
            }
        }
        let mut res = Vec::new();
        bt(&s, &pal, 0, &mut Vec::new(), &mut res);
        res
    }
}
```
**Complexity:** Time O(n · 2ⁿ), with each palindrome check O(1) · Space O(n²)

---

### 17. Letter Combinations of a Phone Number
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/letter-combinations-of-a-phone-number/)

**Problem:** Given a string of digits `2`–`9`, return every letter combination they could spell on a phone keypad.

**Example:** `"23"` → `["ad","ae","af","bd","be","bf","cd","ce","cf"]`

#### Approach 1: Backtracking
```rust
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        const KEYS: [&str; 10] = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        fn bt(d: &[u8], i: usize, buf: &mut String, res: &mut Vec<String>) {
            if i == d.len() {
                res.push(buf.clone());
                return;
            }
            for ch in KEYS[(d[i] - b'0') as usize].chars() {
                buf.push(ch);
                bt(d, i + 1, buf, res);
                buf.pop();
            }
        }
        if digits.is_empty() {
            return vec![];
        }
        let mut res = Vec::new();
        bt(digits.as_bytes(), 0, &mut String::new(), &mut res);
        res
    }
}
```
**Complexity:** Time O(n · 4ⁿ) · Space O(n)

#### Approach 2: Iterative with `fold`
```rust
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        const KEYS: [&str; 10] = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        if digits.is_empty() {
            return vec![];
        }
        digits.bytes().fold(vec![String::new()], |acc, d| {
            acc.iter()
                .flat_map(|prefix| KEYS[(d - b'0') as usize].chars().map(move |c| format!("{}{}", prefix, c)))
                .collect()
        })
    }
}
```
**Complexity:** Time O(n · 4ⁿ) · Space O(n · 4ⁿ)

---

### 51. N-Queens
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/n-queens/)

**Problem:** Place `n` queens on an `n × n` board so that no two attack each other. Return every distinct board, written with `'Q'` for a queen and `'.'` for an empty square.

**Example:** `n = 4` → `[[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]`

**Key insight:** Place one queen per row. A column, a diagonal (`r-c`), or an anti-diagonal (`r+c`) can each hold at most one queen.

#### Approach 1: Backtracking with Sets (boolean arrays)
```rust
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn render(queens: &[usize]) -> Vec<String> {
            let n = queens.len();
            queens
                .iter()
                .map(|&q| (0..n).map(|c| if c == q { 'Q' } else { '.' }).collect())
                .collect()
        }
        #[allow(clippy::too_many_arguments)]
        fn bt(r: usize, n: usize, queens: &mut Vec<usize>, cols: &mut Vec<bool>, diag: &mut Vec<bool>, anti: &mut Vec<bool>, res: &mut Vec<Vec<String>>) {
            if r == n {
                res.push(render(queens));
                return;
            }
            for c in 0..n {
                let (d, a) = (r + n - c, r + c); // r - c shifted to stay non-negative
                if cols[c] || diag[d] || anti[a] {
                    continue;
                }
                cols[c] = true;
                diag[d] = true;
                anti[a] = true;
                queens.push(c);
                bt(r + 1, n, queens, cols, diag, anti, res);
                queens.pop();
                cols[c] = false;
                diag[d] = false;
                anti[a] = false;
            }
        }
        let n = n as usize;
        let mut res = Vec::new();
        bt(0, n, &mut Vec::new(), &mut vec![false; n], &mut vec![false; 2 * n], &mut vec![false; 2 * n], &mut res);
        res
    }
}
```
**Complexity:** Time O(n!) · Space O(n)

#### Approach 2: Bitmask Backtracking (fastest)
```rust
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn bt(r: usize, n: usize, cols: u32, d1: u32, d2: u32, queens: &mut Vec<usize>, res: &mut Vec<Vec<String>>) {
            if r == n {
                res.push(queens.iter().map(|&q| (0..n).map(|c| if c == q { 'Q' } else { '.' }).collect()).collect());
                return;
            }
            let full = (1u32 << n) - 1;
            let mut avail = full & !(cols | d1 | d2);
            while avail != 0 {
                let bit = avail & avail.wrapping_neg(); // lowest set bit
                avail &= avail - 1;
                queens.push(bit.trailing_zeros() as usize);
                bt(r + 1, n, cols | bit, ((d1 | bit) << 1) & full, (d2 | bit) >> 1, queens, res);
                queens.pop();
            }
        }
        let mut res = Vec::new();
        bt(0, n as usize, 0, 0, 0, &mut Vec::new(), &mut res);
        res
    }
}
```
**Complexity:** Time O(n!) with a much smaller constant · Space O(n)

---
## 11. Graphs

> **Grid neighbor idiom:** `for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)]`, then compute `nr = r as i32 + dr` and bounds-check before casting back to `usize`. Alternatively use `wrapping_add` with a `usize` delta and rely on the `< m` check to reject wrapped values.

### 200. Number of Islands
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/number-of-islands/)

**Problem:** Given an `m × n` grid of `'1'` (land) and `'0'` (water), count the islands. An island is a group of land cells connected horizontally or vertically.

**Example:** `[["1","1","0"],["0","1","0"],["0","0","1"]]` → `2`

**Key insight:** Each time you find unvisited land, count one island and flood-fill it so none of its cells are counted again.

#### Approach 1: DFS Flood Fill
```rust
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        fn sink(g: &mut Vec<Vec<char>>, r: usize, c: usize) {
            if r >= g.len() || c >= g[0].len() || g[r][c] != '1' {
                return;
            }
            g[r][c] = '0';
            sink(g, r + 1, c);
            sink(g, r, c + 1);
            if r > 0 { sink(g, r - 1, c); }
            if c > 0 { sink(g, r, c - 1); }
        }
        let mut count = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == '1' {
                    count += 1;
                    sink(&mut grid, r, c);
                }
            }
        }
        count
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n) recursion in the worst case

#### Approach 2: BFS
```rust
use std::collections::VecDeque;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut count = 0;
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] != '1' {
                    continue;
                }
                count += 1;
                grid[r][c] = '0';
                let mut q = VecDeque::from(vec![(r, c)]);
                while let Some((cr, cc)) = q.pop_front() {
                    for (dr, dc) in [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)] {
                        let (nr, nc) = (cr as i32 + dr, cc as i32 + dc);
                        if nr < 0 || nc < 0 || nr as usize >= m || nc as usize >= n {
                            continue;
                        }
                        let (nr, nc) = (nr as usize, nc as usize);
                        if grid[nr][nc] == '1' {
                            grid[nr][nc] = '0';
                            q.push_back((nr, nc));
                        }
                    }
                }
            }
        }
        count
    }
}
```
**Complexity:** Time O(m · n) · Space O(min(m, n)) for the queue

#### Approach 3: Union-Find
```rust
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        fn find(p: &mut Vec<usize>, mut x: usize) -> usize {
            while p[x] != x {
                p[x] = p[p[x]]; // path halving
                x = p[x];
            }
            x
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut parent: Vec<usize> = (0..m * n).collect();
        let mut count = grid.iter().flatten().filter(|&&c| c == '1').count() as i32;
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] != '1' {
                    continue;
                }
                for (nr, nc) in [(r + 1, c), (r, c + 1)] {
                    if nr < m && nc < n && grid[nr][nc] == '1' {
                        let (a, b) = (find(&mut parent, r * n + c), find(&mut parent, nr * n + nc));
                        if a != b {
                            parent[a] = b;
                            count -= 1;
                        }
                    }
                }
            }
        }
        count
    }
}
```
**Complexity:** Time O(m · n · α(mn)) · Space O(m · n)

---

### 133. Clone Graph
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/clone-graph/)

**Problem:** Given a reference to one node in a connected undirected graph, return a deep copy of the whole graph. Each node has a `val` and a list of `neighbors`.

**Key insight:** A map from original node to clone serves as the visited set and also handles cycles.

> No Rust template on LeetCode. The node holds `Rc<RefCell<…>>` neighbors, and the clone map is keyed by each original node's pointer address.

#### Approach 1: DFS + HashMap
```rust
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, neighbors: vec![] }))
    }
}

impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        fn dfs(n: &Rc<RefCell<Node>>, clones: &mut HashMap<*const RefCell<Node>, Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
            if let Some(c) = clones.get(&Rc::as_ptr(n)) {
                return Rc::clone(c);
            }
            let copy = Node::new(n.borrow().val);
            clones.insert(Rc::as_ptr(n), Rc::clone(&copy));
            let nbrs: Vec<Rc<RefCell<Node>>> = n.borrow().neighbors.clone();
            for nb in &nbrs {
                let c = dfs(nb, clones);
                copy.borrow_mut().neighbors.push(c);
            }
            copy
        }
        let mut clones = HashMap::new();
        node.map(|n| dfs(&n, &mut clones))
    }
}
```
**Complexity:** Time O(V + E) · Space O(V)

#### Approach 2: BFS + HashMap
```rust
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, neighbors: vec![] }))
    }
}

impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let start = node?;
        let mut clones: HashMap<*const RefCell<Node>, Rc<RefCell<Node>>> = HashMap::new();
        clones.insert(Rc::as_ptr(&start), Node::new(start.borrow().val));
        let mut q = VecDeque::from(vec![Rc::clone(&start)]);
        while let Some(cur) = q.pop_front() {
            let nbrs = cur.borrow().neighbors.clone();
            for nb in nbrs {
                let key = Rc::as_ptr(&nb);
                if !clones.contains_key(&key) {
                    clones.insert(key, Node::new(nb.borrow().val));
                    q.push_back(Rc::clone(&nb));
                }
                let nb_copy = Rc::clone(&clones[&key]);
                clones[&Rc::as_ptr(&cur)].borrow_mut().neighbors.push(nb_copy);
            }
        }
        Some(Rc::clone(&clones[&Rc::as_ptr(&start)]))
    }
}
```
**Complexity:** Time O(V + E) · Space O(V)

> Cycles of `Rc` leak memory when dropped (each node keeps the others alive). That's fine for a one-shot solution. For long-lived graphs, use `Weak` back-edges or an index-based arena (`Vec<Vec<usize>>`).

---

### 695. Max Area of Island
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/max-area-of-island/)

**Problem:** In a binary grid where `1` is land, return the size of the largest island (4-directionally connected), or `0` if there is no land.

#### Approach 1: DFS Returning Area
```rust
impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(g: &mut Vec<Vec<i32>>, r: usize, c: usize) -> i32 {
            if r >= g.len() || c >= g[0].len() || g[r][c] == 0 {
                return 0;
            }
            g[r][c] = 0;
            // wrapping_sub turns -1 into usize::MAX, which the bounds check rejects
            1 + dfs(g, r + 1, c) + dfs(g, r.wrapping_sub(1), c) + dfs(g, r, c + 1) + dfs(g, r, c.wrapping_sub(1))
        }
        let mut best = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                best = best.max(dfs(&mut grid, r, c));
            }
        }
        best
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Iterative DFS with a Stack
```rust
impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut best = 0;
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] == 0 {
                    continue;
                }
                grid[r][c] = 0;
                let mut stack = vec![(r, c)];
                let mut area = 0;
                while let Some((cr, cc)) = stack.pop() {
                    area += 1;
                    for (nr, nc) in [(cr + 1, cc), (cr.wrapping_sub(1), cc), (cr, cc + 1), (cr, cc.wrapping_sub(1))] {
                        if nr < m && nc < n && grid[nr][nc] == 1 {
                            grid[nr][nc] = 0;
                            stack.push((nr, nc));
                        }
                    }
                }
                best = best.max(area);
            }
        }
        best
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

---

### 417. Pacific Atlantic Water Flow
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/pacific-atlantic-water-flow/)

**Problem:** `heights` is an island. The Pacific Ocean touches its top and left edges, and the Atlantic touches its bottom and right edges. Water flows from a cell to a neighbor of equal or lower height. Return every cell from which water can reach both oceans.

**Key insight:** Search in reverse, from each ocean's border moving to cells of equal or greater height. The answer is the intersection of the two reachable sets.

#### Approach 1: Reverse DFS from Both Oceans (Optimal)
```rust
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(h: &Vec<Vec<i32>>, r: usize, c: usize, seen: &mut Vec<Vec<bool>>, prev: i32) {
            if r >= h.len() || c >= h[0].len() || seen[r][c] || h[r][c] < prev {
                return;
            }
            seen[r][c] = true;
            let v = h[r][c];
            dfs(h, r + 1, c, seen, v);
            dfs(h, r.wrapping_sub(1), c, seen, v);
            dfs(h, r, c + 1, seen, v);
            dfs(h, r, c.wrapping_sub(1), seen, v);
        }
        let (m, n) = (heights.len(), heights[0].len());
        let (mut pac, mut atl) = (vec![vec![false; n]; m], vec![vec![false; n]; m]);
        for c in 0..n {
            dfs(&heights, 0, c, &mut pac, 0);
            dfs(&heights, m - 1, c, &mut atl, 0);
        }
        for r in 0..m {
            dfs(&heights, r, 0, &mut pac, 0);
            dfs(&heights, r, n - 1, &mut atl, 0);
        }
        let mut res = Vec::new();
        for r in 0..m {
            for c in 0..n {
                if pac[r][c] && atl[r][c] {
                    res.push(vec![r as i32, c as i32]);
                }
            }
        }
        res
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Multi-Source BFS from Each Ocean
```rust
use std::collections::VecDeque;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (heights.len(), heights[0].len());
        let bfs = |starts: Vec<(usize, usize)>| -> Vec<Vec<bool>> {
            let mut seen = vec![vec![false; n]; m];
            let mut q = VecDeque::new();
            for (r, c) in starts {
                if !seen[r][c] {
                    seen[r][c] = true;
                    q.push_back((r, c));
                }
            }
            while let Some((r, c)) = q.pop_front() {
                for (nr, nc) in [(r + 1, c), (r.wrapping_sub(1), c), (r, c + 1), (r, c.wrapping_sub(1))] {
                    if nr < m && nc < n && !seen[nr][nc] && heights[nr][nc] >= heights[r][c] {
                        seen[nr][nc] = true;
                        q.push_back((nr, nc));
                    }
                }
            }
            seen
        };
        let pac = bfs((0..n).map(|c| (0, c)).chain((0..m).map(|r| (r, 0))).collect());
        let atl = bfs((0..n).map(|c| (m - 1, c)).chain((0..m).map(|r| (r, n - 1))).collect());
        let mut res = Vec::new();
        for r in 0..m {
            for c in 0..n {
                if pac[r][c] && atl[r][c] {
                    res.push(vec![r as i32, c as i32]);
                }
            }
        }
        res
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n). No deep recursion.

**Brute force:** DFS from every cell toward the oceans: O((m·n)²).

---

### 130. Surrounded Regions
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/surrounded-regions/)

**Problem:** In a board of `'X'` and `'O'`, flip every `'O'` region that is fully surrounded by `'X'` to `'X'`. Regions connected to the border stay as they are.

**Key insight:** Work backward. Mark every `O` reachable from the border as safe (`T`). Flip all remaining `O`s to `X`, then turn the `T`s back into `O`s.

#### Approach 1: Border DFS (Optimal)
```rust
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        fn mark(b: &mut Vec<Vec<char>>, r: usize, c: usize) {
            if r >= b.len() || c >= b[0].len() || b[r][c] != 'O' {
                return;
            }
            b[r][c] = 'T'; // safe: connected to the border
            mark(b, r + 1, c);
            mark(b, r.wrapping_sub(1), c);
            mark(b, r, c + 1);
            mark(b, r, c.wrapping_sub(1));
        }
        let (m, n) = (board.len(), board[0].len());
        for r in 0..m {
            mark(board, r, 0);
            mark(board, r, n - 1);
        }
        for c in 0..n {
            mark(board, 0, c);
            mark(board, m - 1, c);
        }
        for row in board.iter_mut() {
            for cell in row.iter_mut() {
                *cell = match *cell {
                    'O' => 'X',
                    'T' => 'O',
                    other => other,
                };
            }
        }
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n) recursion

#### Approach 2: Union-Find with a Virtual Border Node
```rust
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        fn find(p: &mut Vec<usize>, mut x: usize) -> usize {
            while p[x] != x {
                p[x] = p[p[x]];
                x = p[x];
            }
            x
        }
        let (m, n) = (board.len(), board[0].len());
        let border = m * n;
        let mut parent: Vec<usize> = (0..=m * n).collect();
        for r in 0..m {
            for c in 0..n {
                if board[r][c] != 'O' {
                    continue;
                }
                let id = r * n + c;
                let mut links = Vec::new();
                if r == 0 || c == 0 || r == m - 1 || c == n - 1 {
                    links.push(border);
                }
                if r + 1 < m && board[r + 1][c] == 'O' {
                    links.push(id + n);
                }
                if c + 1 < n && board[r][c + 1] == 'O' {
                    links.push(id + 1);
                }
                for other in links {
                    let (a, b) = (find(&mut parent, id), find(&mut parent, other));
                    parent[a] = b;
                }
            }
        }
        let root = find(&mut parent, border);
        for r in 0..m {
            for c in 0..n {
                if board[r][c] == 'O' && find(&mut parent, r * n + c) != root {
                    board[r][c] = 'X';
                }
            }
        }
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
```rust
use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut q = VecDeque::new();
        let mut fresh = 0;
        for r in 0..m {
            for c in 0..n {
                match grid[r][c] {
                    2 => q.push_back((r, c)),
                    1 => fresh += 1,
                    _ => {}
                }
            }
        }
        let mut minutes = 0;
        while !q.is_empty() && fresh > 0 {
            minutes += 1;
            for _ in 0..q.len() {
                let (r, c) = q.pop_front().unwrap();
                for (nr, nc) in [(r + 1, c), (r.wrapping_sub(1), c), (r, c + 1), (r, c.wrapping_sub(1))] {
                    if nr < m && nc < n && grid[nr][nc] == 1 {
                        grid[nr][nc] = 2;
                        fresh -= 1;
                        q.push_back((nr, nc));
                    }
                }
            }
        }
        if fresh > 0 { -1 } else { minutes }
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

---

### 286. Walls and Gates (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/walls-and-gates/) · Free mirror: LintCode 663

**Problem:** In a grid, `-1` is a wall, `0` is a gate, and `INF = 2147483647` is an empty room. Fill each empty room in place with its distance to the nearest gate, leaving `INF` where no gate is reachable.

**Key insight:** Run multi-source BFS from every gate. The first time BFS reaches a room, that distance is the shortest.

#### Approach 1: Multi-Source BFS (Optimal)
```rust
use std::collections::VecDeque;

impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        const INF: i32 = i32::MAX;
        let (m, n) = (rooms.len(), rooms[0].len());
        let mut q = VecDeque::new();
        for r in 0..m {
            for c in 0..n {
                if rooms[r][c] == 0 {
                    q.push_back((r, c));
                }
            }
        }
        while let Some((r, c)) = q.pop_front() {
            for (nr, nc) in [(r + 1, c), (r.wrapping_sub(1), c), (r, c + 1), (r, c.wrapping_sub(1))] {
                if nr < m && nc < n && rooms[nr][nc] == INF {
                    rooms[nr][nc] = rooms[r][c] + 1;
                    q.push_back((nr, nc));
                }
            }
        }
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: DFS from Each Gate (slower)
```rust
impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        fn dfs(rooms: &mut Vec<Vec<i32>>, r: usize, c: usize, d: i32) {
            if r >= rooms.len() || c >= rooms[0].len() || rooms[r][c] < d {
                return; // a wall (-1), or an already shorter distance
            }
            rooms[r][c] = d;
            dfs(rooms, r + 1, c, d + 1);
            dfs(rooms, r.wrapping_sub(1), c, d + 1);
            dfs(rooms, r, c + 1, d + 1);
            dfs(rooms, r, c.wrapping_sub(1), d + 1);
        }
        for r in 0..rooms.len() {
            for c in 0..rooms[0].len() {
                if rooms[r][c] == 0 {
                    dfs(rooms, r, c, 0);
                }
            }
        }
    }
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
```rust
use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut adj = vec![Vec::new(); n];
        let mut indeg = vec![0; n];
        for p in &prerequisites {
            adj[p[1] as usize].push(p[0] as usize);
            indeg[p[0] as usize] += 1;
        }
        let mut q: VecDeque<usize> = (0..n).filter(|&i| indeg[i] == 0).collect();
        let mut taken = 0;
        while let Some(c) = q.pop_front() {
            taken += 1;
            for &nxt in &adj[c] {
                indeg[nxt] -= 1;
                if indeg[nxt] == 0 {
                    q.push_back(nxt);
                }
            }
        }
        taken == n
    }
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

#### Approach 2: DFS Three-Color Cycle Detection
```rust
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        fn has_cycle(u: usize, adj: &Vec<Vec<usize>>, state: &mut Vec<u8>) -> bool {
            match state[u] {
                1 => return true, // back edge into the current path
                2 => return false,
                _ => {}
            }
            state[u] = 1;
            for &v in &adj[u] {
                if has_cycle(v, adj, state) {
                    return true;
                }
            }
            state[u] = 2;
            false
        }
        let n = num_courses as usize;
        let mut adj = vec![Vec::new(); n];
        for p in &prerequisites {
            adj[p[1] as usize].push(p[0] as usize);
        }
        let mut state = vec![0u8; n]; // 0 = unvisited, 1 = visiting, 2 = done
        !(0..n).any(|i| has_cycle(i, &adj, &mut state))
    }
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

---

### 210. Course Schedule II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/course-schedule-ii/)

**Problem:** Same setup as 207, but return a valid order in which to take the courses, or an empty array if it's impossible.

**Example:** `4, [[1,0],[2,0],[3,1],[3,2]]` → `[0,1,2,3]` or `[0,2,1,3]`

#### Approach 1: Kahn's Algorithm
```rust
use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut adj = vec![Vec::new(); n];
        let mut indeg = vec![0; n];
        for p in &prerequisites {
            adj[p[1] as usize].push(p[0] as usize);
            indeg[p[0] as usize] += 1;
        }
        let mut q: VecDeque<usize> = (0..n).filter(|&i| indeg[i] == 0).collect();
        let mut order = Vec::with_capacity(n);
        while let Some(c) = q.pop_front() {
            order.push(c as i32);
            for &nxt in &adj[c] {
                indeg[nxt] -= 1;
                if indeg[nxt] == 0 {
                    q.push_back(nxt);
                }
            }
        }
        if order.len() == n { order } else { vec![] }
    }
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

#### Approach 2: DFS Post-Order
```rust
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(u: usize, adj: &Vec<Vec<usize>>, state: &mut Vec<u8>, order: &mut Vec<i32>) -> bool {
            if state[u] == 1 {
                return false; // cycle
            }
            if state[u] == 2 {
                return true;
            }
            state[u] = 1;
            for &v in &adj[u] {
                if !dfs(v, adj, state, order) {
                    return false;
                }
            }
            state[u] = 2;
            order.push(u as i32); // prerequisites are recorded first
            true
        }
        let n = num_courses as usize;
        let mut adj = vec![Vec::new(); n];
        for p in &prerequisites {
            adj[p[0] as usize].push(p[1] as usize); // course → its prerequisites
        }
        let (mut state, mut order) = (vec![0u8; n], Vec::with_capacity(n));
        for i in 0..n {
            if !dfs(i, &adj, &mut state, &mut order) {
                return vec![];
            }
        }
        order
    }
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
```rust
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn find(p: &mut Vec<usize>, x: usize) -> usize {
            if p[x] != x {
                let root = find(p, p[x]);
                p[x] = root;
            }
            p[x]
        }
        let n = edges.len();
        let mut parent: Vec<usize> = (0..=n).collect();
        let mut rank = vec![0u8; n + 1];
        for e in &edges {
            let (mut a, mut b) = (find(&mut parent, e[0] as usize), find(&mut parent, e[1] as usize));
            if a == b {
                return e.clone(); // already connected, so this edge closes a cycle
            }
            if rank[a] < rank[b] {
                std::mem::swap(&mut a, &mut b);
            }
            parent[b] = a;
            if rank[a] == rank[b] {
                rank[a] += 1;
            }
        }
        vec![]
    }
}
```
**Complexity:** Time O(n · α(n)) · Space O(n)

#### Approach 2: DFS per Edge (check whether already connected)
```rust
use std::collections::HashSet;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn connected(adj: &Vec<Vec<usize>>, u: usize, target: usize, seen: &mut HashSet<usize>) -> bool {
            if u == target {
                return true;
            }
            seen.insert(u);
            adj[u].iter().any(|&v| !seen.contains(&v) && connected(adj, v, target, seen))
        }
        let mut adj = vec![Vec::new(); edges.len() + 1];
        for e in &edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            if connected(&adj, a, b, &mut HashSet::new()) {
                return e.clone();
            }
            adj[a].push(b);
            adj[b].push(a);
        }
        vec![]
    }
}
```
**Complexity:** Time O(n²) · Space O(n)

---

### 323. Number of Connected Components in an Undirected Graph (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/) · Free mirror: LintCode 3651

**Problem:** Given `n` nodes labeled `0..n-1` and a list of undirected `edges`, return the number of connected components.

**Example:** `n = 5, edges = [[0,1],[1,2],[3,4]]` → `2`

#### Approach 1: Union-Find (Optimal)
```rust
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        fn find(p: &mut Vec<usize>, mut x: usize) -> usize {
            while p[x] != x {
                p[x] = p[p[x]];
                x = p[x];
            }
            x
        }
        let mut parent: Vec<usize> = (0..n as usize).collect();
        let mut comps = n;
        for e in &edges {
            let (a, b) = (find(&mut parent, e[0] as usize), find(&mut parent, e[1] as usize));
            if a != b {
                parent[a] = b;
                comps -= 1;
            }
        }
        comps
    }
}
```
**Complexity:** Time O(V + E · α(V)) · Space O(V)

#### Approach 2: DFS
```rust
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        fn dfs(u: usize, adj: &Vec<Vec<usize>>, seen: &mut Vec<bool>) {
            seen[u] = true;
            for &v in &adj[u] {
                if !seen[v] {
                    dfs(v, adj, seen);
                }
            }
        }
        let n = n as usize;
        let mut adj = vec![Vec::new(); n];
        for e in &edges {
            adj[e[0] as usize].push(e[1] as usize);
            adj[e[1] as usize].push(e[0] as usize);
        }
        let mut seen = vec![false; n];
        let mut comps = 0;
        for i in 0..n {
            if !seen[i] {
                comps += 1;
                dfs(i, &adj, &mut seen);
            }
        }
        comps
    }
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

---

### 261. Graph Valid Tree (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/graph-valid-tree/) · Free mirror: LintCode 178

**Problem:** Given `n` nodes and a list of undirected `edges`, return whether the edges form a valid tree.

**Key insight:** A graph is a tree exactly when it has `n-1` edges and is connected (equivalently, `n-1` edges and no cycle).

#### Approach 1: Edge Count + Union-Find (Optimal)
```rust
impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        fn find(p: &mut Vec<usize>, mut x: usize) -> usize {
            while p[x] != x {
                p[x] = p[p[x]];
                x = p[x];
            }
            x
        }
        if edges.len() as i32 != n - 1 {
            return false;
        }
        let mut parent: Vec<usize> = (0..n as usize).collect();
        for e in &edges {
            let (a, b) = (find(&mut parent, e[0] as usize), find(&mut parent, e[1] as usize));
            if a == b {
                return false; // cycle
            }
            parent[a] = b;
        }
        true
    }
}
```
**Complexity:** Time O(n · α(n)) · Space O(n)

#### Approach 2: Edge Count + BFS Connectivity
```rust
use std::collections::VecDeque;

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        if edges.len() + 1 != n {
            return false;
        }
        let mut adj = vec![Vec::new(); n];
        for e in &edges {
            adj[e[0] as usize].push(e[1] as usize);
            adj[e[1] as usize].push(e[0] as usize);
        }
        let mut seen = vec![false; n];
        seen[0] = true;
        let mut q = VecDeque::from(vec![0usize]);
        let mut visited = 0;
        while let Some(u) = q.pop_front() {
            visited += 1;
            for &v in &adj[u] {
                if !seen[v] {
                    seen[v] = true;
                    q.push_back(v);
                }
            }
        }
        visited == n
    }
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

---

### 127. Word Ladder
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/word-ladder/)

**Problem:** Transform `beginWord` into `endWord` one letter at a time, where every intermediate word must be in `wordList`. Return the number of words in the shortest such sequence, counting both ends, or `0` if no sequence exists.

**Example:** `"hit" → "cog"`, `wordList = ["hot","dot","dog","lot","log","cog"]` → `5`

**Key insight:** BFS over words finds the shortest path. Generate neighbors by trying all 26 letters at each position (or by bucketing words under wildcard patterns like `h*t`).

#### Approach 1: BFS with Letter Substitution
```rust
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut dict: HashSet<Vec<u8>> = word_list.into_iter().map(String::into_bytes).collect();
        let end = end_word.into_bytes();
        if !dict.contains(&end) {
            return 0;
        }
        let begin = begin_word.into_bytes();
        dict.remove(&begin);
        let mut q = VecDeque::from(vec![begin]);
        let mut steps = 1;
        while !q.is_empty() {
            for _ in 0..q.len() {
                let mut w = q.pop_front().unwrap();
                for i in 0..w.len() {
                    let orig = w[i];
                    for c in b'a'..=b'z' {
                        if c == orig {
                            continue;
                        }
                        w[i] = c;
                        if w == end {
                            return steps + 1;
                        }
                        if dict.remove(&w) {
                            // remove() also marks it visited
                            q.push_back(w.clone());
                        }
                    }
                    w[i] = orig;
                }
            }
            steps += 1;
        }
        0
    }
}
```
**Complexity:** Time O(N · L · 26 · L), with hashing · Space O(N · L)

#### Approach 2: Bidirectional BFS (fastest in practice)
```rust
use std::collections::HashSet;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut dict: HashSet<Vec<u8>> = word_list.into_iter().map(String::into_bytes).collect();
        let (begin, end) = (begin_word.into_bytes(), end_word.into_bytes());
        if !dict.contains(&end) {
            return 0;
        }
        dict.remove(&begin);
        dict.remove(&end);
        let mut front: HashSet<Vec<u8>> = HashSet::from([begin]);
        let mut back: HashSet<Vec<u8>> = HashSet::from([end]);
        let mut steps = 1;
        while !front.is_empty() && !back.is_empty() {
            if front.len() > back.len() {
                std::mem::swap(&mut front, &mut back); // always expand the smaller side
            }
            let mut next = HashSet::new();
            for word in &front {
                let mut w = word.clone();
                for i in 0..w.len() {
                    let orig = w[i];
                    for c in b'a'..=b'z' {
                        w[i] = c;
                        if back.contains(&w) {
                            return steps + 1;
                        }
                        if dict.remove(&w) {
                            next.insert(w.clone());
                        }
                    }
                    w[i] = orig;
                }
            }
            front = next;
            steps += 1;
        }
        0
    }
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
Store each airport's destinations sorted in descending order, so `pop()` from the end yields the lexically smallest.
```rust
use std::collections::HashMap;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        fn visit(a: &str, adj: &mut HashMap<String, Vec<String>>, route: &mut Vec<String>) {
            while let Some(next) = adj.get_mut(a).and_then(|d| d.pop()) {
                visit(&next, adj, route);
            }
            route.push(a.to_string()); // post-order
        }
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        for t in tickets {
            adj.entry(t[0].clone()).or_default().push(t[1].clone());
        }
        for dests in adj.values_mut() {
            dests.sort_unstable_by(|a, b| b.cmp(a));
        }
        let mut route = Vec::new();
        visit("JFK", &mut adj, &mut route);
        route.reverse();
        route
    }
}
```
**Complexity:** Time O(E log E) · Space O(E)

#### Approach 2: Iterative Hierholzer with an Explicit Stack
```rust
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut adj: HashMap<String, BinaryHeap<Reverse<String>>> = HashMap::new(); // min-heap per airport
        for t in tickets {
            adj.entry(t[0].clone()).or_default().push(Reverse(t[1].clone()));
        }
        let mut stack = vec!["JFK".to_string()];
        let mut route = Vec::new();
        while let Some(top) = stack.last().cloned() {
            match adj.get_mut(&top).and_then(|h| h.pop()) {
                Some(Reverse(next)) => stack.push(next),
                None => route.push(stack.pop().unwrap()), // dead end: emit in post-order
            }
        }
        route.reverse();
        route
    }
}
```
**Complexity:** Time O(E log E) · Space O(E)

---

### 1584. Min Cost to Connect All Points
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/min-cost-to-connect-all-points/)

**Problem:** Given points on a 2D plane, connecting two points costs their Manhattan distance. Return the minimum total cost to connect every point (a minimum spanning tree).

**Example:** `[[0,0],[2,2],[3,10],[5,2],[7,0]]` → `20`

#### Approach 1: Prim's with an O(n²) Array (Optimal for dense graphs)
```rust
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut dist = vec![i32::MAX; n];
        let mut in_tree = vec![false; n];
        dist[0] = 0;
        let mut total = 0;
        for _ in 0..n {
            let u = (0..n).filter(|&i| !in_tree[i]).min_by_key(|&i| dist[i]).unwrap();
            in_tree[u] = true;
            total += dist[u];
            for v in 0..n {
                if !in_tree[v] {
                    let d = (points[u][0] - points[v][0]).abs() + (points[u][1] - points[v][1]).abs();
                    dist[v] = dist[v].min(d);
                }
            }
        }
        total
    }
}
```
**Complexity:** Time O(n²) · Space O(n)

#### Approach 2: Kruskal's with Union-Find
```rust
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        fn find(p: &mut Vec<usize>, mut x: usize) -> usize {
            while p[x] != x {
                p[x] = p[p[x]];
                x = p[x];
            }
            x
        }
        let n = points.len();
        let mut edges: Vec<(i32, usize, usize)> = Vec::with_capacity(n * n / 2);
        for i in 0..n {
            for j in i + 1..n {
                let w = (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                edges.push((w, i, j));
            }
        }
        edges.sort_unstable();
        let mut parent: Vec<usize> = (0..n).collect();
        let (mut total, mut used) = (0, 0);
        for (w, u, v) in edges {
            if used + 1 == n {
                break;
            }
            let (a, b) = (find(&mut parent, u), find(&mut parent, v));
            if a != b {
                parent[a] = b;
                total += w;
                used += 1;
            }
        }
        total
    }
}
```
**Complexity:** Time O(n² log n) · Space O(n²)

#### Approach 3: Prim's with a Min-Heap (lazy)
```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut in_tree = vec![false; n];
        let mut heap = BinaryHeap::from(vec![Reverse((0, 0usize))]); // (cost, node)
        let (mut total, mut added) = (0, 0);
        while added < n {
            let Reverse((cost, u)) = heap.pop().unwrap();
            if in_tree[u] {
                continue;
            }
            in_tree[u] = true;
            total += cost;
            added += 1;
            for v in 0..n {
                if !in_tree[v] {
                    let d = (points[u][0] - points[v][0]).abs() + (points[u][1] - points[v][1]).abs();
                    heap.push(Reverse((d, v)));
                }
            }
        }
        total
    }
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
```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut adj: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n + 1];
        for t in &times {
            adj[t[0] as usize].push((t[1] as usize, t[2]));
        }
        let mut dist = vec![i32::MAX; n + 1];
        dist[k as usize] = 0;
        let mut heap = BinaryHeap::from(vec![Reverse((0, k as usize))]);
        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] {
                continue; // stale entry
            }
            for &(v, w) in &adj[u] {
                let nd = d + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    heap.push(Reverse((nd, v)));
                }
            }
        }
        let worst = *dist[1..].iter().max().unwrap();
        if worst == i32::MAX { -1 } else { worst }
    }
}
```
**Complexity:** Time O(E log V) · Space O(V + E)

#### Approach 2: Bellman-Ford
```rust
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        const INF: i32 = i32::MAX / 2;
        let n = n as usize;
        let mut dist = vec![INF; n + 1];
        dist[k as usize] = 0;
        for _ in 1..n {
            let mut changed = false;
            for t in &times {
                let (u, v, w) = (t[0] as usize, t[1] as usize, t[2]);
                if dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let worst = *dist[1..].iter().max().unwrap();
        if worst == INF { -1 } else { worst }
    }
}
```
**Complexity:** Time O(V · E) · Space O(V)

#### Approach 3: Floyd–Warshall (n ≤ 100)
```rust
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        const INF: i32 = i32::MAX / 2;
        let n = n as usize;
        let mut d = vec![vec![INF; n + 1]; n + 1];
        for i in 1..=n {
            d[i][i] = 0;
        }
        for t in &times {
            d[t[0] as usize][t[1] as usize] = t[2];
        }
        for m in 1..=n {
            for i in 1..=n {
                for j in 1..=n {
                    if d[i][m] + d[m][j] < d[i][j] {
                        d[i][j] = d[i][m] + d[m][j];
                    }
                }
            }
        }
        let worst = *d[k as usize][1..].iter().max().unwrap();
        if worst == INF { -1 } else { worst }
    }
}
```
**Complexity:** Time O(V³) · Space O(V²)

---

### 778. Swim in Rising Water
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/swim-in-rising-water/)

**Problem:** In an `n × n` grid, `grid[i][j]` is the elevation. At time `t` you can swim between adjacent cells whose elevations are both ≤ `t`. Return the least time at which you can get from `(0,0)` to `(n-1,n-1)`.

**Key insight:** Minimize the maximum elevation along a path. Use Dijkstra where a path's cost is `max(cost so far, next cell)`, or binary search on `t` with a BFS check.

#### Approach 1: Dijkstra / Min-Heap on the Path Maximum (Optimal)
```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut seen = vec![vec![false; n]; n];
        let mut heap = BinaryHeap::from(vec![Reverse((grid[0][0], 0usize, 0usize))]);
        seen[0][0] = true;
        while let Some(Reverse((t, r, c))) = heap.pop() {
            if r == n - 1 && c == n - 1 {
                return t;
            }
            for (nr, nc) in [(r + 1, c), (r.wrapping_sub(1), c), (r, c + 1), (r, c.wrapping_sub(1))] {
                if nr < n && nc < n && !seen[nr][nc] {
                    seen[nr][nc] = true;
                    heap.push(Reverse((t.max(grid[nr][nc]), nr, nc)));
                }
            }
        }
        unreachable!()
    }
}
```
**Complexity:** Time O(n² log n) · Space O(n²)

#### Approach 2: Binary Search on Time + BFS
```rust
use std::collections::VecDeque;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let can_reach = |t: i32| -> bool {
            if grid[0][0] > t {
                return false;
            }
            let mut seen = vec![vec![false; n]; n];
            seen[0][0] = true;
            let mut q = VecDeque::from(vec![(0usize, 0usize)]);
            while let Some((r, c)) = q.pop_front() {
                if r == n - 1 && c == n - 1 {
                    return true;
                }
                for (nr, nc) in [(r + 1, c), (r.wrapping_sub(1), c), (r, c + 1), (r, c.wrapping_sub(1))] {
                    if nr < n && nc < n && !seen[nr][nc] && grid[nr][nc] <= t {
                        seen[nr][nc] = true;
                        q.push_back((nr, nc));
                    }
                }
            }
            false
        };
        let (mut lo, mut hi) = (grid[0][0], (n * n - 1) as i32);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if can_reach(mid) { hi = mid; } else { lo = mid + 1; }
        }
        lo
    }
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
```rust
use std::collections::{BTreeMap, BTreeSet, VecDeque};

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        // BTreeMap/BTreeSet make the output deterministic
        let mut adj: BTreeMap<u8, BTreeSet<u8>> = BTreeMap::new();
        let mut indeg: BTreeMap<u8, i32> = BTreeMap::new();
        for w in &words {
            for b in w.bytes() {
                adj.entry(b).or_default();
                indeg.entry(b).or_insert(0);
            }
        }
        for pair in words.windows(2) {
            let (a, b) = (pair[0].as_bytes(), pair[1].as_bytes());
            if a.len() > b.len() && a.starts_with(b) {
                return String::new(); // a word appears before its own prefix
            }
            if let Some((&x, &y)) = a.iter().zip(b).find(|(x, y)| x != y) {
                if adj.get_mut(&x).unwrap().insert(y) {
                    *indeg.get_mut(&y).unwrap() += 1;
                }
            }
        }
        let mut q: VecDeque<u8> = indeg.iter().filter(|(_, &d)| d == 0).map(|(&c, _)| c).collect();
        let mut out = Vec::new();
        while let Some(c) = q.pop_front() {
            out.push(c);
            for &nxt in &adj[&c] {
                let d = indeg.get_mut(&nxt).unwrap();
                *d -= 1;
                if *d == 0 {
                    q.push_back(nxt);
                }
            }
        }
        if out.len() == indeg.len() { String::from_utf8(out).unwrap() } else { String::new() } // shorter means a cycle
    }
}
```
**Complexity:** Time O(C), where C is the total number of characters · Space O(U + min(U², N)) for U unique letters

#### Approach 2: DFS Post-Order with Cycle Detection
```rust
use std::collections::HashMap;

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        fn dfs(c: u8, adj: &HashMap<u8, Vec<u8>>, state: &mut HashMap<u8, u8>, out: &mut Vec<u8>) -> bool {
            match state.get(&c) {
                Some(1) => return false, // cycle
                Some(2) => return true,
                _ => {}
            }
            state.insert(c, 1);
            for &nx in &adj[&c] {
                if !dfs(nx, adj, state, out) {
                    return false;
                }
            }
            state.insert(c, 2);
            out.push(c);
            true
        }
        let mut adj: HashMap<u8, Vec<u8>> = HashMap::new();
        for w in &words {
            for b in w.bytes() {
                adj.entry(b).or_default();
            }
        }
        for pair in words.windows(2) {
            let (a, b) = (pair[0].as_bytes(), pair[1].as_bytes());
            if a.len() > b.len() && a.starts_with(b) {
                return String::new();
            }
            if let Some((&x, &y)) = a.iter().zip(b).find(|(x, y)| x != y) {
                adj.get_mut(&x).unwrap().push(y);
            }
        }
        let (mut state, mut out) = (HashMap::new(), Vec::new());
        let letters: Vec<u8> = adj.keys().copied().collect();
        for c in letters {
            if !dfs(c, &adj, &mut state, &mut out) {
                return String::new();
            }
        }
        out.reverse();
        String::from_utf8(out).unwrap()
    }
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
```rust
impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        const INF: i32 = i32::MAX;
        let mut prices = vec![INF; n as usize];
        prices[src as usize] = 0;
        for _ in 0..=k {
            let mut tmp = prices.clone(); // relax from the previous round only
            for f in &flights {
                let (u, v, w) = (f[0] as usize, f[1] as usize, f[2]);
                if prices[u] != INF && prices[u] + w < tmp[v] {
                    tmp[v] = prices[u] + w;
                }
            }
            prices = tmp;
        }
        if prices[dst as usize] == INF { -1 } else { prices[dst as usize] }
    }
}
```
**Complexity:** Time O(k · E) · Space O(n)

#### Approach 2: BFS by Number of Stops
```rust
use std::collections::VecDeque;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut adj: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n];
        for f in &flights {
            adj[f[0] as usize].push((f[1] as usize, f[2]));
        }
        let mut best = vec![i32::MAX; n];
        best[src as usize] = 0;
        let mut q = VecDeque::from(vec![(src as usize, 0)]); // (node, cost)
        let mut stops = 0;
        while !q.is_empty() && stops <= k {
            for _ in 0..q.len() {
                let (u, cost) = q.pop_front().unwrap();
                for &(v, w) in &adj[u] {
                    if cost + w < best[v] {
                        best[v] = cost + w;
                        q.push_back((v, cost + w));
                    }
                }
            }
            stops += 1;
        }
        if best[dst as usize] == i32::MAX { -1 } else { best[dst as usize] }
    }
}
```
**Complexity:** Time O(k · E) · Space O(n + E)

---
## 13. 1-D Dynamic Programming

> **Progression for each problem:** recursion → memoization (top-down) → tabulation (bottom-up) → space-optimized, where that helps. For memoized recursion in Rust, pass the memo as `&mut Vec<…>` or `&mut HashMap<…>` into a nested `fn`. Closures can't easily call themselves recursively.

### 70. Climbing Stairs
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/climbing-stairs/)

**Problem:** You can climb 1 or 2 steps at a time. How many distinct ways are there to reach the top of `n` stairs?

**Example:** `n = 3` → `3` (1+1+1, 1+2, 2+1)

**Key insight:** `ways(n) = ways(n-1) + ways(n-2)`, the Fibonacci recurrence.

#### Approach 1: Memoized Recursion
```rust
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fn f(i: usize, memo: &mut Vec<i32>) -> i32 {
            if i <= 2 {
                return i as i32;
            }
            if memo[i] == 0 {
                memo[i] = f(i - 1, memo) + f(i - 2, memo);
            }
            memo[i]
        }
        f(n as usize, &mut vec![0; n as usize + 1])
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Bottom-Up, O(1) Space (Optimal)
```rust
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (1, 1); // ways(0), ways(1)
        for _ in 2..=n {
            let c = a + b;
            a = b;
            b = c;
        }
        b
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 3: Matrix Exponentiation
```rust
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        type M = [[i64; 2]; 2];
        fn mul(x: &M, y: &M) -> M {
            [
                [x[0][0] * y[0][0] + x[0][1] * y[1][0], x[0][0] * y[0][1] + x[0][1] * y[1][1]],
                [x[1][0] * y[0][0] + x[1][1] * y[1][0], x[1][0] * y[0][1] + x[1][1] * y[1][1]],
            ]
        }
        let (mut res, mut base): (M, M) = ([[1, 0], [0, 1]], [[1, 1], [1, 0]]);
        let mut p = n;
        while p > 0 {
            if p & 1 == 1 {
                res = mul(&res, &base);
            }
            base = mul(&base, &base);
            p >>= 1;
        }
        res[0][0] as i32 // F(n+1)
    }
}
```
**Complexity:** Time O(log n) · Space O(1)

---

### 746. Min Cost Climbing Stairs
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/min-cost-climbing-stairs/)

**Problem:** `cost[i]` is the price of stepping on stair `i`. After paying, you can climb 1 or 2 steps. You may start on stair 0 or stair 1. Return the minimum cost to get past the last stair.

**Example:** `[10,15,20]` → `15`

#### Approach 1: Tabulation
```rust
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp = vec![0; n + 1]; // dp[i] = min cost to reach stair i
        for i in 2..=n {
            dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
        }
        dp[n]
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: O(1) Space with `fold` (Optimal)
```rust
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        // (cost to reach i-1, cost to reach i), sliding along the stairs
        let (a, b) = cost.iter().fold((0, 0), |(a, b), &c| (b, a.min(b) + c));
        a.min(b)
    }
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
```rust
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn f(nums: &[i32], i: i32, memo: &mut Vec<i32>) -> i32 {
            if i < 0 {
                return 0;
            }
            let iu = i as usize;
            if memo[iu] < 0 {
                memo[iu] = f(nums, i - 1, memo).max(f(nums, i - 2, memo) + nums[iu]);
            }
            memo[iu]
        }
        f(&nums, nums.len() as i32 - 1, &mut vec![-1; nums.len()])
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: O(1) Space (Optimal)
```rust
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut prev2, mut prev1) = (0, 0);
        for n in nums {
            let cur = prev1.max(prev2 + n);
            prev2 = prev1;
            prev1 = cur;
        }
        prev1
    }
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
```rust
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn rob_line(h: &[i32]) -> i32 {
            h.iter().fold((0, 0), |(a, b), &n| (b, b.max(a + n))).1
        }
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        rob_line(&nums[..n - 1]).max(rob_line(&nums[1..]))
    }
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
```rust
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();
        let (mut start, mut best) = (0usize, 0usize);
        // returns (start, len) of the widest palindrome around center (l, r)
        let expand = |mut l: i32, mut r: i32| -> (usize, usize) {
            while l >= 0 && (r as usize) < n && b[l as usize] == b[r as usize] {
                l -= 1;
                r += 1;
            }
            ((l + 1) as usize, (r - l - 1) as usize)
        };
        for i in 0..n as i32 {
            for (st, len) in [expand(i, i), expand(i, i + 1)] {
                if len > best {
                    start = st;
                    best = len;
                }
            }
        }
        s[start..start + best].to_string()
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: DP Table
```rust
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let b = s.as_bytes();
        let n = b.len();
        let mut dp = vec![vec![false; n]; n];
        let (mut start, mut best) = (0, 1);
        for i in (0..n).rev() {
            for j in i..n {
                if b[i] == b[j] && (j - i < 3 || dp[i + 1][j - 1]) {
                    dp[i][j] = true;
                    if j - i + 1 > best {
                        start = i;
                        best = j - i + 1;
                    }
                }
            }
        }
        s[start..start + best].to_string()
    }
}
```
**Complexity:** Time O(n²) · Space O(n²)

#### Approach 3: Manacher's Algorithm (theoretically optimal)
```rust
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // transform "abc" into "^#a#b#c#$" so every palindrome has odd length
        let mut t = vec![b'^'];
        for &c in s.as_bytes() {
            t.push(b'#');
            t.push(c);
        }
        t.push(b'#');
        t.push(b'$');
        let mut p = vec![0usize; t.len()];
        let (mut center, mut right) = (0usize, 0usize);
        for i in 1..t.len() - 1 {
            if i < right {
                p[i] = (right - i).min(p[2 * center - i]);
            }
            while t[i + p[i] + 1] == t[i - p[i] - 1] {
                p[i] += 1;
            }
            if i + p[i] > right {
                center = i;
                right = i + p[i];
            }
        }
        let (ci, &max_len) = p.iter().enumerate().max_by_key(|&(i, v)| (*v, std::cmp::Reverse(i))).unwrap();
        let begin = (ci - max_len) / 2;
        s[begin..begin + max_len].to_string()
    }
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 647. Palindromic Substrings
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/palindromic-substrings/)

**Problem:** Count the substrings of `s` that are palindromes. Substrings at different positions count separately.

**Example:** `"aaa"` → `6`

#### Approach 1: Expand Around the Center (Optimal in practice)
```rust
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let b = s.as_bytes();
        let n = b.len() as i32;
        let mut count = 0;
        for c in 0..2 * n - 1 {
            let (mut l, mut r) = (c / 2, c / 2 + c % 2);
            while l >= 0 && r < n && b[l as usize] == b[r as usize] {
                count += 1;
                l -= 1;
                r += 1;
            }
        }
        count
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: DP Table
```rust
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let b = s.as_bytes();
        let n = b.len();
        let mut dp = vec![vec![false; n]; n];
        let mut count = 0;
        for i in (0..n).rev() {
            for j in i..n {
                if b[i] == b[j] && (j - i < 3 || dp[i + 1][j - 1]) {
                    dp[i][j] = true;
                    count += 1;
                }
            }
        }
        count
    }
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
```rust
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let b = s.as_bytes();
        let n = b.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 1..=n {
            if b[i - 1] != b'0' {
                dp[i] += dp[i - 1];
            }
            if i >= 2 {
                let two = (b[i - 2] - b'0') * 10 + (b[i - 1] - b'0');
                if (10..=26).contains(&two) {
                    dp[i] += dp[i - 2];
                }
            }
        }
        dp[n]
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: O(1) Space (Optimal)
```rust
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let b = s.as_bytes();
        let (mut prev2, mut prev1) = (0, 1); // dp[i-2], dp[i-1]
        for i in 0..b.len() {
            let mut cur = if b[i] != b'0' { prev1 } else { 0 };
            if i > 0 && (b[i - 1] == b'1' || (b[i - 1] == b'2' && b[i] <= b'6')) {
                cur += prev2;
            }
            prev2 = prev1;
            prev1 = cur;
        }
        prev1
    }
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 322. Coin Change
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/coin-change/)

**Problem:** Given coin denominations and an `amount`, return the fewest coins that add up to `amount`, or `-1` if it can't be made. You have unlimited coins of each kind.

**Example:** `coins = [1,2,5], amount = 11` → `3` (5+5+1)

**Key insight:** `dp[a] = 1 + min(dp[a - c])` over every coin `c`. (Greedy fails for some coin sets.)

#### Approach 1: Bottom-Up DP (Optimal)
```rust
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amt = amount as usize;
        let mut dp = vec![amount + 1; amt + 1]; // amount+1 stands in for infinity
        dp[0] = 0;
        for a in 1..=amt {
            for &c in &coins {
                let c = c as usize;
                if c <= a {
                    dp[a] = dp[a].min(dp[a - c] + 1);
                }
            }
        }
        if dp[amt] > amount { -1 } else { dp[amt] }
    }
}
```
**Complexity:** Time O(amount · len(coins)) · Space O(amount)

#### Approach 2: BFS over Amounts (shortest path)
```rust
use std::collections::VecDeque;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let mut seen = vec![false; amount as usize + 1];
        let mut q = VecDeque::from(vec![0i64]);
        seen[0] = true;
        let mut steps = 0;
        while !q.is_empty() {
            steps += 1;
            for _ in 0..q.len() {
                let cur = q.pop_front().unwrap();
                for &c in &coins {
                    let v = cur + c as i64; // i64: coins can be up to 2^31 - 1
                    if v == amount as i64 {
                        return steps;
                    }
                    if v < amount as i64 && !seen[v as usize] {
                        seen[v as usize] = true;
                        q.push_back(v);
                    }
                }
            }
        }
        -1
    }
}
```
**Complexity:** Time O(amount · len(coins)) · Space O(amount)

#### Approach 3: Top-Down Memoization
```rust
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // memo: None = not computed, Some(-1) = impossible
        fn f(coins: &[i32], a: i32, memo: &mut Vec<Option<i32>>) -> i32 {
            if a == 0 {
                return 0;
            }
            if let Some(v) = memo[a as usize] {
                return v;
            }
            let best = coins
                .iter()
                .filter(|&&c| c <= a)
                .map(|&c| f(coins, a - c, memo))
                .filter(|&r| r >= 0)
                .min()
                .map_or(-1, |r| r + 1);
            memo[a as usize] = Some(best);
            best
        }
        f(&coins, amount, &mut vec![None; amount as usize + 1])
    }
}
```
**Complexity:** Time O(amount · len(coins)) · Space O(amount)

---

### 152. Maximum Product Subarray
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/maximum-product-subarray/)

**Problem:** Return the largest product of any contiguous non-empty subarray.

**Example:** `[2,3,-2,4]` → `6`; `[-2,0,-1]` → `0`

**Key insight:** A negative number turns the minimum product into the maximum, so track both the current max and the current min.

#### Approach 1: Track Max and Min (Optimal)
```rust
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut best, mut cur_max, mut cur_min) = (nums[0], nums[0], nums[0]);
        for &n in &nums[1..] {
            if n < 0 {
                std::mem::swap(&mut cur_max, &mut cur_min); // a negative flips max and min
            }
            cur_max = n.max(cur_max * n);
            cur_min = n.min(cur_min * n);
            best = best.max(cur_max);
        }
        best
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Prefix and Suffix Products
The best subarray always starts at an edge of the array or of a zero-bounded segment.
```rust
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut best, mut pre, mut suf) = (nums[0], 1, 1);
        for i in 0..n {
            pre = if pre == 0 { 1 } else { pre } * nums[i];
            suf = if suf == 0 { 1 } else { suf } * nums[n - 1 - i];
            best = best.max(pre).max(suf);
        }
        best
    }
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 139. Word Break
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/word-break/)

**Problem:** Return whether `s` can be split into a sequence of one or more words from `wordDict`. Words can be reused.

**Example:** `s = "leetcode", wordDict = ["leet","code"]` → `true`

**Key insight:** `dp[i]` is true when some `j < i` has `dp[j]` true and `s[j:i]` in the dictionary.

#### Approach 1: Bottom-Up DP with a HashSet (Optimal)
```rust
use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let max_len = word_dict.iter().map(|w| w.len()).max().unwrap_or(0);
        let dict: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for i in 1..=n {
            for j in (i.saturating_sub(max_len)..i).rev() {
                if dp[j] && dict.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        dp[n]
    }
}
```
**Complexity:** Time O(n · L), where L is the longest word (substring hashing adds O(L) per check) · Space O(n)

#### Approach 2: DP Iterating over Dictionary Words
```rust
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[n] = true;
        for i in (0..n).rev() {
            dp[i] = word_dict.iter().any(|w| i + w.len() <= n && dp[i + w.len()] && s[i..].starts_with(w.as_str()));
        }
        dp[0]
    }
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
```rust
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        *dp.iter().max().unwrap()
    }
}
```
**Complexity:** Time O(n²) · Space O(n)

#### Approach 2: Binary Search on Tails (Optimal)
```rust
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails: Vec<i32> = Vec::new();
        for n in nums {
            let i = tails.partition_point(|&t| t < n); // first index with tails[i] >= n
            if i == tails.len() {
                tails.push(n);
            } else {
                tails[i] = n;
            }
        }
        tails.len() as i32
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 3: Patience Sorting with a BTreeSet
```rust
use std::collections::BTreeSet;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails = BTreeSet::new();
        for n in nums {
            if let Some(&ceil) = tails.range(n..).next() {
                tails.remove(&ceil); // replace the smallest tail >= n
            }
            tails.insert(n);
        }
        tails.len() as i32
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

---

### 416. Partition Equal Subset Sum
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/partition-equal-subset-sum/)

**Problem:** Return whether `nums` can be split into two subsets with equal sums.

**Example:** `[1,5,11,5]` → `true` (`[1,5,5]` and `[11]`)

**Key insight:** This is 0/1 knapsack: can some subset reach `total/2`? Iterate the sums from high to low so each number is used at most once.

#### Approach 1: 1-D Boolean DP (Optimal)
```rust
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();
        if total % 2 == 1 {
            return false;
        }
        let target = (total / 2) as usize;
        let mut dp = vec![false; target + 1];
        dp[0] = true;
        for &n in &nums {
            let n = n as usize;
            for s in (n..=target).rev() {
                // high to low, so each number is used once
                dp[s] = dp[s] || dp[s - n];
            }
        }
        dp[target]
    }
}
```
**Complexity:** Time O(n · target) · Space O(target)

#### Approach 2: Bitset DP with `u64` Words (fast)
```rust
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();
        if total % 2 == 1 {
            return false;
        }
        let target = (total / 2) as usize;
        let words = target / 64 + 1;
        let mut bits = vec![0u64; words]; // bit s set means sum s is reachable
        bits[0] = 1;
        for &n in &nums {
            let (ws, bs) = (n as usize / 64, n as usize % 64);
            // bits |= bits << n, processed from the top word down
            for i in (0..words).rev() {
                let mut v = 0u64;
                if i >= ws {
                    v |= bits[i - ws] << bs;
                    if bs > 0 && i > ws {
                        v |= bits[i - ws - 1] >> (64 - bs);
                    }
                }
                bits[i] |= v;
            }
        }
        bits[target / 64] >> (target % 64) & 1 == 1
    }
}
```
**Complexity:** Time O(n · target / 64) · Space O(target / 64)

#### Approach 3: HashSet of Reachable Sums
```rust
use std::collections::HashSet;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();
        if total % 2 == 1 {
            return false;
        }
        let target = total / 2;
        let mut reach: HashSet<i32> = HashSet::from([0]);
        for n in nums {
            let next: Vec<i32> = reach.iter().map(|&s| s + n).filter(|&s| s <= target).collect();
            reach.extend(next);
            if reach.contains(&target) {
                return true;
            }
        }
        reach.contains(&target)
    }
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
```rust
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![1; n]; m]; // the first row and column are all 1
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dp[m - 1][n - 1]
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D Rolling Row (Optimal DP)
```rust
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut row = vec![1; n as usize];
        for _ in 1..m {
            for j in 1..n as usize {
                row[j] += row[j - 1];
            }
        }
        row[n as usize - 1]
    }
}
```
**Complexity:** Time O(m · n) · Space O(n)

#### Approach 3: Combinatorics C(m+n-2, m-1)
```rust
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (k, total) = ((m.min(n) - 1) as i64, (m + n - 2) as i64);
        let mut res: i64 = 1;
        for i in 1..=k {
            res = res * (total - k + i) / i; // stays an integer at every step
        }
        res as i32
    }
}
```
**Complexity:** Time O(min(m, n)) · Space O(1)

---

### 1143. Longest Common Subsequence
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-common-subsequence/)

**Problem:** Return the length of the longest subsequence that both `text1` and `text2` contain.

**Example:** `"abcde", "ace"` → `3`

**Key insight:** If `a[i] == b[j]`, then `dp[i][j] = 1 + dp[i-1][j-1]`. Otherwise `dp[i][j] = max(dp[i-1][j], dp[i][j-1])`.

#### Approach 1: 2-D Table
```rust
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (a, b) = (text1.as_bytes(), text2.as_bytes());
        let (m, n) = (a.len(), b.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = if a[i - 1] == b[j - 1] {
                    dp[i - 1][j - 1] + 1
                } else {
                    dp[i - 1][j].max(dp[i][j - 1])
                };
            }
        }
        dp[m][n]
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Two Rows (Optimal space)
```rust
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (a, b) = if text1.len() >= text2.len() { (text1, text2) } else { (text2, text1) };
        let (a, b) = (a.as_bytes(), b.as_bytes());
        let n = b.len();
        let (mut prev, mut cur) = (vec![0; n + 1], vec![0; n + 1]);
        for i in 1..=a.len() {
            for j in 1..=n {
                cur[j] = if a[i - 1] == b[j - 1] { prev[j - 1] + 1 } else { prev[j].max(cur[j - 1]) };
            }
            std::mem::swap(&mut prev, &mut cur);
        }
        prev[n]
    }
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
```rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut hold, mut sold, mut rest) = (i32::MIN / 2, 0, 0);
        for p in prices {
            let prev_sold = sold;
            sold = hold + p;
            hold = hold.max(rest - p);
            rest = rest.max(prev_sold);
        }
        sold.max(rest)
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Memoized DFS (index, can_buy)
```rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        fn dfs(p: &[i32], i: usize, buying: usize, memo: &mut Vec<[Option<i32>; 2]>) -> i32 {
            if i >= p.len() {
                return 0;
            }
            if let Some(v) = memo[i][buying] {
                return v;
            }
            let cooldown = dfs(p, i + 1, buying, memo);
            let act = if buying == 1 {
                dfs(p, i + 1, 0, memo) - p[i]
            } else {
                dfs(p, i + 2, 1, memo) + p[i] // selling forces a one-day cooldown
            };
            let best = cooldown.max(act);
            memo[i][buying] = Some(best);
            best
        }
        dfs(&prices, 0, 1, &mut vec![[None; 2]; prices.len()])
    }
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
```rust
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amt = amount as usize;
        let mut dp = vec![0u64; amt + 1]; // wrapping u64: intermediate counts may overflow, the answer fits i32
        dp[0] = 1;
        for &c in &coins {
            // coins in the outer loop: combinations, not permutations
            for a in c as usize..=amt {
                dp[a] = dp[a].wrapping_add(dp[a - c as usize]);
            }
        }
        dp[amt] as i32
    }
}
```
**Complexity:** Time O(n · amount) · Space O(amount)

#### Approach 2: 2-D DP (clearer state)
```rust
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let (n, amt) = (coins.len(), amount as usize);
        let mut dp = vec![vec![0u64; amt + 1]; n + 1]; // dp[i][a] = ways using the first i coins
        for row in dp.iter_mut() {
            row[0] = 1;
        }
        for i in 1..=n {
            let c = coins[i - 1] as usize;
            for a in 1..=amt {
                dp[i][a] = dp[i - 1][a].wrapping_add(if a >= c { dp[i][a - c] } else { 0 });
            }
        }
        dp[n][amt] as i32
    }
}
```
**Complexity:** Time O(n · amount) · Space O(n · amount)

---

### 494. Target Sum
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/target-sum/)

**Problem:** Put a `+` or `-` in front of every number in `nums`. Count the ways the resulting expression equals `target`.

**Example:** `nums = [1,1,1,1,1], target = 3` → `5`

**Key insight:** If `P` is the set of numbers with `+`, then `sum(P) = (total + target) / 2`. That reduces the problem to counting subsets with a given sum.

#### Approach 1: Memoized DFS (index, running sum)
```rust
use std::collections::HashMap;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        fn dfs(nums: &[i32], i: usize, sum: i32, target: i32, memo: &mut HashMap<(usize, i32), i32>) -> i32 {
            if i == nums.len() {
                return (sum == target) as i32;
            }
            if let Some(&v) = memo.get(&(i, sum)) {
                return v;
            }
            let v = dfs(nums, i + 1, sum + nums[i], target, memo) + dfs(nums, i + 1, sum - nums[i], target, memo);
            memo.insert((i, sum), v);
            v
        }
        dfs(&nums, 0, 0, target, &mut HashMap::new())
    }
}
```
**Complexity:** Time O(n · total) · Space O(n · total)

#### Approach 2: Subset-Sum Counting (Optimal)
```rust
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let total: i32 = nums.iter().sum();
        if target.abs() > total || (total + target) % 2 != 0 {
            return 0;
        }
        let p = ((total + target) / 2) as usize; // the sum of the numbers that get '+'
        let mut dp = vec![0; p + 1];
        dp[0] = 1;
        for &n in &nums {
            let n = n as usize;
            for s in (n..=p).rev() {
                dp[s] += dp[s - n];
            }
        }
        dp[p]
    }
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
```rust
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (a, b, c) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let (m, n) = (a.len(), b.len());
        if m + n != c.len() {
            return false;
        }
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        for i in 0..=m {
            for j in 0..=n {
                if i > 0 && a[i - 1] == c[i + j - 1] && dp[i - 1][j] {
                    dp[i][j] = true;
                }
                if j > 0 && b[j - 1] == c[i + j - 1] && dp[i][j - 1] {
                    dp[i][j] = true;
                }
            }
        }
        dp[m][n]
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D DP (Optimal space)
```rust
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (a, b, c) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let (m, n) = (a.len(), b.len());
        if m + n != c.len() {
            return false;
        }
        let mut dp = vec![false; n + 1];
        for i in 0..=m {
            for j in 0..=n {
                dp[j] = match (i, j) {
                    (0, 0) => true,
                    (0, _) => dp[j - 1] && b[j - 1] == c[j - 1],
                    (_, 0) => dp[j] && a[i - 1] == c[i - 1],
                    _ => (dp[j] && a[i - 1] == c[i + j - 1]) || (dp[j - 1] && b[j - 1] == c[i + j - 1]),
                };
            }
        }
        dp[n]
    }
}
```
**Complexity:** Time O(m · n) · Space O(n)

---

### 329. Longest Increasing Path in a Matrix
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/longest-increasing-path-in-a-matrix/)

**Problem:** Return the length of the longest strictly increasing path in a matrix, moving in four directions.

**Example:** `[[9,9,4],[6,6,8],[2,1,1]]` → `4` (1→2→6→9)

**Key insight:** Strictly increasing paths can't form cycles, so the cells form a DAG and DFS with memoization works.

#### Approach 1: DFS + Memoization (Optimal)
```rust
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        fn dfs(g: &Vec<Vec<i32>>, r: usize, c: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if memo[r][c] != 0 {
                return memo[r][c];
            }
            let mut best = 1;
            for (nr, nc) in [(r + 1, c), (r.wrapping_sub(1), c), (r, c + 1), (r, c.wrapping_sub(1))] {
                if nr < g.len() && nc < g[0].len() && g[nr][nc] > g[r][c] {
                    best = best.max(1 + dfs(g, nr, nc, memo));
                }
            }
            memo[r][c] = best;
            best
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut memo = vec![vec![0; n]; m];
        let mut ans = 0;
        for r in 0..m {
            for c in 0..n {
                ans = ans.max(dfs(&matrix, r, c, &mut memo));
            }
        }
        ans
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Topological Sort (peel layers of local maxima)
```rust
use std::collections::VecDeque;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let nbrs = |r: usize, c: usize| {
            [(r + 1, c), (r.wrapping_sub(1), c), (r, c + 1), (r, c.wrapping_sub(1))]
                .into_iter()
                .filter(move |&(nr, nc)| nr < m && nc < n)
        };
        let mut outdeg = vec![vec![0; n]; m];
        let mut q = VecDeque::new();
        for r in 0..m {
            for c in 0..n {
                outdeg[r][c] = nbrs(r, c).filter(|&(nr, nc)| matrix[nr][nc] > matrix[r][c]).count();
                if outdeg[r][c] == 0 {
                    q.push_back((r, c));
                }
            }
        }
        let mut layers = 0;
        while !q.is_empty() {
            layers += 1;
            for _ in 0..q.len() {
                let (r, c) = q.pop_front().unwrap();
                for (nr, nc) in nbrs(r, c) {
                    if matrix[nr][nc] < matrix[r][c] {
                        outdeg[nr][nc] -= 1;
                        if outdeg[nr][nc] == 0 {
                            q.push_back((nr, nc));
                        }
                    }
                }
            }
        }
        layers
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

---

### 115. Distinct Subsequences
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/distinct-subsequences/)

**Problem:** Count the distinct subsequences of `s` that equal `t`.

**Example:** `s = "rabbbit", t = "rabbit"` → `3`

**Key insight:** `dp[i][j] = dp[i-1][j]` (skip `s[i-1]`), plus `dp[i-1][j-1]` when `s[i-1] == t[j-1]`.

#### Approach 1: 2-D DP
```rust
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (a, b) = (s.as_bytes(), t.as_bytes());
        let (m, n) = (a.len(), b.len());
        let mut dp = vec![vec![0u64; n + 1]; m + 1]; // wrapping u64: intermediates can overflow
        for row in dp.iter_mut() {
            row[0] = 1; // the empty t matches once
        }
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = dp[i - 1][j].wrapping_add(if a[i - 1] == b[j - 1] { dp[i - 1][j - 1] } else { 0 });
            }
        }
        dp[m][n] as i32
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D DP, iterate j backward (Optimal space)
```rust
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let b = t.as_bytes();
        let n = b.len();
        let mut dp = vec![0u64; n + 1];
        dp[0] = 1;
        for &ch in s.as_bytes() {
            for j in (1..=n).rev() {
                if ch == b[j - 1] {
                    dp[j] = dp[j].wrapping_add(dp[j - 1]);
                }
            }
        }
        dp[n] as i32
    }
}
```
**Complexity:** Time O(m · n) · Space O(n)

> On some LeetCode tests the intermediate counts overflow even `u64`, though the final answer fits in 32 bits. Wrapping addition computes the answer modulo 2⁶⁴, and the true answer is below 2³¹, so `as i32` recovers it exactly. A plain `+` would panic in debug builds. The same trick is used in #518.

---

### 72. Edit Distance
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/edit-distance/)

**Problem:** Return the minimum number of insert, delete, and replace operations needed to turn `word1` into `word2`.

**Example:** `"horse" → "ros"` → `3`

**Key insight:** If the characters match, `dp[i][j] = dp[i-1][j-1]`. Otherwise `dp[i][j] = 1 + min(delete dp[i-1][j], insert dp[i][j-1], replace dp[i-1][j-1])`.

#### Approach 1: 2-D DP
```rust
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (a, b) = (word1.as_bytes(), word2.as_bytes());
        let (m, n) = (a.len(), b.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 0..=m {
            dp[i][0] = i as i32;
        }
        for j in 0..=n {
            dp[0][j] = j as i32;
        }
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = if a[i - 1] == b[j - 1] {
                    dp[i - 1][j - 1]
                } else {
                    1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]) // replace, delete, insert
                };
            }
        }
        dp[m][n]
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D DP (Optimal space)
```rust
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (a, b) = (word1.as_bytes(), word2.as_bytes());
        let n = b.len();
        let mut dp: Vec<i32> = (0..=n as i32).collect();
        for i in 1..=a.len() {
            let mut diag = dp[0]; // dp[i-1][j-1]
            dp[0] = i as i32;
            for j in 1..=n {
                let tmp = dp[j];
                dp[j] = if a[i - 1] == b[j - 1] { diag } else { 1 + diag.min(dp[j]).min(dp[j - 1]) };
                diag = tmp;
            }
        }
        dp[n]
    }
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
```rust
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut vals = Vec::with_capacity(nums.len() + 2);
        vals.push(1);
        vals.extend(nums);
        vals.push(1);
        let n = vals.len();
        let mut dp = vec![vec![0; n]; n];
        for len in 2..n {
            // gap between l and r
            for l in 0..n - len {
                let r = l + len;
                for k in l + 1..r {
                    // k is the last balloon burst in (l, r)
                    dp[l][r] = dp[l][r].max(dp[l][k] + vals[l] * vals[k] * vals[r] + dp[k][r]);
                }
            }
        }
        dp[0][n - 1]
    }
}
```
**Complexity:** Time O(n³) · Space O(n²)

#### Approach 2: Top-Down Memoization
```rust
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        fn f(v: &[i32], l: usize, r: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if l + 1 == r {
                return 0;
            }
            if memo[l][r] >= 0 {
                return memo[l][r];
            }
            let best = (l + 1..r).map(|k| f(v, l, k, memo) + v[l] * v[k] * v[r] + f(v, k, r, memo)).max().unwrap();
            memo[l][r] = best;
            best
        }
        let vals: Vec<i32> = std::iter::once(1).chain(nums).chain(std::iter::once(1)).collect();
        let n = vals.len();
        f(&vals, 0, n - 1, &mut vec![vec![-1; n]; n])
    }
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
```rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        fn dp(s: &[u8], p: &[u8], i: usize, j: usize, memo: &mut Vec<Vec<Option<bool>>>) -> bool {
            if let Some(v) = memo[i][j] {
                return v;
            }
            let res = if j == p.len() {
                i == s.len()
            } else {
                let first = i < s.len() && (p[j] == s[i] || p[j] == b'.');
                if j + 1 < p.len() && p[j + 1] == b'*' {
                    dp(s, p, i, j + 2, memo) || (first && dp(s, p, i + 1, j, memo)) // skip x*, or consume one char
                } else {
                    first && dp(s, p, i + 1, j + 1, memo)
                }
            };
            memo[i][j] = Some(res);
            res
        }
        let (s, p) = (s.as_bytes(), p.as_bytes());
        dp(s, p, 0, 0, &mut vec![vec![None; p.len() + 1]; s.len() + 1])
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Bottom-Up DP (Optimal)
```rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let (m, n) = (s.len(), p.len());
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[m][n] = true;
        for i in (0..=m).rev() {
            for j in (0..n).rev() {
                let first = i < m && (p[j] == s[i] || p[j] == b'.');
                dp[i][j] = if j + 1 < n && p[j + 1] == b'*' {
                    dp[i][j + 2] || (first && dp[i + 1][j])
                } else {
                    first && dp[i + 1][j + 1]
                };
            }
        }
        dp[0][0]
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

---
## 15. Greedy

### 53. Maximum Subarray
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/maximum-subarray/)

**Problem:** Return the largest sum of any contiguous non-empty subarray.

**Example:** `[-2,1,-3,4,-1,2,1,-5,4]` → `6` (`[4,-1,2,1]`)

**Key insight (Kadane's algorithm):** Throw away a running sum as soon as it goes negative, since it can only drag down whatever follows.

#### Approach 1: Kadane's Algorithm (Optimal)
```rust
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut best, mut cur) = (nums[0], 0);
        for n in nums {
            cur = (cur + n).max(n); // drop the running sum once it would only drag n down
            best = best.max(cur);
        }
        best
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Divide and Conquer (the follow-up)
```rust
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        fn solve(a: &[i32]) -> i32 {
            if a.len() == 1 {
                return a[0];
            }
            let mid = a.len() / 2;
            let (mut left_best, mut sum) = (i32::MIN, 0);
            for &x in a[..mid].iter().rev() {
                sum += x;
                left_best = left_best.max(sum);
            }
            let (mut right_best, mut sum) = (i32::MIN, 0);
            for &x in &a[mid..] {
                sum += x;
                right_best = right_best.max(sum);
            }
            solve(&a[..mid]).max(solve(&a[mid..])).max(left_best + right_best)
        }
        solve(&nums)
    }
}
```
**Complexity:** Time O(n log n) · Space O(log n)

#### Approach 3: Brute Force
```rust
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut best = i32::MIN;
        for i in 0..nums.len() {
            let mut sum = 0;
            for &x in &nums[i..] {
                sum += x;
                best = best.max(sum);
            }
        }
        best
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

---

### 55. Jump Game
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/jump-game/)

**Problem:** `nums[i]` is the farthest you can jump from index `i`. Starting at index 0, can you reach the last index?

**Example:** `[2,3,1,1,4]` → `true`; `[3,2,1,0,4]` → `false`

#### Approach 1: Greedy, Shift the Goal Backward (Optimal)
```rust
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut goal = nums.len() - 1;
        for i in (0..nums.len() - 1).rev() {
            if i + nums[i] as usize >= goal {
                goal = i;
            }
        }
        goal == 0
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Greedy, Track the Farthest Reach Forward
```rust
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut reach = 0usize;
        for (i, &n) in nums.iter().enumerate() {
            if i > reach {
                return false;
            }
            reach = reach.max(i + n as usize);
        }
        true
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 3: DP (slower, shown for contrast)
```rust
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut good = vec![false; n];
        good[n - 1] = true;
        for i in (0..n - 1).rev() {
            let far = (i + nums[i] as usize).min(n - 1);
            good[i] = (i + 1..=far).any(|j| good[j]);
        }
        good[0]
    }
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
```rust
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut jumps, mut cur_end, mut farthest) = (0, 0usize, 0usize);
        for i in 0..nums.len() - 1 {
            farthest = farthest.max(i + nums[i] as usize);
            if i == cur_end {
                // end of this jump's window
                jumps += 1;
                cur_end = farthest;
            }
        }
        jumps
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: DP
```rust
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![i32::MAX; n];
        dp[0] = 0;
        for i in 0..n {
            if dp[i] == i32::MAX {
                continue;
            }
            for j in i + 1..=(i + nums[i] as usize).min(n - 1) {
                dp[j] = dp[j].min(dp[i] + 1);
            }
        }
        dp[n - 1]
    }
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
```rust
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut total, mut tank, mut start) = (0, 0, 0);
        for i in 0..gas.len() {
            let diff = gas[i] - cost[i];
            total += diff;
            tank += diff;
            if tank < 0 {
                // no station in [start, i] can work
                start = i + 1;
                tank = 0;
            }
        }
        if total < 0 { -1 } else { start as i32 }
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Brute Force Simulation
```rust
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        'outer: for s in 0..n {
            let mut tank = 0;
            for k in 0..n {
                let i = (s + k) % n;
                tank += gas[i] - cost[i];
                if tank < 0 {
                    continue 'outer;
                }
            }
            return s as i32;
        }
        -1
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

---

### 846. Hand of Straights
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/hand-of-straights/)

**Problem:** Can the cards in `hand` be rearranged into groups of `groupSize` consecutive values, using every card exactly once?

**Example:** `hand = [1,2,3,6,2,3,4,7,8], groupSize = 3` → `true` (`[1,2,3]`, `[2,3,4]`, `[6,7,8]`)

**Key insight:** The smallest remaining card must start a group, so build each group upward from the current minimum.

#### Approach 1: BTreeMap of Counts
```rust
use std::collections::BTreeMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            return false;
        }
        let mut count: BTreeMap<i32, i32> = BTreeMap::new();
        for h in hand {
            *count.entry(h).or_insert(0) += 1;
        }
        while let Some((&first, _)) = count.iter().next() {
            // the smallest card must start a group
            for v in first..first + group_size {
                match count.get_mut(&v) {
                    None => return false,
                    Some(c) => {
                        *c -= 1;
                        if *c == 0 {
                            count.remove(&v);
                        }
                    }
                }
            }
        }
        true
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sort + HashMap, Consume Whole Runs at Once
```rust
use std::collections::HashMap;

impl Solution {
    pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            return false;
        }
        let mut count: HashMap<i32, i32> = HashMap::new();
        for &h in &hand {
            *count.entry(h).or_insert(0) += 1;
        }
        hand.sort_unstable();
        for &h in &hand {
            let need = *count.get(&h).unwrap_or(&0);
            if need == 0 {
                continue;
            }
            for v in h..h + group_size {
                // start `need` groups at h
                let c = count.entry(v).or_insert(0);
                if *c < need {
                    return false;
                }
                *c -= need;
            }
        }
        true
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

---

### 1899. Merge Triplets to Form Target Triplet
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/merge-triplets-to-form-target-triplet/)

**Problem:** You may merge two triplets by taking the element-wise maximum. Can you produce `target` from some of the given `triplets`?

**Example:** `triplets = [[2,5,3],[1,8,4],[1,7,5]], target = [2,7,5]` → `true`

**Key insight:** Throw out any triplet that exceeds `target` in any position. Among the rest, check that each of the three target values appears in its position in at least one triplet.

#### Approach: Greedy Filter (Optimal)
```rust
impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut found = [false; 3];
        for t in triplets.iter().filter(|t| (0..3).all(|i| t[i] <= target[i])) {
            // anything that would overshoot is skipped above
            for i in 0..3 {
                if t[i] == target[i] {
                    found[i] = true;
                }
            }
        }
        found.iter().all(|&f| f)
    }
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
```rust
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let b = s.as_bytes();
        let mut last = [0usize; 26];
        for (i, &c) in b.iter().enumerate() {
            last[(c - b'a') as usize] = i;
        }
        let (mut res, mut start, mut end) = (Vec::new(), 0, 0);
        for (i, &c) in b.iter().enumerate() {
            end = end.max(last[(c - b'a') as usize]);
            if i == end {
                res.push((end - start + 1) as i32);
                start = i + 1;
            }
        }
        res
    }
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 678. Valid Parenthesis String
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/valid-parenthesis-string/)

**Problem:** The string contains `(`, `)`, and `*`, where each `*` can act as `(`, `)`, or an empty string. Return whether the string can be valid.

**Example:** `"(*))"` → `true`

**Key insight:** Track the range `[lo, hi]` of possible open-paren counts. If `hi` ever drops below 0, the string is invalid. Clamp `lo` at 0. It's valid if `lo == 0` at the end.

#### Approach 1: Greedy Min/Max Open Count (Optimal)
```rust
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let (mut lo, mut hi) = (0i32, 0i32); // range of possible open-paren counts
        for c in s.chars() {
            match c {
                '(' => { lo += 1; hi += 1; }
                ')' => { lo -= 1; hi -= 1; }
                _ => { lo -= 1; hi += 1; } // '*'
            }
            if hi < 0 {
                return false;
            }
            lo = lo.max(0);
        }
        lo == 0
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Two Stacks of Indices
```rust
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let (mut open, mut star): (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => open.push(i),
                '*' => star.push(i),
                _ => {
                    if open.pop().is_none() && star.pop().is_none() {
                        return false;
                    }
                }
            }
        }
        while let (Some(&o), Some(&st)) = (open.last(), star.last()) {
            if o > st {
                return false; // a '(' with no '*' after it to close it
            }
            open.pop();
            star.pop();
        }
        open.is_empty()
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 3: DP over (index, open count)
```rust
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let n = s.len();
        let mut dp = vec![false; n + 2]; // dp[k]: is an open count of k reachable?
        dp[0] = true;
        for c in s.chars() {
            let mut next = vec![false; n + 2];
            for k in 0..=n {
                if !dp[k] {
                    continue;
                }
                if c == '(' || c == '*' {
                    next[k + 1] = true;
                }
                if (c == ')' || c == '*') && k > 0 {
                    next[k - 1] = true;
                }
                if c == '*' {
                    next[k] = true;
                }
            }
            dp = next;
        }
        dp[0]
    }
}
```
**Complexity:** Time O(n²) · Space O(n)

---
## 16. Intervals

> LeetCode passes intervals as `Vec<Vec<i32>>`. Sort with `intervals.sort_unstable_by_key(|iv| iv[0])`.

### 57. Insert Interval
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/insert-interval/)

**Problem:** `intervals` are non-overlapping and sorted by start. Insert `newInterval`, merging wherever needed, and return the result still sorted and non-overlapping.

**Example:** `[[1,3],[6,9]]` + `[2,5]` → `[[1,5],[6,9]]`

**Key insight:** Make three passes over the list: intervals entirely before the new one, intervals that overlap it (merge them), and intervals entirely after.

#### Approach 1: Linear Three-Phase Scan (Optimal)
```rust
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity(intervals.len() + 1);
        let (mut lo, mut hi) = (new_interval[0], new_interval[1]);
        let mut it = intervals.into_iter().peekable();
        while let Some(iv) = it.next_if(|iv| iv[1] < lo) {
            res.push(iv); // entirely before
        }
        while let Some(iv) = it.next_if(|iv| iv[0] <= hi) {
            lo = lo.min(iv[0]); // overlapping: absorb
            hi = hi.max(iv[1]);
        }
        res.push(vec![lo, hi]);
        res.extend(it); // entirely after
        res
    }
}
```
**Complexity:** Time O(n) · Space O(n) for the output

#### Approach 2: Binary Search for the Boundaries
```rust
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let l = intervals.partition_point(|iv| iv[1] < new_interval[0]); // first interval with end >= new start
        let r = intervals.partition_point(|iv| iv[0] <= new_interval[1]); // first interval with start > new end
        let (mut lo, mut hi) = (new_interval[0], new_interval[1]);
        if l < r {
            lo = lo.min(intervals[l][0]);
            hi = hi.max(intervals[r - 1][1]);
        }
        let mut res = intervals[..l].to_vec();
        res.push(vec![lo, hi]);
        res.extend_from_slice(&intervals[r..]);
        res
    }
}
```
**Complexity:** Time O(log n) to search plus O(n) to copy · Space O(n)

---

### 56. Merge Intervals
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/merge-intervals/)

**Problem:** Merge every overlapping interval and return the resulting non-overlapping intervals.

**Example:** `[[1,3],[2,6],[8,10],[15,18]]` → `[[1,6],[8,10],[15,18]]`

#### Approach 1: Sort + Merge (Optimal)
```rust
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|iv| iv[0]);
        let mut res: Vec<Vec<i32>> = Vec::new();
        for iv in intervals {
            match res.last_mut() {
                Some(last) if iv[0] <= last[1] => last[1] = last[1].max(iv[1]),
                _ => res.push(iv),
            }
        }
        res
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sweep over Start and End Events
```rust
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut starts: Vec<i32> = intervals.iter().map(|iv| iv[0]).collect();
        let mut ends: Vec<i32> = intervals.iter().map(|iv| iv[1]).collect();
        starts.sort_unstable();
        ends.sort_unstable();
        let n = starts.len();
        let (mut res, mut begin) = (Vec::new(), 0);
        for i in 0..n {
            // close a merged block when the next start begins after this end
            if i == n - 1 || starts[i + 1] > ends[i] {
                res.push(vec![starts[begin], ends[i]]);
                begin = i + 1;
            }
        }
        res
    }
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
```rust
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|iv| iv[1]);
        let (mut removed, mut end) = (0, i64::MIN);
        for iv in &intervals {
            if iv[0] as i64 >= end {
                end = iv[1] as i64; // keep it
            } else {
                removed += 1;
            }
        }
        removed
    }
}
```
**Complexity:** Time O(n log n) · Space O(1)

#### Approach 2: Sort by Start, Drop the One That Ends Later
```rust
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|iv| iv[0]);
        let (mut removed, mut prev_end) = (0, intervals[0][1]);
        for iv in &intervals[1..] {
            if iv[0] >= prev_end {
                prev_end = iv[1];
            } else {
                removed += 1;
                prev_end = prev_end.min(iv[1]);
            }
        }
        removed
    }
}
```
**Complexity:** Time O(n log n) · Space O(1)

---

### 252. Meeting Rooms (Premium)
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/meeting-rooms/) · Free mirror: LintCode 920

**Problem:** Given meeting time intervals, can one person attend every meeting (that is, do no two meetings overlap)?

**Example:** `[[0,30],[5,10],[15,20]]` → `false`

#### Approach: Sort by Start (Optimal)
```rust
impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_unstable_by_key(|iv| iv[0]);
        intervals.windows(2).all(|w| w[1][0] >= w[0][1])
    }
}
```
**Complexity:** Time O(n log n) · Space O(1)

**Brute force:** Check every pair for overlap: O(n²).

---

### 253. Meeting Rooms II (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/meeting-rooms-ii/) · Free mirror: LintCode 919

**Problem:** Return the minimum number of conference rooms needed to hold every meeting.

**Example:** `[[0,30],[5,10],[15,20]]` → `2`

**Key insight:** The answer is the maximum number of meetings running at the same moment.

#### Approach 1: Separate Sorted Starts and Ends (two pointers)
```rust
impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut starts: Vec<i32> = intervals.iter().map(|iv| iv[0]).collect();
        let mut ends: Vec<i32> = intervals.iter().map(|iv| iv[1]).collect();
        starts.sort_unstable();
        ends.sort_unstable();
        let (mut rooms, mut best, mut e) = (0, 0, 0);
        for &s in &starts {
            while e < ends.len() && ends[e] <= s {
                rooms -= 1; // that meeting has ended and freed its room
                e += 1;
            }
            rooms += 1;
            best = best.max(rooms);
        }
        best
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Min-Heap of End Times
```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|iv| iv[0]);
        let mut ends: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for iv in &intervals {
            if let Some(&Reverse(earliest)) = ends.peek() {
                if earliest <= iv[0] {
                    ends.pop(); // reuse the room that frees up earliest
                }
            }
            ends.push(Reverse(iv[1]));
        }
        ends.len() as i32
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 3: Line Sweep with a BTreeMap
```rust
use std::collections::BTreeMap;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut delta: BTreeMap<i32, i32> = BTreeMap::new();
        for iv in &intervals {
            *delta.entry(iv[0]).or_insert(0) += 1;
            *delta.entry(iv[1]).or_insert(0) -= 1;
        }
        delta
            .values()
            .scan(0, |cur, &d| {
                *cur += d;
                Some(*cur)
            })
            .max()
            .unwrap_or(0)
    }
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
```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_interval(mut intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        intervals.sort_unstable_by_key(|iv| iv[0]);
        let mut q_idx: Vec<usize> = (0..queries.len()).collect();
        q_idx.sort_unstable_by_key(|&i| queries[i]);
        let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new(); // (size, end)
        let mut res = vec![-1; queries.len()];
        let mut i = 0;
        for qi in q_idx {
            let q = queries[qi];
            while i < intervals.len() && intervals[i][0] <= q {
                heap.push(Reverse((intervals[i][1] - intervals[i][0] + 1, intervals[i][1])));
                i += 1;
            }
            while let Some(&Reverse((_, end))) = heap.peek() {
                if end >= q {
                    break;
                }
                heap.pop(); // ends before q
            }
            if let Some(&Reverse((size, _))) = heap.peek() {
                res[qi] = size;
            }
        }
        res
    }
}
```
**Complexity:** Time O(n log n + q log q) · Space O(n + q)

#### Approach 2: Sort Intervals by Size + BTreeMap of Unanswered Queries
Visit intervals from smallest to largest. Each interval answers every unanswered query it contains, and those queries are then removed.
```rust
use std::collections::BTreeMap;

impl Solution {
    pub fn min_interval(mut intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        intervals.sort_unstable_by_key(|iv| iv[1] - iv[0]);
        let mut pending: BTreeMap<i32, Vec<usize>> = BTreeMap::new(); // query value → indices
        for (i, &q) in queries.iter().enumerate() {
            pending.entry(q).or_default().push(i);
        }
        let mut res = vec![-1; queries.len()];
        for iv in &intervals {
            let size = iv[1] - iv[0] + 1;
            let hits: Vec<i32> = pending.range(iv[0]..=iv[1]).map(|(&k, _)| k).collect();
            for k in hits {
                for idx in pending.remove(&k).unwrap() {
                    res[idx] = size;
                }
            }
        }
        res
    }
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
```rust
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in i + 1..n {
                let t = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = t;
            }
        }
        for row in matrix.iter_mut() {
            row.reverse();
        }
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Rotate Four Cells at a Time, Layer by Layer
```rust
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let (mut l, mut r) = (0usize, n.saturating_sub(1));
        while l < r {
            for i in 0..r - l {
                let (top, bottom) = (l, r);
                let tmp = matrix[top][l + i];                     // save top-left
                matrix[top][l + i] = matrix[bottom - i][l];       // bottom-left to top-left
                matrix[bottom - i][l] = matrix[bottom][r - i];    // bottom-right to bottom-left
                matrix[bottom][r - i] = matrix[top + i][r];       // top-right to bottom-right
                matrix[top + i][r] = tmp;                         // top-left to top-right
            }
            l += 1;
            r -= 1;
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
```rust
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();
        // signed bounds avoid usize underflow when they cross
        let (mut top, mut bottom) = (0i32, matrix.len() as i32 - 1);
        let (mut left, mut right) = (0i32, matrix[0].len() as i32 - 1);
        let at = |r: i32, c: i32| matrix[r as usize][c as usize];
        while top <= bottom && left <= right {
            for c in left..=right { res.push(at(top, c)); }
            top += 1;
            for r in top..=bottom { res.push(at(r, right)); }
            right -= 1;
            if top <= bottom {
                for c in (left..=right).rev() { res.push(at(bottom, c)); }
                bottom -= 1;
            }
            if left <= right {
                for r in (top..=bottom).rev() { res.push(at(r, left)); }
                left += 1;
            }
        }
        res
    }
}
```
**Complexity:** Time O(m · n) · Space O(1) extra

#### Approach 2: Direction Vectors with a Visited Grid
```rust
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);
        let mut seen = vec![vec![false; n as usize]; m as usize];
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (mut r, mut c, mut d) = (0i32, 0i32, 0usize);
        let mut res = Vec::with_capacity((m * n) as usize);
        for _ in 0..m * n {
            res.push(matrix[r as usize][c as usize]);
            seen[r as usize][c as usize] = true;
            let (mut nr, mut nc) = (r + dirs[d].0, c + dirs[d].1);
            if nr < 0 || nc < 0 || nr >= m || nc >= n || seen[nr as usize][nc as usize] {
                d = (d + 1) % 4; // turn clockwise
                nr = r + dirs[d].0;
                nc = c + dirs[d].1;
            }
            r = nr;
            c = nc;
        }
        res
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

---

### 73. Set Matrix Zeroes
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/set-matrix-zeroes/)

**Problem:** Wherever the matrix has a `0`, set that cell's entire row and column to `0`, in place.

**Example:** `[[1,1,1],[1,0,1],[1,1,1]]` → `[[1,0,1],[0,0,0],[1,0,1]]`

**Key insight:** Use the first row and first column as marker arrays, with one extra flag for whether the first column itself must be zeroed.

#### Approach 1: Row and Column Flags
```rust
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut rows, mut cols) = (vec![false; m], vec![false; n]);
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    rows[i] = true;
                    cols[j] = true;
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                if rows[i] || cols[j] {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}
```
**Complexity:** Time O(m · n) · Space O(m + n)

#### Approach 2: First Row and Column as Markers (Optimal)
```rust
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut first_col_zero = false;
        for i in 0..m {
            if matrix[i][0] == 0 {
                first_col_zero = true;
            }
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in (0..m).rev() {
            // bottom-up so row 0's markers are used before row 0 is zeroed
            for j in (1..n).rev() {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
            if first_col_zero {
                matrix[i][0] = 0;
            }
        }
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

#### Approach 1: HashSet
```rust
use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        fn next(mut n: i32) -> i32 {
            let mut s = 0;
            while n > 0 {
                s += (n % 10) * (n % 10);
                n /= 10;
            }
            s
        }
        let mut seen = HashSet::new();
        let mut n = n;
        while n != 1 && seen.insert(n) {
            n = next(n);
        }
        n == 1
    }
}
```
**Complexity:** Time O(log n) per step, with only a few steps · Space O(log n)

#### Approach 2: Floyd's Cycle Detection (Optimal)
```rust
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        fn next(mut n: i32) -> i32 {
            let mut s = 0;
            while n > 0 {
                s += (n % 10) * (n % 10);
                n /= 10;
            }
            s
        }
        let (mut slow, mut fast) = (n, next(n));
        while fast != 1 && slow != fast {
            slow = next(slow);
            fast = next(next(fast));
        }
        fast == 1
    }
}
```
**Complexity:** Time O(log n) · Space O(1)

---

### 66. Plus One
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/plus-one/)

**Problem:** A large integer is stored as an array of digits, most significant first. Add one and return the resulting digits.

**Example:** `[1,2,9]` → `[1,3,0]`; `[9,9]` → `[1,0,0]`

#### Approach: Propagate the Carry from the Right (Optimal)
```rust
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for d in digits.iter_mut().rev() {
            if *d < 9 {
                *d += 1;
                return digits;
            }
            *d = 0;
        }
        digits.insert(0, 1); // every digit was 9
        digits
    }
}
```
**Complexity:** Time O(n) · Space O(1), or O(n) when every digit is 9

---

### 50. Pow(x, n)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/powx-n/)

**Problem:** Compute `x` raised to the power `n`, where `n` can be negative (as low as −2³¹).

**Example:** `x = 2.0, n = 10` → `1024.0`; `x = 2.0, n = -2` → `0.25`

**Key insight:** Binary exponentiation: `x^n = (x²)^(n/2)`, with an extra factor of `x` when `n` is odd.

#### Approach 1: Iterative Fast Exponentiation (Optimal)
```rust
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut p = n as i64; // avoids overflow when negating i32::MIN
        let mut x = x;
        if p < 0 {
            x = 1.0 / x;
            p = -p;
        }
        let mut res = 1.0;
        while p > 0 {
            if p & 1 == 1 {
                res *= x;
            }
            x *= x;
            p >>= 1;
        }
        res
    }
}
```
**Complexity:** Time O(log n) · Space O(1)

#### Approach 2: Recursive
```rust
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn pow(b: f64, e: i64) -> f64 {
            if e == 0 {
                return 1.0;
            }
            let half = pow(b, e / 2);
            if e % 2 == 0 { half * half } else { half * half * b }
        }
        let e = n as i64;
        if e < 0 { 1.0 / pow(x, -e) } else { pow(x, e) }
    }
}
```
**Complexity:** Time O(log n) · Space O(log n)

#### Approach 3: Standard Library
```rust
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        x.powi(n)
    }
}
```
**Complexity:** Time O(1)–O(log n) depending on the platform · Space O(1). Good for checking your work, but write Approach 1 in an interview.

---

### 43. Multiply Strings
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/multiply-strings/)

**Problem:** Multiply two non-negative integers given as strings and return the product as a string, without converting the inputs to integers directly.

**Example:** `"123" × "456"` → `"56088"`

**Key insight:** The product of `num1[i]` and `num2[j]` lands in positions `i+j` and `i+j+1` of an array of length `m+n`.

#### Approach: Grade-School Multiplication into a Position Array (Optimal)
```rust
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }
        let (a, b) = (num1.as_bytes(), num2.as_bytes());
        let (m, n) = (a.len(), b.len());
        let mut pos = vec![0u32; m + n];
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let mul = (a[i] - b'0') as u32 * (b[j] - b'0') as u32;
                let sum = mul + pos[i + j + 1];
                pos[i + j + 1] = sum % 10;
                pos[i + j] += sum / 10;
            }
        }
        let start = if pos[0] == 0 { 1 } else { 0 }; // at most one leading zero
        pos[start..].iter().map(|&d| (b'0' + d as u8) as char).collect()
    }
}
```
**Complexity:** Time O(m · n) · Space O(m + n)

**Alternative:** Karatsuba multiplication runs in O(n^1.585), which only pays off for very large inputs. (Big-integer crates aren't available on LeetCode, and the problem forbids using them anyway.)

---

### 2013. Detect Squares
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/detect-squares/)

**Problem:** Design `DetectSquares`. `add(point)` adds a point; duplicates are allowed. `count(point)` returns how many ways you can pick three stored points that form an axis-aligned square of positive area together with the query point.

**Key insight:** Treat each stored point that is diagonal to the query (`|dx| == |dy| ≠ 0`) as the opposite corner. Multiply the counts of the other two corners.

#### Approach 1: Count Map + Iterate Diagonal Candidates (Optimal)
```rust
use std::collections::HashMap;

struct DetectSquares {
    cnt: HashMap<(i32, i32), i32>,
}

impl DetectSquares {
    fn new() -> Self {
        DetectSquares { cnt: HashMap::new() }
    }

    fn add(&mut self, point: Vec<i32>) {
        *self.cnt.entry((point[0], point[1])).or_insert(0) += 1;
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let (qx, qy) = (point[0], point[1]);
        let get = |x: i32, y: i32| *self.cnt.get(&(x, y)).unwrap_or(&0);
        self.cnt
            .iter()
            .filter(|(&(x, y), _)| x != qx && (x - qx).abs() == (y - qy).abs()) // diagonal corners only
            .map(|(&(x, y), &c)| c * get(qx, y) * get(x, qy))
            .sum()
    }
}
```
**Complexity:** `add` O(1) · `count` O(P) for P distinct points · Space O(P)

#### Approach 2: Points Grouped by x-Coordinate
```rust
use std::collections::HashMap;

struct DetectSquares {
    by_x: HashMap<i32, HashMap<i32, i32>>, // x → (y → count)
}

impl DetectSquares {
    fn new() -> Self {
        DetectSquares { by_x: HashMap::new() }
    }

    fn add(&mut self, point: Vec<i32>) {
        *self.by_x.entry(point[0]).or_default().entry(point[1]).or_insert(0) += 1;
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let (qx, qy) = (point[0], point[1]);
        let col = match self.by_x.get(&qx) {
            Some(c) => c,
            None => return 0,
        };
        let mut res = 0;
        for (&y, &c) in col {
            let side = y - qy;
            if side == 0 {
                continue;
            }
            for x in [qx + side, qx - side] {
                // square to the right or to the left
                if let Some(other) = self.by_x.get(&x) {
                    res += c * other.get(&y).unwrap_or(&0) * other.get(&qy).unwrap_or(&0);
                }
            }
        }
        res
    }
}
```
**Complexity:** `add` O(1) · `count` O(points sharing the query's x) · Space O(P)

---
## 18. Bit Manipulation

> Rust has real unsigned types and built-in bit helpers: `count_ones`, `reverse_bits`, `trailing_zeros`, `leading_zeros`. A right shift is logical on `u32` and arithmetic on `i32`, so cast to `u32` when you need zero-fill.

### 136. Single Number
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/single-number/)

**Problem:** Every element appears twice except one. Find that one in O(n) time and O(1) extra space.

**Example:** `[4,1,2,1,2]` → `4`

**Key insight:** `a ^ a = 0` and `a ^ 0 = a`, so XOR-ing everything cancels the pairs.

#### Approach 1: XOR (Optimal)
```rust
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, n| acc ^ n)
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: HashSet Toggle
```rust
use std::collections::HashSet;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for n in nums {
            if !set.insert(n) {
                set.remove(&n);
            }
        }
        *set.iter().next().unwrap()
    }
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 191. Number of 1 Bits
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/number-of-1-bits/)

**Problem:** Return the number of set bits (the Hamming weight) in `n`.

**Example:** `11` (binary `1011`) → `3`

> LeetCode has used both `n: u32` and `n: i32` for this problem's Rust template over time. The bodies below take `i32`; if your editor shows `u32`, change the parameter type and drop the `as u32` cast.

#### Approach 1: Check Every Bit
```rust
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut n = n as u32; // logical shifts
        let mut count = 0;
        while n != 0 {
            count += (n & 1) as i32;
            n >>= 1;
        }
        count
    }
}
```
**Complexity:** Time O(32) · Space O(1)

#### Approach 2: Brian Kernighan's Trick (Optimal)
`n & (n-1)` clears the lowest set bit, so the loop runs once per set bit.
```rust
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut n = n as u32;
        let mut count = 0;
        while n != 0 {
            n &= n - 1;
            count += 1;
        }
        count
    }
}
```
**Complexity:** Time O(number of set bits) · Space O(1)

#### Approach 3: Standard Library (hardware popcount)
```rust
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        n.count_ones() as i32
    }
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
```rust
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0; n + 1];
        for i in 1..=n {
            res[i] = res[i >> 1] + (i & 1) as i32;
        }
        res
    }
}
```
**Complexity:** Time O(n) · Space O(1) extra

#### Approach 2: DP on the Lowest Set Bit
```rust
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0; n + 1];
        for i in 1..=n {
            res[i] = res[i & (i - 1)] + 1;
        }
        res
    }
}
```
**Complexity:** Time O(n) · Space O(1) extra

#### Approach 3: Popcount Each Number
```rust
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        (0..=n as u32).map(|i| i.count_ones() as i32).collect()
    }
}
```
**Complexity:** Time O(n), assuming popcount is O(1) · Space O(1) extra

---

### 190. Reverse Bits
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/reverse-bits/)

**Problem:** Reverse the bits of a 32-bit unsigned integer.

**Example:** `00000010100101000001111010011100` → `00111001011110000010100101000000`

> The Rust template has been `pub fn reverse_bits(x: u32) -> u32`. If your editor shows a signed `n: i32` variant, cast with `n as u32` on the way in and `as i32` on the way out.

#### Approach 1: Bit-by-Bit Shift
```rust
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let (mut x, mut res) = (x, 0u32);
        for _ in 0..32 {
            res = (res << 1) | (x & 1);
            x >>= 1;
        }
        res
    }
}
```
**Complexity:** Time O(32) · Space O(1)

#### Approach 2: Divide and Conquer Swaps (Optimal)
```rust
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut n = x;
        n = (n >> 16) | (n << 16);
        n = ((n & 0xff00ff00) >> 8) | ((n & 0x00ff00ff) << 8);
        n = ((n & 0xf0f0f0f0) >> 4) | ((n & 0x0f0f0f0f) << 4);
        n = ((n & 0xcccccccc) >> 2) | ((n & 0x33333333) << 2);
        n = ((n & 0xaaaaaaaa) >> 1) | ((n & 0x55555555) << 1);
        n
    }
}
```
**Complexity:** Time O(1) (5 steps) · Space O(1)

#### Approach 3: Standard Library
```rust
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        x.reverse_bits()
    }
}
```
**Complexity:** Time O(1) · Space O(1)

---

### 268. Missing Number
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/missing-number/)

**Problem:** `nums` contains `n` distinct numbers from the range `[0, n]`. Return the one that's missing.

**Example:** `[3,0,1]` → `2`

#### Approach 1: Gauss Sum
```rust
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        n * (n + 1) / 2 - nums.iter().sum::<i32>()
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: XOR of Indices and Values (Optimal, can't overflow)
```rust
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        nums.iter().enumerate().fold(nums.len() as i32, |acc, (i, &v)| acc ^ i as i32 ^ v)
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 3: Sort + Binary Search
```rust
impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        // first index whose value no longer equals the index
        let (mut lo, mut hi) = (0usize, nums.len());
        while lo < hi {
            let mid = (lo + hi) / 2;
            if nums[mid] > mid as i32 { hi = mid; } else { lo = mid + 1; }
        }
        lo as i32
    }
}
```
**Complexity:** Time O(n log n) · Space O(1)

---

### 371. Sum of Two Integers
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/sum-of-two-integers/)

**Problem:** Return `a + b` without using the `+` or `-` operators.

**Example:** `a = 1, b = 2` → `3`; `a = -2, b = 3` → `1`

**Key insight:** `a ^ b` is the sum without carries, and `(a & b) << 1` is the carry. Repeat until the carry is zero. Work in `int32` to get LeetCode's 32-bit two's-complement behavior.

#### Approach 1: Iterative XOR/AND (Optimal)
```rust
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let (mut a, mut b) = (a, b);
        while b != 0 {
            let carry = (a & b) << 1; // bits that carry into the next position
            a ^= b;                   // sum without carries
            b = carry;
        }
        a
    }
}
```
**Complexity:** Time O(32) · Space O(1)

> `i32 << 1` never panics in Rust. Shift-overflow checks only trigger when the shift *amount* is ≥ 32, and bits shifted out the top are simply discarded, so negative inputs work as two's complement.

#### Approach 2: Recursive
```rust
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        if b == 0 { a } else { Self::get_sum(a ^ b, (a & b) << 1) }
    }
}
```
**Complexity:** Time O(32) · Space O(32)

---

### 7. Reverse Integer
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/reverse-integer/)

**Problem:** Reverse the digits of a signed 32-bit integer `x`. If the result falls outside `[-2³¹, 2³¹-1]`, return `0`. Assume 64-bit storage isn't available.

**Example:** `123` → `321`; `-123` → `-321`; `1534236469` → `0`

**Key insight:** Before computing `res*10 + d`, check whether it would overflow by comparing `res` against `MaxInt32/10` and `MinInt32/10`.

#### Approach 1: Checked Arithmetic (Optimal, idiomatic Rust)
`checked_mul` / `checked_add` return `None` on overflow, and the `?` operator turns that into an early exit.
```rust
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        fn rev(mut x: i32) -> Option<i32> {
            let mut res: i32 = 0;
            while x != 0 {
                res = res.checked_mul(10)?.checked_add(x % 10)?; // Rust's % keeps the sign of x
                x /= 10;
            }
            Some(res)
        }
        rev(x).unwrap_or(0)
    }
}
```
**Complexity:** Time O(log₁₀ x) · Space O(1)

#### Approach 2: Manual Overflow Guards (no 64-bit, as the problem asks)
```rust
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let (mut x, mut res) = (x, 0i32);
        while x != 0 {
            let d = x % 10;
            x /= 10;
            if res > i32::MAX / 10 || (res == i32::MAX / 10 && d > 7) {
                return 0;
            }
            if res < i32::MIN / 10 || (res == i32::MIN / 10 && d < -8) {
                return 0;
            }
            res = res * 10 + d;
        }
        res
    }
}
```
**Complexity:** Time O(log₁₀ x) · Space O(1)

#### Approach 3: 64-bit Accumulator (simpler, if allowed)
```rust
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let (mut x, mut res) = (x as i64, 0i64);
        while x != 0 {
            res = res * 10 + x % 10;
            x /= 10;
        }
        i32::try_from(res).unwrap_or(0)
    }
}
```
**Complexity:** Time O(log₁₀ x) · Space O(1)

---

## Complexity Cheat Sheet

| Pattern | Typical time | Typical space | Signals in the problem | Rust tools |
|---|---|---|---|---|
| Hash map / set | O(n) | O(n) | "seen before", duplicates, complements | `HashMap`, `HashSet`, `entry().or_insert()` |
| Two pointers | O(n) | O(1) | sorted input, pairs, palindromes | `usize` indices, `Ordering` match |
| Sliding window | O(n) | O(k) | contiguous substring or subarray with a constraint | `[i32; 128]` counts, `as_bytes()` |
| Monotonic stack | O(n) | O(n) | next greater or smaller element, histograms | `Vec` + `while let Some(&top) = stack.last()` |
| Binary search | O(log n) | O(1) | sorted input, or "minimum k such that…" | `partition_point`, `binary_search`, `BTreeMap::range` |
| Heap | O(n log k) | O(k) | top-k, k-way merge, streaming median | `BinaryHeap`, `Reverse` |
| DFS / BFS | O(V + E) | O(V) | grids, connectivity, shortest path in an unweighted graph | `VecDeque`, `wrapping_sub` bounds trick |
| Topological sort | O(V + E) | O(V + E) | dependencies, ordering, cycle detection | in-degree `Vec` + `VecDeque` |
| Union-find | O(α(n)) per operation | O(n) | dynamic connectivity, redundant edges | `Vec<usize>` parent with path halving |
| Dijkstra | O(E log V) | O(V + E) | weighted shortest path with non-negative weights | `BinaryHeap<Reverse<(dist, node)>>` |
| Backtracking | O(2ⁿ) / O(n!) | O(n) | "all combinations / permutations / partitions" | nested `fn` + `&mut Vec`, `path.clone()` |
| 1-D DP | O(n) to O(n²) | O(1) to O(n) | best value over a prefix | `vec![0; n + 1]`, `fold` |
| 2-D DP | O(m · n) | O(n) when rolled | two strings, grids, intervals | `vec![vec![..]]`, `std::mem::swap` rows |
| Greedy | O(n) or O(n log n) | O(1) | a locally best choice provably works | `sort_unstable_by_key` + scan |
| Bit tricks | O(1) to O(n) | O(1) | XOR pairing, powers of two, bit counts | `count_ones`, `reverse_bits`, `u32` shifts |

---

*End of document: 150 problems, 18 categories.*
