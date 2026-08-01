# DP Solutions — Part 8: String DP (Java)
### 20 Problems · Full Problem Statement + Example + Brute Force → Optimized + Complexity

---

## 1. Is Subsequence

**Problem:** Given two strings `s` and `t`, determine if `s` is a subsequence of `t` (characters of `s` appear in `t` in the same relative order, not necessarily contiguous).

**Example:**
```
Input: s = "abc", t = "ahbgdc"
Output: true
Explanation: a...b...c appear in order within t.
```

**Brute force:** try every way to select characters of `t` matching `s` recursively → exponential without the greedy insight.
**Optimized:** two-pointer greedy scan — always advance the `s`-pointer on a match, `t`-pointer every step.
```java
class IsSubsequence {
    public boolean isSubsequence(String s, String t) {
        int i = 0, j = 0;
        while (i < s.length() && j < t.length()) {
            if (s.charAt(i) == t.charAt(j)) i++;
            j++;
        }
        return i == s.length();
    }
}
```
**Complexity:** O(n) time (n = t.length()), O(1) space.

---

## 2. Palindrome Partitioning

**Problem:** Given a string `s`, return ALL possible ways to partition it such that every substring in the partition is a palindrome.

**Example:**
```
Input: s = "aab"
Output: [["a","a","b"],["aa","b"]]
```

**Brute force:** try every cut point combination, checking palindrome validity from scratch each time → O(2ⁿ · n) (exponential cuts × O(n) palindrome check each).
**Optimized:** precompute a palindrome DP table in O(n²), then backtrack using O(1) lookups instead of re-checking.
```java
class PalindromePartitioning {
    public List<List<String>> partition(String s) {
        int n = s.length();
        boolean[][] isPal = new boolean[n][n];
        for (int len = 1; len <= n; len++) {
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                if (s.charAt(i) == s.charAt(j) && (len <= 2 || isPal[i + 1][j - 1])) isPal[i][j] = true;
            }
        }
        List<List<String>> result = new ArrayList<>();
        backtrack(s, 0, new ArrayList<>(), isPal, result);
        return result;
    }

    private void backtrack(String s, int start, List<String> curr, boolean[][] isPal, List<List<String>> result) {
        if (start == s.length()) { result.add(new ArrayList<>(curr)); return; }
        for (int end = start; end < s.length(); end++) {
            if (isPal[start][end]) {
                curr.add(s.substring(start, end + 1));
                backtrack(s, end + 1, curr, isPal, result);
                curr.remove(curr.size() - 1);
            }
        }
    }
}
```
**Complexity:** O(n² ) precompute + O(2ⁿ) worst-case output enumeration (inherent to the problem — output can be exponential), O(n²) space for the table.

---

## 3. Palindrome Partitioning II

**Problem:** Given a string `s`, return the MINIMUM number of cuts needed to partition it so every substring is a palindrome.

**Example:**
```
Input: s = "aab"
Output: 1
Explanation: One cut: "aa" | "b" — both palindromes, achieved with just 1 cut.
```

**Brute force:** try every cut combination recursively, no memo → exponential.
**Optimized:** `dp[i] = min cuts for s[0..i]`, using a precomputed palindrome table.
```java
class PalindromePartitioningII {
    public int minCut(String s) {
        int n = s.length();
        boolean[][] isPal = new boolean[n][n];
        for (int len = 1; len <= n; len++) {
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                if (s.charAt(i) == s.charAt(j) && (len <= 2 || isPal[i + 1][j - 1])) isPal[i][j] = true;
            }
        }

        int[] dp = new int[n];
        for (int i = 0; i < n; i++) {
            if (isPal[0][i]) { dp[i] = 0; continue; }
            dp[i] = Integer.MAX_VALUE;
            for (int j = 1; j <= i; j++) {
                if (isPal[j][i]) dp[i] = Math.min(dp[i], dp[j - 1] + 1);
            }
        }
        return dp[n - 1];
    }
}
```
**Complexity:** O(n²) time, O(n²) space.

---

## 4. Word Break

**Problem:** Given a string `s` and a dictionary of words, determine if `s` can be segmented into a space-separated sequence of one or more dictionary words.

**Example:**
```
Input: s = "leetcode", wordDict = ["leet","code"]
Output: true
Explanation: "leetcode" = "leet" + "code".
```

**Brute force:** try every split point recursively without memo → exponential (overlapping subproblems).
**Optimized:** `dp[i] = true if s[0..i) is breakable`, checking every valid last-word ending at i.
```java
class WordBreak {
    public boolean wordBreak(String s, List<String> wordDict) {
        Set<String> dict = new HashSet<>(wordDict);
        int n = s.length();
        boolean[] dp = new boolean[n + 1];
        dp[0] = true;
        for (int i = 1; i <= n; i++) {
            for (int j = 0; j < i; j++) {
                if (dp[j] && dict.contains(s.substring(j, i))) { dp[i] = true; break; }
            }
        }
        return dp[n];
    }
}
```
**Complexity:** O(n²) time (with O(1) average substring hashing; O(n³) if substring costs counted), O(n) space.

---

## 5. Unique Substrings in Wraparound String

**Problem:** Consider the infinite string formed by wrapping "abcdefghijklmnopqrstuvwxyz" around forever. Given a string `p`, count how many unique non-empty substrings of `p` appear as a substring of this infinite wraparound string.

**Example:**
```
Input: p = "zab"
Output: 6
Explanation: Unique substrings appearing in the wraparound string: "z","a","b","za","ab","zab" — 6 total.
```

**Brute force:** generate every substring of p, check membership in the infinite wraparound conceptually → O(n²) substrings × O(n) validity check = O(n³).
**Optimized:** `dp[c] = length of the longest wraparound-consistent run ending in character c` — since any run of length L ending in c contributes L unique substrings all ending in c (and we only need the max per character to avoid double-counting).
```java
class UniqueSubstringsWraparound {
    public int findSubstringInWraproundString(String s) {
        int[] dp = new int[26];
        int k = 0;
        for (int i = 0; i < s.length(); i++) {
            if (i > 0 && (s.charAt(i) - s.charAt(i - 1) == 1 || s.charAt(i - 1) - s.charAt(i) == 25)) k++;
            else k = 1;
            int idx = s.charAt(i) - 'a';
            dp[idx] = Math.max(dp[idx], k);
        }
        int total = 0;
        for (int v : dp) total += v;
        return total;
    }
}
```
**Complexity:** O(n) time, O(1) space (fixed 26-entry array) — beats the O(n³) brute force by a huge margin.

---

## 6. Minimum ASCII Delete Sum for Two Strings

**Problem:** Given two strings, find the minimum sum of ASCII values of deleted characters to make the two strings equal.

**Example:**
```
Input: s1 = "sea", s2 = "eat"
Output: 231
Explanation: Deleting 's' (115) from s1 and 't' (116) from s2 gives "ea" for both — 
total ASCII sum deleted = 115+116 = 231.
```

**Brute force:** try every combination of deletions from both strings → exponential.
**Optimized:** edit-distance-style DP where the "cost" is ASCII value instead of unit cost — `dp[i][j] = min delete-sum to equalize s1[0..i) and s2[0..j)`.
```java
class MinASCIIDeleteSum {
    public int minimumDeleteSum(String s1, String s2) {
        int m = s1.length(), n = s2.length();
        int[][] dp = new int[m + 1][n + 1];
        for (int i = 1; i <= m; i++) dp[i][0] = dp[i - 1][0] + s1.charAt(i - 1);
        for (int j = 1; j <= n; j++) dp[0][j] = dp[0][j - 1] + s2.charAt(j - 1);

        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                if (s1.charAt(i - 1) == s2.charAt(j - 1)) {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = Math.min(dp[i - 1][j] + s1.charAt(i - 1), dp[i][j - 1] + s2.charAt(j - 1));
                }
            }
        }
        return dp[m][n];
    }
}
```
**Complexity:** O(m·n) time, O(m·n) space.

---

## 7. Longest String Chain

**Problem:** Given a list of words, a word chain is a sequence where each word is formed by inserting exactly one letter into the previous word. Return the length of the longest possible word chain.

**Example:**
```
Input: words = ["a","b","ba","bca","bda","bdca"]
Output: 4
Explanation: One chain: "a" -> "ba" -> "bda" -> "bdca".
```

**Brute force:** try every pair of words to check predecessor relationship, build chains via DFS → exponential without memo.
**Optimized:** sort by length; `dp[word] = 1 + max(dp[predecessor])` over all single-character-removal predecessors of `word`.
```java
class LongestStringChain {
    public int longestStrChain(String[] words) {
        Arrays.sort(words, (a, b) -> a.length() - b.length());
        Map<String, Integer> dp = new HashMap<>();
        int best = 1;

        for (String w : words) {
            int maxLen = 1;
            for (int i = 0; i < w.length(); i++) {
                String pred = w.substring(0, i) + w.substring(i + 1);
                if (dp.containsKey(pred)) maxLen = Math.max(maxLen, dp.get(pred) + 1);
            }
            dp.put(w, maxLen);
            best = Math.max(best, maxLen);
        }
        return best;
    }
}
```
**Complexity:** O(n · L²) time (n words, L = max word length, each word tries L removals each costing O(L) to build the substring), O(n·L) space.

---

## 8. Longest Happy String

**Problem:** A "happy" string has no 3 consecutive identical characters. Given counts of available `a`, `b`, `c`, construct the longest possible happy string using at most those counts of each letter.

**Example:**
```
Input: a = 1, b = 1, c = 7
Output: "ccaccbcc"
Explanation: Uses up to 1 'a', 1 'b', 7 'c', with no 3 consecutive same characters, 
achieving maximum total length 8.
```

**Brute force:** try every character choice at each position, backtrack on violation → exponential.
**Optimized:** greedy with a max-heap — always try to place the currently most-abundant character (unless doing so would create 3-in-a-row, in which case place the next-most-abundant instead).
```java
class LongestHappyString {
    public String longestDiverseString(int a, int b, int c) {
        StringBuilder sb = new StringBuilder();
        PriorityQueue<int[]> pq = new PriorityQueue<>((x, y) -> y[0] - x[0]); // {count, charIndex}
        if (a > 0) pq.offer(new int[]{a, 0});
        if (b > 0) pq.offer(new int[]{b, 1});
        if (c > 0) pq.offer(new int[]{c, 2});
        char[] chars = {'a', 'b', 'c'};

        while (!pq.isEmpty()) {
            int[] top = pq.poll();
            int len = sb.length();
            if (len >= 2 && sb.charAt(len - 1) == chars[top[1]] && sb.charAt(len - 2) == chars[top[1]]) {
                if (pq.isEmpty()) break; // can't place top, and nothing else available
                int[] second = pq.poll();
                sb.append(chars[second[1]]);
                second[0]--;
                if (second[0] > 0) pq.offer(second);
                pq.offer(top); // put top back for next round
            } else {
                sb.append(chars[top[1]]);
                top[0]--;
                if (top[0] > 0) pq.offer(top);
            }
        }
        return sb.toString();
    }
}
```
**Complexity:** O((a+b+c) · log 3) time ≈ O(n), O(1) space (fixed 3-entry heap).

---

## 9. Longest Valid Parentheses

**Problem:** Given a string containing just `(` and `)`, find the length of the longest valid (well-formed) parentheses substring.

**Example:**
```
Input: s = ")()())"
Output: 4
Explanation: The longest valid substring is "()()" of length 4.
```

**Brute force:** check every substring for validity via a counter scan → O(n³) (O(n²) substrings × O(n) validity check), or O(n²) with incremental counters.
**Optimized:** `dp[i] = length of the longest valid substring ENDING at index i` — only meaningful when `s[i] == ')'`, with two cases based on `s[i-1]`.
```java
class LongestValidParentheses {
    public int longestValidParentheses(String s) {
        int n = s.length();
        int[] dp = new int[n];
        int maxLen = 0;
        for (int i = 1; i < n; i++) {
            if (s.charAt(i) == ')') {
                if (s.charAt(i - 1) == '(') {
                    dp[i] = (i >= 2 ? dp[i - 2] : 0) + 2;
                } else if (i - dp[i - 1] - 1 >= 0 && s.charAt(i - dp[i - 1] - 1) == '(') {
                    dp[i] = dp[i - 1] + 2 + (i - dp[i - 1] - 2 >= 0 ? dp[i - dp[i - 1] - 2] : 0);
                }
                maxLen = Math.max(maxLen, dp[i]);
            }
        }
        return maxLen;
    }
}
```
**Complexity:** O(n) time, O(n) space. (A stack-based O(n) time / O(n) space approach is an equally valid alternative.)

---

## 10. Distinct Subsequences

**Problem:** Given strings `s` and `t`, count the number of distinct subsequences of `s` that equal `t`.

**Example:**
```
Input: s = "rabbbit", t = "rabbit"
Output: 3
Explanation: There are 3 ways to select characters from s (by choosing which 
of the three 'b's to include/exclude appropriately) to spell "rabbit".
```

**Brute force:** try every subsequence of s, check equality to t → O(2ⁿ).
**Optimized:** `dp[i][j] = number of ways s[0..i) forms t[0..j)` — either skip `s[i-1]`, or (if characters match) also count using it.
```java
class DistinctSubsequences {
    public int numDistinct(String s, String t) {
        int m = s.length(), n = t.length();
        long[][] dp = new long[m + 1][n + 1];
        for (int i = 0; i <= m; i++) dp[i][0] = 1; // empty t is always formable (1 way: delete everything)

        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                dp[i][j] = dp[i - 1][j];
                if (s.charAt(i - 1) == t.charAt(j - 1)) dp[i][j] += dp[i - 1][j - 1];
            }
        }
        return (int) dp[m][n];
    }
}
```
**Complexity:** O(m·n) time, O(m·n) space (reducible to O(n) with a rolling array).

---

## 11. Word Break II

**Problem:** Given a string `s` and a dictionary, return ALL possible sentences (space-separated dictionary words) that reconstruct `s`.

**Example:**
```
Input: s = "catsanddog", wordDict = ["cat","cats","and","sand","dog"]
Output: ["cats and dog","cat sand dog"]
```

**Brute force:** try every split combination without memoizing repeated suffix computations → exponential blowup on inputs with many valid partial breaks.
**Optimized:** memoized DFS — `memo[start] = list of all valid sentences for s[start..]`, reused whenever the same starting index recurs.
```java
class WordBreakII {
    public List<String> wordBreak(String s, List<String> wordDict) {
        Set<String> dict = new HashSet<>(wordDict);
        Map<Integer, List<String>> memo = new HashMap<>();
        return helper(s, 0, dict, memo);
    }

    private List<String> helper(String s, int start, Set<String> dict, Map<Integer, List<String>> memo) {
        if (memo.containsKey(start)) return memo.get(start);
        List<String> result = new ArrayList<>();
        if (start == s.length()) { result.add(""); return result; }

        for (int end = start + 1; end <= s.length(); end++) {
            String word = s.substring(start, end);
            if (dict.contains(word)) {
                List<String> rest = helper(s, end, dict, memo);
                for (String r : rest) {
                    result.add(word + (r.isEmpty() ? "" : " " + r));
                }
            }
        }
        memo.put(start, result);
        return result;
    }
}
```
**Complexity:** O(2ⁿ) worst case (output itself can be exponential for pathological inputs), but memoization prevents redundant recomputation of shared suffixes — O(n²) distinct subproblems, each doing O(n) work to enumerate splits.

---

## 12. Count The Repetitions

**Problem:** Define `[s, n]` as the string `s` concatenated `n` times. Given `s1, n1, s2, n2`, find the maximum integer `m` such that `[s2, m]` is a subsequence of `[s1, n1]`.

**Example:**
```
Input: s1 = "acb", n1 = 4, s2 = "ab", n2 = 2
Output: 2
Explanation: [s1,4] = "acbacbacbacb" contains "ab" as a subsequence 4 times total,
so [s2,2] = "abab" fits floor(4/2) = 2 times as a subsequence.
```

**Brute force:** simulate character-by-character through all of `[s1,n1]` directly, matching against `s2` — feasible only if `n1` is small; for large `n1` this is too slow without exploiting the cyclic structure.
**Optimized:** precompute, for each of the `len(s2)` possible starting positions within `s2`, how many full `s2` matches occur and what the ending position is after ONE pass through `s1` — since there are only `len(s2)` distinct starting states, avoid recomputing the O(len(s1)) scan per s1-copy from scratch by reusing this precomputed transition table.
```java
class CountTheRepetitions {
    public int getMaxRepetitions(String s1, int n1, String s2, int n2) {
        if (n1 == 0) return 0;
        int len1 = s1.length(), len2 = s2.length();

        int[] countMatchedAfterOneS1 = new int[len2]; // s2-matches completed, starting from index `start`
        int[] nextIndex = new int[len2];               // resulting index into s2 after one pass through s1

        for (int start = 0; start < len2; start++) {
            int idx = start, count = 0;
            for (int i = 0; i < len1; i++) {
                if (s1.charAt(i) == s2.charAt(idx)) {
                    idx++;
                    if (idx == len2) { idx = 0; count++; }
                }
            }
            countMatchedAfterOneS1[start] = count;
            nextIndex[start] = idx;
        }

        int idx = 0, s2Count = 0;
        for (int i = 0; i < n1; i++) {
            s2Count += countMatchedAfterOneS1[idx];
            idx = nextIndex[idx];
        }
        return s2Count / n2;
    }
}
```
**Complexity:** O(len1 · len2 + n1) time, O(len2) space — the precompute step avoids needing full cycle-detection machinery since the transition table has only `len2` distinct states.

---

## 13. Concatenated Words

**Problem:** Given a list of words (no duplicates), return all words that are formed entirely by concatenating at least two shorter words from the SAME list.

**Example:**
```
Input: words = ["cat","cats","catsdogcats","dog","dogcatsdog","hippopotamuses","rat","ratcatdogcat"]
Output: ["catsdogcats","dogcatsdog","ratcatdogcat"]
```

**Brute force:** for each word, try every possible split into 2+ parts recursively without memo → exponential per word.
**Optimized:** for each word, run a Word-Break-style DP against the full dictionary (excluding the word itself), which is O(L²) per word.
```java
class ConcatenatedWords {
    public List<String> findAllConcatenatedWordsInADict(String[] words) {
        Set<String> dict = new HashSet<>(Arrays.asList(words));
        List<String> result = new ArrayList<>();
        for (String word : words) {
            if (canForm(word, dict)) result.add(word);
        }
        return result;
    }

    private boolean canForm(String word, Set<String> dict) {
        int n = word.length();
        if (n == 0) return false;
        boolean[] dp = new boolean[n + 1];
        dp[0] = true;
        for (int i = 1; i <= n; i++) {
            for (int j = 0; j < i; j++) {
                if (!dp[j]) continue;
                String sub = word.substring(j, i);
                if (!sub.equals(word) && dict.contains(sub)) { // exclude matching the whole word itself
                    dp[i] = true;
                    break;
                }
            }
        }
        return dp[n];
    }
}
```
**Complexity:** O(N · L³) time total (N words, L = max length, each Word-Break-style check is O(L²) with O(L) substring cost), O(N·L) space for the dictionary set.

---

## 14. Count Different Palindromic Subsequences

**Problem:** Given a string `s` (typically over a small alphabet like `a-d`), count the number of DIFFERENT (distinct-valued) non-empty palindromic subsequences, modulo 10⁹+7.

**Example:**
```
Input: s = "bccb"
Output: 6
Explanation: The 6 different palindromic subsequences are: 
"b", "c", "bb", "cc", "bcb", "bccb".
```

**Brute force:** enumerate every subsequence, check palindrome property, deduplicate via a set → O(2ⁿ · n).
**Optimized:** interval DP `dp[i][j] = count of distinct palindromic subsequences in s[i..j]`, using boundary-character matching to avoid double-counting (find the first/last occurrence of the boundary character strictly inside the interval).
```java
class CountDifferentPalindromicSubsequences {
    public int countPalindromicSubsequences(String s) {
        int n = s.length();
        long MOD = 1_000_000_007;
        long[][] dp = new long[n][n];
        for (int i = 0; i < n; i++) dp[i][i] = 1;

        for (int len = 2; len <= n; len++) {
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                char lo = s.charAt(i), hi = s.charAt(j);
                if (lo != hi) {
                    dp[i][j] = (dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1] + MOD) % MOD;
                } else {
                    int low = i + 1, high = j - 1;
                    while (low <= high && s.charAt(low) != lo) low++;
                    while (low <= high && s.charAt(high) != lo) high--;

                    if (low > high) {
                        dp[i][j] = (2 * dp[i + 1][j - 1] + 2) % MOD;
                    } else if (low == high) {
                        dp[i][j] = (2 * dp[i + 1][j - 1] + 1) % MOD;
                    } else {
                        dp[i][j] = (2 * dp[i + 1][j - 1] - dp[low + 1][high - 1] + MOD) % MOD;
                    }
                }
            }
        }
        return (int) ((dp[0][n - 1] % MOD + MOD) % MOD);
    }
}
```
**Complexity:** O(n²) time (the inner boundary search adds at most O(n) per cell but amortizes; standard analysis treats this as O(n²)), O(n²) space — one of the harder standard interval DP problems due to the deduplication logic.

---

## 15. Distinct Subsequences II

**Problem:** Given a string `s`, count the number of DISTINCT non-empty subsequences, modulo 10⁹+7.

**Example:**
```
Input: s = "abc"
Output: 7
Explanation: The 7 distinct subsequences are: "a","b","c","ab","ac","bc","abc".
```

**Brute force:** enumerate all 2ⁿ subsequences, deduplicate via a set → O(2ⁿ · n).
**Optimized:** `dp[i] = distinct subsequence count using first i characters` — doubling each step (append current char to every prior subsequence, plus the char alone), then subtracting the overcounted duplicates from the last occurrence of the same character.
```java
class DistinctSubsequencesII {
    public int distinctSubseqII(String s) {
        long MOD = 1_000_000_007;
        int n = s.length();
        long[] dp = new long[n + 1];
        dp[0] = 1; // empty subsequence baseline
        int[] last = new int[26];
        Arrays.fill(last, -1);

        for (int i = 1; i <= n; i++) {
            dp[i] = (2 * dp[i - 1]) % MOD;
            int c = s.charAt(i - 1) - 'a';
            if (last[c] != -1) {
                dp[i] = (dp[i] - dp[last[c] - 1] + MOD) % MOD; // remove duplicates from last occurrence
            }
            last[c] = i;
        }
        return (int) ((dp[n] - 1 + MOD) % MOD); // subtract the empty subsequence
    }
}
```
**Complexity:** O(n) time, O(1) extra space (fixed 26-entry array) — beats the O(2ⁿ) brute force enormously.

---

## 16. Longest Chunked Palindrome Decomposition

**Problem:** Split a string `text` into `k` substrings (`text = subtexts[0] + subtexts[1] + ... + subtexts[k-1]`) such that `subtexts[i] == reverse(subtexts[k-1-i])` for every `i`. Maximize `k`.

**Example:**
```
Input: text = "ghiabcdefhelloadamhelloabcdefghi"
Output: 7
Explanation: One optimal decomposition: ("ghi","abcdef","hello","adam","hello","abcdef","ghi").
```

**Brute force:** try every possible split point combination checking the mirror property → exponential.
**Optimized:** greedy two-pointer from both ends — grow the smallest possible matching prefix/suffix pair, recurse on the middle remainder (proven optimal: taking the shortest valid match never hurts future options).
```java
class LongestChunkedPalindromeDecomposition {
    public int longestDecomposition(String text) {
        if (text.isEmpty()) return 0;
        int n = text.length();
        for (int len = 1; len <= n / 2; len++) {
            if (text.substring(0, len).equals(text.substring(n - len, n))) {
                return 2 + longestDecomposition(text.substring(len, n - len));
            }
        }
        return 1; // no matching prefix/suffix found — whole remaining string is one chunk
    }
}
```
**Complexity:** O(n²) time worst case (n recursive levels, each doing O(n) substring comparison), O(n) space — the greedy shortest-match strategy is provably optimal here, avoiding the need for exponential search.

---

## 17. Palindrome Partitioning III

**Problem:** Given a string `s` and an integer `k`, change the minimum number of characters in `s` so it can be partitioned into exactly `k` palindromic substrings.

**Example:**
```
Input: s = "abc", k = 2
Output: 1
Explanation: Change to "abb" (1 change), then partition as "a" | "bb" — both palindromes.
```

**Brute force:** try every partition into k parts, compute change-cost for each part independently from scratch → exponential partition choices × O(n) cost computation.
**Optimized:** precompute `cost[i][j] = min changes to make s[i..j] a palindrome` via interval DP, then `dp[i][k] = min total cost to partition first i chars into k palindromic parts`.
```java
class PalindromePartitioningIII {
    public int palindromePartition(String s, int k) {
        int n = s.length();
        int[][] cost = new int[n][n];
        for (int len = 2; len <= n; len++) {
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                cost[i][j] = cost[i + 1][j - 1] + (s.charAt(i) == s.charAt(j) ? 0 : 1);
            }
        }

        int[][] dp = new int[n + 1][k + 1];
        for (int[] row : dp) Arrays.fill(row, Integer.MAX_VALUE / 2);
        dp[0][0] = 0;

        for (int i = 1; i <= n; i++) {
            for (int parts = 1; parts <= Math.min(i, k); parts++) {
                for (int j = parts - 1; j < i; j++) {
                    dp[i][parts] = Math.min(dp[i][parts], dp[j][parts - 1] + cost[j][i - 1]);
                }
            }
        }
        return dp[n][k];
    }
}
```
**Complexity:** O(n²) for cost precompute + O(n²·k) for the partition DP = O(n²·k) time, O(n²) space.

---

## 18. Find All Good Strings

**Problem:** Given `n`, and two strings `s1 ≤ s2` (lexicographically, both length n), count strings of length n in the range `[s1, s2]` that do NOT contain `evil` as a substring, modulo 10⁹+7.

**Example:**
```
Input: n = 2, s1 = "aa", s2 = "da", evil = "b"
Output: 51
Explanation: Out of all strings from "aa" to "da" (length 2), 51 do not contain "b".
```

**Brute force:** generate every string in the range and check for the evil substring directly → O((range size) · n) — intractable for large n (range can be astronomically large).
**Optimized:** digit-DP over string positions combined with a KMP failure-function automaton (tracks partial matches of `evil` so we never need to re-scan) — `dp[pos][kmpState][tight] = count of valid completions`. Compute `count(≤ s2) - count(≤ s1) + (1 if s1 itself is valid)`.
```java
class FindAllGoodStrings {
    private static final long MOD = 1_000_000_007;

    public int findGoodStrings(int n, String s1, String s2, String evil) {
        int m = evil.length();
        int[] lps = buildLPS(evil);
        int[][] automaton = buildAutomaton(evil, lps);

        long countUpToS2 = countAtMost(s2, n, automaton, m);
        long countUpToS1 = countAtMost(s1, n, automaton, m);
        long result = (countUpToS2 - countUpToS1 + MOD) % MOD;
        if (!s1.contains(evil)) result = (result + 1) % MOD;
        return (int) result;
    }

    private int[] buildLPS(String pattern) {
        int m = pattern.length();
        int[] lps = new int[m];
        int len = 0, i = 1;
        while (i < m) {
            if (pattern.charAt(i) == pattern.charAt(len)) lps[i++] = ++len;
            else if (len > 0) len = lps[len - 1];
            else lps[i++] = 0;
        }
        return lps;
    }

    private int[][] buildAutomaton(String pattern, int[] lps) {
        int m = pattern.length();
        int[][] automaton = new int[m][26];
        for (int state = 0; state < m; state++) {
            for (char c = 'a'; c <= 'z'; c++) {
                if (c == pattern.charAt(state)) {
                    automaton[state][c - 'a'] = state + 1;
                } else {
                    automaton[state][c - 'a'] = (state == 0) ? 0 : automaton[lps[state - 1]][c - 'a'];
                }
            }
        }
        return automaton;
    }

    private long countAtMost(String bound, int n, int[][] automaton, int m) {
        Long[][][] memo = new Long[n + 1][m + 1][2];
        return dfs(0, 0, true, bound, n, automaton, m, memo);
    }

    private long dfs(int pos, int state, boolean tight, String bound, int n, int[][] automaton, int m, Long[][][] memo) {
        if (state == m) return 0; // evil pattern fully matched — invalid
        if (pos == n) return 1;
        int tightIdx = tight ? 1 : 0;
        if (memo[pos][state][tightIdx] != null) return memo[pos][state][tightIdx];

        int limit = tight ? (bound.charAt(pos) - 'a') : 25;
        long total = 0;
        for (int c = 0; c <= limit; c++) {
            int newState = automaton[state][c];
            boolean newTight = tight && (c == limit);
            total = (total + dfs(pos + 1, newState, newTight, bound, n, automaton, m, memo)) % MOD;
        }
        memo[pos][state][tightIdx] = total;
        return total;
    }
}
```
**Complexity:** O(n · m · 26) time (n positions × m evil-automaton states × 26 letters, each memoized once), O(n·m) space — the KMP automaton is essential to avoid O(n·m) re-scanning per character choice.

---

## 19. String Compression II

**Problem:** Given a string `s` and integer `k`, you may delete up to `k` characters. Return the minimum possible length of the run-length-encoded (RLE) compression of the resulting string (RLE: each run of identical characters becomes `char` + `count` if count>1, digits split when count reaches double/triple digits).

**Example:**
```
Input: s = "aaabcccd", k = 2
Output: 4
Explanation: Delete 'b' and 'd': "aaaccc" compresses to "a3c3" — length 4.
```

**Brute force:** try every subset of ≤k characters to delete, compute RLE length for each → O(C(n,k) · n).
**Optimized:** `dp[i][k] = min compressed length for first i chars using k deletions remaining`, trying every possible "next run" length/deletion combination.
```java
class StringCompressionII {
    public int getLengthOfOptimalCompression(String s, int k) {
        int n = s.length();
        int[][] dp = new int[n + 1][k + 1];
        for (int[] row : dp) Arrays.fill(row, Integer.MAX_VALUE / 2);
        dp[0][0] = 0;

        for (int i = 1; i <= n; i++) {
            for (int j = 0; j <= k; j++) {
                if (j > 0) dp[i][j] = Math.min(dp[i][j], dp[i - 1][j - 1]); // delete s[i-1] outright

                int count = 0, del = 0;
                for (int l = i; l >= 1; l--) {
                    if (s.charAt(l - 1) == s.charAt(i - 1)) count++;
                    else del++;
                    if (j - del < 0) break;
                    if (dp[l - 1][j - del] != Integer.MAX_VALUE / 2) {
                        int lenAdd = (count == 1) ? 1 : (count < 10 ? 2 : (count < 100 ? 3 : 4));
                        dp[i][j] = Math.min(dp[i][j], dp[l - 1][j - del] + lenAdd);
                    }
                }
            }
        }
        return dp[n][k];
    }
}
```
**Complexity:** O(n² · k) time, O(n·k) space — one of the trickier standard DP problems due to the digit-length-jump encoding logic (1→2→3 char length at count 1, 10, 100).

---

## 20. Number of Ways to Form a Target String Given a Dictionary

**Problem:** Given a list of equal-length words and a target string, count the number of ways to build the target by picking one character from each successive word-column position (moving strictly left-to-right through columns, each column used at most once), modulo 10⁹+7.

**Example:**
```
Input: words = ["acca","bbbb","caca"], target = "aba"
Output: 6
Explanation: Multiple ways exist to pick column-positions matching 'a','b','a' 
in order across the words, giving 6 total valid combinations.
```

**Brute force:** try every combination of column choices for the target's letters recursively → exponential.
**Optimized:** `dp[i][j] = ways to form first j characters of target using the first i columns`, using precomputed per-column letter-frequency counts.
```java
class NumWaysFormTargetString {
    public int numWays(String[] words, String target) {
        long MOD = 1_000_000_007;
        int wordLen = words[0].length(), targetLen = target.length();

        int[][] count = new int[wordLen][26];
        for (String w : words) {
            for (int i = 0; i < wordLen; i++) count[i][w.charAt(i) - 'a']++;
        }

        long[][] dp = new long[wordLen + 1][targetLen + 1];
        for (int i = 0; i <= wordLen; i++) dp[i][0] = 1; // empty target always formable

        for (int i = 1; i <= wordLen; i++) {
            for (int j = 1; j <= targetLen; j++) {
                dp[i][j] = dp[i - 1][j]; // skip this column entirely
                int c = target.charAt(j - 1) - 'a';
                dp[i][j] = (dp[i][j] + (long) count[i - 1][c] * dp[i - 1][j - 1]) % MOD;
            }
        }
        return (int) dp[wordLen][targetLen];
    }
}
```
**Complexity:** O(wordLen · targetLen) time (with O(numWords · wordLen) precompute for column counts), O(wordLen · targetLen) space.

---

## 🎯 Part 8 Summary Table

| # | Problem | Time | Space |
|---|---|---|---|
| 1 | Is Subsequence | O(n) | O(1) |
| 2 | Palindrome Partitioning | O(n²+2ⁿ) | O(n²) |
| 3 | Palindrome Partitioning II | O(n²) | O(n²) |
| 4 | Word Break | O(n²) | O(n) |
| 5 | Unique Substrings Wraparound | O(n) | O(1) |
| 6 | Min ASCII Delete Sum | O(m·n) | O(m·n) |
| 7 | Longest String Chain | O(n·L²) | O(n·L) |
| 8 | Longest Happy String | O(n log 3) | O(1) |
| 9 | Longest Valid Parentheses | O(n) | O(n) |
| 10 | Distinct Subsequences | O(m·n) | O(m·n) |
| 11 | Word Break II | O(n²) subproblems | O(n²) |
| 12 | Count The Repetitions | O(len1·len2+n1) | O(len2) |
| 13 | Concatenated Words | O(N·L³) | O(N·L) |
| 14 | Count Different Palindromic Subseq | O(n²) | O(n²) |
| 15 | Distinct Subsequences II | O(n) | O(1) |
| 16 | Longest Chunked Palindrome | O(n²) | O(n) |
| 17 | Palindrome Partitioning III | O(n²·k) | O(n²) |
| 18 | Find All Good Strings | O(n·m·26) | O(n·m) |
| 19 | String Compression II | O(n²·k) | O(n·k) |
| 20 | Ways to Form Target String | O(wordLen·targetLen) | O(wordLen·targetLen) |

---

**Next: Part 9 — Probability DP (3 problems).** Say "continue" to proceed, or name a category to jump to.
