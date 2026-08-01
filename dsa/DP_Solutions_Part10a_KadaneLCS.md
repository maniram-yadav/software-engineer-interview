# DP Solutions — Part 10a: Classic DPs (Kadane's + LCS) (Java)
### 19 Problems · Full Problem Statement + Example + Brute Force → Optimized + Complexity

---

# Section A: Kadane's Algorithm

## A1. Maximum Subarray

**Problem:** Find the contiguous subarray with the largest sum, return that sum.

**Example:**
```
Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
Output: 6
Explanation: The subarray [4,-1,2,1] has the largest sum = 6.
```

**Brute force:** check every subarray's sum → O(n²) (or O(n³) with naive re-summation).
**Optimized:** Kadane's algorithm — `curr = max(nums[i], curr + nums[i])`.
```java
class MaximumSubarray {
    public int maxSubArray(int[] nums) {
        int curr = nums[0], best = nums[0];
        for (int i = 1; i < nums.length; i++) {
            curr = Math.max(nums[i], curr + nums[i]);
            best = Math.max(best, curr);
        }
        return best;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## A2. Maximum Product Subarray

**Problem:** Find the contiguous subarray with the largest PRODUCT, return that product.

**Example:**
```
Input: nums = [2,3,-2,4]
Output: 6
Explanation: [2,3] has product 6, the maximum.
```

**Brute force:** check every subarray's product → O(n²).
**Optimized:** Kadane's variant tracking BOTH running max and running min (a negative number can flip a min into the new max).
```java
class MaximumProductSubarray {
    public int maxProduct(int[] nums) {
        int currMax = nums[0], currMin = nums[0], best = nums[0];
        for (int i = 1; i < nums.length; i++) {
            int num = nums[i];
            if (num < 0) { int tmp = currMax; currMax = currMin; currMin = tmp; }
            currMax = Math.max(num, currMax * num);
            currMin = Math.min(num, currMin * num);
            best = Math.max(best, currMax);
        }
        return best;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## A3. Bitwise ORs of Subarrays

**Problem:** Given an array, count the number of DISTINCT values achievable as the bitwise OR of some contiguous subarray.

**Example:**
```
Input: arr = [1,1,2]
Output: 3
Explanation: Possible subarray ORs: [1]=1, [1,1]=1, [1,1,2]=3, [1]=1, [1,2]=3, [2]=2.
Distinct values: {1, 2, 3} — 3 total.
```

**Brute force:** compute OR for every subarray directly → O(n²).
**Optimized:** maintain the SET of all possible OR-values ending at the current index — crucially, this set has at most ~30 elements (bounded by bit-width) since OR values only grow monotonically as more elements are included.
```java
class BitwiseORsOfSubarrays {
    public int subarrayBitwiseORs(int[] arr) {
        Set<Integer> result = new HashSet<>();
        Set<Integer> curr = new HashSet<>();
        for (int num : arr) {
            Set<Integer> next = new HashSet<>();
            next.add(num);
            for (int val : curr) next.add(val | num);
            curr = next;
            result.addAll(curr);
        }
        return result.size();
    }
}
```
**Complexity:** O(n · 30) time (30 = bit-width bound on distinct OR values per position), O(30) space per step — beats the O(n²) brute force.

---

## A4. Longest Turbulent Subarray

**Problem:** A turbulent subarray has comparison signs that strictly alternate (`>,<,>,<...` or `<,>,<,>...`) between consecutive elements. Return the length of the longest turbulent subarray.

**Example:**
```
Input: arr = [9,4,2,10,7,8,8,1,9]
Output: 5
Explanation: [4,2,10,7,8] alternates <,>,<,> — a turbulent run of length 5.
```

**Brute force:** check every subarray for the alternating property → O(n²).
**Optimized:** track `up` and `down` streak lengths ending at the current index (same pattern as Wiggle Subsequence).
```java
class LongestTurbulentSubarray {
    public int maxTurbulenceSize(int[] arr) {
        int n = arr.length;
        int up = 1, down = 1, best = 1;
        for (int i = 1; i < n; i++) {
            if (arr[i] > arr[i - 1]) { up = down + 1; down = 1; }
            else if (arr[i] < arr[i - 1]) { down = up + 1; up = 1; }
            else { up = 1; down = 1; }
            best = Math.max(best, Math.max(up, down));
        }
        return best;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## A5. Maximum Subarray Sum With One Deletion

**Problem:** Find the maximum sum of a contiguous subarray, allowing at most ONE element to be deleted from within it (the subarray must remain non-empty after deletion).

**Example:**
```
Input: arr = [1,-2,0,3]
Output: 4
Explanation: Delete -2: remaining elements [1,0,3] concatenated conceptually 
give sum 1+0+3 = 4.
```

**Brute force:** try deleting every possible single element, run Kadane's on the result → O(n²).
**Optimized:** track two states per position — `noDelete[i] = max subarray sum ending at i with no deletion used`, `oneDelete[i] = max subarray sum ending at i with exactly one deletion used`.
```java
class MaxSubarraySumOneDeletion {
    public int maximumSum(int[] arr) {
        int n = arr.length;
        int noDelete = arr[0], oneDelete = 0, best = arr[0];
        for (int i = 1; i < n; i++) {
            oneDelete = Math.max(oneDelete + arr[i], noDelete); // delete arr[i] (use prior no-delete state) or extend one-delete
            noDelete = Math.max(arr[i], noDelete + arr[i]);
            best = Math.max(best, Math.max(noDelete, oneDelete));
        }
        return best;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## A6. K Concatenation Maximum Sum

**Problem:** Given an array and integer `k`, form a new array by concatenating the original `k` times. Return the maximum subarray sum of this new array, modulo 10⁹+7.

**Example:**
```
Input: arr = [1,2], k = 3
Output: 9
Explanation: The array repeated 3 times is [1,2,1,2,1,2], whose maximum 
subarray sum (the whole thing, since all positive) is 1+2+1+2+1+2 = 9.
```

**Brute force:** actually construct the k-times-concatenated array and run Kadane's on it → O(n·k), infeasible for large k.
**Optimized insight:** run Kadane's on ONE copy (captures pure single-copy max) and on TWO copies concatenated (captures any wraparound crossing a boundary); if the total array sum is positive, add `(k-2) × totalSum` to the two-copy result (all the "middle" full copies contribute fully).
```java
class KConcatenationMaximumSum {
    public int kConcatenationMaxSum(int[] arr, int k) {
        long MOD = 1_000_000_007;
        long singleMax = kadane(arr);
        if (k == 1) return (int) (singleMax % MOD);

        int n = arr.length;
        int[] doubled = new int[2 * n];
        for (int i = 0; i < n; i++) { doubled[i] = arr[i]; doubled[i + n] = arr[i]; }
        long doubleMax = kadane(doubled);

        long totalSum = 0;
        for (int num : arr) totalSum += num;

        long result = doubleMax;
        if (totalSum > 0) {
            result = Math.max(result, doubleMax + (long)(k - 2) * totalSum);
        }
        return (int) (Math.max(result, 0) % MOD);
    }

    private long kadane(int[] arr) {
        long curr = 0, best = 0;
        for (int num : arr) {
            curr = Math.max(num, curr + num);
            best = Math.max(best, curr);
        }
        return best;
    }
}
```
**Complexity:** O(n) time (Kadane's on ≤ 2n elements), O(n) space — beats the O(n·k) brute-force materialization.

---

## A7. Largest Divisible Subset

**Problem:** Given a set of distinct positive integers, find the largest subset such that every pair `(Si, Sj)` in the subset satisfies `Si % Sj == 0` or `Sj % Si == 0`.

**Example:**
```
Input: nums = [1,2,4,8]
Output: [1,2,4,8]
Explanation: Every pair satisfies the divisibility condition (each divides the next).
```

**Brute force:** try every subset, check pairwise divisibility → O(2ⁿ · n²).
**Optimized:** sort ascending; `dp[i] = length of the largest divisible subset ENDING at index i`, checking only `nums[i] % nums[j] == 0` for `j < i` (sufficient because sorted order guarantees `nums[j]` also divides everything smaller in the chain).
```java
class LargestDivisibleSubset {
    public List<Integer> largestDivisibleSubset(int[] nums) {
        int n = nums.length;
        Arrays.sort(nums);
        int[] dp = new int[n];
        int[] prev = new int[n];
        Arrays.fill(dp, 1);
        Arrays.fill(prev, -1);
        int maxIdx = 0;

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < i; j++) {
                if (nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i]) {
                    dp[i] = dp[j] + 1;
                    prev[i] = j;
                }
            }
            if (dp[i] > dp[maxIdx]) maxIdx = i;
        }

        List<Integer> result = new ArrayList<>();
        for (int i = maxIdx; i != -1; i = prev[i]) result.add(nums[i]);
        Collections.reverse(result);
        return result;
    }
}
```
**Complexity:** O(n²) time (plus O(n log n) sort), O(n) space — beats the O(2ⁿ) brute force.

---

## A8. Length of Longest Fibonacci Subsequence

**Problem:** Given a strictly increasing array, find the length of the longest subsequence that forms a Fibonacci-like sequence (each term is the sum of the two preceding terms, length ≥ 3). Return 0 if none exists.

**Example:**
```
Input: arr = [1,2,3,4,5,6,7,8]
Output: 5
Explanation: The subsequence [1,2,3,5,8] is Fibonacci-like: 1+2=3, 2+3=5, 3+5=8.
```

**Brute force:** try every triple as a starting Fibonacci seed, extend greedily checking membership → O(n² log n) with binary search, or worse without it.
**Optimized:** `dp[(a,b)] = length of the longest Fibonacci-like sequence ENDING with the pair (a,b)`, built via a value-to-index hashmap so lookups are O(1); iterate pairs (j,i) with j<i, check if `arr[i]-arr[j]` exists and is smaller than `arr[j]` (ensuring it comes earlier).
```java
class LongestFibonacciSubsequence {
    public int lenLongestFibSubseq(int[] arr) {
        int n = arr.length;
        Map<Integer, Integer> indexOf = new HashMap<>();
        for (int i = 0; i < n; i++) indexOf.put(arr[i], i);

        Map<Long, Integer> dp = new HashMap<>(); // key: (j * 100000L + i), value: chain length ending at (j,i)
        int best = 0;

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < i; j++) {
                int diff = arr[i] - arr[j];
                if (diff < arr[j] && indexOf.containsKey(diff)) {
                    int k = indexOf.get(diff); // k < j guaranteed since diff < arr[j] and array is increasing
                    long key = (long) k * 100000 + j;
                    int len = dp.getOrDefault(key, 2) + 1;
                    dp.put((long) j * 100000 + i, len);
                    best = Math.max(best, len);
                }
            }
        }
        return best >= 3 ? best : 0;
    }
}
```
**Complexity:** O(n²) time, O(n²) space (for the pair-indexed hashmap).

---

# Section B: Longest Common Subsequence (LCS) Family

## B1. Longest Palindromic Substring

**Problem:** Given a string, return the longest substring that is a palindrome.

**Example:**
```
Input: s = "babad"
Output: "bab" (or "aba" — both valid)
```

**Brute force:** check every substring for palindrome property → O(n³).
**Optimized:** expand-around-center for every possible center (odd and even length) → O(n²) time, O(1) space.
```java
class LongestPalindromicSubstring {
    public String longestPalindrome(String s) {
        if (s.isEmpty()) return "";
        int start = 0, maxLen = 1;
        for (int center = 0; center < s.length(); center++) {
            int len1 = expand(s, center, center);
            int len2 = expand(s, center, center + 1);
            int len = Math.max(len1, len2);
            if (len > maxLen) {
                maxLen = len;
                start = center - (len - 1) / 2;
            }
        }
        return s.substring(start, start + maxLen);
    }

    private int expand(String s, int left, int right) {
        while (left >= 0 && right < s.length() && s.charAt(left) == s.charAt(right)) {
            left--; right++;
        }
        return right - left - 1;
    }
}
```
**Complexity:** O(n²) time, O(1) space.

---

## B2. Longest Palindromic Subsequence

**Problem:** Given a string, find the length of the longest subsequence (not necessarily contiguous) that is a palindrome.

**Example:**
```
Input: s = "bbbab"
Output: 4
Explanation: One longest palindromic subsequence is "bbbb".
```

**Brute force:** try every subsequence, check palindrome property → O(2ⁿ).
**Optimized:** equivalent to `LCS(s, reverse(s))`, or direct interval DP `dp[i][j] = longest palindromic subsequence in s[i..j]`.
```java
class LongestPalindromicSubsequence {
    public int longestPalindromeSubseq(String s) {
        int n = s.length();
        int[][] dp = new int[n][n];
        for (int i = n - 1; i >= 0; i--) {
            dp[i][i] = 1;
            for (int j = i + 1; j < n; j++) {
                if (s.charAt(i) == s.charAt(j)) {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                } else {
                    dp[i][j] = Math.max(dp[i + 1][j], dp[i][j - 1]);
                }
            }
        }
        return dp[0][n - 1];
    }
}
```
**Complexity:** O(n²) time, O(n²) space.

---

## B3. Maximum Length of Repeated Subarray

**Problem:** Given two integer arrays, return the length of the longest common CONTIGUOUS subarray.

**Example:**
```
Input: nums1 = [1,2,3,2,1], nums2 = [3,2,1,4,7]
Output: 3
Explanation: [3,2,1] is the longest common contiguous subarray.
```

**Brute force:** check every pair of starting indices, extend while matching → O(n²·m).
**Optimized:** `dp[i][j] = length of the common suffix ending exactly at nums1[i-1] and nums2[j-1]` (resets to 0 on mismatch, unlike LCS which carries forward).
```java
class MaxLengthRepeatedSubarray {
    public int findLength(int[] nums1, int[] nums2) {
        int m = nums1.length, n = nums2.length;
        int[][] dp = new int[m + 1][n + 1];
        int best = 0;
        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                if (nums1[i - 1] == nums2[j - 1]) {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    best = Math.max(best, dp[i][j]);
                }
            }
        }
        return best;
    }
}
```
**Complexity:** O(m·n) time, O(m·n) space (reducible to O(n) with rolling array).

---

## B4. Longest Common Subsequence

**Problem:** Given two strings, return the length of their longest common subsequence (not necessarily contiguous).

**Example:**
```
Input: text1 = "abcde", text2 = "ace"
Output: 3
Explanation: "ace" is a common subsequence of length 3.
```

**Brute force:** try every subsequence of one string, check if it's a subsequence of the other → O(2ⁿ · m).
**Optimized:** classic `dp[i][j] = LCS(text1[0..i), text2[0..j))`.
```java
class LongestCommonSubsequence {
    public int longestCommonSubsequence(String text1, String text2) {
        int m = text1.length(), n = text2.length();
        int[][] dp = new int[m + 1][n + 1];
        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                if (text1.charAt(i - 1) == text2.charAt(j - 1)) {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }
        return dp[m][n];
    }
}
```
**Complexity:** O(m·n) time, O(m·n) space (reducible to O(min(m,n)) with rolling array).

---

## B5. Regular Expression Matching

**Problem:** Implement regex matching supporting `.` (matches any single character) and `*` (matches zero or more of the preceding element), matching the ENTIRE input string.

**Example:**
```
Input: s = "aa", p = "a*"
Output: true
Explanation: "a*" means zero or more of 'a', matching "aa".
```

**Brute force:** recursive backtracking without memo → exponential (repeated subproblems on `*` branching).
**Optimized:** `dp[i][j] = does s[0..i) match p[0..j)`, with special handling for `*` (either match zero occurrences, skipping the pair, or match one occurrence and stay at the same pattern position).
```java
class RegularExpressionMatching {
    public boolean isMatch(String s, String p) {
        int m = s.length(), n = p.length();
        boolean[][] dp = new boolean[m + 1][n + 1];
        dp[0][0] = true;

        for (int j = 1; j <= n; j++) {
            if (p.charAt(j - 1) == '*') dp[0][j] = dp[0][j - 2];
        }

        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                char pc = p.charAt(j - 1);
                if (pc == '*') {
                    dp[i][j] = dp[i][j - 2]; // zero occurrences
                    char prevPc = p.charAt(j - 2);
                    if (prevPc == '.' || prevPc == s.charAt(i - 1)) {
                        dp[i][j] = dp[i][j] || dp[i - 1][j]; // one more occurrence
                    }
                } else if (pc == '.' || pc == s.charAt(i - 1)) {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }
        return dp[m][n];
    }
}
```
**Complexity:** O(m·n) time, O(m·n) space.

---

## B6. Wildcard Matching

**Problem:** Implement wildcard matching supporting `?` (matches any single character) and `*` (matches any sequence of characters, including empty), matching the ENTIRE input string.

**Example:**
```
Input: s = "adceb", p = "*a*b"
Output: true
Explanation: "*" matches "" before 'a', "*" matches "dce" between 'a' and 'b'.
```

**Brute force:** recursive backtracking without memo → exponential.
**Optimized:** `dp[i][j] = does s[0..i) match p[0..j)`, where `*` either matches zero characters (`dp[i][j-1]`) or consumes one more character of s while staying on the same `*` (`dp[i-1][j]`).
```java
class WildcardMatching {
    public boolean isMatch(String s, String p) {
        int m = s.length(), n = p.length();
        boolean[][] dp = new boolean[m + 1][n + 1];
        dp[0][0] = true;

        for (int j = 1; j <= n; j++) {
            if (p.charAt(j - 1) == '*') dp[0][j] = dp[0][j - 1];
        }

        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                char pc = p.charAt(j - 1);
                if (pc == '*') {
                    dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
                } else if (pc == '?' || pc == s.charAt(i - 1)) {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }
        return dp[m][n];
    }
}
```
**Complexity:** O(m·n) time, O(m·n) space (reducible to O(n) with rolling array).

---

## B7. Edit Distance

**Problem:** Given two words, find the minimum number of operations (insert, delete, replace) to convert word1 into word2.

**Example:**
```
Input: word1 = "horse", word2 = "ros"
Output: 3
Explanation: horse -> rorse (replace h with r) -> rose (delete r) -> ros (delete e).
```

**Brute force:** recursive try-every-operation without memo → exponential.
**Optimized:** classic `dp[i][j] = edit distance between word1[0..i) and word2[0..j)`.
```java
class EditDistance {
    public int minDistance(String word1, String word2) {
        int m = word1.length(), n = word2.length();
        int[][] dp = new int[m + 1][n + 1];
        for (int i = 0; i <= m; i++) dp[i][0] = i;
        for (int j = 0; j <= n; j++) dp[0][j] = j;

        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                if (word1.charAt(i - 1) == word2.charAt(j - 1)) {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + Math.min(dp[i - 1][j - 1], Math.min(dp[i - 1][j], dp[i][j - 1]));
                }
            }
        }
        return dp[m][n];
    }
}
```
**Complexity:** O(m·n) time, O(m·n) space (reducible to O(n) with rolling array).

---

## B8. Interleaving String

**Problem:** Given three strings `s1, s2, s3`, determine if `s3` is formed by interleaving `s1` and `s2` (preserving the relative order of characters within each).

**Example:**
```
Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
Output: true
```

**Brute force:** recursive try-both-branches without memo → exponential.
**Optimized:** `dp[i][j] = can s3[0..i+j) be formed by interleaving s1[0..i) and s2[0..j)`.
```java
class InterleavingString {
    public boolean isInterleave(String s1, String s2, String s3) {
        int m = s1.length(), n = s2.length();
        if (m + n != s3.length()) return false;

        boolean[][] dp = new boolean[m + 1][n + 1];
        dp[0][0] = true;

        for (int i = 1; i <= m; i++) dp[i][0] = dp[i - 1][0] && s1.charAt(i - 1) == s3.charAt(i - 1);
        for (int j = 1; j <= n; j++) dp[0][j] = dp[0][j - 1] && s2.charAt(j - 1) == s3.charAt(j - 1);

        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                char target = s3.charAt(i + j - 1);
                dp[i][j] = (dp[i - 1][j] && s1.charAt(i - 1) == target)
                    || (dp[i][j - 1] && s2.charAt(j - 1) == target);
            }
        }
        return dp[m][n];
    }
}
```
**Complexity:** O(m·n) time, O(m·n) space (reducible to O(n) with rolling array).

---

## B9. Shortest Common Supersequence

**Problem:** Given two strings, return the SHORTEST string that has both as subsequences.

**Example:**
```
Input: str1 = "abac", str2 = "cab"
Output: "cabac"
Explanation: "cabac" contains "abac" as a subsequence (positions 1,2,3,4) and 
"cab" as a subsequence (positions 0,1,2) — length 5 is minimal.
```

**Brute force:** try every possible interleaving/merge, check both subsequence conditions → exponential.
**Optimized:** find the LCS DP table first, then reconstruct the answer by walking the table backward, taking characters from LCS once and non-LCS characters from both strings as needed.
```java
class ShortestCommonSupersequence {
    public String shortestCommonSupersequence(String str1, String str2) {
        int m = str1.length(), n = str2.length();
        int[][] dp = new int[m + 1][n + 1];
        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                if (str1.charAt(i - 1) == str2.charAt(j - 1)) dp[i][j] = dp[i - 1][j - 1] + 1;
                else dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1]);
            }
        }

        StringBuilder sb = new StringBuilder();
        int i = m, j = n;
        while (i > 0 && j > 0) {
            if (str1.charAt(i - 1) == str2.charAt(j - 1)) {
                sb.append(str1.charAt(i - 1));
                i--; j--;
            } else if (dp[i - 1][j] >= dp[i][j - 1]) {
                sb.append(str1.charAt(i - 1));
                i--;
            } else {
                sb.append(str2.charAt(j - 1));
                j--;
            }
        }
        while (i > 0) { sb.append(str1.charAt(i - 1)); i--; }
        while (j > 0) { sb.append(str2.charAt(j - 1)); j--; }
        return sb.reverse().toString();
    }
}
```
**Complexity:** O(m·n) time, O(m·n) space.

---

## B10. Minimum Insertion Steps to Make a String Palindrome

**Problem:** Given a string, return the minimum number of insertions needed to make it a palindrome.

**Example:**
```
Input: s = "mbadm"
Output: 2
Explanation: Insert 2 characters (e.g., "mbdadbm") to form a palindrome.
```

**Brute force:** try every possible insertion sequence → exponential.
**Optimized insight:** `answer = n - LPS(s)` where LPS = length of longest palindromic subsequence (the characters NOT in the LPS are exactly the ones needing a mirrored insertion), computed as `LCS(s, reverse(s))`.
```java
class MinInsertionsPalindrome {
    public int minInsertions(String s) {
        int n = s.length();
        String rev = new StringBuilder(s).reverse().toString();
        int[][] dp = new int[n + 1][n + 1];
        for (int i = 1; i <= n; i++) {
            for (int j = 1; j <= n; j++) {
                if (s.charAt(i - 1) == rev.charAt(j - 1)) dp[i][j] = dp[i - 1][j - 1] + 1;
                else dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
        return n - dp[n][n];
    }
}
```
**Complexity:** O(n²) time, O(n²) space (reducible to O(n) with rolling array).

---

## B11. Max Dot Product of Two Subsequences

**Problem:** Given two arrays, choose non-empty subsequences of EQUAL length from each (preserving relative order), maximizing their dot product.

**Example:**
```
Input: nums1 = [2,1,-2,5], nums2 = [3,0,-6]
Output: 18
Explanation: Choosing subsequence [2,-2] from nums1 and [3,-6] from nums2: 
dot product = 2*3 + (-2)*(-6) = 6+12 = 18.
```

**Brute force:** try every pair of subsequences of matching length → exponential.
**Optimized:** `dp[i][j] = max dot product using nums1[0..i) and nums2[0..j)`, with three options: skip nums1[i-1], skip nums2[j-1], or pair them (starting fresh or extending a positive previous dot product).
```java
class MaxDotProductTwoSubsequences {
    public int maxDotProduct(int[] nums1, int[] nums2) {
        int m = nums1.length, n = nums2.length;
        int[][] dp = new int[m + 1][n + 1];
        for (int[] row : dp) Arrays.fill(row, Integer.MIN_VALUE / 2);

        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                int product = nums1[i - 1] * nums2[j - 1];
                dp[i][j] = Math.max(product, product + Math.max(0, dp[i - 1][j - 1]));
                dp[i][j] = Math.max(dp[i][j], dp[i - 1][j]);
                dp[i][j] = Math.max(dp[i][j], dp[i][j - 1]);
            }
        }
        return dp[m][n];
    }
}
```
**Complexity:** O(m·n) time, O(m·n) space.

---

## 🎯 Part 10a Summary Table

| # | Problem | Time | Space |
|---|---|---|---|
| A1 | Maximum Subarray | O(n) | O(1) |
| A2 | Maximum Product Subarray | O(n) | O(1) |
| A3 | Bitwise ORs of Subarrays | O(30n) | O(30) |
| A4 | Longest Turbulent Subarray | O(n) | O(1) |
| A5 | Max Subarray Sum One Deletion | O(n) | O(1) |
| A6 | K Concatenation Maximum Sum | O(n) | O(n) |
| A7 | Largest Divisible Subset | O(n²) | O(n) |
| A8 | Longest Fibonacci Subsequence | O(n²) | O(n²) |
| B1 | Longest Palindromic Substring | O(n²) | O(1) |
| B2 | Longest Palindromic Subsequence | O(n²) | O(n²) |
| B3 | Max Length Repeated Subarray | O(m·n) | O(m·n) |
| B4 | Longest Common Subsequence | O(m·n) | O(m·n) |
| B5 | Regular Expression Matching | O(m·n) | O(m·n) |
| B6 | Wildcard Matching | O(m·n) | O(m·n) |
| B7 | Edit Distance | O(m·n) | O(m·n) |
| B8 | Interleaving String | O(m·n) | O(m·n) |
| B9 | Shortest Common Supersequence | O(m·n) | O(m·n) |
| B10 | Min Insertions Palindrome | O(n²) | O(n²) |
| B11 | Max Dot Product Two Subsequences | O(m·n) | O(m·n) |

---

**Next: Part 10b — LIS + 2D Grid Traversal (~15 problems).** Say "continue" to proceed.
