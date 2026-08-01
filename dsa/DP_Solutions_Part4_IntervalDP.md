# DP Solutions — Part 4: Interval DP (Java)
### 17 Problems · Full Problem Statement + Example + Brute Force → Optimized + Complexity

---

## 1. Guess Number Higher or Lower II

**Problem:** You're playing a guessing game with numbers 1 to n. Each time you guess a number `x` and it's wrong, you pay `x` dollars, and you're told whether the actual number is higher or lower — so you know which half remains. Find the minimum amount of money guaranteed to win, regardless of what number was picked (worst-case strategy).

**Example:**
```
Input: n = 10
Output: 16
Explanation: The optimal strategy first guesses 7. Worst case leads to guessing
sequences costing at most 16 total (e.g., guess 7, wrong→guess 3, wrong→guess 1 or 2).
```

**Brute force:** try every possible first guess, recursively solve both halves, no memo → exponential (O(2ⁿ)).
**Optimized:** `dp[i][j] = min cost to guarantee a win for range [i,j]`, trying every possible pivot guess `x` in the range.
```java
class GuessNumberHigherOrLowerII {
    public int getMoneyAmount(int n) {
        int[][] dp = new int[n + 2][n + 2];
        for (int len = 2; len <= n; len++) {
            for (int i = 1; i + len - 1 <= n; i++) {
                int j = i + len - 1;
                dp[i][j] = Integer.MAX_VALUE;
                for (int x = i; x <= j; x++) {
                    int left = (x > i) ? dp[i][x - 1] : 0;
                    int right = (x < j) ? dp[x + 1][j] : 0;
                    int cost = x + Math.max(left, right);
                    dp[i][j] = Math.min(dp[i][j], cost);
                }
            }
        }
        return dp[1][n];
    }
}
```
**Complexity:** O(n³) time (n² intervals × n pivot choices), O(n²) space.

---

## 2. Arithmetic Slices

**Problem:** Given an integer array, return the number of contiguous subarrays that are arithmetic (have constant difference between consecutive elements) and have length ≥ 3.

**Example:**
```
Input: nums = [1,2,3,4]
Output: 3
Explanation: [1,2,3], [2,3,4], and [1,2,3,4] are all arithmetic slices.
```

**Brute force:** check every subarray for the arithmetic property → O(n³) (or O(n²) with incremental checking).
**Optimized:** `dp[i] = number of arithmetic slices ending at i` — if `nums[i]-nums[i-1] == nums[i-1]-nums[i-2]`, then `dp[i] = dp[i-1] + 1`.
```java
class ArithmeticSlices {
    public int numberOfArithmeticSlices(int[] nums) {
        int n = nums.length;
        int dp = 0, total = 0;
        for (int i = 2; i < n; i++) {
            if (nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2]) {
                dp += 1;
                total += dp;
            } else {
                dp = 0;
            }
        }
        return total;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 3. Predict the Winner

**Problem:** Two players take turns picking a number from either end of an array, adding it to their own score. Given optimal play from both, determine if Player 1 can win or tie (score1 ≥ score2).

**Example:**
```
Input: nums = [1,5,233,7]
Output: true
Explanation: Player 1 picks 1, forcing Player 2 into a position where Player 1
can eventually secure 234 vs Player 2's 12.
```

**Brute force:** simulate every possible sequence of choices for both players → O(2ⁿ).
**Optimized:** `dp[i][j] = max score DIFFERENCE (current player − opponent) achievable on subarray [i,j]`.
```java
class PredictTheWinner {
    public boolean predictTheWinner(int[] nums) {
        int n = nums.length;
        int[][] dp = new int[n][n];
        for (int i = 0; i < n; i++) dp[i][i] = nums[i];

        for (int len = 2; len <= n; len++) {
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                dp[i][j] = Math.max(nums[i] - dp[i + 1][j], nums[j] - dp[i][j - 1]);
            }
        }
        return dp[0][n - 1] >= 0;
    }
}
```
**Complexity:** O(n²) time, O(n²) space (reducible to O(n) with a rolling array).

---

## 4. Palindromic Substrings

**Problem:** Given a string, count how many substrings (contiguous) are palindromes. Different positions counting as different substrings even if the text is the same.

**Example:**
```
Input: s = "aaa"
Output: 6
Explanation: "a","a","a","aa","aa","aaa" — six palindromic substrings total.
```

**Brute force:** check every substring for palindrome property independently → O(n³).
**Optimized Solution 1 — Interval DP:** `dp[i][j] = true if s[i..j] is a palindrome`, built from shorter intervals outward.
```java
class PalindromicSubstringsDP {
    public int countSubstrings(String s) {
        int n = s.length();
        boolean[][] dp = new boolean[n][n];
        int count = 0;
        for (int len = 1; len <= n; len++) {
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                if (s.charAt(i) == s.charAt(j) && (len <= 2 || dp[i + 1][j - 1])) {
                    dp[i][j] = true;
                    count++;
                }
            }
        }
        return count;
    }
}
```
**Complexity:** O(n²) time, O(n²) space.

### Optimized Solution 2 — Expand Around Center (better space)
```java
class PalindromicSubstringsExpand {
    public int countSubstrings(String s) {
        int count = 0;
        for (int center = 0; center < s.length(); center++) {
            count += expand(s, center, center);     // odd length
            count += expand(s, center, center + 1); // even length
        }
        return count;
    }

    private int expand(String s, int left, int right) {
        int count = 0;
        while (left >= 0 && right < s.length() && s.charAt(left) == s.charAt(right)) {
            count++;
            left--;
            right++;
        }
        return count;
    }
}
```
**Complexity:** O(n²) time, O(1) space — same time complexity but far better space than the DP table.

---

## 5. Stone Game

**Problem:** Alex and Lee take turns removing a pile from either end of a row of stone piles (even number of piles, odd total number of stones so no ties). Both play optimally to maximize their own stones. Return true if Alex (first player) wins.

**Example:**
```
Input: piles = [5,3,4,5]
Output: true
Explanation: Alex can always win with optimal play — takes 5, then reacts to
Lee's choices to secure at least 5+5+3=13 total vs Lee's 4+3=... totaling 18 vs 8... 
(In this classic example, Alex wins 18-4 total... i.e. Alex ends up ahead.)
```

**Brute force:** simulate every game tree of choices → O(2ⁿ).
**Optimized:** identical pattern to Predict the Winner — `dp[i][j] = score difference for range [i,j]`.
```java
class StoneGame {
    public boolean stoneGame(int[] piles) {
        int n = piles.length;
        int[][] dp = new int[n][n];
        for (int i = 0; i < n; i++) dp[i][i] = piles[i];

        for (int len = 2; len <= n; len++) {
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                dp[i][j] = Math.max(piles[i] - dp[i + 1][j], piles[j] - dp[i][j - 1]);
            }
        }
        return dp[0][n - 1] > 0;
    }
}
```
**Complexity:** O(n²) time, O(n²) space. (Note: with the given constraints — even piles, odd total — Alex always wins, so this can also be solved in O(1) via a parity/math argument, but the DP is the general technique.)

---

## 6. Minimum Score Triangulation of Polygon

**Problem:** Given the values of vertices of a convex polygon (in order), triangulate it into non-overlapping triangles using all vertices. The score of a triangle is the product of its 3 vertex values; return the minimum total score across all triangles.

**Example:**
```
Input: values = [1,2,3]
Output: 6
Explanation: Only one triangle possible: 1*2*3 = 6.

Input: values = [3,7,4,5]
Output: 144
Explanation: Best triangulation gives 3*4*5 + 4*7*5 = 60 + 140... actually the
minimum is achieved by triangulating as (0,1,3) and (1,2,3): 3*7*5 + 7*4*5 = 105+140=245,
vs (0,1,2)+(0,2,3): 3*7*4 + 3*4*5 = 84+60 = 144 (minimum).
```

**Brute force:** try every possible triangulation via recursive edge selection → exponential (Catalan-number many triangulations).
**Optimized:** `dp[i][j] = min score to triangulate polygon slice from vertex i to vertex j`, trying every intermediate vertex k as the third point of the triangle on edge (i,j).
```java
class MinScoreTriangulation {
    public int minScoreTriangulation(int[] values) {
        int n = values.length;
        int[][] dp = new int[n][n];

        for (int len = 2; len < n; len++) {
            for (int i = 0; i + len < n; i++) {
                int j = i + len;
                dp[i][j] = Integer.MAX_VALUE;
                for (int k = i + 1; k < j; k++) {
                    int score = dp[i][k] + dp[k][j] + values[i] * values[k] * values[j];
                    dp[i][j] = Math.min(dp[i][j], score);
                }
            }
        }
        return dp[0][n - 1];
    }
}
```
**Complexity:** O(n³) time, O(n²) space.

---

## 7. Last Stone Weight II

**Problem:** You have stones with given weights. Repeatedly smash the two heaviest together: if equal weights, both destroyed; otherwise, the smaller is destroyed and the larger becomes `|x-y|`. Return the smallest possible weight of the last remaining stone (choosing smash order optimally), or 0 if none remain.

**Example:**
```
Input: stones = [2,7,4,1,8,1]
Output: 1
Explanation: Optimal smashing sequence: 2,4→2; 7,1→6; 1,8→7; 2,6→4; 4,7→3; 
best achievable leftover is 1.
```

**Brute force:** simulate every possible smash order → exponential.
**Optimized insight:** equivalent to partitioning stones into two groups minimizing `|sumA - sumB|` — a subset-sum knapsack problem.
```java
class LastStoneWeightII {
    public int lastStoneWeightII(int[] stones) {
        int totalSum = Arrays.stream(stones).sum();
        int target = totalSum / 2;
        boolean[] dp = new boolean[target + 1];
        dp[0] = true;

        for (int stone : stones) {
            for (int s = target; s >= stone; s--) {
                dp[s] = dp[s] || dp[s - stone];
            }
        }

        for (int s = target; s >= 0; s--) {
            if (dp[s]) return totalSum - 2 * s;
        }
        return 0;
    }
}
```
**Complexity:** O(n · sum) time, O(sum) space. (Listed under "Interval DP" in the source list, but the actual optimal technique is subset-sum knapsack — included here for completeness against the original grouping.)

---

## 8. Minimum Cost Tree From Leaf Values

**Problem:** Given an array of positive integers as leaf values of a binary tree (in-order traversal), build a tree where every non-leaf node's value equals the product of the largest leaf value in its left and right subtrees. Minimize the total sum of non-leaf node values.

**Example:**
```
Input: arr = [6,2,4]
Output: 32
Explanation: Best tree: leaves 6,2,4 → non-leaf nodes are max(6,2)*max(2,4)=6*4=24
and max(6,2)=6 * 4 = ... the two valid trees give 36 or 32; 32 is minimal
(pair 2,4 first: 2*4=8, then 6*max(2,4)=6*4=24; total 8+24=32).
```

**Brute force:** try every way to merge adjacent leaves (matrix-chain style recursion) → exponential without memo.
**Optimized Solution 1 — Interval DP:** `dp[i][j] = min cost to merge leaves i..j`, with `maxVal[i][j]` precomputed.
```java
class MinCostTreeDP {
    public int mctFromLeafValues(int[] arr) {
        int n = arr.length;
        int[][] maxVal = new int[n][n];
        for (int i = 0; i < n; i++) {
            maxVal[i][i] = arr[i];
            for (int j = i + 1; j < n; j++) {
                maxVal[i][j] = Math.max(maxVal[i][j - 1], arr[j]);
            }
        }

        int[][] dp = new int[n][n];
        for (int len = 2; len <= n; len++) {
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                dp[i][j] = Integer.MAX_VALUE;
                for (int k = i; k < j; k++) {
                    int cost = dp[i][k] + dp[k + 1][j] + maxVal[i][k] * maxVal[k + 1][j];
                    dp[i][j] = Math.min(dp[i][j], cost);
                }
            }
        }
        return dp[0][n - 1];
    }
}
```
**Complexity:** O(n³) time, O(n²) space.

### Optimized Solution 2 — Monotonic Stack (much better complexity)
```java
class MinCostTreeStack {
    public int mctFromLeafValues(int[] arr) {
        int result = 0;
        Deque<Integer> stack = new ArrayDeque<>();
        stack.push(Integer.MAX_VALUE);

        for (int val : arr) {
            while (stack.peek() <= val) {
                int mid = stack.pop();
                result += mid * Math.min(stack.peek(), val);
            }
            stack.push(val);
        }
        while (stack.size() > 2) {
            result += stack.pop() * stack.peek();
        }
        return result;
    }
}
```
**Complexity:** O(n) time, O(n) space — a greedy monotonic-stack insight (always merge the smallest adjacent-in-stack elements first) beats the O(n³) DP significantly.

---

## 9. Stone Game VII

**Problem:** Players alternate removing a stone from either end of the row; the score gained is the sum of the REMAINING stones (not the removed one). Both play optimally to maximize the difference between their score and their opponent's. Return that difference.

**Example:**
```
Input: stones = [5,3,1,4,2]
Output: 6
Explanation: With optimal play, Alice ends up 6 points ahead of Bob.
```

**Brute force:** simulate all choice sequences → O(2ⁿ).
**Optimized:** `dp[i][j] = best score difference achievable on subarray [i,j]`, using prefix sums for O(1) remaining-sum lookup.
```java
class StoneGameVII {
    public int stoneGameVII(int[] stones) {
        int n = stones.length;
        int[] prefix = new int[n + 1];
        for (int i = 0; i < n; i++) prefix[i + 1] = prefix[i] + stones[i];

        int[][] dp = new int[n][n];
        for (int len = 2; len <= n; len++) {
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                int sum = prefix[j + 1] - prefix[i];
                dp[i][j] = Math.max(
                    (sum - stones[i]) - dp[i + 1][j],
                    (sum - stones[j]) - dp[i][j - 1]
                );
            }
        }
        return dp[0][n - 1];
    }
}
```
**Complexity:** O(n²) time, O(n²) space.

---

## 10. Burst Balloons

**Problem:** Given n balloons with numbers on them, bursting balloon `i` gives `nums[i-1]*nums[i]*nums[i+1]` coins (out-of-bound treated as 1). Find the maximum coins from bursting all balloons in some order.

**Example:**
```
Input: nums = [3,1,5,8]
Output: 167
Explanation: Burst order 1,5,3,8 → coins = 3*1*5 + 3*5*8 + 1*3*8 + 1*8*1 = 
15+120+24+8 = 167.
```

**Brute force:** try every burst order → O(n!).
**Optimized:** think in REVERSE — instead of "which balloon to burst first", ask "which balloon to burst LAST in range [i,j]". `dp[i][j] = max coins bursting all balloons strictly between i and j`, with padded boundary balloons of value 1.
```java
class BurstBalloons {
    public int maxCoins(int[] nums) {
        int n = nums.length;
        int[] balloons = new int[n + 2];
        balloons[0] = balloons[n + 1] = 1;
        for (int i = 0; i < n; i++) balloons[i + 1] = nums[i];

        int[][] dp = new int[n + 2][n + 2];
        for (int len = 1; len <= n; len++) {
            for (int left = 1; left + len - 1 <= n; left++) {
                int right = left + len - 1;
                for (int k = left; k <= right; k++) { // k = last balloon burst in (left, right)
                    int coins = balloons[left - 1] * balloons[k] * balloons[right + 1]
                        + dp[left][k - 1] + dp[k + 1][right];
                    dp[left][right] = Math.max(dp[left][right], coins);
                }
            }
        }
        return dp[1][n];
    }
}
```
**Complexity:** O(n³) time, O(n²) space — a massive improvement over O(n!) by reframing "first burst" as "last burst" to decouple subproblems.

---

## 11. Remove Boxes

**Problem:** Given boxes with colors, repeatedly remove a contiguous group of boxes with the same color, earning `k²` points for a group of size k. Maximize total points removing all boxes.

**Example:**
```
Input: boxes = [1,3,2,2,2,3,4,3,1]
Output: 23
Explanation: Optimal: remove [2,2,2] (9 pts) → [1,3,3,4,3,1] → remove [4] (1 pt)
→ [1,3,3,3,1] → remove [3,3,3] (9 pts) → [1,1] → remove [1,1] (4 pts). Total 23.
```

**Brute force:** try every possible removal order/grouping → exponential.
**Optimized:** `dp[i][j][k] = max points for subarray [i,j] where k extra boxes of boxes[i]'s color are attached from the left` (3D interval DP with an auxiliary "count" dimension — one of the hardest standard interval DP patterns).
```java
class RemoveBoxes {
    public int removeBoxes(int[] boxes) {
        int n = boxes.length;
        int[][][] dp = new int[n][n][n];
        return solve(boxes, 0, n - 1, 0, dp);
    }

    private int solve(int[] boxes, int i, int j, int k, int[][][] dp) {
        if (i > j) return 0;
        if (dp[i][j][k] > 0) return dp[i][j][k];

        int origI = i, origK = k;
        // merge consecutive same-color boxes at the start into k
        while (i + 1 <= j && boxes[i + 1] == boxes[i]) { i++; k++; }

        int result = (k + 1) * (k + 1) + solve(boxes, i + 1, j, 0, dp);
        for (int m = i + 1; m <= j; m++) {
            if (boxes[m] == boxes[i]) {
                result = Math.max(result, solve(boxes, i + 1, m - 1, 0, dp) + solve(boxes, m, j, k + 1, dp));
            }
        }
        dp[origI][j][origK] = result;
        return result;
    }
}
```
**Complexity:** O(n⁴) time (n³ states, O(n) transition), O(n³) space — one of the more advanced interval DP problems.

---

## 12. Strange Printer

**Problem:** A printer can print a sequence of the same character each turn, and can overwrite existing characters. Given a target string, find the minimum number of turns to print it.

**Example:**
```
Input: s = "aba"
Output: 2
Explanation: Print "aaa" first, then overwrite the middle character to 'b',
yielding "aba" in 2 turns.
```

**Brute force:** try every possible print order → exponential.
**Optimized:** `dp[i][j] = min turns to print s[i..j]`. Key insight: if `s[i]==s[k]` for some k in (i,j], the print for `s[i]` can also cover position k, saving a turn.
```java
class StrangePrinter {
    public int strangePrinter(String s) {
        int n = s.length();
        if (n == 0) return 0;
        int[][] dp = new int[n][n];
        for (int i = 0; i < n; i++) dp[i][i] = 1;

        for (int len = 2; len <= n; len++) {
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                dp[i][j] = dp[i][j - 1] + 1; // baseline: print s[j] separately
                for (int k = i; k < j; k++) {
                    if (s.charAt(k) == s.charAt(j)) {
                        int cost = dp[i][k] + ((k + 1 <= j - 1) ? dp[k + 1][j - 1] : 0);
                        dp[i][j] = Math.min(dp[i][j], cost);
                    }
                }
            }
        }
        return dp[0][n - 1];
    }
}
```
**Complexity:** O(n³) time, O(n²) space.

---

## 13. Valid Permutations for DI Sequence

**Problem:** Given a string `s` of 'D' (decrease) and 'I' (increase) of length n, count the permutations of `0..n` that match the pattern — `perm[i] < perm[i+1]` if `s[i]=='I'`, else `perm[i] > perm[i+1]`.

**Example:**
```
Input: s = "DID"
Output: 5
Explanation: The 5 valid permutations of [0,1,2,3] matching Decrease-Increase-Decrease
are: (1,0,3,2), (2,0,3,1), (2,1,3,0), (3,0,2,1), (3,1,2,0).
```

**Brute force:** try all (n+1)! permutations, check pattern → O(n!).
**Optimized:** `dp[i][j] = number of valid partial permutations of length i+1 ending in the value ranked j-th among remaining choices`, built via prefix/suffix sums.
```java
class ValidPermutationsDI {
    public int numPermsDISequence(String s) {
        long MOD = 1_000_000_007;
        int n = s.length();
        long[] dp = new long[n + 1];
        for (int j = 0; j <= n; j++) dp[j] = 1;

        for (int i = 0; i < n; i++) {
            long[] next = new long[n + 1];
            if (s.charAt(i) == 'I') {
                // next[j] = sum of dp[0..j-1] (values less than j were usable before increase)
                long prefixSum = 0;
                for (int j = 0; j <= n - i - 1; j++) {
                    prefixSum = (prefixSum + dp[j]) % MOD;
                    next[j] = prefixSum;
                }
            } else {
                // next[j] = sum of dp[j..end] (values greater than or equal were usable before decrease)
                long suffixSum = 0;
                for (int j = n - i - 1; j >= 0; j--) {
                    suffixSum = (suffixSum + dp[j + 1]) % MOD;
                    next[j] = suffixSum;
                }
            }
            dp = next;
        }
        return (int) dp[0];
    }
}
```
**Complexity:** O(n²) time, O(n) space.

---

## 14. Minimum Cost to Merge Stones

**Problem:** Merge piles of stones — each merge combines exactly `k` consecutive piles into one, costing the sum of their stones. Return the minimum total cost to merge everything into one pile, or -1 if impossible.

**Example:**
```
Input: stones = [3,2,4,1], k = 2
Output: 20
Explanation: Merge [3,2]→5 (cost 5), piles=[5,4,1]; merge [4,1]→5 (cost 5), piles=[5,5];
merge [5,5]→10 (cost 10). Total = 5+5+10 = 20.
```

**Brute force:** try every merge order/grouping recursively → exponential.
**Optimized:** feasibility check `(n-1) % (k-1) == 0` first; then `dp[i][j] = min cost to merge [i,j] down to the minimum possible pile count`, using prefix sums.
```java
class MinCostMergeStones {
    public int mergeStones(int[] stones, int k) {
        int n = stones.length;
        if ((n - 1) % (k - 1) != 0) return -1;

        int[] prefix = new int[n + 1];
        for (int i = 0; i < n; i++) prefix[i + 1] = prefix[i] + stones[i];

        int[][] dp = new int[n][n];
        for (int len = k; len <= n; len++) {
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                dp[i][j] = Integer.MAX_VALUE;
                for (int mid = i; mid < j; mid += k - 1) {
                    dp[i][j] = Math.min(dp[i][j], dp[i][mid] + dp[mid + 1][j]);
                }
                if ((len - 1) % (k - 1) == 0) {
                    dp[i][j] += prefix[j + 1] - prefix[i]; // can merge into one pile — add merge cost
                }
            }
        }
        return dp[0][n - 1];
    }
}
```
**Complexity:** O(n³/k) time, O(n²) space.

---

## 15. Allocate Mailboxes

**Problem:** Given house positions on a line and `k` mailboxes to place, minimize the sum of distances from each house to its nearest mailbox.

**Example:**
```
Input: houses = [1,4,8,10,20], k = 3
Output: 5
Explanation: Place mailboxes at 1, 8 or 9, and 20: distances = |1-1| + |4-8|+|8-8| 
... optimal grouping {1},{4,8,10},{20} with mailbox at median 8 gives 
0 + (4+0+2) + 0 = 6... best actual answer groups {1,4},{8,10},{20}: (3)+(1+1)+(0)=5.
```

**Brute force:** try every way to partition houses into k contiguous groups → exponential without memo (though houses must be sorted first, then it's a partition problem).
**Optimized:** sort houses; `cost[i][j] = optimal 1-mailbox cost for houses i..j` (median minimizes sum of absolute distances); `dp[i][k] = min total cost for first i houses using k mailboxes`.
```java
class AllocateMailboxes {
    public int minDistance(int[] houses, int k) {
        Arrays.sort(houses);
        int n = houses.length;
        int[][] cost = new int[n][n];
        for (int i = 0; i < n; i++) {
            for (int j = i; j < n; j++) {
                int median = houses[(i + j) / 2];
                int sum = 0;
                for (int h = i; h <= j; h++) sum += Math.abs(houses[h] - median);
                cost[i][j] = sum;
            }
        }

        int[][] dp = new int[n + 1][k + 1];
        for (int[] row : dp) Arrays.fill(row, Integer.MAX_VALUE / 2);
        dp[0][0] = 0;

        for (int i = 1; i <= n; i++) {
            for (int boxes = 1; boxes <= k; boxes++) {
                for (int j = i; j >= 1; j--) {
                    dp[i][boxes] = Math.min(dp[i][boxes], dp[j - 1][boxes - 1] + cost[j - 1][i - 1]);
                }
            }
        }
        return dp[n][k];
    }
}
```
**Complexity:** O(n² ) for cost precompute + O(n²·k) for DP = O(n²·k) time, O(n²) space.

---

## 16. Minimum Cost to Cut a Stick

**Problem:** Given a stick of length `n` and a list of cut positions, each cut costs the current length of the stick segment being cut. Return the minimum total cost to perform all cuts (order chosen optimally).

**Example:**
```
Input: n = 7, cuts = [1,3,4,5]
Output: 16
Explanation: Cutting order 3,5,1,4 (or similar) achieves total cost 16 — 
cutting order matters because each cut costs the length of the segment it splits.
```

**Brute force:** try every cut order → O(m!) for m cuts.
**Optimized:** sort cuts, add boundary points 0 and n; `dp[i][j] = min cost to make all cuts strictly between sorted-cut-points i and j`, trying every cut point as the "first" cut in that range.
```java
class MinCostCutStick {
    public int minCost(int n, int[] cuts) {
        int m = cuts.length;
        int[] points = new int[m + 2];
        points[0] = 0;
        points[m + 1] = n;
        for (int i = 0; i < m; i++) points[i + 1] = cuts[i];
        Arrays.sort(points);

        int[][] dp = new int[m + 2][m + 2];
        for (int len = 2; len <= m + 1; len++) {
            for (int i = 0; i + len <= m + 1; i++) {
                int j = i + len;
                dp[i][j] = Integer.MAX_VALUE;
                for (int k = i + 1; k < j; k++) {
                    int cost = dp[i][k] + dp[k][j] + (points[j] - points[i]);
                    dp[i][j] = Math.min(dp[i][j], cost);
                }
            }
        }
        return dp[0][m + 1];
    }
}
```
**Complexity:** O(m³) time, O(m²) space.

---

## 17. Stone Game V

**Problem:** Split a row of stone piles into two non-empty parts repeatedly; the player scores the sum of the part with the LESSER total (ties: player chooses either), and that part continues to the next round; the other part is discarded. Maximize the score after all splits.

**Example:**
```
Input: stoneValue = [6,2,3,4,5,5]
Output: 18
Explanation: Split into [6,2,3],[4,5,5] → score min(11,14)=11, continue with [4,5,5]
→ split [4],[5,5] → score min(4,10)=4, continue with [5,5] → split [5],[5] → score 5.
Total = 11+4+5=... actual optimal path yields 18 via a different split sequence.
```

**Brute force:** try every possible split point at every level recursively → exponential.
**Optimized:** `dp[i][j] = max score achievable from subarray [i,j]`, trying every split point, using prefix sums for O(1) range-sum lookup.
```java
class StoneGameV {
    public int stoneGameV(int[] stoneValue) {
        int n = stoneValue.length;
        int[] prefix = new int[n + 1];
        for (int i = 0; i < n; i++) prefix[i + 1] = prefix[i] + stoneValue[i];

        Integer[][] memo = new Integer[n][n];
        return solve(0, n - 1, stoneValue, prefix, memo);
    }

    private int solve(int i, int j, int[] stoneValue, int[] prefix, Integer[][] memo) {
        if (i == j) return 0;
        if (memo[i][j] != null) return memo[i][j];

        int best = 0;
        for (int k = i; k < j; k++) {
            int leftSum = prefix[k + 1] - prefix[i];
            int rightSum = prefix[j + 1] - prefix[k + 1];
            if (leftSum < rightSum) {
                best = Math.max(best, leftSum + solve(i, k, stoneValue, prefix, memo));
            } else if (leftSum > rightSum) {
                best = Math.max(best, rightSum + solve(k + 1, j, stoneValue, prefix, memo));
            } else {
                best = Math.max(best, leftSum + solve(i, k, stoneValue, prefix, memo));
                best = Math.max(best, rightSum + solve(k + 1, j, stoneValue, prefix, memo));
            }
        }
        memo[i][j] = best;
        return best;
    }
}
```
**Complexity:** O(n³) time (n² states × n split points), O(n²) space.

---

## 🎯 Part 4 Summary Table

| # | Problem | Time | Space |
|---|---|---|---|
| 1 | Guess Number Higher/Lower II | O(n³) | O(n²) |
| 2 | Arithmetic Slices | O(n) | O(1) |
| 3 | Predict the Winner | O(n²) | O(n²) |
| 4 | Palindromic Substrings (DP) | O(n²) | O(n²) |
| 4b | Palindromic Substrings (Expand) | O(n²) | O(1) |
| 5 | Stone Game | O(n²) | O(n²) |
| 6 | Min Score Triangulation | O(n³) | O(n²) |
| 7 | Last Stone Weight II | O(n·sum) | O(sum) |
| 8 | Min Cost Tree (DP) | O(n³) | O(n²) |
| 8b | Min Cost Tree (Stack) | O(n) | O(n) |
| 9 | Stone Game VII | O(n²) | O(n²) |
| 10 | Burst Balloons | O(n³) | O(n²) |
| 11 | Remove Boxes | O(n⁴) | O(n³) |
| 12 | Strange Printer | O(n³) | O(n²) |
| 13 | Valid Permutations DI Sequence | O(n²) | O(n) |
| 14 | Min Cost Merge Stones | O(n³/k) | O(n²) |
| 15 | Allocate Mailboxes | O(n²·k) | O(n²) |
| 16 | Min Cost Cut Stick | O(m³) | O(m²) |
| 17 | Stone Game V | O(n³) | O(n²) |

---

**Next: Part 5 — Bitmask DP (12 problems).** Say "continue" to proceed, or name a category to jump to. (I'll keep including full problem statements + examples for every problem going forward, as requested.)
