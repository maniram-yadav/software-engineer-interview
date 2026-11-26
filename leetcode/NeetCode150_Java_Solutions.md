# LeetCode "Blind 150" (NeetCode 150) — Complete Java Solutions

A full Java reference for the 150 problems in the **NeetCode 150** list (the Blind 75 plus 75 more), grouped by pattern. Every problem has:

- its **LeetCode number and title**, with difficulty
- a **short problem statement** in my own words, plus an example
- one or more **complete Java solutions**: brute force → better → optimal where that comparison helps
- **time and space complexity** for each approach
- a **key insight** line explaining the idea

> Every Java snippet in this file was compiled with **JDK 21** (`javac`), and all 150 problems were run on randomized tests: every alternative approach was checked against the others and, where possible, against an independent reference answer (about 92,000 checks, 0 failures). Each approach is a standalone `class Solution { ... }` with LeetCode's exact method signature, so you can paste any of them straight into the LeetCode editor. Design problems (`MinStack`, `LRUCache`, …) use LeetCode's class names; alternative designs carry a suffix such as `MinStack2`. Code needs **Java 8+** unless a note says otherwise. LeetCode auto-imports `java.util.*`, so imports are omitted.

---

## Shared Definitions

LeetCode provides these classes for linked-list and tree problems. They are assumed (not redeclared) in those sections.

```java
// Definition for singly-linked list.
public class ListNode {
    int val;
    ListNode next;
    ListNode() {}
    ListNode(int val) { this.val = val; }
    ListNode(int val, ListNode next) { this.val = val; this.next = next; }
}

// Definition for a binary tree node.
public class TreeNode {
    int val;
    TreeNode left, right;
    TreeNode() {}
    TreeNode(int val) { this.val = val; }
    TreeNode(int val, TreeNode left, TreeNode right) { this.val = val; this.left = left; this.right = right; }
}
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

## Java Interview Toolkit (quick reference)

```java
// Collections
Map<Integer, Integer> map = new HashMap<>();   map.merge(k, 1, Integer::sum);   map.getOrDefault(k, 0);
Set<Integer> set = new HashSet<>();            set.add(x) returns false if x was already present
Deque<Integer> stack = new ArrayDeque<>();     stack.push(x); stack.pop(); stack.peek();
Deque<Integer> queue = new ArrayDeque<>();     queue.offer(x); queue.poll(); queue.peek();
List<List<Integer>> res = new ArrayList<>();   res.add(new ArrayList<>(path));  // copy the path!

// Heaps
PriorityQueue<Integer> minHeap = new PriorityQueue<>();
PriorityQueue<Integer> maxHeap = new PriorityQueue<>(Collections.reverseOrder());
PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[0] - b[0]); // use Integer.compare to avoid overflow

// Sorting
Arrays.sort(arr);                                    // primitives: dual-pivot quicksort
Arrays.sort(intervals, (a, b) -> Integer.compare(a[0], b[0]));
Collections.sort(list);

// Ordered map (BST) for floor/ceiling queries
TreeMap<Integer, Integer> tm = new TreeMap<>(); tm.floorKey(x); tm.ceilingKey(x); tm.firstKey();

// Strings
StringBuilder sb = new StringBuilder(); char[] cs = s.toCharArray(); String.valueOf(cs);
```

**Java pitfalls to watch for:**
- Compare boxed `Integer` objects with `.equals()`, not `==`. Values outside -128..127 are not cached, so `==` can fail.
- In comparators, `a - b` can overflow. Use `Integer.compare(a, b)`.
- Use `ArrayDeque` for stacks and queues, not the legacy `Stack` class (synchronized) or `LinkedList` (slower).
- `int` overflow: `(lo + hi) / 2` overflows, so use `lo + (hi - lo) / 2`. Use `long` for products.

---
## 1. Arrays & Hashing

### 217. Contains Duplicate
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/contains-duplicate/)

**Problem:** Given an integer array `nums`, return `true` if any value appears at least twice, and `false` if every element is distinct.

**Example:** `nums = [1,2,3,1]` → `true`; `nums = [1,2,3,4]` → `false`

**Key insight:** A hash set gives O(1) membership checks, so you can spot a repeat in one pass.

#### Approach 1: Brute Force
```java
class Solution {
    public boolean containsDuplicate(int[] nums) {
        for (int i = 0; i < nums.length; i++)
            for (int j = i + 1; j < nums.length; j++)
                if (nums[i] == nums[j]) return true;
        return false;
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Sorting
```java
class Solution {
    public boolean containsDuplicate(int[] nums) {
        Arrays.sort(nums);
        for (int i = 1; i < nums.length; i++)
            if (nums[i] == nums[i - 1]) return true;
        return false;
    }
}
```
**Complexity:** Time O(n log n) · Space O(log n) for the sort

#### Approach 3: HashSet (Optimal)
```java
class Solution {
    public boolean containsDuplicate(int[] nums) {
        Set<Integer> seen = new HashSet<>();
        for (int n : nums)
            if (!seen.add(n)) return true; // add() returns false if n is already present
        return false;
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
```java
class Solution {
    public boolean isAnagram(String s, String t) {
        if (s.length() != t.length()) return false;
        char[] a = s.toCharArray(), b = t.toCharArray();
        Arrays.sort(a);
        Arrays.sort(b);
        return Arrays.equals(a, b);
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Frequency Array (Optimal)
```java
class Solution {
    public boolean isAnagram(String s, String t) {
        if (s.length() != t.length()) return false;
        int[] count = new int[26];
        for (int i = 0; i < s.length(); i++) {
            count[s.charAt(i) - 'a']++;
            count[t.charAt(i) - 'a']--;
        }
        for (int c : count) if (c != 0) return false;
        return true;
    }
}
```
**Complexity:** Time O(n) · Space O(1) (26 counters)

#### Approach 3: HashMap (handles Unicode follow-up)
```java
class Solution {
    public boolean isAnagram(String s, String t) {
        if (s.length() != t.length()) return false;
        Map<Integer, Integer> count = new HashMap<>();
        s.codePoints().forEach(c -> count.merge(c, 1, Integer::sum));
        for (int c : t.codePoints().toArray()) {
            int left = count.getOrDefault(c, 0) - 1;
            if (left < 0) return false;
            count.put(c, left);
        }
        return true;
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
```java
class Solution {
    public int[] twoSum(int[] nums, int target) {
        for (int i = 0; i < nums.length; i++)
            for (int j = i + 1; j < nums.length; j++)
                if (nums[i] + nums[j] == target) return new int[]{i, j};
        return new int[0];
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: One-Pass HashMap (Optimal)
```java
class Solution {
    public int[] twoSum(int[] nums, int target) {
        Map<Integer, Integer> idx = new HashMap<>();
        for (int i = 0; i < nums.length; i++) {
            Integer j = idx.get(target - nums[i]);
            if (j != null) return new int[]{j, i};
            idx.put(nums[i], i);
        }
        return new int[0];
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
```java
class Solution {
    public List<List<String>> groupAnagrams(String[] strs) {
        Map<String, List<String>> groups = new HashMap<>();
        for (String s : strs) {
            char[] cs = s.toCharArray();
            Arrays.sort(cs);
            groups.computeIfAbsent(new String(cs), k -> new ArrayList<>()).add(s);
        }
        return new ArrayList<>(groups.values());
    }
}
```
**Complexity:** Time O(m · k log k), for m strings of max length k · Space O(m · k)

#### Approach 2: Count Signature as Key (Optimal)
```java
class Solution {
    public List<List<String>> groupAnagrams(String[] strs) {
        Map<String, List<String>> groups = new HashMap<>();
        for (String s : strs) {
            int[] cnt = new int[26];
            for (char c : s.toCharArray()) cnt[c - 'a']++;
            StringBuilder key = new StringBuilder();
            for (int c : cnt) key.append('#').append(c); // '#' keeps counts like 1,11 unambiguous
            groups.computeIfAbsent(key.toString(), k -> new ArrayList<>()).add(s);
        }
        return new ArrayList<>(groups.values());
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
```java
class Solution {
    public int[] topKFrequent(int[] nums, int k) {
        Map<Integer, Integer> freq = new HashMap<>();
        for (int n : nums) freq.merge(n, 1, Integer::sum);
        List<Integer> keys = new ArrayList<>(freq.keySet());
        keys.sort((a, b) -> freq.get(b) - freq.get(a));
        int[] res = new int[k];
        for (int i = 0; i < k; i++) res[i] = keys.get(i);
        return res;
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Min-Heap of Size k
```java
class Solution {
    public int[] topKFrequent(int[] nums, int k) {
        Map<Integer, Integer> freq = new HashMap<>();
        for (int n : nums) freq.merge(n, 1, Integer::sum);
        PriorityQueue<Integer> heap = new PriorityQueue<>((a, b) -> freq.get(a) - freq.get(b));
        for (int v : freq.keySet()) {
            heap.offer(v);
            if (heap.size() > k) heap.poll();
        }
        int[] res = new int[k];
        for (int i = k - 1; i >= 0; i--) res[i] = heap.poll();
        return res;
    }
}
```
**Complexity:** Time O(n log k) · Space O(n)

#### Approach 3: Bucket Sort (Optimal)
```java
class Solution {
    public int[] topKFrequent(int[] nums, int k) {
        Map<Integer, Integer> freq = new HashMap<>();
        for (int n : nums) freq.merge(n, 1, Integer::sum);
        @SuppressWarnings("unchecked")
        List<Integer>[] buckets = new List[nums.length + 1]; // index = frequency
        for (Map.Entry<Integer, Integer> e : freq.entrySet()) {
            int f = e.getValue();
            if (buckets[f] == null) buckets[f] = new ArrayList<>();
            buckets[f].add(e.getKey());
        }
        int[] res = new int[k];
        int idx = 0;
        for (int f = buckets.length - 1; f >= 0 && idx < k; f--) {
            if (buckets[f] == null) continue;
            for (int v : buckets[f]) {
                if (idx == k) break;
                res[idx++] = v;
            }
        }
        return res;
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
```java
public class Codec {
    // Encodes a list of strings to a single string.
    public String encode(List<String> strs) {
        StringBuilder sb = new StringBuilder();
        for (String s : strs) sb.append(s.length()).append('#').append(s);
        return sb.toString();
    }

    // Decodes a single string to a list of strings.
    public List<String> decode(String s) {
        List<String> res = new ArrayList<>();
        int i = 0;
        while (i < s.length()) {
            int j = s.indexOf('#', i);
            int len = Integer.parseInt(s.substring(i, j));
            res.add(s.substring(j + 1, j + 1 + len));
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
```java
class Solution {
    public int[] productExceptSelf(int[] nums) {
        int n = nums.length;
        int[] prefix = new int[n], suffix = new int[n], res = new int[n];
        prefix[0] = 1;
        suffix[n - 1] = 1;
        for (int i = 1; i < n; i++) prefix[i] = prefix[i - 1] * nums[i - 1];
        for (int i = n - 2; i >= 0; i--) suffix[i] = suffix[i + 1] * nums[i + 1];
        for (int i = 0; i < n; i++) res[i] = prefix[i] * suffix[i];
        return res;
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: In-Place Prefix and Running Suffix (Optimal)
```java
class Solution {
    public int[] productExceptSelf(int[] nums) {
        int n = nums.length;
        int[] res = new int[n];
        res[0] = 1;
        for (int i = 1; i < n; i++) res[i] = res[i - 1] * nums[i - 1];
        int suffix = 1;
        for (int i = n - 1; i >= 0; i--) {
            res[i] *= suffix;
            suffix *= nums[i];
        }
        return res;
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
```java
class Solution {
    public boolean isValidSudoku(char[][] board) {
        boolean[][] rows = new boolean[9][9], cols = new boolean[9][9], boxes = new boolean[9][9];
        for (int r = 0; r < 9; r++) {
            for (int c = 0; c < 9; c++) {
                if (board[r][c] == '.') continue;
                int d = board[r][c] - '1';
                int b = (r / 3) * 3 + c / 3;
                if (rows[r][d] || cols[c][d] || boxes[b][d]) return false;
                rows[r][d] = cols[c][d] = boxes[b][d] = true;
            }
        }
        return true;
    }
}
```
**Complexity:** Time O(81) = O(1) · Space O(1)

#### Approach 2: Bitmasks
```java
class Solution {
    public boolean isValidSudoku(char[][] board) {
        int[] rows = new int[9], cols = new int[9], boxes = new int[9];
        for (int r = 0; r < 9; r++) {
            for (int c = 0; c < 9; c++) {
                if (board[r][c] == '.') continue;
                int bit = 1 << (board[r][c] - '1');
                int b = (r / 3) * 3 + c / 3;
                if ((rows[r] & bit) != 0 || (cols[c] & bit) != 0 || (boxes[b] & bit) != 0) return false;
                rows[r] |= bit;
                cols[c] |= bit;
                boxes[b] |= bit;
            }
        }
        return true;
    }
}
```
**Complexity:** Time O(1) · Space O(1)

#### Approach 3: Single HashSet of Encoded Strings
```java
class Solution {
    public boolean isValidSudoku(char[][] board) {
        Set<String> seen = new HashSet<>();
        for (int r = 0; r < 9; r++) {
            for (int c = 0; c < 9; c++) {
                char d = board[r][c];
                if (d == '.') continue;
                if (!seen.add(d + " in row " + r) || !seen.add(d + " in col " + c)
                        || !seen.add(d + " in box " + r / 3 + "-" + c / 3)) return false;
            }
        }
        return true;
    }
}
```
**Complexity:** Time O(1) · Space O(1). Readable, but slower because of string building.

---

### 128. Longest Consecutive Sequence
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-consecutive-sequence/)

**Problem:** Given an unsorted integer array `nums`, return the length of the longest run of consecutive integers. The algorithm must run in O(n).

**Example:** `[100,4,200,1,3,2]` → `4` (the run is `1,2,3,4`)

**Key insight:** Only start counting at a number `x` when `x-1` isn't in the set. That way each run is walked once.

#### Approach 1: Sorting
```java
class Solution {
    public int longestConsecutive(int[] nums) {
        if (nums.length == 0) return 0;
        Arrays.sort(nums);
        int best = 1, cur = 1;
        for (int i = 1; i < nums.length; i++) {
            if (nums[i] == nums[i - 1]) continue;
            cur = (nums[i] == nums[i - 1] + 1) ? cur + 1 : 1;
            best = Math.max(best, cur);
        }
        return best;
    }
}
```
**Complexity:** Time O(n log n) · Space O(log n)

#### Approach 2: HashSet (Optimal)
```java
class Solution {
    public int longestConsecutive(int[] nums) {
        Set<Integer> set = new HashSet<>();
        for (int n : nums) set.add(n);
        int best = 0;
        for (int n : set) {
            if (set.contains(n - 1)) continue; // not the start of a run
            int len = 1;
            while (set.contains(n + len)) len++;
            best = Math.max(best, len);
        }
        return best;
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
```java
class Solution {
    public boolean isPalindrome(String s) {
        StringBuilder sb = new StringBuilder();
        for (char c : s.toCharArray())
            if (Character.isLetterOrDigit(c)) sb.append(Character.toLowerCase(c));
        String clean = sb.toString();
        return clean.equals(sb.reverse().toString());
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Two Pointers In Place (Optimal)
```java
class Solution {
    public boolean isPalindrome(String s) {
        int l = 0, r = s.length() - 1;
        while (l < r) {
            while (l < r && !Character.isLetterOrDigit(s.charAt(l))) l++;
            while (l < r && !Character.isLetterOrDigit(s.charAt(r))) r--;
            if (Character.toLowerCase(s.charAt(l)) != Character.toLowerCase(s.charAt(r))) return false;
            l++;
            r--;
        }
        return true;
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
```java
class Solution {
    public int[] twoSum(int[] numbers, int target) {
        for (int i = 0; i < numbers.length; i++) {
            int want = target - numbers[i];
            int lo = i + 1, hi = numbers.length - 1;
            while (lo <= hi) {
                int mid = lo + (hi - lo) / 2;
                if (numbers[mid] == want) return new int[]{i + 1, mid + 1};
                if (numbers[mid] < want) lo = mid + 1;
                else hi = mid - 1;
            }
        }
        return new int[0];
    }
}
```
**Complexity:** Time O(n log n) · Space O(1)

#### Approach 2: Two Pointers (Optimal)
```java
class Solution {
    public int[] twoSum(int[] numbers, int target) {
        int l = 0, r = numbers.length - 1;
        while (l < r) {
            int sum = numbers[l] + numbers[r];
            if (sum == target) return new int[]{l + 1, r + 1};
            if (sum < target) l++;
            else r--;
        }
        return new int[0];
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
```java
class Solution {
    public List<List<Integer>> threeSum(int[] nums) {
        Arrays.sort(nums);
        List<List<Integer>> res = new ArrayList<>();
        for (int i = 0; i < nums.length; i++) {
            if (i > 0 && nums[i] == nums[i - 1]) continue;
            Set<Integer> seen = new HashSet<>();
            for (int j = i + 1; j < nums.length; j++) {
                int need = -nums[i] - nums[j];
                if (seen.contains(need)) {
                    res.add(Arrays.asList(nums[i], need, nums[j]));
                    while (j + 1 < nums.length && nums[j] == nums[j + 1]) j++;
                }
                seen.add(nums[j]);
            }
        }
        return res;
    }
}
```
**Complexity:** Time O(n²) · Space O(n)

#### Approach 2: Sort + Two Pointers (Optimal)
```java
class Solution {
    public List<List<Integer>> threeSum(int[] nums) {
        Arrays.sort(nums);
        List<List<Integer>> res = new ArrayList<>();
        for (int i = 0; i < nums.length - 2; i++) {
            if (nums[i] > 0) break; // the smallest value is positive, so no triplet can sum to zero
            if (i > 0 && nums[i] == nums[i - 1]) continue;
            int l = i + 1, r = nums.length - 1;
            while (l < r) {
                int sum = nums[i] + nums[l] + nums[r];
                if (sum < 0) l++;
                else if (sum > 0) r--;
                else {
                    res.add(Arrays.asList(nums[i], nums[l], nums[r]));
                    l++;
                    r--;
                    while (l < r && nums[l] == nums[l - 1]) l++;
                }
            }
        }
        return res;
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
```java
class Solution {
    public int maxArea(int[] height) {
        int best = 0;
        for (int i = 0; i < height.length; i++)
            for (int j = i + 1; j < height.length; j++)
                best = Math.max(best, (j - i) * Math.min(height[i], height[j]));
        return best;
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Two Pointers (Optimal)
```java
class Solution {
    public int maxArea(int[] height) {
        int l = 0, r = height.length - 1, best = 0;
        while (l < r) {
            best = Math.max(best, (r - l) * Math.min(height[l], height[r]));
            if (height[l] < height[r]) l++;
            else r--;
        }
        return best;
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
```java
class Solution {
    public int trap(int[] height) {
        int n = height.length;
        if (n == 0) return 0;
        int[] left = new int[n], right = new int[n];
        left[0] = height[0];
        for (int i = 1; i < n; i++) left[i] = Math.max(left[i - 1], height[i]);
        right[n - 1] = height[n - 1];
        for (int i = n - 2; i >= 0; i--) right[i] = Math.max(right[i + 1], height[i]);
        int water = 0;
        for (int i = 0; i < n; i++) water += Math.min(left[i], right[i]) - height[i];
        return water;
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Monotonic Stack
```java
class Solution {
    public int trap(int[] height) {
        Deque<Integer> stack = new ArrayDeque<>(); // indices with decreasing heights
        int water = 0;
        for (int i = 0; i < height.length; i++) {
            while (!stack.isEmpty() && height[i] > height[stack.peek()]) {
                int bottom = stack.pop();
                if (stack.isEmpty()) break;
                int left = stack.peek();
                int width = i - left - 1;
                int bounded = Math.min(height[left], height[i]) - height[bottom];
                water += width * bounded;
            }
            stack.push(i);
        }
        return water;
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 3: Two Pointers (Optimal)
```java
class Solution {
    public int trap(int[] height) {
        int l = 0, r = height.length - 1, leftMax = 0, rightMax = 0, water = 0;
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
```java
class Solution {
    public int maxProfit(int[] prices) {
        int best = 0;
        for (int i = 0; i < prices.length; i++)
            for (int j = i + 1; j < prices.length; j++)
                best = Math.max(best, prices[j] - prices[i]);
        return best;
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: One Pass with Running Min (Optimal)
```java
class Solution {
    public int maxProfit(int[] prices) {
        int minPrice = Integer.MAX_VALUE, best = 0;
        for (int p : prices) {
            minPrice = Math.min(minPrice, p);
            best = Math.max(best, p - minPrice);
        }
        return best;
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
```java
class Solution {
    public int lengthOfLongestSubstring(String s) {
        Set<Character> window = new HashSet<>();
        int l = 0, best = 0;
        for (int r = 0; r < s.length(); r++) {
            while (window.contains(s.charAt(r))) window.remove(s.charAt(l++));
            window.add(s.charAt(r));
            best = Math.max(best, r - l + 1);
        }
        return best;
    }
}
```
**Complexity:** Time O(n) (each character enters and leaves once) · Space O(min(n, alphabet))

#### Approach 2: Last-Seen Index Jump (Optimal)
```java
class Solution {
    public int lengthOfLongestSubstring(String s) {
        int[] last = new int[128]; // stores index+1; 0 means not seen
        int l = 0, best = 0;
        for (int r = 0; r < s.length(); r++) {
            char c = s.charAt(r);
            l = Math.max(l, last[c]);
            last[c] = r + 1;
            best = Math.max(best, r - l + 1);
        }
        return best;
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
```java
class Solution {
    public int characterReplacement(String s, int k) {
        int[] count = new int[26];
        int l = 0, best = 0;
        for (int r = 0; r < s.length(); r++) {
            count[s.charAt(r) - 'A']++;
            while (r - l + 1 - Arrays.stream(count).max().getAsInt() > k) count[s.charAt(l++) - 'A']--;
            best = Math.max(best, r - l + 1);
        }
        return best;
    }
}
```
**Complexity:** Time O(26 · n) · Space O(1)

#### Approach 2: Running Max Frequency (Optimal)
```java
class Solution {
    public int characterReplacement(String s, int k) {
        int[] count = new int[26];
        int l = 0, maxFreq = 0, best = 0;
        for (int r = 0; r < s.length(); r++) {
            maxFreq = Math.max(maxFreq, ++count[s.charAt(r) - 'A']);
            if (r - l + 1 - maxFreq > k) count[s.charAt(l++) - 'A']--; // shift the window, don't shrink it
            best = Math.max(best, r - l + 1);
        }
        return best;
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
```java
class Solution {
    public boolean checkInclusion(String s1, String s2) {
        if (s1.length() > s2.length()) return false;
        int[] need = new int[26], win = new int[26];
        for (int i = 0; i < s1.length(); i++) {
            need[s1.charAt(i) - 'a']++;
            win[s2.charAt(i) - 'a']++;
        }
        if (Arrays.equals(need, win)) return true;
        for (int r = s1.length(); r < s2.length(); r++) {
            win[s2.charAt(r) - 'a']++;
            win[s2.charAt(r - s1.length()) - 'a']--;
            if (Arrays.equals(need, win)) return true;
        }
        return false;
    }
}
```
**Complexity:** Time O(26 · n) · Space O(1)

#### Approach 2: Track the Number of Matching Letters (Optimal)
```java
class Solution {
    public boolean checkInclusion(String s1, String s2) {
        if (s1.length() > s2.length()) return false;
        int[] c1 = new int[26], c2 = new int[26];
        for (int i = 0; i < s1.length(); i++) {
            c1[s1.charAt(i) - 'a']++;
            c2[s2.charAt(i) - 'a']++;
        }
        int matches = 0;
        for (int i = 0; i < 26; i++) if (c1[i] == c2[i]) matches++;
        for (int r = s1.length(); r < s2.length(); r++) {
            if (matches == 26) return true;
            int in = s2.charAt(r) - 'a';
            if (c1[in] == c2[in]) matches--;
            c2[in]++;
            if (c1[in] == c2[in]) matches++;
            int out = s2.charAt(r - s1.length()) - 'a';
            if (c1[out] == c2[out]) matches--;
            c2[out]--;
            if (c1[out] == c2[out]) matches++;
        }
        return matches == 26;
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
```java
class Solution {
    public String minWindow(String s, String t) {
        if (t.isEmpty() || t.length() > s.length()) return "";
        int[] need = new int[128], win = new int[128];
        int required = 0;
        for (char c : t.toCharArray()) if (need[c]++ == 0) required++;
        int have = 0, l = 0, bestL = 0, bestLen = Integer.MAX_VALUE;
        for (int r = 0; r < s.length(); r++) {
            char c = s.charAt(r);
            if (++win[c] == need[c]) have++;
            while (have == required) {
                if (r - l + 1 < bestLen) {
                    bestL = l;
                    bestLen = r - l + 1;
                }
                char lc = s.charAt(l++);
                if (win[lc]-- == need[lc]) have--;
            }
        }
        return bestLen == Integer.MAX_VALUE ? "" : s.substring(bestL, bestL + bestLen);
    }
}
```
**Complexity:** Time O(|s| + |t|) · Space O(1) (128 ASCII slots)

> `++win[c] == need[c]` only bumps `have` when a needed character reaches its exact required count. Characters not in `t` have `need = 0`, so after the increment they are ≥ 1 and never match.

**Alternative (brute force):** Check every substring of `s` against `t`'s counts: O(n² · 128) time. Too slow for the constraints, but useful as a correctness baseline.

---

### 239. Sliding Window Maximum
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/sliding-window-maximum/)

**Problem:** Given an array `nums` and a window size `k`, slide the window from left to right one step at a time and return the maximum of each window.

**Example:** `nums = [1,3,-1,-3,5,3,6,7], k = 3` → `[3,3,5,5,6,7]`

**Key insight:** Keep a deque of indices whose values are decreasing. The front is always the window's maximum.

#### Approach 1: Max-Heap with Lazy Deletion
```java
class Solution {
    public int[] maxSlidingWindow(int[] nums, int k) {
        // heap holds {value, index}; largest value first
        PriorityQueue<int[]> heap = new PriorityQueue<>((a, b) -> Integer.compare(b[0], a[0]));
        int[] res = new int[nums.length - k + 1];
        for (int i = 0; i < nums.length; i++) {
            heap.offer(new int[]{nums[i], i});
            if (i >= k - 1) {
                while (heap.peek()[1] <= i - k) heap.poll(); // top has left the window
                res[i - k + 1] = heap.peek()[0];
            }
        }
        return res;
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Monotonic Deque (Optimal)
```java
class Solution {
    public int[] maxSlidingWindow(int[] nums, int k) {
        Deque<Integer> dq = new ArrayDeque<>(); // indices; their nums values are decreasing
        int[] res = new int[nums.length - k + 1];
        for (int i = 0; i < nums.length; i++) {
            if (!dq.isEmpty() && dq.peekFirst() <= i - k) dq.pollFirst();
            while (!dq.isEmpty() && nums[dq.peekLast()] < nums[i]) dq.pollLast();
            dq.offerLast(i);
            if (i >= k - 1) res[i - k + 1] = nums[dq.peekFirst()];
        }
        return res;
    }
}
```
**Complexity:** Time O(n) (each index is pushed and popped at most once) · Space O(k)

---
## 4. Stack

> Use `ArrayDeque` as the stack: `push`, `pop`, and `peek` all work on its head.

### 20. Valid Parentheses
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/valid-parentheses/)

**Problem:** Given a string made only of `()[]{}`, decide whether it is valid. Every opening bracket must be closed by the same type of bracket, in the correct order.

**Example:** `"()[]{}"` → `true`; `"(]"` → `false`; `"([)]"` → `false`

**Key insight:** The most recent unmatched opening bracket must be the first one closed, which is last-in, first-out.

#### Approach 1: Stack (Optimal)
```java
class Solution {
    public boolean isValid(String s) {
        Deque<Character> stack = new ArrayDeque<>();
        for (char c : s.toCharArray()) {
            if (c == '(') stack.push(')');      // push the expected closer
            else if (c == '[') stack.push(']');
            else if (c == '{') stack.push('}');
            else if (stack.isEmpty() || stack.pop() != c) return false;
        }
        return stack.isEmpty();
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Repeated Replacement (simple but slow)
```java
class Solution {
    public boolean isValid(String s) {
        String prev;
        do {
            prev = s;
            s = s.replace("()", "").replace("[]", "").replace("{}", "");
        } while (!s.equals(prev));
        return s.isEmpty();
    }
}
```
**Complexity:** Time O(n²) · Space O(n)

---

### 155. Min Stack
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/min-stack/)

**Problem:** Design a stack that supports `push`, `pop`, `top`, and `getMin` (return the smallest element) in O(1) time each.

**Key insight:** Store the minimum at each depth alongside the value, so popping restores the previous minimum automatically.

#### Approach 1: Pairs of (value, current min)
```java
class MinStack {
    private final Deque<int[]> stack = new ArrayDeque<>(); // {value, min at this depth}

    public MinStack() {}

    public void push(int val) {
        int m = stack.isEmpty() ? val : Math.min(val, stack.peek()[1]);
        stack.push(new int[]{val, m});
    }

    public void pop() { stack.pop(); }

    public int top() { return stack.peek()[0]; }

    public int getMin() { return stack.peek()[1]; }
}
```
**Complexity:** O(1) per operation · Space O(n)

#### Approach 2: Two Stacks (min stack only grows on a new min)
```java
class MinStack2 {
    private final Deque<Integer> stack = new ArrayDeque<>();
    private final Deque<Integer> mins = new ArrayDeque<>();

    public void push(int val) {
        stack.push(val);
        if (mins.isEmpty() || val <= mins.peek()) mins.push(val);
    }

    public void pop() {
        int top = stack.pop();
        if (top == mins.peek()) mins.pop(); // unboxed int comparison, safe
    }

    public int top() { return stack.peek(); }

    public int getMin() { return mins.peek(); }
}
```
**Complexity:** O(1) per operation · Space O(n), usually less in practice

#### Approach 3: Single Stack Storing Differences (O(1) extra space)
```java
class MinStack3 {
    private final Deque<Long> stack = new ArrayDeque<>(); // stores value - min
    private long min;

    public void push(int val) {
        if (stack.isEmpty()) {
            stack.push(0L);
            min = val;
        } else {
            stack.push(val - min);
            if (val < min) min = val;
        }
    }

    public void pop() {
        long diff = stack.pop();
        if (diff < 0) min = min - diff; // the popped value was the min, so restore the previous one
    }

    public int top() {
        long diff = stack.peek();
        return (int) (diff < 0 ? min : min + diff);
    }

    public int getMin() { return (int) min; }
}
```
**Complexity:** O(1) per operation · Space O(n) for values, no second stack. `long` prevents overflow in `val - min`.

---

### 150. Evaluate Reverse Polish Notation
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/evaluate-reverse-polish-notation/)

**Problem:** Evaluate an arithmetic expression written in Reverse Polish (postfix) notation. The operators are `+ - * /`, and division truncates toward zero.

**Example:** `["2","1","+","3","*"]` → `9`; `["4","13","5","/","+"]` → `6`

**Key insight:** Push operands. On an operator, pop two values (second operand first) and push the result.

#### Approach 1: Stack (Optimal)
```java
class Solution {
    public int evalRPN(String[] tokens) {
        Deque<Integer> stack = new ArrayDeque<>();
        for (String tok : tokens) {
            switch (tok) {
                case "+": stack.push(stack.pop() + stack.pop()); break;
                case "-": { int b = stack.pop(), a = stack.pop(); stack.push(a - b); break; }
                case "*": stack.push(stack.pop() * stack.pop()); break;
                case "/": { int b = stack.pop(), a = stack.pop(); stack.push(a / b); break; } // truncates toward zero
                default: stack.push(Integer.parseInt(tok));
            }
        }
        return stack.pop();
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Array as Stack (faster, no boxing)
```java
class Solution {
    public int evalRPN(String[] tokens) {
        int[] st = new int[tokens.length];
        int top = -1;
        for (String tok : tokens) {
            if (tok.length() == 1 && "+-*/".indexOf(tok.charAt(0)) >= 0) {
                int b = st[top--], a = st[top--];
                switch (tok.charAt(0)) {
                    case '+': st[++top] = a + b; break;
                    case '-': st[++top] = a - b; break;
                    case '*': st[++top] = a * b; break;
                    default:  st[++top] = a / b;
                }
            } else {
                st[++top] = Integer.parseInt(tok);
            }
        }
        return st[top];
    }
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 22. Generate Parentheses
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/generate-parentheses/)

**Problem:** Given `n`, generate every combination of `n` pairs of well-formed parentheses.

**Example:** `n = 3` → `["((()))","(()())","(())()","()(())","()()()"]`

**Key insight:** You can add `(` while `open < n`, and `)` while `close < open`.

#### Approach 1: Backtracking (Optimal)
```java
class Solution {
    public List<String> generateParenthesis(int n) {
        List<String> res = new ArrayList<>();
        backtrack(res, new StringBuilder(), 0, 0, n);
        return res;
    }

    private void backtrack(List<String> res, StringBuilder sb, int open, int close, int n) {
        if (sb.length() == 2 * n) {
            res.add(sb.toString());
            return;
        }
        if (open < n) {
            sb.append('(');
            backtrack(res, sb, open + 1, close, n);
            sb.deleteCharAt(sb.length() - 1);
        }
        if (close < open) {
            sb.append(')');
            backtrack(res, sb, open, close + 1, n);
            sb.deleteCharAt(sb.length() - 1);
        }
    }
}
```
**Complexity:** Time O(4ⁿ / √n), the nth Catalan number of results × length · Space O(n) recursion

#### Approach 2: Dynamic Programming (closure number)
Every valid string has the form `"(" + a + ")" + b`, where `a` has `i` pairs and `b` has `n-1-i` pairs.
```java
class Solution {
    public List<String> generateParenthesis(int n) {
        List<List<String>> dp = new ArrayList<>();
        dp.add(Collections.singletonList(""));
        for (int k = 1; k <= n; k++) {
            List<String> cur = new ArrayList<>();
            for (int i = 0; i < k; i++)
                for (String a : dp.get(i))
                    for (String b : dp.get(k - 1 - i))
                        cur.add("(" + a + ")" + b);
            dp.add(cur);
        }
        return dp.get(n);
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
```java
class Solution {
    public int[] dailyTemperatures(int[] temperatures) {
        int n = temperatures.length;
        int[] res = new int[n];
        Deque<Integer> stack = new ArrayDeque<>(); // indices of unresolved days
        for (int i = 0; i < n; i++) {
            while (!stack.isEmpty() && temperatures[stack.peek()] < temperatures[i]) {
                int j = stack.pop();
                res[j] = i - j;
            }
            stack.push(i);
        }
        return res;
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Backward Jumps (O(1) extra space)
Scan from the right and reuse `res` to hop forward to the next warmer candidate.
```java
class Solution {
    public int[] dailyTemperatures(int[] temperatures) {
        int n = temperatures.length, hottest = 0;
        int[] res = new int[n];
        for (int i = n - 1; i >= 0; i--) {
            int t = temperatures[i];
            if (t >= hottest) {
                hottest = t;
                continue;
            }
            int days = 1;
            while (temperatures[i + days] <= t) days += res[i + days];
            res[i] = days;
        }
        return res;
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
```java
class Solution {
    public int carFleet(int target, int[] position, int[] speed) {
        int n = position.length;
        Integer[] idx = new Integer[n];
        for (int i = 0; i < n; i++) idx[i] = i;
        Arrays.sort(idx, (a, b) -> position[b] - position[a]); // closest to the target first
        Deque<Double> stack = new ArrayDeque<>();
        for (int i : idx) {
            double t = (double) (target - position[i]) / speed[i];
            if (stack.isEmpty() || t > stack.peek()) stack.push(t); // too slow to catch the fleet ahead, so a new fleet
        }
        return stack.size();
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Bucket by Position (positions < target ≤ 10⁶)
```java
class Solution {
    public int carFleet(int target, int[] position, int[] speed) {
        double[] time = new double[target]; // time[p] = arrival time of the car at p (0 = none)
        for (int i = 0; i < position.length; i++)
            time[position[i]] = (double) (target - position[i]) / speed[i];
        int fleets = 0;
        double slowest = 0;
        for (int p = target - 1; p >= 0; p--) {
            if (time[p] > slowest) {
                fleets++;
                slowest = time[p];
            }
        }
        return fleets;
    }
}
```
**Complexity:** Time O(n + target) · Space O(target)

---

### 84. Largest Rectangle in Histogram
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/largest-rectangle-in-histogram/)

**Problem:** Given bar heights `heights`, each bar having width 1, return the area of the largest rectangle that fits in the histogram.

**Example:** `[2,1,5,6,2,3]` → `10` (bars 5 and 6, width 2)

**Key insight:** When a shorter bar arrives, each taller bar popped from an increasing stack has found its right boundary. Its left boundary is the new stack top.

#### Approach 1: Brute Force (expand each bar)
```java
class Solution {
    public int largestRectangleArea(int[] heights) {
        int best = 0;
        for (int i = 0; i < heights.length; i++) {
            int minH = heights[i];
            for (int j = i; j < heights.length; j++) {
                minH = Math.min(minH, heights[j]);
                best = Math.max(best, minH * (j - i + 1));
            }
        }
        return best;
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Monotonic Increasing Stack (Optimal)
```java
class Solution {
    public int largestRectangleArea(int[] heights) {
        Deque<Integer> stack = new ArrayDeque<>(); // indices of increasing heights
        int best = 0, n = heights.length;
        for (int i = 0; i <= n; i++) {
            int h = (i == n) ? 0 : heights[i]; // sentinel height 0 at i == n flushes the stack
            while (!stack.isEmpty() && heights[stack.peek()] >= h) {
                int top = stack.pop();
                int left = stack.isEmpty() ? -1 : stack.peek();
                best = Math.max(best, heights[top] * (i - left - 1));
            }
            stack.push(i);
        }
        return best;
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 3: Precomputed Nearest-Smaller Boundaries
```java
class Solution {
    public int largestRectangleArea(int[] heights) {
        int n = heights.length;
        int[] left = new int[n], right = new int[n];
        for (int i = 0; i < n; i++) {
            int p = i - 1;
            while (p >= 0 && heights[p] >= heights[i]) p = left[p]; // jump along the boundary chain
            left[i] = p;
        }
        for (int i = n - 1; i >= 0; i--) {
            int p = i + 1;
            while (p < n && heights[p] >= heights[i]) p = right[p];
            right[i] = p;
        }
        int best = 0;
        for (int i = 0; i < n; i++) best = Math.max(best, heights[i] * (right[i] - left[i] - 1));
        return best;
    }
}
```
**Complexity:** Time O(n) amortized · Space O(n)

---
## 5. Binary Search

### 704. Binary Search
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/binary-search/)

**Problem:** Given a sorted array `nums` of distinct integers and a `target`, return the target's index, or `-1` if it's not present. Run in O(log n).

**Example:** `nums = [-1,0,3,5,9,12], target = 9` → `4`

**Key insight:** Halve the search space each step. Use `int mid = lo + (hi - lo) / 2` so the midpoint can't overflow.

#### Approach 1: Iterative (Optimal)
```java
class Solution {
    public int search(int[] nums, int target) {
        int lo = 0, hi = nums.length - 1;
        while (lo <= hi) {
            int mid = lo + (hi - lo) / 2;
            if (nums[mid] == target) return mid;
            if (nums[mid] < target) lo = mid + 1;
            else hi = mid - 1;
        }
        return -1;
    }
}
```
**Complexity:** Time O(log n) · Space O(1)

#### Approach 2: Recursive
```java
class Solution {
    public int search(int[] nums, int target) {
        return rec(nums, target, 0, nums.length - 1);
    }

    private int rec(int[] nums, int target, int lo, int hi) {
        if (lo > hi) return -1;
        int mid = lo + (hi - lo) / 2;
        if (nums[mid] == target) return mid;
        return nums[mid] < target ? rec(nums, target, mid + 1, hi) : rec(nums, target, lo, mid - 1);
    }
}
```
**Complexity:** Time O(log n) · Space O(log n) recursion

#### Approach 3: Standard Library
```java
class Solution {
    public int search(int[] nums, int target) {
        int i = Arrays.binarySearch(nums, target); // negative (-(insertion point) - 1) if absent
        return i >= 0 ? i : -1;
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
```java
class Solution {
    public boolean searchMatrix(int[][] matrix, int target) {
        int m = matrix.length, n = matrix[0].length;
        int lo = 0, hi = m * n - 1;
        while (lo <= hi) {
            int mid = lo + (hi - lo) / 2;
            int v = matrix[mid / n][mid % n];
            if (v == target) return true;
            if (v < target) lo = mid + 1;
            else hi = mid - 1;
        }
        return false;
    }
}
```
**Complexity:** Time O(log(m·n)) · Space O(1)

#### Approach 2: Two Binary Searches (row, then column)
```java
class Solution {
    public boolean searchMatrix(int[][] matrix, int target) {
        int top = 0, bot = matrix.length - 1;
        while (top <= bot) {
            int mid = (top + bot) / 2;
            int[] row = matrix[mid];
            if (target < row[0]) bot = mid - 1;
            else if (target > row[row.length - 1]) top = mid + 1;
            else return Arrays.binarySearch(row, target) >= 0;
        }
        return false;
    }
}
```
**Complexity:** Time O(log m + log n) · Space O(1)

#### Approach 3: Staircase from the Top-Right (also works for LC 240)
```java
class Solution {
    public boolean searchMatrix(int[][] matrix, int target) {
        int r = 0, c = matrix[0].length - 1;
        while (r < matrix.length && c >= 0) {
            if (matrix[r][c] == target) return true;
            if (matrix[r][c] > target) c--;
            else r++;
        }
        return false;
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
```java
class Solution {
    public int minEatingSpeed(int[] piles, int h) {
        int lo = 1, hi = 0;
        for (int p : piles) hi = Math.max(hi, p);
        while (lo < hi) {
            int k = lo + (hi - lo) / 2;
            long hours = 0; // long: up to 10^4 piles × 10^9 bananas overflows int
            for (int p : piles) hours += (p + k - 1) / k; // ceil(p / k)
            if (hours <= h) hi = k; // k works; try smaller
            else lo = k + 1;
        }
        return lo;
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
```java
class Solution {
    public int findMin(int[] nums) {
        int lo = 0, hi = nums.length - 1;
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (nums[mid] > nums[hi]) lo = mid + 1;
            else hi = mid;
        }
        return nums[lo];
    }
}
```
**Complexity:** Time O(log n) · Space O(1)

**Linear alternative:** `Arrays.stream(nums).min().getAsInt()` is O(n). Fine for a sanity check, but it doesn't meet the requirement.

---

### 33. Search in Rotated Sorted Array
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/search-in-rotated-sorted-array/)

**Problem:** Search for `target` in a rotated sorted array of distinct values. Return its index or `-1`, in O(log n).

**Example:** `nums = [4,5,6,7,0,1,2], target = 0` → `4`

**Key insight:** At every `mid`, at least one half is sorted. Check whether `target` falls inside the sorted half's range.

#### Approach 1: One-Pass Binary Search (Optimal)
```java
class Solution {
    public int search(int[] nums, int target) {
        int lo = 0, hi = nums.length - 1;
        while (lo <= hi) {
            int mid = lo + (hi - lo) / 2;
            if (nums[mid] == target) return mid;
            if (nums[lo] <= nums[mid]) { // left half sorted
                if (nums[lo] <= target && target < nums[mid]) hi = mid - 1;
                else lo = mid + 1;
            } else {                     // right half sorted
                if (nums[mid] < target && target <= nums[hi]) lo = mid + 1;
                else hi = mid - 1;
            }
        }
        return -1;
    }
}
```
**Complexity:** Time O(log n) · Space O(1)

#### Approach 2: Find the Pivot, Then Binary Search the Correct Half
```java
class Solution {
    public int search(int[] nums, int target) {
        int n = nums.length, lo = 0, hi = n - 1;
        while (lo < hi) { // find the index of the minimum
            int mid = (lo + hi) / 2;
            if (nums[mid] > nums[hi]) lo = mid + 1;
            else hi = mid;
        }
        int pivot = lo;
        if (pivot > 0 && target >= nums[0]) return bs(nums, 0, pivot - 1, target);
        return bs(nums, pivot, n - 1, target);
    }

    private int bs(int[] a, int l, int r, int t) {
        int i = Arrays.binarySearch(a, l, r + 1, t);
        return i >= 0 ? i : -1;
    }
}
```
**Complexity:** Time O(log n) · Space O(1)

---

### 981. Time Based Key-Value Store
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/time-based-key-value-store/)

**Problem:** Design `TimeMap` with `set(key, value, timestamp)` and `get(key, timestamp)`. `get` returns the value stored at the largest timestamp ≤ `timestamp`, or `""` if none. Calls to `set` arrive with strictly increasing timestamps.

**Key insight:** Each key's entries are already sorted by time, so `get` is an upper-bound binary search.

#### Approach 1: Map of Lists + Manual Binary Search (Optimal)
```java
class TimeMap {
    private final Map<String, List<Integer>> times = new HashMap<>();
    private final Map<String, List<String>> values = new HashMap<>();

    public TimeMap() {}

    public void set(String key, String value, int timestamp) {
        times.computeIfAbsent(key, k -> new ArrayList<>()).add(timestamp);
        values.computeIfAbsent(key, k -> new ArrayList<>()).add(value);
    }

    public String get(String key, int timestamp) {
        List<Integer> ts = times.get(key);
        if (ts == null) return "";
        int lo = 0, hi = ts.size() - 1, ans = -1;
        while (lo <= hi) { // last index with ts <= timestamp
            int mid = (lo + hi) >>> 1;
            if (ts.get(mid) <= timestamp) {
                ans = mid;
                lo = mid + 1;
            } else hi = mid - 1;
        }
        return ans == -1 ? "" : values.get(key).get(ans);
    }
}
```
**Complexity:** `set` O(1) amortized · `get` O(log n) · Space O(total entries)

#### Approach 2: TreeMap per Key (concise)
```java
class TimeMap2 {
    private final Map<String, TreeMap<Integer, String>> store = new HashMap<>();

    public void set(String key, String value, int timestamp) {
        store.computeIfAbsent(key, k -> new TreeMap<>()).put(timestamp, value);
    }

    public String get(String key, int timestamp) {
        TreeMap<Integer, String> tm = store.get(key);
        if (tm == null) return "";
        Map.Entry<Integer, String> e = tm.floorEntry(timestamp);
        return e == null ? "" : e.getValue();
    }
}
```
**Complexity:** `set` O(log n) · `get` O(log n) · Space O(total entries). The TreeMap version also works when timestamps aren't increasing.

---

### 4. Median of Two Sorted Arrays
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/median-of-two-sorted-arrays/)

**Problem:** Given sorted arrays `nums1` (size m) and `nums2` (size n), return the median of their combined elements in O(log(m+n)).

**Example:** `[1,3]`, `[2]` → `2.0`; `[1,2]`, `[3,4]` → `2.5`

**Key insight:** Binary search a cut in the shorter array so the left halves of both arrays together hold half the elements, and every element on the left is ≤ every element on the right.

#### Approach 1: Merge (simple)
```java
class Solution {
    public double findMedianSortedArrays(int[] nums1, int[] nums2) {
        int m = nums1.length, n = nums2.length;
        int[] merged = new int[m + n];
        int i = 0, j = 0, k = 0;
        while (i < m && j < n) merged[k++] = nums1[i] <= nums2[j] ? nums1[i++] : nums2[j++];
        while (i < m) merged[k++] = nums1[i++];
        while (j < n) merged[k++] = nums2[j++];
        int t = m + n;
        return t % 2 == 1 ? merged[t / 2] : (merged[t / 2 - 1] + merged[t / 2]) / 2.0;
    }
}
```
**Complexity:** Time O(m + n) · Space O(m + n)

#### Approach 2: Binary Search Partition (Optimal)
```java
class Solution {
    public double findMedianSortedArrays(int[] nums1, int[] nums2) {
        if (nums1.length > nums2.length) return findMedianSortedArrays(nums2, nums1); // search the shorter array
        int m = nums1.length, n = nums2.length, half = (m + n + 1) / 2;
        int lo = 0, hi = m;
        while (lo <= hi) {
            int i = (lo + hi) / 2; // elements taken from nums1
            int j = half - i;      // elements taken from nums2
            int aLeft  = i > 0 ? nums1[i - 1] : Integer.MIN_VALUE;
            int aRight = i < m ? nums1[i] : Integer.MAX_VALUE;
            int bLeft  = j > 0 ? nums2[j - 1] : Integer.MIN_VALUE;
            int bRight = j < n ? nums2[j] : Integer.MAX_VALUE;
            if (aLeft <= bRight && bLeft <= aRight) {
                if ((m + n) % 2 == 1) return Math.max(aLeft, bLeft);
                return ((long) Math.max(aLeft, bLeft) + Math.min(aRight, bRight)) / 2.0;
            } else if (aLeft > bRight) hi = i - 1;
            else lo = i + 1;
        }
        return 0;
    }
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
```java
class Solution {
    public ListNode reverseList(ListNode head) {
        ListNode prev = null, cur = head;
        while (cur != null) {
            ListNode next = cur.next;
            cur.next = prev;
            prev = cur;
            cur = next;
        }
        return prev;
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Recursive
```java
class Solution {
    public ListNode reverseList(ListNode head) {
        if (head == null || head.next == null) return head;
        ListNode newHead = reverseList(head.next);
        head.next.next = head;
        head.next = null;
        return newHead;
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
```java
class Solution {
    public ListNode mergeTwoLists(ListNode list1, ListNode list2) {
        ListNode dummy = new ListNode(), tail = dummy;
        while (list1 != null && list2 != null) {
            if (list1.val <= list2.val) {
                tail.next = list1;
                list1 = list1.next;
            } else {
                tail.next = list2;
                list2 = list2.next;
            }
            tail = tail.next;
        }
        tail.next = (list1 != null) ? list1 : list2;
        return dummy.next;
    }
}
```
**Complexity:** Time O(n + m) · Space O(1)

#### Approach 2: Recursive
```java
class Solution {
    public ListNode mergeTwoLists(ListNode l1, ListNode l2) {
        if (l1 == null) return l2;
        if (l2 == null) return l1;
        if (l1.val <= l2.val) {
            l1.next = mergeTwoLists(l1.next, l2);
            return l1;
        }
        l2.next = mergeTwoLists(l1, l2.next);
        return l2;
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
```java
class Solution {
    public void reorderList(ListNode head) {
        if (head == null || head.next == null) return;
        // 1. find the middle
        ListNode slow = head, fast = head.next;
        while (fast != null && fast.next != null) {
            slow = slow.next;
            fast = fast.next.next;
        }
        // 2. reverse the second half
        ListNode second = slow.next, prev = null;
        slow.next = null;
        while (second != null) {
            ListNode next = second.next;
            second.next = prev;
            prev = second;
            second = next;
        }
        // 3. interleave
        ListNode first = head;
        second = prev;
        while (second != null) {
            ListNode n1 = first.next, n2 = second.next;
            first.next = second;
            second.next = n1;
            first = n1;
            second = n2;
        }
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: List of Nodes
```java
class Solution {
    public void reorderList(ListNode head) {
        List<ListNode> nodes = new ArrayList<>();
        for (ListNode n = head; n != null; n = n.next) nodes.add(n);
        int i = 0, j = nodes.size() - 1;
        while (i < j) {
            nodes.get(i).next = nodes.get(j);
            i++;
            if (i == j) break;
            nodes.get(j).next = nodes.get(i);
            j--;
        }
        nodes.get(i).next = null;
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

#### Approach 1: Two Pointers, One Pass (Optimal)
```java
class Solution {
    public ListNode removeNthFromEnd(ListNode head, int n) {
        ListNode dummy = new ListNode(0, head), slow = dummy, fast = dummy;
        for (int i = 0; i <= n; i++) fast = fast.next;
        while (fast != null) {
            slow = slow.next;
            fast = fast.next;
        }
        slow.next = slow.next.next;
        return dummy.next;
    }
}
```
**Complexity:** Time O(L) · Space O(1)

#### Approach 2: Two Passes (count the length)
```java
class Solution {
    public ListNode removeNthFromEnd(ListNode head, int n) {
        int len = 0;
        for (ListNode c = head; c != null; c = c.next) len++;
        ListNode dummy = new ListNode(0, head), cur = dummy;
        for (int i = 0; i < len - n; i++) cur = cur.next;
        cur.next = cur.next.next;
        return dummy.next;
    }
}
```
**Complexity:** Time O(L) · Space O(1)

---

### 138. Copy List with Random Pointer
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/copy-list-with-random-pointer/)

**Problem:** Each node has `next` and `random` pointers, and `random` may point to any node or `null`. Return a deep copy of the list.

**Key insight:** Map each original node to its copy. Alternatively, weave each copy right after its original so `orig.Random.Next` is the copied random target.

#### Approach 1: HashMap
```java
class Node {
    int val;
    Node next, random;
    public Node(int val) { this.val = val; }
}

class Solution {
    public Node copyRandomList(Node head) {
        Map<Node, Node> copies = new HashMap<>();
        for (Node n = head; n != null; n = n.next) copies.put(n, new Node(n.val));
        for (Node n = head; n != null; n = n.next) {
            copies.get(n).next = copies.get(n.next);     // get(null) returns null
            copies.get(n).random = copies.get(n.random);
        }
        return copies.get(head);
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Interweaving (O(1) extra space)
```java
class Node {
    int val;
    Node next, random;
    public Node(int val) { this.val = val; }
}

class Solution {
    public Node copyRandomList(Node head) {
        if (head == null) return null;
        // A → A' → B → B' ...
        for (Node n = head; n != null; n = n.next.next) {
            Node cp = new Node(n.val);
            cp.next = n.next;
            n.next = cp;
        }
        // set random pointers on the copies
        for (Node n = head; n != null; n = n.next.next)
            if (n.random != null) n.next.random = n.random.next;
        // unweave the two lists
        Node newHead = head.next;
        for (Node n = head; n != null; n = n.next) {
            Node cp = n.next;
            n.next = cp.next;
            if (cp.next != null) cp.next = cp.next.next;
        }
        return newHead;
    }
}
```
**Complexity:** Time O(n) · Space O(1) extra

> LeetCode provides the `Node` class for this problem. It's repeated above only so each snippet compiles on its own.

---

### 2. Add Two Numbers
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/add-two-numbers/)

**Problem:** Two non-negative integers are stored as linked lists with digits in reverse order, so the head is the ones digit. Return their sum as a list in the same format.

**Example:** `2→4→3` (342) + `5→6→4` (465) → `7→0→8` (807)

**Key insight:** Grade-school addition with a carry. Keep looping while either list or the carry remains.

#### Approach 1: Iterative (Optimal)
```java
class Solution {
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        ListNode dummy = new ListNode(), tail = dummy;
        int carry = 0;
        while (l1 != null || l2 != null || carry != 0) {
            int sum = carry;
            if (l1 != null) { sum += l1.val; l1 = l1.next; }
            if (l2 != null) { sum += l2.val; l2 = l2.next; }
            carry = sum / 10;
            tail.next = new ListNode(sum % 10);
            tail = tail.next;
        }
        return dummy.next;
    }
}
```
**Complexity:** Time O(max(m, n)) · Space O(1) extra (the output doesn't count)

#### Approach 2: Recursive
```java
class Solution {
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        return add(l1, l2, 0);
    }

    private ListNode add(ListNode a, ListNode b, int carry) {
        if (a == null && b == null && carry == 0) return null;
        int sum = carry + (a != null ? a.val : 0) + (b != null ? b.val : 0);
        return new ListNode(sum % 10, add(a != null ? a.next : null, b != null ? b.next : null, sum / 10));
    }
}
```
**Complexity:** Time O(max(m, n)) · Space O(max(m, n)) recursion

---

### 141. Linked List Cycle
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/linked-list-cycle/)

**Problem:** Return `true` if the linked list contains a cycle.

**Key insight:** Floyd's tortoise and hare. If there's a cycle, the fast pointer eventually laps the slow one.

#### Approach 1: HashSet
```java
class Solution {
    public boolean hasCycle(ListNode head) {
        Set<ListNode> seen = new HashSet<>();
        for (ListNode n = head; n != null; n = n.next)
            if (!seen.add(n)) return true;
        return false;
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Floyd's Fast and Slow Pointers (Optimal)
```java
class Solution {
    public boolean hasCycle(ListNode head) {
        ListNode slow = head, fast = head;
        while (fast != null && fast.next != null) {
            slow = slow.next;
            fast = fast.next.next;
            if (slow == fast) return true;
        }
        return false;
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
```java
class Solution {
    public int findDuplicate(int[] nums) {
        Set<Integer> seen = new HashSet<>();
        for (int n : nums) if (!seen.add(n)) return n;
        return -1;
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Binary Search on Value (pigeonhole)
```java
class Solution {
    public int findDuplicate(int[] nums) {
        int lo = 1, hi = nums.length - 1;
        while (lo < hi) {
            int mid = (lo + hi) / 2, cnt = 0;
            for (int n : nums) if (n <= mid) cnt++;
            if (cnt > mid) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }
}
```
**Complexity:** Time O(n log n) · Space O(1)

#### Approach 3: Floyd's Cycle Detection (Optimal)
```java
class Solution {
    public int findDuplicate(int[] nums) {
        int slow = nums[0], fast = nums[nums[0]];
        while (slow != fast) {
            slow = nums[slow];
            fast = nums[nums[fast]];
        }
        slow = 0;
        while (slow != fast) {
            slow = nums[slow];
            fast = nums[fast];
        }
        return slow;
    }
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 146. LRU Cache
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/lru-cache/)

**Problem:** Design an LRU (least recently used) cache with a fixed `capacity`. `get(key)` returns the value or `-1`. `put(key, value)` inserts or updates a key, evicting the least recently used key when over capacity. Both must run in O(1).

**Key insight:** A hash map gives O(1) lookup, and a doubly linked list gives O(1) reordering and eviction.

#### Approach 1: Hand-Written Doubly Linked List (Optimal, interview-preferred)
```java
class LRUCache {
    private static class DNode {
        int key, val;
        DNode prev, next;
        DNode(int k, int v) { key = k; val = v; }
    }

    private final int cap;
    private final Map<Integer, DNode> items = new HashMap<>();
    private final DNode head = new DNode(0, 0), tail = new DNode(0, 0); // head.next is the MRU, tail.prev is the LRU

    public LRUCache(int capacity) {
        cap = capacity;
        head.next = tail;
        tail.prev = head;
    }

    private void remove(DNode n) {
        n.prev.next = n.next;
        n.next.prev = n.prev;
    }

    private void pushFront(DNode n) {
        n.prev = head;
        n.next = head.next;
        head.next.prev = n;
        head.next = n;
    }

    public int get(int key) {
        DNode n = items.get(key);
        if (n == null) return -1;
        remove(n);
        pushFront(n);
        return n.val;
    }

    public void put(int key, int value) {
        DNode n = items.get(key);
        if (n != null) {
            n.val = value;
            remove(n);
            pushFront(n);
            return;
        }
        n = new DNode(key, value);
        items.put(key, n);
        pushFront(n);
        if (items.size() > cap) {
            DNode lru = tail.prev;
            remove(lru);
            items.remove(lru.key);
        }
    }
}
```
**Complexity:** O(1) per operation · Space O(capacity)

#### Approach 2: `LinkedHashMap` in Access Order
```java
class LRUCache2 extends LinkedHashMap<Integer, Integer> {
    private final int cap;

    public LRUCache2(int capacity) {
        super(capacity, 0.75f, true); // accessOrder = true
        cap = capacity;
    }

    public int get(int key) { return super.getOrDefault(key, -1); }

    public void put(int key, int value) { super.put(key, value); }

    @Override
    protected boolean removeEldestEntry(Map.Entry<Integer, Integer> eldest) {
        return size() > cap;
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
```java
class Solution {
    public ListNode mergeKLists(ListNode[] lists) {
        PriorityQueue<ListNode> pq = new PriorityQueue<>((a, b) -> Integer.compare(a.val, b.val));
        for (ListNode l : lists) if (l != null) pq.offer(l);
        ListNode dummy = new ListNode(), tail = dummy;
        while (!pq.isEmpty()) {
            ListNode n = pq.poll();
            tail.next = n;
            tail = n;
            if (n.next != null) pq.offer(n.next);
        }
        return dummy.next;
    }
}
```
**Complexity:** Time O(N log k) for N total nodes · Space O(k)

#### Approach 2: Divide and Conquer Pairwise Merge (Optimal, O(1) extra)
```java
class Solution {
    public ListNode mergeKLists(ListNode[] lists) {
        if (lists.length == 0) return null;
        for (int step = 1; step < lists.length; step *= 2)
            for (int i = 0; i + step < lists.length; i += 2 * step)
                lists[i] = merge2(lists[i], lists[i + step]);
        return lists[0];
    }

    private ListNode merge2(ListNode a, ListNode b) {
        ListNode dummy = new ListNode(), t = dummy;
        while (a != null && b != null) {
            if (a.val <= b.val) { t.next = a; a = a.next; }
            else { t.next = b; b = b.next; }
            t = t.next;
        }
        t.next = (a != null) ? a : b;
        return dummy.next;
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

#### Approach 1: Iterative (Optimal)
```java
class Solution {
    public ListNode reverseKGroup(ListNode head, int k) {
        ListNode dummy = new ListNode(0, head), groupPrev = dummy;
        while (true) {
            // find the kth node of this group
            ListNode kth = groupPrev;
            for (int i = 0; i < k && kth != null; i++) kth = kth.next;
            if (kth == null) break;
            ListNode groupNext = kth.next;
            // reverse the group
            ListNode prev = groupNext, cur = groupPrev.next;
            while (cur != groupNext) {
                ListNode next = cur.next;
                cur.next = prev;
                prev = cur;
                cur = next;
            }
            ListNode first = groupPrev.next; // becomes this group's tail
            groupPrev.next = kth;
            groupPrev = first;
        }
        return dummy.next;
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Recursive
```java
class Solution {
    public ListNode reverseKGroup(ListNode head, int k) {
        ListNode cur = head;
        int count = 0;
        while (cur != null && count < k) {
            cur = cur.next;
            count++;
        }
        if (count < k) return head;
        ListNode prev = reverseKGroup(cur, k), node = head; // cur is the (k+1)th node
        for (int i = 0; i < k; i++) {
            ListNode next = node.next;
            node.next = prev;
            prev = node;
            node = next;
        }
        return prev;
    }
}
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
```java
class Solution {
    public TreeNode invertTree(TreeNode root) {
        if (root == null) return null;
        TreeNode left = invertTree(root.left);
        root.left = invertTree(root.right);
        root.right = left;
        return root;
    }
}
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: Iterative BFS
```java
class Solution {
    public TreeNode invertTree(TreeNode root) {
        if (root == null) return null;
        Deque<TreeNode> q = new ArrayDeque<>();
        q.offer(root);
        while (!q.isEmpty()) {
            TreeNode n = q.poll();
            TreeNode tmp = n.left;
            n.left = n.right;
            n.right = tmp;
            if (n.left != null) q.offer(n.left);
            if (n.right != null) q.offer(n.right);
        }
        return root;
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
```java
class Solution {
    public int maxDepth(TreeNode root) {
        return root == null ? 0 : 1 + Math.max(maxDepth(root.left), maxDepth(root.right));
    }
}
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: BFS Level Count
```java
class Solution {
    public int maxDepth(TreeNode root) {
        if (root == null) return 0;
        Deque<TreeNode> q = new ArrayDeque<>();
        q.offer(root);
        int depth = 0;
        while (!q.isEmpty()) {
            depth++;
            for (int sz = q.size(); sz > 0; sz--) {
                TreeNode n = q.poll();
                if (n.left != null) q.offer(n.left);
                if (n.right != null) q.offer(n.right);
            }
        }
        return depth;
    }
}
```
**Complexity:** Time O(n) · Space O(w)

#### Approach 3: Iterative DFS with an Explicit Stack
```java
class Solution {
    public int maxDepth(TreeNode root) {
        if (root == null) return 0;
        Deque<TreeNode> nodes = new ArrayDeque<>();
        Deque<Integer> depths = new ArrayDeque<>();
        nodes.push(root);
        depths.push(1);
        int best = 0;
        while (!nodes.isEmpty()) {
            TreeNode n = nodes.pop();
            int d = depths.pop();
            best = Math.max(best, d);
            if (n.left != null) { nodes.push(n.left); depths.push(d + 1); }
            if (n.right != null) { nodes.push(n.right); depths.push(d + 1); }
        }
        return best;
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
```java
class Solution {
    private int best = 0;

    public int diameterOfBinaryTree(TreeNode root) {
        height(root);
        return best;
    }

    private int height(TreeNode n) {
        if (n == null) return 0;
        int l = height(n.left), r = height(n.right);
        best = Math.max(best, l + r);
        return 1 + Math.max(l, r);
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
```java
class Solution {
    public boolean isBalanced(TreeNode root) {
        if (root == null) return true;
        return Math.abs(height(root.left) - height(root.right)) <= 1
                && isBalanced(root.left) && isBalanced(root.right);
    }

    private int height(TreeNode n) {
        return n == null ? 0 : 1 + Math.max(height(n.left), height(n.right));
    }
}
```
**Complexity:** Time O(n²) worst case · Space O(h)

#### Approach 2: Bottom-Up with Early Exit (Optimal)
```java
class Solution {
    public boolean isBalanced(TreeNode root) {
        return check(root) != -1;
    }

    private int check(TreeNode n) { // returns the height, or -1 if unbalanced
        if (n == null) return 0;
        int l = check(n.left);
        if (l == -1) return -1;
        int r = check(n.right);
        if (r == -1 || Math.abs(l - r) > 1) return -1;
        return 1 + Math.max(l, r);
    }
}
```
**Complexity:** Time O(n) · Space O(h)

---

### 100. Same Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/same-tree/)

**Problem:** Return whether two binary trees have the same structure and the same node values.

#### Approach 1: Recursive (Optimal)
```java
class Solution {
    public boolean isSameTree(TreeNode p, TreeNode q) {
        if (p == null || q == null) return p == q;
        return p.val == q.val && isSameTree(p.left, q.left) && isSameTree(p.right, q.right);
    }
}
```
**Complexity:** Time O(min(n, m)) · Space O(h)

#### Approach 2: Iterative BFS on Pairs
```java
class Solution {
    public boolean isSameTree(TreeNode p, TreeNode q) {
        Deque<TreeNode[]> queue = new ArrayDeque<>();
        queue.offer(new TreeNode[]{p, q});
        while (!queue.isEmpty()) {
            TreeNode[] pair = queue.poll();
            TreeNode a = pair[0], b = pair[1];
            if (a == null && b == null) continue;
            if (a == null || b == null || a.val != b.val) return false;
            queue.offer(new TreeNode[]{a.left, b.left});
            queue.offer(new TreeNode[]{a.right, b.right});
        }
        return true;
    }
}
```
**Complexity:** Time O(n) · Space O(w)

> `ArrayDeque` doesn't accept `null` elements, but an array that *contains* nulls is fine, which is why the pairs are wrapped in `TreeNode[]`.

---

### 572. Subtree of Another Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/subtree-of-another-tree/)

**Problem:** Return whether `subRoot` matches, in structure and values, some node of `root` together with all of that node's descendants.

**Key insight:** Run Same Tree at every node (O(m·n)), or serialize both trees and use substring search (O(m+n)).

#### Approach 1: Same-Tree Check at Every Node
```java
class Solution {
    public boolean isSubtree(TreeNode root, TreeNode subRoot) {
        if (root == null) return subRoot == null;
        return same(root, subRoot) || isSubtree(root.left, subRoot) || isSubtree(root.right, subRoot);
    }

    private boolean same(TreeNode a, TreeNode b) {
        if (a == null || b == null) return a == b;
        return a.val == b.val && same(a.left, b.left) && same(a.right, b.right);
    }
}
```
**Complexity:** Time O(m · n) · Space O(h)

#### Approach 2: Serialize + Substring Search
Delimiters around each value (`^12` vs `^2`) and explicit null markers prevent false matches.
```java
class Solution {
    public boolean isSubtree(TreeNode root, TreeNode subRoot) {
        StringBuilder a = new StringBuilder(), b = new StringBuilder();
        serialize(root, a);
        serialize(subRoot, b);
        return a.indexOf(b.toString()) >= 0; // swap in KMP for a guaranteed O(m+n)
    }

    private void serialize(TreeNode n, StringBuilder sb) {
        if (n == null) {
            sb.append('#');
            return;
        }
        sb.append('^').append(n.val);
        serialize(n.left, sb);
        serialize(n.right, sb);
    }
}
```
**Complexity:** Time O(m + n) with KMP; `indexOf` is fast in practice · Space O(m + n)

---

### 235. Lowest Common Ancestor of a Binary Search Tree
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)

**Problem:** Given a BST and two of its nodes `p` and `q`, return their lowest common ancestor: the deepest node that has both as descendants (a node counts as its own descendant).

**Key insight:** If both values are smaller than the current node, go left. If both are larger, go right. Otherwise the current node is where they split, and that's the LCA.

#### Approach 1: Iterative (Optimal)
```java
class Solution {
    public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
        TreeNode cur = root;
        while (cur != null) {
            if (p.val < cur.val && q.val < cur.val) cur = cur.left;
            else if (p.val > cur.val && q.val > cur.val) cur = cur.right;
            else return cur;
        }
        return null;
    }
}
```
**Complexity:** Time O(h) · Space O(1)

#### Approach 2: Recursive
```java
class Solution {
    public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
        if (p.val < root.val && q.val < root.val) return lowestCommonAncestor(root.left, p, q);
        if (p.val > root.val && q.val > root.val) return lowestCommonAncestor(root.right, p, q);
        return root;
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
```java
class Solution {
    public List<List<Integer>> levelOrder(TreeNode root) {
        List<List<Integer>> res = new ArrayList<>();
        if (root == null) return res;
        Deque<TreeNode> q = new ArrayDeque<>();
        q.offer(root);
        while (!q.isEmpty()) {
            List<Integer> level = new ArrayList<>();
            for (int sz = q.size(); sz > 0; sz--) {
                TreeNode n = q.poll();
                level.add(n.val);
                if (n.left != null) q.offer(n.left);
                if (n.right != null) q.offer(n.right);
            }
            res.add(level);
        }
        return res;
    }
}
```
**Complexity:** Time O(n) · Space O(w)

#### Approach 2: DFS with a Depth Index
```java
class Solution {
    public List<List<Integer>> levelOrder(TreeNode root) {
        List<List<Integer>> res = new ArrayList<>();
        dfs(root, 0, res);
        return res;
    }

    private void dfs(TreeNode n, int d, List<List<Integer>> res) {
        if (n == null) return;
        if (d == res.size()) res.add(new ArrayList<>());
        res.get(d).add(n.val);
        dfs(n.left, d + 1, res);
        dfs(n.right, d + 1, res);
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
```java
class Solution {
    public List<Integer> rightSideView(TreeNode root) {
        List<Integer> res = new ArrayList<>();
        if (root == null) return res;
        Deque<TreeNode> q = new ArrayDeque<>();
        q.offer(root);
        while (!q.isEmpty()) {
            int sz = q.size();
            for (int i = 0; i < sz; i++) {
                TreeNode n = q.poll();
                if (i == sz - 1) res.add(n.val);
                if (n.left != null) q.offer(n.left);
                if (n.right != null) q.offer(n.right);
            }
        }
        return res;
    }
}
```
**Complexity:** Time O(n) · Space O(w)

#### Approach 2: Right-First DFS
```java
class Solution {
    public List<Integer> rightSideView(TreeNode root) {
        List<Integer> res = new ArrayList<>();
        dfs(root, 0, res);
        return res;
    }

    private void dfs(TreeNode n, int d, List<Integer> res) {
        if (n == null) return;
        if (d == res.size()) res.add(n.val);
        dfs(n.right, d + 1, res);
        dfs(n.left, d + 1, res);
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
```java
class Solution {
    public int goodNodes(TreeNode root) {
        return dfs(root, root.val);
    }

    private int dfs(TreeNode n, int maxSoFar) {
        if (n == null) return 0;
        int good = n.val >= maxSoFar ? 1 : 0;
        int m = Math.max(maxSoFar, n.val);
        return good + dfs(n.left, m) + dfs(n.right, m);
    }
}
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: BFS Carrying (node, max)
```java
class Solution {
    public int goodNodes(TreeNode root) {
        Deque<Object[]> q = new ArrayDeque<>();
        q.offer(new Object[]{root, root.val});
        int count = 0;
        while (!q.isEmpty()) {
            Object[] it = q.poll();
            TreeNode n = (TreeNode) it[0];
            int mx = (Integer) it[1];
            if (n.val >= mx) count++;
            mx = Math.max(mx, n.val);
            if (n.left != null) q.offer(new Object[]{n.left, mx});
            if (n.right != null) q.offer(new Object[]{n.right, mx});
        }
        return count;
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
```java
class Solution {
    public boolean isValidBST(TreeNode root) {
        return valid(root, Long.MIN_VALUE, Long.MAX_VALUE); // long bounds handle Integer.MIN/MAX node values
    }

    private boolean valid(TreeNode n, long lo, long hi) {
        if (n == null) return true;
        if (n.val <= lo || n.val >= hi) return false;
        return valid(n.left, lo, n.val) && valid(n.right, n.val, hi);
    }
}
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: Iterative In-Order Traversal
```java
class Solution {
    public boolean isValidBST(TreeNode root) {
        Deque<TreeNode> stack = new ArrayDeque<>();
        TreeNode cur = root, prev = null;
        while (cur != null || !stack.isEmpty()) {
            while (cur != null) {
                stack.push(cur);
                cur = cur.left;
            }
            cur = stack.pop();
            if (prev != null && cur.val <= prev.val) return false;
            prev = cur;
            cur = cur.right;
        }
        return true;
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
```java
class Solution {
    public int kthSmallest(TreeNode root, int k) {
        Deque<TreeNode> stack = new ArrayDeque<>();
        TreeNode cur = root;
        while (true) {
            while (cur != null) {
                stack.push(cur);
                cur = cur.left;
            }
            cur = stack.pop();
            if (--k == 0) return cur.val;
            cur = cur.right;
        }
    }
}
```
**Complexity:** Time O(h + k) · Space O(h)

#### Approach 2: Recursive In-Order with a Counter
```java
class Solution {
    private int count, ans;

    public int kthSmallest(TreeNode root, int k) {
        count = k;
        inorder(root);
        return ans;
    }

    private void inorder(TreeNode n) {
        if (n == null || count == 0) return;
        inorder(n.left);
        if (--count == 0) {
            ans = n.val;
            return;
        }
        inorder(n.right);
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

#### Approach 1: Array Copies (simple, O(n²))
```java
class Solution {
    public TreeNode buildTree(int[] preorder, int[] inorder) {
        if (preorder.length == 0) return null;
        TreeNode root = new TreeNode(preorder[0]);
        int mid = 0;
        while (inorder[mid] != preorder[0]) mid++;
        root.left = buildTree(Arrays.copyOfRange(preorder, 1, mid + 1), Arrays.copyOfRange(inorder, 0, mid));
        root.right = buildTree(Arrays.copyOfRange(preorder, mid + 1, preorder.length),
                               Arrays.copyOfRange(inorder, mid + 1, inorder.length));
        return root;
    }
}
```
**Complexity:** Time O(n²) worst case · Space O(n²) from the copies

#### Approach 2: HashMap of Inorder Indices (Optimal)
```java
class Solution {
    private int pre = 0;
    private int[] preorder;
    private final Map<Integer, Integer> pos = new HashMap<>();

    public TreeNode buildTree(int[] preorder, int[] inorder) {
        this.preorder = preorder;
        for (int i = 0; i < inorder.length; i++) pos.put(inorder[i], i);
        return build(0, inorder.length - 1);
    }

    private TreeNode build(int lo, int hi) { // inorder range [lo, hi]
        if (lo > hi) return null;
        int v = preorder[pre++];
        TreeNode root = new TreeNode(v);
        int mid = pos.get(v);
        root.left = build(lo, mid - 1);
        root.right = build(mid + 1, hi);
        return root;
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
```java
class Solution {
    private int best = Integer.MIN_VALUE;

    public int maxPathSum(TreeNode root) {
        gain(root);
        return best;
    }

    private int gain(TreeNode n) {
        if (n == null) return 0;
        int l = Math.max(0, gain(n.left));
        int r = Math.max(0, gain(n.right));
        best = Math.max(best, n.val + l + r); // best path that bends at n
        return n.val + Math.max(l, r);        // only one branch can extend upward
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
```java
public class Codec {
    public String serialize(TreeNode root) {
        StringBuilder sb = new StringBuilder();
        ser(root, sb);
        return sb.toString();
    }

    private void ser(TreeNode n, StringBuilder sb) {
        if (n == null) {
            sb.append("N,");
            return;
        }
        sb.append(n.val).append(',');
        ser(n.left, sb);
        ser(n.right, sb);
    }

    public TreeNode deserialize(String data) {
        Deque<String> tokens = new ArrayDeque<>(Arrays.asList(data.split(",")));
        return des(tokens);
    }

    private TreeNode des(Deque<String> tokens) {
        String v = tokens.poll();
        if (v.equals("N")) return null;
        TreeNode node = new TreeNode(Integer.parseInt(v));
        node.left = des(tokens);
        node.right = des(tokens);
        return node;
    }
}
```
**Complexity:** Time O(n) for both · Space O(n)

#### Approach 2: BFS (level order, LeetCode's own format)
```java
public class Codec {
    public String serialize(TreeNode root) {
        if (root == null) return "";
        StringBuilder sb = new StringBuilder();
        Queue<TreeNode> q = new LinkedList<>(); // LinkedList allows null elements
        q.offer(root);
        while (!q.isEmpty()) {
            TreeNode n = q.poll();
            if (n == null) {
                sb.append("N,");
                continue;
            }
            sb.append(n.val).append(',');
            q.offer(n.left);
            q.offer(n.right);
        }
        return sb.toString();
    }

    public TreeNode deserialize(String data) {
        if (data.isEmpty()) return null;
        String[] vals = data.split(",");
        TreeNode root = new TreeNode(Integer.parseInt(vals[0]));
        Deque<TreeNode> q = new ArrayDeque<>();
        q.offer(root);
        int i = 1;
        while (!q.isEmpty()) {
            TreeNode n = q.poll();
            if (!vals[i].equals("N")) {
                n.left = new TreeNode(Integer.parseInt(vals[i]));
                q.offer(n.left);
            }
            i++;
            if (!vals[i].equals("N")) {
                n.right = new TreeNode(Integer.parseInt(vals[i]));
                q.offer(n.right);
            }
            i++;
        }
        return root;
    }
}
```
**Complexity:** Time O(n) · Space O(n)

---
## 8. Tries

### 208. Implement Trie (Prefix Tree)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/implement-trie-prefix-tree/)

**Problem:** Implement a trie with `insert(word)`, `search(word)` (is this exact word stored?), and `startsWith(prefix)` (does any stored word begin with this prefix?). Inputs are lowercase letters.

**Key insight:** Each node has up to 26 children plus an end-of-word flag.

#### Approach 1: Fixed Array of Children (Optimal for lowercase)
```java
class Trie {
    private final Trie[] children = new Trie[26];
    private boolean isEnd;

    public Trie() {}

    public void insert(String word) {
        Trie node = this;
        for (char c : word.toCharArray()) {
            int i = c - 'a';
            if (node.children[i] == null) node.children[i] = new Trie();
            node = node.children[i];
        }
        node.isEnd = true;
    }

    private Trie find(String s) {
        Trie node = this;
        for (char c : s.toCharArray()) {
            node = node.children[c - 'a'];
            if (node == null) return null;
        }
        return node;
    }

    public boolean search(String word) {
        Trie n = find(word);
        return n != null && n.isEnd;
    }

    public boolean startsWith(String prefix) {
        return find(prefix) != null;
    }
}
```
**Complexity:** Time O(L) per operation · Space O(total characters × 26)

#### Approach 2: HashMap Children (any alphabet, less memory when sparse)
```java
class Trie2 {
    private final Map<Character, Trie2> children = new HashMap<>();
    private boolean isEnd;

    public void insert(String word) {
        Trie2 node = this;
        for (char c : word.toCharArray()) node = node.children.computeIfAbsent(c, k -> new Trie2());
        node.isEnd = true;
    }

    public boolean search(String word) {
        Trie2 n = find(word);
        return n != null && n.isEnd;
    }

    public boolean startsWith(String prefix) {
        return find(prefix) != null;
    }

    private Trie2 find(String s) {
        Trie2 node = this;
        for (char c : s.toCharArray()) {
            node = node.children.get(c);
            if (node == null) return null;
        }
        return node;
    }
}
```
**Complexity:** Time O(L) average · Space O(total characters)

---

### 211. Design Add and Search Words Data Structure
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/design-add-and-search-words-data-structure/)

**Problem:** Support `addWord(word)` and `search(word)`. The search pattern may contain `.`, which matches any single letter.

**Example:** add `bad`, `dad`, `mad`; `search("pad")` → `false`, `search(".ad")` → `true`, `search("b..")` → `true`

**Key insight:** Use a trie. On `.`, run DFS into every child.

#### Approach: Trie + DFS (Optimal)
```java
class WordDictionary {
    private static class Node {
        Node[] children = new Node[26];
        boolean isEnd;
    }

    private final Node root = new Node();

    public WordDictionary() {}

    public void addWord(String word) {
        Node n = root;
        for (char c : word.toCharArray()) {
            if (n.children[c - 'a'] == null) n.children[c - 'a'] = new Node();
            n = n.children[c - 'a'];
        }
        n.isEnd = true;
    }

    public boolean search(String word) {
        return dfs(word, 0, root);
    }

    private boolean dfs(String w, int i, Node n) {
        if (n == null) return false;
        if (i == w.length()) return n.isEnd;
        char c = w.charAt(i);
        if (c == '.') {
            for (Node ch : n.children)
                if (ch != null && dfs(w, i + 1, ch)) return true;
            return false;
        }
        return dfs(w, i + 1, n.children[c - 'a']);
    }
}
```
**Complexity:** `addWord` O(L) · `search` O(L) without dots, O(26^L) worst case with all dots · Space O(total characters)

**Alternative:** Bucket words by length in a `Map<Integer, List<String>>` and compare character by character. Simpler, but `search` costs O(words × L).

---

### 212. Word Search II
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/word-search-ii/)

**Problem:** Given an `m × n` letter grid and a list of `words`, return every word that can be traced through horizontally or vertically adjacent cells, without reusing a cell within a word.

**Example:** `board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]` → `["eat","oath"]`

**Key insight:** Build a trie from all the words and run one DFS per cell, descending the grid and the trie together. Prune trie nodes once their words are found.

#### Approach 1: Trie + Backtracking (no pruning)
```java
class Solution {
    private static class Node {
        Node[] children = new Node[26];
        String word; // non-null at the end of a word
    }

    public List<String> findWords(char[][] board, String[] words) {
        Node root = new Node();
        for (String w : words) {
            Node n = root;
            for (char c : w.toCharArray()) {
                if (n.children[c - 'a'] == null) n.children[c - 'a'] = new Node();
                n = n.children[c - 'a'];
            }
            n.word = w;
        }
        List<String> res = new ArrayList<>();
        for (int r = 0; r < board.length; r++)
            for (int c = 0; c < board[0].length; c++)
                dfs(board, r, c, root, res);
        return res;
    }

    private void dfs(char[][] b, int r, int c, Node parent, List<String> res) {
        if (r < 0 || c < 0 || r >= b.length || c >= b[0].length || b[r][c] == '#') return;
        char ch = b[r][c];
        Node node = parent.children[ch - 'a'];
        if (node == null) return;
        if (node.word != null) {
            res.add(node.word);
            node.word = null; // report each word once
        }
        b[r][c] = '#';
        dfs(b, r + 1, c, node, res);
        dfs(b, r - 1, c, node, res);
        dfs(b, r, c + 1, node, res);
        dfs(b, r, c - 1, node, res);
        b[r][c] = ch;
    }
}
```
**Complexity:** Time O(m · n · 4 · 3^(L-1)), where L is the maximum word length · Space O(total characters in `words`)

#### Approach 2: Trie + Backtracking with Leaf Pruning (Optimal)
```java
class Solution {
    private static class Node {
        Node[] children = new Node[26];
        String word;
        int childCount;
    }

    public List<String> findWords(char[][] board, String[] words) {
        Node root = new Node();
        for (String w : words) {
            Node n = root;
            for (char c : w.toCharArray()) {
                int i = c - 'a';
                if (n.children[i] == null) {
                    n.children[i] = new Node();
                    n.childCount++;
                }
                n = n.children[i];
            }
            n.word = w;
        }
        List<String> res = new ArrayList<>();
        for (int r = 0; r < board.length; r++)
            for (int c = 0; c < board[0].length; c++)
                if (root.children[board[r][c] - 'a'] != null) dfs(board, r, c, root, res);
        return res;
    }

    private static final int[][] DIRS = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};

    private void dfs(char[][] b, int r, int c, Node parent, List<String> res) {
        char ch = b[r][c];
        int idx = ch - 'a';
        Node node = parent.children[idx];
        if (node.word != null) {
            res.add(node.word);
            node.word = null;
        }
        b[r][c] = '#';
        for (int[] d : DIRS) {
            int nr = r + d[0], nc = c + d[1];
            if (nr < 0 || nc < 0 || nr >= b.length || nc >= b[0].length) continue;
            char nx = b[nr][nc];
            if (nx != '#' && node.children[nx - 'a'] != null) dfs(b, nr, nc, node, res);
        }
        b[r][c] = ch;
        if (node.childCount == 0 && node.word == null) { // prune an exhausted leaf
            parent.children[idx] = null;
            parent.childCount--;
        }
    }
}
```
**Complexity:** Same worst case. Removing leaves cuts the search space sharply in practice; this is the version to use in interviews.

**Brute force:** Run LC 79 (Word Search) once per word: O(W · m · n · 4^L). Much slower when there are many words.

---
## 9. Heap / Priority Queue

> Java's `PriorityQueue` is a **min-heap** by default. Pass `Collections.reverseOrder()` or a comparator to get a max-heap. `offer` and `poll` are O(log n), `peek` is O(1), and `remove(Object)` is O(n).

### 703. Kth Largest Element in a Stream
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/kth-largest-element-in-a-stream/)

**Problem:** Design a class built from `k` and an initial array `nums`. Each call to `add(val)` adds a value to the stream and returns the current `k`th largest element.

**Example:** `k = 3, nums = [4,5,8,2]`; `add(3)` → 4, `add(5)` → 5, `add(10)` → 5

**Key insight:** Keep a min-heap of size `k`. Its top is the `k`th largest element.

#### Approach: Min-Heap of Size k (Optimal)
```java
class KthLargest {
    private final int k;
    private final PriorityQueue<Integer> heap = new PriorityQueue<>();

    public KthLargest(int k, int[] nums) {
        this.k = k;
        for (int n : nums) add(n);
    }

    public int add(int val) {
        heap.offer(val);
        if (heap.size() > k) heap.poll();
        return heap.peek();
    }
}
```
**Complexity:** `add` O(log k) · Space O(k)

**Alternative:** Keep a sorted `ArrayList` and insert with `Collections.binarySearch`. `add` is then O(n), from shifting elements.

---

### 1046. Last Stone Weight
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/last-stone-weight/)

**Problem:** Each turn, smash the two heaviest stones `x ≤ y`. If they're equal, both are destroyed. Otherwise a stone of weight `y - x` remains. Return the weight of the last stone, or `0` if none are left.

**Example:** `[2,7,4,1,8,1]` → `1`

#### Approach 1: Max-Heap (Optimal)
```java
class Solution {
    public int lastStoneWeight(int[] stones) {
        PriorityQueue<Integer> heap = new PriorityQueue<>(Collections.reverseOrder());
        for (int s : stones) heap.offer(s);
        while (heap.size() > 1) {
            int y = heap.poll(), x = heap.poll();
            if (y != x) heap.offer(y - x);
        }
        return heap.isEmpty() ? 0 : heap.peek();
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Bucket Sort (weights ≤ 1000)
```java
class Solution {
    public int lastStoneWeight(int[] stones) {
        int maxW = 0;
        for (int s : stones) maxW = Math.max(maxW, s);
        int[] bucket = new int[maxW + 1];
        for (int s : stones) bucket[s]++;
        int biggest = 0, cur = maxW;
        while (cur > 0) {
            if (bucket[cur] == 0) {
                cur--;
            } else if (biggest == 0) {
                bucket[cur] %= 2; // equal pairs destroy each other
                if (bucket[cur] == 1) biggest = cur;
                cur--;
            } else {
                bucket[cur]--;
                if (biggest - cur <= cur) {
                    bucket[biggest - cur]++;
                    biggest = 0;
                } else {
                    biggest -= cur;
                }
            }
        }
        return biggest;
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
```java
class Solution {
    public int[][] kClosest(int[][] points, int k) {
        Arrays.sort(points, (a, b) -> Integer.compare(a[0] * a[0] + a[1] * a[1], b[0] * b[0] + b[1] * b[1]));
        return Arrays.copyOfRange(points, 0, k);
    }
}
```
**Complexity:** Time O(n log n) · Space O(log n)

#### Approach 2: Max-Heap of Size k
```java
class Solution {
    public int[][] kClosest(int[][] points, int k) {
        PriorityQueue<int[]> heap = new PriorityQueue<>(
                (a, b) -> Integer.compare(b[0] * b[0] + b[1] * b[1], a[0] * a[0] + a[1] * a[1]));
        for (int[] p : points) {
            heap.offer(p);
            if (heap.size() > k) heap.poll(); // drop the farthest
        }
        return heap.toArray(new int[0][]);
    }
}
```
**Complexity:** Time O(n log k) · Space O(k)

#### Approach 3: Quickselect (Optimal on average)
```java
class Solution {
    public int[][] kClosest(int[][] points, int k) {
        int lo = 0, hi = points.length - 1;
        Random rnd = new Random();
        while (lo < hi) {
            swap(points, lo + rnd.nextInt(hi - lo + 1), hi); // random pivot to the end
            int pivot = dist(points[hi]), store = lo;
            for (int i = lo; i < hi; i++)
                if (dist(points[i]) < pivot) swap(points, i, store++);
            swap(points, store, hi);
            if (store == k - 1) break;
            if (store < k - 1) lo = store + 1;
            else hi = store - 1;
        }
        return Arrays.copyOfRange(points, 0, k);
    }

    private int dist(int[] p) { return p[0] * p[0] + p[1] * p[1]; }

    private void swap(int[][] a, int i, int j) { int[] t = a[i]; a[i] = a[j]; a[j] = t; }
}
```
**Complexity:** Time O(n) average, O(n²) worst case · Space O(1)

---

### 215. Kth Largest Element in an Array
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/kth-largest-element-in-an-array/)

**Problem:** Return the `k`th largest element in an unsorted array (in sorted order, not the `k`th distinct value).

**Example:** `nums = [3,2,1,5,6,4], k = 2` → `5`

#### Approach 1: Sort
```java
class Solution {
    public int findKthLargest(int[] nums, int k) {
        Arrays.sort(nums);
        return nums[nums.length - k];
    }
}
```
**Complexity:** Time O(n log n) · Space O(log n)

#### Approach 2: Min-Heap of Size k
```java
class Solution {
    public int findKthLargest(int[] nums, int k) {
        PriorityQueue<Integer> heap = new PriorityQueue<>();
        for (int n : nums) {
            heap.offer(n);
            if (heap.size() > k) heap.poll();
        }
        return heap.peek();
    }
}
```
**Complexity:** Time O(n log k) · Space O(k)

#### Approach 3: Quickselect with Three-Way Partition (Optimal on average)
A three-way partition handles many duplicates, and the random pivot avoids worst-case inputs.
```java
class Solution {
    public int findKthLargest(int[] nums, int k) {
        int target = nums.length - k, lo = 0, hi = nums.length - 1; // index in ascending order
        Random rnd = new Random();
        while (true) {
            int pivot = nums[lo + rnd.nextInt(hi - lo + 1)];
            int lt = lo, i = lo, gt = hi;
            while (i <= gt) {
                if (nums[i] < pivot) swap(nums, lt++, i++);
                else if (nums[i] > pivot) swap(nums, i, gt--);
                else i++;
            }
            if (target < lt) hi = lt - 1;
            else if (target > gt) lo = gt + 1;
            else return pivot;
        }
    }

    private void swap(int[] a, int i, int j) { int t = a[i]; a[i] = a[j]; a[j] = t; }
}
```
**Complexity:** Time O(n) average · Space O(1)

#### Approach 4: Counting Sort (values in [-10⁴, 10⁴])
```java
class Solution {
    public int findKthLargest(int[] nums, int k) {
        final int off = 10000;
        int[] cnt = new int[2 * off + 1];
        for (int n : nums) cnt[n + off]++;
        for (int v = 2 * off; v >= 0; v--) {
            k -= cnt[v];
            if (k <= 0) return v - off;
        }
        return -1;
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
```java
class Solution {
    public int leastInterval(char[] tasks, int n) {
        int[] freq = new int[26];
        for (char t : tasks) freq[t - 'A']++;
        PriorityQueue<Integer> heap = new PriorityQueue<>(Collections.reverseOrder());
        for (int f : freq) if (f > 0) heap.offer(f);
        Deque<int[]> cooldown = new ArrayDeque<>(); // {remaining count, time when ready}
        int time = 0;
        while (!heap.isEmpty() || !cooldown.isEmpty()) {
            time++;
            if (!heap.isEmpty()) {
                int c = heap.poll() - 1;
                if (c > 0) cooldown.offer(new int[]{c, time + n});
            }
            if (!cooldown.isEmpty() && cooldown.peek()[1] == time) heap.offer(cooldown.poll()[0]);
        }
        return time;
    }
}
```
**Complexity:** Time O(T · log 26) = O(T), where T is the total time · Space O(26)

#### Approach 2: Math Formula (Optimal)
```java
class Solution {
    public int leastInterval(char[] tasks, int n) {
        int[] freq = new int[26];
        int maxF = 0;
        for (char t : tasks) maxF = Math.max(maxF, ++freq[t - 'A']);
        int maxCount = 0;
        for (int f : freq) if (f == maxF) maxCount++;
        return Math.max(tasks.length, (maxF - 1) * (n + 1) + maxCount);
    }
}
```
**Complexity:** Time O(len(tasks)) · Space O(1)

---

### 355. Design Twitter
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/design-twitter/)

**Problem:** Design a simplified Twitter with `postTweet(userId, tweetId)`, `getNewsFeed(userId)` (the 10 most recent tweet IDs from the user and the people they follow, newest first), `follow`, and `unfollow`.

**Key insight:** The news feed is a merge of k sorted lists. Each user's tweets are already in time order, so a heap seeded with each followee's newest tweet works.

#### Approach: HashMaps + Heap Merge (Optimal)
```java
class Twitter {
    private int clock = 0;
    private final Map<Integer, List<int[]>> tweets = new HashMap<>();   // user → list of {time, tweetId}
    private final Map<Integer, Set<Integer>> follows = new HashMap<>();

    public Twitter() {}

    public void postTweet(int userId, int tweetId) {
        tweets.computeIfAbsent(userId, k -> new ArrayList<>()).add(new int[]{clock++, tweetId});
    }

    public List<Integer> getNewsFeed(int userId) {
        // heap entries: {time, tweetId, userId, index into that user's list}; newest first
        PriorityQueue<int[]> heap = new PriorityQueue<>((a, b) -> Integer.compare(b[0], a[0]));
        Set<Integer> users = new HashSet<>(follows.getOrDefault(userId, Collections.emptySet()));
        users.add(userId);
        for (int u : users) {
            List<int[]> ts = tweets.get(u);
            if (ts != null && !ts.isEmpty()) {
                int last = ts.size() - 1;
                heap.offer(new int[]{ts.get(last)[0], ts.get(last)[1], u, last});
            }
        }
        List<Integer> res = new ArrayList<>();
        while (!heap.isEmpty() && res.size() < 10) {
            int[] top = heap.poll();
            res.add(top[1]);
            if (top[3] > 0) {
                int[] next = tweets.get(top[2]).get(top[3] - 1);
                heap.offer(new int[]{next[0], next[1], top[2], top[3] - 1});
            }
        }
        return res;
    }

    public void follow(int followerId, int followeeId) {
        follows.computeIfAbsent(followerId, k -> new HashSet<>()).add(followeeId);
    }

    public void unfollow(int followerId, int followeeId) {
        Set<Integer> s = follows.get(followerId);
        if (s != null) s.remove(followeeId);
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
```java
class MedianFinder {
    private final PriorityQueue<Integer> low = new PriorityQueue<>(Collections.reverseOrder()); // max-heap
    private final PriorityQueue<Integer> high = new PriorityQueue<>();                          // min-heap

    public MedianFinder() {}

    public void addNum(int num) {
        low.offer(num);
        high.offer(low.poll()); // move low's max to high
        if (high.size() > low.size()) low.offer(high.poll());
    }

    public double findMedian() {
        if (low.size() > high.size()) return low.peek();
        return ((long) low.peek() + high.peek()) / 2.0;
    }
}
```
**Complexity:** `addNum` O(log n) · `findMedian` O(1) · Space O(n)

#### Approach 2: Sorted List with Binary Insert (simple)
```java
class MedianFinder2 {
    private final List<Integer> data = new ArrayList<>();

    public void addNum(int num) {
        int i = Collections.binarySearch(data, num);
        if (i < 0) i = -i - 1;
        data.add(i, num);
    }

    public double findMedian() {
        int n = data.size();
        if (n % 2 == 1) return data.get(n / 2);
        return ((long) data.get(n / 2 - 1) + data.get(n / 2)) / 2.0;
    }
}
```
**Complexity:** `addNum` O(n) · `findMedian` O(1)

**Follow-up (all values in [0, 100]):** Keep a 101-bucket count array and walk it to find the median, which is O(100) per query.

---
## 10. Backtracking

> **Template:** choose → recurse → un-choose. In Java, always save a **copy** (`res.add(new ArrayList<>(path))`). Saving `path` itself stores a reference, and later changes would overwrite results you've already stored.

### 78. Subsets
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/subsets/)

**Problem:** Given an array of unique integers, return every possible subset (the power set), with no duplicates.

**Example:** `[1,2,3]` → `[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]`

#### Approach 1: Backtracking (include or exclude)
```java
class Solution {
    public List<List<Integer>> subsets(int[] nums) {
        List<List<Integer>> res = new ArrayList<>();
        bt(nums, 0, new ArrayList<>(), res);
        return res;
    }

    private void bt(int[] nums, int i, List<Integer> path, List<List<Integer>> res) {
        if (i == nums.length) {
            res.add(new ArrayList<>(path));
            return;
        }
        path.add(nums[i]);                 // include
        bt(nums, i + 1, path, res);
        path.remove(path.size() - 1);      // exclude
        bt(nums, i + 1, path, res);
    }
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n) recursion

#### Approach 2: Iterative Cascading
```java
class Solution {
    public List<List<Integer>> subsets(int[] nums) {
        List<List<Integer>> res = new ArrayList<>();
        res.add(new ArrayList<>());
        for (int n : nums) {
            int size = res.size();
            for (int i = 0; i < size; i++) {
                List<Integer> s = new ArrayList<>(res.get(i));
                s.add(n);
                res.add(s);
            }
        }
        return res;
    }
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n · 2ⁿ) for the output

#### Approach 3: Bitmask
```java
class Solution {
    public List<List<Integer>> subsets(int[] nums) {
        int n = nums.length;
        List<List<Integer>> res = new ArrayList<>();
        for (int mask = 0; mask < (1 << n); mask++) {
            List<Integer> s = new ArrayList<>();
            for (int i = 0; i < n; i++) if ((mask & (1 << i)) != 0) s.add(nums[i]);
            res.add(s);
        }
        return res;
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
```java
class Solution {
    public List<List<Integer>> combinationSum(int[] candidates, int target) {
        Arrays.sort(candidates);
        List<List<Integer>> res = new ArrayList<>();
        bt(candidates, 0, target, new ArrayList<>(), res);
        return res;
    }

    private void bt(int[] c, int start, int remain, List<Integer> path, List<List<Integer>> res) {
        if (remain == 0) {
            res.add(new ArrayList<>(path));
            return;
        }
        for (int i = start; i < c.length; i++) {
            if (c[i] > remain) break; // sorted, so every later candidate is too big
            path.add(c[i]);
            bt(c, i, remain - c[i], path, res); // i, not i+1: reuse is allowed
            path.remove(path.size() - 1);
        }
    }
}
```
**Complexity:** Time O(N^(T/M + 1)), where T = target and M = smallest candidate · Space O(T/M) recursion

#### Approach 2: DP Building Combinations (unbounded knapsack style)
```java
class Solution {
    public List<List<Integer>> combinationSum(int[] candidates, int target) {
        List<List<List<Integer>>> dp = new ArrayList<>();
        for (int t = 0; t <= target; t++) dp.add(new ArrayList<>());
        dp.get(0).add(new ArrayList<>());
        for (int c : candidates)           // outer loop over coins means no permutations
            for (int t = c; t <= target; t++)
                for (List<Integer> comb : dp.get(t - c)) {
                    List<Integer> nc = new ArrayList<>(comb);
                    nc.add(c);
                    dp.get(t).add(nc);
                }
        return dp.get(target);
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
```java
class Solution {
    public List<List<Integer>> permute(int[] nums) {
        List<List<Integer>> res = new ArrayList<>();
        bt(nums, new boolean[nums.length], new ArrayList<>(), res);
        return res;
    }

    private void bt(int[] nums, boolean[] used, List<Integer> path, List<List<Integer>> res) {
        if (path.size() == nums.length) {
            res.add(new ArrayList<>(path));
            return;
        }
        for (int i = 0; i < nums.length; i++) {
            if (used[i]) continue;
            used[i] = true;
            path.add(nums[i]);
            bt(nums, used, path, res);
            path.remove(path.size() - 1);
            used[i] = false;
        }
    }
}
```
**Complexity:** Time O(n · n!) · Space O(n)

#### Approach 2: In-Place Swapping
```java
class Solution {
    public List<List<Integer>> permute(int[] nums) {
        List<List<Integer>> res = new ArrayList<>();
        bt(nums, 0, res);
        return res;
    }

    private void bt(int[] nums, int k, List<List<Integer>> res) {
        if (k == nums.length) {
            List<Integer> p = new ArrayList<>();
            for (int n : nums) p.add(n);
            res.add(p);
            return;
        }
        for (int i = k; i < nums.length; i++) {
            swap(nums, k, i);
            bt(nums, k + 1, res);
            swap(nums, k, i);
        }
    }

    private void swap(int[] a, int i, int j) { int t = a[i]; a[i] = a[j]; a[j] = t; }
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
```java
class Solution {
    public List<List<Integer>> subsetsWithDup(int[] nums) {
        Arrays.sort(nums);
        List<List<Integer>> res = new ArrayList<>();
        bt(nums, 0, new ArrayList<>(), res);
        return res;
    }

    private void bt(int[] nums, int start, List<Integer> path, List<List<Integer>> res) {
        res.add(new ArrayList<>(path));
        for (int i = start; i < nums.length; i++) {
            if (i > start && nums[i] == nums[i - 1]) continue; // skip duplicates at the same depth
            path.add(nums[i]);
            bt(nums, i + 1, path, res);
            path.remove(path.size() - 1);
        }
    }
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n)

#### Approach 2: Iterative (extend only the subsets added last round when the value repeats)
```java
class Solution {
    public List<List<Integer>> subsetsWithDup(int[] nums) {
        Arrays.sort(nums);
        List<List<Integer>> res = new ArrayList<>();
        res.add(new ArrayList<>());
        int prevSize = 0;
        for (int i = 0; i < nums.length; i++) {
            int start = (i > 0 && nums[i] == nums[i - 1]) ? prevSize : 0;
            int size = res.size();
            for (int j = start; j < size; j++) {
                List<Integer> s = new ArrayList<>(res.get(j));
                s.add(nums[i]);
                res.add(s);
            }
            prevSize = size;
        }
        return res;
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
```java
class Solution {
    public List<List<Integer>> combinationSum2(int[] candidates, int target) {
        Arrays.sort(candidates);
        List<List<Integer>> res = new ArrayList<>();
        bt(candidates, 0, target, new ArrayList<>(), res);
        return res;
    }

    private void bt(int[] c, int start, int remain, List<Integer> path, List<List<Integer>> res) {
        if (remain == 0) {
            res.add(new ArrayList<>(path));
            return;
        }
        for (int i = start; i < c.length; i++) {
            if (i > start && c[i] == c[i - 1]) continue;
            if (c[i] > remain) break;
            path.add(c[i]);
            bt(c, i + 1, remain - c[i], path, res);
            path.remove(path.size() - 1);
        }
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
```java
class Solution {
    public boolean exist(char[][] board, String word) {
        for (int r = 0; r < board.length; r++)
            for (int c = 0; c < board[0].length; c++)
                if (dfs(board, word, r, c, 0)) return true;
        return false;
    }

    private boolean dfs(char[][] b, String w, int r, int c, int i) {
        if (i == w.length()) return true;
        if (r < 0 || c < 0 || r >= b.length || c >= b[0].length || b[r][c] != w.charAt(i)) return false;
        char tmp = b[r][c];
        b[r][c] = '#';
        boolean found = dfs(b, w, r + 1, c, i + 1) || dfs(b, w, r - 1, c, i + 1)
                || dfs(b, w, r, c + 1, i + 1) || dfs(b, w, r, c - 1, i + 1);
        b[r][c] = tmp;
        return found;
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
```java
class Solution {
    public List<List<String>> partition(String s) {
        List<List<String>> res = new ArrayList<>();
        bt(s, 0, new ArrayList<>(), res);
        return res;
    }

    private void bt(String s, int start, List<String> path, List<List<String>> res) {
        if (start == s.length()) {
            res.add(new ArrayList<>(path));
            return;
        }
        for (int end = start; end < s.length(); end++) {
            if (isPal(s, start, end)) {
                path.add(s.substring(start, end + 1));
                bt(s, end + 1, path, res);
                path.remove(path.size() - 1);
            }
        }
    }

    private boolean isPal(String s, int l, int r) {
        while (l < r) if (s.charAt(l++) != s.charAt(r--)) return false;
        return true;
    }
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n)

#### Approach 2: Precomputed Palindrome DP Table
```java
class Solution {
    public List<List<String>> partition(String s) {
        int n = s.length();
        boolean[][] pal = new boolean[n][n];
        for (int i = n - 1; i >= 0; i--)
            for (int j = i; j < n; j++)
                pal[i][j] = s.charAt(i) == s.charAt(j) && (j - i < 2 || pal[i + 1][j - 1]);
        List<List<String>> res = new ArrayList<>();
        bt(s, 0, pal, new ArrayList<>(), res);
        return res;
    }

    private void bt(String s, int start, boolean[][] pal, List<String> path, List<List<String>> res) {
        if (start == s.length()) {
            res.add(new ArrayList<>(path));
            return;
        }
        for (int end = start; end < s.length(); end++) {
            if (!pal[start][end]) continue;
            path.add(s.substring(start, end + 1));
            bt(s, end + 1, pal, path, res);
            path.remove(path.size() - 1);
        }
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
```java
class Solution {
    private static final String[] KEYS = {"", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"};

    public List<String> letterCombinations(String digits) {
        List<String> res = new ArrayList<>();
        if (digits.isEmpty()) return res;
        bt(digits, 0, new StringBuilder(), res);
        return res;
    }

    private void bt(String digits, int i, StringBuilder sb, List<String> res) {
        if (i == digits.length()) {
            res.add(sb.toString());
            return;
        }
        for (char c : KEYS[digits.charAt(i) - '0'].toCharArray()) {
            sb.append(c);
            bt(digits, i + 1, sb, res);
            sb.deleteCharAt(sb.length() - 1);
        }
    }
}
```
**Complexity:** Time O(n · 4ⁿ) · Space O(n)

#### Approach 2: Iterative BFS
```java
class Solution {
    public List<String> letterCombinations(String digits) {
        if (digits.isEmpty()) return new ArrayList<>();
        String[] keys = {"", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"};
        List<String> res = new ArrayList<>();
        res.add("");
        for (char d : digits.toCharArray()) {
            List<String> next = new ArrayList<>();
            for (String prefix : res)
                for (char c : keys[d - '0'].toCharArray()) next.add(prefix + c);
            res = next;
        }
        return res;
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
```java
class Solution {
    public List<List<String>> solveNQueens(int n) {
        List<List<String>> res = new ArrayList<>();
        int[] queens = new int[n]; // queens[r] = column
        bt(0, n, queens, new boolean[n], new boolean[2 * n], new boolean[2 * n], res);
        return res;
    }

    private void bt(int r, int n, int[] queens, boolean[] cols, boolean[] diag, boolean[] anti,
                    List<List<String>> res) {
        if (r == n) {
            res.add(render(queens, n));
            return;
        }
        for (int c = 0; c < n; c++) {
            if (cols[c] || diag[r - c + n] || anti[r + c]) continue;
            cols[c] = diag[r - c + n] = anti[r + c] = true;
            queens[r] = c;
            bt(r + 1, n, queens, cols, diag, anti, res);
            cols[c] = diag[r - c + n] = anti[r + c] = false;
        }
    }

    private List<String> render(int[] queens, int n) {
        List<String> board = new ArrayList<>();
        for (int q : queens) {
            char[] row = new char[n];
            Arrays.fill(row, '.');
            row[q] = 'Q';
            board.add(new String(row));
        }
        return board;
    }
}
```
**Complexity:** Time O(n!) · Space O(n)

#### Approach 2: Bitmask Backtracking (fastest)
```java
class Solution {
    public List<List<String>> solveNQueens(int n) {
        List<List<String>> res = new ArrayList<>();
        bt(0, n, 0, 0, 0, new int[n], res);
        return res;
    }

    private void bt(int r, int n, int cols, int d1, int d2, int[] queens, List<List<String>> res) {
        if (r == n) {
            List<String> board = new ArrayList<>();
            for (int q : queens) {
                char[] row = new char[n];
                Arrays.fill(row, '.');
                row[q] = 'Q';
                board.add(new String(row));
            }
            res.add(board);
            return;
        }
        int avail = ((1 << n) - 1) & ~(cols | d1 | d2);
        while (avail != 0) {
            int bit = avail & -avail; // lowest set bit
            avail &= avail - 1;
            queens[r] = Integer.numberOfTrailingZeros(bit);
            bt(r + 1, n, cols | bit, (d1 | bit) << 1, (d2 | bit) >> 1, queens, res);
        }
    }
}
```
**Complexity:** Time O(n!) with a much smaller constant · Space O(n)

---
## 11. Graphs

### 200. Number of Islands
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/number-of-islands/)

**Problem:** Given an `m × n` grid of `'1'` (land) and `'0'` (water), count the islands. An island is a group of land cells connected horizontally or vertically.

**Example:** `[["1","1","0"],["0","1","0"],["0","0","1"]]` → `2`

**Key insight:** Each time you find unvisited land, count one island and flood-fill it so none of its cells are counted again.

#### Approach 1: DFS Flood Fill
```java
class Solution {
    public int numIslands(char[][] grid) {
        int count = 0;
        for (int r = 0; r < grid.length; r++)
            for (int c = 0; c < grid[0].length; c++)
                if (grid[r][c] == '1') {
                    count++;
                    sink(grid, r, c);
                }
        return count;
    }

    private void sink(char[][] g, int r, int c) {
        if (r < 0 || c < 0 || r >= g.length || c >= g[0].length || g[r][c] != '1') return;
        g[r][c] = '0';
        sink(g, r + 1, c);
        sink(g, r - 1, c);
        sink(g, r, c + 1);
        sink(g, r, c - 1);
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n) recursion in the worst case

#### Approach 2: BFS
```java
class Solution {
    public int numIslands(char[][] grid) {
        int m = grid.length, n = grid[0].length, count = 0;
        int[][] dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        for (int r = 0; r < m; r++) {
            for (int c = 0; c < n; c++) {
                if (grid[r][c] != '1') continue;
                count++;
                grid[r][c] = '0';
                Deque<int[]> q = new ArrayDeque<>();
                q.offer(new int[]{r, c});
                while (!q.isEmpty()) {
                    int[] cell = q.poll();
                    for (int[] d : dirs) {
                        int nr = cell[0] + d[0], nc = cell[1] + d[1];
                        if (nr >= 0 && nc >= 0 && nr < m && nc < n && grid[nr][nc] == '1') {
                            grid[nr][nc] = '0';
                            q.offer(new int[]{nr, nc});
                        }
                    }
                }
            }
        }
        return count;
    }
}
```
**Complexity:** Time O(m · n) · Space O(min(m, n)) for the queue

#### Approach 3: Union-Find
```java
class Solution {
    private int[] parent;
    private int count;

    public int numIslands(char[][] grid) {
        int m = grid.length, n = grid[0].length;
        parent = new int[m * n];
        for (int r = 0; r < m; r++)
            for (int c = 0; c < n; c++)
                if (grid[r][c] == '1') {
                    parent[r * n + c] = r * n + c;
                    count++;
                }
        for (int r = 0; r < m; r++)
            for (int c = 0; c < n; c++) {
                if (grid[r][c] != '1') continue;
                if (r + 1 < m && grid[r + 1][c] == '1') union(r * n + c, (r + 1) * n + c);
                if (c + 1 < n && grid[r][c + 1] == '1') union(r * n + c, r * n + c + 1);
            }
        return count;
    }

    private int find(int x) {
        while (parent[x] != x) {
            parent[x] = parent[parent[x]]; // path halving
            x = parent[x];
        }
        return x;
    }

    private void union(int a, int b) {
        int ra = find(a), rb = find(b);
        if (ra != rb) {
            parent[ra] = rb;
            count--;
        }
    }
}
```
**Complexity:** Time O(m · n · α(mn)) · Space O(m · n)

---

### 133. Clone Graph
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/clone-graph/)

**Problem:** Given a reference to one node in a connected undirected graph, return a deep copy of the whole graph. Each node has a `val` and a list of `neighbors`.

**Key insight:** A map from original node to clone serves as the visited set and also handles cycles.

#### Approach 1: DFS + HashMap
```java
class Node {
    public int val;
    public List<Node> neighbors = new ArrayList<>();
    public Node(int val) { this.val = val; }
}

class Solution {
    private final Map<Node, Node> clones = new HashMap<>();

    public Node cloneGraph(Node node) {
        if (node == null) return null;
        if (clones.containsKey(node)) return clones.get(node);
        Node c = new Node(node.val);
        clones.put(node, c);
        for (Node nb : node.neighbors) c.neighbors.add(cloneGraph(nb));
        return c;
    }
}
```
**Complexity:** Time O(V + E) · Space O(V)

#### Approach 2: BFS + HashMap
```java
class Node {
    public int val;
    public List<Node> neighbors = new ArrayList<>();
    public Node(int val) { this.val = val; }
}

class Solution {
    public Node cloneGraph(Node node) {
        if (node == null) return null;
        Map<Node, Node> clones = new HashMap<>();
        clones.put(node, new Node(node.val));
        Deque<Node> q = new ArrayDeque<>();
        q.offer(node);
        while (!q.isEmpty()) {
            Node cur = q.poll();
            for (Node nb : cur.neighbors) {
                if (!clones.containsKey(nb)) {
                    clones.put(nb, new Node(nb.val));
                    q.offer(nb);
                }
                clones.get(cur).neighbors.add(clones.get(nb));
            }
        }
        return clones.get(node);
    }
}
```
**Complexity:** Time O(V + E) · Space O(V)

> LeetCode provides the `Node` class for this problem. It's repeated above only so each snippet compiles on its own.

---

### 695. Max Area of Island
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/max-area-of-island/)

**Problem:** In a binary grid where `1` is land, return the size of the largest island (4-directionally connected), or `0` if there is no land.

#### Approach 1: DFS Returning Area
```java
class Solution {
    public int maxAreaOfIsland(int[][] grid) {
        int best = 0;
        for (int r = 0; r < grid.length; r++)
            for (int c = 0; c < grid[0].length; c++)
                best = Math.max(best, dfs(grid, r, c));
        return best;
    }

    private int dfs(int[][] g, int r, int c) {
        if (r < 0 || c < 0 || r >= g.length || c >= g[0].length || g[r][c] == 0) return 0;
        g[r][c] = 0;
        return 1 + dfs(g, r + 1, c) + dfs(g, r - 1, c) + dfs(g, r, c + 1) + dfs(g, r, c - 1);
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Iterative DFS with a Stack
```java
class Solution {
    public int maxAreaOfIsland(int[][] grid) {
        int m = grid.length, n = grid[0].length, best = 0;
        int[][] dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        for (int r = 0; r < m; r++) {
            for (int c = 0; c < n; c++) {
                if (grid[r][c] == 0) continue;
                int area = 0;
                grid[r][c] = 0;
                Deque<int[]> stack = new ArrayDeque<>();
                stack.push(new int[]{r, c});
                while (!stack.isEmpty()) {
                    int[] p = stack.pop();
                    area++;
                    for (int[] d : dirs) {
                        int nr = p[0] + d[0], nc = p[1] + d[1];
                        if (nr >= 0 && nc >= 0 && nr < m && nc < n && grid[nr][nc] == 1) {
                            grid[nr][nc] = 0;
                            stack.push(new int[]{nr, nc});
                        }
                    }
                }
                best = Math.max(best, area);
            }
        }
        return best;
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
```java
class Solution {
    public List<List<Integer>> pacificAtlantic(int[][] heights) {
        int m = heights.length, n = heights[0].length;
        boolean[][] pac = new boolean[m][n], atl = new boolean[m][n];
        for (int c = 0; c < n; c++) {
            dfs(heights, 0, c, pac, 0);
            dfs(heights, m - 1, c, atl, 0);
        }
        for (int r = 0; r < m; r++) {
            dfs(heights, r, 0, pac, 0);
            dfs(heights, r, n - 1, atl, 0);
        }
        List<List<Integer>> res = new ArrayList<>();
        for (int r = 0; r < m; r++)
            for (int c = 0; c < n; c++)
                if (pac[r][c] && atl[r][c]) res.add(Arrays.asList(r, c));
        return res;
    }

    private void dfs(int[][] h, int r, int c, boolean[][] seen, int prev) {
        if (r < 0 || c < 0 || r >= h.length || c >= h[0].length || seen[r][c] || h[r][c] < prev) return;
        seen[r][c] = true;
        dfs(h, r + 1, c, seen, h[r][c]);
        dfs(h, r - 1, c, seen, h[r][c]);
        dfs(h, r, c + 1, seen, h[r][c]);
        dfs(h, r, c - 1, seen, h[r][c]);
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Multi-Source BFS from Each Ocean
```java
class Solution {
    private static final int[][] DIRS = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};

    public List<List<Integer>> pacificAtlantic(int[][] heights) {
        int m = heights.length, n = heights[0].length;
        Deque<int[]> pq = new ArrayDeque<>(), aq = new ArrayDeque<>();
        boolean[][] pac = new boolean[m][n], atl = new boolean[m][n];
        for (int r = 0; r < m; r++) {
            pac[r][0] = true; pq.offer(new int[]{r, 0});
            atl[r][n - 1] = true; aq.offer(new int[]{r, n - 1});
        }
        for (int c = 0; c < n; c++) {
            if (!pac[0][c]) { pac[0][c] = true; pq.offer(new int[]{0, c}); }
            if (!atl[m - 1][c]) { atl[m - 1][c] = true; aq.offer(new int[]{m - 1, c}); }
        }
        bfs(heights, pq, pac);
        bfs(heights, aq, atl);
        List<List<Integer>> res = new ArrayList<>();
        for (int r = 0; r < m; r++)
            for (int c = 0; c < n; c++)
                if (pac[r][c] && atl[r][c]) res.add(Arrays.asList(r, c));
        return res;
    }

    private void bfs(int[][] h, Deque<int[]> q, boolean[][] seen) {
        while (!q.isEmpty()) {
            int[] p = q.poll();
            for (int[] d : DIRS) {
                int nr = p[0] + d[0], nc = p[1] + d[1];
                if (nr < 0 || nc < 0 || nr >= h.length || nc >= h[0].length || seen[nr][nc]) continue;
                if (h[nr][nc] < h[p[0]][p[1]]) continue;
                seen[nr][nc] = true;
                q.offer(new int[]{nr, nc});
            }
        }
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
```java
class Solution {
    public void solve(char[][] board) {
        int m = board.length, n = board[0].length;
        for (int r = 0; r < m; r++) {
            mark(board, r, 0);
            mark(board, r, n - 1);
        }
        for (int c = 0; c < n; c++) {
            mark(board, 0, c);
            mark(board, m - 1, c);
        }
        for (int r = 0; r < m; r++)
            for (int c = 0; c < n; c++) {
                if (board[r][c] == 'O') board[r][c] = 'X';
                else if (board[r][c] == 'T') board[r][c] = 'O';
            }
    }

    private void mark(char[][] b, int r, int c) {
        if (r < 0 || c < 0 || r >= b.length || c >= b[0].length || b[r][c] != 'O') return;
        b[r][c] = 'T'; // safe: connected to the border
        mark(b, r + 1, c);
        mark(b, r - 1, c);
        mark(b, r, c + 1);
        mark(b, r, c - 1);
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n) recursion

#### Approach 2: Union-Find with a Virtual Border Node
```java
class Solution {
    private int[] parent;

    private int find(int x) {
        while (parent[x] != x) x = parent[x] = parent[parent[x]];
        return x;
    }

    private void union(int a, int b) { parent[find(a)] = find(b); }

    public void solve(char[][] board) {
        int m = board.length, n = board[0].length, border = m * n;
        parent = new int[m * n + 1];
        for (int i = 0; i <= m * n; i++) parent[i] = i;
        for (int r = 0; r < m; r++)
            for (int c = 0; c < n; c++) {
                if (board[r][c] != 'O') continue;
                int id = r * n + c;
                if (r == 0 || c == 0 || r == m - 1 || c == n - 1) union(id, border);
                if (r + 1 < m && board[r + 1][c] == 'O') union(id, id + n);
                if (c + 1 < n && board[r][c + 1] == 'O') union(id, id + 1);
            }
        int root = find(border);
        for (int r = 0; r < m; r++)
            for (int c = 0; c < n; c++)
                if (board[r][c] == 'O' && find(r * n + c) != root) board[r][c] = 'X';
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
```java
class Solution {
    public int orangesRotting(int[][] grid) {
        int m = grid.length, n = grid[0].length, fresh = 0;
        Deque<int[]> q = new ArrayDeque<>();
        for (int r = 0; r < m; r++)
            for (int c = 0; c < n; c++) {
                if (grid[r][c] == 2) q.offer(new int[]{r, c});
                else if (grid[r][c] == 1) fresh++;
            }
        int minutes = 0;
        int[][] dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        while (!q.isEmpty() && fresh > 0) {
            minutes++;
            for (int sz = q.size(); sz > 0; sz--) {
                int[] cell = q.poll();
                for (int[] d : dirs) {
                    int nr = cell[0] + d[0], nc = cell[1] + d[1];
                    if (nr >= 0 && nc >= 0 && nr < m && nc < n && grid[nr][nc] == 1) {
                        grid[nr][nc] = 2;
                        fresh--;
                        q.offer(new int[]{nr, nc});
                    }
                }
            }
        }
        return fresh > 0 ? -1 : minutes;
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
```java
class Solution {
    public void wallsAndGates(int[][] rooms) {
        final int INF = Integer.MAX_VALUE;
        int m = rooms.length, n = rooms[0].length;
        Deque<int[]> q = new ArrayDeque<>();
        for (int r = 0; r < m; r++)
            for (int c = 0; c < n; c++)
                if (rooms[r][c] == 0) q.offer(new int[]{r, c});
        int[][] dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        while (!q.isEmpty()) {
            int[] cell = q.poll();
            for (int[] d : dirs) {
                int nr = cell[0] + d[0], nc = cell[1] + d[1];
                if (nr >= 0 && nc >= 0 && nr < m && nc < n && rooms[nr][nc] == INF) {
                    rooms[nr][nc] = rooms[cell[0]][cell[1]] + 1;
                    q.offer(new int[]{nr, nc});
                }
            }
        }
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: DFS from Each Gate (slower)
```java
class Solution {
    public void wallsAndGates(int[][] rooms) {
        for (int r = 0; r < rooms.length; r++)
            for (int c = 0; c < rooms[0].length; c++)
                if (rooms[r][c] == 0) dfs(rooms, r, c, 0);
    }

    private void dfs(int[][] rooms, int r, int c, int d) {
        if (r < 0 || c < 0 || r >= rooms.length || c >= rooms[0].length || rooms[r][c] < d) return; // a wall or an already shorter distance
        rooms[r][c] = d;
        dfs(rooms, r + 1, c, d + 1);
        dfs(rooms, r - 1, c, d + 1);
        dfs(rooms, r, c + 1, d + 1);
        dfs(rooms, r, c - 1, d + 1);
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
```java
class Solution {
    public boolean canFinish(int numCourses, int[][] prerequisites) {
        List<List<Integer>> adj = new ArrayList<>();
        for (int i = 0; i < numCourses; i++) adj.add(new ArrayList<>());
        int[] indeg = new int[numCourses];
        for (int[] p : prerequisites) {
            adj.get(p[1]).add(p[0]);
            indeg[p[0]]++;
        }
        Deque<Integer> q = new ArrayDeque<>();
        for (int i = 0; i < numCourses; i++) if (indeg[i] == 0) q.offer(i);
        int taken = 0;
        while (!q.isEmpty()) {
            int c = q.poll();
            taken++;
            for (int nxt : adj.get(c)) if (--indeg[nxt] == 0) q.offer(nxt);
        }
        return taken == numCourses;
    }
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

#### Approach 2: DFS Three-Color Cycle Detection
```java
class Solution {
    public boolean canFinish(int numCourses, int[][] prerequisites) {
        List<List<Integer>> adj = new ArrayList<>();
        for (int i = 0; i < numCourses; i++) adj.add(new ArrayList<>());
        for (int[] p : prerequisites) adj.get(p[1]).add(p[0]);
        int[] state = new int[numCourses]; // 0 = unvisited, 1 = visiting, 2 = done
        for (int i = 0; i < numCourses; i++) if (hasCycle(i, adj, state)) return false;
        return true;
    }

    private boolean hasCycle(int u, List<List<Integer>> adj, int[] state) {
        if (state[u] == 1) return true;
        if (state[u] == 2) return false;
        state[u] = 1;
        for (int v : adj.get(u)) if (hasCycle(v, adj, state)) return true;
        state[u] = 2;
        return false;
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
```java
class Solution {
    public int[] findOrder(int numCourses, int[][] prerequisites) {
        List<List<Integer>> adj = new ArrayList<>();
        for (int i = 0; i < numCourses; i++) adj.add(new ArrayList<>());
        int[] indeg = new int[numCourses];
        for (int[] p : prerequisites) {
            adj.get(p[1]).add(p[0]);
            indeg[p[0]]++;
        }
        Deque<Integer> q = new ArrayDeque<>();
        for (int i = 0; i < numCourses; i++) if (indeg[i] == 0) q.offer(i);
        int[] order = new int[numCourses];
        int idx = 0;
        while (!q.isEmpty()) {
            int c = q.poll();
            order[idx++] = c;
            for (int nxt : adj.get(c)) if (--indeg[nxt] == 0) q.offer(nxt);
        }
        return idx == numCourses ? order : new int[0];
    }
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

#### Approach 2: DFS Post-Order
```java
class Solution {
    private List<List<Integer>> adj;
    private int[] state, order;
    private int idx;

    public int[] findOrder(int numCourses, int[][] prerequisites) {
        adj = new ArrayList<>();
        for (int i = 0; i < numCourses; i++) adj.add(new ArrayList<>());
        for (int[] p : prerequisites) adj.get(p[0]).add(p[1]); // course → its prerequisites
        state = new int[numCourses];
        order = new int[numCourses];
        for (int i = 0; i < numCourses; i++) if (!dfs(i)) return new int[0];
        return order;
    }

    private boolean dfs(int u) { // returns false when a cycle is found
        if (state[u] == 1) return false;
        if (state[u] == 2) return true;
        state[u] = 1;
        for (int v : adj.get(u)) if (!dfs(v)) return false;
        state[u] = 2;
        order[idx++] = u; // prerequisites are recorded first
        return true;
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
```java
class Solution {
    private int[] parent, rank;

    public int[] findRedundantConnection(int[][] edges) {
        int n = edges.length;
        parent = new int[n + 1];
        rank = new int[n + 1];
        for (int i = 0; i <= n; i++) parent[i] = i;
        for (int[] e : edges) if (!union(e[0], e[1])) return e;
        return new int[0];
    }

    private int find(int x) {
        if (parent[x] != x) parent[x] = find(parent[x]);
        return parent[x];
    }

    private boolean union(int a, int b) {
        int ra = find(a), rb = find(b);
        if (ra == rb) return false; // already connected, so this edge closes a cycle
        if (rank[ra] < rank[rb]) { int t = ra; ra = rb; rb = t; }
        parent[rb] = ra;
        if (rank[ra] == rank[rb]) rank[ra]++;
        return true;
    }
}
```
**Complexity:** Time O(n · α(n)) · Space O(n)

#### Approach 2: DFS per Edge (check whether already connected)
```java
class Solution {
    public int[] findRedundantConnection(int[][] edges) {
        Map<Integer, List<Integer>> adj = new HashMap<>();
        for (int[] e : edges) {
            if (connected(adj, e[0], e[1], new HashSet<>())) return e;
            adj.computeIfAbsent(e[0], k -> new ArrayList<>()).add(e[1]);
            adj.computeIfAbsent(e[1], k -> new ArrayList<>()).add(e[0]);
        }
        return new int[0];
    }

    private boolean connected(Map<Integer, List<Integer>> adj, int u, int target, Set<Integer> seen) {
        if (u == target) return true;
        seen.add(u);
        for (int v : adj.getOrDefault(u, Collections.emptyList()))
            if (!seen.contains(v) && connected(adj, v, target, seen)) return true;
        return false;
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
```java
class Solution {
    public int countComponents(int n, int[][] edges) {
        int[] parent = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
        int comps = n;
        for (int[] e : edges) {
            int a = find(parent, e[0]), b = find(parent, e[1]);
            if (a != b) {
                parent[a] = b;
                comps--;
            }
        }
        return comps;
    }

    private int find(int[] p, int x) {
        while (p[x] != x) x = p[x] = p[p[x]];
        return x;
    }
}
```
**Complexity:** Time O(V + E · α(V)) · Space O(V)

#### Approach 2: DFS
```java
class Solution {
    public int countComponents(int n, int[][] edges) {
        List<List<Integer>> adj = new ArrayList<>();
        for (int i = 0; i < n; i++) adj.add(new ArrayList<>());
        for (int[] e : edges) {
            adj.get(e[0]).add(e[1]);
            adj.get(e[1]).add(e[0]);
        }
        boolean[] seen = new boolean[n];
        int comps = 0;
        for (int i = 0; i < n; i++)
            if (!seen[i]) {
                comps++;
                dfs(i, adj, seen);
            }
        return comps;
    }

    private void dfs(int u, List<List<Integer>> adj, boolean[] seen) {
        seen[u] = true;
        for (int v : adj.get(u)) if (!seen[v]) dfs(v, adj, seen);
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
```java
class Solution {
    public boolean validTree(int n, int[][] edges) {
        if (edges.length != n - 1) return false;
        int[] parent = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
        for (int[] e : edges) {
            int a = find(parent, e[0]), b = find(parent, e[1]);
            if (a == b) return false; // cycle
            parent[a] = b;
        }
        return true;
    }

    private int find(int[] p, int x) {
        while (p[x] != x) x = p[x] = p[p[x]];
        return x;
    }
}
```
**Complexity:** Time O(n · α(n)) · Space O(n)

#### Approach 2: Edge Count + BFS Connectivity
```java
class Solution {
    public boolean validTree(int n, int[][] edges) {
        if (edges.length != n - 1) return false;
        List<List<Integer>> adj = new ArrayList<>();
        for (int i = 0; i < n; i++) adj.add(new ArrayList<>());
        for (int[] e : edges) {
            adj.get(e[0]).add(e[1]);
            adj.get(e[1]).add(e[0]);
        }
        boolean[] seen = new boolean[n];
        Deque<Integer> q = new ArrayDeque<>();
        q.offer(0);
        seen[0] = true;
        int visited = 0;
        while (!q.isEmpty()) {
            int u = q.poll();
            visited++;
            for (int v : adj.get(u))
                if (!seen[v]) {
                    seen[v] = true;
                    q.offer(v);
                }
        }
        return visited == n;
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
```java
class Solution {
    public int ladderLength(String beginWord, String endWord, List<String> wordList) {
        Set<String> dict = new HashSet<>(wordList);
        if (!dict.contains(endWord)) return 0;
        Deque<String> q = new ArrayDeque<>();
        q.offer(beginWord);
        dict.remove(beginWord);
        int steps = 1;
        while (!q.isEmpty()) {
            for (int sz = q.size(); sz > 0; sz--) {
                char[] w = q.poll().toCharArray();
                for (int i = 0; i < w.length; i++) {
                    char orig = w[i];
                    for (char c = 'a'; c <= 'z'; c++) {
                        if (c == orig) continue;
                        w[i] = c;
                        String s = new String(w);
                        if (s.equals(endWord)) return steps + 1;
                        if (dict.remove(s)) q.offer(s); // remove() also marks it visited
                    }
                    w[i] = orig;
                }
            }
            steps++;
        }
        return 0;
    }
}
```
**Complexity:** Time O(N · L · 26 · L), with string building · Space O(N · L)

#### Approach 2: Bidirectional BFS (fastest in practice)
```java
class Solution {
    public int ladderLength(String beginWord, String endWord, List<String> wordList) {
        Set<String> dict = new HashSet<>(wordList);
        if (!dict.contains(endWord)) return 0;
        Set<String> front = new HashSet<>(), back = new HashSet<>();
        front.add(beginWord);
        back.add(endWord);
        dict.remove(endWord);
        dict.remove(beginWord);
        int steps = 1;
        while (!front.isEmpty() && !back.isEmpty()) {
            if (front.size() > back.size()) { Set<String> t = front; front = back; back = t; } // always expand the smaller side
            Set<String> next = new HashSet<>();
            for (String word : front) {
                char[] w = word.toCharArray();
                for (int i = 0; i < w.length; i++) {
                    char orig = w[i];
                    for (char c = 'a'; c <= 'z'; c++) {
                        w[i] = c;
                        String s = new String(w);
                        if (back.contains(s)) return steps + 1;
                        if (dict.remove(s)) next.add(s);
                    }
                    w[i] = orig;
                }
            }
            front = next;
            steps++;
        }
        return 0;
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

#### Approach 1: Hierholzer's Algorithm with a PriorityQueue (Optimal)
```java
class Solution {
    private final Map<String, PriorityQueue<String>> adj = new HashMap<>();
    private final LinkedList<String> route = new LinkedList<>();

    public List<String> findItinerary(List<List<String>> tickets) {
        for (List<String> t : tickets)
            adj.computeIfAbsent(t.get(0), k -> new PriorityQueue<>()).offer(t.get(1));
        visit("JFK");
        return route;
    }

    private void visit(String a) {
        PriorityQueue<String> dests = adj.get(a);
        while (dests != null && !dests.isEmpty()) visit(dests.poll()); // smallest destination first
        route.addFirst(a); // post-order, prepended, so the route comes out already reversed
    }
}
```
**Complexity:** Time O(E log E) · Space O(E)

#### Approach 2: Backtracking DFS (simpler idea, worse worst case)
```java
class Solution {
    public List<String> findItinerary(List<List<String>> tickets) {
        Map<String, List<String>> adj = new HashMap<>();
        for (List<String> t : tickets) adj.computeIfAbsent(t.get(0), k -> new ArrayList<>()).add(t.get(1));
        for (List<String> d : adj.values()) Collections.sort(d);
        List<String> res = new ArrayList<>();
        res.add("JFK");
        dfs("JFK", adj, res, tickets.size() + 1);
        return res;
    }

    private boolean dfs(String src, Map<String, List<String>> adj, List<String> res, int total) {
        if (res.size() == total) return true;
        List<String> dests = adj.get(src);
        if (dests == null) return false;
        for (int i = 0; i < dests.size(); i++) {
            String v = dests.get(i);
            if (v == null) continue; // ticket already used
            dests.set(i, null);
            res.add(v);
            if (dfs(v, adj, res, total)) return true;
            res.remove(res.size() - 1);
            dests.set(i, v);
        }
        return false;
    }
}
```
**Complexity:** Time O(V · E) in practice, exponential in the worst case · Space O(E)

---

### 1584. Min Cost to Connect All Points
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/min-cost-to-connect-all-points/)

**Problem:** Given points on a 2D plane, connecting two points costs their Manhattan distance. Return the minimum total cost to connect every point (a minimum spanning tree).

**Example:** `[[0,0],[2,2],[3,10],[5,2],[7,0]]` → `20`

#### Approach 1: Prim's with an O(n²) Array (Optimal for dense graphs)
```java
class Solution {
    public int minCostConnectPoints(int[][] points) {
        int n = points.length, total = 0;
        int[] dist = new int[n];
        boolean[] inTree = new boolean[n];
        Arrays.fill(dist, Integer.MAX_VALUE);
        dist[0] = 0;
        for (int k = 0; k < n; k++) {
            int u = -1;
            for (int i = 0; i < n; i++)
                if (!inTree[i] && (u == -1 || dist[i] < dist[u])) u = i;
            inTree[u] = true;
            total += dist[u];
            for (int v = 0; v < n; v++) {
                if (inTree[v]) continue;
                int d = Math.abs(points[u][0] - points[v][0]) + Math.abs(points[u][1] - points[v][1]);
                dist[v] = Math.min(dist[v], d);
            }
        }
        return total;
    }
}
```
**Complexity:** Time O(n²) · Space O(n)

#### Approach 2: Kruskal's with Union-Find
```java
class Solution {
    public int minCostConnectPoints(int[][] points) {
        int n = points.length;
        List<int[]> edges = new ArrayList<>(); // {weight, u, v}
        for (int i = 0; i < n; i++)
            for (int j = i + 1; j < n; j++)
                edges.add(new int[]{Math.abs(points[i][0] - points[j][0]) + Math.abs(points[i][1] - points[j][1]), i, j});
        edges.sort((a, b) -> Integer.compare(a[0], b[0]));
        int[] parent = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
        int total = 0, used = 0;
        for (int[] e : edges) {
            int a = find(parent, e[1]), b = find(parent, e[2]);
            if (a == b) continue;
            parent[a] = b;
            total += e[0];
            if (++used == n - 1) break;
        }
        return total;
    }

    private int find(int[] p, int x) {
        while (p[x] != x) x = p[x] = p[p[x]];
        return x;
    }
}
```
**Complexity:** Time O(n² log n) · Space O(n²)

#### Approach 3: Prim's with a Min-Heap (lazy)
```java
class Solution {
    public int minCostConnectPoints(int[][] points) {
        int n = points.length, total = 0, added = 0;
        boolean[] inTree = new boolean[n];
        PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> Integer.compare(a[0], b[0])); // {cost, node}
        pq.offer(new int[]{0, 0});
        while (added < n) {
            int[] top = pq.poll();
            int u = top[1];
            if (inTree[u]) continue;
            inTree[u] = true;
            total += top[0];
            added++;
            for (int v = 0; v < n; v++)
                if (!inTree[v])
                    pq.offer(new int[]{Math.abs(points[u][0] - points[v][0]) + Math.abs(points[u][1] - points[v][1]), v});
        }
        return total;
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
```java
class Solution {
    public int networkDelayTime(int[][] times, int n, int k) {
        List<List<int[]>> adj = new ArrayList<>();
        for (int i = 0; i <= n; i++) adj.add(new ArrayList<>());
        for (int[] t : times) adj.get(t[0]).add(new int[]{t[1], t[2]});
        int[] dist = new int[n + 1];
        Arrays.fill(dist, Integer.MAX_VALUE);
        dist[k] = 0;
        PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> Integer.compare(a[1], b[1])); // {node, dist}
        pq.offer(new int[]{k, 0});
        while (!pq.isEmpty()) {
            int[] cur = pq.poll();
            if (cur[1] > dist[cur[0]]) continue; // stale entry
            for (int[] e : adj.get(cur[0])) {
                int nd = cur[1] + e[1];
                if (nd < dist[e[0]]) {
                    dist[e[0]] = nd;
                    pq.offer(new int[]{e[0], nd});
                }
            }
        }
        int ans = 0;
        for (int i = 1; i <= n; i++) {
            if (dist[i] == Integer.MAX_VALUE) return -1;
            ans = Math.max(ans, dist[i]);
        }
        return ans;
    }
}
```
**Complexity:** Time O(E log V) · Space O(V + E)

#### Approach 2: Bellman-Ford
```java
class Solution {
    public int networkDelayTime(int[][] times, int n, int k) {
        final int INF = Integer.MAX_VALUE / 2;
        int[] dist = new int[n + 1];
        Arrays.fill(dist, INF);
        dist[k] = 0;
        for (int i = 1; i < n; i++) {
            boolean changed = false;
            for (int[] t : times)
                if (dist[t[0]] + t[2] < dist[t[1]]) {
                    dist[t[1]] = dist[t[0]] + t[2];
                    changed = true;
                }
            if (!changed) break;
        }
        int ans = 0;
        for (int i = 1; i <= n; i++) {
            if (dist[i] == INF) return -1;
            ans = Math.max(ans, dist[i]);
        }
        return ans;
    }
}
```
**Complexity:** Time O(V · E) · Space O(V)

#### Approach 3: Floyd–Warshall (n ≤ 100)
```java
class Solution {
    public int networkDelayTime(int[][] times, int n, int k) {
        final int INF = Integer.MAX_VALUE / 2;
        int[][] d = new int[n + 1][n + 1];
        for (int[] row : d) Arrays.fill(row, INF);
        for (int i = 1; i <= n; i++) d[i][i] = 0;
        for (int[] t : times) d[t[0]][t[1]] = t[2];
        for (int m = 1; m <= n; m++)
            for (int i = 1; i <= n; i++)
                for (int j = 1; j <= n; j++)
                    if (d[i][m] + d[m][j] < d[i][j]) d[i][j] = d[i][m] + d[m][j];
        int ans = 0;
        for (int i = 1; i <= n; i++) {
            if (d[k][i] == INF) return -1;
            ans = Math.max(ans, d[k][i]);
        }
        return ans;
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
```java
class Solution {
    public int swimInWater(int[][] grid) {
        int n = grid.length;
        boolean[][] seen = new boolean[n][n];
        PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> Integer.compare(a[0], b[0])); // {time, r, c}
        pq.offer(new int[]{grid[0][0], 0, 0});
        seen[0][0] = true;
        int[][] dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        while (true) {
            int[] cur = pq.poll();
            if (cur[1] == n - 1 && cur[2] == n - 1) return cur[0];
            for (int[] d : dirs) {
                int nr = cur[1] + d[0], nc = cur[2] + d[1];
                if (nr >= 0 && nc >= 0 && nr < n && nc < n && !seen[nr][nc]) {
                    seen[nr][nc] = true;
                    pq.offer(new int[]{Math.max(cur[0], grid[nr][nc]), nr, nc});
                }
            }
        }
    }
}
```
**Complexity:** Time O(n² log n) · Space O(n²)

#### Approach 2: Binary Search on Time + BFS
```java
class Solution {
    public int swimInWater(int[][] grid) {
        int n = grid.length, lo = grid[0][0], hi = n * n - 1;
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (canReach(grid, mid)) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

    private boolean canReach(int[][] g, int t) {
        int n = g.length;
        if (g[0][0] > t) return false;
        boolean[][] seen = new boolean[n][n];
        Deque<int[]> q = new ArrayDeque<>();
        q.offer(new int[]{0, 0});
        seen[0][0] = true;
        int[][] dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        while (!q.isEmpty()) {
            int[] p = q.poll();
            if (p[0] == n - 1 && p[1] == n - 1) return true;
            for (int[] d : dirs) {
                int nr = p[0] + d[0], nc = p[1] + d[1];
                if (nr >= 0 && nc >= 0 && nr < n && nc < n && !seen[nr][nc] && g[nr][nc] <= t) {
                    seen[nr][nc] = true;
                    q.offer(new int[]{nr, nc});
                }
            }
        }
        return false;
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
```java
class Solution {
    public String alienOrder(String[] words) {
        Map<Character, Set<Character>> adj = new HashMap<>();
        Map<Character, Integer> indeg = new HashMap<>();
        for (String w : words)
            for (char c : w.toCharArray()) {
                adj.putIfAbsent(c, new HashSet<>());
                indeg.putIfAbsent(c, 0);
            }
        for (int i = 0; i + 1 < words.length; i++) {
            String a = words[i], b = words[i + 1];
            int minLen = Math.min(a.length(), b.length());
            if (a.length() > b.length() && a.startsWith(b)) return ""; // a word appears before its own prefix
            for (int j = 0; j < minLen; j++) {
                char x = a.charAt(j), y = b.charAt(j);
                if (x != y) {
                    if (adj.get(x).add(y)) indeg.merge(y, 1, Integer::sum);
                    break;
                }
            }
        }
        Deque<Character> q = new ArrayDeque<>();
        for (Map.Entry<Character, Integer> e : indeg.entrySet()) if (e.getValue() == 0) q.offer(e.getKey());
        StringBuilder sb = new StringBuilder();
        while (!q.isEmpty()) {
            char c = q.poll();
            sb.append(c);
            for (char nxt : adj.get(c)) if (indeg.merge(nxt, -1, Integer::sum) == 0) q.offer(nxt);
        }
        return sb.length() == indeg.size() ? sb.toString() : ""; // shorter means there was a cycle
    }
}
```
**Complexity:** Time O(C), where C is the total number of characters · Space O(U + min(U², N)) for U unique letters

#### Approach 2: DFS Post-Order with Cycle Detection
```java
class Solution {
    private final Map<Character, List<Character>> adj = new HashMap<>();
    private final Map<Character, Integer> state = new HashMap<>(); // 1 = visiting, 2 = done
    private final StringBuilder out = new StringBuilder();

    public String alienOrder(String[] words) {
        for (String w : words) for (char c : w.toCharArray()) adj.putIfAbsent(c, new ArrayList<>());
        for (int i = 0; i + 1 < words.length; i++) {
            String a = words[i], b = words[i + 1];
            if (a.length() > b.length() && a.startsWith(b)) return "";
            for (int j = 0; j < Math.min(a.length(), b.length()); j++)
                if (a.charAt(j) != b.charAt(j)) {
                    adj.get(a.charAt(j)).add(b.charAt(j));
                    break;
                }
        }
        for (char c : adj.keySet()) if (!dfs(c)) return "";
        return out.reverse().toString();
    }

    private boolean dfs(char c) {
        int s = state.getOrDefault(c, 0);
        if (s == 1) return false; // cycle
        if (s == 2) return true;
        state.put(c, 1);
        for (char nx : adj.get(c)) if (!dfs(nx)) return false;
        state.put(c, 2);
        out.append(c);
        return true;
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
```java
class Solution {
    public int findCheapestPrice(int n, int[][] flights, int src, int dst, int k) {
        final int INF = Integer.MAX_VALUE;
        int[] prices = new int[n];
        Arrays.fill(prices, INF);
        prices[src] = 0;
        for (int i = 0; i <= k; i++) {
            int[] tmp = prices.clone(); // relax from the previous round only
            for (int[] f : flights) {
                if (prices[f[0]] == INF) continue;
                tmp[f[1]] = Math.min(tmp[f[1]], prices[f[0]] + f[2]);
            }
            prices = tmp;
        }
        return prices[dst] == INF ? -1 : prices[dst];
    }
}
```
**Complexity:** Time O(k · E) · Space O(n)

#### Approach 2: BFS by Number of Stops
```java
class Solution {
    public int findCheapestPrice(int n, int[][] flights, int src, int dst, int k) {
        List<List<int[]>> adj = new ArrayList<>();
        for (int i = 0; i < n; i++) adj.add(new ArrayList<>());
        for (int[] f : flights) adj.get(f[0]).add(new int[]{f[1], f[2]});
        int[] best = new int[n];
        Arrays.fill(best, Integer.MAX_VALUE);
        best[src] = 0;
        Deque<int[]> q = new ArrayDeque<>(); // {node, cost}
        q.offer(new int[]{src, 0});
        for (int stops = 0; stops <= k && !q.isEmpty(); stops++) {
            for (int sz = q.size(); sz > 0; sz--) {
                int[] cur = q.poll();
                for (int[] e : adj.get(cur[0])) {
                    int nc = cur[1] + e[1];
                    if (nc < best[e[0]]) {
                        best[e[0]] = nc;
                        q.offer(new int[]{e[0], nc});
                    }
                }
            }
        }
        return best[dst] == Integer.MAX_VALUE ? -1 : best[dst];
    }
}
```
**Complexity:** Time O(k · E) · Space O(n + E)

---
## 13. 1-D Dynamic Programming

> **Progression for each problem:** recursion → memoization (top-down) → tabulation (bottom-up) → space-optimized, where that helps.

### 70. Climbing Stairs
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/climbing-stairs/)

**Problem:** You can climb 1 or 2 steps at a time. How many distinct ways are there to reach the top of `n` stairs?

**Example:** `n = 3` → `3` (1+1+1, 1+2, 2+1)

**Key insight:** `ways(n) = ways(n-1) + ways(n-2)`, the Fibonacci recurrence.

#### Approach 1: Memoized Recursion
```java
class Solution {
    public int climbStairs(int n) {
        return f(n, new int[n + 1]);
    }

    private int f(int i, int[] memo) {
        if (i <= 2) return i;
        if (memo[i] == 0) memo[i] = f(i - 1, memo) + f(i - 2, memo);
        return memo[i];
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Bottom-Up, O(1) Space (Optimal)
```java
class Solution {
    public int climbStairs(int n) {
        int a = 1, b = 1; // ways(0), ways(1)
        for (int i = 2; i <= n; i++) {
            int c = a + b;
            a = b;
            b = c;
        }
        return b;
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 3: Matrix Exponentiation
```java
class Solution {
    public int climbStairs(int n) {
        long[][] res = {{1, 0}, {0, 1}}, base = {{1, 1}, {1, 0}};
        for (int p = n; p > 0; p >>= 1) {
            if ((p & 1) == 1) res = mul(res, base);
            base = mul(base, base);
        }
        return (int) res[0][0]; // F(n+1)
    }

    private long[][] mul(long[][] x, long[][] y) {
        return new long[][]{
            {x[0][0] * y[0][0] + x[0][1] * y[1][0], x[0][0] * y[0][1] + x[0][1] * y[1][1]},
            {x[1][0] * y[0][0] + x[1][1] * y[1][0], x[1][0] * y[0][1] + x[1][1] * y[1][1]}
        };
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
```java
class Solution {
    public int minCostClimbingStairs(int[] cost) {
        int n = cost.length;
        int[] dp = new int[n + 1]; // dp[i] = min cost to reach stair i
        for (int i = 2; i <= n; i++) dp[i] = Math.min(dp[i - 1] + cost[i - 1], dp[i - 2] + cost[i - 2]);
        return dp[n];
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: O(1) Space (Optimal)
```java
class Solution {
    public int minCostClimbingStairs(int[] cost) {
        int a = 0, b = 0;
        for (int i = 2; i <= cost.length; i++) {
            int c = Math.min(b + cost[i - 1], a + cost[i - 2]);
            a = b;
            b = c;
        }
        return b;
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
```java
class Solution {
    public int rob(int[] nums) {
        int[] memo = new int[nums.length];
        Arrays.fill(memo, -1);
        return f(nums, nums.length - 1, memo);
    }

    private int f(int[] nums, int i, int[] memo) {
        if (i < 0) return 0;
        if (memo[i] == -1) memo[i] = Math.max(f(nums, i - 1, memo), f(nums, i - 2, memo) + nums[i]);
        return memo[i];
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: O(1) Space (Optimal)
```java
class Solution {
    public int rob(int[] nums) {
        int prev2 = 0, prev1 = 0;
        for (int n : nums) {
            int cur = Math.max(prev1, prev2 + n);
            prev2 = prev1;
            prev1 = cur;
        }
        return prev1;
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
```java
class Solution {
    public int rob(int[] nums) {
        int n = nums.length;
        if (n == 1) return nums[0];
        return Math.max(robRange(nums, 0, n - 2), robRange(nums, 1, n - 1));
    }

    private int robRange(int[] nums, int lo, int hi) {
        int a = 0, b = 0;
        for (int i = lo; i <= hi; i++) {
            int c = Math.max(b, a + nums[i]);
            a = b;
            b = c;
        }
        return b;
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
```java
class Solution {
    public String longestPalindrome(String s) {
        int start = 0, best = 0;
        for (int i = 0; i < s.length(); i++) {
            for (int[] lr : new int[][]{{i, i}, {i, i + 1}}) { // odd, then even length
                int l = lr[0], r = lr[1];
                while (l >= 0 && r < s.length() && s.charAt(l) == s.charAt(r)) {
                    l--;
                    r++;
                }
                if (r - l - 1 > best) {
                    best = r - l - 1;
                    start = l + 1;
                }
            }
        }
        return s.substring(start, start + best);
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: DP Table
```java
class Solution {
    public String longestPalindrome(String s) {
        int n = s.length(), start = 0, best = 1;
        boolean[][] dp = new boolean[n][n];
        for (int i = n - 1; i >= 0; i--)
            for (int j = i; j < n; j++)
                if (s.charAt(i) == s.charAt(j) && (j - i < 3 || dp[i + 1][j - 1])) {
                    dp[i][j] = true;
                    if (j - i + 1 > best) {
                        best = j - i + 1;
                        start = i;
                    }
                }
        return s.substring(start, start + best);
    }
}
```
**Complexity:** Time O(n²) · Space O(n²)

#### Approach 3: Manacher's Algorithm (theoretically optimal)
```java
class Solution {
    public String longestPalindrome(String s) {
        // transform "abc" into "^#a#b#c#$" so every palindrome has odd length
        StringBuilder sb = new StringBuilder("^");
        for (char c : s.toCharArray()) sb.append('#').append(c);
        sb.append("#$");
        char[] t = sb.toString().toCharArray();
        int[] p = new int[t.length];
        int center = 0, right = 0;
        for (int i = 1; i < t.length - 1; i++) {
            if (i < right) p[i] = Math.min(right - i, p[2 * center - i]);
            while (t[i + p[i] + 1] == t[i - p[i] - 1]) p[i]++;
            if (i + p[i] > right) {
                center = i;
                right = i + p[i];
            }
        }
        int maxLen = 0, ci = 0;
        for (int i = 0; i < p.length; i++)
            if (p[i] > maxLen) {
                maxLen = p[i];
                ci = i;
            }
        int begin = (ci - maxLen) / 2;
        return s.substring(begin, begin + maxLen);
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
```java
class Solution {
    public int countSubstrings(String s) {
        int count = 0, n = s.length();
        for (int c = 0; c < 2 * n - 1; c++) {
            int l = c / 2, r = c / 2 + c % 2;
            while (l >= 0 && r < n && s.charAt(l) == s.charAt(r)) {
                count++;
                l--;
                r++;
            }
        }
        return count;
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: DP Table
```java
class Solution {
    public int countSubstrings(String s) {
        int n = s.length(), count = 0;
        boolean[][] dp = new boolean[n][n];
        for (int i = n - 1; i >= 0; i--)
            for (int j = i; j < n; j++)
                if (s.charAt(i) == s.charAt(j) && (j - i < 3 || dp[i + 1][j - 1])) {
                    dp[i][j] = true;
                    count++;
                }
        return count;
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
```java
class Solution {
    public int numDecodings(String s) {
        int n = s.length();
        int[] dp = new int[n + 1];
        dp[0] = 1;
        for (int i = 1; i <= n; i++) {
            if (s.charAt(i - 1) != '0') dp[i] += dp[i - 1];
            if (i >= 2) {
                int two = Integer.parseInt(s.substring(i - 2, i));
                if (two >= 10 && two <= 26) dp[i] += dp[i - 2];
            }
        }
        return dp[n];
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: O(1) Space (Optimal)
```java
class Solution {
    public int numDecodings(String s) {
        int prev2 = 0, prev1 = 1; // dp[i-2], dp[i-1]
        for (int i = 0; i < s.length(); i++) {
            int cur = s.charAt(i) != '0' ? prev1 : 0;
            if (i > 0 && (s.charAt(i - 1) == '1' || (s.charAt(i - 1) == '2' && s.charAt(i) <= '6'))) cur += prev2;
            prev2 = prev1;
            prev1 = cur;
        }
        return prev1;
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
```java
class Solution {
    public int coinChange(int[] coins, int amount) {
        int[] dp = new int[amount + 1];
        Arrays.fill(dp, amount + 1); // stands in for infinity
        dp[0] = 0;
        for (int a = 1; a <= amount; a++)
            for (int c : coins)
                if (c <= a) dp[a] = Math.min(dp[a], dp[a - c] + 1);
        return dp[amount] > amount ? -1 : dp[amount];
    }
}
```
**Complexity:** Time O(amount · len(coins)) · Space O(amount)

#### Approach 2: BFS over Amounts (shortest path)
```java
class Solution {
    public int coinChange(int[] coins, int amount) {
        if (amount == 0) return 0;
        boolean[] seen = new boolean[amount + 1];
        Deque<Integer> q = new ArrayDeque<>();
        q.offer(0);
        seen[0] = true;
        for (int steps = 1; !q.isEmpty(); steps++) {
            for (int sz = q.size(); sz > 0; sz--) {
                int cur = q.poll();
                for (int c : coins) {
                    long v = (long) cur + c; // coins can be up to 2^31-1
                    if (v == amount) return steps;
                    if (v < amount && !seen[(int) v]) {
                        seen[(int) v] = true;
                        q.offer((int) v);
                    }
                }
            }
        }
        return -1;
    }
}
```
**Complexity:** Time O(amount · len(coins)) · Space O(amount)

#### Approach 3: Top-Down Memoization
```java
class Solution {
    private int[] memo;

    public int coinChange(int[] coins, int amount) {
        memo = new int[amount + 1];
        Arrays.fill(memo, -2); // -2 = not computed yet
        return f(coins, amount);
    }

    private int f(int[] coins, int a) {
        if (a == 0) return 0;
        if (a < 0) return -1;
        if (memo[a] != -2) return memo[a];
        int best = Integer.MAX_VALUE;
        for (int c : coins) {
            int r = f(coins, a - c);
            if (r >= 0 && r + 1 < best) best = r + 1;
        }
        return memo[a] = (best == Integer.MAX_VALUE) ? -1 : best;
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
```java
class Solution {
    public int maxProduct(int[] nums) {
        int best = nums[0], curMax = nums[0], curMin = nums[0];
        for (int i = 1; i < nums.length; i++) {
            int n = nums[i];
            if (n < 0) { int t = curMax; curMax = curMin; curMin = t; }
            curMax = Math.max(n, curMax * n);
            curMin = Math.min(n, curMin * n);
            best = Math.max(best, curMax);
        }
        return best;
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Prefix and Suffix Products
The best subarray always starts at an edge of the array or of a zero-bounded segment.
```java
class Solution {
    public int maxProduct(int[] nums) {
        int n = nums.length, best = nums[0], pre = 1, suf = 1;
        for (int i = 0; i < n; i++) {
            pre = (pre == 0 ? 1 : pre) * nums[i];
            suf = (suf == 0 ? 1 : suf) * nums[n - 1 - i];
            best = Math.max(best, Math.max(pre, suf));
        }
        return best;
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

#### Approach 1: Bottom-Up DP with a Set (Optimal)
```java
class Solution {
    public boolean wordBreak(String s, List<String> wordDict) {
        Set<String> dict = new HashSet<>(wordDict);
        int maxLen = 0;
        for (String w : wordDict) maxLen = Math.max(maxLen, w.length());
        boolean[] dp = new boolean[s.length() + 1];
        dp[0] = true;
        for (int i = 1; i <= s.length(); i++)
            for (int j = i - 1; j >= 0 && i - j <= maxLen; j--)
                if (dp[j] && dict.contains(s.substring(j, i))) {
                    dp[i] = true;
                    break;
                }
        return dp[s.length()];
    }
}
```
**Complexity:** Time O(n · L), where L is the longest word (substring hashing adds O(L) per check) · Space O(n)

#### Approach 2: DP Iterating over Dictionary Words
```java
class Solution {
    public boolean wordBreak(String s, List<String> wordDict) {
        int n = s.length();
        boolean[] dp = new boolean[n + 1];
        dp[n] = true;
        for (int i = n - 1; i >= 0; i--)
            for (String w : wordDict)
                if (i + w.length() <= n && dp[i + w.length()] && s.startsWith(w, i)) {
                    dp[i] = true;
                    break;
                }
        return dp[0];
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
```java
class Solution {
    public int lengthOfLIS(int[] nums) {
        int[] dp = new int[nums.length];
        int best = 0;
        for (int i = 0; i < nums.length; i++) {
            dp[i] = 1;
            for (int j = 0; j < i; j++) if (nums[j] < nums[i]) dp[i] = Math.max(dp[i], dp[j] + 1);
            best = Math.max(best, dp[i]);
        }
        return best;
    }
}
```
**Complexity:** Time O(n²) · Space O(n)

#### Approach 2: Binary Search on Tails (Optimal)
```java
class Solution {
    public int lengthOfLIS(int[] nums) {
        int[] tails = new int[nums.length];
        int size = 0;
        for (int n : nums) {
            int lo = 0, hi = size;
            while (lo < hi) { // first index with tails[i] >= n
                int mid = (lo + hi) >>> 1;
                if (tails[mid] < n) lo = mid + 1;
                else hi = mid;
            }
            tails[lo] = n;
            if (lo == size) size++;
        }
        return size;
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 3: Patience Sorting with a TreeSet
```java
class Solution {
    public int lengthOfLIS(int[] nums) {
        TreeSet<Integer> tails = new TreeSet<>();
        for (int n : nums) {
            Integer ceil = tails.ceiling(n);
            if (ceil != null) tails.remove(ceil);
            tails.add(n);
        }
        return tails.size();
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
```java
class Solution {
    public boolean canPartition(int[] nums) {
        int total = Arrays.stream(nums).sum();
        if (total % 2 == 1) return false;
        int target = total / 2;
        boolean[] dp = new boolean[target + 1];
        dp[0] = true;
        for (int n : nums)
            for (int s = target; s >= n; s--) // high to low, so each number is used once
                dp[s] |= dp[s - n];
        return dp[target];
    }
}
```
**Complexity:** Time O(n · target) · Space O(target)

#### Approach 2: BitSet DP (fast)
```java
class Solution {
    public boolean canPartition(int[] nums) {
        int total = Arrays.stream(nums).sum();
        if (total % 2 == 1) return false;
        java.math.BigInteger bits = java.math.BigInteger.ONE; // bit i set means sum i is reachable
        for (int n : nums) bits = bits.or(bits.shiftLeft(n));
        return bits.testBit(total / 2);
    }
}
```
**Complexity:** Time O(n · total / 64) · Space O(total / 64)

#### Approach 3: Set of Reachable Sums
```java
class Solution {
    public boolean canPartition(int[] nums) {
        int total = Arrays.stream(nums).sum();
        if (total % 2 == 1) return false;
        int target = total / 2;
        Set<Integer> reach = new HashSet<>();
        reach.add(0);
        for (int n : nums) {
            Set<Integer> next = new HashSet<>(reach);
            for (int s : reach) {
                if (s + n == target) return true;
                if (s + n < target) next.add(s + n);
            }
            reach = next;
        }
        return reach.contains(target);
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
```java
class Solution {
    public int uniquePaths(int m, int n) {
        int[][] dp = new int[m][n];
        for (int i = 0; i < m; i++) dp[i][0] = 1;
        for (int j = 0; j < n; j++) dp[0][j] = 1;
        for (int i = 1; i < m; i++)
            for (int j = 1; j < n; j++)
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        return dp[m - 1][n - 1];
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D Rolling Row (Optimal DP)
```java
class Solution {
    public int uniquePaths(int m, int n) {
        int[] row = new int[n];
        Arrays.fill(row, 1);
        for (int i = 1; i < m; i++)
            for (int j = 1; j < n; j++)
                row[j] += row[j - 1];
        return row[n - 1];
    }
}
```
**Complexity:** Time O(m · n) · Space O(n)

#### Approach 3: Combinatorics C(m+n-2, m-1)
```java
class Solution {
    public int uniquePaths(int m, int n) {
        long res = 1;
        int k = Math.min(m - 1, n - 1), total = m + n - 2;
        for (int i = 1; i <= k; i++) res = res * (total - k + i) / i; // stays an integer at every step
        return (int) res;
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
```java
class Solution {
    public int longestCommonSubsequence(String text1, String text2) {
        int m = text1.length(), n = text2.length();
        int[][] dp = new int[m + 1][n + 1];
        for (int i = 1; i <= m; i++)
            for (int j = 1; j <= n; j++)
                dp[i][j] = text1.charAt(i - 1) == text2.charAt(j - 1)
                        ? dp[i - 1][j - 1] + 1
                        : Math.max(dp[i - 1][j], dp[i][j - 1]);
        return dp[m][n];
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Two Rows (Optimal space)
```java
class Solution {
    public int longestCommonSubsequence(String text1, String text2) {
        if (text2.length() > text1.length()) return longestCommonSubsequence(text2, text1);
        int n = text2.length();
        int[] prev = new int[n + 1], cur = new int[n + 1];
        for (int i = 1; i <= text1.length(); i++) {
            for (int j = 1; j <= n; j++)
                cur[j] = text1.charAt(i - 1) == text2.charAt(j - 1) ? prev[j - 1] + 1 : Math.max(prev[j], cur[j - 1]);
            int[] t = prev; prev = cur; cur = t;
        }
        return prev[n];
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
```java
class Solution {
    public int maxProfit(int[] prices) {
        int hold = Integer.MIN_VALUE / 2, sold = 0, rest = 0;
        for (int p : prices) {
            int prevSold = sold;
            sold = hold + p;
            hold = Math.max(hold, rest - p);
            rest = Math.max(rest, prevSold);
        }
        return Math.max(sold, rest);
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Memoized DFS (index, canBuy)
```java
class Solution {
    private Integer[][] memo;
    private int[] prices;

    public int maxProfit(int[] prices) {
        this.prices = prices;
        memo = new Integer[prices.length][2];
        return dfs(0, 1);
    }

    private int dfs(int i, int buying) {
        if (i >= prices.length) return 0;
        if (memo[i][buying] != null) return memo[i][buying];
        int cooldown = dfs(i + 1, buying);
        int act = buying == 1
                ? dfs(i + 1, 0) - prices[i]
                : dfs(i + 2, 1) + prices[i]; // selling forces a one-day cooldown
        return memo[i][buying] = Math.max(cooldown, act);
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
```java
class Solution {
    public int change(int amount, int[] coins) {
        int[] dp = new int[amount + 1];
        dp[0] = 1;
        for (int c : coins)               // coins outside means combinations, not permutations
            for (int a = c; a <= amount; a++)
                dp[a] += dp[a - c];
        return dp[amount];
    }
}
```
**Complexity:** Time O(n · amount) · Space O(amount)

#### Approach 2: 2-D DP (clearer state)
```java
class Solution {
    public int change(int amount, int[] coins) {
        int n = coins.length;
        int[][] dp = new int[n + 1][amount + 1]; // dp[i][a] = ways using the first i coins
        for (int i = 0; i <= n; i++) dp[i][0] = 1;
        for (int i = 1; i <= n; i++)
            for (int a = 1; a <= amount; a++) {
                dp[i][a] = dp[i - 1][a];
                if (a >= coins[i - 1]) dp[i][a] += dp[i][a - coins[i - 1]];
            }
        return dp[n][amount];
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
```java
class Solution {
    public int findTargetSumWays(int[] nums, int target) {
        int total = Arrays.stream(nums).sum();
        Integer[][] memo = new Integer[nums.length][2 * total + 1]; // sum offset by total
        return dfs(nums, 0, 0, target, total, memo);
    }

    private int dfs(int[] nums, int i, int sum, int target, int off, Integer[][] memo) {
        if (i == nums.length) return sum == target ? 1 : 0;
        if (memo[i][sum + off] != null) return memo[i][sum + off];
        int v = dfs(nums, i + 1, sum + nums[i], target, off, memo) + dfs(nums, i + 1, sum - nums[i], target, off, memo);
        return memo[i][sum + off] = v;
    }
}
```
**Complexity:** Time O(n · total) · Space O(n · total)

#### Approach 2: Subset-Sum Counting (Optimal)
```java
class Solution {
    public int findTargetSumWays(int[] nums, int target) {
        int total = Arrays.stream(nums).sum();
        if (Math.abs(target) > total || (total + target) % 2 != 0) return 0;
        int p = (total + target) / 2; // the sum of the numbers that get '+'
        int[] dp = new int[p + 1];
        dp[0] = 1;
        for (int n : nums)
            for (int s = p; s >= n; s--)
                dp[s] += dp[s - n];
        return dp[p];
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
```java
class Solution {
    public boolean isInterleave(String s1, String s2, String s3) {
        int m = s1.length(), n = s2.length();
        if (m + n != s3.length()) return false;
        boolean[][] dp = new boolean[m + 1][n + 1];
        dp[0][0] = true;
        for (int i = 0; i <= m; i++)
            for (int j = 0; j <= n; j++) {
                if (i > 0 && s1.charAt(i - 1) == s3.charAt(i + j - 1) && dp[i - 1][j]) dp[i][j] = true;
                if (j > 0 && s2.charAt(j - 1) == s3.charAt(i + j - 1) && dp[i][j - 1]) dp[i][j] = true;
            }
        return dp[m][n];
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D DP (Optimal space)
```java
class Solution {
    public boolean isInterleave(String s1, String s2, String s3) {
        int m = s1.length(), n = s2.length();
        if (m + n != s3.length()) return false;
        boolean[] dp = new boolean[n + 1];
        for (int i = 0; i <= m; i++)
            for (int j = 0; j <= n; j++) {
                if (i == 0 && j == 0) dp[j] = true;
                else if (i == 0) dp[j] = dp[j - 1] && s2.charAt(j - 1) == s3.charAt(j - 1);
                else if (j == 0) dp[j] = dp[j] && s1.charAt(i - 1) == s3.charAt(i - 1);
                else dp[j] = (dp[j] && s1.charAt(i - 1) == s3.charAt(i + j - 1))
                          || (dp[j - 1] && s2.charAt(j - 1) == s3.charAt(i + j - 1));
            }
        return dp[n];
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
```java
class Solution {
    private static final int[][] DIRS = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
    private int[][] memo;

    public int longestIncreasingPath(int[][] matrix) {
        int m = matrix.length, n = matrix[0].length, ans = 0;
        memo = new int[m][n];
        for (int r = 0; r < m; r++)
            for (int c = 0; c < n; c++)
                ans = Math.max(ans, dfs(matrix, r, c));
        return ans;
    }

    private int dfs(int[][] g, int r, int c) {
        if (memo[r][c] != 0) return memo[r][c];
        int best = 1;
        for (int[] d : DIRS) {
            int nr = r + d[0], nc = c + d[1];
            if (nr >= 0 && nc >= 0 && nr < g.length && nc < g[0].length && g[nr][nc] > g[r][c])
                best = Math.max(best, 1 + dfs(g, nr, nc));
        }
        return memo[r][c] = best;
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Topological Sort (peel layers of local maxima)
```java
class Solution {
    public int longestIncreasingPath(int[][] matrix) {
        int m = matrix.length, n = matrix[0].length;
        int[][] dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        int[][] outdeg = new int[m][n];
        Deque<int[]> q = new ArrayDeque<>();
        for (int r = 0; r < m; r++)
            for (int c = 0; c < n; c++) {
                for (int[] d : dirs) {
                    int nr = r + d[0], nc = c + d[1];
                    if (nr >= 0 && nc >= 0 && nr < m && nc < n && matrix[nr][nc] > matrix[r][c]) outdeg[r][c]++;
                }
                if (outdeg[r][c] == 0) q.offer(new int[]{r, c});
            }
        int layers = 0;
        while (!q.isEmpty()) {
            layers++;
            for (int sz = q.size(); sz > 0; sz--) {
                int[] p = q.poll();
                for (int[] d : dirs) {
                    int nr = p[0] + d[0], nc = p[1] + d[1];
                    if (nr >= 0 && nc >= 0 && nr < m && nc < n && matrix[nr][nc] < matrix[p[0]][p[1]]
                            && --outdeg[nr][nc] == 0)
                        q.offer(new int[]{nr, nc});
                }
            }
        }
        return layers;
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
```java
class Solution {
    public int numDistinct(String s, String t) {
        int m = s.length(), n = t.length();
        int[][] dp = new int[m + 1][n + 1];
        for (int i = 0; i <= m; i++) dp[i][0] = 1; // the empty t matches once
        for (int i = 1; i <= m; i++)
            for (int j = 1; j <= n; j++) {
                dp[i][j] = dp[i - 1][j];
                if (s.charAt(i - 1) == t.charAt(j - 1)) dp[i][j] += dp[i - 1][j - 1];
            }
        return dp[m][n];
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D DP, iterate j backward (Optimal space)
```java
class Solution {
    public int numDistinct(String s, String t) {
        int n = t.length();
        int[] dp = new int[n + 1];
        dp[0] = 1;
        for (int i = 0; i < s.length(); i++)
            for (int j = n; j >= 1; j--)
                if (s.charAt(i) == t.charAt(j - 1)) dp[j] += dp[j - 1];
        return dp[n];
    }
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
```java
class Solution {
    public int minDistance(String word1, String word2) {
        int m = word1.length(), n = word2.length();
        int[][] dp = new int[m + 1][n + 1];
        for (int i = 0; i <= m; i++) dp[i][0] = i;
        for (int j = 0; j <= n; j++) dp[0][j] = j;
        for (int i = 1; i <= m; i++)
            for (int j = 1; j <= n; j++)
                dp[i][j] = word1.charAt(i - 1) == word2.charAt(j - 1)
                        ? dp[i - 1][j - 1]
                        : 1 + Math.min(dp[i - 1][j - 1], Math.min(dp[i - 1][j], dp[i][j - 1]));
        return dp[m][n];
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D DP (Optimal space)
```java
class Solution {
    public int minDistance(String word1, String word2) {
        int n = word2.length();
        int[] dp = new int[n + 1];
        for (int j = 0; j <= n; j++) dp[j] = j;
        for (int i = 1; i <= word1.length(); i++) {
            int diag = dp[0]; // dp[i-1][j-1]
            dp[0] = i;
            for (int j = 1; j <= n; j++) {
                int tmp = dp[j];
                dp[j] = word1.charAt(i - 1) == word2.charAt(j - 1)
                        ? diag
                        : 1 + Math.min(diag, Math.min(dp[j], dp[j - 1]));
                diag = tmp;
            }
        }
        return dp[n];
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
```java
class Solution {
    public int maxCoins(int[] nums) {
        int n = nums.length + 2;
        int[] vals = new int[n];
        vals[0] = vals[n - 1] = 1;
        System.arraycopy(nums, 0, vals, 1, nums.length);
        int[][] dp = new int[n][n];
        for (int len = 2; len < n; len++)       // gap between l and r
            for (int l = 0; l + len < n; l++) {
                int r = l + len;
                for (int k = l + 1; k < r; k++) // k is the last balloon burst in (l, r)
                    dp[l][r] = Math.max(dp[l][r], dp[l][k] + vals[l] * vals[k] * vals[r] + dp[k][r]);
            }
        return dp[0][n - 1];
    }
}
```
**Complexity:** Time O(n³) · Space O(n²)

#### Approach 2: Top-Down Memoization
```java
class Solution {
    private int[] vals;
    private int[][] memo;

    public int maxCoins(int[] nums) {
        int n = nums.length + 2;
        vals = new int[n];
        vals[0] = vals[n - 1] = 1;
        System.arraycopy(nums, 0, vals, 1, nums.length);
        memo = new int[n][n];
        for (int[] row : memo) Arrays.fill(row, -1);
        return f(0, n - 1);
    }

    private int f(int l, int r) {
        if (l + 1 == r) return 0;
        if (memo[l][r] >= 0) return memo[l][r];
        int best = 0;
        for (int k = l + 1; k < r; k++) best = Math.max(best, f(l, k) + vals[l] * vals[k] * vals[r] + f(k, r));
        return memo[l][r] = best;
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
```java
class Solution {
    private Boolean[][] memo;

    public boolean isMatch(String s, String p) {
        memo = new Boolean[s.length() + 1][p.length() + 1];
        return dp(s, p, 0, 0);
    }

    private boolean dp(String s, String p, int i, int j) {
        if (j == p.length()) return i == s.length();
        if (memo[i][j] != null) return memo[i][j];
        boolean first = i < s.length() && (p.charAt(j) == s.charAt(i) || p.charAt(j) == '.');
        boolean res;
        if (j + 1 < p.length() && p.charAt(j + 1) == '*')
            res = dp(s, p, i, j + 2) || (first && dp(s, p, i + 1, j)); // skip x*, or consume one char
        else
            res = first && dp(s, p, i + 1, j + 1);
        return memo[i][j] = res;
    }
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Bottom-Up DP (Optimal)
```java
class Solution {
    public boolean isMatch(String s, String p) {
        int m = s.length(), n = p.length();
        boolean[][] dp = new boolean[m + 1][n + 1];
        dp[m][n] = true;
        for (int i = m; i >= 0; i--)
            for (int j = n - 1; j >= 0; j--) {
                boolean first = i < m && (p.charAt(j) == s.charAt(i) || p.charAt(j) == '.');
                if (j + 1 < n && p.charAt(j + 1) == '*') dp[i][j] = dp[i][j + 2] || (first && dp[i + 1][j]);
                else dp[i][j] = first && dp[i + 1][j + 1];
            }
        return dp[0][0];
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
```java
class Solution {
    public int maxSubArray(int[] nums) {
        int best = nums[0], cur = 0;
        for (int n : nums) {
            cur = Math.max(cur + n, n); // drop the running sum once it would only drag n down
            best = Math.max(best, cur);
        }
        return best;
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Divide and Conquer (the follow-up)
```java
class Solution {
    public int maxSubArray(int[] nums) {
        return solve(nums, 0, nums.length - 1);
    }

    private int solve(int[] a, int l, int r) {
        if (l == r) return a[l];
        int mid = (l + r) / 2, leftBest = Integer.MIN_VALUE, rightBest = Integer.MIN_VALUE, sum = 0;
        for (int i = mid; i >= l; i--) leftBest = Math.max(leftBest, sum += a[i]);
        sum = 0;
        for (int i = mid + 1; i <= r; i++) rightBest = Math.max(rightBest, sum += a[i]);
        return Math.max(Math.max(solve(a, l, mid), solve(a, mid + 1, r)), leftBest + rightBest);
    }
}
```
**Complexity:** Time O(n log n) · Space O(log n)

#### Approach 3: Brute Force
```java
class Solution {
    public int maxSubArray(int[] nums) {
        int best = Integer.MIN_VALUE;
        for (int i = 0; i < nums.length; i++) {
            int sum = 0;
            for (int j = i; j < nums.length; j++) best = Math.max(best, sum += nums[j]);
        }
        return best;
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
```java
class Solution {
    public boolean canJump(int[] nums) {
        int goal = nums.length - 1;
        for (int i = nums.length - 2; i >= 0; i--)
            if (i + nums[i] >= goal) goal = i;
        return goal == 0;
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Greedy, Track the Farthest Reach Forward
```java
class Solution {
    public boolean canJump(int[] nums) {
        int reach = 0;
        for (int i = 0; i < nums.length; i++) {
            if (i > reach) return false;
            reach = Math.max(reach, i + nums[i]);
        }
        return true;
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 3: DP (slower, shown for contrast)
```java
class Solution {
    public boolean canJump(int[] nums) {
        int n = nums.length;
        boolean[] good = new boolean[n];
        good[n - 1] = true;
        for (int i = n - 2; i >= 0; i--)
            for (int j = 1; j <= nums[i] && i + j < n; j++)
                if (good[i + j]) {
                    good[i] = true;
                    break;
                }
        return good[0];
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
```java
class Solution {
    public int jump(int[] nums) {
        int jumps = 0, curEnd = 0, farthest = 0;
        for (int i = 0; i < nums.length - 1; i++) {
            farthest = Math.max(farthest, i + nums[i]);
            if (i == curEnd) { // end of this jump's window
                jumps++;
                curEnd = farthest;
            }
        }
        return jumps;
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: DP
```java
class Solution {
    public int jump(int[] nums) {
        int n = nums.length;
        int[] dp = new int[n];
        Arrays.fill(dp, Integer.MAX_VALUE);
        dp[0] = 0;
        for (int i = 0; i < n; i++)
            for (int j = 1; j <= nums[i] && i + j < n; j++)
                dp[i + j] = Math.min(dp[i + j], dp[i] + 1);
        return dp[n - 1];
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
```java
class Solution {
    public int canCompleteCircuit(int[] gas, int[] cost) {
        int total = 0, tank = 0, start = 0;
        for (int i = 0; i < gas.length; i++) {
            int diff = gas[i] - cost[i];
            total += diff;
            tank += diff;
            if (tank < 0) { // no station in [start, i] can work
                start = i + 1;
                tank = 0;
            }
        }
        return total < 0 ? -1 : start;
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Brute Force Simulation
```java
class Solution {
    public int canCompleteCircuit(int[] gas, int[] cost) {
        int n = gas.length;
        for (int s = 0; s < n; s++) {
            int tank = 0, k = 0;
            for (; k < n; k++) {
                int i = (s + k) % n;
                tank += gas[i] - cost[i];
                if (tank < 0) break;
            }
            if (k == n) return s;
        }
        return -1;
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

#### Approach 1: TreeMap of Counts
```java
class Solution {
    public boolean isNStraightHand(int[] hand, int groupSize) {
        if (hand.length % groupSize != 0) return false;
        TreeMap<Integer, Integer> count = new TreeMap<>();
        for (int h : hand) count.merge(h, 1, Integer::sum);
        while (!count.isEmpty()) {
            int first = count.firstKey(); // the smallest card must start a group
            for (int v = first; v < first + groupSize; v++) {
                Integer c = count.get(v);
                if (c == null) return false;
                if (c == 1) count.remove(v);
                else count.put(v, c - 1);
            }
        }
        return true;
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sort + HashMap, Consume Whole Runs at Once
```java
class Solution {
    public boolean isNStraightHand(int[] hand, int groupSize) {
        if (hand.length % groupSize != 0) return false;
        Map<Integer, Integer> count = new HashMap<>();
        for (int h : hand) count.merge(h, 1, Integer::sum);
        Arrays.sort(hand);
        for (int h : hand) {
            int need = count.getOrDefault(h, 0);
            if (need == 0) continue;
            for (int v = h; v < h + groupSize; v++) { // start `need` groups at h
                int c = count.getOrDefault(v, 0);
                if (c < need) return false;
                count.put(v, c - need);
            }
        }
        return true;
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
```java
class Solution {
    public boolean mergeTriplets(int[][] triplets, int[] target) {
        boolean[] found = new boolean[3];
        for (int[] t : triplets) {
            if (t[0] > target[0] || t[1] > target[1] || t[2] > target[2]) continue; // would overshoot
            for (int i = 0; i < 3; i++) if (t[i] == target[i]) found[i] = true;
        }
        return found[0] && found[1] && found[2];
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
```java
class Solution {
    public List<Integer> partitionLabels(String s) {
        int[] last = new int[26];
        for (int i = 0; i < s.length(); i++) last[s.charAt(i) - 'a'] = i;
        List<Integer> res = new ArrayList<>();
        int start = 0, end = 0;
        for (int i = 0; i < s.length(); i++) {
            end = Math.max(end, last[s.charAt(i) - 'a']);
            if (i == end) {
                res.add(end - start + 1);
                start = i + 1;
            }
        }
        return res;
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
```java
class Solution {
    public boolean checkValidString(String s) {
        int lo = 0, hi = 0; // range of possible open-paren counts
        for (char c : s.toCharArray()) {
            if (c == '(') { lo++; hi++; }
            else if (c == ')') { lo--; hi--; }
            else { lo--; hi++; } // '*'
            if (hi < 0) return false;
            lo = Math.max(lo, 0);
        }
        return lo == 0;
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Two Stacks of Indices
```java
class Solution {
    public boolean checkValidString(String s) {
        Deque<Integer> open = new ArrayDeque<>(), star = new ArrayDeque<>();
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c == '(') open.push(i);
            else if (c == '*') star.push(i);
            else if (!open.isEmpty()) open.pop();
            else if (!star.isEmpty()) star.pop();
            else return false;
        }
        while (!open.isEmpty() && !star.isEmpty())
            if (open.pop() > star.pop()) return false; // a '(' with no '*' after it to close it
        return open.isEmpty();
    }
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 3: DP over (index, open count)
```java
class Solution {
    public boolean checkValidString(String s) {
        int n = s.length();
        boolean[] dp = new boolean[n + 2]; // dp[k]: is an open count of k reachable?
        dp[0] = true;
        for (char c : s.toCharArray()) {
            boolean[] next = new boolean[n + 2];
            for (int k = 0; k <= n; k++) {
                if (!dp[k]) continue;
                if (c == '(' || c == '*') next[k + 1] = true;
                if ((c == ')' || c == '*') && k > 0) next[k - 1] = true;
                if (c == '*') next[k] = true;
            }
            dp = next;
        }
        return dp[0];
    }
}
```
**Complexity:** Time O(n²) · Space O(n)

---
## 16. Intervals

> Sort intervals with `Arrays.sort(intervals, (a, b) -> Integer.compare(a[0], b[0]))`. `Integer.compare` avoids the overflow that `a[0] - b[0]` can hit with large negative values.

### 57. Insert Interval
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/insert-interval/)

**Problem:** `intervals` are non-overlapping and sorted by start. Insert `newInterval`, merging wherever needed, and return the result still sorted and non-overlapping.

**Example:** `[[1,3],[6,9]]` + `[2,5]` → `[[1,5],[6,9]]`

**Key insight:** Make three passes over the list: intervals entirely before the new one, intervals that overlap it (merge them), and intervals entirely after.

#### Approach 1: Linear Three-Phase Scan (Optimal)
```java
class Solution {
    public int[][] insert(int[][] intervals, int[] newInterval) {
        List<int[]> res = new ArrayList<>();
        int i = 0, n = intervals.length;
        while (i < n && intervals[i][1] < newInterval[0]) res.add(intervals[i++]); // entirely before
        int lo = newInterval[0], hi = newInterval[1];
        while (i < n && intervals[i][0] <= hi) {                                    // overlapping
            lo = Math.min(lo, intervals[i][0]);
            hi = Math.max(hi, intervals[i][1]);
            i++;
        }
        res.add(new int[]{lo, hi});
        while (i < n) res.add(intervals[i++]);                                      // entirely after
        return res.toArray(new int[0][]);
    }
}
```
**Complexity:** Time O(n) · Space O(n) for the output

#### Approach 2: Binary Search for the Boundaries
```java
class Solution {
    public int[][] insert(int[][] intervals, int[] newInterval) {
        int n = intervals.length;
        int l = lowerBound(intervals, newInterval[0], 1);     // first interval with end >= new start
        int r = upperBound(intervals, newInterval[1], 0);     // first interval with start > new end
        int lo = newInterval[0], hi = newInterval[1];
        if (l < r) {
            lo = Math.min(lo, intervals[l][0]);
            hi = Math.max(hi, intervals[r - 1][1]);
        }
        int[][] res = new int[n - (r - l) + 1][];
        int k = 0;
        for (int i = 0; i < l; i++) res[k++] = intervals[i];
        res[k++] = new int[]{lo, hi};
        for (int i = r; i < n; i++) res[k++] = intervals[i];
        return res;
    }

    private int lowerBound(int[][] a, int x, int col) { // first i with a[i][col] >= x
        int lo = 0, hi = a.length;
        while (lo < hi) {
            int mid = (lo + hi) >>> 1;
            if (a[mid][col] >= x) hi = mid; else lo = mid + 1;
        }
        return lo;
    }

    private int upperBound(int[][] a, int x, int col) { // first i with a[i][col] > x
        int lo = 0, hi = a.length;
        while (lo < hi) {
            int mid = (lo + hi) >>> 1;
            if (a[mid][col] > x) hi = mid; else lo = mid + 1;
        }
        return lo;
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
```java
class Solution {
    public int[][] merge(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[0], b[0]));
        List<int[]> res = new ArrayList<>();
        for (int[] iv : intervals) {
            if (!res.isEmpty() && iv[0] <= res.get(res.size() - 1)[1]) {
                int[] last = res.get(res.size() - 1);
                last[1] = Math.max(last[1], iv[1]);
            } else {
                res.add(iv);
            }
        }
        return res.toArray(new int[0][]);
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sweep over Start and End Events
```java
class Solution {
    public int[][] merge(int[][] intervals) {
        int n = intervals.length;
        int[] starts = new int[n], ends = new int[n];
        for (int i = 0; i < n; i++) {
            starts[i] = intervals[i][0];
            ends[i] = intervals[i][1];
        }
        Arrays.sort(starts);
        Arrays.sort(ends);
        List<int[]> res = new ArrayList<>();
        for (int i = 0, begin = 0; i < n; i++) {
            // close a merged block when the next start begins after this end
            if (i == n - 1 || starts[i + 1] > ends[i]) {
                res.add(new int[]{starts[begin], ends[i]});
                begin = i + 1;
            }
        }
        return res.toArray(new int[0][]);
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
```java
class Solution {
    public int eraseOverlapIntervals(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[1], b[1]));
        int removed = 0;
        long end = Long.MIN_VALUE;
        for (int[] iv : intervals) {
            if (iv[0] >= end) end = iv[1]; // keep it
            else removed++;
        }
        return removed;
    }
}
```
**Complexity:** Time O(n log n) · Space O(log n)

#### Approach 2: Sort by Start, Drop the One That Ends Later
```java
class Solution {
    public int eraseOverlapIntervals(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[0], b[0]));
        int removed = 0, prevEnd = intervals[0][1];
        for (int i = 1; i < intervals.length; i++) {
            if (intervals[i][0] >= prevEnd) {
                prevEnd = intervals[i][1];
            } else {
                removed++;
                prevEnd = Math.min(prevEnd, intervals[i][1]);
            }
        }
        return removed;
    }
}
```
**Complexity:** Time O(n log n) · Space O(log n)

---

### 252. Meeting Rooms (Premium)
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/meeting-rooms/) · Free mirror: LintCode 920

**Problem:** Given meeting time intervals, can one person attend every meeting (that is, do no two meetings overlap)?

**Example:** `[[0,30],[5,10],[15,20]]` → `false`

#### Approach: Sort by Start (Optimal)
```java
class Solution {
    public boolean canAttendMeetings(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[0], b[0]));
        for (int i = 1; i < intervals.length; i++)
            if (intervals[i][0] < intervals[i - 1][1]) return false;
        return true;
    }
}
```
**Complexity:** Time O(n log n) · Space O(log n)

**Brute force:** Check every pair for overlap: O(n²).

---

### 253. Meeting Rooms II (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/meeting-rooms-ii/) · Free mirror: LintCode 919

**Problem:** Return the minimum number of conference rooms needed to hold every meeting.

**Example:** `[[0,30],[5,10],[15,20]]` → `2`

**Key insight:** The answer is the maximum number of meetings running at the same moment.

#### Approach 1: Separate Sorted Starts and Ends (two pointers)
```java
class Solution {
    public int minMeetingRooms(int[][] intervals) {
        int n = intervals.length;
        int[] starts = new int[n], ends = new int[n];
        for (int i = 0; i < n; i++) {
            starts[i] = intervals[i][0];
            ends[i] = intervals[i][1];
        }
        Arrays.sort(starts);
        Arrays.sort(ends);
        int rooms = 0, best = 0, e = 0;
        for (int s = 0; s < n; s++) {
            while (e < n && ends[e] <= starts[s]) {
                rooms--; // that meeting has ended and freed its room
                e++;
            }
            best = Math.max(best, ++rooms);
        }
        return best;
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Min-Heap of End Times
```java
class Solution {
    public int minMeetingRooms(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[0], b[0]));
        PriorityQueue<Integer> ends = new PriorityQueue<>();
        for (int[] iv : intervals) {
            if (!ends.isEmpty() && ends.peek() <= iv[0]) ends.poll(); // reuse the room that frees up earliest
            ends.offer(iv[1]);
        }
        return ends.size();
    }
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 3: Line Sweep with a TreeMap
```java
class Solution {
    public int minMeetingRooms(int[][] intervals) {
        TreeMap<Integer, Integer> delta = new TreeMap<>();
        for (int[] iv : intervals) {
            delta.merge(iv[0], 1, Integer::sum);
            delta.merge(iv[1], -1, Integer::sum);
        }
        int cur = 0, best = 0;
        for (int d : delta.values()) best = Math.max(best, cur += d);
        return best;
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
```java
class Solution {
    public int[] minInterval(int[][] intervals, int[] queries) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[0], b[0]));
        Integer[] qIdx = new Integer[queries.length];
        for (int i = 0; i < queries.length; i++) qIdx[i] = i;
        Arrays.sort(qIdx, (a, b) -> Integer.compare(queries[a], queries[b]));
        PriorityQueue<int[]> heap = new PriorityQueue<>((a, b) -> Integer.compare(a[0], b[0])); // {size, end}
        int[] res = new int[queries.length];
        int i = 0;
        for (int qi : qIdx) {
            int q = queries[qi];
            while (i < intervals.length && intervals[i][0] <= q) {
                heap.offer(new int[]{intervals[i][1] - intervals[i][0] + 1, intervals[i][1]});
                i++;
            }
            while (!heap.isEmpty() && heap.peek()[1] < q) heap.poll(); // ends before q
            res[qi] = heap.isEmpty() ? -1 : heap.peek()[0];
        }
        return res;
    }
}
```
**Complexity:** Time O(n log n + q log q) · Space O(n + q)

#### Approach 2: Sort Intervals by Size + TreeMap of Unanswered Queries
Visit intervals from smallest to largest. Each interval answers every unanswered query it contains, and those queries are then removed.
```java
class Solution {
    public int[] minInterval(int[][] intervals, int[] queries) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[1] - a[0], b[1] - b[0]));
        TreeMap<Integer, List<Integer>> pending = new TreeMap<>(); // query value → indices
        for (int i = 0; i < queries.length; i++) pending.computeIfAbsent(queries[i], k -> new ArrayList<>()).add(i);
        int[] res = new int[queries.length];
        Arrays.fill(res, -1);
        for (int[] iv : intervals) {
            int size = iv[1] - iv[0] + 1;
            Integer q;
            while ((q = pending.ceilingKey(iv[0])) != null && q <= iv[1])
                for (int idx : pending.remove(q)) res[idx] = size;
        }
        return res;
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
```java
class Solution {
    public void rotate(int[][] matrix) {
        int n = matrix.length;
        for (int i = 0; i < n; i++)
            for (int j = i + 1; j < n; j++) {
                int t = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = t;
            }
        for (int[] row : matrix)
            for (int l = 0, r = n - 1; l < r; l++, r--) {
                int t = row[l];
                row[l] = row[r];
                row[r] = t;
            }
    }
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Rotate Four Cells at a Time, Layer by Layer
```java
class Solution {
    public void rotate(int[][] matrix) {
        int n = matrix.length;
        for (int l = 0, r = n - 1; l < r; l++, r--) {
            for (int i = 0; i < r - l; i++) {
                int top = l, bottom = r;
                int tmp = matrix[top][l + i];                    // save top-left
                matrix[top][l + i] = matrix[bottom - i][l];      // bottom-left to top-left
                matrix[bottom - i][l] = matrix[bottom][r - i];   // bottom-right to bottom-left
                matrix[bottom][r - i] = matrix[top + i][r];      // top-right to bottom-right
                matrix[top + i][r] = tmp;                        // top-left to top-right
            }
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
```java
class Solution {
    public List<Integer> spiralOrder(int[][] matrix) {
        List<Integer> res = new ArrayList<>();
        int top = 0, bottom = matrix.length - 1, left = 0, right = matrix[0].length - 1;
        while (top <= bottom && left <= right) {
            for (int c = left; c <= right; c++) res.add(matrix[top][c]);
            top++;
            for (int r = top; r <= bottom; r++) res.add(matrix[r][right]);
            right--;
            if (top <= bottom) {
                for (int c = right; c >= left; c--) res.add(matrix[bottom][c]);
                bottom--;
            }
            if (left <= right) {
                for (int r = bottom; r >= top; r--) res.add(matrix[r][left]);
                left++;
            }
        }
        return res;
    }
}
```
**Complexity:** Time O(m · n) · Space O(1) extra

#### Approach 2: Direction Vectors with a Visited Grid
```java
class Solution {
    public List<Integer> spiralOrder(int[][] matrix) {
        int m = matrix.length, n = matrix[0].length;
        boolean[][] seen = new boolean[m][n];
        int[] dr = {0, 1, 0, -1}, dc = {1, 0, -1, 0};
        List<Integer> res = new ArrayList<>();
        int r = 0, c = 0, d = 0;
        for (int k = 0; k < m * n; k++) {
            res.add(matrix[r][c]);
            seen[r][c] = true;
            int nr = r + dr[d], nc = c + dc[d];
            if (nr < 0 || nc < 0 || nr >= m || nc >= n || seen[nr][nc]) { // turn clockwise
                d = (d + 1) % 4;
                nr = r + dr[d];
                nc = c + dc[d];
            }
            r = nr;
            c = nc;
        }
        return res;
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
```java
class Solution {
    public void setZeroes(int[][] matrix) {
        int m = matrix.length, n = matrix[0].length;
        boolean[] rows = new boolean[m], cols = new boolean[n];
        for (int i = 0; i < m; i++)
            for (int j = 0; j < n; j++)
                if (matrix[i][j] == 0) rows[i] = cols[j] = true;
        for (int i = 0; i < m; i++)
            for (int j = 0; j < n; j++)
                if (rows[i] || cols[j]) matrix[i][j] = 0;
    }
}
```
**Complexity:** Time O(m · n) · Space O(m + n)

#### Approach 2: First Row and Column as Markers (Optimal)
```java
class Solution {
    public void setZeroes(int[][] matrix) {
        int m = matrix.length, n = matrix[0].length;
        boolean firstColZero = false;
        for (int i = 0; i < m; i++) {
            if (matrix[i][0] == 0) firstColZero = true;
            for (int j = 1; j < n; j++)
                if (matrix[i][j] == 0) matrix[i][0] = matrix[0][j] = 0;
        }
        for (int i = m - 1; i >= 0; i--) { // bottom-up so row 0's markers are used before row 0 is zeroed
            for (int j = n - 1; j >= 1; j--)
                if (matrix[i][0] == 0 || matrix[0][j] == 0) matrix[i][j] = 0;
            if (firstColZero) matrix[i][0] = 0;
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
```java
class Solution {
    public boolean isHappy(int n) {
        Set<Integer> seen = new HashSet<>();
        while (n != 1 && seen.add(n)) n = next(n);
        return n == 1;
    }

    private int next(int n) {
        int s = 0;
        for (; n > 0; n /= 10) s += (n % 10) * (n % 10);
        return s;
    }
}
```
**Complexity:** Time O(log n) per step, with only a few steps · Space O(log n)

#### Approach 2: Floyd's Cycle Detection (Optimal)
```java
class Solution {
    public boolean isHappy(int n) {
        int slow = n, fast = next(n);
        while (fast != 1 && slow != fast) {
            slow = next(slow);
            fast = next(next(fast));
        }
        return fast == 1;
    }

    private int next(int n) {
        int s = 0;
        for (; n > 0; n /= 10) s += (n % 10) * (n % 10);
        return s;
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
```java
class Solution {
    public int[] plusOne(int[] digits) {
        for (int i = digits.length - 1; i >= 0; i--) {
            if (digits[i] < 9) {
                digits[i]++;
                return digits;
            }
            digits[i] = 0;
        }
        int[] res = new int[digits.length + 1]; // every digit was 9
        res[0] = 1;
        return res;
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
```java
class Solution {
    public double myPow(double x, int n) {
        long p = n; // avoids overflow when negating Integer.MIN_VALUE
        if (p < 0) {
            x = 1 / x;
            p = -p;
        }
        double res = 1.0;
        while (p > 0) {
            if ((p & 1) == 1) res *= x;
            x *= x;
            p >>= 1;
        }
        return res;
    }
}
```
**Complexity:** Time O(log n) · Space O(1)

#### Approach 2: Recursive
```java
class Solution {
    public double myPow(double x, int n) {
        long e = n;
        return e < 0 ? 1 / pow(x, -e) : pow(x, e);
    }

    private double pow(double b, long e) {
        if (e == 0) return 1;
        double half = pow(b, e / 2);
        return e % 2 == 0 ? half * half : half * half * b;
    }
}
```
**Complexity:** Time O(log n) · Space O(log n)

---

### 43. Multiply Strings
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/multiply-strings/)

**Problem:** Multiply two non-negative integers given as strings and return the product as a string, without converting the inputs to integers directly.

**Example:** `"123" × "456"` → `"56088"`

**Key insight:** The product of `num1[i]` and `num2[j]` lands in positions `i+j` and `i+j+1` of an array of length `m+n`.

#### Approach: Grade-School Multiplication into a Position Array (Optimal)
```java
class Solution {
    public String multiply(String num1, String num2) {
        if (num1.equals("0") || num2.equals("0")) return "0";
        int m = num1.length(), n = num2.length();
        int[] pos = new int[m + n];
        for (int i = m - 1; i >= 0; i--)
            for (int j = n - 1; j >= 0; j--) {
                int mul = (num1.charAt(i) - '0') * (num2.charAt(j) - '0');
                int sum = mul + pos[i + j + 1];
                pos[i + j + 1] = sum % 10;
                pos[i + j] += sum / 10;
            }
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < pos.length; i++) {
            if (i == 0 && pos[i] == 0) continue; // at most one leading zero
            sb.append(pos[i]);
        }
        return sb.toString();
    }
}
```
**Complexity:** Time O(m · n) · Space O(m + n)

**Alternative:** Karatsuba multiplication runs in O(n^1.585), which only pays off for very large inputs. (`BigInteger` is not allowed by the problem.)

---

### 2013. Detect Squares
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/detect-squares/)

**Problem:** Design `DetectSquares`. `add(point)` adds a point; duplicates are allowed. `count(point)` returns how many ways you can pick three stored points that form an axis-aligned square of positive area together with the query point.

**Key insight:** Treat each stored point that is diagonal to the query (`|dx| == |dy| ≠ 0`) as the opposite corner. Multiply the counts of the other two corners.

#### Approach 1: Count Map + Iterate Diagonal Candidates (Optimal)
```java
class DetectSquares {
    private final Map<Long, Integer> cnt = new HashMap<>();
    private final List<int[]> pts = new ArrayList<>(); // distinct points

    public DetectSquares() {}

    private long key(int x, int y) { return ((long) x << 32) | (y & 0xffffffffL); }

    public void add(int[] point) {
        long k = key(point[0], point[1]);
        if (cnt.merge(k, 1, Integer::sum) == 1) pts.add(point);
    }

    public int count(int[] point) {
        int qx = point[0], qy = point[1], res = 0;
        for (int[] p : pts) {
            int x = p[0], y = p[1];
            if (x == qx || Math.abs(x - qx) != Math.abs(y - qy)) continue; // not a diagonal corner
            res += cnt.get(key(x, y)) * cnt.getOrDefault(key(qx, y), 0) * cnt.getOrDefault(key(x, qy), 0);
        }
        return res;
    }
}
```
**Complexity:** `add` O(1) · `count` O(P) for P distinct points · Space O(P)

#### Approach 2: Points Grouped by x-Coordinate
```java
class DetectSquares2 {
    private final Map<Integer, Map<Integer, Integer>> byX = new HashMap<>(); // x → (y → count)

    public void add(int[] point) {
        byX.computeIfAbsent(point[0], k -> new HashMap<>()).merge(point[1], 1, Integer::sum);
    }

    public int count(int[] point) {
        int qx = point[0], qy = point[1], res = 0;
        Map<Integer, Integer> col = byX.get(qx);
        if (col == null) return 0;
        for (Map.Entry<Integer, Integer> e : col.entrySet()) {
            int y = e.getKey(), side = y - qy;
            if (side == 0) continue;
            for (int x : new int[]{qx + side, qx - side}) { // square to the right or to the left
                Map<Integer, Integer> other = byX.get(x);
                if (other == null) continue;
                res += e.getValue() * other.getOrDefault(y, 0) * other.getOrDefault(qy, 0);
            }
        }
        return res;
    }
}
```
**Complexity:** `add` O(1) · `count` O(points sharing the query's x) · Space O(P)

---
## 18. Bit Manipulation

> Java has no unsigned `int`. Use `>>>` (logical shift, fills with zeros) rather than `>>` (arithmetic shift, copies the sign bit) when walking the bits of a possibly negative value.

### 136. Single Number
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/single-number/)

**Problem:** Every element appears twice except one. Find that one in O(n) time and O(1) extra space.

**Example:** `[4,1,2,1,2]` → `4`

**Key insight:** `a ^ a = 0` and `a ^ 0 = a`, so XOR-ing everything cancels the pairs.

#### Approach 1: XOR (Optimal)
```java
class Solution {
    public int singleNumber(int[] nums) {
        int res = 0;
        for (int n : nums) res ^= n;
        return res;
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: HashSet Toggle
```java
class Solution {
    public int singleNumber(int[] nums) {
        Set<Integer> set = new HashSet<>();
        for (int n : nums) if (!set.add(n)) set.remove(n);
        return set.iterator().next();
    }
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 191. Number of 1 Bits
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/number-of-1-bits/)

**Problem:** Return the number of set bits (the Hamming weight) in `n`.

**Example:** `11` (binary `1011`) → `3`

#### Approach 1: Check Every Bit
```java
class Solution {
    public int hammingWeight(int n) {
        int count = 0;
        for (int i = 0; i < 32; i++) {
            count += n & 1;
            n >>>= 1; // logical shift also handles a negative n
        }
        return count;
    }
}
```
**Complexity:** Time O(32) · Space O(1)

#### Approach 2: Brian Kernighan's Trick (Optimal)
`n & (n-1)` clears the lowest set bit, so the loop runs once per set bit.
```java
class Solution {
    public int hammingWeight(int n) {
        int count = 0;
        while (n != 0) {
            n &= n - 1;
            count++;
        }
        return count;
    }
}
```
**Complexity:** Time O(number of set bits) · Space O(1)

#### Approach 3: Standard Library (hardware popcount)
```java
class Solution {
    public int hammingWeight(int n) {
        return Integer.bitCount(n);
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
```java
class Solution {
    public int[] countBits(int n) {
        int[] res = new int[n + 1];
        for (int i = 1; i <= n; i++) res[i] = res[i >> 1] + (i & 1);
        return res;
    }
}
```
**Complexity:** Time O(n) · Space O(1) extra

#### Approach 2: DP on the Lowest Set Bit
```java
class Solution {
    public int[] countBits(int n) {
        int[] res = new int[n + 1];
        for (int i = 1; i <= n; i++) res[i] = res[i & (i - 1)] + 1;
        return res;
    }
}
```
**Complexity:** Time O(n) · Space O(1) extra

#### Approach 3: Popcount Each Number
```java
class Solution {
    public int[] countBits(int n) {
        int[] res = new int[n + 1];
        for (int i = 0; i <= n; i++) res[i] = Integer.bitCount(i);
        return res;
    }
}
```
**Complexity:** Time O(n), assuming popcount is O(1) · Space O(1) extra

---

### 190. Reverse Bits
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/reverse-bits/)

**Problem:** Reverse the bits of a 32-bit unsigned integer.

**Example:** `00000010100101000001111010011100` → `00111001011110000010100101000000`

> LeetCode's Java signature is `public int reverseBits(int n)`, with the input treated as an unsigned 32-bit value.

#### Approach 1: Bit-by-Bit Shift
```java
class Solution {
    public int reverseBits(int n) {
        int res = 0;
        for (int i = 0; i < 32; i++) {
            res = (res << 1) | (n & 1);
            n >>>= 1;
        }
        return res;
    }
}
```
**Complexity:** Time O(32) · Space O(1)

#### Approach 2: Divide and Conquer Swaps (Optimal)
```java
class Solution {
    public int reverseBits(int n) {
        n = (n >>> 16) | (n << 16);
        n = ((n & 0xff00ff00) >>> 8) | ((n & 0x00ff00ff) << 8);
        n = ((n & 0xf0f0f0f0) >>> 4) | ((n & 0x0f0f0f0f) << 4);
        n = ((n & 0xcccccccc) >>> 2) | ((n & 0x33333333) << 2);
        n = ((n & 0xaaaaaaaa) >>> 1) | ((n & 0x55555555) << 1);
        return n;
    }
}
```
**Complexity:** Time O(1) (5 steps) · Space O(1)

#### Approach 3: Standard Library
```java
class Solution {
    public int reverseBits(int n) {
        return Integer.reverse(n);
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
```java
class Solution {
    public int missingNumber(int[] nums) {
        int n = nums.length, expected = n * (n + 1) / 2;
        for (int v : nums) expected -= v;
        return expected;
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: XOR of Indices and Values (Optimal, can't overflow)
```java
class Solution {
    public int missingNumber(int[] nums) {
        int res = nums.length;
        for (int i = 0; i < nums.length; i++) res ^= i ^ nums[i];
        return res;
    }
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 3: Sort + Binary Search
```java
class Solution {
    public int missingNumber(int[] nums) {
        Arrays.sort(nums);
        int lo = 0, hi = nums.length;
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (nums[mid] > mid) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }
}
```
**Complexity:** Time O(n log n) · Space O(log n)

---

### 371. Sum of Two Integers
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/sum-of-two-integers/)

**Problem:** Return `a + b` without using the `+` or `-` operators.

**Example:** `a = 1, b = 2` → `3`; `a = -2, b = 3` → `1`

**Key insight:** `a ^ b` is the sum without carries, and `(a & b) << 1` is the carry. Repeat until the carry is zero. Work in `int32` to get LeetCode's 32-bit two's-complement behavior.

#### Approach 1: Iterative XOR/AND (Optimal)
```java
class Solution {
    public int getSum(int a, int b) {
        while (b != 0) {
            int carry = (a & b) << 1; // bits that carry into the next position
            a ^= b;                   // sum without carries
            b = carry;
        }
        return a;
    }
}
```
**Complexity:** Time O(32) · Space O(1)

> Java's `int` is already 32-bit two's complement, so negative inputs work with no special handling. The carry shifts off the top bit within 32 iterations.

#### Approach 2: Recursive
```java
class Solution {
    public int getSum(int a, int b) {
        return b == 0 ? a : getSum(a ^ b, (a & b) << 1);
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

#### Approach 1: Pop and Push with Overflow Guards (Optimal)
```java
class Solution {
    public int reverse(int x) {
        int res = 0;
        while (x != 0) {
            int d = x % 10; // Java's % keeps the sign of x
            x /= 10;
            if (res > Integer.MAX_VALUE / 10 || (res == Integer.MAX_VALUE / 10 && d > 7)) return 0;
            if (res < Integer.MIN_VALUE / 10 || (res == Integer.MIN_VALUE / 10 && d < -8)) return 0;
            res = res * 10 + d;
        }
        return res;
    }
}
```
**Complexity:** Time O(log₁₀ x) · Space O(1)

#### Approach 2: 64-bit Accumulator (simpler, if allowed)
```java
class Solution {
    public int reverse(int x) {
        long res = 0;
        for (; x != 0; x /= 10) res = res * 10 + x % 10;
        return (res > Integer.MAX_VALUE || res < Integer.MIN_VALUE) ? 0 : (int) res;
    }
}
```
**Complexity:** Time O(log₁₀ x) · Space O(1)

#### Approach 3: `Math.multiplyExact` / `addExact` (Java 8+)
```java
class Solution {
    public int reverse(int x) {
        int res = 0;
        try {
            for (; x != 0; x /= 10) res = Math.addExact(Math.multiplyExact(res, 10), x % 10);
        } catch (ArithmeticException overflow) {
            return 0;
        }
        return res;
    }
}
```
**Complexity:** Time O(log₁₀ x) · Space O(1). Readable, but exceptions are slow if overflow happens often.

---

## Complexity Cheat Sheet

| Pattern | Typical time | Typical space | Signals in the problem | Java tools |
|---|---|---|---|---|
| Hash map / set | O(n) | O(n) | "seen before", duplicates, complements | `HashMap`, `HashSet`, `merge` |
| Two pointers | O(n) | O(1) | sorted input, pairs, palindromes | indices on an array |
| Sliding window | O(n) | O(k) | contiguous substring or subarray with a constraint | `int[128]` counts |
| Monotonic stack | O(n) | O(n) | next greater or smaller element, histograms | `ArrayDeque` |
| Binary search | O(log n) | O(1) | sorted input, or "minimum k such that…" | `Arrays.binarySearch`, `TreeMap.floorKey` |
| Heap | O(n log k) | O(k) | top-k, k-way merge, streaming median | `PriorityQueue` |
| DFS / BFS | O(V + E) | O(V) | grids, connectivity, shortest path in an unweighted graph | `ArrayDeque<int[]>` |
| Topological sort | O(V + E) | O(V + E) | dependencies, ordering, cycle detection | in-degree array + queue |
| Union-find | O(α(n)) per operation | O(n) | dynamic connectivity, redundant edges | `int[] parent` |
| Dijkstra | O(E log V) | O(V + E) | weighted shortest path with non-negative weights | `PriorityQueue<int[]>` |
| Backtracking | O(2ⁿ) / O(n!) | O(n) | "all combinations / permutations / partitions" | `new ArrayList<>(path)` |
| 1-D DP | O(n) to O(n²) | O(1) to O(n) | best value over a prefix | `int[] dp` |
| 2-D DP | O(m · n) | O(n) when rolled | two strings, grids, intervals | `int[][] dp` |
| Greedy | O(n) or O(n log n) | O(1) | a locally best choice provably works | sort + scan |
| Bit tricks | O(1) to O(n) | O(1) | XOR pairing, powers of two, bit counts | `Integer.bitCount`, `>>>` |

---

*End of document: 150 problems, 18 categories.*
