# LeetCode "Blind 150" (NeetCode 150) — Complete Go Solutions

A full Go reference for the 150 problems in the **NeetCode 150** list (the Blind 75 plus 75 more), grouped by pattern. Every problem has:

- its **LeetCode number and title**, with difficulty
- a **short problem statement** in my own words, plus an example
- one or more **complete Go solutions**: brute force → better → optimal where that comparison helps
- **time and space complexity** for each approach
- a **key insight** line explaining the idea

> Every Go snippet in this file was compiled with Go 1.24 (`go vet` clean). The code uses the built-in `min`/`max` functions, so you need **Go 1.21+**. Function signatures match LeetCode's Go templates, so you can paste a solution straight into the editor. Alternative approaches use different function names, such as `twoSumBrute`, so they can sit side by side.

---

## Shared Definitions

LeetCode provides these types for linked-list and tree problems. They are assumed (not redeclared) in those sections.

```go
// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
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

## Go Interview Toolkit (quick reference)

```go
// Sorting
sort.Ints(a)
sort.Slice(a, func(i, j int) bool { return a[i] < a[j] })
slices.Sort(a) // Go 1.21+

// Min-heap of ints with container/heap
type IntHeap []int
func (h IntHeap) Len() int            { return len(h) }
func (h IntHeap) Less(i, j int) bool  { return h[i] < h[j] } // use > for max-heap
func (h IntHeap) Swap(i, j int)       { h[i], h[j] = h[j], h[i] }
func (h *IntHeap) Push(x any)         { *h = append(*h, x.(int)) }
func (h *IntHeap) Pop() any           { old := *h; n := len(old); x := old[n-1]; *h = old[:n-1]; return x }
// heap.Init(h); heap.Push(h, 5); x := heap.Pop(h).(int); top := (*h)[0]

// Stack: s = append(s, x); top := s[len(s)-1]; s = s[:len(s)-1]
// Queue: q = append(q, x); front := q[0]; q = q[1:]
// Set:   set := map[int]struct{}{}; set[x] = struct{}{}; _, ok := set[x]
// Strings: use []byte for mutation, strings.Builder for building
```

---
## 1. Arrays & Hashing

### 217. Contains Duplicate
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/contains-duplicate/)

**Problem:** Given an integer array `nums`, return `true` if any value appears at least twice, and `false` if every element is distinct.

**Example:** `nums = [1,2,3,1]` → `true`; `nums = [1,2,3,4]` → `false`

**Key insight:** A hash set gives O(1) membership checks, so you can spot a repeat in one pass.

#### Approach 1: Brute Force
```go
func containsDuplicateBrute(nums []int) bool {
	for i := 0; i < len(nums); i++ {
		for j := i + 1; j < len(nums); j++ {
			if nums[i] == nums[j] {
				return true
			}
		}
	}
	return false
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Sorting
```go
func containsDuplicateSort(nums []int) bool {
	sort.Ints(nums)
	for i := 1; i < len(nums); i++ {
		if nums[i] == nums[i-1] {
			return true
		}
	}
	return false
}
```
**Complexity:** Time O(n log n) · Space O(1), or O(log n) for the sort's stack

#### Approach 3: Hash Set (Optimal)
```go
func containsDuplicate(nums []int) bool {
	seen := make(map[int]struct{}, len(nums))
	for _, n := range nums {
		if _, ok := seen[n]; ok {
			return true
		}
		seen[n] = struct{}{}
	}
	return false
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
```go
func isAnagramSort(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}
	a, b := []byte(s), []byte(t)
	sort.Slice(a, func(i, j int) bool { return a[i] < a[j] })
	sort.Slice(b, func(i, j int) bool { return b[i] < b[j] })
	return string(a) == string(b)
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Frequency Array (Optimal)
```go
func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}
	var count [26]int
	for i := 0; i < len(s); i++ {
		count[s[i]-'a']++
		count[t[i]-'a']--
	}
	for _, c := range count {
		if c != 0 {
			return false
		}
	}
	return true
}
```
**Complexity:** Time O(n) · Space O(1) (26 counters)

#### Approach 3: Hash Map (handles Unicode follow-up)
```go
func isAnagramUnicode(s string, t string) bool {
	count := map[rune]int{}
	for _, r := range s {
		count[r]++
	}
	for _, r := range t {
		count[r]--
		if count[r] < 0 {
			return false
		}
	}
	for _, c := range count {
		if c != 0 {
			return false
		}
	}
	return true
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
```go
func twoSumBrute(nums []int, target int) []int {
	for i := 0; i < len(nums); i++ {
		for j := i + 1; j < len(nums); j++ {
			if nums[i]+nums[j] == target {
				return []int{i, j}
			}
		}
	}
	return nil
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: One-Pass Hash Map (Optimal)
```go
func twoSum(nums []int, target int) []int {
	idx := make(map[int]int, len(nums))
	for i, n := range nums {
		if j, ok := idx[target-n]; ok {
			return []int{j, i}
		}
		idx[n] = i
	}
	return nil
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
```go
func groupAnagramsSort(strs []string) [][]string {
	groups := map[string][]string{}
	for _, s := range strs {
		b := []byte(s)
		sort.Slice(b, func(i, j int) bool { return b[i] < b[j] })
		key := string(b)
		groups[key] = append(groups[key], s)
	}
	res := make([][]string, 0, len(groups))
	for _, g := range groups {
		res = append(res, g)
	}
	return res
}
```
**Complexity:** Time O(m · k log k), for m strings of max length k · Space O(m · k)

#### Approach 2: Count Array as Key (Optimal)
Go arrays (unlike slices) are comparable, so `[26]int` works directly as a map key.
```go
func groupAnagrams(strs []string) [][]string {
	groups := map[[26]int][]string{}
	for _, s := range strs {
		var key [26]int
		for i := 0; i < len(s); i++ {
			key[s[i]-'a']++
		}
		groups[key] = append(groups[key], s)
	}
	res := make([][]string, 0, len(groups))
	for _, g := range groups {
		res = append(res, g)
	}
	return res
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
```go
func topKFrequentSort(nums []int, k int) []int {
	freq := map[int]int{}
	for _, n := range nums {
		freq[n]++
	}
	keys := make([]int, 0, len(freq))
	for n := range freq {
		keys = append(keys, n)
	}
	sort.Slice(keys, func(i, j int) bool { return freq[keys[i]] > freq[keys[j]] })
	return keys[:k]
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Min-Heap of Size k
```go
type freqPair struct{ val, cnt int }
type freqHeap []freqPair

func (h freqHeap) Len() int           { return len(h) }
func (h freqHeap) Less(i, j int) bool { return h[i].cnt < h[j].cnt }
func (h freqHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *freqHeap) Push(x any)        { *h = append(*h, x.(freqPair)) }
func (h *freqHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func topKFrequentHeap(nums []int, k int) []int {
	freq := map[int]int{}
	for _, n := range nums {
		freq[n]++
	}
	h := &freqHeap{}
	for v, c := range freq {
		heap.Push(h, freqPair{v, c})
		if h.Len() > k {
			heap.Pop(h)
		}
	}
	res := make([]int, 0, k)
	for h.Len() > 0 {
		res = append(res, heap.Pop(h).(freqPair).val)
	}
	return res
}
```
**Complexity:** Time O(n log k) · Space O(n)

#### Approach 3: Bucket Sort (Optimal)
```go
func topKFrequent(nums []int, k int) []int {
	freq := map[int]int{}
	for _, n := range nums {
		freq[n]++
	}
	buckets := make([][]int, len(nums)+1) // index = frequency
	for v, c := range freq {
		buckets[c] = append(buckets[c], v)
	}
	res := make([]int, 0, k)
	for f := len(buckets) - 1; f >= 0 && len(res) < k; f-- {
		res = append(res, buckets[f]...)
	}
	return res[:k]
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
```go
type Codec struct{}

func (c *Codec) Encode(strs []string) string {
	var sb strings.Builder
	for _, s := range strs {
		sb.WriteString(strconv.Itoa(len(s)))
		sb.WriteByte('#')
		sb.WriteString(s)
	}
	return sb.String()
}

func (c *Codec) Decode(s string) []string {
	res := []string{}
	i := 0
	for i < len(s) {
		j := i
		for s[j] != '#' {
			j++
		}
		length, _ := strconv.Atoi(s[i:j])
		start := j + 1
		res = append(res, s[start:start+length])
		i = start + length
	}
	return res
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
```go
func productExceptSelfArrays(nums []int) []int {
	n := len(nums)
	prefix, suffix := make([]int, n), make([]int, n)
	prefix[0], suffix[n-1] = 1, 1
	for i := 1; i < n; i++ {
		prefix[i] = prefix[i-1] * nums[i-1]
	}
	for i := n - 2; i >= 0; i-- {
		suffix[i] = suffix[i+1] * nums[i+1]
	}
	res := make([]int, n)
	for i := range nums {
		res[i] = prefix[i] * suffix[i]
	}
	return res
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: In-Place Prefix and Running Suffix (Optimal)
```go
func productExceptSelf(nums []int) []int {
	n := len(nums)
	res := make([]int, n)
	res[0] = 1
	for i := 1; i < n; i++ {
		res[i] = res[i-1] * nums[i-1]
	}
	suffix := 1
	for i := n - 1; i >= 0; i-- {
		res[i] *= suffix
		suffix *= nums[i]
	}
	return res
}
```
**Complexity:** Time O(n) · Space O(1) extra (the output doesn't count)

---

### 36. Valid Sudoku
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/valid-sudoku/)

**Problem:** Check whether a partially filled 9×9 Sudoku board is valid. Each row, each column, and each 3×3 sub-box must have no repeated digits 1–9. Empty cells are `'.'`. The board doesn't need to be solvable.

**Key insight:** Track seen digits per row, per column, and per box. Box index = `(r/3)*3 + c/3`.

#### Approach 1: Boolean Tables
```go
func isValidSudoku(board [][]byte) bool {
	var rows, cols, boxes [9][9]bool
	for r := 0; r < 9; r++ {
		for c := 0; c < 9; c++ {
			if board[r][c] == '.' {
				continue
			}
			d := board[r][c] - '1'
			b := (r/3)*3 + c/3
			if rows[r][d] || cols[c][d] || boxes[b][d] {
				return false
			}
			rows[r][d], cols[c][d], boxes[b][d] = true, true, true
		}
	}
	return true
}
```
**Complexity:** Time O(81) = O(1) · Space O(1)

#### Approach 2: Bitmasks
```go
func isValidSudokuBits(board [][]byte) bool {
	var rows, cols, boxes [9]uint16
	for r := 0; r < 9; r++ {
		for c := 0; c < 9; c++ {
			if board[r][c] == '.' {
				continue
			}
			bit := uint16(1) << (board[r][c] - '1')
			b := (r/3)*3 + c/3
			if rows[r]&bit != 0 || cols[c]&bit != 0 || boxes[b]&bit != 0 {
				return false
			}
			rows[r] |= bit
			cols[c] |= bit
			boxes[b] |= bit
		}
	}
	return true
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
```go
func longestConsecutiveSort(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	sort.Ints(nums)
	best, cur := 1, 1
	for i := 1; i < len(nums); i++ {
		if nums[i] == nums[i-1] {
			continue
		}
		if nums[i] == nums[i-1]+1 {
			cur++
		} else {
			cur = 1
		}
		best = max(best, cur)
	}
	return best
}
```
**Complexity:** Time O(n log n) · Space O(1)

#### Approach 2: Hash Set (Optimal)
```go
func longestConsecutive(nums []int) int {
	set := make(map[int]struct{}, len(nums))
	for _, n := range nums {
		set[n] = struct{}{}
	}
	best := 0
	for n := range set {
		if _, ok := set[n-1]; ok {
			continue // not the start of a run
		}
		length := 1
		for {
			if _, ok := set[n+length]; !ok {
				break
			}
			length++
		}
		best = max(best, length)
	}
	return best
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
```go
func isPalindromeClean(s string) bool {
	clean := make([]byte, 0, len(s))
	for i := 0; i < len(s); i++ {
		c := s[i]
		if c >= 'A' && c <= 'Z' {
			c += 'a' - 'A'
		}
		if (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') {
			clean = append(clean, c)
		}
	}
	for i, j := 0, len(clean)-1; i < j; i, j = i+1, j-1 {
		if clean[i] != clean[j] {
			return false
		}
	}
	return true
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Two Pointers In Place (Optimal)
```go
func isAlnum(c byte) bool {
	return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9')
}

func toLower(c byte) byte {
	if c >= 'A' && c <= 'Z' {
		return c + 'a' - 'A'
	}
	return c
}

func isPalindrome(s string) bool {
	l, r := 0, len(s)-1
	for l < r {
		for l < r && !isAlnum(s[l]) {
			l++
		}
		for l < r && !isAlnum(s[r]) {
			r--
		}
		if toLower(s[l]) != toLower(s[r]) {
			return false
		}
		l++
		r--
	}
	return true
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
```go
func twoSumIIBinary(numbers []int, target int) []int {
	for i := 0; i < len(numbers); i++ {
		want := target - numbers[i]
		lo, hi := i+1, len(numbers)-1
		for lo <= hi {
			mid := lo + (hi-lo)/2
			if numbers[mid] == want {
				return []int{i + 1, mid + 1}
			} else if numbers[mid] < want {
				lo = mid + 1
			} else {
				hi = mid - 1
			}
		}
	}
	return nil
}
```
**Complexity:** Time O(n log n) · Space O(1)

#### Approach 2: Two Pointers (Optimal)
```go
func twoSum(numbers []int, target int) []int {
	l, r := 0, len(numbers)-1
	for l < r {
		sum := numbers[l] + numbers[r]
		switch {
		case sum == target:
			return []int{l + 1, r + 1}
		case sum < target:
			l++
		default:
			r--
		}
	}
	return nil
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 15. 3Sum
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/3sum/)

**Problem:** Given an integer array `nums`, return every unique triplet `[a, b, c]` with distinct indices such that `a + b + c == 0`. The result must not contain duplicate triplets.

**Example:** `[-1,0,1,2,-1,-4]` → `[[-1,-1,2],[-1,0,1]]`

**Key insight:** Sort the array, fix the first element, then run Two Sum II on the rest. Skip equal values to avoid duplicate triplets.

#### Approach 1: Hash Set per Anchor
```go
func threeSumHash(nums []int) [][]int {
	sort.Ints(nums)
	res := [][]int{}
	for i := 0; i < len(nums); i++ {
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}
		seen := map[int]bool{}
		for j := i + 1; j < len(nums); j++ {
			need := -nums[i] - nums[j]
			if seen[need] {
				res = append(res, []int{nums[i], need, nums[j]})
				for j+1 < len(nums) && nums[j] == nums[j+1] {
					j++
				}
			}
			seen[nums[j]] = true
		}
	}
	return res
}
```
**Complexity:** Time O(n²) · Space O(n)

#### Approach 2: Sort + Two Pointers (Optimal)
```go
func threeSum(nums []int) [][]int {
	sort.Ints(nums)
	res := [][]int{}
	for i := 0; i < len(nums)-2; i++ {
		if nums[i] > 0 {
			break // the smallest value is positive, so no triplet can sum to zero
		}
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}
		l, r := i+1, len(nums)-1
		for l < r {
			sum := nums[i] + nums[l] + nums[r]
			if sum < 0 {
				l++
			} else if sum > 0 {
				r--
			} else {
				res = append(res, []int{nums[i], nums[l], nums[r]})
				l++
				r--
				for l < r && nums[l] == nums[l-1] {
					l++
				}
			}
		}
	}
	return res
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
```go
func maxAreaBrute(height []int) int {
	best := 0
	for i := 0; i < len(height); i++ {
		for j := i + 1; j < len(height); j++ {
			best = max(best, (j-i)*min(height[i], height[j]))
		}
	}
	return best
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Two Pointers (Optimal)
```go
func maxArea(height []int) int {
	l, r, best := 0, len(height)-1, 0
	for l < r {
		best = max(best, (r-l)*min(height[l], height[r]))
		if height[l] < height[r] {
			l++
		} else {
			r--
		}
	}
	return best
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
```go
func trapArrays(height []int) int {
	n := len(height)
	if n == 0 {
		return 0
	}
	left, right := make([]int, n), make([]int, n)
	left[0] = height[0]
	for i := 1; i < n; i++ {
		left[i] = max(left[i-1], height[i])
	}
	right[n-1] = height[n-1]
	for i := n - 2; i >= 0; i-- {
		right[i] = max(right[i+1], height[i])
	}
	water := 0
	for i := 0; i < n; i++ {
		water += min(left[i], right[i]) - height[i]
	}
	return water
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Monotonic Stack
```go
func trapStack(height []int) int {
	stack := []int{} // indices with decreasing heights
	water := 0
	for i, h := range height {
		for len(stack) > 0 && h > height[stack[len(stack)-1]] {
			bottom := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			if len(stack) == 0 {
				break
			}
			left := stack[len(stack)-1]
			width := i - left - 1
			bounded := min(height[left], h) - height[bottom]
			water += width * bounded
		}
		stack = append(stack, i)
	}
	return water
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 3: Two Pointers (Optimal)
```go
func trap(height []int) int {
	l, r := 0, len(height)-1
	leftMax, rightMax, water := 0, 0, 0
	for l < r {
		if height[l] < height[r] {
			leftMax = max(leftMax, height[l])
			water += leftMax - height[l]
			l++
		} else {
			rightMax = max(rightMax, height[r])
			water += rightMax - height[r]
			r--
		}
	}
	return water
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
```go
func maxProfitBrute(prices []int) int {
	best := 0
	for i := 0; i < len(prices); i++ {
		for j := i + 1; j < len(prices); j++ {
			best = max(best, prices[j]-prices[i])
		}
	}
	return best
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: One Pass with Running Min (Optimal)
```go
func maxProfit(prices []int) int {
	minPrice, best := prices[0], 0
	for _, p := range prices[1:] {
		best = max(best, p-minPrice)
		minPrice = min(minPrice, p)
	}
	return best
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
```go
func lengthOfLongestSubstringSet(s string) int {
	inWin := map[byte]bool{}
	l, best := 0, 0
	for r := 0; r < len(s); r++ {
		for inWin[s[r]] {
			delete(inWin, s[l])
			l++
		}
		inWin[s[r]] = true
		best = max(best, r-l+1)
	}
	return best
}
```
**Complexity:** Time O(n) (each character enters and leaves once) · Space O(min(n, alphabet))

#### Approach 2: Last-Seen Index Jump (Optimal)
```go
func lengthOfLongestSubstring(s string) int {
	var last [256]int // stores index+1; 0 means not seen
	l, best := 0, 0
	for r := 0; r < len(s); r++ {
		if last[s[r]] > l {
			l = last[s[r]]
		}
		last[s[r]] = r + 1
		best = max(best, r-l+1)
	}
	return best
}
```
**Complexity:** Time O(n) · Space O(1) (256 slots)

---

### 424. Longest Repeating Character Replacement
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-repeating-character-replacement/)

**Problem:** Given an uppercase string `s` and an integer `k`, you may change up to `k` characters to any other uppercase letter. Return the length of the longest substring containing a single repeated letter after the changes.

**Example:** `s = "AABABBA", k = 1` → `4`

**Key insight:** A window is valid when `windowLen - maxFreq <= k`. `maxFreq` never needs to decrease, because only a larger `maxFreq` can produce a longer answer.

#### Approach 1: Recompute Max Frequency (O(26) per step)
```go
func characterReplacementScan(s string, k int) int {
	var count [26]int
	l, best := 0, 0
	for r := 0; r < len(s); r++ {
		count[s[r]-'A']++
		for {
			mx := 0
			for _, c := range count {
				mx = max(mx, c)
			}
			if r-l+1-mx <= k {
				break
			}
			count[s[l]-'A']--
			l++
		}
		best = max(best, r-l+1)
	}
	return best
}
```
**Complexity:** Time O(26 · n) · Space O(1)

#### Approach 2: Running Max Frequency (Optimal)
```go
func characterReplacement(s string, k int) int {
	var count [26]int
	l, maxFreq, best := 0, 0, 0
	for r := 0; r < len(s); r++ {
		count[s[r]-'A']++
		maxFreq = max(maxFreq, count[s[r]-'A'])
		if r-l+1-maxFreq > k {
			count[s[l]-'A']--
			l++
		}
		best = max(best, r-l+1)
	}
	return best
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
```go
func checkInclusionCompare(s1 string, s2 string) bool {
	if len(s1) > len(s2) {
		return false
	}
	var need, win [26]int
	for i := 0; i < len(s1); i++ {
		need[s1[i]-'a']++
		win[s2[i]-'a']++
	}
	if need == win { // Go arrays are comparable
		return true
	}
	for r := len(s1); r < len(s2); r++ {
		win[s2[r]-'a']++
		win[s2[r-len(s1)]-'a']--
		if need == win {
			return true
		}
	}
	return false
}
```
**Complexity:** Time O(26 · n) · Space O(1)

#### Approach 2: Track the Number of Matching Letters (Optimal)
```go
func checkInclusion(s1 string, s2 string) bool {
	if len(s1) > len(s2) {
		return false
	}
	var c1, c2 [26]int
	for i := 0; i < len(s1); i++ {
		c1[s1[i]-'a']++
		c2[s2[i]-'a']++
	}
	matches := 0
	for i := 0; i < 26; i++ {
		if c1[i] == c2[i] {
			matches++
		}
	}
	update := func(idx, delta int) {
		if c1[idx] == c2[idx] {
			matches--
		}
		c2[idx] += delta
		if c1[idx] == c2[idx] {
			matches++
		}
	}
	for r := len(s1); r < len(s2); r++ {
		if matches == 26 {
			return true
		}
		update(int(s2[r]-'a'), 1)
		update(int(s2[r-len(s1)]-'a'), -1)
	}
	return matches == 26
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
```go
func minWindow(s string, t string) string {
	if len(t) == 0 || len(t) > len(s) {
		return ""
	}
	var need, win [128]int
	required := 0
	for i := 0; i < len(t); i++ {
		if need[t[i]] == 0 {
			required++
		}
		need[t[i]]++
	}
	have, l := 0, 0
	bestL, bestLen := 0, len(s)+1
	for r := 0; r < len(s); r++ {
		c := s[r]
		win[c]++
		if need[c] > 0 && win[c] == need[c] {
			have++
		}
		for have == required {
			if r-l+1 < bestLen {
				bestL, bestLen = l, r-l+1
			}
			lc := s[l]
			win[lc]--
			if need[lc] > 0 && win[lc] < need[lc] {
				have--
			}
			l++
		}
	}
	if bestLen > len(s) {
		return ""
	}
	return s[bestL : bestL+bestLen]
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
```go
type idxHeap struct {
	idx  []int
	nums []int
}

func (h idxHeap) Len() int           { return len(h.idx) }
func (h idxHeap) Less(i, j int) bool { return h.nums[h.idx[i]] > h.nums[h.idx[j]] }
func (h idxHeap) Swap(i, j int)      { h.idx[i], h.idx[j] = h.idx[j], h.idx[i] }
func (h *idxHeap) Push(x any)        { h.idx = append(h.idx, x.(int)) }
func (h *idxHeap) Pop() any {
	x := h.idx[len(h.idx)-1]
	h.idx = h.idx[:len(h.idx)-1]
	return x
}

func maxSlidingWindowHeap(nums []int, k int) []int {
	h := &idxHeap{nums: nums}
	res := []int{}
	for i := range nums {
		heap.Push(h, i)
		if i >= k-1 {
			for h.idx[0] <= i-k { // top has left the window
				heap.Pop(h)
			}
			res = append(res, nums[h.idx[0]])
		}
	}
	return res
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Monotonic Deque (Optimal)
```go
func maxSlidingWindow(nums []int, k int) []int {
	dq := []int{} // indices; their nums values are decreasing
	res := make([]int, 0, len(nums)-k+1)
	for i, n := range nums {
		if len(dq) > 0 && dq[0] <= i-k {
			dq = dq[1:]
		}
		for len(dq) > 0 && nums[dq[len(dq)-1]] < n {
			dq = dq[:len(dq)-1]
		}
		dq = append(dq, i)
		if i >= k-1 {
			res = append(res, nums[dq[0]])
		}
	}
	return res
}
```
**Complexity:** Time O(n) (each index is pushed and popped at most once) · Space O(k)

---
## 4. Stack

### 20. Valid Parentheses
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/valid-parentheses/)

**Problem:** Given a string made only of `()[]{}`, decide whether it is valid. Every opening bracket must be closed by the same type of bracket, in the correct order.

**Example:** `"()[]{}"` → `true`; `"(]"` → `false`; `"([)]"` → `false`

**Key insight:** The most recent unmatched opening bracket must be the first one closed, which is last-in, first-out.

#### Approach 1: Stack (Optimal)
```go
func isValid(s string) bool {
	pairs := map[byte]byte{')': '(', ']': '[', '}': '{'}
	stack := []byte{}
	for i := 0; i < len(s); i++ {
		c := s[i]
		if open, isClose := pairs[c]; isClose {
			if len(stack) == 0 || stack[len(stack)-1] != open {
				return false
			}
			stack = stack[:len(stack)-1]
		} else {
			stack = append(stack, c)
		}
	}
	return len(stack) == 0
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Repeated Replacement (simple but slow)
```go
func isValidReplace(s string) bool {
	for {
		next := strings.NewReplacer("()", "", "[]", "", "{}", "").Replace(s)
		if next == s {
			return s == ""
		}
		s = next
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
```go
type MinStack struct {
	data [][2]int // {value, min at this depth}
}

func Constructor() MinStack { return MinStack{} }

func (s *MinStack) Push(val int) {
	m := val
	if len(s.data) > 0 {
		m = min(m, s.data[len(s.data)-1][1])
	}
	s.data = append(s.data, [2]int{val, m})
}

func (s *MinStack) Pop()        { s.data = s.data[:len(s.data)-1] }
func (s *MinStack) Top() int    { return s.data[len(s.data)-1][0] }
func (s *MinStack) GetMin() int { return s.data[len(s.data)-1][1] }
```
**Complexity:** O(1) per operation · Space O(n)

#### Approach 2: Two Stacks (min stack only grows on a new min)
```go
type MinStack2 struct {
	stack, mins []int
}

func Constructor2() MinStack2 { return MinStack2{} }

func (s *MinStack2) Push(val int) {
	s.stack = append(s.stack, val)
	if len(s.mins) == 0 || val <= s.mins[len(s.mins)-1] {
		s.mins = append(s.mins, val)
	}
}

func (s *MinStack2) Pop() {
	top := s.stack[len(s.stack)-1]
	s.stack = s.stack[:len(s.stack)-1]
	if top == s.mins[len(s.mins)-1] {
		s.mins = s.mins[:len(s.mins)-1]
	}
}

func (s *MinStack2) Top() int    { return s.stack[len(s.stack)-1] }
func (s *MinStack2) GetMin() int { return s.mins[len(s.mins)-1] }
```
**Complexity:** O(1) per operation · Space O(n), usually less in practice

---

### 150. Evaluate Reverse Polish Notation
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/evaluate-reverse-polish-notation/)

**Problem:** Evaluate an arithmetic expression written in Reverse Polish (postfix) notation. The operators are `+ - * /`, and division truncates toward zero.

**Example:** `["2","1","+","3","*"]` → `9`; `["4","13","5","/","+"]` → `6`

**Key insight:** Push operands. On an operator, pop two values (second operand first) and push the result.

#### Approach: Stack (Optimal)
```go
func evalRPN(tokens []string) int {
	stack := []int{}
	for _, tok := range tokens {
		switch tok {
		case "+", "-", "*", "/":
			b, a := stack[len(stack)-1], stack[len(stack)-2]
			stack = stack[:len(stack)-2]
			var v int
			switch tok {
			case "+":
				v = a + b
			case "-":
				v = a - b
			case "*":
				v = a * b
			case "/":
				v = a / b // Go integer division already truncates toward zero
			}
			stack = append(stack, v)
		default:
			n, _ := strconv.Atoi(tok)
			stack = append(stack, n)
		}
	}
	return stack[0]
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
```go
func generateParenthesis(n int) []string {
	res := []string{}
	buf := make([]byte, 0, 2*n)
	var bt func(open, close int)
	bt = func(open, close int) {
		if len(buf) == 2*n {
			res = append(res, string(buf))
			return
		}
		if open < n {
			buf = append(buf, '(')
			bt(open+1, close)
			buf = buf[:len(buf)-1]
		}
		if close < open {
			buf = append(buf, ')')
			bt(open, close+1)
			buf = buf[:len(buf)-1]
		}
	}
	bt(0, 0)
	return res
}
```
**Complexity:** Time O(4ⁿ / √n), the nth Catalan number of results × length · Space O(n) recursion

#### Approach 2: Dynamic Programming (closure number)
Every valid string has the form `"(" + a + ")" + b`, where `a` has `i` pairs and `b` has `n-1-i` pairs.
```go
func generateParenthesisDP(n int) []string {
	dp := make([][]string, n+1)
	dp[0] = []string{""}
	for k := 1; k <= n; k++ {
		for i := 0; i < k; i++ {
			for _, a := range dp[i] {
				for _, b := range dp[k-1-i] {
					dp[k] = append(dp[k], "("+a+")"+b)
				}
			}
		}
	}
	return dp[n]
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
```go
func dailyTemperatures(temperatures []int) []int {
	res := make([]int, len(temperatures))
	stack := []int{} // indices
	for i, t := range temperatures {
		for len(stack) > 0 && temperatures[stack[len(stack)-1]] < t {
			j := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			res[j] = i - j
		}
		stack = append(stack, i)
	}
	return res
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Backward Jumps (O(1) extra space)
Scan from the right and reuse `res` to hop forward to the next warmer candidate.
```go
func dailyTemperaturesJump(temperatures []int) []int {
	n := len(temperatures)
	res := make([]int, n)
	hottest := 0
	for i := n - 1; i >= 0; i-- {
		t := temperatures[i]
		if t >= hottest {
			hottest = t
			continue
		}
		days := 1
		for temperatures[i+days] <= t {
			days += res[i+days]
		}
		res[i] = days
	}
	return res
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
```go
func carFleet(target int, position []int, speed []int) int {
	n := len(position)
	idx := make([]int, n)
	for i := range idx {
		idx[i] = i
	}
	sort.Slice(idx, func(a, b int) bool { return position[idx[a]] > position[idx[b]] })
	stack := []float64{}
	for _, i := range idx {
		t := float64(target-position[i]) / float64(speed[i])
		if len(stack) == 0 || t > stack[len(stack)-1] {
			stack = append(stack, t) // too slow to catch the fleet ahead, so a new fleet
		}
	}
	return len(stack)
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sort + Single Variable
```go
func carFleetVar(target int, position []int, speed []int) int {
	type car struct{ pos, spd int }
	cars := make([]car, len(position))
	for i := range position {
		cars[i] = car{position[i], speed[i]}
	}
	sort.Slice(cars, func(a, b int) bool { return cars[a].pos > cars[b].pos })
	fleets, slowest := 0, 0.0
	for _, c := range cars {
		t := float64(target-c.pos) / float64(c.spd)
		if t > slowest {
			fleets++
			slowest = t
		}
	}
	return fleets
}
```
**Complexity:** Time O(n log n) · Space O(n) for the sorted copy

---

### 84. Largest Rectangle in Histogram
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/largest-rectangle-in-histogram/)

**Problem:** Given bar heights `heights`, each bar having width 1, return the area of the largest rectangle that fits in the histogram.

**Example:** `[2,1,5,6,2,3]` → `10` (bars 5 and 6, width 2)

**Key insight:** When a shorter bar arrives, each taller bar popped from an increasing stack has found its right boundary. Its left boundary is the new stack top.

#### Approach 1: Brute Force (expand each bar)
```go
func largestRectangleAreaBrute(heights []int) int {
	best := 0
	for i := range heights {
		minH := heights[i]
		for j := i; j < len(heights); j++ {
			minH = min(minH, heights[j])
			best = max(best, minH*(j-i+1))
		}
	}
	return best
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Monotonic Increasing Stack (Optimal)
```go
func largestRectangleArea(heights []int) int {
	stack := []int{} // indices of increasing heights
	best := 0
	n := len(heights)
	for i := 0; i <= n; i++ {
		h := 0 // sentinel height 0 at i == n flushes the stack
		if i < n {
			h = heights[i]
		}
		for len(stack) > 0 && heights[stack[len(stack)-1]] >= h {
			top := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			left := -1
			if len(stack) > 0 {
				left = stack[len(stack)-1]
			}
			best = max(best, heights[top]*(i-left-1))
		}
		stack = append(stack, i)
	}
	return best
}
```
**Complexity:** Time O(n) · Space O(n)

---
## 5. Binary Search

### 704. Binary Search
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/binary-search/)

**Problem:** Given a sorted array `nums` of distinct integers and a `target`, return the target's index, or `-1` if it's not present. Run in O(log n).

**Example:** `nums = [-1,0,3,5,9,12], target = 9` → `4`

**Key insight:** Halve the search space each step. Use `mid := lo + (hi-lo)/2` so the midpoint can't overflow.

#### Approach 1: Iterative (Optimal)
```go
func search(nums []int, target int) int {
	lo, hi := 0, len(nums)-1
	for lo <= hi {
		mid := lo + (hi-lo)/2
		switch {
		case nums[mid] == target:
			return mid
		case nums[mid] < target:
			lo = mid + 1
		default:
			hi = mid - 1
		}
	}
	return -1
}
```
**Complexity:** Time O(log n) · Space O(1)

#### Approach 2: Recursive
```go
func searchRec(nums []int, target int) int {
	var rec func(lo, hi int) int
	rec = func(lo, hi int) int {
		if lo > hi {
			return -1
		}
		mid := lo + (hi-lo)/2
		if nums[mid] == target {
			return mid
		} else if nums[mid] < target {
			return rec(mid+1, hi)
		}
		return rec(lo, mid-1)
	}
	return rec(0, len(nums)-1)
}
```
**Complexity:** Time O(log n) · Space O(log n) recursion

#### Approach 3: Standard Library
```go
func searchStd(nums []int, target int) int {
	i := sort.SearchInts(nums, target) // lower bound
	if i < len(nums) && nums[i] == target {
		return i
	}
	return -1
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
```go
func searchMatrix(matrix [][]int, target int) bool {
	m, n := len(matrix), len(matrix[0])
	lo, hi := 0, m*n-1
	for lo <= hi {
		mid := lo + (hi-lo)/2
		v := matrix[mid/n][mid%n]
		if v == target {
			return true
		} else if v < target {
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	return false
}
```
**Complexity:** Time O(log(m·n)) · Space O(1)

#### Approach 2: Two Binary Searches (row, then column)
```go
func searchMatrixTwoPass(matrix [][]int, target int) bool {
	top, bot := 0, len(matrix)-1
	for top <= bot {
		mid := (top + bot) / 2
		if target < matrix[mid][0] {
			bot = mid - 1
		} else if target > matrix[mid][len(matrix[mid])-1] {
			top = mid + 1
		} else {
			row := matrix[mid]
			i := sort.SearchInts(row, target)
			return i < len(row) && row[i] == target
		}
	}
	return false
}
```
**Complexity:** Time O(log m + log n) · Space O(1)

#### Approach 3: Staircase from the Top-Right (also works for LC 240)
```go
func searchMatrixStaircase(matrix [][]int, target int) bool {
	r, c := 0, len(matrix[0])-1
	for r < len(matrix) && c >= 0 {
		if matrix[r][c] == target {
			return true
		} else if matrix[r][c] > target {
			c--
		} else {
			r++
		}
	}
	return false
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
```go
func minEatingSpeed(piles []int, h int) int {
	lo, hi := 1, 0
	for _, p := range piles {
		hi = max(hi, p)
	}
	for lo < hi {
		k := lo + (hi-lo)/2
		hours := 0
		for _, p := range piles {
			hours += (p + k - 1) / k // ceil(p / k)
		}
		if hours <= h {
			hi = k // k works; try smaller
		} else {
			lo = k + 1
		}
	}
	return lo
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
```go
func findMin(nums []int) int {
	lo, hi := 0, len(nums)-1
	for lo < hi {
		mid := lo + (hi-lo)/2
		if nums[mid] > nums[hi] {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return nums[lo]
}
```
**Complexity:** Time O(log n) · Space O(1)

**Linear alternative:** `slices.Min(nums)` is O(n). Fine for a sanity check, but it doesn't meet the requirement.

---

### 33. Search in Rotated Sorted Array
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/search-in-rotated-sorted-array/)

**Problem:** Search for `target` in a rotated sorted array of distinct values. Return its index or `-1`, in O(log n).

**Example:** `nums = [4,5,6,7,0,1,2], target = 0` → `4`

**Key insight:** At every `mid`, at least one half is sorted. Check whether `target` falls inside the sorted half's range.

#### Approach 1: One-Pass Binary Search (Optimal)
```go
func searchRotated(nums []int, target int) int {
	lo, hi := 0, len(nums)-1
	for lo <= hi {
		mid := lo + (hi-lo)/2
		if nums[mid] == target {
			return mid
		}
		if nums[lo] <= nums[mid] { // left half sorted
			if nums[lo] <= target && target < nums[mid] {
				hi = mid - 1
			} else {
				lo = mid + 1
			}
		} else { // right half sorted
			if nums[mid] < target && target <= nums[hi] {
				lo = mid + 1
			} else {
				hi = mid - 1
			}
		}
	}
	return -1
}
```
**Complexity:** Time O(log n) · Space O(1)

#### Approach 2: Find the Pivot, Then Binary Search the Correct Half
```go
func searchRotatedPivot(nums []int, target int) int {
	n := len(nums)
	lo, hi := 0, n-1
	for lo < hi { // find the index of the minimum
		mid := (lo + hi) / 2
		if nums[mid] > nums[hi] {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	pivot := lo
	bs := func(l, r int) int {
		for l <= r {
			m := (l + r) / 2
			if nums[m] == target {
				return m
			} else if nums[m] < target {
				l = m + 1
			} else {
				r = m - 1
			}
		}
		return -1
	}
	if pivot > 0 && target >= nums[0] {
		return bs(0, pivot-1)
	}
	return bs(pivot, n-1)
}
```
**Complexity:** Time O(log n) · Space O(1)

---

### 981. Time Based Key-Value Store
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/time-based-key-value-store/)

**Problem:** Design `TimeMap` with `set(key, value, timestamp)` and `get(key, timestamp)`. `get` returns the value stored at the largest timestamp ≤ `timestamp`, or `""` if none. Calls to `set` arrive with strictly increasing timestamps.

**Key insight:** Each key's entries are already sorted by time, so `get` is an upper-bound binary search.

#### Approach: Map of Sorted Slices + Binary Search (Optimal)
```go
type entry struct {
	ts  int
	val string
}

type TimeMap struct {
	store map[string][]entry
}

func Constructor() TimeMap {
	return TimeMap{store: map[string][]entry{}}
}

func (t *TimeMap) Set(key string, value string, timestamp int) {
	t.store[key] = append(t.store[key], entry{timestamp, value})
}

func (t *TimeMap) Get(key string, timestamp int) string {
	list := t.store[key]
	// first index with ts > timestamp
	i := sort.Search(len(list), func(i int) bool { return list[i].ts > timestamp })
	if i == 0 {
		return ""
	}
	return list[i-1].val
}
```
**Complexity:** `Set` O(1) amortized · `Get` O(log n) · Space O(total entries)

---

### 4. Median of Two Sorted Arrays
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/median-of-two-sorted-arrays/)

**Problem:** Given sorted arrays `nums1` (size m) and `nums2` (size n), return the median of their combined elements in O(log(m+n)).

**Example:** `[1,3]`, `[2]` → `2.0`; `[1,2]`, `[3,4]` → `2.5`

**Key insight:** Binary search a cut in the shorter array so the left halves of both arrays together hold half the elements, and every element on the left is ≤ every element on the right.

#### Approach 1: Merge (simple)
```go
func findMedianSortedArraysMerge(nums1 []int, nums2 []int) float64 {
	merged := make([]int, 0, len(nums1)+len(nums2))
	i, j := 0, 0
	for i < len(nums1) && j < len(nums2) {
		if nums1[i] <= nums2[j] {
			merged = append(merged, nums1[i])
			i++
		} else {
			merged = append(merged, nums2[j])
			j++
		}
	}
	merged = append(merged, nums1[i:]...)
	merged = append(merged, nums2[j:]...)
	t := len(merged)
	if t%2 == 1 {
		return float64(merged[t/2])
	}
	return float64(merged[t/2-1]+merged[t/2]) / 2.0
}
```
**Complexity:** Time O(m + n) · Space O(m + n)

#### Approach 2: Binary Search Partition (Optimal)
```go
func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	a, b := nums1, nums2
	if len(a) > len(b) {
		a, b = b, a // binary search the shorter array
	}
	m, n := len(a), len(b)
	half := (m + n + 1) / 2
	lo, hi := 0, m
	for lo <= hi {
		i := (lo + hi) / 2 // elements taken from a
		j := half - i      // elements taken from b
		aLeft, aRight := math.MinInt, math.MaxInt
		bLeft, bRight := math.MinInt, math.MaxInt
		if i > 0 {
			aLeft = a[i-1]
		}
		if i < m {
			aRight = a[i]
		}
		if j > 0 {
			bLeft = b[j-1]
		}
		if j < n {
			bRight = b[j]
		}
		if aLeft <= bRight && bLeft <= aRight {
			if (m+n)%2 == 1 {
				return float64(max(aLeft, bLeft))
			}
			return float64(max(aLeft, bLeft)+min(aRight, bRight)) / 2.0
		} else if aLeft > bRight {
			hi = i - 1
		} else {
			lo = i + 1
		}
	}
	return 0
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
```go
func reverseList(head *ListNode) *ListNode {
	var prev *ListNode
	for cur := head; cur != nil; {
		next := cur.Next
		cur.Next = prev
		prev, cur = cur, next
	}
	return prev
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Recursive
```go
func reverseListRec(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}
	newHead := reverseListRec(head.Next)
	head.Next.Next = head
	head.Next = nil
	return newHead
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
```go
func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	dummy := &ListNode{}
	tail := dummy
	for list1 != nil && list2 != nil {
		if list1.Val <= list2.Val {
			tail.Next, list1 = list1, list1.Next
		} else {
			tail.Next, list2 = list2, list2.Next
		}
		tail = tail.Next
	}
	if list1 != nil {
		tail.Next = list1
	} else {
		tail.Next = list2
	}
	return dummy.Next
}
```
**Complexity:** Time O(n + m) · Space O(1)

#### Approach 2: Recursive
```go
func mergeTwoListsRec(l1 *ListNode, l2 *ListNode) *ListNode {
	if l1 == nil {
		return l2
	}
	if l2 == nil {
		return l1
	}
	if l1.Val <= l2.Val {
		l1.Next = mergeTwoListsRec(l1.Next, l2)
		return l1
	}
	l2.Next = mergeTwoListsRec(l1, l2.Next)
	return l2
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
```go
func reorderList(head *ListNode) {
	if head == nil || head.Next == nil {
		return
	}
	// 1. find the middle
	slow, fast := head, head.Next
	for fast != nil && fast.Next != nil {
		slow, fast = slow.Next, fast.Next.Next
	}
	// 2. reverse the second half
	second := slow.Next
	slow.Next = nil
	var prev *ListNode
	for second != nil {
		next := second.Next
		second.Next = prev
		prev, second = second, next
	}
	// 3. interleave
	first, second := head, prev
	for second != nil {
		n1, n2 := first.Next, second.Next
		first.Next = second
		second.Next = n1
		first, second = n1, n2
	}
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Array of Nodes
```go
func reorderListArray(head *ListNode) {
	nodes := []*ListNode{}
	for n := head; n != nil; n = n.Next {
		nodes = append(nodes, n)
	}
	i, j := 0, len(nodes)-1
	for i < j {
		nodes[i].Next = nodes[j]
		i++
		if i == j {
			break
		}
		nodes[j].Next = nodes[i]
		j--
	}
	nodes[i].Next = nil
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
```go
func removeNthFromEnd(head *ListNode, n int) *ListNode {
	dummy := &ListNode{Next: head}
	slow, fast := dummy, dummy
	for i := 0; i <= n; i++ {
		fast = fast.Next
	}
	for fast != nil {
		slow, fast = slow.Next, fast.Next
	}
	slow.Next = slow.Next.Next
	return dummy.Next
}
```
**Complexity:** Time O(L) · Space O(1)

#### Approach 2: Two Passes (count the length)
```go
func removeNthFromEndTwoPass(head *ListNode, n int) *ListNode {
	length := 0
	for c := head; c != nil; c = c.Next {
		length++
	}
	dummy := &ListNode{Next: head}
	cur := dummy
	for i := 0; i < length-n; i++ {
		cur = cur.Next
	}
	cur.Next = cur.Next.Next
	return dummy.Next
}
```
**Complexity:** Time O(L) · Space O(1)

---

### 138. Copy List with Random Pointer
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/copy-list-with-random-pointer/)

**Problem:** Each node has `Next` and `Random` pointers, and `Random` may point to any node or `nil`. Return a deep copy of the list.

**Key insight:** Map each original node to its copy. Alternatively, weave each copy right after its original so `orig.Random.Next` is the copied random target.

#### Approach 1: Hash Map
```go
type Node struct {
	Val    int
	Next   *Node
	Random *Node
}

func copyRandomList(head *Node) *Node {
	copies := map[*Node]*Node{nil: nil}
	for n := head; n != nil; n = n.Next {
		copies[n] = &Node{Val: n.Val}
	}
	for n := head; n != nil; n = n.Next {
		copies[n].Next = copies[n.Next]
		copies[n].Random = copies[n.Random]
	}
	return copies[head]
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Interweaving (O(1) extra space)
```go
func copyRandomListWeave(head *Node) *Node {
	if head == nil {
		return nil
	}
	// A → A' → B → B' ...
	for n := head; n != nil; n = n.Next.Next {
		n.Next = &Node{Val: n.Val, Next: n.Next}
	}
	// set random pointers on the copies
	for n := head; n != nil; n = n.Next.Next {
		if n.Random != nil {
			n.Next.Random = n.Random.Next
		}
	}
	// unweave the two lists
	newHead := head.Next
	for n := head; n != nil; n = n.Next {
		cp := n.Next
		n.Next = cp.Next
		if cp.Next != nil {
			cp.Next = cp.Next.Next
		}
	}
	return newHead
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
```go
func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	dummy := &ListNode{}
	tail, carry := dummy, 0
	for l1 != nil || l2 != nil || carry != 0 {
		sum := carry
		if l1 != nil {
			sum += l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			sum += l2.Val
			l2 = l2.Next
		}
		carry = sum / 10
		tail.Next = &ListNode{Val: sum % 10}
		tail = tail.Next
	}
	return dummy.Next
}
```
**Complexity:** Time O(max(m, n)) · Space O(1) extra (the output doesn't count)

#### Approach 2: Recursive
```go
func addTwoNumbersRec(l1 *ListNode, l2 *ListNode) *ListNode {
	var rec func(a, b *ListNode, carry int) *ListNode
	rec = func(a, b *ListNode, carry int) *ListNode {
		if a == nil && b == nil && carry == 0 {
			return nil
		}
		sum := carry
		var an, bn *ListNode
		if a != nil {
			sum += a.Val
			an = a.Next
		}
		if b != nil {
			sum += b.Val
			bn = b.Next
		}
		return &ListNode{Val: sum % 10, Next: rec(an, bn, sum/10)}
	}
	return rec(l1, l2, 0)
}
```
**Complexity:** Time O(max(m, n)) · Space O(max(m, n)) recursion

---

### 141. Linked List Cycle
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/linked-list-cycle/)

**Problem:** Return `true` if the linked list contains a cycle.

**Key insight:** Floyd's tortoise and hare. If there's a cycle, the fast pointer eventually laps the slow one.

#### Approach 1: Hash Set
```go
func hasCycleSet(head *ListNode) bool {
	seen := map[*ListNode]bool{}
	for n := head; n != nil; n = n.Next {
		if seen[n] {
			return true
		}
		seen[n] = true
	}
	return false
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Floyd's Fast and Slow Pointers (Optimal)
```go
func hasCycle(head *ListNode) bool {
	slow, fast := head, head
	for fast != nil && fast.Next != nil {
		slow, fast = slow.Next, fast.Next.Next
		if slow == fast {
			return true
		}
	}
	return false
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 287. Find the Duplicate Number
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/find-the-duplicate-number/)

**Problem:** `nums` has `n + 1` integers, each in `[1, n]`, and exactly one value is repeated (possibly many times). Return it without modifying the array, using O(1) extra space.

**Example:** `[1,3,4,2,2]` → `2`

**Key insight:** Treat `i → nums[i]` as a linked list. The duplicate value is the entry point of the cycle, which you can find with Floyd's algorithm.

#### Approach 1: Hash Set (uses extra space)
```go
func findDuplicateSet(nums []int) int {
	seen := map[int]bool{}
	for _, n := range nums {
		if seen[n] {
			return n
		}
		seen[n] = true
	}
	return -1
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Binary Search on Value (pigeonhole)
```go
func findDuplicateBS(nums []int) int {
	lo, hi := 1, len(nums)-1
	for lo < hi {
		mid := (lo + hi) / 2
		cnt := 0
		for _, n := range nums {
			if n <= mid {
				cnt++
			}
		}
		if cnt > mid {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
}
```
**Complexity:** Time O(n log n) · Space O(1)

#### Approach 3: Floyd's Cycle Detection (Optimal)
```go
func findDuplicate(nums []int) int {
	slow, fast := nums[0], nums[nums[0]]
	for slow != fast {
		slow = nums[slow]
		fast = nums[nums[fast]]
	}
	slow = 0
	for slow != fast {
		slow = nums[slow]
		fast = nums[fast]
	}
	return slow
}
```
**Complexity:** Time O(n) · Space O(1)

---

### 146. LRU Cache
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/lru-cache/)

**Problem:** Design an LRU (least recently used) cache with a fixed `capacity`. `get(key)` returns the value or `-1`. `put(key, value)` inserts or updates a key, evicting the least recently used key when over capacity. Both must run in O(1).

**Key insight:** A hash map gives O(1) lookup, and a doubly linked list gives O(1) reordering and eviction.

#### Approach 1: Hand-Written Doubly Linked List (Optimal, interview-preferred)
```go
type dNode struct {
	key, val   int
	prev, next *dNode
}

type LRUCache struct {
	cap        int
	items      map[int]*dNode
	head, tail *dNode // sentinels: head.next is the MRU, tail.prev is the LRU
}

func Constructor(capacity int) LRUCache {
	h, t := &dNode{}, &dNode{}
	h.next, t.prev = t, h
	return LRUCache{cap: capacity, items: map[int]*dNode{}, head: h, tail: t}
}

func (c *LRUCache) remove(n *dNode) {
	n.prev.next, n.next.prev = n.next, n.prev
}

func (c *LRUCache) pushFront(n *dNode) {
	n.prev, n.next = c.head, c.head.next
	c.head.next.prev = n
	c.head.next = n
}

func (c *LRUCache) Get(key int) int {
	n, ok := c.items[key]
	if !ok {
		return -1
	}
	c.remove(n)
	c.pushFront(n)
	return n.val
}

func (c *LRUCache) Put(key int, value int) {
	if n, ok := c.items[key]; ok {
		n.val = value
		c.remove(n)
		c.pushFront(n)
		return
	}
	n := &dNode{key: key, val: value}
	c.items[key] = n
	c.pushFront(n)
	if len(c.items) > c.cap {
		lru := c.tail.prev
		c.remove(lru)
		delete(c.items, lru.key)
	}
}
```
**Complexity:** O(1) per operation · Space O(capacity)

#### Approach 2: Using `container/list`
```go
type kv struct{ key, val int }

type LRUCacheStd struct {
	cap   int
	ll    *list.List
	items map[int]*list.Element
}

func ConstructorStd(capacity int) LRUCacheStd {
	return LRUCacheStd{cap: capacity, ll: list.New(), items: map[int]*list.Element{}}
}

func (c *LRUCacheStd) Get(key int) int {
	if e, ok := c.items[key]; ok {
		c.ll.MoveToFront(e)
		return e.Value.(*kv).val
	}
	return -1
}

func (c *LRUCacheStd) Put(key, value int) {
	if e, ok := c.items[key]; ok {
		e.Value.(*kv).val = value
		c.ll.MoveToFront(e)
		return
	}
	c.items[key] = c.ll.PushFront(&kv{key, value})
	if c.ll.Len() > c.cap {
		last := c.ll.Back()
		c.ll.Remove(last)
		delete(c.items, last.Value.(*kv).key)
	}
}
```
**Complexity:** O(1) per operation · Space O(capacity)

---

### 23. Merge k Sorted Lists
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/merge-k-sorted-lists/)

**Problem:** Merge `k` sorted linked lists into one sorted list.

**Example:** `[1→4→5, 1→3→4, 2→6]` → `1→1→2→3→4→4→5→6`

**Key insight:** Either keep a min-heap of the `k` current heads, or merge the lists in pairs (divide and conquer). Both are O(N log k).

#### Approach 1: Min-Heap
```go
type nodeHeap []*ListNode

func (h nodeHeap) Len() int           { return len(h) }
func (h nodeHeap) Less(i, j int) bool { return h[i].Val < h[j].Val }
func (h nodeHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *nodeHeap) Push(x any)        { *h = append(*h, x.(*ListNode)) }
func (h *nodeHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func mergeKListsHeap(lists []*ListNode) *ListNode {
	h := &nodeHeap{}
	for _, l := range lists {
		if l != nil {
			*h = append(*h, l)
		}
	}
	heap.Init(h)
	dummy := &ListNode{}
	tail := dummy
	for h.Len() > 0 {
		n := heap.Pop(h).(*ListNode)
		tail.Next = n
		tail = n
		if n.Next != nil {
			heap.Push(h, n.Next)
		}
	}
	return dummy.Next
}
```
**Complexity:** Time O(N log k) for N total nodes · Space O(k)

#### Approach 2: Divide and Conquer Pairwise Merge (Optimal, O(1) extra)
```go
func merge2(a, b *ListNode) *ListNode {
	dummy := &ListNode{}
	t := dummy
	for a != nil && b != nil {
		if a.Val <= b.Val {
			t.Next, a = a, a.Next
		} else {
			t.Next, b = b, b.Next
		}
		t = t.Next
	}
	if a != nil {
		t.Next = a
	} else {
		t.Next = b
	}
	return dummy.Next
}

func mergeKLists(lists []*ListNode) *ListNode {
	if len(lists) == 0 {
		return nil
	}
	for step := 1; step < len(lists); step *= 2 {
		for i := 0; i+step < len(lists); i += 2 * step {
			lists[i] = merge2(lists[i], lists[i+step])
		}
	}
	return lists[0]
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
```go
func reverseKGroup(head *ListNode, k int) *ListNode {
	dummy := &ListNode{Next: head}
	groupPrev := dummy
	for {
		// find the kth node of this group
		kth := groupPrev
		for i := 0; i < k && kth != nil; i++ {
			kth = kth.Next
		}
		if kth == nil {
			break
		}
		groupNext := kth.Next
		// reverse the group
		prev, cur := groupNext, groupPrev.Next
		for cur != groupNext {
			next := cur.Next
			cur.Next = prev
			prev, cur = cur, next
		}
		first := groupPrev.Next // becomes this group's tail
		groupPrev.Next = kth
		groupPrev = first
	}
	return dummy.Next
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Recursive
```go
func reverseKGroupRec(head *ListNode, k int) *ListNode {
	cur, count := head, 0
	for cur != nil && count < k {
		cur = cur.Next
		count++
	}
	if count < k {
		return head
	}
	rest := reverseKGroupRec(cur, k) // cur is the (k+1)th node
	prev, node := rest, head
	for i := 0; i < k; i++ {
		next := node.Next
		node.Next = prev
		prev, node = node, next
	}
	return prev
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
```go
func invertTree(root *TreeNode) *TreeNode {
	if root == nil {
		return nil
	}
	root.Left, root.Right = invertTree(root.Right), invertTree(root.Left)
	return root
}
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: Iterative BFS
```go
func invertTreeBFS(root *TreeNode) *TreeNode {
	if root == nil {
		return nil
	}
	q := []*TreeNode{root}
	for len(q) > 0 {
		n := q[0]
		q = q[1:]
		n.Left, n.Right = n.Right, n.Left
		if n.Left != nil {
			q = append(q, n.Left)
		}
		if n.Right != nil {
			q = append(q, n.Right)
		}
	}
	return root
}
```
**Complexity:** Time O(n) · Space O(w), where w is the maximum width

---

### 104. Maximum Depth of Binary Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/maximum-depth-of-binary-tree/)

**Problem:** Return the number of nodes on the longest path from the root down to a leaf.

**Example:** `[3,9,20,null,null,15,7]` → `3`

#### Approach 1: Recursive DFS
```go
func maxDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}
	return 1 + max(maxDepth(root.Left), maxDepth(root.Right))
}
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: BFS Level Count
```go
func maxDepthBFS(root *TreeNode) int {
	if root == nil {
		return 0
	}
	depth, q := 0, []*TreeNode{root}
	for len(q) > 0 {
		depth++
		for sz := len(q); sz > 0; sz-- {
			n := q[0]
			q = q[1:]
			if n.Left != nil {
				q = append(q, n.Left)
			}
			if n.Right != nil {
				q = append(q, n.Right)
			}
		}
	}
	return depth
}
```
**Complexity:** Time O(n) · Space O(w)

#### Approach 3: Iterative DFS with an Explicit Stack
```go
func maxDepthStack(root *TreeNode) int {
	type item struct {
		node  *TreeNode
		depth int
	}
	best := 0
	stack := []item{{root, 1}}
	for len(stack) > 0 {
		it := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if it.node == nil {
			continue
		}
		best = max(best, it.depth)
		stack = append(stack, item{it.node.Left, it.depth + 1}, item{it.node.Right, it.depth + 1})
	}
	return best
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
```go
func diameterOfBinaryTree(root *TreeNode) int {
	best := 0
	var height func(*TreeNode) int
	height = func(n *TreeNode) int {
		if n == nil {
			return 0
		}
		l, r := height(n.Left), height(n.Right)
		best = max(best, l+r)
		return 1 + max(l, r)
	}
	height(root)
	return best
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
```go
func heightTD(n *TreeNode) int {
	if n == nil {
		return 0
	}
	return 1 + max(heightTD(n.Left), heightTD(n.Right))
}

func isBalancedTopDown(root *TreeNode) bool {
	if root == nil {
		return true
	}
	d := heightTD(root.Left) - heightTD(root.Right)
	return d >= -1 && d <= 1 && isBalancedTopDown(root.Left) && isBalancedTopDown(root.Right)
}
```
**Complexity:** Time O(n²) worst case · Space O(h)

#### Approach 2: Bottom-Up with Early Exit (Optimal)
```go
func isBalanced(root *TreeNode) bool {
	var check func(*TreeNode) int
	check = func(n *TreeNode) int {
		if n == nil {
			return 0
		}
		l := check(n.Left)
		if l == -1 {
			return -1
		}
		r := check(n.Right)
		if r == -1 || l-r > 1 || r-l > 1 {
			return -1
		}
		return 1 + max(l, r)
	}
	return check(root) != -1
}
```
**Complexity:** Time O(n) · Space O(h)

---

### 100. Same Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/same-tree/)

**Problem:** Return whether two binary trees have the same structure and the same node values.

#### Approach 1: Recursive (Optimal)
```go
func isSameTree(p *TreeNode, q *TreeNode) bool {
	if p == nil || q == nil {
		return p == q
	}
	return p.Val == q.Val && isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right)
}
```
**Complexity:** Time O(min(n, m)) · Space O(h)

#### Approach 2: Iterative BFS on Pairs
```go
func isSameTreeBFS(p *TreeNode, q *TreeNode) bool {
	queue := [][2]*TreeNode{{p, q}}
	for len(queue) > 0 {
		a, b := queue[0][0], queue[0][1]
		queue = queue[1:]
		if a == nil && b == nil {
			continue
		}
		if a == nil || b == nil || a.Val != b.Val {
			return false
		}
		queue = append(queue, [2]*TreeNode{a.Left, b.Left}, [2]*TreeNode{a.Right, b.Right})
	}
	return true
}
```
**Complexity:** Time O(n) · Space O(w)

---

### 572. Subtree of Another Tree
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/subtree-of-another-tree/)

**Problem:** Return whether `subRoot` matches, in structure and values, some node of `root` together with all of that node's descendants.

**Key insight:** Run Same Tree at every node (O(m·n)), or serialize both trees and use substring search (O(m+n)).

#### Approach 1: Same-Tree Check at Every Node
```go
func sameTree(a, b *TreeNode) bool {
	if a == nil || b == nil {
		return a == b
	}
	return a.Val == b.Val && sameTree(a.Left, b.Left) && sameTree(a.Right, b.Right)
}

func isSubtree(root *TreeNode, subRoot *TreeNode) bool {
	if root == nil {
		return subRoot == nil
	}
	return sameTree(root, subRoot) || isSubtree(root.Left, subRoot) || isSubtree(root.Right, subRoot)
}
```
**Complexity:** Time O(m · n) · Space O(h)

#### Approach 2: Serialize + Substring Search
Delimiters around each value (`^12` vs `^2`) and explicit null markers prevent false matches.
```go
func serialize572(n *TreeNode, sb *strings.Builder) {
	if n == nil {
		sb.WriteString("#")
		return
	}
	sb.WriteString("^")
	sb.WriteString(strconv.Itoa(n.Val))
	serialize572(n.Left, sb)
	serialize572(n.Right, sb)
}

func isSubtreeSerialize(root *TreeNode, subRoot *TreeNode) bool {
	var a, b strings.Builder
	serialize572(root, &a)
	serialize572(subRoot, &b)
	return strings.Contains(a.String(), b.String()) // swap in KMP for a guaranteed O(m+n)
}
```
**Complexity:** Time O(m + n) with KMP; Go's `strings.Contains` is fast in practice · Space O(m + n)

---

### 235. Lowest Common Ancestor of a Binary Search Tree
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)

**Problem:** Given a BST and two of its nodes `p` and `q`, return their lowest common ancestor: the deepest node that has both as descendants (a node counts as its own descendant).

**Key insight:** If both values are smaller than the current node, go left. If both are larger, go right. Otherwise the current node is where they split, and that's the LCA.

#### Approach 1: Iterative (Optimal)
```go
func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	cur := root
	for cur != nil {
		if p.Val < cur.Val && q.Val < cur.Val {
			cur = cur.Left
		} else if p.Val > cur.Val && q.Val > cur.Val {
			cur = cur.Right
		} else {
			return cur
		}
	}
	return nil
}
```
**Complexity:** Time O(h) · Space O(1)

#### Approach 2: Recursive
```go
func lowestCommonAncestorRec(root, p, q *TreeNode) *TreeNode {
	if p.Val < root.Val && q.Val < root.Val {
		return lowestCommonAncestorRec(root.Left, p, q)
	}
	if p.Val > root.Val && q.Val > root.Val {
		return lowestCommonAncestorRec(root.Right, p, q)
	}
	return root
}
```
**Complexity:** Time O(h) · Space O(h)

---

### 102. Binary Tree Level Order Traversal
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/binary-tree-level-order-traversal/)

**Problem:** Return the node values level by level, left to right.

**Example:** `[3,9,20,null,null,15,7]` → `[[3],[9,20],[15,7]]`

#### Approach 1: BFS with a Queue (Optimal)
```go
func levelOrder(root *TreeNode) [][]int {
	res := [][]int{}
	if root == nil {
		return res
	}
	q := []*TreeNode{root}
	for len(q) > 0 {
		level := make([]int, 0, len(q))
		for sz := len(q); sz > 0; sz-- {
			n := q[0]
			q = q[1:]
			level = append(level, n.Val)
			if n.Left != nil {
				q = append(q, n.Left)
			}
			if n.Right != nil {
				q = append(q, n.Right)
			}
		}
		res = append(res, level)
	}
	return res
}
```
**Complexity:** Time O(n) · Space O(w)

#### Approach 2: DFS with a Depth Index
```go
func levelOrderDFS(root *TreeNode) [][]int {
	res := [][]int{}
	var dfs func(*TreeNode, int)
	dfs = func(n *TreeNode, d int) {
		if n == nil {
			return
		}
		if d == len(res) {
			res = append(res, []int{})
		}
		res[d] = append(res[d], n.Val)
		dfs(n.Left, d+1)
		dfs(n.Right, d+1)
	}
	dfs(root, 0)
	return res
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
```go
func rightSideView(root *TreeNode) []int {
	res := []int{}
	if root == nil {
		return res
	}
	q := []*TreeNode{root}
	for len(q) > 0 {
		sz := len(q)
		for i := 0; i < sz; i++ {
			n := q[0]
			q = q[1:]
			if i == sz-1 {
				res = append(res, n.Val)
			}
			if n.Left != nil {
				q = append(q, n.Left)
			}
			if n.Right != nil {
				q = append(q, n.Right)
			}
		}
	}
	return res
}
```
**Complexity:** Time O(n) · Space O(w)

#### Approach 2: Right-First DFS
```go
func rightSideViewDFS(root *TreeNode) []int {
	res := []int{}
	var dfs func(*TreeNode, int)
	dfs = func(n *TreeNode, d int) {
		if n == nil {
			return
		}
		if d == len(res) {
			res = append(res, n.Val)
		}
		dfs(n.Right, d+1)
		dfs(n.Left, d+1)
	}
	dfs(root, 0)
	return res
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
```go
func goodNodes(root *TreeNode) int {
	var dfs func(*TreeNode, int) int
	dfs = func(n *TreeNode, maxSoFar int) int {
		if n == nil {
			return 0
		}
		good := 0
		if n.Val >= maxSoFar {
			good = 1
			maxSoFar = n.Val
		}
		return good + dfs(n.Left, maxSoFar) + dfs(n.Right, maxSoFar)
	}
	return dfs(root, root.Val)
}
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: BFS Carrying (node, max)
```go
func goodNodesBFS(root *TreeNode) int {
	type item struct {
		n  *TreeNode
		mx int
	}
	count := 0
	q := []item{{root, root.Val}}
	for len(q) > 0 {
		it := q[0]
		q = q[1:]
		if it.n.Val >= it.mx {
			count++
		}
		mx := max(it.mx, it.n.Val)
		if it.n.Left != nil {
			q = append(q, item{it.n.Left, mx})
		}
		if it.n.Right != nil {
			q = append(q, item{it.n.Right, mx})
		}
	}
	return count
}
```
**Complexity:** Time O(n) · Space O(w)

---

### 98. Validate Binary Search Tree
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/validate-binary-search-tree/)

**Problem:** Return whether the tree is a valid BST. Every node's left subtree must hold only smaller values, its right subtree only larger values, and both subtrees must themselves be BSTs.

**Key insight:** Comparing a node only with its direct children isn't enough. Pass down a valid `(lo, hi)` range, or check that the in-order traversal is strictly increasing.

#### Approach 1: DFS with Bounds (Optimal)
```go
func isValidBST(root *TreeNode) bool {
	var valid func(*TreeNode, int, int) bool
	valid = func(n *TreeNode, lo, hi int) bool {
		if n == nil {
			return true
		}
		if n.Val <= lo || n.Val >= hi {
			return false
		}
		return valid(n.Left, lo, n.Val) && valid(n.Right, n.Val, hi)
	}
	return valid(root, math.MinInt, math.MaxInt)
}
```
**Complexity:** Time O(n) · Space O(h)

#### Approach 2: Iterative In-Order Traversal
```go
func isValidBSTInorder(root *TreeNode) bool {
	stack := []*TreeNode{}
	var prev *TreeNode
	cur := root
	for cur != nil || len(stack) > 0 {
		for cur != nil {
			stack = append(stack, cur)
			cur = cur.Left
		}
		cur = stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if prev != nil && cur.Val <= prev.Val {
			return false
		}
		prev = cur
		cur = cur.Right
	}
	return true
}
```
**Complexity:** Time O(n) · Space O(h)

---

### 230. Kth Smallest Element in a BST
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/kth-smallest-element-in-a-bst/)

**Problem:** Return the `k`th smallest value (1-indexed) in a BST.

**Key insight:** In-order traversal visits a BST's values in sorted order, so stop at the `k`th node visited.

#### Approach 1: Iterative In-Order (Optimal, stops early)
```go
func kthSmallest(root *TreeNode, k int) int {
	stack := []*TreeNode{}
	cur := root
	for {
		for cur != nil {
			stack = append(stack, cur)
			cur = cur.Left
		}
		cur = stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		k--
		if k == 0 {
			return cur.Val
		}
		cur = cur.Right
	}
}
```
**Complexity:** Time O(h + k) · Space O(h)

#### Approach 2: Recursive In-Order with a Counter
```go
func kthSmallestRec(root *TreeNode, k int) int {
	ans := 0
	var dfs func(*TreeNode)
	dfs = func(n *TreeNode) {
		if n == nil || k == 0 {
			return
		}
		dfs(n.Left)
		k--
		if k == 0 {
			ans = n.Val
			return
		}
		dfs(n.Right)
	}
	dfs(root)
	return ans
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
```go
func buildTreeSlice(preorder []int, inorder []int) *TreeNode {
	if len(preorder) == 0 {
		return nil
	}
	root := &TreeNode{Val: preorder[0]}
	mid := 0
	for inorder[mid] != preorder[0] {
		mid++
	}
	root.Left = buildTreeSlice(preorder[1:mid+1], inorder[:mid])
	root.Right = buildTreeSlice(preorder[mid+1:], inorder[mid+1:])
	return root
}
```
**Complexity:** Time O(n²) worst case · Space O(n)

#### Approach 2: Hash Map of Inorder Indices (Optimal)
```go
func buildTree(preorder []int, inorder []int) *TreeNode {
	pos := make(map[int]int, len(inorder))
	for i, v := range inorder {
		pos[v] = i
	}
	pre := 0
	var build func(lo, hi int) *TreeNode // inorder range [lo, hi]
	build = func(lo, hi int) *TreeNode {
		if lo > hi {
			return nil
		}
		v := preorder[pre]
		pre++
		root := &TreeNode{Val: v}
		mid := pos[v]
		root.Left = build(lo, mid-1)
		root.Right = build(mid+1, hi)
		return root
	}
	return build(0, len(inorder)-1)
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
```go
func maxPathSum(root *TreeNode) int {
	best := math.MinInt
	var gain func(*TreeNode) int
	gain = func(n *TreeNode) int {
		if n == nil {
			return 0
		}
		l := max(0, gain(n.Left))
		r := max(0, gain(n.Right))
		best = max(best, n.Val+l+r)
		return n.Val + max(l, r)
	}
	gain(root)
	return best
}
```
**Complexity:** Time O(n) · Space O(h)

---

### 297. Serialize and Deserialize Binary Tree
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/)

**Problem:** Design `serialize`, which turns a binary tree into a string, and `deserialize`, which turns that string back into the same tree.

**Key insight:** A pre-order traversal with explicit null markers (`N`) is enough to rebuild the tree exactly.

#### Approach 1: Pre-Order DFS
```go
type Codec struct{}

func Constructor() Codec { return Codec{} }

func (c *Codec) serialize(root *TreeNode) string {
	parts := []string{}
	var dfs func(*TreeNode)
	dfs = func(n *TreeNode) {
		if n == nil {
			parts = append(parts, "N")
			return
		}
		parts = append(parts, strconv.Itoa(n.Val))
		dfs(n.Left)
		dfs(n.Right)
	}
	dfs(root)
	return strings.Join(parts, ",")
}

func (c *Codec) deserialize(data string) *TreeNode {
	vals := strings.Split(data, ",")
	i := 0
	var build func() *TreeNode
	build = func() *TreeNode {
		v := vals[i]
		i++
		if v == "N" {
			return nil
		}
		n, _ := strconv.Atoi(v)
		node := &TreeNode{Val: n}
		node.Left = build()
		node.Right = build()
		return node
	}
	return build()
}
```
**Complexity:** Time O(n) for both · Space O(n)

#### Approach 2: BFS (level order, LeetCode's own format)
```go
type CodecBFS struct{}

func (c *CodecBFS) serialize(root *TreeNode) string {
	if root == nil {
		return ""
	}
	parts := []string{}
	q := []*TreeNode{root}
	for len(q) > 0 {
		n := q[0]
		q = q[1:]
		if n == nil {
			parts = append(parts, "N")
			continue
		}
		parts = append(parts, strconv.Itoa(n.Val))
		q = append(q, n.Left, n.Right)
	}
	return strings.Join(parts, ",")
}

func (c *CodecBFS) deserialize(data string) *TreeNode {
	if data == "" {
		return nil
	}
	vals := strings.Split(data, ",")
	v, _ := strconv.Atoi(vals[0])
	root := &TreeNode{Val: v}
	q := []*TreeNode{root}
	i := 1
	for len(q) > 0 {
		n := q[0]
		q = q[1:]
		if vals[i] != "N" {
			x, _ := strconv.Atoi(vals[i])
			n.Left = &TreeNode{Val: x}
			q = append(q, n.Left)
		}
		i++
		if vals[i] != "N" {
			x, _ := strconv.Atoi(vals[i])
			n.Right = &TreeNode{Val: x}
			q = append(q, n.Right)
		}
		i++
	}
	return root
}
```
**Complexity:** Time O(n) · Space O(n)

---
## 8. Tries

### 208. Implement Trie (Prefix Tree)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/implement-trie-prefix-tree/)

**Problem:** Implement a trie with `Insert(word)`, `Search(word)` (is this exact word stored?), and `StartsWith(prefix)` (does any stored word begin with this prefix?). Inputs are lowercase letters.

**Key insight:** Each node has up to 26 children plus an end-of-word flag.

#### Approach 1: Fixed Array of Children (Optimal for lowercase)
```go
type Trie struct {
	children [26]*Trie
	isEnd    bool
}

func Constructor() Trie { return Trie{} }

func (t *Trie) Insert(word string) {
	node := t
	for i := 0; i < len(word); i++ {
		c := word[i] - 'a'
		if node.children[c] == nil {
			node.children[c] = &Trie{}
		}
		node = node.children[c]
	}
	node.isEnd = true
}

func (t *Trie) find(s string) *Trie {
	node := t
	for i := 0; i < len(s); i++ {
		node = node.children[s[i]-'a']
		if node == nil {
			return nil
		}
	}
	return node
}

func (t *Trie) Search(word string) bool {
	n := t.find(word)
	return n != nil && n.isEnd
}

func (t *Trie) StartsWith(prefix string) bool {
	return t.find(prefix) != nil
}
```
**Complexity:** Time O(L) per operation · Space O(total characters × 26)

#### Approach 2: Map Children (any alphabet, less memory when sparse)
```go
type TrieMap struct {
	children map[rune]*TrieMap
	isEnd    bool
}

func NewTrieMap() *TrieMap { return &TrieMap{children: map[rune]*TrieMap{}} }

func (t *TrieMap) Insert(word string) {
	node := t
	for _, r := range word {
		if node.children[r] == nil {
			node.children[r] = NewTrieMap()
		}
		node = node.children[r]
	}
	node.isEnd = true
}

func (t *TrieMap) Search(word string) bool {
	node := t
	for _, r := range word {
		if node = node.children[r]; node == nil {
			return false
		}
	}
	return node.isEnd
}
```
**Complexity:** Time O(L) average · Space O(total characters)

---

### 211. Design Add and Search Words Data Structure
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/design-add-and-search-words-data-structure/)

**Problem:** Support `AddWord(word)` and `Search(word)`. The search pattern may contain `.`, which matches any single letter.

**Example:** add `bad`, `dad`, `mad`; `search("pad")` → `false`, `search(".ad")` → `true`, `search("b..")` → `true`

**Key insight:** Use a trie. On `.`, run DFS into every child.

#### Approach: Trie + DFS (Optimal)
```go
type wdNode struct {
	children [26]*wdNode
	isEnd    bool
}

type WordDictionary struct{ root *wdNode }

func Constructor() WordDictionary { return WordDictionary{root: &wdNode{}} }

func (d *WordDictionary) AddWord(word string) {
	n := d.root
	for i := 0; i < len(word); i++ {
		c := word[i] - 'a'
		if n.children[c] == nil {
			n.children[c] = &wdNode{}
		}
		n = n.children[c]
	}
	n.isEnd = true
}

func (d *WordDictionary) Search(word string) bool {
	var dfs func(n *wdNode, i int) bool
	dfs = func(n *wdNode, i int) bool {
		if n == nil {
			return false
		}
		if i == len(word) {
			return n.isEnd
		}
		if word[i] == '.' {
			for _, ch := range n.children {
				if ch != nil && dfs(ch, i+1) {
					return true
				}
			}
			return false
		}
		return dfs(n.children[word[i]-'a'], i+1)
	}
	return dfs(d.root, 0)
}
```
**Complexity:** `AddWord` O(L) · `Search` O(L) without dots, O(26^L) worst case with all dots · Space O(total characters)

**Alternative:** Bucket words by length in a `map[int][]string` and compare character by character. Simpler, but `Search` costs O(words × L).

---

### 212. Word Search II
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/word-search-ii/)

**Problem:** Given an `m × n` letter grid and a list of `words`, return every word that can be traced through horizontally or vertically adjacent cells, without reusing a cell within a word.

**Example:** `board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]` → `["eat","oath"]`

**Key insight:** Build a trie from all the words and run one DFS per cell, descending the grid and the trie together. Prune trie nodes once their words are found.

#### Approach 1: Trie + Backtracking (no pruning)
```go
type wsNode struct {
	children [26]*wsNode
	word     string // non-empty at the end of a word
}

func findWords(board [][]byte, words []string) []string {
	root := &wsNode{}
	for _, w := range words {
		n := root
		for i := 0; i < len(w); i++ {
			c := w[i] - 'a'
			if n.children[c] == nil {
				n.children[c] = &wsNode{}
			}
			n = n.children[c]
		}
		n.word = w
	}
	m, nCols := len(board), len(board[0])
	res := []string{}
	var dfs func(r, c int, parent *wsNode)
	dfs = func(r, c int, parent *wsNode) {
		if r < 0 || c < 0 || r >= m || c >= nCols || board[r][c] == '#' {
			return
		}
		ch := board[r][c]
		node := parent.children[ch-'a']
		if node == nil {
			return
		}
		if node.word != "" {
			res = append(res, node.word)
			node.word = "" // report each word once
		}
		board[r][c] = '#'
		dfs(r+1, c, node)
		dfs(r-1, c, node)
		dfs(r, c+1, node)
		dfs(r, c-1, node)
		board[r][c] = ch
	}
	for r := 0; r < m; r++ {
		for c := 0; c < nCols; c++ {
			dfs(r, c, root)
		}
	}
	return res
}
```
**Complexity:** Time O(m · n · 4 · 3^(L-1)), where L is the maximum word length · Space O(total characters in `words`)

**Brute force:** Run LC 79 (Word Search) once per word: O(W · m · n · 4^L). Much slower when there are many words.

#### Approach 2: Trie + Backtracking with Leaf Pruning (Optimal)
```go
type wsNode2 struct {
	children [26]*wsNode2
	word     string
}

func findWords2(board [][]byte, words []string) []string {
	root := &wsNode2{}
	for _, w := range words {
		n := root
		for i := 0; i < len(w); i++ {
			c := w[i] - 'a'
			if n.children[c] == nil {
				n.children[c] = &wsNode2{}
			}
			n = n.children[c]
		}
		n.word = w
	}
	res := []string{}
	var dfs func(r, c int, parent *wsNode2)
	dfs = func(r, c int, parent *wsNode2) {
		ch := board[r][c]
		idx := ch - 'a'
		node := parent.children[idx]
		if node.word != "" {
			res = append(res, node.word)
			node.word = "" // stop duplicate matches
		}
		board[r][c] = '#'
		dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
		for _, d := range dirs {
			nr, nc := r+d[0], c+d[1]
			if nr < 0 || nc < 0 || nr >= len(board) || nc >= len(board[0]) {
				continue
			}
			b := board[nr][nc]
			if b != '#' && node.children[b-'a'] != nil {
				dfs(nr, nc, node)
			}
		}
		board[r][c] = ch
		// prune a leaf that has no word left
		empty := node.word == ""
		for _, k := range node.children {
			if k != nil {
				empty = false
				break
			}
		}
		if empty {
			parent.children[idx] = nil
		}
	}
	for r := range board {
		for c := range board[0] {
			if root.children[board[r][c]-'a'] != nil {
				dfs(r, c, root)
			}
		}
	}
	return res
}
```
**Complexity:** Same worst case. Removing leaves cuts the search space sharply in practice; this is the version to use in interviews.

---
## 9. Heap / Priority Queue

> Go has no built-in priority queue. These solutions use `container/heap` with a small type that implements its interface (see the [toolkit](#go-interview-toolkit-quick-reference)).

### 703. Kth Largest Element in a Stream
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/kth-largest-element-in-a-stream/)

**Problem:** Design a class built from `k` and an initial array `nums`. Each call to `Add(val)` adds a value to the stream and returns the current `k`th largest element.

**Example:** `k = 3, nums = [4,5,8,2]`; `add(3)` → 4, `add(5)` → 5, `add(10)` → 5

**Key insight:** Keep a min-heap of size `k`. Its top is the `k`th largest element.

#### Approach: Min-Heap of Size k (Optimal)
```go
type IntHeap []int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *IntHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *IntHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

type KthLargest struct {
	k int
	h *IntHeap
}

func Constructor(k int, nums []int) KthLargest {
	h := &IntHeap{}
	kl := KthLargest{k: k, h: h}
	for _, n := range nums {
		kl.Add(n)
	}
	return kl
}

func (kl *KthLargest) Add(val int) int {
	heap.Push(kl.h, val)
	if kl.h.Len() > kl.k {
		heap.Pop(kl.h)
	}
	return (*kl.h)[0]
}
```
**Complexity:** `Add` O(log k) · Space O(k)

**Alternative:** Keep a sorted slice and insert with binary search. `Add` is then O(n), from shifting elements.

---

### 1046. Last Stone Weight
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/last-stone-weight/)

**Problem:** Each turn, smash the two heaviest stones `x ≤ y`. If they're equal, both are destroyed. Otherwise a stone of weight `y - x` remains. Return the weight of the last stone, or `0` if none are left.

**Example:** `[2,7,4,1,8,1]` → `1`

#### Approach 1: Max-Heap (Optimal)
```go
type MaxHeap []int

func (h MaxHeap) Len() int           { return len(h) }
func (h MaxHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h MaxHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *MaxHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *MaxHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func lastStoneWeight(stones []int) int {
	h := MaxHeap(append([]int{}, stones...))
	heap.Init(&h)
	for h.Len() > 1 {
		y := heap.Pop(&h).(int)
		x := heap.Pop(&h).(int)
		if y != x {
			heap.Push(&h, y-x)
		}
	}
	if h.Len() == 0 {
		return 0
	}
	return h[0]
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Bucket Sort (weights ≤ 1000)
```go
func lastStoneWeightBucket(stones []int) int {
	maxW := 0
	for _, s := range stones {
		maxW = max(maxW, s)
	}
	bucket := make([]int, maxW+1)
	for _, s := range stones {
		bucket[s]++
	}
	biggest, cur := 0, maxW
	for cur > 0 {
		if bucket[cur] == 0 {
			cur--
			continue
		}
		if biggest == 0 {
			bucket[cur] %= 2 // equal pairs destroy each other
			if bucket[cur] == 1 {
				biggest = cur
			}
			cur--
			continue
		}
		bucket[cur]--
		if biggest-cur <= cur {
			bucket[biggest-cur]++
			biggest = 0
		} else {
			biggest -= cur
		}
	}
	return biggest
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
```go
func kClosestSort(points [][]int, k int) [][]int {
	d := func(p []int) int { return p[0]*p[0] + p[1]*p[1] }
	sort.Slice(points, func(i, j int) bool { return d(points[i]) < d(points[j]) })
	return points[:k]
}
```
**Complexity:** Time O(n log n) · Space O(log n)

#### Approach 2: Max-Heap of Size k
```go
type ptHeap [][]int

func (h ptHeap) Len() int { return len(h) }
func (h ptHeap) Less(i, j int) bool {
	return h[i][0]*h[i][0]+h[i][1]*h[i][1] > h[j][0]*h[j][0]+h[j][1]*h[j][1]
}
func (h ptHeap) Swap(i, j int) { h[i], h[j] = h[j], h[i] }
func (h *ptHeap) Push(x any)   { *h = append(*h, x.([]int)) }
func (h *ptHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func kClosestHeap(points [][]int, k int) [][]int {
	h := &ptHeap{}
	for _, p := range points {
		heap.Push(h, p)
		if h.Len() > k {
			heap.Pop(h)
		}
	}
	return *h
}
```
**Complexity:** Time O(n log k) · Space O(k)

#### Approach 3: Quickselect (Optimal on average)
```go
func kClosest(points [][]int, k int) [][]int {
	dist := func(i int) int { return points[i][0]*points[i][0] + points[i][1]*points[i][1] }
	lo, hi := 0, len(points)-1
	for lo < hi {
		// Lomuto partition with the last element as pivot
		pivot := dist(hi)
		store := lo
		for i := lo; i < hi; i++ {
			if dist(i) < pivot {
				points[i], points[store] = points[store], points[i]
				store++
			}
		}
		points[store], points[hi] = points[hi], points[store]
		if store == k-1 {
			break
		} else if store < k-1 {
			lo = store + 1
		} else {
			hi = store - 1
		}
	}
	return points[:k]
}
```
**Complexity:** Time O(n) average, O(n²) worst case · Space O(1)

---

### 215. Kth Largest Element in an Array
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/kth-largest-element-in-an-array/)

**Problem:** Return the `k`th largest element in an unsorted array (in sorted order, not the `k`th distinct value).

**Example:** `nums = [3,2,1,5,6,4], k = 2` → `5`

#### Approach 1: Sort
```go
func findKthLargestSort(nums []int, k int) int {
	sort.Ints(nums)
	return nums[len(nums)-k]
}
```
**Complexity:** Time O(n log n) · Space O(log n)

#### Approach 2: Min-Heap of Size k
```go
type minH []int

func (h minH) Len() int           { return len(h) }
func (h minH) Less(i, j int) bool { return h[i] < h[j] }
func (h minH) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *minH) Push(x any)        { *h = append(*h, x.(int)) }
func (h *minH) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func findKthLargestHeap(nums []int, k int) int {
	h := &minH{}
	for _, n := range nums {
		heap.Push(h, n)
		if h.Len() > k {
			heap.Pop(h)
		}
	}
	return (*h)[0]
}
```
**Complexity:** Time O(n log k) · Space O(k)

#### Approach 3: Quickselect with Three-Way Partition (Optimal on average)
A three-way partition handles many duplicates, and the random pivot avoids worst-case inputs.
```go
func findKthLargest(nums []int, k int) int {
	target := len(nums) - k // index in ascending order
	lo, hi := 0, len(nums)-1
	for {
		pivot := nums[lo+rand.Intn(hi-lo+1)]
		lt, i, gt := lo, lo, hi
		for i <= gt {
			if nums[i] < pivot {
				nums[lt], nums[i] = nums[i], nums[lt]
				lt++
				i++
			} else if nums[i] > pivot {
				nums[i], nums[gt] = nums[gt], nums[i]
				gt--
			} else {
				i++
			}
		}
		if target < lt {
			hi = lt - 1
		} else if target > gt {
			lo = gt + 1
		} else {
			return pivot
		}
	}
}
```
**Complexity:** Time O(n) average · Space O(1)

#### Approach 4: Counting Sort (values in [-10⁴, 10⁴])
```go
func findKthLargestCount(nums []int, k int) int {
	const off = 10000
	var cnt [2*off + 1]int
	for _, n := range nums {
		cnt[n+off]++
	}
	for v := 2 * off; v >= 0; v-- {
		k -= cnt[v]
		if k <= 0 {
			return v - off
		}
	}
	return -1
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
```go
type cntHeap []int

func (h cntHeap) Len() int           { return len(h) }
func (h cntHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h cntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *cntHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *cntHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func leastIntervalHeap(tasks []byte, n int) int {
	var freq [26]int
	for _, t := range tasks {
		freq[t-'A']++
	}
	h := &cntHeap{}
	for _, f := range freq {
		if f > 0 {
			*h = append(*h, f)
		}
	}
	heap.Init(h)
	type waiting struct{ cnt, readyAt int }
	q := []waiting{}
	time := 0
	for h.Len() > 0 || len(q) > 0 {
		time++
		if h.Len() > 0 {
			c := heap.Pop(h).(int) - 1
			if c > 0 {
				q = append(q, waiting{c, time + n})
			}
		}
		if len(q) > 0 && q[0].readyAt == time {
			heap.Push(h, q[0].cnt)
			q = q[1:]
		}
	}
	return time
}
```
**Complexity:** Time O(T · log 26) = O(T), where T is the total time · Space O(26)

#### Approach 2: Math Formula (Optimal)
```go
func leastInterval(tasks []byte, n int) int {
	var freq [26]int
	maxF := 0
	for _, t := range tasks {
		freq[t-'A']++
		maxF = max(maxF, freq[t-'A'])
	}
	maxCount := 0
	for _, f := range freq {
		if f == maxF {
			maxCount++
		}
	}
	return max(len(tasks), (maxF-1)*(n+1)+maxCount)
}
```
**Complexity:** Time O(len(tasks)) · Space O(1)

---

### 355. Design Twitter
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/design-twitter/)

**Problem:** Design a simplified Twitter with `PostTweet(userId, tweetId)`, `GetNewsFeed(userId)` (the 10 most recent tweet IDs from the user and the people they follow, newest first), `Follow`, and `Unfollow`.

**Key insight:** The news feed is a merge of k sorted lists. Each user's tweets are already in time order, so a heap seeded with each followee's newest tweet works.

#### Approach: Hash Maps + Heap Merge (Optimal)
```go
type tweet struct{ time, id int }

type feedItem struct {
	time, id, user, idx int
}

type feedHeap []feedItem

func (h feedHeap) Len() int           { return len(h) }
func (h feedHeap) Less(i, j int) bool { return h[i].time > h[j].time }
func (h feedHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *feedHeap) Push(x any)        { *h = append(*h, x.(feedItem)) }
func (h *feedHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

type Twitter struct {
	clock   int
	tweets  map[int][]tweet
	follows map[int]map[int]bool
}

func Constructor() Twitter {
	return Twitter{tweets: map[int][]tweet{}, follows: map[int]map[int]bool{}}
}

func (t *Twitter) PostTweet(userId int, tweetId int) {
	t.clock++
	t.tweets[userId] = append(t.tweets[userId], tweet{t.clock, tweetId})
}

func (t *Twitter) GetNewsFeed(userId int) []int {
	h := &feedHeap{}
	users := []int{userId}
	for f := range t.follows[userId] {
		if f != userId {
			users = append(users, f)
		}
	}
	for _, u := range users {
		if ts := t.tweets[u]; len(ts) > 0 {
			last := len(ts) - 1
			heap.Push(h, feedItem{ts[last].time, ts[last].id, u, last})
		}
	}
	res := []int{}
	for h.Len() > 0 && len(res) < 10 {
		it := heap.Pop(h).(feedItem)
		res = append(res, it.id)
		if it.idx > 0 {
			nt := t.tweets[it.user][it.idx-1]
			heap.Push(h, feedItem{nt.time, nt.id, it.user, it.idx - 1})
		}
	}
	return res
}

func (t *Twitter) Follow(followerId int, followeeId int) {
	if t.follows[followerId] == nil {
		t.follows[followerId] = map[int]bool{}
	}
	t.follows[followerId][followeeId] = true
}

func (t *Twitter) Unfollow(followerId int, followeeId int) {
	delete(t.follows[followerId], followeeId)
}
```
**Complexity:** `PostTweet`, `Follow`, `Unfollow` O(1) · `GetNewsFeed` O(F + 10 log F) for F followees · Space O(users + tweets + follows)

**Simpler alternative:** Gather every tweet from the user and their followees, sort by time, and take 10: O(N log N) per feed.

---

### 295. Find Median from Data Stream
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/find-median-from-data-stream/)

**Problem:** Design `MedianFinder` with `AddNum(num)` and `FindMedian()`, which returns the median of every number added so far.

**Key insight:** Keep two heaps: a max-heap for the smaller half and a min-heap for the larger half. Keep their sizes within one of each other, so the median is on top.

#### Approach 1: Two Heaps (Optimal)
```go
type minIntHeap []int

func (h minIntHeap) Len() int           { return len(h) }
func (h minIntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h minIntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *minIntHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *minIntHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

type MedianFinder struct {
	low  *minIntHeap // max-heap via negated values
	high *minIntHeap // min-heap
}

func Constructor() MedianFinder {
	return MedianFinder{low: &minIntHeap{}, high: &minIntHeap{}}
}

func (m *MedianFinder) AddNum(num int) {
	heap.Push(m.low, -num)
	heap.Push(m.high, -heap.Pop(m.low).(int)) // move low's max to high
	if m.high.Len() > m.low.Len() {
		heap.Push(m.low, -heap.Pop(m.high).(int))
	}
}

func (m *MedianFinder) FindMedian() float64 {
	if m.low.Len() > m.high.Len() {
		return float64(-(*m.low)[0])
	}
	return float64(-(*m.low)[0]+(*m.high)[0]) / 2.0
}
```
**Complexity:** `AddNum` O(log n) · `FindMedian` O(1) · Space O(n)

#### Approach 2: Sorted Slice with Binary Insert (simple)
```go
type MedianFinderSorted struct{ data []int }

func (m *MedianFinderSorted) AddNum(num int) {
	i := sort.SearchInts(m.data, num)
	m.data = append(m.data, 0)
	copy(m.data[i+1:], m.data[i:])
	m.data[i] = num
}

func (m *MedianFinderSorted) FindMedian() float64 {
	n := len(m.data)
	if n%2 == 1 {
		return float64(m.data[n/2])
	}
	return float64(m.data[n/2-1]+m.data[n/2]) / 2.0
}
```
**Complexity:** `AddNum` O(n) · `FindMedian` O(1)

**Follow-up (all values in [0, 100]):** Keep a 101-bucket count array and walk it to find the median, which is O(100) per query.

---
## 10. Backtracking

> **Template:** choose → recurse → un-choose. In Go, copy the path before saving it (`append([]int{}, path...)`). Saving `path` directly shares its backing array, so later changes would overwrite results you've already stored.

### 78. Subsets
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/subsets/)

**Problem:** Given an array of unique integers, return every possible subset (the power set), with no duplicates.

**Example:** `[1,2,3]` → `[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]`

#### Approach 1: Backtracking (include or exclude)
```go
func subsets(nums []int) [][]int {
	res := [][]int{}
	path := []int{}
	var bt func(i int)
	bt = func(i int) {
		if i == len(nums) {
			res = append(res, append([]int{}, path...))
			return
		}
		path = append(path, nums[i]) // include
		bt(i + 1)
		path = path[:len(path)-1] // exclude
		bt(i + 1)
	}
	bt(0)
	return res
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n) recursion

#### Approach 2: Iterative Cascading
```go
func subsetsIter(nums []int) [][]int {
	res := [][]int{{}}
	for _, n := range nums {
		size := len(res)
		for i := 0; i < size; i++ {
			s := append(append([]int{}, res[i]...), n)
			res = append(res, s)
		}
	}
	return res
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n · 2ⁿ) for the output

#### Approach 3: Bitmask
```go
func subsetsBits(nums []int) [][]int {
	n := len(nums)
	res := make([][]int, 0, 1<<n)
	for mask := 0; mask < 1<<n; mask++ {
		s := []int{}
		for i := 0; i < n; i++ {
			if mask&(1<<i) != 0 {
				s = append(s, nums[i])
			}
		}
		res = append(res, s)
	}
	return res
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
```go
func combinationSum(candidates []int, target int) [][]int {
	sort.Ints(candidates)
	res := [][]int{}
	path := []int{}
	var bt func(start, remain int)
	bt = func(start, remain int) {
		if remain == 0 {
			res = append(res, append([]int{}, path...))
			return
		}
		for i := start; i < len(candidates); i++ {
			if candidates[i] > remain {
				break // sorted, so every later candidate is too big
			}
			path = append(path, candidates[i])
			bt(i, remain-candidates[i]) // i, not i+1: reuse is allowed
			path = path[:len(path)-1]
		}
	}
	bt(0, target)
	return res
}
```
**Complexity:** Time O(N^(T/M + 1)), where T = target and M = smallest candidate · Space O(T/M) recursion

#### Approach 2: DP Building Combinations (unbounded knapsack style)
```go
func combinationSumDP(candidates []int, target int) [][]int {
	dp := make([][][]int, target+1)
	dp[0] = [][]int{{}}
	for _, c := range candidates { // outer loop over coins means no permutations
		for t := c; t <= target; t++ {
			for _, comb := range dp[t-c] {
				nc := append(append([]int{}, comb...), c)
				dp[t] = append(dp[t], nc)
			}
		}
	}
	return dp[target]
}
```
**Complexity:** Exponential in the output size · Space proportional to all partial combinations

---

### 46. Permutations
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/permutations/)

**Problem:** Given an array of distinct integers, return every permutation.

**Example:** `[1,2,3]` → `[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]`

#### Approach 1: Backtracking with a Used Array
```go
func permute(nums []int) [][]int {
	res := [][]int{}
	path := []int{}
	used := make([]bool, len(nums))
	var bt func()
	bt = func() {
		if len(path) == len(nums) {
			res = append(res, append([]int{}, path...))
			return
		}
		for i, n := range nums {
			if used[i] {
				continue
			}
			used[i] = true
			path = append(path, n)
			bt()
			path = path[:len(path)-1]
			used[i] = false
		}
	}
	bt()
	return res
}
```
**Complexity:** Time O(n · n!) · Space O(n)

#### Approach 2: In-Place Swapping
```go
func permuteSwap(nums []int) [][]int {
	res := [][]int{}
	var bt func(k int)
	bt = func(k int) {
		if k == len(nums) {
			res = append(res, append([]int{}, nums...))
			return
		}
		for i := k; i < len(nums); i++ {
			nums[k], nums[i] = nums[i], nums[k]
			bt(k + 1)
			nums[k], nums[i] = nums[i], nums[k]
		}
	}
	bt(0)
	return res
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
```go
func subsetsWithDup(nums []int) [][]int {
	sort.Ints(nums)
	res := [][]int{}
	path := []int{}
	var bt func(start int)
	bt = func(start int) {
		res = append(res, append([]int{}, path...))
		for i := start; i < len(nums); i++ {
			if i > start && nums[i] == nums[i-1] {
				continue
			}
			path = append(path, nums[i])
			bt(i + 1)
			path = path[:len(path)-1]
		}
	}
	bt(0)
	return res
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n)

#### Approach 2: Iterative (extend only the subsets added last round when the value repeats)
```go
func subsetsWithDupIter(nums []int) [][]int {
	sort.Ints(nums)
	res := [][]int{{}}
	prevSize := 0
	for i, n := range nums {
		start := 0
		if i > 0 && nums[i] == nums[i-1] {
			start = prevSize
		}
		size := len(res)
		for j := start; j < size; j++ {
			res = append(res, append(append([]int{}, res[j]...), n))
		}
		prevSize = size
	}
	return res
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n · 2ⁿ) for the output

---

### 40. Combination Sum II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/combination-sum-ii/)

**Problem:** Given `candidates` (which may contain duplicates) and a `target`, return every unique combination that sums to `target`. Each element may be used at most once.

**Example:** `candidates = [10,1,2,7,6,1,5], target = 8` → `[[1,1,6],[1,2,5],[1,7],[2,6]]`

#### Approach: Sort + Backtracking with Duplicate Skipping (Optimal)
```go
func combinationSum2(candidates []int, target int) [][]int {
	sort.Ints(candidates)
	res := [][]int{}
	path := []int{}
	var bt func(start, remain int)
	bt = func(start, remain int) {
		if remain == 0 {
			res = append(res, append([]int{}, path...))
			return
		}
		for i := start; i < len(candidates); i++ {
			if i > start && candidates[i] == candidates[i-1] {
				continue
			}
			if candidates[i] > remain {
				break
			}
			path = append(path, candidates[i])
			bt(i+1, remain-candidates[i])
			path = path[:len(path)-1]
		}
	}
	bt(0, target)
	return res
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
```go
func exist(board [][]byte, word string) bool {
	m, n := len(board), len(board[0])
	var dfs func(r, c, i int) bool
	dfs = func(r, c, i int) bool {
		if i == len(word) {
			return true
		}
		if r < 0 || c < 0 || r >= m || c >= n || board[r][c] != word[i] {
			return false
		}
		tmp := board[r][c]
		board[r][c] = '#'
		found := dfs(r+1, c, i+1) || dfs(r-1, c, i+1) || dfs(r, c+1, i+1) || dfs(r, c-1, i+1)
		board[r][c] = tmp
		return found
	}
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if dfs(r, c, 0) {
				return true
			}
		}
	}
	return false
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
```go
func partition(s string) [][]string {
	res := [][]string{}
	path := []string{}
	isPal := func(l, r int) bool {
		for l < r {
			if s[l] != s[r] {
				return false
			}
			l++
			r--
		}
		return true
	}
	var bt func(start int)
	bt = func(start int) {
		if start == len(s) {
			res = append(res, append([]string{}, path...))
			return
		}
		for end := start; end < len(s); end++ {
			if isPal(start, end) {
				path = append(path, s[start:end+1])
				bt(end + 1)
				path = path[:len(path)-1]
			}
		}
	}
	bt(0)
	return res
}
```
**Complexity:** Time O(n · 2ⁿ) · Space O(n)

#### Approach 2: Precomputed Palindrome DP Table
```go
func partitionDP(s string) [][]string {
	n := len(s)
	pal := make([][]bool, n)
	for i := range pal {
		pal[i] = make([]bool, n)
	}
	for i := n - 1; i >= 0; i-- {
		for j := i; j < n; j++ {
			pal[i][j] = s[i] == s[j] && (j-i < 2 || pal[i+1][j-1])
		}
	}
	res := [][]string{}
	path := []string{}
	var bt func(int)
	bt = func(start int) {
		if start == n {
			res = append(res, append([]string{}, path...))
			return
		}
		for end := start; end < n; end++ {
			if pal[start][end] {
				path = append(path, s[start:end+1])
				bt(end + 1)
				path = path[:len(path)-1]
			}
		}
	}
	bt(0)
	return res
}
```
**Complexity:** Time O(n · 2ⁿ), with each palindrome check O(1) · Space O(n²)

---

### 17. Letter Combinations of a Phone Number
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/letter-combinations-of-a-phone-number/)

**Problem:** Given a string of digits `2`–`9`, return every letter combination they could spell on a phone keypad.

**Example:** `"23"` → `["ad","ae","af","bd","be","bf","cd","ce","cf"]`

#### Approach 1: Backtracking
```go
func letterCombinations(digits string) []string {
	if digits == "" {
		return []string{}
	}
	keys := []string{"", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"}
	res := []string{}
	buf := make([]byte, 0, len(digits))
	var bt func(i int)
	bt = func(i int) {
		if i == len(digits) {
			res = append(res, string(buf))
			return
		}
		for _, ch := range []byte(keys[digits[i]-'0']) {
			buf = append(buf, ch)
			bt(i + 1)
			buf = buf[:len(buf)-1]
		}
	}
	bt(0)
	return res
}
```
**Complexity:** Time O(n · 4ⁿ) · Space O(n)

#### Approach 2: Iterative BFS
```go
func letterCombinationsIter(digits string) []string {
	if digits == "" {
		return []string{}
	}
	keys := []string{"", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"}
	res := []string{""}
	for i := 0; i < len(digits); i++ {
		next := []string{}
		for _, prefix := range res {
			for _, ch := range keys[digits[i]-'0'] {
				next = append(next, prefix+string(ch))
			}
		}
		res = next
	}
	return res
}
```
**Complexity:** Time O(n · 4ⁿ) · Space O(n · 4ⁿ)

---

### 51. N-Queens
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/n-queens/)

**Problem:** Place `n` queens on an `n × n` board so that no two attack each other. Return every distinct board, written with `'Q'` for a queen and `'.'` for an empty square.

**Example:** `n = 4` → `[[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]`

**Key insight:** Place one queen per row. A column, a diagonal (`r-c`), or an anti-diagonal (`r+c`) can each hold at most one queen.

#### Approach 1: Backtracking with Sets (bool arrays)
```go
func solveNQueens(n int) [][]string {
	res := [][]string{}
	cols := make([]bool, n)
	diag := make([]bool, 2*n)  // r - c + n
	anti := make([]bool, 2*n)  // r + c
	queens := make([]int, n)   // queens[r] = column
	var bt func(r int)
	bt = func(r int) {
		if r == n {
			board := make([]string, n)
			for i := 0; i < n; i++ {
				row := []byte(strings.Repeat(".", n))
				row[queens[i]] = 'Q'
				board[i] = string(row)
			}
			res = append(res, board)
			return
		}
		for c := 0; c < n; c++ {
			if cols[c] || diag[r-c+n] || anti[r+c] {
				continue
			}
			cols[c], diag[r-c+n], anti[r+c] = true, true, true
			queens[r] = c
			bt(r + 1)
			cols[c], diag[r-c+n], anti[r+c] = false, false, false
		}
	}
	bt(0)
	return res
}
```
**Complexity:** Time O(n!) · Space O(n)

#### Approach 2: Bitmask Backtracking (fastest)
```go
func solveNQueensBits(n int) [][]string {
	res := [][]string{}
	queens := make([]int, n)
	full := (1 << n) - 1
	var bt func(r, cols, d1, d2 int)
	bt = func(r, cols, d1, d2 int) {
		if r == n {
			board := make([]string, n)
			for i, c := range queens {
				row := []byte(strings.Repeat(".", n))
				row[c] = 'Q'
				board[i] = string(row)
			}
			res = append(res, board)
			return
		}
		avail := full &^ (cols | d1 | d2)
		for avail != 0 {
			bit := avail & -avail // lowest set bit
			avail &^= bit
			queens[r] = bits.TrailingZeros(uint(bit))
			bt(r+1, cols|bit, (d1|bit)<<1, (d2|bit)>>1)
		}
	}
	bt(0, 0, 0, 0)
	return res
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
```go
func numIslands(grid [][]byte) int {
	m, n := len(grid), len(grid[0])
	var sink func(r, c int)
	sink = func(r, c int) {
		if r < 0 || c < 0 || r >= m || c >= n || grid[r][c] != '1' {
			return
		}
		grid[r][c] = '0'
		sink(r+1, c)
		sink(r-1, c)
		sink(r, c+1)
		sink(r, c-1)
	}
	count := 0
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if grid[r][c] == '1' {
				count++
				sink(r, c)
			}
		}
	}
	return count
}
```
**Complexity:** Time O(m · n) · Space O(m · n) recursion in the worst case

#### Approach 2: BFS
```go
func numIslandsBFS(grid [][]byte) int {
	m, n := len(grid), len(grid[0])
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	count := 0
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if grid[r][c] != '1' {
				continue
			}
			count++
			grid[r][c] = '0'
			q := [][2]int{{r, c}}
			for len(q) > 0 {
				cell := q[0]
				q = q[1:]
				for _, d := range dirs {
					nr, nc := cell[0]+d[0], cell[1]+d[1]
					if nr >= 0 && nc >= 0 && nr < m && nc < n && grid[nr][nc] == '1' {
						grid[nr][nc] = '0'
						q = append(q, [2]int{nr, nc})
					}
				}
			}
		}
	}
	return count
}
```
**Complexity:** Time O(m · n) · Space O(min(m, n)) for the queue

#### Approach 3: Union-Find
```go
func numIslandsUF(grid [][]byte) int {
	m, n := len(grid), len(grid[0])
	parent := make([]int, m*n)
	count := 0
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if grid[r][c] == '1' {
				parent[r*n+c] = r*n + c
				count++
			}
		}
	}
	var find func(int) int
	find = func(x int) int {
		for parent[x] != x {
			parent[x] = parent[parent[x]] // path halving
			x = parent[x]
		}
		return x
	}
	union := func(a, b int) {
		ra, rb := find(a), find(b)
		if ra != rb {
			parent[ra] = rb
			count--
		}
	}
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if grid[r][c] != '1' {
				continue
			}
			if r+1 < m && grid[r+1][c] == '1' {
				union(r*n+c, (r+1)*n+c)
			}
			if c+1 < n && grid[r][c+1] == '1' {
				union(r*n+c, r*n+c+1)
			}
		}
	}
	return count
}
```
**Complexity:** Time O(m · n · α(mn)) · Space O(m · n)

---

### 133. Clone Graph
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/clone-graph/)

**Problem:** Given a reference to one node in a connected undirected graph, return a deep copy of the whole graph. Each node has a `Val` and a list of `Neighbors`.

**Key insight:** A map from original node to clone serves as the visited set and also handles cycles.

#### Approach 1: DFS + Hash Map
```go
type Node struct {
	Val       int
	Neighbors []*Node
}

func cloneGraph(node *Node) *Node {
	clones := map[*Node]*Node{}
	var dfs func(*Node) *Node
	dfs = func(n *Node) *Node {
		if n == nil {
			return nil
		}
		if c, ok := clones[n]; ok {
			return c
		}
		c := &Node{Val: n.Val}
		clones[n] = c
		for _, nb := range n.Neighbors {
			c.Neighbors = append(c.Neighbors, dfs(nb))
		}
		return c
	}
	return dfs(node)
}
```
**Complexity:** Time O(V + E) · Space O(V)

#### Approach 2: BFS + Hash Map
```go
func cloneGraphBFS(node *Node) *Node {
	if node == nil {
		return nil
	}
	clones := map[*Node]*Node{node: {Val: node.Val}}
	q := []*Node{node}
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		for _, nb := range cur.Neighbors {
			if _, ok := clones[nb]; !ok {
				clones[nb] = &Node{Val: nb.Val}
				q = append(q, nb)
			}
			clones[cur].Neighbors = append(clones[cur].Neighbors, clones[nb])
		}
	}
	return clones[node]
}
```
**Complexity:** Time O(V + E) · Space O(V)

---

### 695. Max Area of Island
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/max-area-of-island/)

**Problem:** In a binary grid where `1` is land, return the size of the largest island (4-directionally connected), or `0` if there is no land.

#### Approach 1: DFS Returning Area
```go
func maxAreaOfIsland(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	var dfs func(r, c int) int
	dfs = func(r, c int) int {
		if r < 0 || c < 0 || r >= m || c >= n || grid[r][c] == 0 {
			return 0
		}
		grid[r][c] = 0
		return 1 + dfs(r+1, c) + dfs(r-1, c) + dfs(r, c+1) + dfs(r, c-1)
	}
	best := 0
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			best = max(best, dfs(r, c))
		}
	}
	return best
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Iterative DFS with a Stack
```go
func maxAreaOfIslandIter(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	best := 0
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if grid[r][c] == 0 {
				continue
			}
			area := 0
			grid[r][c] = 0
			stack := [][2]int{{r, c}}
			for len(stack) > 0 {
				p := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				area++
				for _, d := range [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}} {
					nr, nc := p[0]+d[0], p[1]+d[1]
					if nr >= 0 && nc >= 0 && nr < m && nc < n && grid[nr][nc] == 1 {
						grid[nr][nc] = 0
						stack = append(stack, [2]int{nr, nc})
					}
				}
			}
			best = max(best, area)
		}
	}
	return best
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

---

### 417. Pacific Atlantic Water Flow
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/pacific-atlantic-water-flow/)

**Problem:** `heights` is an island. The Pacific Ocean touches its top and left edges, and the Atlantic touches its bottom and right edges. Water flows from a cell to a neighbor of equal or lower height. Return every cell from which water can reach both oceans.

**Key insight:** Search in reverse, from each ocean's border moving to cells of equal or greater height. The answer is the intersection of the two reachable sets.

#### Approach 1: Reverse DFS from Both Oceans (Optimal)
```go
func pacificAtlantic(heights [][]int) [][]int {
	m, n := len(heights), len(heights[0])
	pac := make([][]bool, m)
	atl := make([][]bool, m)
	for i := range pac {
		pac[i] = make([]bool, n)
		atl[i] = make([]bool, n)
	}
	var dfs func(r, c int, seen [][]bool, prev int)
	dfs = func(r, c int, seen [][]bool, prev int) {
		if r < 0 || c < 0 || r >= m || c >= n || seen[r][c] || heights[r][c] < prev {
			return
		}
		seen[r][c] = true
		h := heights[r][c]
		dfs(r+1, c, seen, h)
		dfs(r-1, c, seen, h)
		dfs(r, c+1, seen, h)
		dfs(r, c-1, seen, h)
	}
	for c := 0; c < n; c++ {
		dfs(0, c, pac, 0)
		dfs(m-1, c, atl, 0)
	}
	for r := 0; r < m; r++ {
		dfs(r, 0, pac, 0)
		dfs(r, n-1, atl, 0)
	}
	res := [][]int{}
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if pac[r][c] && atl[r][c] {
				res = append(res, []int{r, c})
			}
		}
	}
	return res
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

**Alternative:** Multi-source BFS from each ocean's border cells gives the same bounds and avoids deep recursion.

**Brute force:** DFS from every cell toward the oceans: O((m·n)²).

---

### 130. Surrounded Regions
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/surrounded-regions/)

**Problem:** In a board of `'X'` and `'O'`, flip every `'O'` region that is fully surrounded by `'X'` to `'X'`. Regions connected to the border stay as they are.

**Key insight:** Work backward. Mark every `O` reachable from the border as safe (`T`). Flip all remaining `O`s to `X`, then turn the `T`s back into `O`s.

#### Approach 1: Border DFS (Optimal)
```go
func solve(board [][]byte) {
	m, n := len(board), len(board[0])
	var mark func(r, c int)
	mark = func(r, c int) {
		if r < 0 || c < 0 || r >= m || c >= n || board[r][c] != 'O' {
			return
		}
		board[r][c] = 'T'
		mark(r+1, c)
		mark(r-1, c)
		mark(r, c+1)
		mark(r, c-1)
	}
	for r := 0; r < m; r++ {
		mark(r, 0)
		mark(r, n-1)
	}
	for c := 0; c < n; c++ {
		mark(0, c)
		mark(m-1, c)
	}
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			switch board[r][c] {
			case 'O':
				board[r][c] = 'X'
			case 'T':
				board[r][c] = 'O'
			}
		}
	}
}
```
**Complexity:** Time O(m · n) · Space O(m · n) recursion

**Alternative:** Union-find with a virtual "border" node that every border `O` joins. At the end, flip any `O` that isn't connected to it.

---

### 994. Rotting Oranges
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/rotting-oranges/)

**Problem:** Grid cells are `0` (empty), `1` (fresh), or `2` (rotten). Each minute, fresh oranges next to a rotten one become rotten. Return the minutes until no fresh orange remains, or `-1` if some never rot.

**Key insight:** Run multi-source BFS starting from every rotten orange at once. Each BFS level is one minute.

#### Approach: Multi-Source BFS (Optimal)
```go
func orangesRotting(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	q := [][2]int{}
	fresh := 0
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if grid[r][c] == 2 {
				q = append(q, [2]int{r, c})
			} else if grid[r][c] == 1 {
				fresh++
			}
		}
	}
	minutes := 0
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for len(q) > 0 && fresh > 0 {
		minutes++
		for sz := len(q); sz > 0; sz-- {
			cell := q[0]
			q = q[1:]
			for _, d := range dirs {
				nr, nc := cell[0]+d[0], cell[1]+d[1]
				if nr >= 0 && nc >= 0 && nr < m && nc < n && grid[nr][nc] == 1 {
					grid[nr][nc] = 2
					fresh--
					q = append(q, [2]int{nr, nc})
				}
			}
		}
	}
	if fresh > 0 {
		return -1
	}
	return minutes
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

---

### 286. Walls and Gates (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/walls-and-gates/) · Free mirror: LintCode 663

**Problem:** In a grid, `-1` is a wall, `0` is a gate, and `INF = 2147483647` is an empty room. Fill each empty room in place with its distance to the nearest gate, leaving `INF` where no gate is reachable.

**Key insight:** Run multi-source BFS from every gate. The first time BFS reaches a room, that distance is the shortest.

#### Approach 1: Multi-Source BFS (Optimal)
```go
func wallsAndGates(rooms [][]int) {
	const INF = 2147483647
	m, n := len(rooms), len(rooms[0])
	q := [][2]int{}
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if rooms[r][c] == 0 {
				q = append(q, [2]int{r, c})
			}
		}
	}
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for len(q) > 0 {
		cell := q[0]
		q = q[1:]
		for _, d := range dirs {
			nr, nc := cell[0]+d[0], cell[1]+d[1]
			if nr >= 0 && nc >= 0 && nr < m && nc < n && rooms[nr][nc] == INF {
				rooms[nr][nc] = rooms[cell[0]][cell[1]] + 1
				q = append(q, [2]int{nr, nc})
			}
		}
	}
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: DFS from Each Gate (slower)
```go
func wallsAndGatesDFS(rooms [][]int) {
	m, n := len(rooms), len(rooms[0])
	var dfs func(r, c, d int)
	dfs = func(r, c, d int) {
		if r < 0 || c < 0 || r >= m || c >= n || rooms[r][c] < d {
			return // a wall (-1) or an already shorter distance
		}
		rooms[r][c] = d
		dfs(r+1, c, d+1)
		dfs(r-1, c, d+1)
		dfs(r, c+1, d+1)
		dfs(r, c-1, d+1)
	}
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			if rooms[r][c] == 0 {
				dfs(r, c, 0)
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
```go
func canFinish(numCourses int, prerequisites [][]int) bool {
	adj := make([][]int, numCourses)
	indeg := make([]int, numCourses)
	for _, p := range prerequisites {
		adj[p[1]] = append(adj[p[1]], p[0])
		indeg[p[0]]++
	}
	q := []int{}
	for i, d := range indeg {
		if d == 0 {
			q = append(q, i)
		}
	}
	taken := 0
	for len(q) > 0 {
		c := q[0]
		q = q[1:]
		taken++
		for _, nxt := range adj[c] {
			indeg[nxt]--
			if indeg[nxt] == 0 {
				q = append(q, nxt)
			}
		}
	}
	return taken == numCourses
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

#### Approach 2: DFS Three-Color Cycle Detection
```go
func canFinishDFS(numCourses int, prerequisites [][]int) bool {
	adj := make([][]int, numCourses)
	for _, p := range prerequisites {
		adj[p[1]] = append(adj[p[1]], p[0])
	}
	state := make([]int, numCourses) // 0 = unvisited, 1 = visiting, 2 = done
	var hasCycle func(int) bool
	hasCycle = func(u int) bool {
		if state[u] == 1 {
			return true
		}
		if state[u] == 2 {
			return false
		}
		state[u] = 1
		for _, v := range adj[u] {
			if hasCycle(v) {
				return true
			}
		}
		state[u] = 2
		return false
	}
	for i := 0; i < numCourses; i++ {
		if hasCycle(i) {
			return false
		}
	}
	return true
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

---

### 210. Course Schedule II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/course-schedule-ii/)

**Problem:** Same setup as 207, but return a valid order in which to take the courses, or an empty array if it's impossible.

**Example:** `4, [[1,0],[2,0],[3,1],[3,2]]` → `[0,1,2,3]` or `[0,2,1,3]`

#### Approach 1: Kahn's Algorithm
```go
func findOrder(numCourses int, prerequisites [][]int) []int {
	adj := make([][]int, numCourses)
	indeg := make([]int, numCourses)
	for _, p := range prerequisites {
		adj[p[1]] = append(adj[p[1]], p[0])
		indeg[p[0]]++
	}
	q := []int{}
	for i, d := range indeg {
		if d == 0 {
			q = append(q, i)
		}
	}
	order := make([]int, 0, numCourses)
	for len(q) > 0 {
		c := q[0]
		q = q[1:]
		order = append(order, c)
		for _, nxt := range adj[c] {
			indeg[nxt]--
			if indeg[nxt] == 0 {
				q = append(q, nxt)
			}
		}
	}
	if len(order) != numCourses {
		return []int{}
	}
	return order
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

#### Approach 2: DFS Post-Order
```go
func findOrderDFS(numCourses int, prerequisites [][]int) []int {
	adj := make([][]int, numCourses)
	for _, p := range prerequisites {
		adj[p[0]] = append(adj[p[0]], p[1]) // course → its prerequisites
	}
	state := make([]int, numCourses)
	order := []int{}
	var dfs func(int) bool // returns false when a cycle is found
	dfs = func(u int) bool {
		if state[u] == 1 {
			return false
		}
		if state[u] == 2 {
			return true
		}
		state[u] = 1
		for _, v := range adj[u] {
			if !dfs(v) {
				return false
			}
		}
		state[u] = 2
		order = append(order, u) // prerequisites are appended first
		return true
	}
	for i := 0; i < numCourses; i++ {
		if !dfs(i) {
			return []int{}
		}
	}
	return order
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
```go
func findRedundantConnection(edges [][]int) []int {
	n := len(edges)
	parent := make([]int, n+1)
	rank := make([]int, n+1)
	for i := range parent {
		parent[i] = i
	}
	var find func(int) int
	find = func(x int) int {
		if parent[x] != x {
			parent[x] = find(parent[x])
		}
		return parent[x]
	}
	for _, e := range edges {
		a, b := find(e[0]), find(e[1])
		if a == b {
			return e
		}
		if rank[a] < rank[b] {
			a, b = b, a
		}
		parent[b] = a
		if rank[a] == rank[b] {
			rank[a]++
		}
	}
	return nil
}
```
**Complexity:** Time O(n · α(n)) · Space O(n)

#### Approach 2: DFS per Edge (check whether already connected)
```go
func findRedundantConnectionDFS(edges [][]int) []int {
	adj := map[int][]int{}
	var connected func(u, target int, seen map[int]bool) bool
	connected = func(u, target int, seen map[int]bool) bool {
		if u == target {
			return true
		}
		seen[u] = true
		for _, v := range adj[u] {
			if !seen[v] && connected(v, target, seen) {
				return true
			}
		}
		return false
	}
	for _, e := range edges {
		if connected(e[0], e[1], map[int]bool{}) {
			return e
		}
		adj[e[0]] = append(adj[e[0]], e[1])
		adj[e[1]] = append(adj[e[1]], e[0])
	}
	return nil
}
```
**Complexity:** Time O(n²) · Space O(n)

---

### 323. Number of Connected Components in an Undirected Graph (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/) · Free mirror: LintCode 3651

**Problem:** Given `n` nodes labeled `0..n-1` and a list of undirected `edges`, return the number of connected components.

**Example:** `n = 5, edges = [[0,1],[1,2],[3,4]]` → `2`

#### Approach 1: Union-Find (Optimal)
```go
func countComponents(n int, edges [][]int) int {
	parent := make([]int, n)
	for i := range parent {
		parent[i] = i
	}
	var find func(int) int
	find = func(x int) int {
		if parent[x] != x {
			parent[x] = find(parent[x])
		}
		return parent[x]
	}
	comps := n
	for _, e := range edges {
		a, b := find(e[0]), find(e[1])
		if a != b {
			parent[a] = b
			comps--
		}
	}
	return comps
}
```
**Complexity:** Time O(V + E · α(V)) · Space O(V)

#### Approach 2: DFS
```go
func countComponentsDFS(n int, edges [][]int) int {
	adj := make([][]int, n)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], e[1])
		adj[e[1]] = append(adj[e[1]], e[0])
	}
	seen := make([]bool, n)
	var dfs func(int)
	dfs = func(u int) {
		seen[u] = true
		for _, v := range adj[u] {
			if !seen[v] {
				dfs(v)
			}
		}
	}
	comps := 0
	for i := 0; i < n; i++ {
		if !seen[i] {
			comps++
			dfs(i)
		}
	}
	return comps
}
```
**Complexity:** Time O(V + E) · Space O(V + E)

---

### 261. Graph Valid Tree (Premium)
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/graph-valid-tree/) · Free mirror: LintCode 178

**Problem:** Given `n` nodes and a list of undirected `edges`, return whether the edges form a valid tree.

**Key insight:** A graph is a tree exactly when it has `n-1` edges and is connected (equivalently, `n-1` edges and no cycle).

#### Approach 1: Edge Count + Union-Find (Optimal)
```go
func validTree(n int, edges [][]int) bool {
	if len(edges) != n-1 {
		return false
	}
	parent := make([]int, n)
	for i := range parent {
		parent[i] = i
	}
	var find func(int) int
	find = func(x int) int {
		if parent[x] != x {
			parent[x] = find(parent[x])
		}
		return parent[x]
	}
	for _, e := range edges {
		a, b := find(e[0]), find(e[1])
		if a == b {
			return false // cycle
		}
		parent[a] = b
	}
	return true
}
```
**Complexity:** Time O(n · α(n)) · Space O(n)

#### Approach 2: Edge Count + DFS Connectivity
```go
func validTreeDFS(n int, edges [][]int) bool {
	if len(edges) != n-1 {
		return false
	}
	adj := make([][]int, n)
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], e[1])
		adj[e[1]] = append(adj[e[1]], e[0])
	}
	seen := make([]bool, n)
	var dfs func(int)
	dfs = func(u int) {
		seen[u] = true
		for _, v := range adj[u] {
			if !seen[v] {
				dfs(v)
			}
		}
	}
	dfs(0)
	for _, s := range seen {
		if !s {
			return false
		}
	}
	return true
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
```go
func ladderLength(beginWord string, endWord string, wordList []string) int {
	dict := map[string]bool{}
	for _, w := range wordList {
		dict[w] = true
	}
	if !dict[endWord] {
		return 0
	}
	q := []string{beginWord}
	delete(dict, beginWord)
	steps := 1
	for len(q) > 0 {
		for sz := len(q); sz > 0; sz-- {
			w := []byte(q[0])
			q = q[1:]
			for i := range w {
				orig := w[i]
				for c := byte('a'); c <= 'z'; c++ {
					if c == orig {
						continue
					}
					w[i] = c
					s := string(w)
					if s == endWord {
						return steps + 1
					}
					if dict[s] {
						delete(dict, s) // mark visited
						q = append(q, s)
					}
				}
				w[i] = orig
			}
		}
		steps++
	}
	return 0
}
```
**Complexity:** Time O(N · L · 26 · L), with string building · Space O(N · L)

#### Approach 2: Bidirectional BFS (fastest in practice)
```go
func ladderLengthBi(beginWord string, endWord string, wordList []string) int {
	dict := map[string]bool{}
	for _, w := range wordList {
		dict[w] = true
	}
	if !dict[endWord] {
		return 0
	}
	front, back := map[string]bool{beginWord: true}, map[string]bool{endWord: true}
	delete(dict, endWord)
	steps := 1
	for len(front) > 0 && len(back) > 0 {
		if len(front) > len(back) {
			front, back = back, front // always expand the smaller side
		}
		next := map[string]bool{}
		for word := range front {
			w := []byte(word)
			for i := range w {
				orig := w[i]
				for c := byte('a'); c <= 'z'; c++ {
					w[i] = c
					s := string(w)
					if back[s] {
						return steps + 1
					}
					if dict[s] {
						delete(dict, s)
						next[s] = true
					}
				}
				w[i] = orig
			}
		}
		front = next
		steps++
	}
	return 0
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
```go
func findItinerary(tickets [][]string) []string {
	adj := map[string][]string{}
	for _, t := range tickets {
		adj[t[0]] = append(adj[t[0]], t[1])
	}
	for k := range adj {
		// sort descending so we can pop the smallest from the end
		sort.Sort(sort.Reverse(sort.StringSlice(adj[k])))
	}
	route := []string{}
	var visit func(string)
	visit = func(a string) {
		for len(adj[a]) > 0 {
			last := len(adj[a]) - 1
			next := adj[a][last]
			adj[a] = adj[a][:last]
			visit(next)
		}
		route = append(route, a)
	}
	visit("JFK")
	for i, j := 0, len(route)-1; i < j; i, j = i+1, j-1 {
		route[i], route[j] = route[j], route[i]
	}
	return route
}
```
**Complexity:** Time O(E log E) · Space O(E)

#### Approach 2: Backtracking DFS (simpler idea, worse worst case)
```go
func findItineraryBT(tickets [][]string) []string {
	adj := map[string][]string{}
	sort.Slice(tickets, func(i, j int) bool { return tickets[i][1] < tickets[j][1] })
	for _, t := range tickets {
		adj[t[0]] = append(adj[t[0]], t[1])
	}
	res := []string{"JFK"}
	var dfs func(src string) bool
	dfs = func(src string) bool {
		if len(res) == len(tickets)+1 {
			return true
		}
		dests := adj[src]
		for i := 0; i < len(dests); i++ {
			v := dests[i]
			if v == "" {
				continue // ticket already used
			}
			dests[i] = ""
			res = append(res, v)
			if dfs(v) {
				return true
			}
			res = res[:len(res)-1]
			dests[i] = v
		}
		return false
	}
	dfs("JFK")
	return res
}
```
**Complexity:** Time O(V · E) in practice, exponential in the worst case · Space O(E)

---

### 1584. Min Cost to Connect All Points
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/min-cost-to-connect-all-points/)

**Problem:** Given points on a 2D plane, connecting two points costs their Manhattan distance. Return the minimum total cost to connect every point (a minimum spanning tree).

**Example:** `[[0,0],[2,2],[3,10],[5,2],[7,0]]` → `20`

#### Approach 1: Prim's with an O(n²) Array (Optimal for dense graphs)
```go
func minCostConnectPoints(points [][]int) int {
	n := len(points)
	dist := make([]int, n)
	inTree := make([]bool, n)
	for i := range dist {
		dist[i] = math.MaxInt
	}
	dist[0] = 0
	total := 0
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	for k := 0; k < n; k++ {
		u := -1
		for i := 0; i < n; i++ {
			if !inTree[i] && (u == -1 || dist[i] < dist[u]) {
				u = i
			}
		}
		inTree[u] = true
		total += dist[u]
		for v := 0; v < n; v++ {
			if !inTree[v] {
				d := abs(points[u][0]-points[v][0]) + abs(points[u][1]-points[v][1])
				dist[v] = min(dist[v], d)
			}
		}
	}
	return total
}
```
**Complexity:** Time O(n²) · Space O(n)

#### Approach 2: Kruskal's with Union-Find
```go
func minCostConnectPointsKruskal(points [][]int) int {
	n := len(points)
	type edge struct{ w, u, v int }
	edges := make([]edge, 0, n*(n-1)/2)
	abs := func(x int) int {
		if x < 0 {
			return -x
		}
		return x
	}
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			w := abs(points[i][0]-points[j][0]) + abs(points[i][1]-points[j][1])
			edges = append(edges, edge{w, i, j})
		}
	}
	sort.Slice(edges, func(a, b int) bool { return edges[a].w < edges[b].w })
	parent := make([]int, n)
	for i := range parent {
		parent[i] = i
	}
	var find func(int) int
	find = func(x int) int {
		if parent[x] != x {
			parent[x] = find(parent[x])
		}
		return parent[x]
	}
	total, used := 0, 0
	for _, e := range edges {
		a, b := find(e.u), find(e.v)
		if a != b {
			parent[a] = b
			total += e.w
			used++
			if used == n-1 {
				break
			}
		}
	}
	return total
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
```go
type distItem struct{ node, dist int }
type distHeap []distItem

func (h distHeap) Len() int           { return len(h) }
func (h distHeap) Less(i, j int) bool { return h[i].dist < h[j].dist }
func (h distHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *distHeap) Push(x any)        { *h = append(*h, x.(distItem)) }
func (h *distHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func networkDelayTime(times [][]int, n int, k int) int {
	adj := make([][][2]int, n+1)
	for _, t := range times {
		adj[t[0]] = append(adj[t[0]], [2]int{t[1], t[2]})
	}
	dist := make([]int, n+1)
	for i := range dist {
		dist[i] = math.MaxInt
	}
	dist[k] = 0
	h := &distHeap{{k, 0}}
	for h.Len() > 0 {
		cur := heap.Pop(h).(distItem)
		if cur.dist > dist[cur.node] {
			continue // stale entry
		}
		for _, e := range adj[cur.node] {
			nd := cur.dist + e[1]
			if nd < dist[e[0]] {
				dist[e[0]] = nd
				heap.Push(h, distItem{e[0], nd})
			}
		}
	}
	ans := 0
	for i := 1; i <= n; i++ {
		if dist[i] == math.MaxInt {
			return -1
		}
		ans = max(ans, dist[i])
	}
	return ans
}
```
**Complexity:** Time O(E log V) · Space O(V + E)

#### Approach 2: Bellman-Ford
```go
func networkDelayTimeBF(times [][]int, n int, k int) int {
	const inf = math.MaxInt / 2
	dist := make([]int, n+1)
	for i := range dist {
		dist[i] = inf
	}
	dist[k] = 0
	for i := 1; i < n; i++ {
		changed := false
		for _, t := range times {
			if dist[t[0]]+t[2] < dist[t[1]] {
				dist[t[1]] = dist[t[0]] + t[2]
				changed = true
			}
		}
		if !changed {
			break
		}
	}
	ans := 0
	for i := 1; i <= n; i++ {
		if dist[i] == inf {
			return -1
		}
		ans = max(ans, dist[i])
	}
	return ans
}
```
**Complexity:** Time O(V · E) · Space O(V)

**Also valid:** Floyd–Warshall in O(V³), which is fine since `n ≤ 100`.

---

### 778. Swim in Rising Water
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/swim-in-rising-water/)

**Problem:** In an `n × n` grid, `grid[i][j]` is the elevation. At time `t` you can swim between adjacent cells whose elevations are both ≤ `t`. Return the least time at which you can get from `(0,0)` to `(n-1,n-1)`.

**Key insight:** Minimize the maximum elevation along a path. Use Dijkstra where a path's cost is `max(cost so far, next cell)`, or binary search on `t` with a BFS check.

#### Approach 1: Dijkstra / Min-Heap on the Path Maximum (Optimal)
```go
type cell struct{ t, r, c int }
type cellHeap []cell

func (h cellHeap) Len() int           { return len(h) }
func (h cellHeap) Less(i, j int) bool { return h[i].t < h[j].t }
func (h cellHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *cellHeap) Push(x any)        { *h = append(*h, x.(cell)) }
func (h *cellHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func swimInWater(grid [][]int) int {
	n := len(grid)
	seen := make([][]bool, n)
	for i := range seen {
		seen[i] = make([]bool, n)
	}
	h := &cellHeap{{grid[0][0], 0, 0}}
	seen[0][0] = true
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for {
		cur := heap.Pop(h).(cell)
		if cur.r == n-1 && cur.c == n-1 {
			return cur.t
		}
		for _, d := range dirs {
			nr, nc := cur.r+d[0], cur.c+d[1]
			if nr >= 0 && nc >= 0 && nr < n && nc < n && !seen[nr][nc] {
				seen[nr][nc] = true
				heap.Push(h, cell{max(cur.t, grid[nr][nc]), nr, nc})
			}
		}
	}
}
```
**Complexity:** Time O(n² log n) · Space O(n²)

#### Approach 2: Binary Search on Time + BFS
```go
func swimInWaterBS(grid [][]int) int {
	n := len(grid)
	canReach := func(t int) bool {
		if grid[0][0] > t {
			return false
		}
		seen := make([]bool, n*n)
		seen[0] = true
		q := []int{0}
		for len(q) > 0 {
			p := q[0]
			q = q[1:]
			r, c := p/n, p%n
			if r == n-1 && c == n-1 {
				return true
			}
			for _, d := range [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}} {
				nr, nc := r+d[0], c+d[1]
				if nr >= 0 && nc >= 0 && nr < n && nc < n && !seen[nr*n+nc] && grid[nr][nc] <= t {
					seen[nr*n+nc] = true
					q = append(q, nr*n+nc)
				}
			}
		}
		return false
	}
	lo, hi := 0, n*n-1
	for lo < hi {
		mid := (lo + hi) / 2
		if canReach(mid) {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
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
```go
func alienOrder(words []string) string {
	adj := map[byte]map[byte]bool{}
	indeg := map[byte]int{}
	for _, w := range words {
		for i := 0; i < len(w); i++ {
			if _, ok := indeg[w[i]]; !ok {
				indeg[w[i]] = 0
				adj[w[i]] = map[byte]bool{}
			}
		}
	}
	for i := 0; i+1 < len(words); i++ {
		a, b := words[i], words[i+1]
		minLen := min(len(a), len(b))
		if len(a) > len(b) && a[:minLen] == b[:minLen] {
			return "" // a word appears before its own prefix
		}
		for j := 0; j < minLen; j++ {
			if a[j] != b[j] {
				if !adj[a[j]][b[j]] {
					adj[a[j]][b[j]] = true
					indeg[b[j]]++
				}
				break
			}
		}
	}
	q := []byte{}
	for c, d := range indeg {
		if d == 0 {
			q = append(q, c)
		}
	}
	var sb strings.Builder
	for len(q) > 0 {
		c := q[0]
		q = q[1:]
		sb.WriteByte(c)
		for nxt := range adj[c] {
			indeg[nxt]--
			if indeg[nxt] == 0 {
				q = append(q, nxt)
			}
		}
	}
	if sb.Len() != len(indeg) {
		return "" // cycle
	}
	return sb.String()
}
```
**Complexity:** Time O(C), where C is the total number of characters · Space O(U + min(U², N)) for U unique letters

#### Approach 2: DFS Post-Order with Cycle Detection
```go
func alienOrderDFS(words []string) string {
	adj := map[byte][]byte{}
	for _, w := range words {
		for i := 0; i < len(w); i++ {
			if _, ok := adj[w[i]]; !ok {
				adj[w[i]] = nil
			}
		}
	}
	for i := 0; i+1 < len(words); i++ {
		a, b := words[i], words[i+1]
		minLen := min(len(a), len(b))
		if len(a) > len(b) && a[:minLen] == b[:minLen] {
			return ""
		}
		for j := 0; j < minLen; j++ {
			if a[j] != b[j] {
				adj[a[j]] = append(adj[a[j]], b[j])
				break
			}
		}
	}
	state := map[byte]int{} // 1 = visiting, 2 = done
	out := []byte{}
	var dfs func(byte) bool
	dfs = func(c byte) bool {
		if state[c] == 1 {
			return false
		}
		if state[c] == 2 {
			return true
		}
		state[c] = 1
		for _, n := range adj[c] {
			if !dfs(n) {
				return false
			}
		}
		state[c] = 2
		out = append(out, c)
		return true
	}
	for c := range adj {
		if !dfs(c) {
			return ""
		}
	}
	for i, j := 0, len(out)-1; i < j; i, j = i+1, j-1 {
		out[i], out[j] = out[j], out[i]
	}
	return string(out)
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
```go
func findCheapestPrice(n int, flights [][]int, src int, dst int, k int) int {
	const inf = math.MaxInt / 2
	prices := make([]int, n)
	for i := range prices {
		prices[i] = inf
	}
	prices[src] = 0
	for i := 0; i <= k; i++ {
		tmp := append([]int{}, prices...)
		for _, f := range flights {
			if prices[f[0]] == inf {
				continue
			}
			if prices[f[0]]+f[2] < tmp[f[1]] {
				tmp[f[1]] = prices[f[0]] + f[2]
			}
		}
		prices = tmp
	}
	if prices[dst] == inf {
		return -1
	}
	return prices[dst]
}
```
**Complexity:** Time O(k · E) · Space O(n)

#### Approach 2: BFS by Number of Stops
```go
func findCheapestPriceBFS(n int, flights [][]int, src int, dst int, k int) int {
	adj := make([][][2]int, n)
	for _, f := range flights {
		adj[f[0]] = append(adj[f[0]], [2]int{f[1], f[2]})
	}
	best := make([]int, n)
	for i := range best {
		best[i] = math.MaxInt
	}
	best[src] = 0
	q := [][2]int{{src, 0}} // {node, cost}
	for stops := 0; stops <= k && len(q) > 0; stops++ {
		next := [][2]int{}
		for _, cur := range q {
			for _, e := range adj[cur[0]] {
				nc := cur[1] + e[1]
				if nc < best[e[0]] {
					best[e[0]] = nc
					next = append(next, [2]int{e[0], nc})
				}
			}
		}
		q = next
	}
	if best[dst] == math.MaxInt {
		return -1
	}
	return best[dst]
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
```go
func climbStairsMemo(n int) int {
	memo := make([]int, n+1)
	var f func(int) int
	f = func(i int) int {
		if i <= 2 {
			return i
		}
		if memo[i] == 0 {
			memo[i] = f(i-1) + f(i-2)
		}
		return memo[i]
	}
	return f(n)
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: Bottom-Up, O(1) Space (Optimal)
```go
func climbStairs(n int) int {
	a, b := 1, 1 // ways(0), ways(1)
	for i := 2; i <= n; i++ {
		a, b = b, a+b
	}
	return b
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 3: Matrix Exponentiation
```go
func climbStairsMatrix(n int) int {
	type mat [2][2]int
	mul := func(x, y mat) mat {
		return mat{
			{x[0][0]*y[0][0] + x[0][1]*y[1][0], x[0][0]*y[0][1] + x[0][1]*y[1][1]},
			{x[1][0]*y[0][0] + x[1][1]*y[1][0], x[1][0]*y[0][1] + x[1][1]*y[1][1]},
		}
	}
	res, base := mat{{1, 0}, {0, 1}}, mat{{1, 1}, {1, 0}}
	for p := n; p > 0; p >>= 1 {
		if p&1 == 1 {
			res = mul(res, base)
		}
		base = mul(base, base)
	}
	return res[0][0] // F(n+1)
}
```
**Complexity:** Time O(log n) · Space O(1)

---

### 746. Min Cost Climbing Stairs
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/min-cost-climbing-stairs/)

**Problem:** `cost[i]` is the price of stepping on stair `i`. After paying, you can climb 1 or 2 steps. You may start on stair 0 or stair 1. Return the minimum cost to get past the last stair.

**Example:** `[10,15,20]` → `15`

#### Approach 1: Tabulation
```go
func minCostClimbingStairsTab(cost []int) int {
	n := len(cost)
	dp := make([]int, n+1) // dp[i] = min cost to reach stair i
	for i := 2; i <= n; i++ {
		dp[i] = min(dp[i-1]+cost[i-1], dp[i-2]+cost[i-2])
	}
	return dp[n]
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: O(1) Space (Optimal)
```go
func minCostClimbingStairs(cost []int) int {
	a, b := 0, 0
	for i := 2; i <= len(cost); i++ {
		a, b = b, min(b+cost[i-1], a+cost[i-2])
	}
	return b
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
```go
func robMemo(nums []int) int {
	memo := make([]int, len(nums))
	for i := range memo {
		memo[i] = -1
	}
	var f func(int) int
	f = func(i int) int {
		if i < 0 {
			return 0
		}
		if memo[i] == -1 {
			memo[i] = max(f(i-1), f(i-2)+nums[i])
		}
		return memo[i]
	}
	return f(len(nums) - 1)
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: O(1) Space (Optimal)
```go
func rob(nums []int) int {
	prev2, prev1 := 0, 0
	for _, n := range nums {
		prev2, prev1 = prev1, max(prev1, prev2+n)
	}
	return prev1
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
```go
func robLinear(nums []int) int {
	a, b := 0, 0
	for _, n := range nums {
		a, b = b, max(b, a+n)
	}
	return b
}

func robII(nums []int) int {
	if len(nums) == 1 {
		return nums[0]
	}
	return max(robLinear(nums[1:]), robLinear(nums[:len(nums)-1]))
}
```
**Complexity:** Time O(n) · Space O(1)

> On LeetCode the function is named `rob`. It's renamed `robII` here so it doesn't clash with problem 198's `rob`.

---

### 5. Longest Palindromic Substring
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/longest-palindromic-substring/)

**Problem:** Return the longest substring of `s` that is a palindrome.

**Example:** `"babad"` → `"bab"` (or `"aba"`); `"cbbd"` → `"bb"`

**Key insight:** Every palindrome has a center: a single character (odd length) or the gap between two characters (even length). Try each of the 2n−1 centers and expand outward.

#### Approach 1: Expand Around the Center (Optimal in practice)
```go
func longestPalindrome(s string) string {
	start, best := 0, 0
	expand := func(l, r int) {
		for l >= 0 && r < len(s) && s[l] == s[r] {
			l--
			r++
		}
		if r-l-1 > best {
			start, best = l+1, r-l-1
		}
	}
	for i := 0; i < len(s); i++ {
		expand(i, i)   // odd length
		expand(i, i+1) // even length
	}
	return s[start : start+best]
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: DP Table
```go
func longestPalindromeDP(s string) string {
	n := len(s)
	dp := make([][]bool, n)
	for i := range dp {
		dp[i] = make([]bool, n)
	}
	start, best := 0, 1
	for i := n - 1; i >= 0; i-- {
		for j := i; j < n; j++ {
			if s[i] == s[j] && (j-i < 3 || dp[i+1][j-1]) {
				dp[i][j] = true
				if j-i+1 > best {
					start, best = i, j-i+1
				}
			}
		}
	}
	return s[start : start+best]
}
```
**Complexity:** Time O(n²) · Space O(n²)

#### Approach 3: Manacher's Algorithm (theoretically optimal)
```go
func longestPalindromeManacher(s string) string {
	// transform "abc" into "^#a#b#c#$" so every palindrome has odd length
	t := make([]byte, 0, 2*len(s)+3)
	t = append(t, '^')
	for i := 0; i < len(s); i++ {
		t = append(t, '#', s[i])
	}
	t = append(t, '#', '$')
	p := make([]int, len(t))
	center, right := 0, 0
	for i := 1; i < len(t)-1; i++ {
		if i < right {
			p[i] = min(right-i, p[2*center-i])
		}
		for t[i+p[i]+1] == t[i-p[i]-1] {
			p[i]++
		}
		if i+p[i] > right {
			center, right = i, i+p[i]
		}
	}
	maxLen, ci := 0, 0
	for i, v := range p {
		if v > maxLen {
			maxLen, ci = v, i
		}
	}
	begin := (ci - maxLen) / 2
	return s[begin : begin+maxLen]
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 647. Palindromic Substrings
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/palindromic-substrings/)

**Problem:** Count the substrings of `s` that are palindromes. Substrings at different positions count separately.

**Example:** `"aaa"` → `6`

#### Approach 1: Expand Around the Center (Optimal in practice)
```go
func countSubstrings(s string) int {
	count := 0
	for c := 0; c < 2*len(s)-1; c++ {
		l, r := c/2, c/2+c%2
		for l >= 0 && r < len(s) && s[l] == s[r] {
			count++
			l--
			r++
		}
	}
	return count
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: DP Table
```go
func countSubstringsDP(s string) int {
	n := len(s)
	dp := make([][]bool, n)
	for i := range dp {
		dp[i] = make([]bool, n)
	}
	count := 0
	for i := n - 1; i >= 0; i-- {
		for j := i; j < n; j++ {
			if s[i] == s[j] && (j-i < 3 || dp[i+1][j-1]) {
				dp[i][j] = true
				count++
			}
		}
	}
	return count
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
```go
func numDecodingsTab(s string) int {
	n := len(s)
	dp := make([]int, n+1)
	dp[0] = 1
	for i := 1; i <= n; i++ {
		if s[i-1] != '0' {
			dp[i] += dp[i-1]
		}
		if i >= 2 {
			two := int(s[i-2]-'0')*10 + int(s[i-1]-'0')
			if two >= 10 && two <= 26 {
				dp[i] += dp[i-2]
			}
		}
	}
	return dp[n]
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 2: O(1) Space (Optimal)
```go
func numDecodings(s string) int {
	prev2, prev1 := 0, 1 // dp[i-2], dp[i-1]
	for i := 0; i < len(s); i++ {
		cur := 0
		if s[i] != '0' {
			cur = prev1
		}
		if i > 0 && (s[i-1] == '1' || (s[i-1] == '2' && s[i] <= '6')) {
			cur += prev2
		}
		prev2, prev1 = prev1, cur
	}
	return prev1
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
```go
func coinChange(coins []int, amount int) int {
	dp := make([]int, amount+1)
	for i := 1; i <= amount; i++ {
		dp[i] = amount + 1 // stands in for infinity
		for _, c := range coins {
			if c <= i {
				dp[i] = min(dp[i], dp[i-c]+1)
			}
		}
	}
	if dp[amount] > amount {
		return -1
	}
	return dp[amount]
}
```
**Complexity:** Time O(amount · len(coins)) · Space O(amount)

#### Approach 2: BFS over Amounts (shortest path)
```go
func coinChangeBFS(coins []int, amount int) int {
	if amount == 0 {
		return 0
	}
	seen := make([]bool, amount+1)
	q := []int{0}
	seen[0] = true
	for steps := 1; len(q) > 0; steps++ {
		next := []int{}
		for _, cur := range q {
			for _, c := range coins {
				v := cur + c
				if v == amount {
					return steps
				}
				if v < amount && !seen[v] {
					seen[v] = true
					next = append(next, v)
				}
			}
		}
		q = next
	}
	return -1
}
```
**Complexity:** Time O(amount · len(coins)) · Space O(amount)

#### Approach 3: Top-Down Memoization
```go
func coinChangeMemo(coins []int, amount int) int {
	memo := map[int]int{}
	var f func(int) int
	f = func(a int) int {
		if a == 0 {
			return 0
		}
		if a < 0 {
			return -1
		}
		if v, ok := memo[a]; ok {
			return v
		}
		best := math.MaxInt
		for _, c := range coins {
			if r := f(a - c); r >= 0 && r+1 < best {
				best = r + 1
			}
		}
		if best == math.MaxInt {
			best = -1
		}
		memo[a] = best
		return best
	}
	return f(amount)
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
```go
func maxProduct(nums []int) int {
	best, curMax, curMin := nums[0], nums[0], nums[0]
	for _, n := range nums[1:] {
		if n < 0 {
			curMax, curMin = curMin, curMax
		}
		curMax = max(n, curMax*n)
		curMin = min(n, curMin*n)
		best = max(best, curMax)
	}
	return best
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Prefix and Suffix Products
The best subarray always starts at an edge of the array or of a zero-bounded segment.
```go
func maxProductPrefixSuffix(nums []int) int {
	n := len(nums)
	best := nums[0]
	pre, suf := 1, 1
	for i := 0; i < n; i++ {
		if pre == 0 {
			pre = 1
		}
		if suf == 0 {
			suf = 1
		}
		pre *= nums[i]
		suf *= nums[n-1-i]
		best = max(best, pre, suf)
	}
	return best
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
```go
func wordBreak(s string, wordDict []string) bool {
	dict := map[string]bool{}
	maxLen := 0
	for _, w := range wordDict {
		dict[w] = true
		maxLen = max(maxLen, len(w))
	}
	dp := make([]bool, len(s)+1)
	dp[0] = true
	for i := 1; i <= len(s); i++ {
		for j := i - 1; j >= 0 && i-j <= maxLen; j-- {
			if dp[j] && dict[s[j:i]] {
				dp[i] = true
				break
			}
		}
	}
	return dp[len(s)]
}
```
**Complexity:** Time O(n · L), where L is the longest word (substring hashing adds O(L) per check) · Space O(n)

#### Approach 2: DP Iterating over Dictionary Words
```go
func wordBreakWords(s string, wordDict []string) bool {
	dp := make([]bool, len(s)+1)
	dp[len(s)] = true
	for i := len(s) - 1; i >= 0; i-- {
		for _, w := range wordDict {
			if i+len(w) <= len(s) && s[i:i+len(w)] == w && dp[i+len(w)] {
				dp[i] = true
				break
			}
		}
	}
	return dp[0]
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
```go
func lengthOfLISDP(nums []int) int {
	dp := make([]int, len(nums))
	best := 0
	for i := range nums {
		dp[i] = 1
		for j := 0; j < i; j++ {
			if nums[j] < nums[i] {
				dp[i] = max(dp[i], dp[j]+1)
			}
		}
		best = max(best, dp[i])
	}
	return best
}
```
**Complexity:** Time O(n²) · Space O(n)

#### Approach 2: Binary Search on Tails (Optimal)
```go
func lengthOfLIS(nums []int) int {
	tails := []int{}
	for _, n := range nums {
		i := sort.SearchInts(tails, n) // first index with tails[i] >= n
		if i == len(tails) {
			tails = append(tails, n)
		} else {
			tails[i] = n
		}
	}
	return len(tails)
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
```go
func canPartition(nums []int) bool {
	total := 0
	for _, n := range nums {
		total += n
	}
	if total%2 == 1 {
		return false
	}
	target := total / 2
	dp := make([]bool, target+1)
	dp[0] = true
	for _, n := range nums {
		for s := target; s >= n; s-- {
			dp[s] = dp[s] || dp[s-n]
		}
	}
	return dp[target]
}
```
**Complexity:** Time O(n · target) · Space O(target)

#### Approach 2: Bitset DP (fast)
`math/big.Int` works as an arbitrary-length bitset.
```go
func canPartitionBitset(nums []int) bool {
	total := 0
	for _, n := range nums {
		total += n
	}
	if total%2 == 1 {
		return false
	}
	bitset := big.NewInt(1) // bit i set means sum i is reachable
	for _, n := range nums {
		shifted := new(big.Int).Lsh(bitset, uint(n))
		bitset.Or(bitset, shifted)
	}
	return bitset.Bit(total/2) == 1
}
```
**Complexity:** Time O(n · total / 64) · Space O(total / 64)

#### Approach 3: Set of Reachable Sums
```go
func canPartitionSet(nums []int) bool {
	total := 0
	for _, n := range nums {
		total += n
	}
	if total%2 == 1 {
		return false
	}
	target := total / 2
	reach := map[int]bool{0: true}
	for _, n := range nums {
		next := map[int]bool{}
		for s := range reach {
			next[s] = true
			if s+n == target {
				return true
			}
			if s+n < target {
				next[s+n] = true
			}
		}
		reach = next
	}
	return reach[target]
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
```go
func uniquePaths2D(m int, n int) int {
	dp := make([][]int, m)
	for i := range dp {
		dp[i] = make([]int, n)
		dp[i][0] = 1
	}
	for j := 0; j < n; j++ {
		dp[0][j] = 1
	}
	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			dp[i][j] = dp[i-1][j] + dp[i][j-1]
		}
	}
	return dp[m-1][n-1]
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D Rolling Row (Optimal DP)
```go
func uniquePaths(m int, n int) int {
	row := make([]int, n)
	for j := range row {
		row[j] = 1
	}
	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			row[j] += row[j-1]
		}
	}
	return row[n-1]
}
```
**Complexity:** Time O(m · n) · Space O(n)

#### Approach 3: Combinatorics C(m+n-2, m-1)
```go
func uniquePathsMath(m int, n int) int {
	res := 1
	k := min(m-1, n-1)
	total := m + n - 2
	for i := 1; i <= k; i++ {
		res = res * (total - k + i) / i // stays an integer at every step
	}
	return res
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
```go
func longestCommonSubsequence2D(text1 string, text2 string) int {
	m, n := len(text1), len(text2)
	dp := make([][]int, m+1)
	for i := range dp {
		dp[i] = make([]int, n+1)
	}
	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			if text1[i-1] == text2[j-1] {
				dp[i][j] = dp[i-1][j-1] + 1
			} else {
				dp[i][j] = max(dp[i-1][j], dp[i][j-1])
			}
		}
	}
	return dp[m][n]
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Two Rows (Optimal space)
```go
func longestCommonSubsequence(text1 string, text2 string) int {
	if len(text2) > len(text1) {
		text1, text2 = text2, text1
	}
	n := len(text2)
	prev, cur := make([]int, n+1), make([]int, n+1)
	for i := 1; i <= len(text1); i++ {
		for j := 1; j <= n; j++ {
			if text1[i-1] == text2[j-1] {
				cur[j] = prev[j-1] + 1
			} else {
				cur[j] = max(prev[j], cur[j-1])
			}
		}
		prev, cur = cur, prev
	}
	return prev[n]
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
```go
func maxProfitCooldown(prices []int) int {
	hold, sold, rest := math.MinInt/2, 0, 0
	for _, p := range prices {
		prevSold := sold
		sold = hold + p
		hold = max(hold, rest-p)
		rest = max(rest, prevSold)
	}
	return max(sold, rest)
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Memoized DFS (index, canBuy)
```go
func maxProfitCooldownMemo(prices []int) int {
	n := len(prices)
	memo := make([][2]int, n)
	seen := make([][2]bool, n)
	var dfs func(i, buying int) int
	dfs = func(i, buying int) int {
		if i >= n {
			return 0
		}
		if seen[i][buying] {
			return memo[i][buying]
		}
		cooldown := dfs(i+1, buying)
		var act int
		if buying == 1 {
			act = dfs(i+1, 0) - prices[i]
		} else {
			act = dfs(i+2, 1) + prices[i] // selling forces a one-day cooldown
		}
		memo[i][buying] = max(cooldown, act)
		seen[i][buying] = true
		return memo[i][buying]
	}
	return dfs(0, 1)
}
```
**Complexity:** Time O(n) · Space O(n)

> On LeetCode the function is named `maxProfit`.

---

### 518. Coin Change II
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/coin-change-ii/)

**Problem:** Return how many combinations of coins add up to `amount`. Coin supply is unlimited, and combinations are unordered.

**Example:** `amount = 5, coins = [1,2,5]` → `4`

**Key insight:** Loop over coins on the outside and amounts on the inside. That counts each combination once, not once per ordering.

#### Approach 1: 1-D DP (Optimal)
```go
func change(amount int, coins []int) int {
	dp := make([]int, amount+1)
	dp[0] = 1
	for _, c := range coins {
		for a := c; a <= amount; a++ {
			dp[a] += dp[a-c]
		}
	}
	return dp[amount]
}
```
**Complexity:** Time O(n · amount) · Space O(amount)

#### Approach 2: 2-D DP (clearer state)
```go
func change2D(amount int, coins []int) int {
	n := len(coins)
	dp := make([][]int, n+1) // dp[i][a] = ways using the first i coins
	for i := range dp {
		dp[i] = make([]int, amount+1)
		dp[i][0] = 1
	}
	for i := 1; i <= n; i++ {
		for a := 1; a <= amount; a++ {
			dp[i][a] = dp[i-1][a]
			if a >= coins[i-1] {
				dp[i][a] += dp[i][a-coins[i-1]]
			}
		}
	}
	return dp[n][amount]
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
```go
func findTargetSumWaysMemo(nums []int, target int) int {
	memo := map[[2]int]int{}
	var dfs func(i, sum int) int
	dfs = func(i, sum int) int {
		if i == len(nums) {
			if sum == target {
				return 1
			}
			return 0
		}
		key := [2]int{i, sum}
		if v, ok := memo[key]; ok {
			return v
		}
		v := dfs(i+1, sum+nums[i]) + dfs(i+1, sum-nums[i])
		memo[key] = v
		return v
	}
	return dfs(0, 0)
}
```
**Complexity:** Time O(n · total) · Space O(n · total)

#### Approach 2: Subset-Sum Counting (Optimal)
```go
func findTargetSumWays(nums []int, target int) int {
	total := 0
	for _, n := range nums {
		total += n
	}
	if (total+target)%2 != 0 || total < abs494(target) {
		return 0
	}
	p := (total + target) / 2
	dp := make([]int, p+1)
	dp[0] = 1
	for _, n := range nums {
		for s := p; s >= n; s-- {
			dp[s] += dp[s-n]
		}
	}
	return dp[p]
}

func abs494(x int) int {
	if x < 0 {
		return -x
	}
	return x
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
```go
func isInterleave2D(s1 string, s2 string, s3 string) bool {
	m, n := len(s1), len(s2)
	if m+n != len(s3) {
		return false
	}
	dp := make([][]bool, m+1)
	for i := range dp {
		dp[i] = make([]bool, n+1)
	}
	dp[0][0] = true
	for i := 0; i <= m; i++ {
		for j := 0; j <= n; j++ {
			if i > 0 && s1[i-1] == s3[i+j-1] && dp[i-1][j] {
				dp[i][j] = true
			}
			if j > 0 && s2[j-1] == s3[i+j-1] && dp[i][j-1] {
				dp[i][j] = true
			}
		}
	}
	return dp[m][n]
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D DP (Optimal space)
```go
func isInterleave(s1 string, s2 string, s3 string) bool {
	m, n := len(s1), len(s2)
	if m+n != len(s3) {
		return false
	}
	dp := make([]bool, n+1)
	for i := 0; i <= m; i++ {
		for j := 0; j <= n; j++ {
			switch {
			case i == 0 && j == 0:
				dp[j] = true
			case i == 0:
				dp[j] = dp[j-1] && s2[j-1] == s3[j-1]
			case j == 0:
				dp[j] = dp[j] && s1[i-1] == s3[i-1]
			default:
				dp[j] = (dp[j] && s1[i-1] == s3[i+j-1]) || (dp[j-1] && s2[j-1] == s3[i+j-1])
			}
		}
	}
	return dp[n]
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
```go
func longestIncreasingPath(matrix [][]int) int {
	m, n := len(matrix), len(matrix[0])
	memo := make([][]int, m)
	for i := range memo {
		memo[i] = make([]int, n)
	}
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	var dfs func(r, c int) int
	dfs = func(r, c int) int {
		if memo[r][c] != 0 {
			return memo[r][c]
		}
		best := 1
		for _, d := range dirs {
			nr, nc := r+d[0], c+d[1]
			if nr >= 0 && nc >= 0 && nr < m && nc < n && matrix[nr][nc] > matrix[r][c] {
				best = max(best, 1+dfs(nr, nc))
			}
		}
		memo[r][c] = best
		return best
	}
	ans := 0
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			ans = max(ans, dfs(r, c))
		}
	}
	return ans
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Topological Sort (peel layers of local maxima)
```go
func longestIncreasingPathTopo(matrix [][]int) int {
	m, n := len(matrix), len(matrix[0])
	dirs := [4][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	outdeg := make([][]int, m)
	for i := range outdeg {
		outdeg[i] = make([]int, n)
	}
	q := [][2]int{}
	for r := 0; r < m; r++ {
		for c := 0; c < n; c++ {
			for _, d := range dirs {
				nr, nc := r+d[0], c+d[1]
				if nr >= 0 && nc >= 0 && nr < m && nc < n && matrix[nr][nc] > matrix[r][c] {
					outdeg[r][c]++
				}
			}
			if outdeg[r][c] == 0 {
				q = append(q, [2]int{r, c})
			}
		}
	}
	layers := 0
	for len(q) > 0 {
		layers++
		next := [][2]int{}
		for _, p := range q {
			for _, d := range dirs {
				nr, nc := p[0]+d[0], p[1]+d[1]
				if nr >= 0 && nc >= 0 && nr < m && nc < n && matrix[nr][nc] < matrix[p[0]][p[1]] {
					outdeg[nr][nc]--
					if outdeg[nr][nc] == 0 {
						next = append(next, [2]int{nr, nc})
					}
				}
			}
		}
		q = next
	}
	return layers
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
```go
func numDistinct2D(s string, t string) int {
	m, n := len(s), len(t)
	dp := make([][]int, m+1)
	for i := range dp {
		dp[i] = make([]int, n+1)
		dp[i][0] = 1 // the empty t matches once
	}
	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			dp[i][j] = dp[i-1][j]
			if s[i-1] == t[j-1] {
				dp[i][j] += dp[i-1][j-1]
			}
		}
	}
	return dp[m][n]
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D DP, iterate j backward (Optimal space)
```go
func numDistinct(s string, t string) int {
	n := len(t)
	dp := make([]int, n+1)
	dp[0] = 1
	for i := 0; i < len(s); i++ {
		for j := n; j >= 1; j-- {
			if s[i] == t[j-1] {
				dp[j] += dp[j-1]
			}
		}
	}
	return dp[n]
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
```go
func minDistance2D(word1 string, word2 string) int {
	m, n := len(word1), len(word2)
	dp := make([][]int, m+1)
	for i := range dp {
		dp[i] = make([]int, n+1)
		dp[i][0] = i
	}
	for j := 0; j <= n; j++ {
		dp[0][j] = j
	}
	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			if word1[i-1] == word2[j-1] {
				dp[i][j] = dp[i-1][j-1]
			} else {
				dp[i][j] = 1 + min(dp[i-1][j], dp[i][j-1], dp[i-1][j-1])
			}
		}
	}
	return dp[m][n]
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: 1-D DP (Optimal space)
```go
func minDistance(word1 string, word2 string) int {
	n := len(word2)
	dp := make([]int, n+1)
	for j := range dp {
		dp[j] = j
	}
	for i := 1; i <= len(word1); i++ {
		diag := dp[0] // dp[i-1][j-1]
		dp[0] = i
		for j := 1; j <= n; j++ {
			tmp := dp[j]
			if word1[i-1] == word2[j-1] {
				dp[j] = diag
			} else {
				dp[j] = 1 + min(dp[j], dp[j-1], diag)
			}
			diag = tmp
		}
	}
	return dp[n]
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
```go
func maxCoins(nums []int) int {
	vals := make([]int, 0, len(nums)+2)
	vals = append(vals, 1)
	vals = append(vals, nums...)
	vals = append(vals, 1)
	n := len(vals)
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, n)
	}
	for length := 2; length < n; length++ { // gap between l and r
		for l := 0; l+length < n; l++ {
			r := l + length
			for k := l + 1; k < r; k++ {
				dp[l][r] = max(dp[l][r], dp[l][k]+vals[l]*vals[k]*vals[r]+dp[k][r])
			}
		}
	}
	return dp[0][n-1]
}
```
**Complexity:** Time O(n³) · Space O(n²)

#### Approach 2: Top-Down Memoization
```go
func maxCoinsMemo(nums []int) int {
	vals := append(append([]int{1}, nums...), 1)
	n := len(vals)
	memo := make([][]int, n)
	for i := range memo {
		memo[i] = make([]int, n)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}
	var f func(l, r int) int
	f = func(l, r int) int {
		if l+1 == r {
			return 0
		}
		if memo[l][r] >= 0 {
			return memo[l][r]
		}
		best := 0
		for k := l + 1; k < r; k++ {
			best = max(best, f(l, k)+vals[l]*vals[k]*vals[r]+f(k, r))
		}
		memo[l][r] = best
		return best
	}
	return f(0, n-1)
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
```go
func isMatchMemo(s string, p string) bool {
	memo := map[[2]int]bool{}
	var dp func(i, j int) bool
	dp = func(i, j int) bool {
		if j == len(p) {
			return i == len(s)
		}
		key := [2]int{i, j}
		if v, ok := memo[key]; ok {
			return v
		}
		first := i < len(s) && (p[j] == s[i] || p[j] == '.')
		var res bool
		if j+1 < len(p) && p[j+1] == '*' {
			res = dp(i, j+2) || (first && dp(i+1, j))
		} else {
			res = first && dp(i+1, j+1)
		}
		memo[key] = res
		return res
	}
	return dp(0, 0)
}
```
**Complexity:** Time O(m · n) · Space O(m · n)

#### Approach 2: Bottom-Up DP (Optimal)
```go
func isMatch(s string, p string) bool {
	m, n := len(s), len(p)
	dp := make([][]bool, m+1)
	for i := range dp {
		dp[i] = make([]bool, n+1)
	}
	dp[m][n] = true
	for i := m; i >= 0; i-- {
		for j := n - 1; j >= 0; j-- {
			first := i < m && (p[j] == s[i] || p[j] == '.')
			if j+1 < n && p[j+1] == '*' {
				dp[i][j] = dp[i][j+2] || (first && dp[i+1][j])
			} else {
				dp[i][j] = first && dp[i+1][j+1]
			}
		}
	}
	return dp[0][0]
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
```go
func maxSubArray(nums []int) int {
	best, cur := nums[0], 0
	for _, n := range nums {
		cur = max(cur+n, n)
		best = max(best, cur)
	}
	return best
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Divide and Conquer (the follow-up)
```go
func maxSubArrayDC(nums []int) int {
	var solve func(l, r int) int
	solve = func(l, r int) int {
		if l == r {
			return nums[l]
		}
		mid := (l + r) / 2
		leftBest, sum := math.MinInt, 0
		for i := mid; i >= l; i-- {
			sum += nums[i]
			leftBest = max(leftBest, sum)
		}
		rightBest := math.MinInt
		sum = 0
		for i := mid + 1; i <= r; i++ {
			sum += nums[i]
			rightBest = max(rightBest, sum)
		}
		return max(solve(l, mid), solve(mid+1, r), leftBest+rightBest)
	}
	return solve(0, len(nums)-1)
}
```
**Complexity:** Time O(n log n) · Space O(log n)

#### Approach 3: Brute Force
```go
func maxSubArrayBrute(nums []int) int {
	best := math.MinInt
	for i := range nums {
		sum := 0
		for j := i; j < len(nums); j++ {
			sum += nums[j]
			best = max(best, sum)
		}
	}
	return best
}
```
**Complexity:** Time O(n²) · Space O(1)

---

### 55. Jump Game
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/jump-game/)

**Problem:** `nums[i]` is the farthest you can jump from index `i`. Starting at index 0, can you reach the last index?

**Example:** `[2,3,1,1,4]` → `true`; `[3,2,1,0,4]` → `false`

#### Approach 1: Greedy, Shift the Goal Backward (Optimal)
```go
func canJump(nums []int) bool {
	goal := len(nums) - 1
	for i := len(nums) - 2; i >= 0; i-- {
		if i+nums[i] >= goal {
			goal = i
		}
	}
	return goal == 0
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Greedy, Track the Farthest Reach Forward
```go
func canJumpForward(nums []int) bool {
	reach := 0
	for i := 0; i < len(nums); i++ {
		if i > reach {
			return false
		}
		reach = max(reach, i+nums[i])
	}
	return true
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 3: DP (slower, shown for contrast)
```go
func canJumpDP(nums []int) bool {
	n := len(nums)
	good := make([]bool, n)
	good[n-1] = true
	for i := n - 2; i >= 0; i-- {
		for j := 1; j <= nums[i] && i+j < n; j++ {
			if good[i+j] {
				good[i] = true
				break
			}
		}
	}
	return good[0]
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
```go
func jump(nums []int) int {
	jumps, curEnd, farthest := 0, 0, 0
	for i := 0; i < len(nums)-1; i++ {
		farthest = max(farthest, i+nums[i])
		if i == curEnd {
			jumps++
			curEnd = farthest
		}
	}
	return jumps
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: DP
```go
func jumpDP(nums []int) int {
	n := len(nums)
	dp := make([]int, n)
	for i := 1; i < n; i++ {
		dp[i] = math.MaxInt
	}
	for i := 0; i < n; i++ {
		for j := 1; j <= nums[i] && i+j < n; j++ {
			dp[i+j] = min(dp[i+j], dp[i]+1)
		}
	}
	return dp[n-1]
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
```go
func canCompleteCircuit(gas []int, cost []int) int {
	total, tank, start := 0, 0, 0
	for i := range gas {
		diff := gas[i] - cost[i]
		total += diff
		tank += diff
		if tank < 0 {
			start, tank = i+1, 0
		}
	}
	if total < 0 {
		return -1
	}
	return start
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Brute Force Simulation
```go
func canCompleteCircuitBrute(gas []int, cost []int) int {
	n := len(gas)
	for s := 0; s < n; s++ {
		tank, ok := 0, true
		for k := 0; k < n; k++ {
			i := (s + k) % n
			tank += gas[i] - cost[i]
			if tank < 0 {
				ok = false
				break
			}
		}
		if ok {
			return s
		}
	}
	return -1
}
```
**Complexity:** Time O(n²) · Space O(1)

---

### 846. Hand of Straights
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/hand-of-straights/)

**Problem:** Can the cards in `hand` be rearranged into groups of `groupSize` consecutive values, using every card exactly once?

**Example:** `hand = [1,2,3,6,2,3,4,7,8], groupSize = 3` → `true` (`[1,2,3]`, `[2,3,4]`, `[6,7,8]`)

**Key insight:** The smallest remaining card must start a group, so build each group upward from the current minimum.

#### Approach 1: Sort + Count Map
```go
func isNStraightHand(hand []int, groupSize int) bool {
	if len(hand)%groupSize != 0 {
		return false
	}
	count := map[int]int{}
	for _, h := range hand {
		count[h]++
	}
	keys := make([]int, 0, len(count))
	for k := range count {
		keys = append(keys, k)
	}
	sort.Ints(keys)
	for _, k := range keys {
		need := count[k]
		if need == 0 {
			continue
		}
		for v := k; v < k+groupSize; v++ {
			if count[v] < need {
				return false
			}
			count[v] -= need
		}
	}
	return true
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Find the Start of Each Run (no sorting of keys)
For each card, walk down to the start of its consecutive run, then consume groups upward from there.
```go
func isNStraightHandRuns(hand []int, groupSize int) bool {
	if len(hand)%groupSize != 0 {
		return false
	}
	count := map[int]int{}
	for _, h := range hand {
		count[h]++
	}
	for _, h := range hand {
		start := h
		for count[start-1] > 0 {
			start--
		}
		for start <= h {
			for count[start] > 0 {
				for v := start; v < start+groupSize; v++ {
					if count[v] == 0 {
						return false
					}
					count[v]--
				}
			}
			start++
		}
	}
	return true
}
```
**Complexity:** Time O(n) amortized · Space O(n)

---

### 1899. Merge Triplets to Form Target Triplet
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/merge-triplets-to-form-target-triplet/)

**Problem:** You may merge two triplets by taking the element-wise maximum. Can you produce `target` from some of the given `triplets`?

**Example:** `triplets = [[2,5,3],[1,8,4],[1,7,5]], target = [2,7,5]` → `true`

**Key insight:** Throw out any triplet that exceeds `target` in any position. Among the rest, check that each of the three target values appears in its position in at least one triplet.

#### Approach: Greedy Filter (Optimal)
```go
func mergeTriplets(triplets [][]int, target []int) bool {
	var found [3]bool
	for _, t := range triplets {
		if t[0] > target[0] || t[1] > target[1] || t[2] > target[2] {
			continue
		}
		for i := 0; i < 3; i++ {
			if t[i] == target[i] {
				found[i] = true
			}
		}
	}
	return found[0] && found[1] && found[2]
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
```go
func partitionLabels(s string) []int {
	var last [26]int
	for i := 0; i < len(s); i++ {
		last[s[i]-'a'] = i
	}
	res := []int{}
	start, end := 0, 0
	for i := 0; i < len(s); i++ {
		end = max(end, last[s[i]-'a'])
		if i == end {
			res = append(res, end-start+1)
			start = i + 1
		}
	}
	return res
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
```go
func checkValidString(s string) bool {
	lo, hi := 0, 0
	for i := 0; i < len(s); i++ {
		switch s[i] {
		case '(':
			lo++
			hi++
		case ')':
			lo--
			hi--
		default: // '*'
			lo--
			hi++
		}
		if hi < 0 {
			return false
		}
		lo = max(lo, 0)
	}
	return lo == 0
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Two Stacks of Indices
```go
func checkValidStringStacks(s string) bool {
	open, star := []int{}, []int{}
	for i := 0; i < len(s); i++ {
		switch s[i] {
		case '(':
			open = append(open, i)
		case '*':
			star = append(star, i)
		default:
			if len(open) > 0 {
				open = open[:len(open)-1]
			} else if len(star) > 0 {
				star = star[:len(star)-1]
			} else {
				return false
			}
		}
	}
	for len(open) > 0 && len(star) > 0 {
		if open[len(open)-1] > star[len(star)-1] {
			return false // a '(' with no '*' after it to close it
		}
		open = open[:len(open)-1]
		star = star[:len(star)-1]
	}
	return len(open) == 0
}
```
**Complexity:** Time O(n) · Space O(n)

#### Approach 3: DP over (index, open count)
```go
func checkValidStringDP(s string) bool {
	n := len(s)
	dp := make([]bool, n+2) // dp[k]: is an open count of k reachable?
	dp[0] = true
	for i := 0; i < n; i++ {
		next := make([]bool, n+2)
		for k := 0; k <= n; k++ {
			if !dp[k] {
				continue
			}
			switch s[i] {
			case '(':
				next[k+1] = true
			case ')':
				if k > 0 {
					next[k-1] = true
				}
			default:
				next[k+1] = true
				next[k] = true
				if k > 0 {
					next[k-1] = true
				}
			}
		}
		dp = next
	}
	return dp[0]
}
```
**Complexity:** Time O(n²) · Space O(n)

---
## 16. Intervals

### 57. Insert Interval
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/insert-interval/)

**Problem:** `intervals` are non-overlapping and sorted by start. Insert `newInterval`, merging wherever needed, and return the result still sorted and non-overlapping.

**Example:** `[[1,3],[6,9]]` + `[2,5]` → `[[1,5],[6,9]]`

**Key insight:** Make three passes over the list: intervals entirely before the new one, intervals that overlap it (merge them), and intervals entirely after.

#### Approach 1: Linear Three-Phase Scan (Optimal)
```go
func insert(intervals [][]int, newInterval []int) [][]int {
	res := [][]int{}
	i, n := 0, len(intervals)
	for i < n && intervals[i][1] < newInterval[0] {
		res = append(res, intervals[i])
		i++
	}
	for i < n && intervals[i][0] <= newInterval[1] {
		newInterval[0] = min(newInterval[0], intervals[i][0])
		newInterval[1] = max(newInterval[1], intervals[i][1])
		i++
	}
	res = append(res, newInterval)
	return append(res, intervals[i:]...)
}
```
**Complexity:** Time O(n) · Space O(n) for the output

#### Approach 2: Binary Search for the Boundaries
```go
func insertBS(intervals [][]int, newInterval []int) [][]int {
	n := len(intervals)
	// first interval whose end >= new start
	l := sort.Search(n, func(i int) bool { return intervals[i][1] >= newInterval[0] })
	// first interval whose start > new end
	r := sort.Search(n, func(i int) bool { return intervals[i][0] > newInterval[1] })
	merged := []int{newInterval[0], newInterval[1]}
	if l < r {
		merged[0] = min(merged[0], intervals[l][0])
		merged[1] = max(merged[1], intervals[r-1][1])
	}
	res := make([][]int, 0, n-(r-l)+1)
	res = append(res, intervals[:l]...)
	res = append(res, merged)
	return append(res, intervals[r:]...)
}
```
**Complexity:** Time O(log n) to search plus O(n) to copy · Space O(n)

---

### 56. Merge Intervals
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/merge-intervals/)

**Problem:** Merge every overlapping interval and return the resulting non-overlapping intervals.

**Example:** `[[1,3],[2,6],[8,10],[15,18]]` → `[[1,6],[8,10],[15,18]]`

#### Approach 1: Sort + Merge (Optimal)
```go
func merge(intervals [][]int) [][]int {
	sort.Slice(intervals, func(i, j int) bool { return intervals[i][0] < intervals[j][0] })
	res := [][]int{intervals[0]}
	for _, iv := range intervals[1:] {
		last := res[len(res)-1]
		if iv[0] <= last[1] {
			last[1] = max(last[1], iv[1])
		} else {
			res = append(res, iv)
		}
	}
	return res
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Sweep over Start and End Events
```go
func mergeSweep(intervals [][]int) [][]int {
	n := len(intervals)
	starts, ends := make([]int, n), make([]int, n)
	for i, iv := range intervals {
		starts[i], ends[i] = iv[0], iv[1]
	}
	sort.Ints(starts)
	sort.Ints(ends)
	res := [][]int{}
	begin := 0
	for i := 0; i < n; i++ {
		// close a merged block when the next start begins after this end
		if i == n-1 || starts[i+1] > ends[i] {
			res = append(res, []int{starts[begin], ends[i]})
			begin = i + 1
		}
	}
	return res
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
```go
func eraseOverlapIntervals(intervals [][]int) int {
	sort.Slice(intervals, func(i, j int) bool { return intervals[i][1] < intervals[j][1] })
	removed, end := 0, math.MinInt
	for _, iv := range intervals {
		if iv[0] >= end {
			end = iv[1] // keep it
		} else {
			removed++
		}
	}
	return removed
}
```
**Complexity:** Time O(n log n) · Space O(1)

#### Approach 2: Sort by Start, Drop the One That Ends Later
```go
func eraseOverlapIntervalsByStart(intervals [][]int) int {
	sort.Slice(intervals, func(i, j int) bool { return intervals[i][0] < intervals[j][0] })
	removed, prevEnd := 0, intervals[0][1]
	for _, iv := range intervals[1:] {
		if iv[0] >= prevEnd {
			prevEnd = iv[1]
		} else {
			removed++
			prevEnd = min(prevEnd, iv[1])
		}
	}
	return removed
}
```
**Complexity:** Time O(n log n) · Space O(1)

---

### 252. Meeting Rooms (Premium)
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/meeting-rooms/) · Free mirror: LintCode 920

**Problem:** Given meeting time intervals, can one person attend every meeting (that is, do no two meetings overlap)?

**Example:** `[[0,30],[5,10],[15,20]]` → `false`

#### Approach: Sort by Start (Optimal)
```go
func canAttendMeetings(intervals [][]int) bool {
	sort.Slice(intervals, func(i, j int) bool { return intervals[i][0] < intervals[j][0] })
	for i := 1; i < len(intervals); i++ {
		if intervals[i][0] < intervals[i-1][1] {
			return false
		}
	}
	return true
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
```go
func minMeetingRooms(intervals [][]int) int {
	n := len(intervals)
	starts, ends := make([]int, n), make([]int, n)
	for i, iv := range intervals {
		starts[i], ends[i] = iv[0], iv[1]
	}
	sort.Ints(starts)
	sort.Ints(ends)
	rooms, best, e := 0, 0, 0
	for s := 0; s < n; s++ {
		for e < n && ends[e] <= starts[s] {
			rooms-- // that meeting has ended and freed its room
			e++
		}
		rooms++
		best = max(best, rooms)
	}
	return best
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 2: Min-Heap of End Times
```go
type endHeap []int

func (h endHeap) Len() int           { return len(h) }
func (h endHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h endHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *endHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *endHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func minMeetingRoomsHeap(intervals [][]int) int {
	sort.Slice(intervals, func(i, j int) bool { return intervals[i][0] < intervals[j][0] })
	h := &endHeap{}
	for _, iv := range intervals {
		if h.Len() > 0 && (*h)[0] <= iv[0] {
			heap.Pop(h) // reuse the room that frees up earliest
		}
		heap.Push(h, iv[1])
	}
	return h.Len()
}
```
**Complexity:** Time O(n log n) · Space O(n)

#### Approach 3: Line Sweep with a Difference Map
```go
func minMeetingRoomsSweep(intervals [][]int) int {
	delta := map[int]int{}
	for _, iv := range intervals {
		delta[iv[0]]++
		delta[iv[1]]--
	}
	times := make([]int, 0, len(delta))
	for t := range delta {
		times = append(times, t)
	}
	sort.Ints(times)
	cur, best := 0, 0
	for _, t := range times {
		cur += delta[t]
		best = max(best, cur)
	}
	return best
}
```
**Complexity:** Time O(n log n) · Space O(n)

---

### 1851. Minimum Interval to Include Each Query
**Difficulty:** Hard · [LeetCode](https://leetcode.com/problems/minimum-interval-to-include-each-query/)

**Problem:** For each `queries[j]`, find the size (`right - left + 1`) of the smallest interval that contains it, or `-1` if no interval does.

**Example:** `intervals = [[1,4],[2,4],[3,6],[4,4]], queries = [2,3,4,5]` → `[3,3,1,4]`

**Key insight:** Process queries in increasing order (offline). Push every interval whose start ≤ q onto a min-heap keyed by size, then pop from the top any interval whose end < q.

#### Approach: Sort + Min-Heap (Optimal)
```go
type ivItem struct{ size, end int }
type ivHeap []ivItem

func (h ivHeap) Len() int           { return len(h) }
func (h ivHeap) Less(i, j int) bool { return h[i].size < h[j].size }
func (h ivHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *ivHeap) Push(x any)        { *h = append(*h, x.(ivItem)) }
func (h *ivHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func minInterval(intervals [][]int, queries []int) []int {
	sort.Slice(intervals, func(i, j int) bool { return intervals[i][0] < intervals[j][0] })
	qIdx := make([]int, len(queries))
	for i := range qIdx {
		qIdx[i] = i
	}
	sort.Slice(qIdx, func(a, b int) bool { return queries[qIdx[a]] < queries[qIdx[b]] })
	res := make([]int, len(queries))
	h := &ivHeap{}
	i := 0
	for _, qi := range qIdx {
		q := queries[qi]
		for i < len(intervals) && intervals[i][0] <= q {
			heap.Push(h, ivItem{intervals[i][1] - intervals[i][0] + 1, intervals[i][1]})
			i++
		}
		for h.Len() > 0 && (*h)[0].end < q {
			heap.Pop(h)
		}
		if h.Len() > 0 {
			res[qi] = (*h)[0].size
		} else {
			res[qi] = -1
		}
	}
	return res
}
```
**Complexity:** Time O(n log n + q log q) · Space O(n + q)

**Brute force:** Scan every interval for every query: O(n · q).

---
## 17. Math & Geometry

### 48. Rotate Image
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/rotate-image/)

**Problem:** Rotate an `n × n` matrix 90° clockwise **in place**.

**Example:** `[[1,2,3],[4,5,6],[7,8,9]]` → `[[7,4,1],[8,5,2],[9,6,3]]`

**Key insight:** A clockwise rotation equals a transpose followed by reversing each row.

#### Approach 1: Transpose + Reverse Rows (Optimal)
```go
func rotate(matrix [][]int) {
	n := len(matrix)
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			matrix[i][j], matrix[j][i] = matrix[j][i], matrix[i][j]
		}
	}
	for _, row := range matrix {
		for l, r := 0, n-1; l < r; l, r = l+1, r-1 {
			row[l], row[r] = row[r], row[l]
		}
	}
}
```
**Complexity:** Time O(n²) · Space O(1)

#### Approach 2: Rotate Four Cells at a Time, Layer by Layer
```go
func rotateLayers(matrix [][]int) {
	n := len(matrix)
	for l, r := 0, n-1; l < r; l, r = l+1, r-1 {
		for i := 0; i < r-l; i++ {
			top, bottom := l, r
			tmp := matrix[top][l+i]                     // save top-left
			matrix[top][l+i] = matrix[bottom-i][l]      // bottom-left to top-left
			matrix[bottom-i][l] = matrix[bottom][r-i]   // bottom-right to bottom-left
			matrix[bottom][r-i] = matrix[top+i][r]      // top-right to bottom-right
			matrix[top+i][r] = tmp                      // top-left to top-right
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
```go
func spiralOrder(matrix [][]int) []int {
	top, bottom := 0, len(matrix)-1
	left, right := 0, len(matrix[0])-1
	res := make([]int, 0, len(matrix)*len(matrix[0]))
	for top <= bottom && left <= right {
		for c := left; c <= right; c++ {
			res = append(res, matrix[top][c])
		}
		top++
		for r := top; r <= bottom; r++ {
			res = append(res, matrix[r][right])
		}
		right--
		if top <= bottom {
			for c := right; c >= left; c-- {
				res = append(res, matrix[bottom][c])
			}
			bottom--
		}
		if left <= right {
			for r := bottom; r >= top; r-- {
				res = append(res, matrix[r][left])
			}
			left++
		}
	}
	return res
}
```
**Complexity:** Time O(m · n) · Space O(1) extra

#### Approach 2: Direction Vectors with a Visited Grid
```go
func spiralOrderDirs(matrix [][]int) []int {
	m, n := len(matrix), len(matrix[0])
	seen := make([][]bool, m)
	for i := range seen {
		seen[i] = make([]bool, n)
	}
	dr := [4]int{0, 1, 0, -1}
	dc := [4]int{1, 0, -1, 0}
	res := make([]int, 0, m*n)
	r, c, d := 0, 0, 0
	for k := 0; k < m*n; k++ {
		res = append(res, matrix[r][c])
		seen[r][c] = true
		nr, nc := r+dr[d], c+dc[d]
		if nr < 0 || nc < 0 || nr >= m || nc >= n || seen[nr][nc] {
			d = (d + 1) % 4
			nr, nc = r+dr[d], c+dc[d]
		}
		r, c = nr, nc
	}
	return res
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
```go
func setZeroesSets(matrix [][]int) {
	m, n := len(matrix), len(matrix[0])
	rows, cols := make([]bool, m), make([]bool, n)
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if matrix[i][j] == 0 {
				rows[i], cols[j] = true, true
			}
		}
	}
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if rows[i] || cols[j] {
				matrix[i][j] = 0
			}
		}
	}
}
```
**Complexity:** Time O(m · n) · Space O(m + n)

#### Approach 2: First Row and Column as Markers (Optimal)
```go
func setZeroes(matrix [][]int) {
	m, n := len(matrix), len(matrix[0])
	firstColZero := false
	for i := 0; i < m; i++ {
		if matrix[i][0] == 0 {
			firstColZero = true
		}
		for j := 1; j < n; j++ {
			if matrix[i][j] == 0 {
				matrix[i][0], matrix[0][j] = 0, 0
			}
		}
	}
	for i := m - 1; i >= 0; i-- { // go bottom-up so row 0's markers are used before row 0 is zeroed
		for j := n - 1; j >= 1; j-- {
			if matrix[i][0] == 0 || matrix[0][j] == 0 {
				matrix[i][j] = 0
			}
		}
		if firstColZero {
			matrix[i][0] = 0
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

#### Approach 1: Hash Set
```go
func digitSquareSum(n int) int {
	s := 0
	for n > 0 {
		d := n % 10
		s += d * d
		n /= 10
	}
	return s
}

func isHappySet(n int) bool {
	seen := map[int]bool{}
	for n != 1 && !seen[n] {
		seen[n] = true
		n = digitSquareSum(n)
	}
	return n == 1
}
```
**Complexity:** Time O(log n) per step, with only a few steps · Space O(log n)

#### Approach 2: Floyd's Cycle Detection (Optimal)
```go
func isHappy(n int) bool {
	slow, fast := n, digitSquareSum(n)
	for fast != 1 && slow != fast {
		slow = digitSquareSum(slow)
		fast = digitSquareSum(digitSquareSum(fast))
	}
	return fast == 1
}
```
**Complexity:** Time O(log n) · Space O(1)

---

### 66. Plus One
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/plus-one/)

**Problem:** A large integer is stored as an array of digits, most significant first. Add one and return the resulting digits.

**Example:** `[1,2,9]` → `[1,3,0]`; `[9,9]` → `[1,0,0]`

#### Approach: Propagate the Carry from the Right (Optimal)
```go
func plusOne(digits []int) []int {
	for i := len(digits) - 1; i >= 0; i-- {
		if digits[i] < 9 {
			digits[i]++
			return digits
		}
		digits[i] = 0
	}
	// every digit was 9
	return append([]int{1}, digits...)
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
```go
func myPow(x float64, n int) float64 {
	p := int64(n) // avoids overflow when negating MinInt32
	if p < 0 {
		x, p = 1/x, -p
	}
	res := 1.0
	for p > 0 {
		if p&1 == 1 {
			res *= x
		}
		x *= x
		p >>= 1
	}
	return res
}
```
**Complexity:** Time O(log n) · Space O(1)

#### Approach 2: Recursive
```go
func myPowRec(x float64, n int) float64 {
	var f func(float64, int64) float64
	f = func(b float64, e int64) float64 {
		if e == 0 {
			return 1
		}
		half := f(b, e/2)
		if e%2 == 0 {
			return half * half
		}
		return half * half * b
	}
	e := int64(n)
	if e < 0 {
		return 1 / f(x, -e)
	}
	return f(x, e)
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
```go
func multiply(num1 string, num2 string) string {
	if num1 == "0" || num2 == "0" {
		return "0"
	}
	m, n := len(num1), len(num2)
	pos := make([]int, m+n)
	for i := m - 1; i >= 0; i-- {
		for j := n - 1; j >= 0; j-- {
			mul := int(num1[i]-'0') * int(num2[j]-'0')
			sum := mul + pos[i+j+1]
			pos[i+j+1] = sum % 10
			pos[i+j] += sum / 10
		}
	}
	var sb strings.Builder
	for i, d := range pos {
		if i == 0 && d == 0 {
			continue // at most one leading zero
		}
		sb.WriteByte(byte('0' + d))
	}
	return sb.String()
}
```
**Complexity:** Time O(m · n) · Space O(m + n)

**Alternative:** Karatsuba multiplication runs in O(n^1.585), which only pays off for very large inputs.

---

### 2013. Detect Squares
**Difficulty:** Medium · [LeetCode](https://leetcode.com/problems/detect-squares/)

**Problem:** Design `DetectSquares`. `Add(point)` adds a point; duplicates are allowed. `Count(point)` returns how many ways you can pick three stored points that form an axis-aligned square of positive area together with the query point.

**Key insight:** Treat each stored point that is diagonal to the query (`|dx| == |dy| ≠ 0`) as the opposite corner. Multiply the counts of the other two corners.

#### Approach: Count Map + Iterate Diagonal Candidates (Optimal)
```go
type DetectSquares struct {
	cnt map[[2]int]int
	pts [][2]int // distinct points
}

func Constructor() DetectSquares {
	return DetectSquares{cnt: map[[2]int]int{}}
}

func (d *DetectSquares) Add(point []int) {
	p := [2]int{point[0], point[1]}
	if d.cnt[p] == 0 {
		d.pts = append(d.pts, p)
	}
	d.cnt[p]++
}

func (d *DetectSquares) Count(point []int) int {
	qx, qy := point[0], point[1]
	res := 0
	for _, p := range d.pts {
		x, y := p[0], p[1]
		dx, dy := x-qx, y-qy
		if dx == 0 || (dx != dy && dx != -dy) {
			continue // not a diagonal corner
		}
		res += d.cnt[p] * d.cnt[[2]int{qx, y}] * d.cnt[[2]int{x, qy}]
	}
	return res
}
```
**Complexity:** `Add` O(1) · `Count` O(P) for P distinct points · Space O(P)

**Alternative:** Store points by x-coordinate (`map[int]map[int]int`), then for each y on the query's vertical line, try the two side lengths. `Count` becomes O(points sharing the query's x).

---
## 18. Bit Manipulation

### 136. Single Number
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/single-number/)

**Problem:** Every element appears twice except one. Find that one in O(n) time and O(1) extra space.

**Example:** `[4,1,2,1,2]` → `4`

**Key insight:** `a ^ a = 0` and `a ^ 0 = a`, so XOR-ing everything cancels the pairs.

#### Approach 1: XOR (Optimal)
```go
func singleNumber(nums []int) int {
	res := 0
	for _, n := range nums {
		res ^= n
	}
	return res
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: Hash Set Toggle
```go
func singleNumberSet(nums []int) int {
	set := map[int]bool{}
	for _, n := range nums {
		if set[n] {
			delete(set, n)
		} else {
			set[n] = true
		}
	}
	for k := range set {
		return k
	}
	return 0
}
```
**Complexity:** Time O(n) · Space O(n)

---

### 191. Number of 1 Bits
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/number-of-1-bits/)

**Problem:** Return the number of set bits (the Hamming weight) in `n`.

**Example:** `11` (binary `1011`) → `3`

#### Approach 1: Check Every Bit
```go
func hammingWeightLoop(n int) int {
	count := 0
	for n != 0 {
		count += n & 1
		n >>= 1
	}
	return count
}
```
**Complexity:** Time O(32) · Space O(1)

#### Approach 2: Brian Kernighan's Trick (Optimal)
`n & (n-1)` clears the lowest set bit, so the loop runs once per set bit.
```go
func hammingWeight(n int) int {
	count := 0
	for n != 0 {
		n &= n - 1
		count++
	}
	return count
}
```
**Complexity:** Time O(number of set bits) · Space O(1)

#### Approach 3: Standard Library (hardware popcount)
```go
func hammingWeightStd(n int) int {
	return bits.OnesCount(uint(n))
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
```go
func countBits(n int) []int {
	res := make([]int, n+1)
	for i := 1; i <= n; i++ {
		res[i] = res[i>>1] + i&1
	}
	return res
}
```
**Complexity:** Time O(n) · Space O(1) extra

#### Approach 2: DP on the Lowest Set Bit
```go
func countBitsKernighan(n int) []int {
	res := make([]int, n+1)
	for i := 1; i <= n; i++ {
		res[i] = res[i&(i-1)] + 1
	}
	return res
}
```
**Complexity:** Time O(n) · Space O(1) extra

#### Approach 3: Popcount Each Number
```go
func countBitsNaive(n int) []int {
	res := make([]int, n+1)
	for i := 0; i <= n; i++ {
		res[i] = bits.OnesCount(uint(i))
	}
	return res
}
```
**Complexity:** Time O(n), assuming popcount is O(1) · Space O(1) extra

---

### 190. Reverse Bits
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/reverse-bits/)

**Problem:** Reverse the bits of a 32-bit unsigned integer.

**Example:** `00000010100101000001111010011100` → `00111001011110000010100101000000`

> LeetCode's Go template has used both `func reverseBits(num uint32) uint32` and `func reverseBits(n int) int`. The `uint32` form is shown here. Adjust the signature to match your editor.

#### Approach 1: Bit-by-Bit Shift
```go
func reverseBitsLoop(num uint32) uint32 {
	var res uint32
	for i := 0; i < 32; i++ {
		res = res<<1 | num&1
		num >>= 1
	}
	return res
}
```
**Complexity:** Time O(32) · Space O(1)

#### Approach 2: Divide and Conquer Swaps (Optimal)
```go
func reverseBits(num uint32) uint32 {
	num = num>>16 | num<<16
	num = (num&0xff00ff00)>>8 | (num&0x00ff00ff)<<8
	num = (num&0xf0f0f0f0)>>4 | (num&0x0f0f0f0f)<<4
	num = (num&0xcccccccc)>>2 | (num&0x33333333)<<2
	num = (num&0xaaaaaaaa)>>1 | (num&0x55555555)<<1
	return num
}
```
**Complexity:** Time O(1) (5 steps) · Space O(1)

#### Approach 3: Standard Library
```go
func reverseBitsStd(num uint32) uint32 {
	return bits.Reverse32(num)
}
```
**Complexity:** Time O(1) · Space O(1)

---

### 268. Missing Number
**Difficulty:** Easy · [LeetCode](https://leetcode.com/problems/missing-number/)

**Problem:** `nums` contains `n` distinct numbers from the range `[0, n]`. Return the one that's missing.

**Example:** `[3,0,1]` → `2`

#### Approach 1: Gauss Sum
```go
func missingNumberSum(nums []int) int {
	n := len(nums)
	expected := n * (n + 1) / 2
	for _, v := range nums {
		expected -= v
	}
	return expected
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 2: XOR of Indices and Values (Optimal, can't overflow)
```go
func missingNumber(nums []int) int {
	res := len(nums)
	for i, v := range nums {
		res ^= i ^ v
	}
	return res
}
```
**Complexity:** Time O(n) · Space O(1)

#### Approach 3: Sort + Binary Search
```go
func missingNumberBS(nums []int) int {
	sort.Ints(nums)
	lo, hi := 0, len(nums)
	for lo < hi {
		mid := (lo + hi) / 2
		if nums[mid] > mid {
			hi = mid
		} else {
			lo = mid + 1
		}
	}
	return lo
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
```go
func getSum(a int, b int) int {
	x, y := int32(a), int32(b)
	for y != 0 {
		carry := (x & y) << 1
		x ^= y
		y = carry
	}
	return int(x)
}
```
**Complexity:** Time O(32) · Space O(1)

> Go's `int` is 64-bit. With plain `int` the loop still ends, because the carry eventually shifts off the top bit, but casting to `int32` matches the problem's 32-bit model exactly.

#### Approach 2: Recursive
```go
func getSumRec(a int, b int) int {
	var f func(x, y int32) int32
	f = func(x, y int32) int32 {
		if y == 0 {
			return x
		}
		return f(x^y, (x&y)<<1)
	}
	return int(f(int32(a), int32(b)))
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
```go
func reverse(x int) int {
	res := int32(0)
	v := int32(x)
	for v != 0 {
		d := v % 10 // Go's % keeps the sign of v
		v /= 10
		if res > math.MaxInt32/10 || (res == math.MaxInt32/10 && d > 7) {
			return 0
		}
		if res < math.MinInt32/10 || (res == math.MinInt32/10 && d < -8) {
			return 0
		}
		res = res*10 + d
	}
	return int(res)
}
```
**Complexity:** Time O(log₁₀ x) · Space O(1)

#### Approach 2: 64-bit Accumulator (simpler, if allowed)
```go
func reverse64(x int) int {
	res := 0
	for x != 0 {
		res = res*10 + x%10
		x /= 10
	}
	if res > math.MaxInt32 || res < math.MinInt32 {
		return 0
	}
	return res
}
```
**Complexity:** Time O(log₁₀ x) · Space O(1)

---

## Complexity Cheat Sheet

| Pattern | Typical time | Typical space | Signals in the problem |
|---|---|---|---|
| Hash map / set | O(n) | O(n) | "seen before", duplicates, complements |
| Two pointers | O(n) | O(1) | sorted input, pairs, palindromes |
| Sliding window | O(n) | O(k) | contiguous substring or subarray with a constraint |
| Monotonic stack | O(n) | O(n) | next greater or smaller element, histograms |
| Binary search | O(log n) | O(1) | sorted input, or "minimum k such that…" |
| Heap | O(n log k) | O(k) | top-k, k-way merge, streaming median |
| DFS / BFS | O(V + E) | O(V) | grids, connectivity, shortest path in an unweighted graph |
| Topological sort | O(V + E) | O(V + E) | dependencies, ordering, cycle detection |
| Union-find | O(α(n)) per operation | O(n) | dynamic connectivity, redundant edges |
| Dijkstra | O(E log V) | O(V + E) | weighted shortest path with non-negative weights |
| Backtracking | O(2ⁿ) / O(n!) | O(n) | "all combinations / permutations / partitions" |
| 1-D DP | O(n) to O(n²) | O(1) to O(n) | best value over a prefix |
| 2-D DP | O(m · n) | O(n) when rolled | two strings, grids, intervals |
| Greedy | O(n) or O(n log n) | O(1) | a locally best choice provably works |
| Bit tricks | O(1) to O(n) | O(1) | XOR pairing, powers of two, bit counts |

---

*End of document: 150 problems, 18 categories.*
