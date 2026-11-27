# LeetCode "Blind 150" (NeetCode 150) — Complete Python Solutions

A full Python 3 reference for the 150 problems in the **NeetCode 150** list (the Blind 75 plus 75 more), grouped by pattern. Every problem has:

- its **LeetCode number and title**, with difficulty
- a **short problem statement** in my own words, plus an example
- one or more **complete Python solutions**: brute force → better → optimal where that comparison helps
- **time and space complexity** for each approach
- a **key insight** line explaining the idea

> **Every snippet includes all the imports it needs** (`from typing import List, Optional`, `from collections import Counter, defaultdict, deque`, `import heapq`, `import bisect`, `from functools import cache`, …), so it runs exactly as shown. Each one is LeetCode's `class Solution:` with the exact camelCase method name and type hints from LeetCode's Python3 template, so you can paste any approach straight into the editor. Design problems (`MinStack`, `LRUCache`, …) use LeetCode's class names.
>
> Every snippet was checked with **pyflakes** (no undefined names, no unused imports) and run on **Python 3.13**. All 150 problems were run on randomized tests that check every alternative approach against the others and, where possible, against an independent reference answer. The code needs **Python 3.9+**; `functools.cache` is 3.9+, and `math.comb`/`isqrt` are 3.8+.

---

## Shared Definitions

LeetCode provides these classes for linked-list and tree problems. They are assumed (not redeclared) in those sections; each snippet still imports `Optional` for its type hints.

```python
from typing import Optional


# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
```

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

## Python Interview Toolkit (quick reference)

```python
import bisect
import heapq
from collections import Counter, OrderedDict, defaultdict, deque
from functools import cache, lru_cache
from itertools import accumulate, combinations, permutations, product

# Counting and grouping
freq = Counter(nums)                 # freq.most_common(k)
groups = defaultdict(list)           # groups[key].append(x)

# Stack and queue
stack = []                           # append / pop / stack[-1]
q = deque([start])                   # append / popleft

# heapq is a MIN-heap; negate values for a max-heap
heap = []
heapq.heappush(heap, (dist, node))   # tuples compare lexicographically
dist, node = heapq.heappop(heap)
heapq.heapify(arr)                   # O(n)
top_k = heapq.nlargest(k, arr)

# Binary search on a sorted list
i = bisect.bisect_left(arr, x)       # first index with arr[i] >= x
j = bisect.bisect_right(arr, x)      # first index with arr[j] > x

# Memoized recursion
@cache
def dp(i, j): ...
```

**Python gotchas for LeetCode:**
- `heapq` only does min-heaps. Push `-x` for a max-heap, or `(-priority, tiebreak, item)` when the items themselves aren't comparable.
- Deep recursion hits the default limit of about 1000 frames. Raise it with `sys.setrecursionlimit(10**6)` or use an iterative version for long linked lists and deep grids.
- On CPython 3.12+, **`@cache`-decorated recursion can't go much deeper than ~3,000 levels** even with a raised limit, because the C-implemented wrapper consumes C stack. When recursion depth can reach the input size (for example `amount` up to 10⁴ in Coin Change), this file uses a plain `dict` memo instead, which survives 40,000+ levels.
- `[[0] * n] * m` creates `m` references to the **same** row. Use `[[0] * n for _ in range(m)]`.
- Python ints never overflow, so problems that ask you to detect 32-bit overflow (#7, #371) need explicit masking or bounds checks.
- `list.pop(0)` is O(n). Use `collections.deque.popleft()` for queues.

---
## 1. Arrays & Hashing

### 217. Contains Duplicate
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/contains-duplicate/)

**Problem:** Given an integer array `nums`, return `True` if any value appears at least twice, and `False` if every element is distinct.

**Example:** `nums = [1,2,3,1]` → `True`; `nums = [1,2,3,4]` → `False`

**Key insight:** A hash set gives O(1) membership checks, so you can spot a repeat in one pass.

#### Approach 1: Brute Force
```python
from typing import List


class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        for i in range(len(nums)):
            for j in range(i + 1, len(nums)):
                if nums[i] == nums[j]:
                    return True
        return False
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Sorting
```python
from typing import List


class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        nums = sorted(nums)
        return any(nums[i] == nums[i - 1] for i in range(1, len(nums)))
```
**Complexity:** Time O(n log n) · Space O(n) (`sorted` makes a copy; `nums.sort()` would be O(1) extra)

#### Approach 3: Hash Set (Optimal)
```python
from typing import List


class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        seen = set()
        for n in nums:
            if n in seen:
                return True
            seen.add(n)
        return False
```
**Complexity:** Time O(n) · Space O(n)

**One-liner:** `return len(set(nums)) < len(nums)` has the same complexity but can't stop early.

---

### 242. Valid Anagram
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/valid-anagram/)

**Problem:** Given strings `s` and `t`, return `True` if `t` is an anagram of `s`, meaning both use exactly the same letters with the same counts.

**Example:** `s = "anagram", t = "nagaram"` → `True`; `s = "rat", t = "car"` → `False`

**Key insight:** Two strings are anagrams when their character frequency counts match.

#### Approach 1: Sorting
```python
class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        return sorted(s) == sorted(t)
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Frequency Array (Optimal)
```python
class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if len(s) != len(t):
            return False
        count = [0] * 26
        for a, b in zip(s, t):
            count[ord(a) - ord("a")] += 1
            count[ord(b) - ord("a")] -= 1
        return all(c == 0 for c in count)
```
**Complexity:** Time O(n) · Space O(1) (26 counters)

#### Approach 3: `Counter` (handles Unicode follow-up)
```python
from collections import Counter


class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        return Counter(s) == Counter(t)
```
**Complexity:** Time O(n) · Space O(k), where k is the number of distinct characters

---

### 1. Two Sum
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/two-sum/)

**Problem:** Given an integer array `nums` and an integer `target`, return the indices of the two numbers that add up to `target`. Exactly one solution exists, and you may not use the same element twice.

**Example:** `nums = [2,7,11,15], target = 9` → `[0,1]`

**Key insight:** For each number `x`, you need `target - x`. Store values you've already seen in a map from value to index.

#### Approach 1: Brute Force
```python
from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        for i in range(len(nums)):
            for j in range(i + 1, len(nums)):
                if nums[i] + nums[j] == target:
                    return [i, j]
        return []
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: One-Pass Hash Map (Optimal)
```python
from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        idx = {}  # value -> index
        for i, n in enumerate(nums):
            if target - n in idx:
                return [idx[target - n], i]
            idx[n] = i
        return []
```
**Complexity:** Time O(n) · Space O(n)

---

### 49. Group Anagrams
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/group-anagrams/)

**Problem:** Given an array of strings `strs`, group the anagrams together. You may return the groups in any order.

**Example:** `["eat","tea","tan","ate","nat","bat"]` → `[["bat"],["nat","tan"],["ate","eat","tea"]]`

**Key insight:** All anagrams share a canonical key, either the sorted string or the 26-letter count vector.

#### Approach 1: Sorted String as Key
```python
from collections import defaultdict
from typing import List


class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        groups = defaultdict(list)
        for s in strs:
            groups["".join(sorted(s))].append(s)
        return list(groups.values())
```
**Complexity:** Time O(m · k log k), for m strings of max length k · Space O(m · k)

#### Approach 2: Count Tuple as Key (Optimal)
```python
from collections import defaultdict
from typing import List


class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        groups = defaultdict(list)
        for s in strs:
            count = [0] * 26
            for c in s:
                count[ord(c) - ord("a")] += 1
            groups[tuple(count)].append(s)  # tuples are hashable, lists are not
        return list(groups.values())
```
**Complexity:** Time O(m · k) · Space O(m · k)

---

### 347. Top K Frequent Elements
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/top-k-frequent-elements/)

**Problem:** Given an integer array `nums` and an integer `k`, return the `k` most frequent elements, in any order.

**Example:** `nums = [1,1,1,2,2,3], k = 2` → `[1,2]`

**Key insight:** Frequencies are bounded by `n`, so bucket sort by frequency gives O(n).

#### Approach 1: Sort by Frequency
```python
from collections import Counter
from typing import List


class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        freq = Counter(nums)
        return sorted(freq, key=freq.get, reverse=True)[:k]
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Min-Heap of Size k
```python
import heapq
from collections import Counter
from typing import List


class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        freq = Counter(nums)
        heap = []  # (count, value), smallest count on top
        for v, c in freq.items():
            heapq.heappush(heap, (c, v))
            if len(heap) > k:
                heapq.heappop(heap)
        return [v for _, v in heap]
```
**Complexity:** Time O(n log k) · Space O(n)

**Built-in shortcut:** `[v for v, _ in Counter(nums).most_common(k)]` uses a heap internally, also O(n log k).

#### Approach 3: Bucket Sort (Optimal)
```python
from collections import Counter
from typing import List


class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        buckets = [[] for _ in range(len(nums) + 1)]  # index = frequency
        for v, c in Counter(nums).items():
            buckets[c].append(v)
        res = []
        for f in range(len(buckets) - 1, 0, -1):
            for v in buckets[f]:
                res.append(v)
                if len(res) == k:
                    return res
        return res
```
**Complexity:** Time O(n) · Space O(n)

---

### 271. Encode and Decode Strings (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/encode-and-decode-strings/) · Free mirror: LintCode 659

**Problem:** Design an algorithm that encodes a list of strings into a single string, and decodes that string back into the original list. The strings may contain any character, including delimiters.

**Example:** `["neet","code","love","you"]` → encode → decode → `["neet","code","love","you"]`

**Key insight:** Length-prefix each string (`len#str`). The decoder reads the length first, so delimiter characters inside the strings can't confuse it.

#### Approach: Length Prefix (Optimal)
```python
from typing import List


class Codec:
    def encode(self, strs: List[str]) -> str:
        """Encodes a list of strings to a single string."""
        return "".join(f"{len(s)}#{s}" for s in strs)

    def decode(self, s: str) -> List[str]:
        """Decodes a single string to a list of strings."""
        res, i = [], 0
        while i < len(s):
            j = s.index("#", i)
            length = int(s[i:j])
            res.append(s[j + 1 : j + 1 + length])
            i = j + 1 + length
        return res
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
```python
from typing import List


class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        n = len(nums)
        prefix, suffix = [1] * n, [1] * n
        for i in range(1, n):
            prefix[i] = prefix[i - 1] * nums[i - 1]
        for i in range(n - 2, -1, -1):
            suffix[i] = suffix[i + 1] * nums[i + 1]
        return [prefix[i] * suffix[i] for i in range(n)]
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: In-Place Prefix and Running Suffix (Optimal)
```python
from typing import List


class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        n = len(nums)
        res = [1] * n
        for i in range(1, n):
            res[i] = res[i - 1] * nums[i - 1]
        suffix = 1
        for i in range(n - 1, -1, -1):
            res[i] *= suffix
            suffix *= nums[i]
        return res
```
**Complexity:** Time O(n) · Space O(1) extra (the output doesn't count)

---

### 36. Valid Sudoku
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/valid-sudoku/)

**Problem:** Check whether a partially filled 9×9 Sudoku board is valid. Each row, each column, and each 3×3 sub-box must have no repeated digits 1–9. Empty cells are `'.'`. The board doesn't need to be solvable.

**Key insight:** Track seen digits per row, per column, and per box. Box index = `(r/3)*3 + c/3`.

#### Approach 1: Sets per Row, Column, and Box
```python
from collections import defaultdict
from typing import List


class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        rows, cols, boxes = defaultdict(set), defaultdict(set), defaultdict(set)
        for r in range(9):
            for c in range(9):
                d = board[r][c]
                if d == ".":
                    continue
                b = (r // 3, c // 3)
                if d in rows[r] or d in cols[c] or d in boxes[b]:
                    return False
                rows[r].add(d)
                cols[c].add(d)
                boxes[b].add(d)
        return True
```
**Complexity:** Time O(81) = O(1) · Space O(1)

#### Approach 2: Bitmasks
```python
from typing import List


class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        rows, cols, boxes = [0] * 9, [0] * 9, [0] * 9
        for r in range(9):
            for c in range(9):
                if board[r][c] == ".":
                    continue
                bit = 1 << (int(board[r][c]) - 1)
                b = (r // 3) * 3 + c // 3
                if rows[r] & bit or cols[c] & bit or boxes[b] & bit:
                    return False
                rows[r] |= bit
                cols[c] |= bit
                boxes[b] |= bit
        return True
```
**Complexity:** Time O(1) · Space O(1)

#### Approach 3: Single Set of Encoded Tuples
```python
from typing import List


class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        seen = set()
        for r in range(9):
            for c in range(9):
                d = board[r][c]
                if d == ".":
                    continue
                keys = [(d, "row", r), (d, "col", c), (d, "box", r // 3, c // 3)]
                if any(k in seen for k in keys):
                    return False
                seen.update(keys)
        return True
```
**Complexity:** Time O(1) · Space O(1)

---

### 128. Longest Consecutive Sequence
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-consecutive-sequence/)

**Problem:** Given an unsorted integer array `nums`, return the length of the longest run of consecutive integers. The algorithm must run in O(n).

**Example:** `[100,4,200,1,3,2]` → `4` (the run is `1,2,3,4`)

**Key insight:** Only start counting at a number `x` when `x-1` isn't in the set. That way each run is walked once.

#### Approach 1: Sorting
```python
from typing import List


class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        if not nums:
            return 0
        nums = sorted(set(nums))
        best = cur = 1
        for i in range(1, len(nums)):
            cur = cur + 1 if nums[i] == nums[i - 1] + 1 else 1
            best = max(best, cur)
        return best
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Hash Set (Optimal)
```python
from typing import List


class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        num_set = set(nums)
        best = 0
        for n in num_set:
            if n - 1 in num_set:
                continue  # not the start of a run
            length = 1
            while n + length in num_set:
                length += 1
            best = max(best, length)
        return best
```
**Complexity:** Time O(n) · Space O(n)

---
## 2. Two Pointers

### 125. Valid Palindrome
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/valid-palindrome/)

**Problem:** A phrase is a palindrome if, after lowercasing it and removing every non-alphanumeric character, it reads the same forward and backward. Given a string `s`, return whether it is a palindrome.

**Example:** `"A man, a plan, a canal: Panama"` → `True`; `"race a car"` → `False`

**Key insight:** Move two pointers inward, skip non-alphanumeric characters, and compare case-insensitively.

#### Approach 1: Clean, Then Compare
```python
class Solution:
    def isPalindrome(self, s: str) -> bool:
        clean = [c.lower() for c in s if c.isalnum()]
        return clean == clean[::-1]
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Two Pointers In Place (Optimal)
```python
class Solution:
    def isPalindrome(self, s: str) -> bool:
        l, r = 0, len(s) - 1
        while l < r:
            if not s[l].isalnum():
                l += 1
            elif not s[r].isalnum():
                r -= 1
            else:
                if s[l].lower() != s[r].lower():
                    return False
                l += 1
                r -= 1
        return True
```
**Complexity:** Time O(n) · Space O(1)

> `str.isalnum()` also accepts Unicode letters and digits. That's harmless here, because LeetCode's input is printable ASCII.

---

### 167. Two Sum II – Input Array Is Sorted
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)

**Problem:** Given a **1-indexed** array `numbers` sorted in non-decreasing order, find two numbers that add up to `target`. Return their 1-based indices `[i, j]` with `i < j`. Use only constant extra space.

**Example:** `numbers = [2,7,11,15], target = 9` → `[1,2]`

**Key insight:** If the sum is too small, move the left pointer right. If it's too big, move the right pointer left.

#### Approach 1: Binary Search for the Complement
```python
import bisect
from typing import List


class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        for i, n in enumerate(numbers):
            j = bisect.bisect_left(numbers, target - n, i + 1)
            if j < len(numbers) and numbers[j] == target - n:
                return [i + 1, j + 1]
        return []
```
**Complexity:** Time O(n log n) · Space O(1)

#### Approach 2: Two Pointers (Optimal)
```python
from typing import List


class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        l, r = 0, len(numbers) - 1
        while l < r:
            s = numbers[l] + numbers[r]
            if s == target:
                return [l + 1, r + 1]
            if s < target:
                l += 1
            else:
                r -= 1
        return []
```
**Complexity:** Time O(n) · Space O(1)

---

### 15. 3Sum
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/3sum/)

**Problem:** Given an integer array `nums`, return every unique triplet `[a, b, c]` with distinct indices such that `a + b + c == 0`. The result must not contain duplicate triplets.

**Example:** `[-1,0,1,2,-1,-4]` → `[[-1,-1,2],[-1,0,1]]`

**Key insight:** Sort the array, fix the first element, then run Two Sum II on the rest. Skip equal values to avoid duplicate triplets.

#### Approach 1: Hash Set per Anchor
```python
from typing import List


class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        res = set()
        for i in range(len(nums)):
            if i > 0 and nums[i] == nums[i - 1]:
                continue
            seen = set()
            for j in range(i + 1, len(nums)):
                need = -nums[i] - nums[j]
                if need in seen:
                    res.add((nums[i], need, nums[j]))  # the set removes duplicate triplets
                seen.add(nums[j])
        return [list(t) for t in res]
```
**Complexity:** Time O(n²) · Space O(n)

#### Approach 2: Sort + Two Pointers (Optimal)
```python
from typing import List


class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        res = []
        for i in range(len(nums) - 2):
            if nums[i] > 0:
                break  # the smallest value is positive, so no triplet can sum to zero
            if i > 0 and nums[i] == nums[i - 1]:
                continue
            l, r = i + 1, len(nums) - 1
            while l < r:
                s = nums[i] + nums[l] + nums[r]
                if s < 0:
                    l += 1
                elif s > 0:
                    r -= 1
                else:
                    res.append([nums[i], nums[l], nums[r]])
                    l += 1
                    r -= 1
                    while l < r and nums[l] == nums[l - 1]:
                        l += 1
        return res
```
**Complexity:** Time O(n²) · Space O(1) extra (plus the sort and the output)

---

### 11. Container With Most Water
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/container-with-most-water/)

**Problem:** `height[i]` is the height of a vertical line at position `i`. Pick two lines that, together with the x-axis, hold the most water, and return that maximum area.

**Example:** `[1,8,6,2,5,4,8,3,7]` → `49`

**Key insight:** The area is limited by the shorter line, so moving the taller pointer can never help. Always move the shorter one.

#### Approach 1: Brute Force
```python
from typing import List


class Solution:
    def maxArea(self, height: List[int]) -> int:
        n = len(height)
        return max((j - i) * min(height[i], height[j]) for i in range(n) for j in range(i + 1, n))
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Two Pointers (Optimal)
```python
from typing import List


class Solution:
    def maxArea(self, height: List[int]) -> int:
        l, r, best = 0, len(height) - 1, 0
        while l < r:
            best = max(best, (r - l) * min(height[l], height[r]))
            if height[l] < height[r]:
                l += 1
            else:
                r -= 1
        return best
```
**Complexity:** Time O(n) · Space O(1)

---

### 42. Trapping Rain Water
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/trapping-rain-water/)

**Problem:** Given `n` non-negative integers representing an elevation map where each bar has width 1, compute how much water it traps after raining.

**Example:** `[0,1,0,2,1,0,1,3,2,1,2,1]` → `6`

**Key insight:** The water above bar `i` is `min(maxLeft, maxRight) - height[i]`.

#### Approach 1: Prefix and Suffix Max Arrays
```python
from itertools import accumulate
from typing import List


class Solution:
    def trap(self, height: List[int]) -> int:
        if not height:
            return 0
        left = list(accumulate(height, max))
        right = list(accumulate(reversed(height), max))[::-1]
        return sum(min(l, r) - h for l, r, h in zip(left, right, height))
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Monotonic Stack
```python
from typing import List


class Solution:
    def trap(self, height: List[int]) -> int:
        stack = []  # indices with decreasing heights
        water = 0
        for i, h in enumerate(height):
            while stack and h > height[stack[-1]]:
                bottom = stack.pop()
                if not stack:
                    break
                left = stack[-1]
                width = i - left - 1
                water += width * (min(height[left], h) - height[bottom])
            stack.append(i)
        return water
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 3: Two Pointers (Optimal)
```python
from typing import List


class Solution:
    def trap(self, height: List[int]) -> int:
        l, r = 0, len(height) - 1
        left_max = right_max = water = 0
        while l < r:
            if height[l] < height[r]:
                left_max = max(left_max, height[l])
                water += left_max - height[l]
                l += 1
            else:
                right_max = max(right_max, height[r])
                water += right_max - height[r]
                r -= 1
        return water
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
```python
from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        best = 0
        for i in range(len(prices)):
            for j in range(i + 1, len(prices)):
                best = max(best, prices[j] - prices[i])
        return best
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: One Pass with Running Min (Optimal)
```python
from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        min_price, best = float("inf"), 0
        for p in prices:
            min_price = min(min_price, p)
            best = max(best, p - min_price)
        return best
```
**Complexity:** Time O(n) · Space O(1)

---

### 3. Longest Substring Without Repeating Characters
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-substring-without-repeating-characters/)

**Problem:** Given a string `s`, return the length of the longest substring that has no repeated characters.

**Example:** `"abcabcbb"` → `3` (`"abc"`); `"pwwkew"` → `3` (`"wke"`)

**Key insight:** Keep a window with no duplicates. When a duplicate arrives, jump the left edge past its previous occurrence.

#### Approach 1: Sliding Window with a Set
```python
class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        window = set()
        l = best = 0
        for r, c in enumerate(s):
            while c in window:
                window.remove(s[l])
                l += 1
            window.add(c)
            best = max(best, r - l + 1)
        return best
```
**Complexity:** Time O(n) (each character enters and leaves once) · Space O(min(n, alphabet))

#### Approach 2: Last-Seen Index Jump (Optimal)
```python
class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        last = {}  # char -> last index seen
        l = best = 0
        for r, c in enumerate(s):
            if c in last and last[c] >= l:
                l = last[c] + 1
            last[c] = r
            best = max(best, r - l + 1)
        return best
```
**Complexity:** Time O(n) · Space O(min(n, alphabet))

---

### 424. Longest Repeating Character Replacement
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-repeating-character-replacement/)

**Problem:** Given an uppercase string `s` and an integer `k`, you may change up to `k` characters to any other uppercase letter. Return the length of the longest substring containing a single repeated letter after the changes.

**Example:** `s = "AABABBA", k = 1` → `4`

**Key insight:** A window is valid when `windowLen - maxFreq <= k`. `maxFreq` never needs to decrease, because only a larger `maxFreq` can produce a longer answer.

#### Approach 1: Recompute Max Frequency (O(26) per step)
```python
from collections import Counter


class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        count = Counter()
        l = best = 0
        for r, c in enumerate(s):
            count[c] += 1
            while (r - l + 1) - max(count.values()) > k:
                count[s[l]] -= 1
                l += 1
            best = max(best, r - l + 1)
        return best
```
**Complexity:** Time O(26 · n) · Space O(1)

#### Approach 2: Running Max Frequency (Optimal)
```python
from collections import Counter


class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        count = Counter()
        l = max_freq = 0
        for r, c in enumerate(s):
            count[c] += 1
            max_freq = max(max_freq, count[c])
            if (r - l + 1) - max_freq > k:
                count[s[l]] -= 1  # shift the window, don't shrink it
                l += 1
        return len(s) - l  # the window size never shrinks, so its final size is the answer
```
**Complexity:** Time O(n) · Space O(1)

---

### 567. Permutation in String
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/permutation-in-string/)

**Problem:** Given strings `s1` and `s2`, return `True` if some permutation of `s1` appears as a substring of `s2`.

**Example:** `s1 = "ab", s2 = "eidbaooo"` → `True` (`"ba"`)

**Key insight:** Slide a fixed window of size `len(s1)` across `s2` and compare letter counts.

#### Approach 1: Compare Counters Each Step
```python
from collections import Counter


class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        k = len(s1)
        if k > len(s2):
            return False
        need, win = Counter(s1), Counter(s2[:k])
        if need == win:
            return True
        for r in range(k, len(s2)):
            win[s2[r]] += 1
            win[s2[r - k]] -= 1
            if need == win:  # Counter equality ignores zero counts (3.10+)
                return True
        return False
```
**Complexity:** Time O(26 · n) · Space O(1)

> On Python 3.9 and earlier, `Counter.__eq__` treats a key with count 0 as different from a missing key. In that case use the array-based Approach 2.

#### Approach 2: Track the Number of Matching Letters (Optimal)
```python
class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        k = len(s1)
        if k > len(s2):
            return False
        c1, c2 = [0] * 26, [0] * 26
        for i in range(k):
            c1[ord(s1[i]) - 97] += 1
            c2[ord(s2[i]) - 97] += 1
        matches = sum(c1[i] == c2[i] for i in range(26))
        for r in range(k, len(s2)):
            if matches == 26:
                return True
            for idx, delta in ((ord(s2[r]) - 97, 1), (ord(s2[r - k]) - 97, -1)):
                if c1[idx] == c2[idx]:
                    matches -= 1
                c2[idx] += delta
                if c1[idx] == c2[idx]:
                    matches += 1
        return matches == 26
```
**Complexity:** Time O(n) · Space O(1)

---

### 76. Minimum Window Substring
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/minimum-window-substring/)

**Problem:** Given strings `s` and `t`, return the shortest substring of `s` that contains every character of `t`, including duplicates. Return `""` if there is none.

**Example:** `s = "ADOBECODEBANC", t = "ABC"` → `"BANC"`

**Key insight:** Expand right until the window covers `t` (`have == need`), then shrink from the left while it still covers `t`, recording the smallest window.

#### Approach: Sliding Window with Counts (Optimal)
```python
from collections import Counter


class Solution:
    def minWindow(self, s: str, t: str) -> str:
        if not t or len(t) > len(s):
            return ""
        need = Counter(t)
        window = Counter()
        have, required = 0, len(need)
        best_l, best_len = 0, float("inf")
        l = 0
        for r, c in enumerate(s):
            window[c] += 1
            if c in need and window[c] == need[c]:
                have += 1
            while have == required:
                if r - l + 1 < best_len:
                    best_l, best_len = l, r - l + 1
                lc = s[l]
                window[lc] -= 1
                if lc in need and window[lc] < need[lc]:
                    have -= 1
                l += 1
        return "" if best_len == float("inf") else s[best_l : best_l + best_len]
```
**Complexity:** Time O(|s| + |t|) · Space O(alphabet)

**Alternative (brute force):** Check every substring of `s` against `t`'s counts: O(n² · 52) time. Too slow for the constraints, but useful as a correctness baseline.

---

### 239. Sliding Window Maximum
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/sliding-window-maximum/)

**Problem:** Given an array `nums` and a window size `k`, slide the window from left to right one step at a time and return the maximum of each window.

**Example:** `nums = [1,3,-1,-3,5,3,6,7], k = 3` → `[3,3,5,5,6,7]`

**Key insight:** Keep a deque of indices whose values are decreasing. The front is always the window's maximum.

#### Approach 1: Max-Heap with Lazy Deletion
```python
import heapq
from typing import List


class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        heap, res = [], []  # (-value, index)
        for i, n in enumerate(nums):
            heapq.heappush(heap, (-n, i))
            if i >= k - 1:
                while heap[0][1] <= i - k:  # top has left the window
                    heapq.heappop(heap)
                res.append(-heap[0][0])
        return res
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Monotonic Deque (Optimal)
```python
from collections import deque
from typing import List


class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        dq = deque()  # indices; their nums values are decreasing
        res = []
        for i, n in enumerate(nums):
            if dq and dq[0] <= i - k:
                dq.popleft()
            while dq and nums[dq[-1]] < n:
                dq.pop()
            dq.append(i)
            if i >= k - 1:
                res.append(nums[dq[0]])
        return res
```
**Complexity:** Time O(n) (each index is pushed and popped at most once) · Space O(k)

---
## 4. Stack

> A plain Python `list` is the stack: `append`, `pop`, and `stack[-1]` to peek.

### 20. Valid Parentheses
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/valid-parentheses/)

**Problem:** Given a string made only of `()[]{}`, decide whether it is valid. Every opening bracket must be closed by the same type of bracket, in the correct order.

**Example:** `"()[]{}"` → `True`; `"(]"` → `False`; `"([)]"` → `False`

**Key insight:** The most recent unmatched opening bracket must be the first one closed, which is last-in, first-out.

#### Approach 1: Stack (Optimal)
```python
class Solution:
    def isValid(self, s: str) -> bool:
        pairs = {")": "(", "]": "[", "}": "{"}
        stack = []
        for c in s:
            if c in pairs:
                if not stack or stack.pop() != pairs[c]:
                    return False
            else:
                stack.append(c)
        return not stack
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Repeated Replacement (simple but slow)
```python
class Solution:
    def isValid(self, s: str) -> bool:
        prev = None
        while prev != s:
            prev = s
            s = s.replace("()", "").replace("[]", "").replace("{}", "")
        return s == ""
```
**Complexity:** Time O(n²) · Space O(n)

---

### 155. Min Stack
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/min-stack/)

**Problem:** Design a stack that supports `push`, `pop`, `top`, and `getMin` (return the smallest element) in O(1) time each.

**Key insight:** Store the minimum at each depth alongside the value, so popping restores the previous minimum automatically.

#### Approach 1: Pairs of (value, current min)
```python
class MinStack:
    def __init__(self):
        self.stack = []  # (value, min at this depth)

    def push(self, val: int) -> None:
        cur_min = min(val, self.stack[-1][1]) if self.stack else val
        self.stack.append((val, cur_min))

    def pop(self) -> None:
        self.stack.pop()

    def top(self) -> int:
        return self.stack[-1][0]

    def getMin(self) -> int:
        return self.stack[-1][1]
```
**Complexity:** O(1) per operation · Space O(n)

#### Approach 2: Two Stacks (min stack only grows on a new min)
```python
class MinStack:
    def __init__(self):
        self.stack = []
        self.mins = []

    def push(self, val: int) -> None:
        self.stack.append(val)
        if not self.mins or val <= self.mins[-1]:
            self.mins.append(val)

    def pop(self) -> None:
        if self.stack.pop() == self.mins[-1]:
            self.mins.pop()

    def top(self) -> int:
        return self.stack[-1]

    def getMin(self) -> int:
        return self.mins[-1]
```
**Complexity:** O(1) per operation · Space O(n), usually less in practice

#### Approach 3: Single Stack Storing Differences (O(1) extra space)
```python
class MinStack:
    def __init__(self):
        self.stack = []  # value - min at push time
        self.min = 0

    def push(self, val: int) -> None:
        if not self.stack:
            self.stack.append(0)
            self.min = val
        else:
            self.stack.append(val - self.min)
            self.min = min(self.min, val)

    def pop(self) -> None:
        diff = self.stack.pop()
        if diff < 0:
            self.min -= diff  # the popped value was the min, so restore the previous one

    def top(self) -> int:
        diff = self.stack[-1]
        return self.min if diff < 0 else self.min + diff

    def getMin(self) -> int:
        return self.min
```
**Complexity:** O(1) per operation · Space O(n) for values, no second stack

---

### 150. Evaluate Reverse Polish Notation
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/evaluate-reverse-polish-notation/)

**Problem:** Evaluate an arithmetic expression written in Reverse Polish (postfix) notation. The operators are `+ - * /`, and division truncates toward zero.

**Example:** `["2","1","+","3","*"]` → `9`; `["4","13","5","/","+"]` → `6`

**Key insight:** Push operands. On an operator, pop two values (second operand first) and push the result.

#### Approach 1: Stack with an Operator Table (Optimal)
```python
import operator
from typing import List


class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        ops = {
            "+": operator.add,
            "-": operator.sub,
            "*": operator.mul,
            "/": lambda a, b: int(a / b),  # truncate toward zero; Python's // floors
        }
        stack = []
        for tok in tokens:
            if tok in ops:
                b, a = stack.pop(), stack.pop()
                stack.append(ops[tok](a, b))
            else:
                stack.append(int(tok))
        return stack[0]
```
**Complexity:** Time O(n) · Space O(n)

> **Pitfall:** `-7 // 2 == -4` in Python, but the problem wants `-3`. `int(a / b)` truncates toward zero. Its float division is exact for the 32-bit range LeetCode uses.

#### Approach 2: Recursive Evaluation from the End
```python
from typing import List


class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        def evaluate() -> int:
            tok = tokens.pop()
            if tok not in "+-*/" or len(tok) > 1:  # "-3" is a number, not an operator
                return int(tok)
            b = evaluate()  # right operand comes first when reading backward
            a = evaluate()
            if tok == "+":
                return a + b
            if tok == "-":
                return a - b
            if tok == "*":
                return a * b
            return int(a / b)

        tokens = list(tokens)
        return evaluate()
```
**Complexity:** Time O(n) · Space O(n) recursion

---

### 22. Generate Parentheses
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/generate-parentheses/)

**Problem:** Given `n`, generate every combination of `n` pairs of well-formed parentheses.

**Example:** `n = 3` → `["((()))","(()())","(())()","()(())","()()()"]`

**Key insight:** You can add `(` while `open < n`, and `)` while `close < open`.

#### Approach 1: Backtracking (Optimal)
```python
from typing import List


class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        res, path = [], []

        def bt(open_: int, close: int) -> None:
            if len(path) == 2 * n:
                res.append("".join(path))
                return
            if open_ < n:
                path.append("(")
                bt(open_ + 1, close)
                path.pop()
            if close < open_:
                path.append(")")
                bt(open_, close + 1)
                path.pop()

        bt(0, 0)
        return res
```
**Complexity:** Time O(4ⁿ / √n), the nth Catalan number of results × length · Space O(n) recursion

#### Approach 2: Dynamic Programming (closure number)
Every valid string has the form `"(" + a + ")" + b`, where `a` has `i` pairs and `b` has `n-1-i` pairs.
```python
from typing import List


class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        dp = [[""]]
        for k in range(1, n + 1):
            dp.append([f"({a}){b}" for i in range(k) for a in dp[i] for b in dp[k - 1 - i]])
        return dp[n]
```
**Complexity:** Time O(4ⁿ / √n) · Space O(4ⁿ / √n)

---

### 739. Daily Temperatures
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/daily-temperatures/)

**Problem:** Given daily `temperatures`, return `answer` where `answer[i]` is the number of days you must wait after day `i` for a warmer temperature, or `0` if no warmer day comes.

**Example:** `[73,74,75,71,69,72,76,73]` → `[1,1,4,2,1,1,0,0]`

**Key insight:** Keep a monotonic decreasing stack of unresolved days. A warmer day resolves every colder day on top of the stack.

#### Approach 1: Monotonic Stack (Optimal)
```python
from typing import List


class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        res = [0] * len(temperatures)
        stack = []  # indices of unresolved days
        for i, t in enumerate(temperatures):
            while stack and temperatures[stack[-1]] < t:
                j = stack.pop()
                res[j] = i - j
            stack.append(i)
        return res
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Backward Jumps (O(1) extra space)
Scan from the right and reuse `res` to hop forward to the next warmer candidate.
```python
from typing import List


class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        n = len(temperatures)
        res = [0] * n
        hottest = 0
        for i in range(n - 1, -1, -1):
            t = temperatures[i]
            if t >= hottest:
                hottest = t
                continue
            days = 1
            while temperatures[i + days] <= t:
                days += res[i + days]
            res[i] = days
        return res
```
**Complexity:** Time O(n) amortized · Space O(1) extra

---

### 853. Car Fleet
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/car-fleet/)

**Problem:** `n` cars drive toward `target` on a one-lane road. Car `i` starts at `position[i]` with speed `speed[i]`. A faster car that catches a slower one ahead joins it and moves at the slower speed, forming a fleet. Return how many fleets arrive at the target.

**Example:** `target = 12, position = [10,8,0,5,3], speed = [2,4,1,1,3]` → `3`

**Key insight:** Sort cars by position, closest to the target first. A car whose arrival time is ≤ the fleet ahead of it merges with that fleet. Otherwise it starts a new fleet.

#### Approach 1: Sort + Monotonic Stack of Times
```python
from typing import List


class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        stack = []  # arrival times of fleets ahead
        for p, s in sorted(zip(position, speed), reverse=True):  # closest to the target first
            t = (target - p) / s
            if not stack or t > stack[-1]:
                stack.append(t)  # too slow to catch the fleet ahead, so a new fleet
        return len(stack)
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sort + Single Variable with Exact Fractions
`fractions.Fraction` avoids any floating-point tie issues when two cars arrive at exactly the same moment.
```python
from fractions import Fraction
from typing import List


class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        fleets, slowest = 0, Fraction(-1)
        for p, s in sorted(zip(position, speed), reverse=True):
            t = Fraction(target - p, s)
            if t > slowest:
                fleets += 1
                slowest = t
        return fleets
```
**Complexity:** Time O(n log n) · Space O(n)

---

### 84. Largest Rectangle in Histogram
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/largest-rectangle-in-histogram/)

**Problem:** Given bar heights `heights`, each bar having width 1, return the area of the largest rectangle that fits in the histogram.

**Example:** `[2,1,5,6,2,3]` → `10` (bars 5 and 6, width 2)

**Key insight:** When a shorter bar arrives, each taller bar popped from an increasing stack has found its right boundary. Its left boundary is the new stack top.

#### Approach 1: Brute Force (expand each bar)
```python
from typing import List


class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        best = 0
        for i in range(len(heights)):
            min_h = heights[i]
            for j in range(i, len(heights)):
                min_h = min(min_h, heights[j])
                best = max(best, min_h * (j - i + 1))
        return best
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Monotonic Increasing Stack (Optimal)
```python
from typing import List


class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        stack = []  # (start index, height), heights increasing
        best = 0
        for i, h in enumerate(heights + [0]):  # sentinel 0 flushes the stack at the end
            start = i
            while stack and stack[-1][1] >= h:
                idx, height = stack.pop()
                best = max(best, height * (i - idx))
                start = idx  # the new bar can extend back to where the popped bar started
            stack.append((start, h))
        return best
```
**Complexity:** Time O(n) · Space O(n)

---
## 5. Binary Search

### 704. Binary Search
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/binary-search/)

**Problem:** Given a sorted array `nums` of distinct integers and a `target`, return the target's index, or `-1` if it's not present. Run in O(log n).

**Example:** `nums = [-1,0,3,5,9,12], target = 9` → `4`

**Key insight:** Halve the search space each step. In Python, `mid = (lo + hi) // 2` is safe because ints never overflow.

#### Approach 1: Iterative (Optimal)
```python
from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        lo, hi = 0, len(nums) - 1
        while lo <= hi:
            mid = (lo + hi) // 2  # Python ints don't overflow
            if nums[mid] == target:
                return mid
            if nums[mid] < target:
                lo = mid + 1
            else:
                hi = mid - 1
        return -1
```
**Complexity:** Time O(log n) · Space O(1)

#### Approach 2: Recursive
```python
from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        def rec(lo: int, hi: int) -> int:
            if lo > hi:
                return -1
            mid = (lo + hi) // 2
            if nums[mid] == target:
                return mid
            return rec(mid + 1, hi) if nums[mid] < target else rec(lo, mid - 1)

        return rec(0, len(nums) - 1)
```
**Complexity:** Time O(log n) · Space O(log n) recursion

#### Approach 3: Standard Library `bisect`
```python
import bisect
from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        i = bisect.bisect_left(nums, target)
        return i if i < len(nums) and nums[i] == target else -1
```
**Complexity:** Time O(log n) · Space O(1)

---

### 74. Search a 2D Matrix
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/search-a-2d-matrix/)

**Problem:** In an `m × n` matrix, each row is sorted, and each row's first value is greater than the previous row's last value. Return whether `target` is in the matrix, in O(log(m·n)).

**Example:** `[[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3` → `True`

**Key insight:** The matrix is one sorted array of length `m·n`. Index `k` maps to `(k/n, k%n)`.

#### Approach 1: Flattened Binary Search (Optimal)
```python
from typing import List


class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        m, n = len(matrix), len(matrix[0])
        lo, hi = 0, m * n - 1
        while lo <= hi:
            mid = (lo + hi) // 2
            v = matrix[mid // n][mid % n]
            if v == target:
                return True
            if v < target:
                lo = mid + 1
            else:
                hi = mid - 1
        return False
```
**Complexity:** Time O(log(m·n)) · Space O(1)

#### Approach 2: Two Binary Searches (row, then column)
```python
import bisect
from typing import List


class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        # the row that could hold target is the last one whose first element is <= target
        row = bisect.bisect_right([r[0] for r in matrix], target) - 1
        if row < 0:
            return False
        i = bisect.bisect_left(matrix[row], target)
        return i < len(matrix[row]) and matrix[row][i] == target
```
**Complexity:** Time O(m + log n). Building the first-column list is O(m); a manual row search would make it O(log m + log n). · Space O(m)

#### Approach 3: Staircase from the Top-Right (also works for LC 240)
```python
from typing import List


class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        r, c = 0, len(matrix[0]) - 1
        while r < len(matrix) and c >= 0:
            if matrix[r][c] == target:
                return True
            if matrix[r][c] > target:
                c -= 1
            else:
                r += 1
        return False
```
**Complexity:** Time O(m + n) · Space O(1)

---

### 875. Koko Eating Bananas
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/koko-eating-bananas/)

**Problem:** There are `piles[i]` bananas in each pile. Each hour Koko picks one pile and eats up to `k` bananas from it. Return the minimum integer `k` that lets her finish everything within `h` hours.

**Example:** `piles = [3,6,7,11], h = 8` → `4`

**Key insight:** Hours needed decreases as `k` grows. Binary search on the answer `k` in `[1, max(piles)]`.

#### Approach: Binary Search on the Answer (Optimal)
```python
import math
from typing import List


class Solution:
    def minEatingSpeed(self, piles: List[int], h: int) -> int:
        lo, hi = 1, max(piles)
        while lo < hi:
            k = (lo + hi) // 2
            if sum(math.ceil(p / k) for p in piles) <= h:
                hi = k  # k works; try smaller
            else:
                lo = k + 1
        return lo
```
**Complexity:** Time O(n · log M), where M = max pile · Space O(1)

> `math.ceil(p / k)` goes through a float. That's safe for these bounds (≤ 10⁹), but `(p + k - 1) // k` or `-(-p // k)` is exact integer ceiling division.

**Brute force:** Try `k = 1, 2, …` until it works: O(n · M), which times out.

---

### 153. Find Minimum in Rotated Sorted Array
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/)

**Problem:** A sorted array of unique values was rotated an unknown number of times. Return its minimum element in O(log n).

**Example:** `[3,4,5,1,2]` → `1`; `[4,5,6,7,0,1,2]` → `0`

**Key insight:** Compare `nums[mid]` with `nums[hi]`. If `mid` is larger, the minimum is to the right. Otherwise it's at `mid` or to its left.

#### Approach: Binary Search vs. the Right End (Optimal)
```python
from typing import List


class Solution:
    def findMin(self, nums: List[int]) -> int:
        lo, hi = 0, len(nums) - 1
        while lo < hi:
            mid = (lo + hi) // 2
            if nums[mid] > nums[hi]:
                lo = mid + 1
            else:
                hi = mid
        return nums[lo]
```
**Complexity:** Time O(log n) · Space O(1)

**Linear alternative:** `min(nums)` is O(n). Fine for a sanity check, but it doesn't meet the requirement.

---

### 33. Search in Rotated Sorted Array
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/search-in-rotated-sorted-array/)

**Problem:** Search for `target` in a rotated sorted array of distinct values. Return its index or `-1`, in O(log n).

**Example:** `nums = [4,5,6,7,0,1,2], target = 0` → `4`

**Key insight:** At every `mid`, at least one half is sorted. Check whether `target` falls inside the sorted half's range.

#### Approach 1: One-Pass Binary Search (Optimal)
```python
from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        lo, hi = 0, len(nums) - 1
        while lo <= hi:
            mid = (lo + hi) // 2
            if nums[mid] == target:
                return mid
            if nums[lo] <= nums[mid]:  # left half sorted
                if nums[lo] <= target < nums[mid]:
                    hi = mid - 1
                else:
                    lo = mid + 1
            else:  # right half sorted
                if nums[mid] < target <= nums[hi]:
                    lo = mid + 1
                else:
                    hi = mid - 1
        return -1
```
**Complexity:** Time O(log n) · Space O(1)

#### Approach 2: Find the Pivot, Then Binary Search the Correct Half
```python
import bisect
from typing import List


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        lo, hi = 0, len(nums) - 1
        while lo < hi:  # find the index of the minimum
            mid = (lo + hi) // 2
            if nums[mid] > nums[hi]:
                lo = mid + 1
            else:
                hi = mid
        pivot = lo
        lo, hi = (0, pivot) if pivot > 0 and target >= nums[0] else (pivot, len(nums))
        i = bisect.bisect_left(nums, target, lo, hi)
        return i if i < hi and nums[i] == target else -1
```
**Complexity:** Time O(log n) · Space O(1)

---

### 981. Time Based Key-Value Store
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/time-based-key-value-store/)

**Problem:** Design `TimeMap` with `set(key, value, timestamp)` and `get(key, timestamp)`. `get` returns the value stored at the largest timestamp ≤ `timestamp`, or `""` if none. Calls to `set` arrive with strictly increasing timestamps.

**Key insight:** Each key's entries are already sorted by time, so `get` is an upper-bound binary search.

#### Approach 1: Dict of Lists + `bisect` (Optimal)
```python
import bisect
from collections import defaultdict


class TimeMap:
    def __init__(self):
        self.times = defaultdict(list)   # key -> timestamps (increasing)
        self.values = defaultdict(list)  # key -> values, aligned with times

    def set(self, key: str, value: str, timestamp: int) -> None:
        self.times[key].append(timestamp)
        self.values[key].append(value)

    def get(self, key: str, timestamp: int) -> str:
        i = bisect.bisect_right(self.times.get(key, []), timestamp)  # first index with ts > timestamp
        return self.values[key][i - 1] if i else ""
```
**Complexity:** `set` O(1) amortized · `get` O(log n) · Space O(total entries)

#### Approach 2: Manual Binary Search over (timestamp, value) Pairs
```python
from collections import defaultdict


class TimeMap:
    def __init__(self):
        self.store = defaultdict(list)  # key -> [(timestamp, value)]

    def set(self, key: str, value: str, timestamp: int) -> None:
        self.store[key].append((timestamp, value))

    def get(self, key: str, timestamp: int) -> str:
        entries = self.store.get(key, [])
        lo, hi, ans = 0, len(entries) - 1, ""
        while lo <= hi:  # last index with ts <= timestamp
            mid = (lo + hi) // 2
            if entries[mid][0] <= timestamp:
                ans = entries[mid][1]
                lo = mid + 1
            else:
                hi = mid - 1
        return ans
```
**Complexity:** `set` O(1) amortized · `get` O(log n) · Space O(total entries)

---

### 4. Median of Two Sorted Arrays
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/median-of-two-sorted-arrays/)

**Problem:** Given sorted arrays `nums1` (size m) and `nums2` (size n), return the median of their combined elements in O(log(m+n)).

**Example:** `[1,3]`, `[2]` → `2.0`; `[1,2]`, `[3,4]` → `2.5`

**Key insight:** Binary search a cut in the shorter array so the left halves of both arrays together hold half the elements, and every element on the left is ≤ every element on the right.

#### Approach 1: Merge (simple)
```python
import heapq
from typing import List


class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        merged = list(heapq.merge(nums1, nums2))  # linear merge of two sorted lists
        t = len(merged)
        return merged[t // 2] if t % 2 else (merged[t // 2 - 1] + merged[t // 2]) / 2
```
**Complexity:** Time O(m + n) · Space O(m + n)

#### Approach 2: Binary Search Partition (Optimal)
```python
from typing import List


class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        a, b = (nums1, nums2) if len(nums1) <= len(nums2) else (nums2, nums1)  # search the shorter array
        m, n = len(a), len(b)
        half = (m + n + 1) // 2
        lo, hi = 0, m
        inf = float("inf")
        while lo <= hi:
            i = (lo + hi) // 2  # elements taken from a
            j = half - i        # elements taken from b
            a_left = a[i - 1] if i > 0 else -inf
            a_right = a[i] if i < m else inf
            b_left = b[j - 1] if j > 0 else -inf
            b_right = b[j] if j < n else inf
            if a_left <= b_right and b_left <= a_right:
                if (m + n) % 2:
                    return float(max(a_left, b_left))
                return (max(a_left, b_left) + min(a_right, b_right)) / 2
            if a_left > b_right:
                hi = i - 1
            else:
                lo = i + 1
        return 0.0
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
```python
from typing import Optional


class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        prev, cur = None, head
        while cur:
            cur.next, prev, cur = prev, cur, cur.next  # the right side is evaluated first
        return prev
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Recursive
```python
from typing import Optional


class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not head or not head.next:
            return head
        new_head = self.reverseList(head.next)
        head.next.next = head
        head.next = None
        return new_head
```
**Complexity:** Time O(n) · Space O(n) recursion

---

### 21. Merge Two Sorted Lists
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/merge-two-sorted-lists/)

**Problem:** Merge two sorted linked lists into one sorted list by splicing their nodes together, and return its head.

**Example:** `1→2→4` + `1→3→4` → `1→1→2→3→4→4`

**Key insight:** A dummy head removes the special case for the first node.

#### Approach 1: Iterative with a Dummy Node (Optimal)
```python
from typing import Optional


class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        dummy = tail = ListNode()
        while list1 and list2:
            if list1.val <= list2.val:
                tail.next, list1 = list1, list1.next
            else:
                tail.next, list2 = list2, list2.next
            tail = tail.next
        tail.next = list1 or list2
        return dummy.next
```
**Complexity:** Time O(n + m) · Space O(1)

#### Approach 2: Recursive
```python
from typing import Optional


class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        if not list1 or not list2:
            return list1 or list2
        if list1.val <= list2.val:
            list1.next = self.mergeTwoLists(list1.next, list2)
            return list1
        list2.next = self.mergeTwoLists(list1, list2.next)
        return list2
```
**Complexity:** Time O(n + m) · Space O(n + m) recursion

---

### 143. Reorder List
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/reorder-list/)

**Problem:** Reorder `L0→L1→…→Ln` in place into `L0→Ln→L1→Ln-1→L2→…`. Change the links, not the values.

**Example:** `1→2→3→4→5` → `1→5→2→4→3`

**Key insight:** Find the middle, reverse the second half, then interleave the two halves.

#### Approach 1: Split + Reverse + Merge (Optimal)
```python
from typing import Optional


class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        """Do not return anything, modify head in-place instead."""
        if not head or not head.next:
            return
        # 1. find the middle
        slow, fast = head, head.next
        while fast and fast.next:
            slow, fast = slow.next, fast.next.next
        # 2. reverse the second half
        second, slow.next = slow.next, None
        prev = None
        while second:
            second.next, prev, second = prev, second, second.next
        # 3. interleave
        first, second = head, prev
        while second:
            n1, n2 = first.next, second.next
            first.next, second.next = second, n1
            first, second = n1, n2
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: List of Nodes
```python
from typing import Optional


class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        nodes = []
        while head:
            nodes.append(head)
            head = head.next
        i, j = 0, len(nodes) - 1
        while i < j:
            nodes[i].next = nodes[j]
            i += 1
            if i == j:
                break
            nodes[j].next = nodes[i]
            j -= 1
        if nodes:
            nodes[i].next = None
```
**Complexity:** Time O(n) · Space O(n)

---

### 19. Remove Nth Node From End of List
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/remove-nth-node-from-end-of-list/)

**Problem:** Remove the `n`th node from the end of a linked list and return the head.

**Example:** `1→2→3→4→5, n = 2` → `1→2→3→5`

**Key insight:** Move a lead pointer `n` steps ahead, then advance both pointers until the lead hits the end. The trailing pointer is then just before the target.

#### Approach 1: Two Pointers, One Pass (Optimal)
```python
from typing import Optional


class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        dummy = ListNode(0, head)
        slow = fast = dummy
        for _ in range(n + 1):
            fast = fast.next
        while fast:
            slow, fast = slow.next, fast.next
        slow.next = slow.next.next
        return dummy.next
```
**Complexity:** Time O(L) · Space O(1)

#### Approach 2: Two Passes (count the length)
```python
from typing import Optional


class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        length, cur = 0, head
        while cur:
            length, cur = length + 1, cur.next
        dummy = cur = ListNode(0, head)
        for _ in range(length - n):
            cur = cur.next
        cur.next = cur.next.next
        return dummy.next
```
**Complexity:** Time O(L) · Space O(1)

---

### 138. Copy List with Random Pointer
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/copy-list-with-random-pointer/)

**Problem:** Each node has `next` and `random` pointers, and `random` may point to any node or `None`. Return a deep copy of the list.

**Key insight:** Map each original node to its copy. Alternatively, weave each copy right after its original so `orig.Random.Next` is the copied random target.

> LeetCode provides the `Node` class below for this problem. It's included so the snippet runs on its own.

#### Approach 1: Hash Map
```python
from typing import Optional


class Node:
    def __init__(self, x: int, next: "Node" = None, random: "Node" = None):
        self.val = int(x)
        self.next = next
        self.random = random


class Solution:
    def copyRandomList(self, head: "Optional[Node]") -> "Optional[Node]":
        copies = {None: None}
        cur = head
        while cur:
            copies[cur] = Node(cur.val)
            cur = cur.next
        cur = head
        while cur:
            copies[cur].next = copies[cur.next]
            copies[cur].random = copies[cur.random]
            cur = cur.next
        return copies[head]
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Interweaving (O(1) extra space)
```python
from typing import Optional


class Node:
    def __init__(self, x: int, next: "Node" = None, random: "Node" = None):
        self.val = int(x)
        self.next = next
        self.random = random


class Solution:
    def copyRandomList(self, head: "Optional[Node]") -> "Optional[Node]":
        if not head:
            return None
        cur = head
        while cur:  # A -> A' -> B -> B' ...
            cur.next = Node(cur.val, cur.next)
            cur = cur.next.next
        cur = head
        while cur:  # copy.random = original.random.next
            if cur.random:
                cur.next.random = cur.random.next
            cur = cur.next.next
        new_head, cur = head.next, head
        while cur:  # unweave the two lists
            copy = cur.next
            cur.next = copy.next
            copy.next = copy.next.next if copy.next else None
            cur = cur.next
        return new_head
```
**Complexity:** Time O(n) · Space O(1) extra

---

### 2. Add Two Numbers
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/add-two-numbers/)

**Problem:** Two non-negative integers are stored as linked lists with digits in reverse order, so the head is the ones digit. Return their sum as a list in the same format.

**Example:** `2→4→3` (342) + `5→6→4` (465) → `7→0→8` (807)

**Key insight:** Grade-school addition with a carry. Keep looping while either list or the carry remains.

#### Approach 1: Iterative (Optimal)
```python
from typing import Optional


class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        dummy = tail = ListNode()
        carry = 0
        while l1 or l2 or carry:
            total = carry + (l1.val if l1 else 0) + (l2.val if l2 else 0)
            carry, digit = divmod(total, 10)
            tail.next = ListNode(digit)
            tail = tail.next
            l1 = l1.next if l1 else None
            l2 = l2.next if l2 else None
        return dummy.next
```
**Complexity:** Time O(max(m, n)) · Space O(1) extra (the output doesn't count)

#### Approach 2: Recursive
```python
from typing import Optional


class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode], carry: int = 0) -> Optional[ListNode]:
        if not l1 and not l2 and not carry:
            return None
        total = carry + (l1.val if l1 else 0) + (l2.val if l2 else 0)
        node = ListNode(total % 10)
        node.next = self.addTwoNumbers(l1.next if l1 else None, l2.next if l2 else None, total // 10)
        return node
```
**Complexity:** Time O(max(m, n)) · Space O(max(m, n)) recursion

---

### 141. Linked List Cycle
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/linked-list-cycle/)

**Problem:** Return `True` if the linked list contains a cycle.

**Key insight:** Floyd's tortoise and hare. If there's a cycle, the fast pointer eventually laps the slow one.

#### Approach 1: Hash Set
```python
from typing import Optional


class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        seen = set()
        while head:
            if head in seen:  # nodes hash by identity
                return True
            seen.add(head)
            head = head.next
        return False
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Floyd's Fast and Slow Pointers (Optimal)
```python
from typing import Optional


class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        slow = fast = head
        while fast and fast.next:
            slow, fast = slow.next, fast.next.next
            if slow is fast:
                return True
        return False
```
**Complexity:** Time O(n) · Space O(1)

---

### 287. Find the Duplicate Number
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/find-the-duplicate-number/)

**Problem:** `nums` has `n + 1` integers, each in `[1, n]`, and exactly one value is repeated (possibly many times). Return it without modifying the array, using O(1) extra space.

**Example:** `[1,3,4,2,2]` → `2`

**Key insight:** Treat `i → nums[i]` as a linked list. The duplicate value is the entry point of the cycle, which you can find with Floyd's algorithm.

#### Approach 1: Hash Set (uses extra space)
```python
from typing import List


class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        seen = set()
        for n in nums:
            if n in seen:
                return n
            seen.add(n)
        return -1
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Binary Search on Value (pigeonhole)
```python
from typing import List


class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        lo, hi = 1, len(nums) - 1
        while lo < hi:
            mid = (lo + hi) // 2
            if sum(n <= mid for n in nums) > mid:
                hi = mid
            else:
                lo = mid + 1
        return lo
```
**Complexity:** Time O(n log n) · Space O(1)

#### Approach 3: Floyd's Cycle Detection (Optimal)
```python
from typing import List


class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        slow, fast = nums[0], nums[nums[0]]
        while slow != fast:
            slow, fast = nums[slow], nums[nums[fast]]
        slow = 0
        while slow != fast:
            slow, fast = nums[slow], nums[fast]
        return slow
```
**Complexity:** Time O(n) · Space O(1)

---

### 146. LRU Cache
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/lru-cache/)

**Problem:** Design an LRU (least recently used) cache with a fixed `capacity`. `get(key)` returns the value or `-1`. `put(key, value)` inserts or updates a key, evicting the least recently used key when over capacity. Both must run in O(1).

**Key insight:** A hash map gives O(1) lookup, and a doubly linked list gives O(1) reordering and eviction.

#### Approach 1: Hash Map + Doubly Linked List (Optimal, interview-preferred)
```python
class DNode:
    __slots__ = ("key", "val", "prev", "next")

    def __init__(self, key: int = 0, val: int = 0):
        self.key, self.val = key, val
        self.prev = self.next = None


class LRUCache:
    def __init__(self, capacity: int):
        self.cap = capacity
        self.items = {}  # key -> DNode
        self.head, self.tail = DNode(), DNode()  # sentinels: head.next is the MRU, tail.prev is the LRU
        self.head.next, self.tail.prev = self.tail, self.head

    def _remove(self, node: DNode) -> None:
        node.prev.next, node.next.prev = node.next, node.prev

    def _push_front(self, node: DNode) -> None:
        node.prev, node.next = self.head, self.head.next
        self.head.next.prev = node
        self.head.next = node

    def get(self, key: int) -> int:
        node = self.items.get(key)
        if not node:
            return -1
        self._remove(node)
        self._push_front(node)
        return node.val

    def put(self, key: int, value: int) -> None:
        if key in self.items:
            node = self.items[key]
            node.val = value
            self._remove(node)
            self._push_front(node)
            return
        node = DNode(key, value)
        self.items[key] = node
        self._push_front(node)
        if len(self.items) > self.cap:
            lru = self.tail.prev
            self._remove(lru)
            del self.items[lru.key]
```
**Complexity:** O(1) per operation · Space O(capacity)

#### Approach 2: `collections.OrderedDict`
```python
from collections import OrderedDict


class LRUCache:
    def __init__(self, capacity: int):
        self.cap = capacity
        self.cache = OrderedDict()  # oldest first

    def get(self, key: int) -> int:
        if key not in self.cache:
            return -1
        self.cache.move_to_end(key)  # mark as most recently used
        return self.cache[key]

    def put(self, key: int, value: int) -> None:
        self.cache[key] = value
        self.cache.move_to_end(key)
        if len(self.cache) > self.cap:
            self.cache.popitem(last=False)  # evict the LRU
```
**Complexity:** O(1) per operation · Space O(capacity). Good to mention in an interview, but interviewers usually want Approach 1.

---

### 23. Merge k Sorted Lists
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/merge-k-sorted-lists/)

**Problem:** Merge `k` sorted linked lists into one sorted list.

**Example:** `[1→4→5, 1→3→4, 2→6]` → `1→1→2→3→4→4→5→6`

**Key insight:** Either keep a min-heap of the `k` current heads, or merge the lists in pairs (divide and conquer). Both are O(N log k).

#### Approach 1: Min-Heap
`ListNode` isn't comparable, so each heap entry carries a unique index as a tiebreaker.
```python
import heapq
from typing import List, Optional


class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        heap = [(node.val, i, node) for i, node in enumerate(lists) if node]
        heapq.heapify(heap)
        dummy = tail = ListNode()
        while heap:
            _, i, node = heapq.heappop(heap)
            tail.next = tail = node
            if node.next:
                heapq.heappush(heap, (node.next.val, i, node.next))
        return dummy.next
```
**Complexity:** Time O(N log k) for N total nodes · Space O(k)

#### Approach 2: Divide and Conquer Pairwise Merge (Optimal, O(1) extra)
```python
from typing import List, Optional


class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        def merge2(a: Optional[ListNode], b: Optional[ListNode]) -> Optional[ListNode]:
            dummy = tail = ListNode()
            while a and b:
                if a.val <= b.val:
                    tail.next, a = a, a.next
                else:
                    tail.next, b = b, b.next
                tail = tail.next
            tail.next = a or b
            return dummy.next

        if not lists:
            return None
        step = 1
        while step < len(lists):
            for i in range(0, len(lists) - step, 2 * step):
                lists[i] = merge2(lists[i], lists[i + step])
            step *= 2
        return lists[0]
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
```python
from typing import Optional


class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        dummy = ListNode(0, head)
        group_prev = dummy
        while True:
            kth = group_prev  # find the kth node of this group
            for _ in range(k):
                kth = kth.next
                if not kth:
                    return dummy.next
            group_next = kth.next
            prev, cur = group_next, group_prev.next  # reverse the group
            while cur is not group_next:
                cur.next, prev, cur = prev, cur, cur.next
            first = group_prev.next  # becomes this group's tail
            group_prev.next = kth
            group_prev = first
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Recursive
```python
from typing import Optional


class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        cur, count = head, 0
        while cur and count < k:
            cur, count = cur.next, count + 1
        if count < k:
            return head
        prev, node = self.reverseKGroup(cur, k), head  # cur is the (k+1)th node
        for _ in range(k):
            node.next, prev, node = prev, node, node.next
        return prev
```
**Complexity:** Time O(n) · Space O(n/k) recursion

---
## 7. Trees

> All problems here use LeetCode's `TreeNode` (see [Shared Definitions](#shared-definitions)).

### 226. Invert Binary Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/invert-binary-tree/)

**Problem:** Mirror a binary tree by swapping every node's left and right children, then return the root.

**Example:** `[4,2,7,1,3,6,9]` → `[4,7,2,9,6,3,1]`

**Key insight:** Swap the children at every node. Traversal order doesn't matter.

#### Approach 1: Recursive DFS (Optimal)
```python
from typing import Optional


class Solution:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if root:
            root.left, root.right = self.invertTree(root.right), self.invertTree(root.left)
        return root
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: Iterative BFS
```python
from collections import deque
from typing import Optional


class Solution:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        q = deque([root] if root else [])
        while q:
            node = q.popleft()
            node.left, node.right = node.right, node.left
            q.extend(child for child in (node.left, node.right) if child)
        return root
```
**Complexity:** Time O(n) · Space O(w), where w is the maximum width

---

### 104. Maximum Depth of Binary Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/maximum-depth-of-binary-tree/)

**Problem:** Return the number of nodes on the longest path from the root down to a leaf.

**Example:** `[3,9,20,null,null,15,7]` → `3`

#### Approach 1: Recursive DFS
```python
from typing import Optional


class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        return 1 + max(self.maxDepth(root.left), self.maxDepth(root.right)) if root else 0
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: BFS Level Count
```python
from collections import deque
from typing import Optional


class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        q = deque([root] if root else [])
        depth = 0
        while q:
            depth += 1
            for _ in range(len(q)):
                node = q.popleft()
                q.extend(c for c in (node.left, node.right) if c)
        return depth
```
**Complexity:** Time O(n) · Space O(w)

#### Approach 3: Iterative DFS with an Explicit Stack
```python
from typing import Optional


class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        stack, best = [(root, 1)] if root else [], 0
        while stack:
            node, d = stack.pop()
            best = max(best, d)
            stack.extend((c, d + 1) for c in (node.left, node.right) if c)
        return best
```
**Complexity:** Time O(n) · Space O(h)

---

### 543. Diameter of Binary Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/diameter-of-binary-tree/)

**Problem:** Return the length (in edges) of the longest path between any two nodes. The path doesn't have to pass through the root.

**Example:** `[1,2,3,4,5]` → `3` (path 4→2→1→3)

**Key insight:** The longest path through a node is `height(left) + height(right)`. Compute heights bottom-up and track the best sum.

#### Approach: DFS Returning Height (Optimal)
```python
from typing import Optional


class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        best = 0

        def height(node: Optional[TreeNode]) -> int:
            nonlocal best
            if not node:
                return 0
            l, r = height(node.left), height(node.right)
            best = max(best, l + r)
            return 1 + max(l, r)

        height(root)
        return best
```
**Complexity:** Time O(n) · Space O(h)

**Brute force:** At each node, compute the left and right heights separately: O(n²) on a skewed tree.

---

### 110. Balanced Binary Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/balanced-binary-tree/)

**Problem:** A tree is height-balanced if, at every node, the left and right subtree heights differ by at most 1. Return whether the tree is balanced.

**Key insight:** Return height bottom-up, and use `-1` to signal that a subtree is already unbalanced.

#### Approach 1: Top-Down (simple, O(n²))
```python
from typing import Optional


class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        def height(node: Optional[TreeNode]) -> int:
            return 1 + max(height(node.left), height(node.right)) if node else 0

        if not root:
            return True
        return (
            abs(height(root.left) - height(root.right)) <= 1
            and self.isBalanced(root.left)
            and self.isBalanced(root.right)
        )
```
**Complexity:** Time O(n²) worst case · Space O(h)

#### Approach 2: Bottom-Up with Early Exit (Optimal)
```python
from typing import Optional


class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        def check(node: Optional[TreeNode]) -> int:  # height, or -1 if unbalanced
            if not node:
                return 0
            l = check(node.left)
            if l < 0:
                return -1
            r = check(node.right)
            if r < 0 or abs(l - r) > 1:
                return -1
            return 1 + max(l, r)

        return check(root) >= 0
```
**Complexity:** Time O(n) · Space O(h)

---

### 100. Same Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/same-tree/)

**Problem:** Return whether two binary trees have the same structure and the same node values.

#### Approach 1: Recursive (Optimal)
```python
from typing import Optional


class Solution:
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        if not p or not q:
            return p is q  # both None
        return p.val == q.val and self.isSameTree(p.left, q.left) and self.isSameTree(p.right, q.right)
```
**Complexity:** Time O(min(n, m)) · Space O(h)

#### Approach 2: Iterative BFS on Pairs
```python
from collections import deque
from typing import Optional


class Solution:
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        queue = deque([(p, q)])
        while queue:
            a, b = queue.popleft()
            if not a and not b:
                continue
            if not a or not b or a.val != b.val:
                return False
            queue.append((a.left, b.left))
            queue.append((a.right, b.right))
        return True
```
**Complexity:** Time O(n) · Space O(w)

---

### 572. Subtree of Another Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/subtree-of-another-tree/)

**Problem:** Return whether `subRoot` matches, in structure and values, some node of `root` together with all of that node's descendants.

**Key insight:** Run Same Tree at every node (O(m·n)), or serialize both trees and use substring search (O(m+n)).

#### Approach 1: Same-Tree Check at Every Node
```python
from typing import Optional


class Solution:
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        def same(a: Optional[TreeNode], b: Optional[TreeNode]) -> bool:
            if not a or not b:
                return a is b
            return a.val == b.val and same(a.left, b.left) and same(a.right, b.right)

        if not root:
            return subRoot is None
        return same(root, subRoot) or self.isSubtree(root.left, subRoot) or self.isSubtree(root.right, subRoot)
```
**Complexity:** Time O(m · n) · Space O(h)

#### Approach 2: Serialize + Substring Search
Delimiters around each value (`^12` vs `^2`) and explicit null markers prevent false matches.
```python
from typing import Optional


class Solution:
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        def ser(node: Optional[TreeNode]) -> str:
            if not node:
                return "#"
            return f"^{node.val}{ser(node.left)}{ser(node.right)}"

        return ser(subRoot) in ser(root)  # swap in KMP for a guaranteed O(m+n)
```
**Complexity:** Time O(m + n) with KMP; the `in` operator is fast in practice · Space O(m + n)

---

### 235. Lowest Common Ancestor of a Binary Search Tree
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)

**Problem:** Given a BST and two of its nodes `p` and `q`, return their lowest common ancestor: the deepest node that has both as descendants (a node counts as its own descendant).

**Key insight:** If both values are smaller than the current node, go left. If both are larger, go right. Otherwise the current node is where they split, and that's the LCA.

#### Approach 1: Iterative (Optimal)
```python
class Solution:
    def lowestCommonAncestor(self, root: "TreeNode", p: "TreeNode", q: "TreeNode") -> "TreeNode":
        cur = root
        while cur:
            if p.val < cur.val and q.val < cur.val:
                cur = cur.left
            elif p.val > cur.val and q.val > cur.val:
                cur = cur.right
            else:
                return cur  # p and q split here (or one of them is this node)
        return None
```
**Complexity:** Time O(h) · Space O(1)

#### Approach 2: Recursive
```python
class Solution:
    def lowestCommonAncestor(self, root: "TreeNode", p: "TreeNode", q: "TreeNode") -> "TreeNode":
        if p.val < root.val and q.val < root.val:
            return self.lowestCommonAncestor(root.left, p, q)
        if p.val > root.val and q.val > root.val:
            return self.lowestCommonAncestor(root.right, p, q)
        return root
```
**Complexity:** Time O(h) · Space O(h)

---

### 102. Binary Tree Level Order Traversal
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/binary-tree-level-order-traversal/)

**Problem:** Return the node values level by level, left to right.

**Example:** `[3,9,20,null,null,15,7]` → `[[3],[9,20],[15,7]]`

#### Approach 1: BFS with a Queue (Optimal)
```python
from collections import deque
from typing import List, Optional


class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        res = []
        q = deque([root] if root else [])
        while q:
            level = []
            for _ in range(len(q)):
                node = q.popleft()
                level.append(node.val)
                q.extend(c for c in (node.left, node.right) if c)
            res.append(level)
        return res
```
**Complexity:** Time O(n) · Space O(w)

#### Approach 2: DFS with a Depth Index
```python
from typing import List, Optional


class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        res = []

        def dfs(node: Optional[TreeNode], d: int) -> None:
            if not node:
                return
            if d == len(res):
                res.append([])
            res[d].append(node.val)
            dfs(node.left, d + 1)
            dfs(node.right, d + 1)

        dfs(root, 0)
        return res
```
**Complexity:** Time O(n) · Space O(h)

---

### 199. Binary Tree Right Side View
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/binary-tree-right-side-view/)

**Problem:** Imagine standing on the tree's right side. Return the values you can see, from top to bottom.

**Example:** `[1,2,3,null,5,null,4]` → `[1,3,4]`

**Key insight:** The answer is the last node of each BFS level, or the first node reached at each depth in a right-first DFS.

#### Approach 1: BFS, Last Node of Each Level
```python
from collections import deque
from typing import List, Optional


class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        res = []
        q = deque([root] if root else [])
        while q:
            res.append(q[-1].val)  # the last node in the queue is this level's rightmost
            for _ in range(len(q)):
                node = q.popleft()
                q.extend(c for c in (node.left, node.right) if c)
        return res
```
**Complexity:** Time O(n) · Space O(w)

#### Approach 2: Right-First DFS
```python
from typing import List, Optional


class Solution:
    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        res = []

        def dfs(node: Optional[TreeNode], d: int) -> None:
            if not node:
                return
            if d == len(res):
                res.append(node.val)  # first node seen at this depth is the rightmost
            dfs(node.right, d + 1)
            dfs(node.left, d + 1)

        dfs(root, 0)
        return res
```
**Complexity:** Time O(n) · Space O(h)

---

### 1448. Count Good Nodes in Binary Tree
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/count-good-nodes-in-binary-tree/)

**Problem:** A node `X` is *good* if no node on the path from the root to `X` has a value greater than `X`. Count the good nodes.

**Example:** `[3,1,4,3,null,1,5]` → `4`

**Key insight:** Pass the maximum value seen on the path so far down the DFS.

#### Approach 1: DFS with the Path Max (Optimal)
```python
class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        def dfs(node: TreeNode, max_so_far: int) -> int:
            if not node:
                return 0
            good = 1 if node.val >= max_so_far else 0
            m = max(max_so_far, node.val)
            return good + dfs(node.left, m) + dfs(node.right, m)

        return dfs(root, root.val)
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: BFS Carrying (node, max)
```python
from collections import deque


class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        q, count = deque([(root, root.val)]), 0
        while q:
            node, mx = q.popleft()
            if node.val >= mx:
                count += 1
            mx = max(mx, node.val)
            q.extend((c, mx) for c in (node.left, node.right) if c)
        return count
```
**Complexity:** Time O(n) · Space O(w)

---

### 98. Validate Binary Search Tree
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/validate-binary-search-tree/)

**Problem:** Return whether the tree is a valid BST. Every node's left subtree must hold only smaller values, its right subtree only larger values, and both subtrees must themselves be BSTs.

**Key insight:** Comparing a node only with its direct children isn't enough. Pass down a valid `(lo, hi)` range, or check that the in-order traversal is strictly increasing.

#### Approach 1: DFS with Bounds (Optimal)
```python
from typing import Optional


class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        def valid(node: Optional[TreeNode], lo: float, hi: float) -> bool:
            if not node:
                return True
            if not lo < node.val < hi:
                return False
            return valid(node.left, lo, node.val) and valid(node.right, node.val, hi)

        return valid(root, float("-inf"), float("inf"))
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: Iterative In-Order Traversal
```python
from typing import Optional


class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        stack, cur, prev = [], root, None
        while cur or stack:
            while cur:
                stack.append(cur)
                cur = cur.left
            cur = stack.pop()
            if prev is not None and cur.val <= prev:
                return False
            prev = cur.val
            cur = cur.right
        return True
```
**Complexity:** Time O(n) · Space O(h)

---

### 230. Kth Smallest Element in a BST
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/kth-smallest-element-in-a-bst/)

**Problem:** Return the `k`th smallest value (1-indexed) in a BST.

**Key insight:** In-order traversal visits a BST's values in sorted order, so stop at the `k`th node visited.

#### Approach 1: Iterative In-Order (Optimal, stops early)
```python
from typing import Optional


class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        stack, cur = [], root
        while True:
            while cur:
                stack.append(cur)
                cur = cur.left
            cur = stack.pop()
            k -= 1
            if k == 0:
                return cur.val
            cur = cur.right
```
**Complexity:** Time O(h + k) · Space O(h)

#### Approach 2: Generator-Based In-Order
```python
from itertools import islice
from typing import Iterator, Optional


class Solution:
    def kthSmallest(self, root: Optional[TreeNode], k: int) -> int:
        def inorder(node: Optional[TreeNode]) -> Iterator[int]:
            if node:
                yield from inorder(node.left)
                yield node.val
                yield from inorder(node.right)

        return next(islice(inorder(root), k - 1, None))  # lazy: stops after k values
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
```python
from typing import List, Optional


class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        if not preorder:
            return None
        root = TreeNode(preorder[0])
        mid = inorder.index(preorder[0])
        root.left = self.buildTree(preorder[1 : mid + 1], inorder[:mid])
        root.right = self.buildTree(preorder[mid + 1 :], inorder[mid + 1 :])
        return root
```
**Complexity:** Time O(n²) worst case · Space O(n²) from the slices

#### Approach 2: Hash Map of Inorder Indices (Optimal)
```python
from typing import List, Optional


class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> Optional[TreeNode]:
        pos = {v: i for i, v in enumerate(inorder)}
        it = iter(preorder)  # preorder values are consumed in order

        def build(lo: int, hi: int) -> Optional[TreeNode]:  # inorder range [lo, hi)
            if lo >= hi:
                return None
            v = next(it)
            node = TreeNode(v)
            node.left = build(lo, pos[v])
            node.right = build(pos[v] + 1, hi)
            return node

        return build(0, len(inorder))
```
**Complexity:** Time O(n) · Space O(n)

---

### 124. Binary Tree Maximum Path Sum
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/binary-tree-maximum-path-sum/)

**Problem:** A path is any sequence of connected nodes where each node appears at most once. It doesn't have to pass through the root. Return the maximum path sum over all non-empty paths.

**Example:** `[-10,9,20,null,null,15,7]` → `42` (15→20→7)

**Key insight:** At each node, the best path that bends there is `val + max(0, leftGain) + max(0, rightGain)`. Pass up to the parent only `val + max(0, max(leftGain, rightGain))`, since a path can't branch upward.

#### Approach: Post-Order DFS (Optimal)
```python
from typing import Optional


class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        best = float("-inf")

        def gain(node: Optional[TreeNode]) -> int:
            nonlocal best
            if not node:
                return 0
            l = max(0, gain(node.left))
            r = max(0, gain(node.right))
            best = max(best, node.val + l + r)  # best path that bends at this node
            return node.val + max(l, r)         # only one branch can extend upward

        gain(root)
        return best
```
**Complexity:** Time O(n) · Space O(h)

---

### 297. Serialize and Deserialize Binary Tree
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/)

**Problem:** Design `serialize`, which turns a binary tree into a string, and `deserialize`, which turns that string back into the same tree.

**Key insight:** A pre-order traversal with explicit null markers (`N`) is enough to rebuild the tree exactly.

#### Approach 1: Pre-Order DFS
```python
from typing import Optional


class Codec:
    def serialize(self, root: Optional[TreeNode]) -> str:
        out = []

        def dfs(node: Optional[TreeNode]) -> None:
            if not node:
                out.append("N")
                return
            out.append(str(node.val))
            dfs(node.left)
            dfs(node.right)

        dfs(root)
        return ",".join(out)

    def deserialize(self, data: str) -> Optional[TreeNode]:
        vals = iter(data.split(","))

        def build() -> Optional[TreeNode]:
            v = next(vals)
            if v == "N":
                return None
            node = TreeNode(int(v))
            node.left = build()
            node.right = build()
            return node

        return build()
```
**Complexity:** Time O(n) for both · Space O(n)

#### Approach 2: BFS (level order, LeetCode's own format)
```python
from collections import deque
from typing import Optional


class Codec:
    def serialize(self, root: Optional[TreeNode]) -> str:
        out, q = [], deque([root])
        while q:
            node = q.popleft()
            if node:
                out.append(str(node.val))
                q.append(node.left)
                q.append(node.right)
            else:
                out.append("N")
        return ",".join(out)

    def deserialize(self, data: str) -> Optional[TreeNode]:
        vals = data.split(",")
        if vals[0] == "N":
            return None
        root = TreeNode(int(vals[0]))
        q, i = deque([root]), 1
        while q:
            node = q.popleft()
            for side in ("left", "right"):
                if vals[i] != "N":
                    child = TreeNode(int(vals[i]))
                    setattr(node, side, child)
                    q.append(child)
                i += 1
        return root
```
**Complexity:** Time O(n) · Space O(n)

---
## 8. Tries

### 208. Implement Trie (Prefix Tree)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/implement-trie-prefix-tree/)

**Problem:** Implement a trie with `insert(word)`, `search(word)` (is this exact word stored?), and `startsWith(prefix)` (does any stored word begin with this prefix?). Inputs are lowercase letters.

**Key insight:** Each node has up to 26 children plus an end-of-word flag.

#### Approach 1: Node Class with a Children Dict (Optimal)
```python
from typing import Optional


class TrieNode:
    __slots__ = ("children", "is_end")

    def __init__(self):
        self.children = {}
        self.is_end = False


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        node = self.root
        for c in word:
            node = node.children.setdefault(c, TrieNode())
        node.is_end = True

    def _find(self, s: str) -> Optional[TrieNode]:
        node = self.root
        for c in s:
            node = node.children.get(c)
            if node is None:
                return None
        return node

    def search(self, word: str) -> bool:
        node = self._find(word)
        return node is not None and node.is_end

    def startsWith(self, prefix: str) -> bool:
        return self._find(prefix) is not None
```
**Complexity:** Time O(L) per operation · Space O(total characters)

#### Approach 2: Nested Dicts with an End Marker (compact)
```python
class Trie:
    END = "$"  # can't collide with lowercase letters

    def __init__(self):
        self.root = {}

    def insert(self, word: str) -> None:
        node = self.root
        for c in word:
            node = node.setdefault(c, {})
        node[self.END] = True

    def search(self, word: str) -> bool:
        node = self.root
        for c in word:
            if c not in node:
                return False
            node = node[c]
        return self.END in node

    def startsWith(self, prefix: str) -> bool:
        node = self.root
        for c in prefix:
            if c not in node:
                return False
            node = node[c]
        return True
```
**Complexity:** Time O(L) per operation · Space O(total characters)

---

### 211. Design Add and Search Words Data Structure
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/design-add-and-search-words-data-structure/)

**Problem:** Support `addWord(word)` and `search(word)`. The search pattern may contain `.`, which matches any single letter.

**Example:** add `bad`, `dad`, `mad`; `search("pad")` → `False`, `search(".ad")` → `True`, `search("b..")` → `True`

**Key insight:** Use a trie. On `.`, run DFS into every child.

#### Approach 1: Trie + DFS (Optimal)
```python
class WordDictionary:
    def __init__(self):
        self.root = {}

    def addWord(self, word: str) -> None:
        node = self.root
        for c in word:
            node = node.setdefault(c, {})
        node["$"] = True

    def search(self, word: str) -> bool:
        def dfs(node: dict, i: int) -> bool:
            if i == len(word):
                return "$" in node
            c = word[i]
            if c == ".":
                return any(dfs(child, i + 1) for key, child in node.items() if key != "$")
            return c in node and dfs(node[c], i + 1)

        return dfs(self.root, 0)
```
**Complexity:** `addWord` O(L) · `search` O(L) without dots, O(26^L) worst case with all dots · Space O(total characters)

#### Approach 2: Bucket Words by Length (simple)
```python
from collections import defaultdict


class WordDictionary:
    def __init__(self):
        self.by_len = defaultdict(set)

    def addWord(self, word: str) -> None:
        self.by_len[len(word)].add(word)

    def search(self, word: str) -> bool:
        if "." not in word:
            return word in self.by_len[len(word)]
        return any(
            all(p == "." or p == c for p, c in zip(word, w))
            for w in self.by_len[len(word)]
        )
```
**Complexity:** `addWord` O(L) · `search` O(1) average without dots, O(words × L) with dots · Space O(total characters)

---

### 212. Word Search II
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/word-search-ii/)

**Problem:** Given an `m × n` letter grid and a list of `words`, return every word that can be traced through horizontally or vertically adjacent cells, without reusing a cell within a word.

**Example:** `board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]` → `["eat","oath"]`

**Key insight:** Build a trie from all the words and run one DFS per cell, descending the grid and the trie together. Prune trie nodes once their words are found.

#### Approach 1: Trie + Backtracking (no pruning)
```python
from typing import List


class Solution:
    def findWords(self, board: List[List[str]], words: List[str]) -> List[str]:
        root = {}
        for w in words:
            node = root
            for c in w:
                node = node.setdefault(c, {})
            node["$"] = w  # store the word at its end node
        m, n = len(board), len(board[0])
        res = []

        def dfs(r: int, c: int, parent: dict) -> None:
            ch = board[r][c]
            node = parent.get(ch)
            if node is None:
                return
            if "$" in node:
                res.append(node.pop("$"))  # pop: report each word once
            board[r][c] = "#"
            for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                if 0 <= nr < m and 0 <= nc < n and board[nr][nc] != "#":
                    dfs(nr, nc, node)
            board[r][c] = ch

        for r in range(m):
            for c in range(n):
                dfs(r, c, root)
        return res
```
**Complexity:** Time O(m · n · 4 · 3^(L-1)), where L is the maximum word length · Space O(total characters in `words`)

#### Approach 2: Trie + Backtracking with Leaf Pruning (Optimal)
```python
from typing import List


class Solution:
    def findWords(self, board: List[List[str]], words: List[str]) -> List[str]:
        root = {}
        for w in words:
            node = root
            for c in w:
                node = node.setdefault(c, {})
            node["$"] = w
        m, n = len(board), len(board[0])
        res = []

        def dfs(r: int, c: int, parent: dict) -> None:
            ch = board[r][c]
            node = parent[ch]
            if "$" in node:
                res.append(node.pop("$"))
            board[r][c] = "#"
            for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                if 0 <= nr < m and 0 <= nc < n and board[nr][nc] in node:
                    dfs(nr, nc, node)
            board[r][c] = ch
            if not node:  # leaf with no words left below: prune it
                parent.pop(ch)

        for r in range(m):
            for c in range(n):
                if board[r][c] in root:
                    dfs(r, c, root)
        return res
```
**Complexity:** Same worst case. Removing leaves cuts the search space sharply in practice; this is the version to use in interviews.

> `board[nr][nc] in node` also safely rejects `"#"`, because no trie key is `"#"`. The only special key is `"$"`, and it never appears on the board.

**Brute force:** Run LC 79 (Word Search) once per word: O(W · m · n · 4^L). Much slower when there are many words.

---
## 9. Heap / Priority Queue

> `heapq` works on plain lists and is a **min-heap**. For a max-heap, push negated values. When items aren't comparable (like `ListNode`), push `(priority, unique_counter, item)` tuples.

### 703. Kth Largest Element in a Stream
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/kth-largest-element-in-a-stream/)

**Problem:** Design a class built from `k` and an initial array `nums`. Each call to `add(val)` adds a value to the stream and returns the current `k`th largest element.

**Example:** `k = 3, nums = [4,5,8,2]`; `add(3)` → 4, `add(5)` → 5, `add(10)` → 5

**Key insight:** Keep a min-heap of size `k`. Its top is the `k`th largest element.

#### Approach: Min-Heap of Size k (Optimal)
```python
import heapq
from typing import List


class KthLargest:
    def __init__(self, k: int, nums: List[int]):
        self.k = k
        self.heap = nums[:]
        heapq.heapify(self.heap)
        while len(self.heap) > k:
            heapq.heappop(self.heap)

    def add(self, val: int) -> int:
        if len(self.heap) < self.k:
            heapq.heappush(self.heap, val)
        elif val > self.heap[0]:
            heapq.heapreplace(self.heap, val)  # pop + push in one O(log k) step
        return self.heap[0]
```
**Complexity:** `add` O(log k) · `__init__` O(n log n) worst case · Space O(k)

**Alternative:** Keep a sorted list and insert with `bisect.insort`. `add` is then O(n), from shifting elements.

---

### 1046. Last Stone Weight
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/last-stone-weight/)

**Problem:** Each turn, smash the two heaviest stones `x ≤ y`. If they're equal, both are destroyed. Otherwise a stone of weight `y - x` remains. Return the weight of the last stone, or `0` if none are left.

**Example:** `[2,7,4,1,8,1]` → `1`

#### Approach 1: Max-Heap via Negation (Optimal)
```python
import heapq
from typing import List


class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        heap = [-s for s in stones]
        heapq.heapify(heap)
        while len(heap) > 1:
            y, x = -heapq.heappop(heap), -heapq.heappop(heap)
            if y != x:
                heapq.heappush(heap, -(y - x))
        return -heap[0] if heap else 0
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Bucket Sort (weights ≤ 1000)
```python
from typing import List


class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        max_w = max(stones)
        bucket = [0] * (max_w + 1)
        for s in stones:
            bucket[s] += 1
        biggest, cur = 0, max_w
        while cur > 0:
            if bucket[cur] == 0:
                cur -= 1
            elif biggest == 0:
                bucket[cur] %= 2  # equal pairs destroy each other
                if bucket[cur]:
                    biggest = cur
                cur -= 1
            else:
                bucket[cur] -= 1
                if biggest - cur <= cur:
                    bucket[biggest - cur] += 1
                    biggest = 0
                else:
                    biggest -= cur
        return biggest
```
**Complexity:** Time O(n + W), where W is the maximum weight · Space O(W)

---

### 973. K Closest Points to Origin
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/k-closest-points-to-origin/)

**Problem:** Given `points[i] = [x, y]`, return the `k` points closest to the origin by Euclidean distance, in any order.

**Example:** `points = [[1,3],[-2,2]], k = 1` → `[[-2,2]]`

**Key insight:** Compare squared distances (no square root needed). Keep a max-heap of size `k`, or use quickselect.

#### Approach 1: Sort
```python
from typing import List


class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        return sorted(points, key=lambda p: p[0] * p[0] + p[1] * p[1])[:k]
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: `heapq.nsmallest` (heap of size k)
```python
import heapq
from typing import List


class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        return heapq.nsmallest(k, points, key=lambda p: p[0] * p[0] + p[1] * p[1])
```
**Complexity:** Time O(n log k) · Space O(k)

#### Approach 3: Quickselect (Optimal on average)
```python
import random
from typing import List


class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        def dist(p: List[int]) -> int:
            return p[0] * p[0] + p[1] * p[1]

        lo, hi = 0, len(points) - 1
        while lo < hi:
            p = random.randint(lo, hi)  # random pivot to the end
            points[p], points[hi] = points[hi], points[p]
            pivot, store = dist(points[hi]), lo
            for i in range(lo, hi):
                if dist(points[i]) < pivot:
                    points[i], points[store] = points[store], points[i]
                    store += 1
            points[store], points[hi] = points[hi], points[store]
            if store == k - 1:
                break
            if store < k - 1:
                lo = store + 1
            else:
                hi = store - 1
        return points[:k]
```
**Complexity:** Time O(n) average, O(n²) worst case · Space O(1)

---

### 215. Kth Largest Element in an Array
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/kth-largest-element-in-an-array/)

**Problem:** Return the `k`th largest element in an unsorted array (in sorted order, not the `k`th distinct value).

**Example:** `nums = [3,2,1,5,6,4], k = 2` → `5`

#### Approach 1: Sort
```python
from typing import List


class Solution:
    def findKthLargest(self, nums: List[int], k: int) -> int:
        return sorted(nums)[-k]
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Min-Heap of Size k
```python
import heapq
from typing import List


class Solution:
    def findKthLargest(self, nums: List[int], k: int) -> int:
        heap = nums[:k]
        heapq.heapify(heap)
        for n in nums[k:]:
            if n > heap[0]:
                heapq.heapreplace(heap, n)
        return heap[0]
```
**Complexity:** Time O(n log k) · Space O(k)

**Built-in shortcut:** `heapq.nlargest(k, nums)[-1]` does the same thing.

#### Approach 3: Quickselect with Three-Way Partition (Optimal on average)
A three-way partition handles many duplicates, and the random pivot avoids worst-case inputs.
```python
import random
from typing import List


class Solution:
    def findKthLargest(self, nums: List[int], k: int) -> int:
        while True:
            pivot = random.choice(nums)
            bigger = [x for x in nums if x > pivot]
            equal_count = sum(1 for x in nums if x == pivot)
            if k <= len(bigger):
                nums = bigger
            elif k <= len(bigger) + equal_count:
                return pivot
            else:
                k -= len(bigger) + equal_count
                nums = [x for x in nums if x < pivot]
```
**Complexity:** Time O(n) average · Space O(n) for the filtered lists. An in-place version trades readability for O(1) extra space.

#### Approach 4: Counting Sort (values in [-10⁴, 10⁴])
```python
from typing import List


class Solution:
    def findKthLargest(self, nums: List[int], k: int) -> int:
        off = 10_000
        cnt = [0] * (2 * off + 1)
        for n in nums:
            cnt[n + off] += 1
        for v in range(2 * off, -1, -1):
            k -= cnt[v]
            if k <= 0:
                return v - off
        return -1
```
**Complexity:** Time O(n + R), where R is the value range · Space O(R)

---

### 621. Task Scheduler
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/task-scheduler/)

**Problem:** You're given a list of CPU tasks labeled `A`–`Z` and a cooldown `n`: two identical tasks must be separated by at least `n` time units. The CPU can sit idle. Return the minimum number of time units needed to finish every task.

**Example:** `tasks = ["A","A","A","B","B","B"], n = 2` → `8` (A B idle A B idle A B)

**Key insight:** The most frequent task sets the frame: `(maxFreq-1) × (n+1) + countOfMaxFreqTasks`. The answer is never less than `len(tasks)`.

#### Approach 1: Max-Heap + Cooldown Queue (simulation)
```python
import heapq
from collections import Counter, deque
from typing import List


class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        heap = [-c for c in Counter(tasks).values()]
        heapq.heapify(heap)
        cooldown = deque()  # (negative remaining count, time when ready)
        time = 0
        while heap or cooldown:
            time += 1
            if heap:
                c = heapq.heappop(heap) + 1  # one fewer remaining (counts are negative)
                if c:
                    cooldown.append((c, time + n))
            if cooldown and cooldown[0][1] == time:
                heapq.heappush(heap, cooldown.popleft()[0])
        return time
```
**Complexity:** Time O(T · log 26) = O(T), where T is the total time · Space O(26)

#### Approach 2: Math Formula (Optimal)
```python
from collections import Counter
from typing import List


class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        freq = Counter(tasks).values()
        max_f = max(freq)
        max_count = sum(1 for f in freq if f == max_f)
        return max(len(tasks), (max_f - 1) * (n + 1) + max_count)
```
**Complexity:** Time O(len(tasks)) · Space O(1)

---

### 355. Design Twitter
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/design-twitter/)

**Problem:** Design a simplified Twitter with `postTweet(userId, tweetId)`, `getNewsFeed(userId)` (the 10 most recent tweet IDs from the user and the people they follow, newest first), `follow`, and `unfollow`.

**Key insight:** The news feed is a merge of k sorted lists. Each user's tweets are already in time order, so a heap seeded with each followee's newest tweet works.

#### Approach: Hash Maps + Heap Merge (Optimal)
```python
import heapq
from collections import defaultdict
from typing import List


class Twitter:
    def __init__(self):
        self.clock = 0
        self.tweets = defaultdict(list)   # user -> [(time, tweet_id)] in time order
        self.follows = defaultdict(set)

    def postTweet(self, userId: int, tweetId: int) -> None:
        self.clock += 1
        self.tweets[userId].append((self.clock, tweetId))

    def getNewsFeed(self, userId: int) -> List[int]:
        heap = []  # (-time, tweet_id, user, index into that user's list)
        for u in self.follows[userId] | {userId}:
            if self.tweets[u]:
                i = len(self.tweets[u]) - 1
                t, tid = self.tweets[u][i]
                heap.append((-t, tid, u, i))
        heapq.heapify(heap)
        res = []
        while heap and len(res) < 10:
            _, tid, u, i = heapq.heappop(heap)
            res.append(tid)
            if i > 0:
                t, nid = self.tweets[u][i - 1]
                heapq.heappush(heap, (-t, nid, u, i - 1))
        return res

    def follow(self, followerId: int, followeeId: int) -> None:
        self.follows[followerId].add(followeeId)

    def unfollow(self, followerId: int, followeeId: int) -> None:
        self.follows[followerId].discard(followeeId)
```
**Complexity:** `postTweet`, `follow`, `unfollow` O(1) · `getNewsFeed` O(F + 10 log F) for F followees · Space O(users + tweets + follows)

**Simpler alternative:** `heapq.merge(*(reversed(self.tweets[u]) for u in users), reverse=True)` lazily merges the users' newest-first streams. Take 10 with `itertools.islice`.

---

### 295. Find Median from Data Stream
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/find-median-from-data-stream/)

**Problem:** Design `MedianFinder` with `addNum(num)` and `findMedian()`, which returns the median of every number added so far.

**Key insight:** Keep two heaps: a max-heap for the smaller half and a min-heap for the larger half. Keep their sizes within one of each other, so the median is on top.

#### Approach 1: Two Heaps (Optimal)
```python
import heapq


class MedianFinder:
    def __init__(self):
        self.low = []   # max-heap (negated): smaller half
        self.high = []  # min-heap: larger half

    def addNum(self, num: int) -> None:
        heapq.heappush(self.low, -num)
        heapq.heappush(self.high, -heapq.heappop(self.low))  # move low's max to high
        if len(self.high) > len(self.low):
            heapq.heappush(self.low, -heapq.heappop(self.high))

    def findMedian(self) -> float:
        if len(self.low) > len(self.high):
            return float(-self.low[0])
        return (-self.low[0] + self.high[0]) / 2
```
**Complexity:** `addNum` O(log n) · `findMedian` O(1) · Space O(n)

#### Approach 2: Sorted List with `bisect.insort` (simple)
```python
import bisect


class MedianFinder:
    def __init__(self):
        self.data = []

    def addNum(self, num: int) -> None:
        bisect.insort(self.data, num)

    def findMedian(self) -> float:
        n = len(self.data)
        if n % 2:
            return float(self.data[n // 2])
        return (self.data[n // 2 - 1] + self.data[n // 2]) / 2
```
**Complexity:** `addNum` O(n) for the list shift (it's a fast `memmove`) · `findMedian` O(1)

**Follow-up (all values in [0, 100]):** Keep a 101-bucket count array and walk it to find the median, which is O(100) per query.

---
## 10. Backtracking

> **Template:** choose → recurse → un-choose. Save a **copy** of the path (`res.append(path[:])`). Appending `path` itself stores a reference, and later changes would overwrite results you've already stored. `itertools` has ready-made generators for many of these; they're shown as alternatives.

### 78. Subsets
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/subsets/)

**Problem:** Given an array of unique integers, return every possible subset (the power set), with no duplicates.

**Example:** `[1,2,3]` → `[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]`

#### Approach 1: Backtracking (include or exclude)
```python
from typing import List


class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        res, path = [], []

        def bt(i: int) -> None:
            if i == len(nums):
                res.append(path[:])
                return
            path.append(nums[i])  # include
            bt(i + 1)
            path.pop()            # exclude
            bt(i + 1)

        bt(0)
        return res
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n) recursion

#### Approach 2: Iterative Cascading
```python
from typing import List


class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        res = [[]]
        for n in nums:
            res += [s + [n] for s in res]
        return res
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n · 2ⁿ) for the output

#### Approach 3: `itertools.combinations`
```python
from itertools import chain, combinations
from typing import List


class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        return [list(c) for c in chain.from_iterable(combinations(nums, r) for r in range(len(nums) + 1))]
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n · 2ⁿ)

---

### 39. Combination Sum
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/combination-sum/)

**Problem:** Given distinct `candidates` and a `target`, return every unique combination that sums to `target`. Each number may be used any number of times.

**Example:** `candidates = [2,3,6,7], target = 7` → `[[2,2,3],[7]]`

**Key insight:** Recurse from the same index `i` to allow reuse, and pass `i` forward so the same combination isn't produced in a different order.

#### Approach 1: Backtracking with Sorting and Pruning (Optimal)
```python
from typing import List


class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        candidates.sort()
        res, path = [], []

        def bt(start: int, remain: int) -> None:
            if remain == 0:
                res.append(path[:])
                return
            for i in range(start, len(candidates)):
                c = candidates[i]
                if c > remain:
                    break  # sorted, so every later candidate is too big
                path.append(c)
                bt(i, remain - c)  # i, not i+1: reuse is allowed
                path.pop()

        bt(0, target)
        return res
```
**Complexity:** Time O(N^(T/M + 1)), where T = target and M = smallest candidate · Space O(T/M) recursion

#### Approach 2: DP Building Combinations (unbounded knapsack style)
```python
from typing import List


class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        dp = [[] for _ in range(target + 1)]
        dp[0] = [[]]
        for c in candidates:  # outer loop over coins means no permutations
            for t in range(c, target + 1):
                dp[t] += [comb + [c] for comb in dp[t - c]]
        return dp[target]
```
**Complexity:** Exponential in the output size · Space proportional to all partial combinations

---

### 46. Permutations
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/permutations/)

**Problem:** Given an array of distinct integers, return every permutation.

**Example:** `[1,2,3]` → `[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]`

#### Approach 1: Backtracking with a Used Array
```python
from typing import List


class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        res, path, used = [], [], [False] * len(nums)

        def bt() -> None:
            if len(path) == len(nums):
                res.append(path[:])
                return
            for i, n in enumerate(nums):
                if used[i]:
                    continue
                used[i] = True
                path.append(n)
                bt()
                path.pop()
                used[i] = False

        bt()
        return res
```
**Complexity:** Time O(n · n!) · Space O(n)

#### Approach 2: In-Place Swapping
```python
from typing import List


class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        res = []

        def bt(k: int) -> None:
            if k == len(nums):
                res.append(nums[:])
                return
            for i in range(k, len(nums)):
                nums[k], nums[i] = nums[i], nums[k]
                bt(k + 1)
                nums[k], nums[i] = nums[i], nums[k]

        bt(0)
        return res
```
**Complexity:** Time O(n · n!) · Space O(n)

#### Approach 3: `itertools.permutations`
```python
from itertools import permutations
from typing import List


class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        return [list(p) for p in permutations(nums)]
```
**Complexity:** Time O(n · n!) · Space O(n · n!)

---

### 90. Subsets II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/subsets-ii/)

**Problem:** Same as Subsets, but `nums` may contain duplicates. The result must not contain duplicate subsets.

**Example:** `[1,2,2]` → `[[],[1],[1,2],[1,2,2],[2],[2,2]]`

**Key insight:** Sort first. At each recursion depth, skip a value equal to the previous one at that same depth.

#### Approach 1: Backtracking with Duplicate Skipping (Optimal)
```python
from typing import List


class Solution:
    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        res, path = [], []

        def bt(start: int) -> None:
            res.append(path[:])
            for i in range(start, len(nums)):
                if i > start and nums[i] == nums[i - 1]:
                    continue  # skip duplicates at the same depth
                path.append(nums[i])
                bt(i + 1)
                path.pop()

        bt(0)
        return res
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n)

#### Approach 2: Iterative (extend only the subsets added last round when the value repeats)
```python
from typing import List


class Solution:
    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        res, prev_size = [[]], 0
        for i, n in enumerate(nums):
            start = prev_size if i > 0 and n == nums[i - 1] else 0
            prev_size = len(res)
            res += [s + [n] for s in res[start:]]
        return res
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n · 2ⁿ) for the output

---

### 40. Combination Sum II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/combination-sum-ii/)

**Problem:** Given `candidates` (which may contain duplicates) and a `target`, return every unique combination that sums to `target`. Each element may be used at most once.

**Example:** `candidates = [10,1,2,7,6,1,5], target = 8` → `[[1,1,6],[1,2,5],[1,7],[2,6]]`

#### Approach 1: Sort + Backtracking with Duplicate Skipping (Optimal)
```python
from typing import List


class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        candidates.sort()
        res, path = [], []

        def bt(start: int, remain: int) -> None:
            if remain == 0:
                res.append(path[:])
                return
            for i in range(start, len(candidates)):
                if i > start and candidates[i] == candidates[i - 1]:
                    continue
                if candidates[i] > remain:
                    break
                path.append(candidates[i])
                bt(i + 1, remain - candidates[i])
                path.pop()

        bt(0, target)
        return res
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n)

#### Approach 2: Backtracking over (value, count) Pairs
Grouping equal values means duplicates are never generated in the first place.
```python
from collections import Counter
from typing import List


class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        items = sorted(Counter(candidates).items())  # (value, count)
        res, path = [], []

        def bt(i: int, remain: int) -> None:
            if remain == 0:
                res.append(path[:])
                return
            if i == len(items) or items[i][0] > remain:
                return
            v, cnt = items[i]
            for use in range(min(cnt, remain // v), -1, -1):  # use this value 0..cnt times
                path.extend([v] * use)
                bt(i + 1, remain - v * use)
                del path[len(path) - use :]

        bt(0, target)
        return res
```
**Complexity:** Time O(output · n) in practice · Space O(n)

---

### 79. Word Search
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/word-search/)

**Problem:** Given an `m × n` grid of letters and a `word`, return whether the word can be traced through horizontally or vertically adjacent cells, without reusing a cell.

**Example:** `board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"` → `True`

**Key insight:** Run DFS from each cell, marking cells as visited in place, and undo the mark when backtracking.

#### Approach: DFS Backtracking (Optimal)
```python
from typing import List


class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        m, n = len(board), len(board[0])

        def dfs(r: int, c: int, i: int) -> bool:
            if i == len(word):
                return True
            if not (0 <= r < m and 0 <= c < n) or board[r][c] != word[i]:
                return False
            board[r][c] = "#"
            found = dfs(r + 1, c, i + 1) or dfs(r - 1, c, i + 1) or dfs(r, c + 1, i + 1) or dfs(r, c - 1, i + 1)
            board[r][c] = word[i]
            return found

        return any(dfs(r, c, 0) for r in range(m) for c in range(n))
```
**Complexity:** Time O(m · n · 3^L) · Space O(L) recursion

**Pruning tricks:** Return `False` early if the board has fewer of any letter than the word needs (`Counter(word) - Counter(chain(*board))` is non-empty). If the word's last letter is rarer on the board than its first letter, search for the reversed word instead.

---

### 131. Palindrome Partitioning
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/palindrome-partitioning/)

**Problem:** Split `s` so that every piece is a palindrome, and return every such partition.

**Example:** `"aab"` → `[["a","a","b"],["aa","b"]]`

#### Approach 1: Backtracking with an On-the-Fly Palindrome Check
```python
from typing import List


class Solution:
    def partition(self, s: str) -> List[List[str]]:
        res, path = [], []

        def bt(start: int) -> None:
            if start == len(s):
                res.append(path[:])
                return
            for end in range(start + 1, len(s) + 1):
                piece = s[start:end]
                if piece == piece[::-1]:
                    path.append(piece)
                    bt(end)
                    path.pop()

        bt(0)
        return res
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n)

#### Approach 2: Precomputed Palindrome DP Table + Memoized Partitions
```python
from functools import cache
from typing import List


class Solution:
    def partition(self, s: str) -> List[List[str]]:
        n = len(s)
        pal = [[False] * n for _ in range(n)]
        for i in range(n - 1, -1, -1):
            for j in range(i, n):
                pal[i][j] = s[i] == s[j] and (j - i < 2 or pal[i + 1][j - 1])

        @cache
        def parts(start: int) -> List[List[str]]:  # every partition of s[start:]
            if start == n:
                return [[]]
            return [[s[start : end + 1]] + rest for end in range(start, n) if pal[start][end] for rest in parts(end + 1)]

        return parts(0)
```
**Complexity:** Time O(n · 2ⁿ), with each palindrome check O(1) · Space O(n²) plus the output

---

### 17. Letter Combinations of a Phone Number
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/letter-combinations-of-a-phone-number/)

**Problem:** Given a string of digits `2`–`9`, return every letter combination they could spell on a phone keypad.

**Example:** `"23"` → `["ad","ae","af","bd","be","bf","cd","ce","cf"]`

#### Approach 1: Backtracking
```python
from typing import List


class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        if not digits:
            return []
        keys = {"2": "abc", "3": "def", "4": "ghi", "5": "jkl", "6": "mno", "7": "pqrs", "8": "tuv", "9": "wxyz"}
        res, path = [], []

        def bt(i: int) -> None:
            if i == len(digits):
                res.append("".join(path))
                return
            for c in keys[digits[i]]:
                path.append(c)
                bt(i + 1)
                path.pop()

        bt(0)
        return res
```
**Complexity:** Time O(n · 4ⁿ) · Space O(n)

#### Approach 2: `itertools.product`
```python
from itertools import product
from typing import List


class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        if not digits:
            return []
        keys = {"2": "abc", "3": "def", "4": "ghi", "5": "jkl", "6": "mno", "7": "pqrs", "8": "tuv", "9": "wxyz"}
        return ["".join(p) for p in product(*(keys[d] for d in digits))]
```
**Complexity:** Time O(n · 4ⁿ) · Space O(n · 4ⁿ)

---

### 51. N-Queens
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/n-queens/)

**Problem:** Place `n` queens on an `n × n` board so that no two attack each other. Return every distinct board, written with `'Q'` for a queen and `'.'` for an empty square.

**Example:** `n = 4` → `[[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]`

**Key insight:** Place one queen per row. A column, a diagonal (`r-c`), or an anti-diagonal (`r+c`) can each hold at most one queen.

#### Approach 1: Backtracking with Sets
```python
from typing import List


class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        res, queens = [], []
        cols, diag, anti = set(), set(), set()

        def bt(r: int) -> None:
            if r == n:
                res.append(["." * c + "Q" + "." * (n - c - 1) for c in queens])
                return
            for c in range(n):
                if c in cols or r - c in diag or r + c in anti:
                    continue
                cols.add(c)
                diag.add(r - c)
                anti.add(r + c)
                queens.append(c)
                bt(r + 1)
                queens.pop()
                cols.remove(c)
                diag.remove(r - c)
                anti.remove(r + c)

        bt(0)
        return res
```
**Complexity:** Time O(n!) · Space O(n)

#### Approach 2: Bitmask Backtracking (fastest)
```python
from typing import List


class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        res, queens, full = [], [], (1 << n) - 1

        def bt(cols: int, d1: int, d2: int) -> None:
            if len(queens) == n:
                res.append(["." * c + "Q" + "." * (n - c - 1) for c in queens])
                return
            avail = full & ~(cols | d1 | d2)
            while avail:
                bit = avail & -avail  # lowest set bit
                avail ^= bit
                queens.append(bit.bit_length() - 1)
                bt(cols | bit, ((d1 | bit) << 1) & full, (d2 | bit) >> 1)
                queens.pop()

        bt(0, 0, 0)
        return res
```
**Complexity:** Time O(n!) with a much smaller constant · Space O(n)

---
## 11. Graphs

> Recursive DFS on a large grid (for example 300 × 300 of land) can exceed Python's default recursion limit of 1000. The BFS or explicit-stack versions avoid that. Alternatively, call `sys.setrecursionlimit(10**6)` at the top of the solution.

### 200. Number of Islands
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/number-of-islands/)

**Problem:** Given an `m × n` grid of `'1'` (land) and `'0'` (water), count the islands. An island is a group of land cells connected horizontally or vertically.

**Example:** `[["1","1","0"],["0","1","0"],["0","0","1"]]` → `2`

**Key insight:** Each time you find unvisited land, count one island and flood-fill it so none of its cells are counted again.

#### Approach 1: DFS Flood Fill
```python
import sys
from typing import List


class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        sys.setrecursionlimit(10**6)
        m, n = len(grid), len(grid[0])

        def sink(r: int, c: int) -> None:
            if not (0 <= r < m and 0 <= c < n) or grid[r][c] != "1":
                return
            grid[r][c] = "0"
            sink(r + 1, c)
            sink(r - 1, c)
            sink(r, c + 1)
            sink(r, c - 1)

        count = 0
        for r in range(m):
            for c in range(n):
                if grid[r][c] == "1":
                    count += 1
                    sink(r, c)
        return count
```
**Complexity:** Time O(m · n) · Space O(m · n) recursion in the worst case

#### Approach 2: BFS
```python
from collections import deque
from typing import List


class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        m, n = len(grid), len(grid[0])
        count = 0
        for r in range(m):
            for c in range(n):
                if grid[r][c] != "1":
                    continue
                count += 1
                grid[r][c] = "0"
                q = deque([(r, c)])
                while q:
                    cr, cc = q.popleft()
                    for nr, nc in ((cr + 1, cc), (cr - 1, cc), (cr, cc + 1), (cr, cc - 1)):
                        if 0 <= nr < m and 0 <= nc < n and grid[nr][nc] == "1":
                            grid[nr][nc] = "0"
                            q.append((nr, nc))
        return count
```
**Complexity:** Time O(m · n) · Space O(min(m, n)) for the queue

#### Approach 3: Union-Find
```python
from typing import List


class Solution:
    def numIslands(self, grid: List[List[str]]) -> int:
        m, n = len(grid), len(grid[0])
        parent = list(range(m * n))
        count = sum(row.count("1") for row in grid)

        def find(x: int) -> int:
            while parent[x] != x:
                parent[x] = parent[parent[x]]  # path halving
                x = parent[x]
            return x

        for r in range(m):
            for c in range(n):
                if grid[r][c] != "1":
                    continue
                for nr, nc in ((r + 1, c), (r, c + 1)):
                    if nr < m and nc < n and grid[nr][nc] == "1":
                        a, b = find(r * n + c), find(nr * n + nc)
                        if a != b:
                            parent[a] = b
                            count -= 1
        return count
```
**Complexity:** Time O(m · n · α(mn)) · Space O(m · n)

---

### 133. Clone Graph
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/clone-graph/)

**Problem:** Given a reference to one node in a connected undirected graph, return a deep copy of the whole graph. Each node has a `val` and a list of `neighbors`.

**Key insight:** A map from original node to clone serves as the visited set and also handles cycles.

> LeetCode provides the `Node` class below for this problem. It's included so the snippet runs on its own.

#### Approach 1: DFS + Hash Map
```python
from typing import Optional


class Node:
    def __init__(self, val=0, neighbors=None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []


class Solution:
    def cloneGraph(self, node: Optional["Node"]) -> Optional["Node"]:
        clones = {}

        def dfs(n: "Node") -> "Node":
            if n in clones:
                return clones[n]
            copy = Node(n.val)
            clones[n] = copy  # register before recursing, so cycles terminate
            copy.neighbors = [dfs(nb) for nb in n.neighbors]
            return copy

        return dfs(node) if node else None
```
**Complexity:** Time O(V + E) · Space O(V)

#### Approach 2: BFS + Hash Map
```python
from collections import deque
from typing import Optional


class Node:
    def __init__(self, val=0, neighbors=None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []


class Solution:
    def cloneGraph(self, node: Optional["Node"]) -> Optional["Node"]:
        if not node:
            return None
        clones = {node: Node(node.val)}
        q = deque([node])
        while q:
            cur = q.popleft()
            for nb in cur.neighbors:
                if nb not in clones:
                    clones[nb] = Node(nb.val)
                    q.append(nb)
                clones[cur].neighbors.append(clones[nb])
        return clones[node]
```
**Complexity:** Time O(V + E) · Space O(V)

---

### 695. Max Area of Island
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/max-area-of-island/)

**Problem:** In a binary grid where `1` is land, return the size of the largest island (4-directionally connected), or `0` if there is no land.

#### Approach 1: DFS Returning Area
```python
import sys
from typing import List


class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        sys.setrecursionlimit(10**6)
        m, n = len(grid), len(grid[0])

        def dfs(r: int, c: int) -> int:
            if not (0 <= r < m and 0 <= c < n) or grid[r][c] == 0:
                return 0
            grid[r][c] = 0
            return 1 + dfs(r + 1, c) + dfs(r - 1, c) + dfs(r, c + 1) + dfs(r, c - 1)

        return max(dfs(r, c) for r in range(m) for c in range(n))
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Iterative DFS with a Stack
```python
from typing import List


class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        best = 0
        for r in range(m):
            for c in range(n):
                if not grid[r][c]:
                    continue
                grid[r][c] = 0
                stack, area = [(r, c)], 0
                while stack:
                    cr, cc = stack.pop()
                    area += 1
                    for nr, nc in ((cr + 1, cc), (cr - 1, cc), (cr, cc + 1), (cr, cc - 1)):
                        if 0 <= nr < m and 0 <= nc < n and grid[nr][nc]:
                            grid[nr][nc] = 0
                            stack.append((nr, nc))
                best = max(best, area)
        return best
```
**Complexity:** Time O(m · n) · Space O(m · n)

---

### 417. Pacific Atlantic Water Flow
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/pacific-atlantic-water-flow/)

**Problem:** `heights` is an island. The Pacific Ocean touches its top and left edges, and the Atlantic touches its bottom and right edges. Water flows from a cell to a neighbor of equal or lower height. Return every cell from which water can reach both oceans.

**Key insight:** Search in reverse, from each ocean's border moving to cells of equal or greater height. The answer is the intersection of the two reachable sets.

#### Approach 1: Reverse DFS from Both Oceans (Optimal)
```python
import sys
from typing import List


class Solution:
    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:
        sys.setrecursionlimit(10**6)
        m, n = len(heights), len(heights[0])
        pac, atl = set(), set()

        def dfs(r: int, c: int, seen: set, prev: int) -> None:
            if (r, c) in seen or not (0 <= r < m and 0 <= c < n) or heights[r][c] < prev:
                return
            seen.add((r, c))
            for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                dfs(nr, nc, seen, heights[r][c])

        for c in range(n):
            dfs(0, c, pac, 0)
            dfs(m - 1, c, atl, 0)
        for r in range(m):
            dfs(r, 0, pac, 0)
            dfs(r, n - 1, atl, 0)
        return [list(p) for p in pac & atl]
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Multi-Source BFS from Each Ocean
```python
from collections import deque
from typing import List


class Solution:
    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:
        m, n = len(heights), len(heights[0])

        def bfs(starts: list) -> set:
            seen, q = set(starts), deque(starts)
            while q:
                r, c = q.popleft()
                for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                    if 0 <= nr < m and 0 <= nc < n and (nr, nc) not in seen and heights[nr][nc] >= heights[r][c]:
                        seen.add((nr, nc))
                        q.append((nr, nc))
            return seen

        pac = bfs([(0, c) for c in range(n)] + [(r, 0) for r in range(1, m)])
        atl = bfs([(m - 1, c) for c in range(n)] + [(r, n - 1) for r in range(m - 1)])
        return [list(p) for p in pac & atl]
```
**Complexity:** Time O(m · n) · Space O(m · n). No deep recursion.

**Brute force:** DFS from every cell toward the oceans: O((m·n)²).

---

### 130. Surrounded Regions
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/surrounded-regions/)

**Problem:** In a board of `'X'` and `'O'`, flip every `'O'` region that is fully surrounded by `'X'` to `'X'`. Regions connected to the border stay as they are.

**Key insight:** Work backward. Mark every `O` reachable from the border as safe (`T`). Flip all remaining `O`s to `X`, then turn the `T`s back into `O`s.

#### Approach 1: Border BFS (Optimal)
```python
from collections import deque
from typing import List


class Solution:
    def solve(self, board: List[List[str]]) -> None:
        """Do not return anything, modify board in-place instead."""
        m, n = len(board), len(board[0])
        q = deque((r, c) for r in range(m) for c in range(n) if (r in (0, m - 1) or c in (0, n - 1)) and board[r][c] == "O")
        while q:
            r, c = q.popleft()
            if 0 <= r < m and 0 <= c < n and board[r][c] == "O":
                board[r][c] = "T"  # safe: connected to the border
                q.extend(((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)))
        for r in range(m):
            for c in range(n):
                board[r][c] = "O" if board[r][c] == "T" else "X"
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Union-Find with a Virtual Border Node
```python
from typing import List


class Solution:
    def solve(self, board: List[List[str]]) -> None:
        m, n = len(board), len(board[0])
        border = m * n
        parent = list(range(m * n + 1))

        def find(x: int) -> int:
            while parent[x] != x:
                parent[x] = parent[parent[x]]
                x = parent[x]
            return x

        def union(a: int, b: int) -> None:
            parent[find(a)] = find(b)

        for r in range(m):
            for c in range(n):
                if board[r][c] != "O":
                    continue
                if r in (0, m - 1) or c in (0, n - 1):
                    union(r * n + c, border)
                if r + 1 < m and board[r + 1][c] == "O":
                    union(r * n + c, (r + 1) * n + c)
                if c + 1 < n and board[r][c + 1] == "O":
                    union(r * n + c, r * n + c + 1)
        root = find(border)
        for r in range(m):
            for c in range(n):
                if board[r][c] == "O" and find(r * n + c) != root:
                    board[r][c] = "X"
```
**Complexity:** Time O(m · n · α(mn)) · Space O(m · n)

---

### 994. Rotting Oranges
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/rotting-oranges/)

**Problem:** Grid cells are `0` (empty), `1` (fresh), or `2` (rotten). Each minute, fresh oranges next to a rotten one become rotten. Return the minutes until no fresh orange remains, or `-1` if some never rot.

**Key insight:** Run multi-source BFS starting from every rotten orange at once. Each BFS level is one minute.

#### Approach: Multi-Source BFS (Optimal)
```python
from collections import deque
from typing import List


class Solution:
    def orangesRotting(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        q = deque((r, c) for r in range(m) for c in range(n) if grid[r][c] == 2)
        fresh = sum(row.count(1) for row in grid)
        minutes = 0
        while q and fresh:
            minutes += 1
            for _ in range(len(q)):
                r, c = q.popleft()
                for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                    if 0 <= nr < m and 0 <= nc < n and grid[nr][nc] == 1:
                        grid[nr][nc] = 2
                        fresh -= 1
                        q.append((nr, nc))
        return -1 if fresh else minutes
```
**Complexity:** Time O(m · n) · Space O(m · n)

---

### 286. Walls and Gates (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/walls-and-gates/) · Free mirror: LintCode 663

**Problem:** In a grid, `-1` is a wall, `0` is a gate, and `INF = 2147483647` is an empty room. Fill each empty room in place with its distance to the nearest gate, leaving `INF` where no gate is reachable.

**Key insight:** Run multi-source BFS from every gate. The first time BFS reaches a room, that distance is the shortest.

#### Approach 1: Multi-Source BFS (Optimal)
```python
from collections import deque
from typing import List


class Solution:
    def wallsAndGates(self, rooms: List[List[int]]) -> None:
        """Do not return anything, modify rooms in-place instead."""
        INF = 2147483647
        m, n = len(rooms), len(rooms[0])
        q = deque((r, c) for r in range(m) for c in range(n) if rooms[r][c] == 0)
        while q:
            r, c = q.popleft()
            for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                if 0 <= nr < m and 0 <= nc < n and rooms[nr][nc] == INF:
                    rooms[nr][nc] = rooms[r][c] + 1
                    q.append((nr, nc))
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: DFS from Each Gate (slower)
```python
import sys
from typing import List


class Solution:
    def wallsAndGates(self, rooms: List[List[int]]) -> None:
        sys.setrecursionlimit(10**6)
        m, n = len(rooms), len(rooms[0])

        def dfs(r: int, c: int, d: int) -> None:
            if not (0 <= r < m and 0 <= c < n) or rooms[r][c] < d:
                return  # a wall (-1), or an already shorter distance
            rooms[r][c] = d
            for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                dfs(nr, nc, d + 1)

        for r in range(m):
            for c in range(n):
                if rooms[r][c] == 0:
                    dfs(r, c, 0)
```
**Complexity:** Time O(k · m · n) for k gates · Space O(m · n)

---

### 207. Course Schedule
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/course-schedule/)

**Problem:** There are `numCourses` courses, and `prerequisites[i] = [a, b]` means you must take `b` before `a`. Return whether you can finish every course.

**Example:** `2, [[1,0]]` → `True`; `2, [[1,0],[0,1]]` → `False`

**Key insight:** You can finish every course exactly when the prerequisite graph has no cycle.

#### Approach 1: Kahn's Algorithm (BFS Topological Sort)
```python
from collections import deque
from typing import List


class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        adj = [[] for _ in range(numCourses)]
        indeg = [0] * numCourses
        for a, b in prerequisites:
            adj[b].append(a)
            indeg[a] += 1
        q = deque(i for i in range(numCourses) if indeg[i] == 0)
        taken = 0
        while q:
            c = q.popleft()
            taken += 1
            for nxt in adj[c]:
                indeg[nxt] -= 1
                if indeg[nxt] == 0:
                    q.append(nxt)
        return taken == numCourses
```
**Complexity:** Time O(V + E) · Space O(V + E)

#### Approach 2: DFS Three-Color Cycle Detection
```python
from typing import List


class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        adj = [[] for _ in range(numCourses)]
        for a, b in prerequisites:
            adj[b].append(a)
        state = [0] * numCourses  # 0 = unvisited, 1 = visiting, 2 = done

        def has_cycle(u: int) -> bool:
            if state[u] == 1:
                return True  # back edge into the current path
            if state[u] == 2:
                return False
            state[u] = 1
            if any(has_cycle(v) for v in adj[u]):
                return True
            state[u] = 2
            return False

        return not any(has_cycle(i) for i in range(numCourses))
```
**Complexity:** Time O(V + E) · Space O(V + E)

#### Approach 3: Standard Library `graphlib` (Python 3.9+)
```python
from graphlib import CycleError, TopologicalSorter
from typing import List


class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        ts = TopologicalSorter({i: [] for i in range(numCourses)})
        for a, b in prerequisites:
            ts.add(a, b)  # a depends on b
        try:
            ts.prepare()
            return True
        except CycleError:
            return False
```
**Complexity:** Time O(V + E) · Space O(V + E)

---

### 210. Course Schedule II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/course-schedule-ii/)

**Problem:** Same setup as 207, but return a valid order in which to take the courses, or an empty array if it's impossible.

**Example:** `4, [[1,0],[2,0],[3,1],[3,2]]` → `[0,1,2,3]` or `[0,2,1,3]`

#### Approach 1: Kahn's Algorithm
```python
from collections import deque
from typing import List


class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        adj = [[] for _ in range(numCourses)]
        indeg = [0] * numCourses
        for a, b in prerequisites:
            adj[b].append(a)
            indeg[a] += 1
        q = deque(i for i in range(numCourses) if indeg[i] == 0)
        order = []
        while q:
            c = q.popleft()
            order.append(c)
            for nxt in adj[c]:
                indeg[nxt] -= 1
                if indeg[nxt] == 0:
                    q.append(nxt)
        return order if len(order) == numCourses else []
```
**Complexity:** Time O(V + E) · Space O(V + E)

#### Approach 2: DFS Post-Order
```python
from typing import List


class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        adj = [[] for _ in range(numCourses)]
        for a, b in prerequisites:
            adj[a].append(b)  # course -> its prerequisites
        state, order = [0] * numCourses, []

        def dfs(u: int) -> bool:  # returns False when a cycle is found
            if state[u] == 1:
                return False
            if state[u] == 2:
                return True
            state[u] = 1
            if not all(dfs(v) for v in adj[u]):
                return False
            state[u] = 2
            order.append(u)  # prerequisites are recorded first
            return True

        return order if all(dfs(i) for i in range(numCourses)) else []
```
**Complexity:** Time O(V + E) · Space O(V + E)

---

### 684. Redundant Connection
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/redundant-connection/)

**Problem:** A tree with nodes `1..n` had one extra edge added. Return an edge you can remove to get a tree again. If several edges qualify, return the one that appears last in the input.

**Example:** `[[1,2],[1,3],[2,3]]` → `[2,3]`

**Key insight:** Union-find. The first edge whose two endpoints are already connected is the redundant one.

#### Approach 1: Union-Find with Rank and Path Compression (Optimal)
```python
from typing import List


class Solution:
    def findRedundantConnection(self, edges: List[List[int]]) -> List[int]:
        n = len(edges)
        parent, rank = list(range(n + 1)), [0] * (n + 1)

        def find(x: int) -> int:
            if parent[x] != x:
                parent[x] = find(parent[x])
            return parent[x]

        for a, b in edges:
            ra, rb = find(a), find(b)
            if ra == rb:
                return [a, b]  # already connected, so this edge closes a cycle
            if rank[ra] < rank[rb]:
                ra, rb = rb, ra
            parent[rb] = ra
            if rank[ra] == rank[rb]:
                rank[ra] += 1
        return []
```
**Complexity:** Time O(n · α(n)) · Space O(n)

#### Approach 2: DFS per Edge (check whether already connected)
```python
from collections import defaultdict
from typing import List


class Solution:
    def findRedundantConnection(self, edges: List[List[int]]) -> List[int]:
        adj = defaultdict(list)

        def connected(u: int, target: int, seen: set) -> bool:
            if u == target:
                return True
            seen.add(u)
            return any(v not in seen and connected(v, target, seen) for v in adj[u])

        for a, b in edges:
            if a in adj and b in adj and connected(a, b, set()):
                return [a, b]
            adj[a].append(b)
            adj[b].append(a)
        return []
```
**Complexity:** Time O(n²) · Space O(n)

---

### 323. Number of Connected Components in an Undirected Graph (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/) · Free mirror: LintCode 3651

**Problem:** Given `n` nodes labeled `0..n-1` and a list of undirected `edges`, return the number of connected components.

**Example:** `n = 5, edges = [[0,1],[1,2],[3,4]]` → `2`

#### Approach 1: Union-Find (Optimal)
```python
from typing import List


class Solution:
    def countComponents(self, n: int, edges: List[List[int]]) -> int:
        parent = list(range(n))

        def find(x: int) -> int:
            while parent[x] != x:
                parent[x] = parent[parent[x]]
                x = parent[x]
            return x

        comps = n
        for a, b in edges:
            ra, rb = find(a), find(b)
            if ra != rb:
                parent[ra] = rb
                comps -= 1
        return comps
```
**Complexity:** Time O(V + E · α(V)) · Space O(V)

#### Approach 2: BFS
```python
from collections import deque
from typing import List


class Solution:
    def countComponents(self, n: int, edges: List[List[int]]) -> int:
        adj = [[] for _ in range(n)]
        for a, b in edges:
            adj[a].append(b)
            adj[b].append(a)
        seen, comps = [False] * n, 0
        for i in range(n):
            if seen[i]:
                continue
            comps += 1
            seen[i] = True
            q = deque([i])
            while q:
                u = q.popleft()
                for v in adj[u]:
                    if not seen[v]:
                        seen[v] = True
                        q.append(v)
        return comps
```
**Complexity:** Time O(V + E) · Space O(V + E)

---

### 261. Graph Valid Tree (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/graph-valid-tree/) · Free mirror: LintCode 178

**Problem:** Given `n` nodes and a list of undirected `edges`, return whether the edges form a valid tree.

**Key insight:** A graph is a tree exactly when it has `n-1` edges and is connected (equivalently, `n-1` edges and no cycle).

#### Approach 1: Edge Count + Union-Find (Optimal)
```python
from typing import List


class Solution:
    def validTree(self, n: int, edges: List[List[int]]) -> bool:
        if len(edges) != n - 1:
            return False
        parent = list(range(n))

        def find(x: int) -> int:
            while parent[x] != x:
                parent[x] = parent[parent[x]]
                x = parent[x]
            return x

        for a, b in edges:
            ra, rb = find(a), find(b)
            if ra == rb:
                return False  # cycle
            parent[ra] = rb
        return True
```
**Complexity:** Time O(n · α(n)) · Space O(n)

#### Approach 2: Edge Count + BFS Connectivity
```python
from collections import deque
from typing import List


class Solution:
    def validTree(self, n: int, edges: List[List[int]]) -> bool:
        if len(edges) != n - 1:
            return False
        adj = [[] for _ in range(n)]
        for a, b in edges:
            adj[a].append(b)
            adj[b].append(a)
        seen, q = {0}, deque([0])
        while q:
            for v in adj[q.popleft()]:
                if v not in seen:
                    seen.add(v)
                    q.append(v)
        return len(seen) == n
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
```python
from collections import defaultdict, deque
from typing import List


class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        if endWord not in wordList:
            return 0
        buckets = defaultdict(list)
        for w in wordList:
            for i in range(len(w)):
                buckets[w[:i] + "*" + w[i + 1 :]].append(w)
        seen, q = {beginWord}, deque([(beginWord, 1)])
        while q:
            word, steps = q.popleft()
            for i in range(len(word)):
                pattern = word[:i] + "*" + word[i + 1 :]
                for nxt in buckets.pop(pattern, []):  # pop: each bucket is used once
                    if nxt == endWord:
                        return steps + 1
                    if nxt not in seen:
                        seen.add(nxt)
                        q.append((nxt, steps + 1))
        return 0
```
**Complexity:** Time O(N · L²) · Space O(N · L²)

#### Approach 2: Bidirectional BFS with Letter Substitution (fastest in practice)
```python
import string
from typing import List


class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: List[str]) -> int:
        words = set(wordList)
        if endWord not in words:
            return 0
        words.discard(beginWord)
        words.discard(endWord)
        front, back, steps = {beginWord}, {endWord}, 1
        while front and back:
            if len(front) > len(back):
                front, back = back, front  # always expand the smaller side
            nxt = set()
            for w in front:
                for i in range(len(w)):
                    for c in string.ascii_lowercase:
                        cand = w[:i] + c + w[i + 1 :]
                        if cand in back:
                            return steps + 1
                        if cand in words:
                            words.remove(cand)
                            nxt.add(cand)
            front, steps = nxt, steps + 1
        return 0
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
```python
from collections import defaultdict
from typing import List


class Solution:
    def findItinerary(self, tickets: List[List[str]]) -> List[str]:
        adj = defaultdict(list)
        for src, dst in sorted(tickets, reverse=True):
            adj[src].append(dst)
        route = []

        def visit(a: str) -> None:
            while adj[a]:
                visit(adj[a].pop())
            route.append(a)  # post-order

        visit("JFK")
        return route[::-1]
```
**Complexity:** Time O(E log E) · Space O(E)

#### Approach 2: Iterative Hierholzer with an Explicit Stack + Min-Heaps
```python
import heapq
from collections import defaultdict
from typing import List


class Solution:
    def findItinerary(self, tickets: List[List[str]]) -> List[str]:
        adj = defaultdict(list)
        for src, dst in tickets:
            heapq.heappush(adj[src], dst)
        stack, route = ["JFK"], []
        while stack:
            if adj[stack[-1]]:
                stack.append(heapq.heappop(adj[stack[-1]]))
            else:
                route.append(stack.pop())  # dead end: emit in post-order
        return route[::-1]
```
**Complexity:** Time O(E log E) · Space O(E)

---

### 1584. Min Cost to Connect All Points
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/min-cost-to-connect-all-points/)

**Problem:** Given points on a 2D plane, connecting two points costs their Manhattan distance. Return the minimum total cost to connect every point (a minimum spanning tree).

**Example:** `[[0,0],[2,2],[3,10],[5,2],[7,0]]` → `20`

#### Approach 1: Prim's with an O(n²) Array (Optimal for dense graphs)
```python
from typing import List


class Solution:
    def minCostConnectPoints(self, points: List[List[int]]) -> int:
        n = len(points)
        dist = [float("inf")] * n
        dist[0] = 0
        in_tree = [False] * n
        total = 0
        for _ in range(n):
            u = min((i for i in range(n) if not in_tree[i]), key=dist.__getitem__)
            in_tree[u] = True
            total += dist[u]
            ux, uy = points[u]
            for v in range(n):
                if not in_tree[v]:
                    dist[v] = min(dist[v], abs(ux - points[v][0]) + abs(uy - points[v][1]))
        return int(total)
```
**Complexity:** Time O(n²) · Space O(n)

#### Approach 2: Kruskal's with Union-Find
```python
from typing import List


class Solution:
    def minCostConnectPoints(self, points: List[List[int]]) -> int:
        n = len(points)
        edges = sorted(
            (abs(points[i][0] - points[j][0]) + abs(points[i][1] - points[j][1]), i, j)
            for i in range(n)
            for j in range(i + 1, n)
        )
        parent = list(range(n))

        def find(x: int) -> int:
            while parent[x] != x:
                parent[x] = parent[parent[x]]
                x = parent[x]
            return x

        total = used = 0
        for w, a, b in edges:
            if used == n - 1:
                break
            ra, rb = find(a), find(b)
            if ra != rb:
                parent[ra] = rb
                total += w
                used += 1
        return total
```
**Complexity:** Time O(n² log n) · Space O(n²)

#### Approach 3: Prim's with a Min-Heap (lazy)
```python
import heapq
from typing import List


class Solution:
    def minCostConnectPoints(self, points: List[List[int]]) -> int:
        n = len(points)
        in_tree, heap, total = set(), [(0, 0)], 0  # (cost, node)
        while len(in_tree) < n:
            cost, u = heapq.heappop(heap)
            if u in in_tree:
                continue
            in_tree.add(u)
            total += cost
            ux, uy = points[u]
            for v in range(n):
                if v not in in_tree:
                    heapq.heappush(heap, (abs(ux - points[v][0]) + abs(uy - points[v][1]), v))
        return total
```
**Complexity:** Time O(n² log n) · Space O(n²)

---

### 743. Network Delay Time
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/network-delay-time/)

**Problem:** `times[i] = [u, v, w]` is a directed edge from `u` to `v` with travel time `w`. A signal starts at node `k`. Return how long until all `n` nodes receive it, or `-1` if some node never does.

**Example:** `times = [[2,1,1],[2,3,1],[3,4,1]], n = 4, k = 2` → `2`

**Key insight:** Single-source shortest paths with non-negative weights calls for Dijkstra. The answer is the largest of the shortest distances.

#### Approach 1: Dijkstra with a Min-Heap (Optimal)
```python
import heapq
from collections import defaultdict
from typing import List


class Solution:
    def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:
        adj = defaultdict(list)
        for u, v, w in times:
            adj[u].append((v, w))
        dist = {}
        heap = [(0, k)]
        while heap:
            d, u = heapq.heappop(heap)
            if u in dist:
                continue  # already finalized with a shorter distance
            dist[u] = d
            for v, w in adj[u]:
                if v not in dist:
                    heapq.heappush(heap, (d + w, v))
        return max(dist.values()) if len(dist) == n else -1
```
**Complexity:** Time O(E log E) · Space O(V + E)

#### Approach 2: Bellman-Ford
```python
from typing import List


class Solution:
    def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:
        dist = [float("inf")] * (n + 1)
        dist[k] = 0
        for _ in range(n - 1):
            changed = False
            for u, v, w in times:
                if dist[u] + w < dist[v]:
                    dist[v] = dist[u] + w
                    changed = True
            if not changed:
                break
        worst = max(dist[1:])
        return -1 if worst == float("inf") else int(worst)
```
**Complexity:** Time O(V · E) · Space O(V)

#### Approach 3: Floyd–Warshall (n ≤ 100)
```python
from typing import List


class Solution:
    def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:
        INF = float("inf")
        d = [[0 if i == j else INF for j in range(n + 1)] for i in range(n + 1)]
        for u, v, w in times:
            d[u][v] = w
        for m in range(1, n + 1):
            for i in range(1, n + 1):
                for j in range(1, n + 1):
                    if d[i][m] + d[m][j] < d[i][j]:
                        d[i][j] = d[i][m] + d[m][j]
        worst = max(d[k][1:])
        return -1 if worst == INF else int(worst)
```
**Complexity:** Time O(V³) · Space O(V²)

---

### 778. Swim in Rising Water
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/swim-in-rising-water/)

**Problem:** In an `n × n` grid, `grid[i][j]` is the elevation. At time `t` you can swim between adjacent cells whose elevations are both ≤ `t`. Return the least time at which you can get from `(0,0)` to `(n-1,n-1)`.

**Key insight:** Minimize the maximum elevation along a path. Use Dijkstra where a path's cost is `max(cost so far, next cell)`, or binary search on `t` with a BFS check.

#### Approach 1: Dijkstra / Min-Heap on the Path Maximum (Optimal)
```python
import heapq
from typing import List


class Solution:
    def swimInWater(self, grid: List[List[int]]) -> int:
        n = len(grid)
        seen = {(0, 0)}
        heap = [(grid[0][0], 0, 0)]
        while heap:
            t, r, c = heapq.heappop(heap)
            if r == c == n - 1:
                return t
            for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                if 0 <= nr < n and 0 <= nc < n and (nr, nc) not in seen:
                    seen.add((nr, nc))
                    heapq.heappush(heap, (max(t, grid[nr][nc]), nr, nc))
        return -1
```
**Complexity:** Time O(n² log n) · Space O(n²)

#### Approach 2: Binary Search on Time + BFS
```python
from collections import deque
from typing import List


class Solution:
    def swimInWater(self, grid: List[List[int]]) -> int:
        n = len(grid)

        def can_reach(t: int) -> bool:
            if grid[0][0] > t:
                return False
            seen, q = {(0, 0)}, deque([(0, 0)])
            while q:
                r, c = q.popleft()
                if r == c == n - 1:
                    return True
                for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                    if 0 <= nr < n and 0 <= nc < n and (nr, nc) not in seen and grid[nr][nc] <= t:
                        seen.add((nr, nc))
                        q.append((nr, nc))
            return False

        lo, hi = grid[0][0], n * n - 1
        while lo < hi:
            mid = (lo + hi) // 2
            if can_reach(mid):
                hi = mid
            else:
                lo = mid + 1
        return lo
```
**Complexity:** Time O(n² log n) · Space O(n²)

---

### 269. Alien Dictionary (Premium)
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/alien-dictionary/) · Free mirror: LintCode 892

**Problem:** You're given a list of `words` sorted in an unknown alien alphabet. Work out an order for the letters that is consistent with that sorting. Return `""` if the input is contradictory, including when a word comes before its own prefix.

**Example:** `["wrt","wrf","er","ett","rftt"]` → `"wertf"`

**Key insight:** In each adjacent pair of words, the first position where they differ gives one edge `a → b`. A topological sort of those edges gives the alphabet, and a cycle means the input is invalid.

#### Approach 1: Kahn's BFS Topological Sort
```python
from collections import deque
from typing import List


class Solution:
    def alienOrder(self, words: List[str]) -> str:
        adj = {c: set() for w in words for c in w}
        indeg = {c: 0 for c in adj}
        for a, b in zip(words, words[1:]):
            for x, y in zip(a, b):
                if x != y:
                    if y not in adj[x]:
                        adj[x].add(y)
                        indeg[y] += 1
                    break
            else:  # no differing character in the common prefix
                if len(a) > len(b):
                    return ""  # a word appears before its own prefix
        q = deque(c for c in indeg if indeg[c] == 0)
        out = []
        while q:
            c = q.popleft()
            out.append(c)
            for nxt in adj[c]:
                indeg[nxt] -= 1
                if indeg[nxt] == 0:
                    q.append(nxt)
        return "".join(out) if len(out) == len(indeg) else ""  # shorter means a cycle
```
**Complexity:** Time O(C), where C is the total number of characters · Space O(U + min(U², N)) for U unique letters

#### Approach 2: DFS Post-Order with Cycle Detection
```python
from typing import List


class Solution:
    def alienOrder(self, words: List[str]) -> str:
        adj = {c: set() for w in words for c in w}
        for a, b in zip(words, words[1:]):
            for x, y in zip(a, b):
                if x != y:
                    adj[x].add(y)
                    break
            else:
                if len(a) > len(b):
                    return ""
        state, out = {}, []  # state: True = visiting, False = done

        def dfs(c: str) -> bool:  # returns True if a cycle is found
            if c in state:
                return state[c]
            state[c] = True
            if any(dfs(nx) for nx in adj[c]):
                return True
            state[c] = False
            out.append(c)
            return False

        if any(dfs(c) for c in adj):
            return ""
        return "".join(reversed(out))
```
**Complexity:** Time O(C) · Space O(U + edges)

---

### 787. Cheapest Flights Within K Stops
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/cheapest-flights-within-k-stops/)

**Problem:** `flights[i] = [from, to, price]`. Return the cheapest price from `src` to `dst` using at most `k` stops, or `-1` if no such route exists.

**Example:** `n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1` → `700`

**Key insight:** Bellman-Ford limited to `k+1` rounds. Relax from a copy of the previous round's prices so a single round can't chain several edges.

#### Approach 1: Bellman-Ford with k+1 Iterations (Optimal)
```python
from typing import List


class Solution:
    def findCheapestPrice(self, n: int, flights: List[List[int]], src: int, dst: int, k: int) -> int:
        prices = [float("inf")] * n
        prices[src] = 0
        for _ in range(k + 1):
            tmp = prices[:]  # relax from the previous round only
            for u, v, w in flights:
                if prices[u] + w < tmp[v]:
                    tmp[v] = prices[u] + w
            prices = tmp
        return -1 if prices[dst] == float("inf") else int(prices[dst])
```
**Complexity:** Time O(k · E) · Space O(n)

#### Approach 2: BFS by Number of Stops
```python
from collections import defaultdict, deque
from typing import List


class Solution:
    def findCheapestPrice(self, n: int, flights: List[List[int]], src: int, dst: int, k: int) -> int:
        adj = defaultdict(list)
        for u, v, w in flights:
            adj[u].append((v, w))
        best = [float("inf")] * n
        best[src] = 0
        q, stops = deque([(src, 0)]), 0
        while q and stops <= k:
            for _ in range(len(q)):
                u, cost = q.popleft()
                for v, w in adj[u]:
                    if cost + w < best[v]:
                        best[v] = cost + w
                        q.append((v, cost + w))
            stops += 1
        return -1 if best[dst] == float("inf") else int(best[dst])
```
**Complexity:** Time O(k · E) · Space O(n + E)

---
## 13. 1-D Dynamic Programming

> **Progression for each problem:** recursion → memoization (top-down, often just `@cache`) → tabulation (bottom-up) → space-optimized, where that helps.

### 70. Climbing Stairs
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/climbing-stairs/)

**Problem:** You can climb 1 or 2 steps at a time. How many distinct ways are there to reach the top of `n` stairs?

**Example:** `n = 3` → `3` (1+1+1, 1+2, 2+1)

**Key insight:** `ways(n) = ways(n-1) + ways(n-2)`, the Fibonacci recurrence.

#### Approach 1: Memoized Recursion with `@cache`
```python
from functools import cache


class Solution:
    def climbStairs(self, n: int) -> int:
        @cache
        def f(i: int) -> int:
            return i if i <= 2 else f(i - 1) + f(i - 2)

        return f(n)
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Bottom-Up, O(1) Space (Optimal)
```python
class Solution:
    def climbStairs(self, n: int) -> int:
        a, b = 1, 1  # ways(0), ways(1)
        for _ in range(n - 1):
            a, b = b, a + b
        return b
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 3: Combinatorics
Counting how many 2-steps are used: `sum(C(n - k, k))` for `k` in `0..n//2`. This is exact, with no floating point.
```python
from math import comb


class Solution:
    def climbStairs(self, n: int) -> int:
        return sum(comb(n - k, k) for k in range(n // 2 + 1))
```
**Complexity:** Time O(n²) for the big-integer binomials, but tiny for n ≤ 45 · Space O(1)

---

### 746. Min Cost Climbing Stairs
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/min-cost-climbing-stairs/)

**Problem:** `cost[i]` is the price of stepping on stair `i`. After paying, you can climb 1 or 2 steps. You may start on stair 0 or stair 1. Return the minimum cost to get past the last stair.

**Example:** `[10,15,20]` → `15`

#### Approach 1: Tabulation
```python
from typing import List


class Solution:
    def minCostClimbingStairs(self, cost: List[int]) -> int:
        n = len(cost)
        dp = [0] * (n + 1)  # dp[i] = min cost to reach stair i
        for i in range(2, n + 1):
            dp[i] = min(dp[i - 1] + cost[i - 1], dp[i - 2] + cost[i - 2])
        return dp[n]
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: In-Place, O(1) Extra (Optimal)
```python
from typing import List


class Solution:
    def minCostClimbingStairs(self, cost: List[int]) -> int:
        a = b = 0  # cost to stand on stair i-2 and i-1, having paid for them
        for c in cost:
            a, b = b, min(a, b) + c
        return min(a, b)
```
**Complexity:** Time O(n) · Space O(1)

---

### 198. House Robber
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/house-robber/)

**Problem:** Houses along a street hold `nums[i]` money each. You can't rob two adjacent houses. Return the most money you can take.

**Example:** `[2,7,9,3,1]` → `12` (2 + 9 + 1)

**Key insight:** `dp[i] = max(dp[i-1], dp[i-2] + nums[i])`.

#### Approach 1: Memoized Recursion
```python
from functools import cache
from typing import List


class Solution:
    def rob(self, nums: List[int]) -> int:
        @cache
        def f(i: int) -> int:
            return 0 if i < 0 else max(f(i - 1), f(i - 2) + nums[i])

        return f(len(nums) - 1)
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: O(1) Space (Optimal)
```python
from typing import List


class Solution:
    def rob(self, nums: List[int]) -> int:
        prev2 = prev1 = 0
        for n in nums:
            prev2, prev1 = prev1, max(prev1, prev2 + n)
        return prev1
```
**Complexity:** Time O(n) · Space O(1)

---

### 213. House Robber II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/house-robber-ii/)

**Problem:** Same as House Robber, but the houses form a circle, so the first and last houses are adjacent.

**Example:** `[2,3,2]` → `3`; `[1,2,3,1]` → `4`

**Key insight:** You can't rob both the first and the last house. Answer = `max(rob(nums[1:]), rob(nums[:n-1]))`.

#### Approach: Two Linear Passes (Optimal)
```python
from typing import List


class Solution:
    def rob(self, nums: List[int]) -> int:
        def rob_line(houses: List[int]) -> int:
            a = b = 0
            for h in houses:
                a, b = b, max(b, a + h)
            return b

        if len(nums) == 1:
            return nums[0]
        return max(rob_line(nums[:-1]), rob_line(nums[1:]))
```
**Complexity:** Time O(n) · Space O(n) for the slices (index ranges make it O(1))

---

### 5. Longest Palindromic Substring
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-palindromic-substring/)

**Problem:** Return the longest substring of `s` that is a palindrome.

**Example:** `"babad"` → `"bab"` (or `"aba"`); `"cbbd"` → `"bb"`

**Key insight:** Every palindrome has a center: a single character (odd length) or the gap between two characters (even length). Try each of the 2n−1 centers and expand outward.

#### Approach 1: Expand Around the Center (Optimal in practice)
```python
class Solution:
    def longestPalindrome(self, s: str) -> str:
        def expand(l: int, r: int) -> str:
            while l >= 0 and r < len(s) and s[l] == s[r]:
                l, r = l - 1, r + 1
            return s[l + 1 : r]

        best = ""
        for i in range(len(s)):
            best = max(best, expand(i, i), expand(i, i + 1), key=len)  # odd, even length
        return best
```
**Complexity:** Time O(n²) · Space O(n) for the slice

#### Approach 2: DP Table
```python
class Solution:
    def longestPalindrome(self, s: str) -> str:
        n = len(s)
        dp = [[False] * n for _ in range(n)]
        start, best = 0, 1
        for i in range(n - 1, -1, -1):
            for j in range(i, n):
                if s[i] == s[j] and (j - i < 3 or dp[i + 1][j - 1]):
                    dp[i][j] = True
                    if j - i + 1 > best:
                        start, best = i, j - i + 1
        return s[start : start + best]
```
**Complexity:** Time O(n²) · Space O(n²)

#### Approach 3: Manacher's Algorithm (theoretically optimal)
```python
class Solution:
    def longestPalindrome(self, s: str) -> str:
        t = "^#" + "#".join(s) + "#$"  # "abc" -> "^#a#b#c#$"; every palindrome now has odd length
        p = [0] * len(t)
        center = right = 0
        for i in range(1, len(t) - 1):
            if i < right:
                p[i] = min(right - i, p[2 * center - i])
            while t[i + p[i] + 1] == t[i - p[i] - 1]:
                p[i] += 1
            if i + p[i] > right:
                center, right = i, i + p[i]
        max_len, ci = max((v, i) for i, v in enumerate(p))
        begin = (ci - max_len) // 2
        return s[begin : begin + max_len]
```
**Complexity:** Time O(n) · Space O(n)

---

### 647. Palindromic Substrings
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/palindromic-substrings/)

**Problem:** Count the substrings of `s` that are palindromes. Substrings at different positions count separately.

**Example:** `"aaa"` → `6`

#### Approach 1: Expand Around the Center (Optimal in practice)
```python
class Solution:
    def countSubstrings(self, s: str) -> int:
        count = 0
        for c in range(2 * len(s) - 1):
            l, r = c // 2, c // 2 + c % 2
            while l >= 0 and r < len(s) and s[l] == s[r]:
                count += 1
                l, r = l - 1, r + 1
        return count
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: DP Table
```python
class Solution:
    def countSubstrings(self, s: str) -> int:
        n = len(s)
        dp = [[False] * n for _ in range(n)]
        count = 0
        for i in range(n - 1, -1, -1):
            for j in range(i, n):
                if s[i] == s[j] and (j - i < 3 or dp[i + 1][j - 1]):
                    dp[i][j] = True
                    count += 1
        return count
```
**Complexity:** Time O(n²) · Space O(n²)

#### Approach 3: Manacher's Algorithm
```python
class Solution:
    def countSubstrings(self, s: str) -> int:
        t = "^#" + "#".join(s) + "#$"
        p = [0] * len(t)
        center = right = 0
        for i in range(1, len(t) - 1):
            if i < right:
                p[i] = min(right - i, p[2 * center - i])
            while t[i + p[i] + 1] == t[i - p[i] - 1]:
                p[i] += 1
            if i + p[i] > right:
                center, right = i, i + p[i]
        return sum((v + 1) // 2 for v in p)  # each radius v covers (v+1)//2 real palindromes
```
**Complexity:** Time O(n) · Space O(n)

---

### 91. Decode Ways
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/decode-ways/)

**Problem:** Letters are encoded as numbers (`'A' → "1"`, …, `'Z' → "26"`). Given a string of digits, count the ways to decode it.

**Example:** `"12"` → `2` ("AB", "L"); `"226"` → `3`; `"06"` → `0`

**Key insight:** `dp[i] = dp[i-1]` if `s[i-1] != '0'`, plus `dp[i-2]` if `s[i-2:i]` is between 10 and 26.

#### Approach 1: Memoized Recursion
```python
from functools import cache


class Solution:
    def numDecodings(self, s: str) -> int:
        @cache
        def f(i: int) -> int:
            if i == len(s):
                return 1
            if s[i] == "0":
                return 0
            ways = f(i + 1)
            if i + 1 < len(s) and 10 <= int(s[i : i + 2]) <= 26:
                ways += f(i + 2)
            return ways

        return f(0)
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: O(1) Space (Optimal)
```python
class Solution:
    def numDecodings(self, s: str) -> int:
        prev2, prev1 = 0, 1  # dp[i-2], dp[i-1]
        for i, ch in enumerate(s):
            cur = prev1 if ch != "0" else 0
            if i > 0 and (s[i - 1] == "1" or (s[i - 1] == "2" and ch <= "6")):
                cur += prev2
            prev2, prev1 = prev1, cur
        return prev1
```
**Complexity:** Time O(n) · Space O(1)

---

### 322. Coin Change
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/coin-change/)

**Problem:** Given coin denominations and an `amount`, return the fewest coins that add up to `amount`, or `-1` if it can't be made. You have unlimited coins of each kind.

**Example:** `coins = [1,2,5], amount = 11` → `3` (5+5+1)

**Key insight:** `dp[a] = 1 + min(dp[a - c])` over every coin `c`. (Greedy fails for some coin sets.)

#### Approach 1: Bottom-Up DP (Optimal)
```python
from typing import List


class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        INF = amount + 1
        dp = [0] + [INF] * amount
        for a in range(1, amount + 1):
            for c in coins:
                if c <= a and dp[a - c] + 1 < dp[a]:
                    dp[a] = dp[a - c] + 1
        return dp[amount] if dp[amount] < INF else -1
```
**Complexity:** Time O(amount · len(coins)) · Space O(amount)

#### Approach 2: BFS over Amounts (shortest path)
```python
from collections import deque
from typing import List


class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        if amount == 0:
            return 0
        seen, q, steps = {0}, deque([0]), 0
        while q:
            steps += 1
            for _ in range(len(q)):
                cur = q.popleft()
                for c in coins:
                    v = cur + c
                    if v == amount:
                        return steps
                    if v < amount and v not in seen:
                        seen.add(v)
                        q.append(v)
        return -1
```
**Complexity:** Time O(amount · len(coins)) · Space O(amount)

#### Approach 3: Top-Down Memoization
```python
import sys
from typing import List


class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        sys.setrecursionlimit(10**6)
        memo = {0: 0}  # a plain dict: @cache's C wrapper caps the depth at ~3,300 frames

        def f(a: int) -> float:
            if a in memo:
                return memo[a]
            best = float("inf")
            for c in coins:
                if c <= a:
                    best = min(best, f(a - c) + 1)
            memo[a] = best
            return best

        res = f(amount)
        return -1 if res == float("inf") else int(res)
```
**Complexity:** Time O(amount · len(coins)) · Space O(amount); recursion depth reaches `amount`, hence the raised limit

---

### 152. Maximum Product Subarray
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/maximum-product-subarray/)

**Problem:** Return the largest product of any contiguous non-empty subarray.

**Example:** `[2,3,-2,4]` → `6`; `[-2,0,-1]` → `0`

**Key insight:** A negative number turns the minimum product into the maximum, so track both the current max and the current min.

#### Approach 1: Track Max and Min (Optimal)
```python
from typing import List


class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        best = cur_max = cur_min = nums[0]
        for n in nums[1:]:
            candidates = (n, cur_max * n, cur_min * n)  # a negative flips max and min
            cur_max, cur_min = max(candidates), min(candidates)
            best = max(best, cur_max)
        return best
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Prefix and Suffix Products
The best subarray always starts at an edge of the array or of a zero-bounded segment.
```python
from typing import List


class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        best, pre, suf = nums[0], 1, 1
        n = len(nums)
        for i in range(n):
            pre = (pre or 1) * nums[i]          # restart after a zero
            suf = (suf or 1) * nums[n - 1 - i]
            best = max(best, pre, suf)
        return best
```
**Complexity:** Time O(n) · Space O(1)

---

### 139. Word Break
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/word-break/)

**Problem:** Return whether `s` can be split into a sequence of one or more words from `wordDict`. Words can be reused.

**Example:** `s = "leetcode", wordDict = ["leet","code"]` → `True`

**Key insight:** `dp[i]` is true when some `j < i` has `dp[j]` true and `s[j:i]` in the dictionary.

#### Approach 1: Bottom-Up DP with a Set (Optimal)
```python
from typing import List


class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        words = set(wordDict)
        max_len = max(map(len, words))
        dp = [True] + [False] * len(s)
        for i in range(1, len(s) + 1):
            for j in range(max(0, i - max_len), i):
                if dp[j] and s[j:i] in words:
                    dp[i] = True
                    break
        return dp[len(s)]
```
**Complexity:** Time O(n · L), where L is the longest word (slicing and hashing add O(L) per check) · Space O(n)

#### Approach 2: Memoized DFS over Dictionary Words
```python
from functools import cache
from typing import List


class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        @cache
        def can(i: int) -> bool:
            if i == len(s):
                return True
            return any(s.startswith(w, i) and can(i + len(w)) for w in wordDict)

        return can(0)
```
**Complexity:** Time O(n · m · L) for m words · Space O(n)

---

### 300. Longest Increasing Subsequence
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-increasing-subsequence/)

**Problem:** Return the length of the longest strictly increasing subsequence.

**Example:** `[10,9,2,5,3,7,101,18]` → `4` (e.g. `2,3,7,101`)

**Key insight (patience sorting):** Keep `tails[k]`, the smallest possible tail of an increasing subsequence of length `k+1`. Each new number replaces the first tail that is ≥ it.

#### Approach 1: O(n²) DP
```python
from typing import List


class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        dp = [1] * len(nums)
        for i in range(len(nums)):
            for j in range(i):
                if nums[j] < nums[i]:
                    dp[i] = max(dp[i], dp[j] + 1)
        return max(dp)
```
**Complexity:** Time O(n²) · Space O(n)

#### Approach 2: Binary Search on Tails (Optimal)
```python
import bisect
from typing import List


class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        tails = []  # tails[k] = smallest tail of an increasing subsequence of length k+1
        for n in nums:
            i = bisect.bisect_left(tails, n)
            if i == len(tails):
                tails.append(n)
            else:
                tails[i] = n
        return len(tails)
```
**Complexity:** Time O(n log n) · Space O(n)

---

### 416. Partition Equal Subset Sum
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/partition-equal-subset-sum/)

**Problem:** Return whether `nums` can be split into two subsets with equal sums.

**Example:** `[1,5,11,5]` → `True` (`[1,5,5]` and `[11]`)

**Key insight:** This is 0/1 knapsack: can some subset reach `total/2`? Iterate the sums from high to low so each number is used at most once.

#### Approach 1: 1-D Boolean DP
```python
from typing import List


class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        total = sum(nums)
        if total % 2:
            return False
        target = total // 2
        dp = [True] + [False] * target
        for n in nums:
            for s in range(target, n - 1, -1):  # high to low, so each number is used once
                dp[s] = dp[s] or dp[s - n]
        return dp[target]
```
**Complexity:** Time O(n · target) · Space O(target)

#### Approach 2: Big-Integer Bitset (Optimal in Python)
Python ints are arbitrary-precision bitsets. Shifting left by `n` adds `n` to every reachable sum at once.
```python
from typing import List


class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        total = sum(nums)
        if total % 2:
            return False
        bits = 1  # bit s set means sum s is reachable
        for n in nums:
            bits |= bits << n
        return bool(bits >> (total // 2) & 1)
```
**Complexity:** Time O(n · total / 64) machine-word operations · Space O(total) bits

#### Approach 3: Set of Reachable Sums
```python
from typing import List


class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        total = sum(nums)
        if total % 2:
            return False
        target = total // 2
        reach = {0}
        for n in nums:
            reach |= {s + n for s in reach if s + n <= target}
            if target in reach:
                return True
        return target in reach
```
**Complexity:** Time O(n · target) · Space O(target)

---
## 14. 2-D Dynamic Programming

### 62. Unique Paths
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/unique-paths/)

**Problem:** A robot starts at the top-left of an `m × n` grid and can only move right or down. How many distinct paths lead to the bottom-right corner?

**Example:** `m = 3, n = 7` → `28`

#### Approach 1: 2-D DP
```python
class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        dp = [[1] * n for _ in range(m)]  # the first row and column are all 1
        for i in range(1, m):
            for j in range(1, n):
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1]
        return dp[m - 1][n - 1]
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D Rolling Row (Optimal DP)
```python
class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        row = [1] * n
        for _ in range(1, m):
            for j in range(1, n):
                row[j] += row[j - 1]
        return row[-1]
```
**Complexity:** Time O(m · n) · Space O(n)

#### Approach 3: Combinatorics C(m+n-2, m-1)
```python
from math import comb


class Solution:
    def uniquePaths(self, m: int, n: int) -> int:
        return comb(m + n - 2, m - 1)  # choose which of the m+n-2 moves go down
```
**Complexity:** Time O(min(m, n)) · Space O(1)

---

### 1143. Longest Common Subsequence
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-common-subsequence/)

**Problem:** Return the length of the longest subsequence that both `text1` and `text2` contain.

**Example:** `"abcde", "ace"` → `3`

**Key insight:** If `a[i] == b[j]`, then `dp[i][j] = 1 + dp[i-1][j-1]`. Otherwise `dp[i][j] = max(dp[i-1][j], dp[i][j-1])`.

#### Approach 1: Memoized Recursion
```python
import sys
from functools import cache


class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        sys.setrecursionlimit(10**6)

        @cache
        def f(i: int, j: int) -> int:
            if i == len(text1) or j == len(text2):
                return 0
            if text1[i] == text2[j]:
                return 1 + f(i + 1, j + 1)
            return max(f(i + 1, j), f(i, j + 1))

        return f(0, 0)
```
**Complexity:** Time O(m · n) · Space O(m · n), with recursion depth up to m + n

#### Approach 2: 2-D Table
```python
class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        m, n = len(text1), len(text2)
        dp = [[0] * (n + 1) for _ in range(m + 1)]
        for i in range(1, m + 1):
            for j in range(1, n + 1):
                if text1[i - 1] == text2[j - 1]:
                    dp[i][j] = dp[i - 1][j - 1] + 1
                else:
                    dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])
        return dp[m][n]
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 3: Two Rows (Optimal space)
```python
class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        if len(text2) > len(text1):
            text1, text2 = text2, text1
        prev = [0] * (len(text2) + 1)
        for a in text1:
            cur = [0]
            for j, b in enumerate(text2, 1):
                cur.append(prev[j - 1] + 1 if a == b else max(prev[j], cur[j - 1]))
            prev = cur
        return prev[-1]
```
**Complexity:** Time O(m · n) · Space O(min(m, n))

---

### 309. Best Time to Buy and Sell Stock with Cooldown
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/)

**Problem:** You may make as many transactions as you like, but after selling you must wait one day before buying again, and you can hold at most one share. Return the maximum profit.

**Example:** `[1,2,3,0,2]` → `3` (buy, sell, cooldown, buy, sell)

**Key insight:** A state machine with three states: `hold` (own a share), `sold` (sold today, so tomorrow is a cooldown), and `rest` (no share, free to buy).

#### Approach 1: State Machine DP (Optimal)
```python
from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        hold, sold, rest = float("-inf"), 0, 0
        for p in prices:
            hold, sold, rest = max(hold, rest - p), hold + p, max(rest, sold)  # all from the previous day
        return int(max(sold, rest))
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Memoized DFS (index, can_buy)
```python
import sys
from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        sys.setrecursionlimit(10**6)
        memo = {}

        def dfs(i: int, buying: bool) -> int:
            if i >= len(prices):
                return 0
            if (i, buying) not in memo:
                cooldown = dfs(i + 1, buying)
                if buying:
                    act = dfs(i + 1, False) - prices[i]
                else:
                    act = dfs(i + 2, True) + prices[i]  # selling forces a one-day cooldown
                memo[(i, buying)] = max(cooldown, act)
            return memo[(i, buying)]

        return dfs(0, True)
```
**Complexity:** Time O(n) · Space O(n)

---

### 518. Coin Change II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/coin-change-ii/)

**Problem:** Return how many combinations of coins add up to `amount`. Coin supply is unlimited, and combinations are unordered.

**Example:** `amount = 5, coins = [1,2,5]` → `4`

**Key insight:** Loop over coins on the outside and amounts on the inside. That counts each combination once, not once per ordering.

#### Approach 1: 1-D DP (Optimal)
```python
from typing import List


class Solution:
    def change(self, amount: int, coins: List[int]) -> int:
        dp = [1] + [0] * amount
        for c in coins:  # coins in the outer loop: combinations, not permutations
            for a in range(c, amount + 1):
                dp[a] += dp[a - c]
        return dp[amount]
```
**Complexity:** Time O(n · amount) · Space O(amount)

> Python ints never overflow, so the huge intermediate counts that trip up fixed-width languages on some LeetCode tests aren't an issue here.

#### Approach 2: Memoized Recursion (index, remaining)
```python
import sys
from typing import List


class Solution:
    def change(self, amount: int, coins: List[int]) -> int:
        sys.setrecursionlimit(10**6)
        memo = {}

        def f(i: int, remain: int) -> int:
            if remain == 0:
                return 1
            if i == len(coins) or remain < 0:
                return 0
            if (i, remain) not in memo:
                memo[(i, remain)] = f(i, remain - coins[i]) + f(i + 1, remain)  # use coin i again, or move on
            return memo[(i, remain)]

        return f(0, amount)
```
**Complexity:** Time O(n · amount) · Space O(n · amount)

---

### 494. Target Sum
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/target-sum/)

**Problem:** Put a `+` or `-` in front of every number in `nums`. Count the ways the resulting expression equals `target`.

**Example:** `nums = [1,1,1,1,1], target = 3` → `5`

**Key insight:** If `P` is the set of numbers with `+`, then `sum(P) = (total + target) / 2`. That reduces the problem to counting subsets with a given sum.

#### Approach 1: Memoized DFS (index, running sum)
```python
from functools import cache
from typing import List


class Solution:
    def findTargetSumWays(self, nums: List[int], target: int) -> int:
        @cache
        def dfs(i: int, total: int) -> int:
            if i == len(nums):
                return int(total == target)
            return dfs(i + 1, total + nums[i]) + dfs(i + 1, total - nums[i])

        return dfs(0, 0)
```
**Complexity:** Time O(n · total) · Space O(n · total)

#### Approach 2: Dictionary of Reachable Sums
```python
from collections import Counter
from typing import List


class Solution:
    def findTargetSumWays(self, nums: List[int], target: int) -> int:
        ways = Counter({0: 1})  # sum -> number of sign assignments reaching it
        for n in nums:
            nxt = Counter()
            for s, c in ways.items():
                nxt[s + n] += c
                nxt[s - n] += c
            ways = nxt
        return ways[target]
```
**Complexity:** Time O(n · total) · Space O(total)

#### Approach 3: Subset-Sum Counting (Optimal)
```python
from typing import List


class Solution:
    def findTargetSumWays(self, nums: List[int], target: int) -> int:
        total = sum(nums)
        if abs(target) > total or (total + target) % 2:
            return 0
        p = (total + target) // 2  # the sum of the numbers that get '+'
        dp = [1] + [0] * p
        for n in nums:
            for s in range(p, n - 1, -1):
                dp[s] += dp[s - n]
        return dp[p]
```
**Complexity:** Time O(n · P) · Space O(P)

---

### 97. Interleaving String
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/interleaving-string/)

**Problem:** Return whether `s3` can be formed by interleaving `s1` and `s2` while keeping each string's characters in their original order.

**Example:** `s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"` → `True`

**Key insight:** `dp[i][j]` is true when `s3[:i+j]` is an interleaving of `s1[:i]` and `s2[:j]`.

#### Approach 1: Memoized Recursion
```python
from functools import cache


class Solution:
    def isInterleave(self, s1: str, s2: str, s3: str) -> bool:
        if len(s1) + len(s2) != len(s3):
            return False

        @cache
        def f(i: int, j: int) -> bool:
            if i == len(s1) and j == len(s2):
                return True
            k = i + j
            return (i < len(s1) and s1[i] == s3[k] and f(i + 1, j)) or (
                j < len(s2) and s2[j] == s3[k] and f(i, j + 1)
            )

        return f(0, 0)
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D DP (Optimal space)
```python
class Solution:
    def isInterleave(self, s1: str, s2: str, s3: str) -> bool:
        m, n = len(s1), len(s2)
        if m + n != len(s3):
            return False
        dp = [False] * (n + 1)
        for i in range(m + 1):
            for j in range(n + 1):
                if i == 0 and j == 0:
                    dp[j] = True
                elif i == 0:
                    dp[j] = dp[j - 1] and s2[j - 1] == s3[j - 1]
                elif j == 0:
                    dp[j] = dp[j] and s1[i - 1] == s3[i - 1]
                else:
                    dp[j] = (dp[j] and s1[i - 1] == s3[i + j - 1]) or (dp[j - 1] and s2[j - 1] == s3[i + j - 1])
        return dp[n]
```
**Complexity:** Time O(m · n) · Space O(n)

---

### 329. Longest Increasing Path in a Matrix
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/longest-increasing-path-in-a-matrix/)

**Problem:** Return the length of the longest strictly increasing path in a matrix, moving in four directions.

**Example:** `[[9,9,4],[6,6,8],[2,1,1]]` → `4` (1→2→6→9)

**Key insight:** Strictly increasing paths can't form cycles, so the cells form a DAG and DFS with memoization works.

#### Approach 1: DFS + Memoization (Optimal)
```python
import sys
from typing import List


class Solution:
    def longestIncreasingPath(self, matrix: List[List[int]]) -> int:
        sys.setrecursionlimit(10**6)
        m, n = len(matrix), len(matrix[0])
        memo = [[0] * n for _ in range(m)]  # 0 = not computed; a snake-shaped path can be m*n deep

        def dfs(r: int, c: int) -> int:
            if memo[r][c]:
                return memo[r][c]
            best = 1
            for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                if 0 <= nr < m and 0 <= nc < n and matrix[nr][nc] > matrix[r][c]:
                    best = max(best, 1 + dfs(nr, nc))
            memo[r][c] = best
            return best

        return max(dfs(r, c) for r in range(m) for c in range(n))
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Process Cells in Increasing Value Order (no recursion)
Sorting cells by value gives a topological order for "strictly increasing" edges.
```python
from typing import List


class Solution:
    def longestIncreasingPath(self, matrix: List[List[int]]) -> int:
        m, n = len(matrix), len(matrix[0])
        best = [[1] * n for _ in range(m)]
        cells = sorted((matrix[r][c], r, c) for r in range(m) for c in range(n))
        for v, r, c in cells:  # every smaller neighbor is already final
            for nr, nc in ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)):
                if 0 <= nr < m and 0 <= nc < n and matrix[nr][nc] < v:
                    best[r][c] = max(best[r][c], best[nr][nc] + 1)
        return max(map(max, best))
```
**Complexity:** Time O(m · n · log(m · n)) · Space O(m · n)

---

### 115. Distinct Subsequences
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/distinct-subsequences/)

**Problem:** Count the distinct subsequences of `s` that equal `t`.

**Example:** `s = "rabbbit", t = "rabbit"` → `3`

**Key insight:** `dp[i][j] = dp[i-1][j]` (skip `s[i-1]`), plus `dp[i-1][j-1]` when `s[i-1] == t[j-1]`.

#### Approach 1: 2-D DP
```python
class Solution:
    def numDistinct(self, s: str, t: str) -> int:
        m, n = len(s), len(t)
        dp = [[0] * (n + 1) for _ in range(m + 1)]
        for i in range(m + 1):
            dp[i][0] = 1  # the empty t matches once
        for i in range(1, m + 1):
            for j in range(1, n + 1):
                dp[i][j] = dp[i - 1][j] + (dp[i - 1][j - 1] if s[i - 1] == t[j - 1] else 0)
        return dp[m][n]
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D DP, iterate j backward (Optimal space)
```python
class Solution:
    def numDistinct(self, s: str, t: str) -> int:
        dp = [1] + [0] * len(t)
        for ch in s:
            for j in range(len(t), 0, -1):
                if ch == t[j - 1]:
                    dp[j] += dp[j - 1]
        return dp[-1]
```
**Complexity:** Time O(m · n) · Space O(n)

---

### 72. Edit Distance
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/edit-distance/)

**Problem:** Return the minimum number of insert, delete, and replace operations needed to turn `word1` into `word2`.

**Example:** `"horse" → "ros"` → `3`

**Key insight:** If the characters match, `dp[i][j] = dp[i-1][j-1]`. Otherwise `dp[i][j] = 1 + min(delete dp[i-1][j], insert dp[i][j-1], replace dp[i-1][j-1])`.

#### Approach 1: Memoized Recursion
```python
import sys
from functools import cache


class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        sys.setrecursionlimit(10**6)

        @cache
        def f(i: int, j: int) -> int:
            if i == len(word1):
                return len(word2) - j
            if j == len(word2):
                return len(word1) - i
            if word1[i] == word2[j]:
                return f(i + 1, j + 1)
            return 1 + min(f(i + 1, j + 1), f(i + 1, j), f(i, j + 1))  # replace, delete, insert

        return f(0, 0)
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D DP (Optimal space)
```python
class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        n = len(word2)
        dp = list(range(n + 1))
        for i in range(1, len(word1) + 1):
            diag, dp[0] = dp[0], i  # diag = dp[i-1][j-1]
            for j in range(1, n + 1):
                tmp = dp[j]
                if word1[i - 1] == word2[j - 1]:
                    dp[j] = diag
                else:
                    dp[j] = 1 + min(diag, dp[j], dp[j - 1])
                diag = tmp
        return dp[n]
```
**Complexity:** Time O(m · n) · Space O(n)

---

### 312. Burst Balloons
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/burst-balloons/)

**Problem:** Bursting balloon `i` earns `nums[left] * nums[i] * nums[right]`, where `left` and `right` are its current neighbors, and the ends are padded with 1s. Return the maximum coins you can collect by bursting every balloon.

**Example:** `[3,1,5,8]` → `167`

**Key insight:** Think about the **last** balloon `k` to burst in the open interval `(l, r)`. Its neighbors at that moment are `l` and `r`, which makes the two sides independent subproblems.

#### Approach 1: Interval DP, Bottom-Up (Optimal)
```python
from typing import List


class Solution:
    def maxCoins(self, nums: List[int]) -> int:
        vals = [1] + nums + [1]
        n = len(vals)
        dp = [[0] * n for _ in range(n)]
        for length in range(2, n):  # gap between l and r
            for l in range(n - length):
                r = l + length
                dp[l][r] = max(dp[l][k] + vals[l] * vals[k] * vals[r] + dp[k][r] for k in range(l + 1, r))
        return dp[0][n - 1]
```
**Complexity:** Time O(n³) · Space O(n²)

#### Approach 2: Top-Down with `@cache`
```python
from functools import cache
from typing import List


class Solution:
    def maxCoins(self, nums: List[int]) -> int:
        vals = [1] + nums + [1]

        @cache
        def f(l: int, r: int) -> int:  # best coins from bursting everything strictly between l and r
            if l + 1 == r:
                return 0
            return max(f(l, k) + vals[l] * vals[k] * vals[r] + f(k, r) for k in range(l + 1, r))

        return f(0, len(vals) - 1)
```
**Complexity:** Time O(n³) · Space O(n²)

---

### 10. Regular Expression Matching
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/regular-expression-matching/)

**Problem:** Implement regex matching with `.` (any single character) and `*` (zero or more of the preceding element). The pattern must match the **entire** input string.

**Example:** `s = "aa", p = "a*"` → `True`; `s = "ab", p = ".*"` → `True`; `s = "mississippi", p = "mis*is*p*."` → `False`

**Key insight:** When `p[j+1] == '*'`, either skip `x*` entirely (`dp(i, j+2)`) or, if the first characters match, consume one character of `s` (`dp(i+1, j)`).

#### Approach 1: Top-Down Memoization
```python
from functools import cache


class Solution:
    def isMatch(self, s: str, p: str) -> bool:
        @cache
        def dp(i: int, j: int) -> bool:
            if j == len(p):
                return i == len(s)
            first = i < len(s) and p[j] in (s[i], ".")
            if j + 1 < len(p) and p[j + 1] == "*":
                return dp(i, j + 2) or (first and dp(i + 1, j))  # skip x*, or consume one char
            return first and dp(i + 1, j + 1)

        return dp(0, 0)
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Bottom-Up DP (Optimal)
```python
class Solution:
    def isMatch(self, s: str, p: str) -> bool:
        m, n = len(s), len(p)
        dp = [[False] * (n + 1) for _ in range(m + 1)]
        dp[m][n] = True
        for i in range(m, -1, -1):
            for j in range(n - 1, -1, -1):
                first = i < m and p[j] in (s[i], ".")
                if j + 1 < n and p[j + 1] == "*":
                    dp[i][j] = dp[i][j + 2] or (first and dp[i + 1][j])
                else:
                    dp[i][j] = first and dp[i + 1][j + 1]
        return dp[0][0]
```
**Complexity:** Time O(m · n) · Space O(m · n)

> **Shortcut for checking your work:** `re.fullmatch(p, s) is not None` gives the same answer for this problem's restricted syntax.

---
## 15. Greedy

### 53. Maximum Subarray
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/maximum-subarray/)

**Problem:** Return the largest sum of any contiguous non-empty subarray.

**Example:** `[-2,1,-3,4,-1,2,1,-5,4]` → `6` (`[4,-1,2,1]`)

**Key insight (Kadane's algorithm):** Throw away a running sum as soon as it goes negative, since it can only drag down whatever follows.

#### Approach 1: Kadane's Algorithm (Optimal)
```python
from typing import List


class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        best = cur = nums[0]
        for n in nums[1:]:
            cur = max(n, cur + n)  # drop the running sum once it would only drag n down
            best = max(best, cur)
        return best
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Divide and Conquer (the follow-up)
```python
from typing import List


class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        def solve(lo: int, hi: int) -> int:  # inclusive range
            if lo == hi:
                return nums[lo]
            mid = (lo + hi) // 2
            left_best = float("-inf")
            total = 0
            for i in range(mid, lo - 1, -1):
                total += nums[i]
                left_best = max(left_best, total)
            right_best, total = float("-inf"), 0
            for i in range(mid + 1, hi + 1):
                total += nums[i]
                right_best = max(right_best, total)
            return max(solve(lo, mid), solve(mid + 1, hi), left_best + right_best)

        return solve(0, len(nums) - 1)
```
**Complexity:** Time O(n log n) · Space O(log n)

#### Approach 3: Prefix Sums (min prefix so far)
```python
from typing import List


class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        best, prefix, min_prefix = float("-inf"), 0, 0
        for n in nums:
            prefix += n
            best = max(best, prefix - min_prefix)  # best subarray ending here
            min_prefix = min(min_prefix, prefix)
        return best
```
**Complexity:** Time O(n) · Space O(1)

---

### 55. Jump Game
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/jump-game/)

**Problem:** `nums[i]` is the farthest you can jump from index `i`. Starting at index 0, can you reach the last index?

**Example:** `[2,3,1,1,4]` → `True`; `[3,2,1,0,4]` → `False`

#### Approach 1: Greedy, Shift the Goal Backward (Optimal)
```python
from typing import List


class Solution:
    def canJump(self, nums: List[int]) -> bool:
        goal = len(nums) - 1
        for i in range(len(nums) - 2, -1, -1):
            if i + nums[i] >= goal:
                goal = i
        return goal == 0
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Greedy, Track the Farthest Reach Forward
```python
from typing import List


class Solution:
    def canJump(self, nums: List[int]) -> bool:
        reach = 0
        for i, n in enumerate(nums):
            if i > reach:
                return False
            reach = max(reach, i + n)
        return True
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 3: DP (slower, shown for contrast)
```python
from typing import List


class Solution:
    def canJump(self, nums: List[int]) -> bool:
        n = len(nums)
        good = [False] * n
        good[-1] = True
        for i in range(n - 2, -1, -1):
            good[i] = any(good[j] for j in range(i + 1, min(n, i + nums[i] + 1)))
        return good[0]
```
**Complexity:** Time O(n²) · Space O(n)

---

### 45. Jump Game II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/jump-game-ii/)

**Problem:** Same jumps as Jump Game, but the last index is always reachable. Return the minimum number of jumps to get there.

**Example:** `[2,3,1,1,4]` → `2`

**Key insight:** This is BFS by levels. The window `[l, r]` holds every index reachable in `jumps` jumps, and the next window ends at the farthest index reachable from it.

#### Approach 1: Greedy BFS Windows (Optimal)
```python
from typing import List


class Solution:
    def jump(self, nums: List[int]) -> int:
        jumps = cur_end = farthest = 0
        for i in range(len(nums) - 1):
            farthest = max(farthest, i + nums[i])
            if i == cur_end:  # end of this jump's window
                jumps += 1
                cur_end = farthest
        return jumps
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: DP
```python
from typing import List


class Solution:
    def jump(self, nums: List[int]) -> int:
        n = len(nums)
        dp = [0] + [float("inf")] * (n - 1)
        for i in range(n):
            for j in range(i + 1, min(n, i + nums[i] + 1)):
                dp[j] = min(dp[j], dp[i] + 1)
        return int(dp[-1])
```
**Complexity:** Time O(n · max jump) · Space O(n)

---

### 134. Gas Station
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/gas-station/)

**Problem:** Stations sit on a circular route. Station `i` provides `gas[i]`, and driving to the next station costs `cost[i]`. Return the starting station from which you can complete the full loop, or `-1` if none works. A valid answer is unique.

**Example:** `gas = [1,2,3,4,5], cost = [3,4,5,1,2]` → `3`

**Key insight:** If total gas ≥ total cost, a solution exists. Whenever the running tank goes negative at `i`, no station from the current start through `i` can work, so restart at `i+1`.

#### Approach 1: One-Pass Greedy (Optimal)
```python
from typing import List


class Solution:
    def canCompleteCircuit(self, gas: List[int], cost: List[int]) -> int:
        if sum(gas) < sum(cost):
            return -1
        tank = start = 0
        for i in range(len(gas)):
            tank += gas[i] - cost[i]
            if tank < 0:  # no station in [start, i] can work
                start, tank = i + 1, 0
        return start
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Brute Force Simulation
```python
from typing import List


class Solution:
    def canCompleteCircuit(self, gas: List[int], cost: List[int]) -> int:
        n = len(gas)
        for s in range(n):
            tank = 0
            for k in range(n):
                i = (s + k) % n
                tank += gas[i] - cost[i]
                if tank < 0:
                    break
            else:  # the loop finished without break: the full circuit worked
                return s
        return -1
```
**Complexity:** Time O(n²) · Space O(1)

---

### 846. Hand of Straights
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/hand-of-straights/)

**Problem:** Can the cards in `hand` be rearranged into groups of `groupSize` consecutive values, using every card exactly once?

**Example:** `hand = [1,2,3,6,2,3,4,7,8], groupSize = 3` → `True` (`[1,2,3]`, `[2,3,4]`, `[6,7,8]`)

**Key insight:** The smallest remaining card must start a group, so build each group upward from the current minimum.

#### Approach 1: Counter + Min-Heap of Distinct Values
```python
import heapq
from collections import Counter
from typing import List


class Solution:
    def isNStraightHand(self, hand: List[int], groupSize: int) -> bool:
        if len(hand) % groupSize:
            return False
        count = Counter(hand)
        heap = list(count)
        heapq.heapify(heap)
        while heap:
            first = heap[0]  # the smallest card must start a group
            for v in range(first, first + groupSize):
                if count[v] == 0:
                    return False
                count[v] -= 1
                if count[v] == 0:
                    if v != heap[0]:
                        return False  # a smaller value would be left unfinished
                    heapq.heappop(heap)
        return True
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sorted Keys, Consume Whole Runs at Once
```python
from collections import Counter
from typing import List


class Solution:
    def isNStraightHand(self, hand: List[int], groupSize: int) -> bool:
        if len(hand) % groupSize:
            return False
        count = Counter(hand)
        for start in sorted(count):
            need = count[start]
            if need == 0:
                continue
            for v in range(start, start + groupSize):  # start `need` groups at `start`
                if count[v] < need:
                    return False
                count[v] -= need
        return True
```
**Complexity:** Time O(n log n) · Space O(n)

---

### 1899. Merge Triplets to Form Target Triplet
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/merge-triplets-to-form-target-triplet/)

**Problem:** You may merge two triplets by taking the element-wise maximum. Can you produce `target` from some of the given `triplets`?

**Example:** `triplets = [[2,5,3],[1,8,4],[1,7,5]], target = [2,7,5]` → `True`

**Key insight:** Throw out any triplet that exceeds `target` in any position. Among the rest, check that each of the three target values appears in its position in at least one triplet.

#### Approach: Greedy Filter (Optimal)
```python
from typing import List


class Solution:
    def mergeTriplets(self, triplets: List[List[int]], target: List[int]) -> bool:
        found = set()
        for t in triplets:
            if all(t[i] <= target[i] for i in range(3)):  # anything that would overshoot is skipped
                found.update(i for i in range(3) if t[i] == target[i])
        return len(found) == 3
```
**Complexity:** Time O(n) · Space O(1)

---

### 763. Partition Labels
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/partition-labels/)

**Problem:** Split `s` into as many parts as possible so that each letter appears in only one part. Return the sizes of the parts.

**Example:** `"ababcbacadefegdehijhklij"` → `[9,7,8]`

**Key insight:** Record each letter's last index. Extend the current part's end to the last index of every letter you see, and close the part when `i` reaches that end.

#### Approach: Last Occurrence + Greedy Extension (Optimal)
```python
from typing import List


class Solution:
    def partitionLabels(self, s: str) -> List[int]:
        last = {c: i for i, c in enumerate(s)}  # later indices overwrite earlier ones
        res, start, end = [], 0, 0
        for i, c in enumerate(s):
            end = max(end, last[c])
            if i == end:
                res.append(end - start + 1)
                start = i + 1
        return res
```
**Complexity:** Time O(n) · Space O(1) (at most 26 keys)

---

### 678. Valid Parenthesis String
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/valid-parenthesis-string/)

**Problem:** The string contains `(`, `)`, and `*`, where each `*` can act as `(`, `)`, or an empty string. Return whether the string can be valid.

**Example:** `"(*))"` → `True`

**Key insight:** Track the range `[lo, hi]` of possible open-paren counts. If `hi` ever drops below 0, the string is invalid. Clamp `lo` at 0. It's valid if `lo == 0` at the end.

#### Approach 1: Greedy Min/Max Open Count (Optimal)
```python
class Solution:
    def checkValidString(self, s: str) -> bool:
        lo = hi = 0  # range of possible open-paren counts
        for c in s:
            if c == "(":
                lo, hi = lo + 1, hi + 1
            elif c == ")":
                lo, hi = lo - 1, hi - 1
            else:  # '*'
                lo, hi = lo - 1, hi + 1
            if hi < 0:
                return False
            lo = max(lo, 0)
        return lo == 0
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Two Stacks of Indices
```python
class Solution:
    def checkValidString(self, s: str) -> bool:
        open_, star = [], []
        for i, c in enumerate(s):
            if c == "(":
                open_.append(i)
            elif c == "*":
                star.append(i)
            elif open_:
                open_.pop()
            elif star:
                star.pop()
            else:
                return False
        while open_ and star:
            if open_.pop() > star.pop():
                return False  # a '(' with no '*' after it to close it
        return not open_
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 3: Memoized DP over (index, open count)
```python
from functools import cache


class Solution:
    def checkValidString(self, s: str) -> bool:
        @cache
        def ok(i: int, open_: int) -> bool:
            if open_ < 0:
                return False
            if i == len(s):
                return open_ == 0
            c = s[i]
            if c == "(":
                return ok(i + 1, open_ + 1)
            if c == ")":
                return ok(i + 1, open_ - 1)
            return ok(i + 1, open_ + 1) or ok(i + 1, open_ - 1) or ok(i + 1, open_)

        return ok(0, 0)
```
**Complexity:** Time O(n²) · Space O(n²)

---
## 16. Intervals

### 57. Insert Interval
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/insert-interval/)

**Problem:** `intervals` are non-overlapping and sorted by start. Insert `newInterval`, merging wherever needed, and return the result still sorted and non-overlapping.

**Example:** `[[1,3],[6,9]]` + `[2,5]` → `[[1,5],[6,9]]`

**Key insight:** Make three passes over the list: intervals entirely before the new one, intervals that overlap it (merge them), and intervals entirely after.

#### Approach 1: Linear Three-Phase Scan (Optimal)
```python
from typing import List


class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        res, i, n = [], 0, len(intervals)
        lo, hi = newInterval
        while i < n and intervals[i][1] < lo:  # entirely before
            res.append(intervals[i])
            i += 1
        while i < n and intervals[i][0] <= hi:  # overlapping: absorb
            lo, hi = min(lo, intervals[i][0]), max(hi, intervals[i][1])
            i += 1
        res.append([lo, hi])
        return res + intervals[i:]  # entirely after
```
**Complexity:** Time O(n) · Space O(n) for the output

#### Approach 2: Binary Search for the Boundaries
```python
import bisect
from typing import List


class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        lo, hi = newInterval
        ends = [iv[1] for iv in intervals]
        starts = [iv[0] for iv in intervals]
        l = bisect.bisect_left(ends, lo)    # first interval with end >= new start
        r = bisect.bisect_right(starts, hi)  # first interval with start > new end
        if l < r:
            lo, hi = min(lo, intervals[l][0]), max(hi, intervals[r - 1][1])
        return intervals[:l] + [[lo, hi]] + intervals[r:]
```
**Complexity:** Time O(n) (building the key lists dominates; the searches themselves are O(log n)) · Space O(n)

---

### 56. Merge Intervals
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/merge-intervals/)

**Problem:** Merge every overlapping interval and return the resulting non-overlapping intervals.

**Example:** `[[1,3],[2,6],[8,10],[15,18]]` → `[[1,6],[8,10],[15,18]]`

#### Approach 1: Sort + Merge (Optimal)
```python
from typing import List


class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        res = []
        for start, end in sorted(intervals):
            if res and start <= res[-1][1]:
                res[-1][1] = max(res[-1][1], end)
            else:
                res.append([start, end])
        return res
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sweep over Start and End Events
```python
from typing import List


class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        starts = sorted(iv[0] for iv in intervals)
        ends = sorted(iv[1] for iv in intervals)
        res, begin, n = [], 0, len(intervals)
        for i in range(n):
            # close a merged block when the next start begins after this end
            if i == n - 1 or starts[i + 1] > ends[i]:
                res.append([starts[begin], ends[i]])
                begin = i + 1
        return res
```
**Complexity:** Time O(n log n) · Space O(n)

---

### 435. Non-overlapping Intervals
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/non-overlapping-intervals/)

**Problem:** Return the minimum number of intervals to remove so the rest don't overlap. Intervals that only touch at an endpoint don't overlap.

**Example:** `[[1,2],[2,3],[3,4],[1,3]]` → `1`

**Key insight:** This is activity selection: always keep the interval that **ends** earliest, since that leaves the most room for the rest.

#### Approach 1: Sort by End (Optimal)
```python
from typing import List


class Solution:
    def eraseOverlapIntervals(self, intervals: List[List[int]]) -> int:
        removed, end = 0, float("-inf")
        for s, e in sorted(intervals, key=lambda iv: iv[1]):
            if s >= end:
                end = e  # keep it
            else:
                removed += 1
        return removed
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sort by Start, Drop the One That Ends Later
```python
from typing import List


class Solution:
    def eraseOverlapIntervals(self, intervals: List[List[int]]) -> int:
        intervals.sort()
        removed, prev_end = 0, intervals[0][1]
        for s, e in intervals[1:]:
            if s >= prev_end:
                prev_end = e
            else:
                removed += 1
                prev_end = min(prev_end, e)
        return removed
```
**Complexity:** Time O(n log n) · Space O(1) extra

---

### 252. Meeting Rooms (Premium)
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/meeting-rooms/) · Free mirror: LintCode 920

**Problem:** Given meeting time intervals, can one person attend every meeting (that is, do no two meetings overlap)?

**Example:** `[[0,30],[5,10],[15,20]]` → `False`

#### Approach: Sort by Start (Optimal)
```python
from typing import List


class Solution:
    def canAttendMeetings(self, intervals: List[List[int]]) -> bool:
        intervals.sort()
        return all(intervals[i][0] >= intervals[i - 1][1] for i in range(1, len(intervals)))
```
**Complexity:** Time O(n log n) · Space O(1) extra

**Brute force:** Check every pair for overlap: O(n²).

---

### 253. Meeting Rooms II (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/meeting-rooms-ii/) · Free mirror: LintCode 919

**Problem:** Return the minimum number of conference rooms needed to hold every meeting.

**Example:** `[[0,30],[5,10],[15,20]]` → `2`

**Key insight:** The answer is the maximum number of meetings running at the same moment.

#### Approach 1: Separate Sorted Starts and Ends (two pointers)
```python
from typing import List


class Solution:
    def minMeetingRooms(self, intervals: List[List[int]]) -> int:
        starts = sorted(iv[0] for iv in intervals)
        ends = sorted(iv[1] for iv in intervals)
        rooms = best = e = 0
        for s in starts:
            while e < len(ends) and ends[e] <= s:
                rooms -= 1  # that meeting has ended and freed its room
                e += 1
            rooms += 1
            best = max(best, rooms)
        return best
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Min-Heap of End Times
```python
import heapq
from typing import List


class Solution:
    def minMeetingRooms(self, intervals: List[List[int]]) -> int:
        ends = []
        for s, e in sorted(intervals):
            if ends and ends[0] <= s:
                heapq.heapreplace(ends, e)  # reuse the room that frees up earliest
            else:
                heapq.heappush(ends, e)
        return len(ends)
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 3: Line Sweep with a Counter of Events
```python
from collections import Counter
from itertools import accumulate
from typing import List


class Solution:
    def minMeetingRooms(self, intervals: List[List[int]]) -> int:
        delta = Counter()
        for s, e in intervals:
            delta[s] += 1
            delta[e] -= 1
        return max(accumulate(delta[t] for t in sorted(delta)), default=0)
```
**Complexity:** Time O(n log n) · Space O(n)

---

### 1851. Minimum Interval to Include Each Query
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/minimum-interval-to-include-each-query/)

**Problem:** For each `queries[j]`, find the size (`right - left + 1`) of the smallest interval that contains it, or `-1` if no interval does.

**Example:** `intervals = [[1,4],[2,4],[3,6],[4,4]], queries = [2,3,4,5]` → `[3,3,1,4]`

**Key insight:** Process queries in increasing order (offline). Push every interval whose start ≤ q onto a min-heap keyed by size, then pop from the top any interval whose end < q.

#### Approach 1: Sort + Min-Heap (Optimal)
```python
import heapq
from typing import List


class Solution:
    def minInterval(self, intervals: List[List[int]], queries: List[int]) -> List[int]:
        intervals.sort()
        res, heap, i = {}, [], 0  # heap of (size, end)
        for q in sorted(set(queries)):
            while i < len(intervals) and intervals[i][0] <= q:
                l, r = intervals[i]
                heapq.heappush(heap, (r - l + 1, r))
                i += 1
            while heap and heap[0][1] < q:
                heapq.heappop(heap)  # ends before q
            res[q] = heap[0][0] if heap else -1
        return [res[q] for q in queries]
```
**Complexity:** Time O(n log n + q log q) · Space O(n + q)

#### Approach 2: Sort Intervals by Size + Union-Find "Next Unanswered Query"
Visit intervals from smallest to largest. Each one answers every unanswered query it covers. The union-find jumps over queries that already have answers.
```python
import bisect
from typing import List


class Solution:
    def minInterval(self, intervals: List[List[int]], queries: List[int]) -> List[int]:
        qs = sorted(set(queries))
        nxt = list(range(len(qs) + 1))  # nxt[i] = next unanswered index >= i
        ans = [-1] * len(qs)

        def find(x: int) -> int:
            while nxt[x] != x:
                nxt[x] = nxt[nxt[x]]
                x = nxt[x]
            return x

        for l, r in sorted(intervals, key=lambda iv: iv[1] - iv[0]):
            j = find(bisect.bisect_left(qs, l))
            while j < len(qs) and qs[j] <= r:
                ans[j] = r - l + 1
                nxt[j] = j + 1  # mark answered
                j = find(j + 1)
        pos = {q: i for i, q in enumerate(qs)}
        return [ans[pos[q]] for q in queries]
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
```python
from typing import List


class Solution:
    def rotate(self, matrix: List[List[int]]) -> None:
        """Do not return anything, modify matrix in-place instead."""
        n = len(matrix)
        for i in range(n):
            for j in range(i + 1, n):
                matrix[i][j], matrix[j][i] = matrix[j][i], matrix[i][j]
        for row in matrix:
            row.reverse()
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Rotate Four Cells at a Time, Layer by Layer
```python
from typing import List


class Solution:
    def rotate(self, matrix: List[List[int]]) -> None:
        l, r = 0, len(matrix) - 1
        while l < r:
            for i in range(r - l):
                top, bottom = l, r
                tmp = matrix[top][l + i]                       # save top-left
                matrix[top][l + i] = matrix[bottom - i][l]     # bottom-left to top-left
                matrix[bottom - i][l] = matrix[bottom][r - i]  # bottom-right to bottom-left
                matrix[bottom][r - i] = matrix[top + i][r]     # top-right to bottom-right
                matrix[top + i][r] = tmp                       # top-left to top-right
            l, r = l + 1, r - 1
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 3: Pythonic `zip` (not in place, then copied back)
```python
from typing import List


class Solution:
    def rotate(self, matrix: List[List[int]]) -> None:
        matrix[:] = [list(row) for row in zip(*matrix[::-1])]  # slice assignment keeps the same list object
```
**Complexity:** Time O(n²) · Space O(n²) temporary

---

### 54. Spiral Matrix
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/spiral-matrix/)

**Problem:** Return every element of an `m × n` matrix in spiral order (clockwise, starting at the top-left).

**Example:** `[[1,2,3],[4,5,6],[7,8,9]]` → `[1,2,3,6,9,8,7,4,5]`

#### Approach 1: Shrinking Boundaries (Optimal)
```python
from typing import List


class Solution:
    def spiralOrder(self, matrix: List[List[int]]) -> List[int]:
        res = []
        top, bottom, left, right = 0, len(matrix) - 1, 0, len(matrix[0]) - 1
        while top <= bottom and left <= right:
            res += [matrix[top][c] for c in range(left, right + 1)]
            top += 1
            res += [matrix[r][right] for r in range(top, bottom + 1)]
            right -= 1
            if top <= bottom:
                res += [matrix[bottom][c] for c in range(right, left - 1, -1)]
                bottom -= 1
            if left <= right:
                res += [matrix[r][left] for r in range(bottom, top - 1, -1)]
                left += 1
        return res
```
**Complexity:** Time O(m · n) · Space O(1) extra

#### Approach 2: Peel the First Row, Rotate the Rest (Pythonic)
```python
from typing import List


class Solution:
    def spiralOrder(self, matrix: List[List[int]]) -> List[int]:
        res = []
        while matrix:
            res += matrix.pop(0)                       # take the top row
            matrix = [list(r) for r in zip(*matrix)][::-1]  # rotate the rest counter-clockwise
        return res
```
**Complexity:** Time O(m · n · min(m, n)) from repeated rotation · Space O(m · n). Elegant, but slower.

---

### 73. Set Matrix Zeroes
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/set-matrix-zeroes/)

**Problem:** Wherever the matrix has a `0`, set that cell's entire row and column to `0`, in place.

**Example:** `[[1,1,1],[1,0,1],[1,1,1]]` → `[[1,0,1],[0,0,0],[1,0,1]]`

**Key insight:** Use the first row and first column as marker arrays, with one extra flag for whether the first column itself must be zeroed.

#### Approach 1: Row and Column Sets
```python
from typing import List


class Solution:
    def setZeroes(self, matrix: List[List[int]]) -> None:
        """Do not return anything, modify matrix in-place instead."""
        m, n = len(matrix), len(matrix[0])
        rows = {i for i in range(m) for j in range(n) if matrix[i][j] == 0}
        cols = {j for i in range(m) for j in range(n) if matrix[i][j] == 0}
        for i in range(m):
            for j in range(n):
                if i in rows or j in cols:
                    matrix[i][j] = 0
```
**Complexity:** Time O(m · n) · Space O(m + n)

#### Approach 2: First Row and Column as Markers (Optimal)
```python
from typing import List


class Solution:
    def setZeroes(self, matrix: List[List[int]]) -> None:
        m, n = len(matrix), len(matrix[0])
        first_col_zero = False
        for i in range(m):
            if matrix[i][0] == 0:
                first_col_zero = True
            for j in range(1, n):
                if matrix[i][j] == 0:
                    matrix[i][0] = matrix[0][j] = 0
        for i in range(m - 1, -1, -1):  # bottom-up so row 0's markers are used before row 0 is zeroed
            for j in range(n - 1, 0, -1):
                if matrix[i][0] == 0 or matrix[0][j] == 0:
                    matrix[i][j] = 0
            if first_col_zero:
                matrix[i][0] = 0
```
**Complexity:** Time O(m · n) · Space O(1)

---

### 202. Happy Number
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/happy-number/)

**Problem:** Repeatedly replace `n` with the sum of the squares of its digits. `n` is *happy* if this process reaches 1. Otherwise it loops forever. Return whether `n` is happy.

**Example:** `19` → `True` (1²+9²=82 → 68 → 100 → 1)

**Key insight:** The sequence either reaches 1 or enters a cycle. Detect the cycle with a set or with Floyd's algorithm.

#### Approach 1: Hash Set
```python
class Solution:
    def isHappy(self, n: int) -> bool:
        seen = set()
        while n != 1 and n not in seen:
            seen.add(n)
            n = sum(int(d) ** 2 for d in str(n))
        return n == 1
```
**Complexity:** Time O(log n) per step, with only a few steps · Space O(log n)

#### Approach 2: Floyd's Cycle Detection (Optimal)
```python
class Solution:
    def isHappy(self, n: int) -> bool:
        def nxt(x: int) -> int:
            total = 0
            while x:
                x, d = divmod(x, 10)
                total += d * d
            return total

        slow, fast = n, nxt(n)
        while fast != 1 and slow != fast:
            slow, fast = nxt(slow), nxt(nxt(fast))
        return fast == 1
```
**Complexity:** Time O(log n) · Space O(1)

---

### 66. Plus One
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/plus-one/)

**Problem:** A large integer is stored as an array of digits, most significant first. Add one and return the resulting digits.

**Example:** `[1,2,9]` → `[1,3,0]`; `[9,9]` → `[1,0,0]`

#### Approach 1: Propagate the Carry from the Right (Optimal)
```python
from typing import List


class Solution:
    def plusOne(self, digits: List[int]) -> List[int]:
        for i in range(len(digits) - 1, -1, -1):
            if digits[i] < 9:
                digits[i] += 1
                return digits
            digits[i] = 0
        return [1] + digits  # every digit was 9
```
**Complexity:** Time O(n) · Space O(1), or O(n) when every digit is 9

#### Approach 2: Big-Integer Round Trip (Pythonic)
```python
from typing import List


class Solution:
    def plusOne(self, digits: List[int]) -> List[int]:
        return [int(d) for d in str(int("".join(map(str, digits))) + 1)]
```
**Complexity:** Time O(n) · Space O(n). Fine in Python since ints are unbounded; it misses the point in an interview.

---

### 50. Pow(x, n)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/powx-n/)

**Problem:** Compute `x` raised to the power `n`, where `n` can be negative (as low as −2³¹).

**Example:** `x = 2.0, n = 10` → `1024.0`; `x = 2.0, n = -2` → `0.25`

**Key insight:** Binary exponentiation: `x^n = (x²)^(n/2)`, with an extra factor of `x` when `n` is odd.

#### Approach 1: Iterative Fast Exponentiation (Optimal)
```python
class Solution:
    def myPow(self, x: float, n: int) -> float:
        if n < 0:
            x, n = 1 / x, -n  # Python ints don't overflow, so -n is always safe
        res = 1.0
        while n:
            if n & 1:
                res *= x
            x *= x
            n >>= 1
        return res
```
**Complexity:** Time O(log n) · Space O(1)

#### Approach 2: Recursive
```python
class Solution:
    def myPow(self, x: float, n: int) -> float:
        if n == 0:
            return 1.0
        if n < 0:
            return 1 / self.myPow(x, -n)
        half = self.myPow(x, n // 2)
        return half * half * (x if n % 2 else 1)
```
**Complexity:** Time O(log n) · Space O(log n)

#### Approach 3: Built-in `**`
```python
class Solution:
    def myPow(self, x: float, n: int) -> float:
        return x**n
```
**Complexity:** Time O(log n) · Space O(1). Good for checking your work, but write Approach 1 in an interview.

---

### 43. Multiply Strings
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/multiply-strings/)

**Problem:** Multiply two non-negative integers given as strings and return the product as a string, without converting the inputs to integers directly.

**Example:** `"123" × "456"` → `"56088"`

**Key insight:** The product of `num1[i]` and `num2[j]` lands in positions `i+j` and `i+j+1` of an array of length `m+n`.

#### Approach: Grade-School Multiplication into a Position Array (Optimal)
```python
class Solution:
    def multiply(self, num1: str, num2: str) -> str:
        if num1 == "0" or num2 == "0":
            return "0"
        m, n = len(num1), len(num2)
        pos = [0] * (m + n)
        for i in range(m - 1, -1, -1):
            for j in range(n - 1, -1, -1):
                total = (ord(num1[i]) - 48) * (ord(num2[j]) - 48) + pos[i + j + 1]
                pos[i + j + 1] = total % 10
                pos[i + j] += total // 10
        return "".join(map(str, pos)).lstrip("0")  # at most one leading zero
```
**Complexity:** Time O(m · n) · Space O(m + n)

**Note:** `str(int(num1) * int(num2))` would be a one-liner, but the problem forbids converting the inputs to integers directly.

---

### 2013. Detect Squares
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/detect-squares/)

**Problem:** Design `DetectSquares`. `add(point)` adds a point; duplicates are allowed. `count(point)` returns how many ways you can pick three stored points that form an axis-aligned square of positive area together with the query point.

**Key insight:** Treat each stored point that is diagonal to the query (`|dx| == |dy| ≠ 0`) as the opposite corner. Multiply the counts of the other two corners.

#### Approach 1: Counter + Iterate Diagonal Candidates (Optimal)
```python
from collections import Counter
from typing import List


class DetectSquares:
    def __init__(self):
        self.cnt = Counter()  # (x, y) -> count

    def add(self, point: List[int]) -> None:
        self.cnt[tuple(point)] += 1

    def count(self, point: List[int]) -> int:
        qx, qy = point
        res = 0
        for (x, y), c in list(self.cnt.items()):  # list(): Counter lookups below may insert zero keys
            if x != qx and abs(x - qx) == abs(y - qy):  # diagonal corners only
                res += c * self.cnt[(qx, y)] * self.cnt[(x, qy)]
        return res
```
**Complexity:** `add` O(1) · `count` O(P) for P distinct points · Space O(P)

> Looking up a missing key on a `Counter` returns 0 **without** inserting it. The `list(...)` is still a cheap safeguard against "dict changed size during iteration".

#### Approach 2: Points Grouped by x-Coordinate
```python
from collections import Counter, defaultdict
from typing import List


class DetectSquares:
    def __init__(self):
        self.by_x = defaultdict(Counter)  # x -> Counter of y

    def add(self, point: List[int]) -> None:
        x, y = point
        self.by_x[x][y] += 1

    def count(self, point: List[int]) -> int:
        qx, qy = point
        if qx not in self.by_x:
            return 0
        res = 0
        for y, c in self.by_x[qx].items():
            side = y - qy
            if side == 0:
                continue
            for x in (qx + side, qx - side):  # square to the right or to the left
                if x in self.by_x:
                    col = self.by_x[x]
                    res += c * col[y] * col[qy]
        return res
```
**Complexity:** `add` O(1) · `count` O(points sharing the query's x) · Space O(P)

---
## 18. Bit Manipulation

> Python ints are arbitrary-precision, and negative numbers behave as if they had infinitely many leading 1 bits. For problems defined on 32-bit values, mask with `& 0xFFFFFFFF` and convert back to signed at the end. Useful built-ins: `int.bit_count()` (3.10+), `bin(x).count("1")`, and `int.bit_length()`.

### 136. Single Number
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/single-number/)

**Problem:** Every element appears twice except one. Find that one in O(n) time and O(1) extra space.

**Example:** `[4,1,2,1,2]` → `4`

**Key insight:** `a ^ a = 0` and `a ^ 0 = a`, so XOR-ing everything cancels the pairs.

#### Approach 1: XOR (Optimal)
```python
from functools import reduce
from operator import xor
from typing import List


class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        return reduce(xor, nums)
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Set Toggle
```python
from typing import List


class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        seen = set()
        for n in nums:
            seen ^= {n}  # symmetric difference toggles membership
        return seen.pop()
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 3: Math
```python
from typing import List


class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        return 2 * sum(set(nums)) - sum(nums)
```
**Complexity:** Time O(n) · Space O(n)

---

### 191. Number of 1 Bits
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/number-of-1-bits/)

**Problem:** Return the number of set bits (the Hamming weight) in `n`.

**Example:** `11` (binary `1011`) → `3`

#### Approach 1: Check Every Bit
```python
class Solution:
    def hammingWeight(self, n: int) -> int:
        count = 0
        while n:
            count += n & 1
            n >>= 1
        return count
```
**Complexity:** Time O(32) · Space O(1)

#### Approach 2: Brian Kernighan's Trick (Optimal)
`n & (n-1)` clears the lowest set bit, so the loop runs once per set bit.
```python
class Solution:
    def hammingWeight(self, n: int) -> int:
        count = 0
        while n:
            n &= n - 1
            count += 1
        return count
```
**Complexity:** Time O(number of set bits) · Space O(1)

#### Approach 3: Built-in
```python
class Solution:
    def hammingWeight(self, n: int) -> int:
        return bin(n).count("1")  # or n.bit_count() on Python 3.10+
```
**Complexity:** Time O(32) · Space O(32)

---

### 338. Counting Bits
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/counting-bits/)

**Problem:** For every `i` in `0..n`, return the number of set bits in `i`.

**Example:** `n = 5` → `[0,1,1,2,1,2]`

**Key insight:** `bits(i) = bits(i >> 1) + (i & 1)`, or equivalently `bits(i) = bits(i & (i-1)) + 1`.

#### Approach 1: DP on the Right Shift (Optimal)
```python
from typing import List


class Solution:
    def countBits(self, n: int) -> List[int]:
        res = [0] * (n + 1)
        for i in range(1, n + 1):
            res[i] = res[i >> 1] + (i & 1)
        return res
```
**Complexity:** Time O(n) · Space O(1) extra

#### Approach 2: DP on the Lowest Set Bit
```python
from typing import List


class Solution:
    def countBits(self, n: int) -> List[int]:
        res = [0] * (n + 1)
        for i in range(1, n + 1):
            res[i] = res[i & (i - 1)] + 1
        return res
```
**Complexity:** Time O(n) · Space O(1) extra

#### Approach 3: Popcount Each Number
```python
from typing import List


class Solution:
    def countBits(self, n: int) -> List[int]:
        return [bin(i).count("1") for i in range(n + 1)]
```
**Complexity:** Time O(n log n) · Space O(1) extra

---

### 190. Reverse Bits
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/reverse-bits/)

**Problem:** Reverse the bits of a 32-bit unsigned integer.

**Example:** `00000010100101000001111010011100` → `00111001011110000010100101000000`

#### Approach 1: Bit-by-Bit Shift
```python
class Solution:
    def reverseBits(self, n: int) -> int:
        res = 0
        for _ in range(32):
            res = (res << 1) | (n & 1)
            n >>= 1
        return res
```
**Complexity:** Time O(32) · Space O(1)

#### Approach 2: Divide and Conquer Swaps (Optimal)
```python
class Solution:
    def reverseBits(self, n: int) -> int:
        n = ((n >> 16) | (n << 16)) & 0xFFFFFFFF
        n = ((n & 0xFF00FF00) >> 8) | ((n & 0x00FF00FF) << 8)
        n = ((n & 0xF0F0F0F0) >> 4) | ((n & 0x0F0F0F0F) << 4)
        n = ((n & 0xCCCCCCCC) >> 2) | ((n & 0x33333333) << 2)
        n = ((n & 0xAAAAAAAA) >> 1) | ((n & 0x55555555) << 1)
        return n
```
**Complexity:** Time O(1) (5 steps) · Space O(1)

#### Approach 3: String Formatting
```python
class Solution:
    def reverseBits(self, n: int) -> int:
        return int(f"{n:032b}"[::-1], 2)
```
**Complexity:** Time O(32) · Space O(32)

---

### 268. Missing Number
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/missing-number/)

**Problem:** `nums` contains `n` distinct numbers from the range `[0, n]`. Return the one that's missing.

**Example:** `[3,0,1]` → `2`

#### Approach 1: Gauss Sum
```python
from typing import List


class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        n = len(nums)
        return n * (n + 1) // 2 - sum(nums)
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: XOR of Indices and Values (Optimal)
```python
from typing import List


class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        res = len(nums)
        for i, v in enumerate(nums):
            res ^= i ^ v
        return res
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 3: Set Difference
```python
from typing import List


class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        return (set(range(len(nums) + 1)) - set(nums)).pop()
```
**Complexity:** Time O(n) · Space O(n)

---

### 371. Sum of Two Integers
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/sum-of-two-integers/)

**Problem:** Return `a + b` without using the `+` or `-` operators.

**Example:** `a = 1, b = 2` → `3`; `a = -2, b = 3` → `1`

**Key insight:** `a ^ b` is the sum without carries, and `(a & b) << 1` is the carry. Repeat until the carry is zero. Work in `int32` to get LeetCode's 32-bit two's-complement behavior.

#### Approach 1: Iterative XOR/AND with a 32-bit Mask (Optimal)
Python ints don't wrap, so without the mask a negative carry would loop forever. Simulate 32-bit registers and convert back to signed at the end.
```python
class Solution:
    def getSum(self, a: int, b: int) -> int:
        MASK, MAX_INT = 0xFFFFFFFF, 0x7FFFFFFF
        a, b = a & MASK, b & MASK
        while b:
            a, b = (a ^ b) & MASK, ((a & b) << 1) & MASK  # sum without carry, carry
        return a if a <= MAX_INT else ~(a ^ MASK)  # reinterpret bit 31 as the sign
```
**Complexity:** Time O(32) · Space O(1)

#### Approach 2: Recursive with the Same Mask
```python
class Solution:
    def getSum(self, a: int, b: int) -> int:
        MASK = 0xFFFFFFFF

        def add(x: int, y: int) -> int:
            return x if y == 0 else add((x ^ y) & MASK, ((x & y) << 1) & MASK)

        res = add(a & MASK, b & MASK)
        return res if res < 0x80000000 else res - (1 << 32)
```
**Complexity:** Time O(32) · Space O(32)

---

### 7. Reverse Integer
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/reverse-integer/)

**Problem:** Reverse the digits of a signed 32-bit integer `x`. If the result falls outside `[-2³¹, 2³¹-1]`, return `0`. Assume 64-bit storage isn't available.

**Example:** `123` → `321`; `-123` → `-321`; `1534236469` → `0`

**Key insight:** Before computing `res*10 + d`, check whether it would overflow by comparing `res` against `MaxInt32/10` and `MinInt32/10`.

#### Approach 1: Pop and Push with Overflow Guards (Optimal)
The checks mirror the "no 64-bit storage" rule even though Python wouldn't overflow. The positive limit is 2³¹ − 1 and the negative limit's magnitude is 2³¹.
```python
class Solution:
    def reverse(self, x: int) -> int:
        sign = -1 if x < 0 else 1
        limit = 2**31 - 1 if sign > 0 else 2**31
        x, res = abs(x), 0
        while x:
            x, d = divmod(x, 10)
            if res > (limit - d) // 10:  # res*10 + d would exceed the limit
                return 0
            res = res * 10 + d
        return sign * res
```
**Complexity:** Time O(log₁₀ x) · Space O(1)

#### Approach 2: String Reversal (Pythonic)
```python
class Solution:
    def reverse(self, x: int) -> int:
        res = int(str(abs(x))[::-1]) * (-1 if x < 0 else 1)
        return res if -(2**31) <= res <= 2**31 - 1 else 0
```
**Complexity:** Time O(log₁₀ x) · Space O(log₁₀ x)

---

## Complexity Cheat Sheet

| Pattern | Typical time | Typical space | Signals in the problem | Python tools |
|---|---|---|---|---|
| Hash map / set | O(n) | O(n) | "seen before", duplicates, complements | `dict`, `set`, `Counter`, `defaultdict` |
| Two pointers | O(n) | O(1) | sorted input, pairs, palindromes | index pointers, tuple swaps |
| Sliding window | O(n) | O(k) | contiguous substring or subarray with a constraint | `Counter`, `[0] * 26` |
| Monotonic stack | O(n) | O(n) | next greater or smaller element, histograms | `list` as a stack |
| Binary search | O(log n) | O(1) | sorted input, or "minimum k such that…" | `bisect_left`, `bisect_right` |
| Heap | O(n log k) | O(k) | top-k, k-way merge, streaming median | `heapq`, `nlargest`, `heapq.merge` |
| DFS / BFS | O(V + E) | O(V) | grids, connectivity, shortest path in an unweighted graph | `deque`, `sys.setrecursionlimit` |
| Topological sort | O(V + E) | O(V + E) | dependencies, ordering, cycle detection | in-degree list + `deque`, `graphlib` |
| Union-find | O(α(n)) per operation | O(n) | dynamic connectivity, redundant edges | `parent = list(range(n))` |
| Dijkstra | O(E log V) | O(V + E) | weighted shortest path with non-negative weights | `heapq` of `(dist, node)` |
| Backtracking | O(2ⁿ) / O(n!) | O(n) | "all combinations / permutations / partitions" | `path[:]`, `itertools` |
| 1-D DP | O(n) to O(n²) | O(1) to O(n) | best value over a prefix | `@cache` (shallow) or dict memo (deep) |
| 2-D DP | O(m · n) | O(n) when rolled | two strings, grids, intervals | list-of-lists, rolling rows |
| Greedy | O(n) or O(n log n) | O(1) | a locally best choice provably works | `sorted(key=…)` + scan |
| Bit tricks | O(1) to O(n) | O(1) | XOR pairing, powers of two, bit counts | `& 0xFFFFFFFF`, `bit_count`, `bin` |

---

*End of document: 150 problems, 18 categories.*
