# DP Solutions — Part 3: Multi-Dimension DP (Java)
### 29 Problems · Brute Force Note + Optimized Approach + Complexity

---

## 1. Triangle
**Brute force:** try both paths recursively at each row, no memo → O(2ⁿ).
**Optimized:** bottom-up DP, `dp[j] = triangle[row][j] + min(dp[j], dp[j+1])`.
```java
class Triangle {
    public int minimumTotal(List<List<Integer>> triangle) {
        int n = triangle.size();
        int[] dp = new int[n + 1];
        for (int row = n - 1; row >= 0; row--) {
            for (int j = 0; j <= row; j++) {
                dp[j] = triangle.get(row).get(j) + Math.min(dp[j], dp[j + 1]);
            }
        }
        return dp[0];
    }
}
```
**Complexity:** O(n²) time, O(n) space.

---

## 2. Combination Sum IV
**Brute force:** DFS trying every number at each step, no memo → exponential.
**Optimized:** `dp[i] = Σ dp[i-num]` for each num — order-sensitive (permutations), so num loop is INNER.
```java
class CombinationSumIV {
    public int combinationSum4(int[] nums, int target) {
        int[] dp = new int[target + 1];
        dp[0] = 1;
        for (int t = 1; t <= target; t++) {
            for (int num : nums) {
                if (num <= t) dp[t] += dp[t - num];
            }
        }
        return dp[target];
    }
}
```
**Complexity:** O(target · n) time, O(target) space.

---

## 3. Out of Boundary Paths
**Optimized:** `dp[move][r][c] = Σ dp[move-1][neighbor]`, count out-of-bound transitions.
```java
class OutOfBoundaryPaths {
    public int findPaths(int m, int n, int maxMove, int startRow, int startColumn) {
        long MOD = 1_000_000_007;
        long[][] dp = new long[m][n];
        dp[startRow][startColumn] = 1;
        long result = 0;
        int[][] dirs = {{1,0},{-1,0},{0,1},{0,-1}};

        for (int move = 0; move < maxMove; move++) {
            long[][] next = new long[m][n];
            for (int r = 0; r < m; r++) {
                for (int c = 0; c < n; c++) {
                    if (dp[r][c] == 0) continue;
                    for (int[] d : dirs) {
                        int nr = r + d[0], nc = c + d[1];
                        if (nr < 0 || nr >= m || nc < 0 || nc >= n) {
                            result = (result + dp[r][c]) % MOD;
                        } else {
                            next[nr][nc] = (next[nr][nc] + dp[r][c]) % MOD;
                        }
                    }
                }
            }
            dp = next;
        }
        return (int) result;
    }
}
```
**Complexity:** O(maxMove · m · n) time, O(m·n) space.

---

## 4. Knight Probability in Chessboard
**Optimized:** `dp[k][r][c] = probability of being at (r,c) after k moves`, accumulate forward.
```java
class KnightProbability {
    private static final int[][] MOVES = {{1,2},{2,1},{-1,2},{-2,1},{1,-2},{2,-1},{-1,-2},{-2,-1}};

    public double knightProbability(int n, int k, int row, int column) {
        double[][] dp = new double[n][n];
        dp[row][column] = 1.0;

        for (int move = 0; move < k; move++) {
            double[][] next = new double[n][n];
            for (int r = 0; r < n; r++) {
                for (int c = 0; c < n; c++) {
                    if (dp[r][c] == 0) continue;
                    for (int[] mv : MOVES) {
                        int nr = r + mv[0], nc = c + mv[1];
                        if (nr >= 0 && nr < n && nc >= 0 && nc < n) {
                            next[nr][nc] += dp[r][c] / 8.0;
                        }
                    }
                }
            }
            dp = next;
        }

        double total = 0;
        for (double[] row_ : dp) for (double v : row_) total += v;
        return total;
    }
}
```
**Complexity:** O(k · n²) time, O(n²) space.

---

## 5. Champagne Tower
**Optimized:** simulate pour amounts row by row, `next[c] += (dp[c]-1)/2`, `next[c+1] += (dp[c]-1)/2`.
```java
class ChampagneTower {
    public double champagneTower(int poured, int queryRow, int queryGlass) {
        double[] dp = new double[queryRow + 2];
        dp[0] = poured;

        for (int row = 0; row < queryRow; row++) {
            double[] next = new double[queryRow + 2];
            for (int c = 0; c <= row; c++) {
                double excess = (dp[c] - 1.0) / 2.0;
                if (excess > 0) {
                    next[c] += excess;
                    next[c + 1] += excess;
                }
            }
            dp = next;
        }
        return Math.min(1.0, dp[queryGlass]);
    }
}
```
**Complexity:** O(queryRow²) time, O(queryRow) space.

---

## 6. Largest Sum of Averages
**Brute force:** try every possible partition into k groups → exponential.
**Optimized:** `dp[i][k] = max average sum splitting first i elements into k groups`, using prefix sums.
```java
class LargestSumOfAverages {
    public double largestSumOfAverages(int[] nums, int k) {
        int n = nums.length;
        double[] prefix = new double[n + 1];
        for (int i = 0; i < n; i++) prefix[i + 1] = prefix[i] + nums[i];

        double[][] dp = new double[n + 1][k + 1];
        for (int i = 1; i <= n; i++) dp[i][1] = prefix[i] / i;

        for (int groups = 2; groups <= k; groups++) {
            for (int i = groups; i <= n; i++) {
                for (int j = groups - 1; j < i; j++) {
                    double avg = (prefix[i] - prefix[j]) / (i - j);
                    dp[i][groups] = Math.max(dp[i][groups], dp[j][groups - 1] + avg);
                }
            }
        }
        return dp[n][k];
    }
}
```
**Complexity:** O(n² · k) time, O(n·k) space.

---

## 7. Minimum Falling Path Sum
**Optimized:** `dp[c] = matrix[row][c] + min(dp[c-1], dp[c], dp[c+1])`.
```java
class MinFallingPathSum {
    public int minFallingPathSum(int[][] matrix) {
        int n = matrix.length;
        int[] dp = matrix[0].clone();

        for (int row = 1; row < n; row++) {
            int[] next = new int[n];
            for (int c = 0; c < n; c++) {
                int best = dp[c];
                if (c > 0) best = Math.min(best, dp[c - 1]);
                if (c < n - 1) best = Math.min(best, dp[c + 1]);
                next[c] = matrix[row][c] + best;
            }
            dp = next;
        }
        return Arrays.stream(dp).min().getAsInt();
    }
}
```
**Complexity:** O(n²) time, O(n) space.

---

## 8. Video Stitching
**Brute force:** try all subsets of clips → O(2ⁿ).
**Optimized:** greedy — same "max reach" jump-game pattern as Min Taps.
```java
class VideoStitching {
    public int videoStitching(int[][] clips, int time) {
        int[] maxReach = new int[time + 1];
        for (int[] clip : clips) {
            if (clip[0] <= time) maxReach[clip[0]] = Math.max(maxReach[clip[0]], clip[1]);
        }

        int count = 0, currEnd = 0, farthest = 0;
        for (int i = 0; i < time; i++) {
            farthest = Math.max(farthest, maxReach[i]);
            if (i == currEnd) {
                if (farthest <= i) return -1;
                count++;
                currEnd = farthest;
            }
        }
        return count;
    }
}
```
**Complexity:** O(n + time) time, O(time) space.

---

## 9. Longest Arithmetic Subsequence
**Brute force:** check all pairs/triples of indices for arithmetic runs → O(n³).
**Optimized:** `dp[i][diff] = length of longest AP ending at i with common difference diff`.
```java
class LongestArithmeticSubsequence {
    public int longestArithSeqLength(int[] nums) {
        int n = nums.length;
        Map<Integer, Integer>[] dp = new HashMap[n];
        int maxLen = 1;

        for (int i = 0; i < n; i++) {
            dp[i] = new HashMap<>();
            for (int j = 0; j < i; j++) {
                int diff = nums[i] - nums[j];
                int len = dp[j].getOrDefault(diff, 1) + 1;
                dp[i].put(diff, Math.max(dp[i].getOrDefault(diff, 0), len));
                maxLen = Math.max(maxLen, len);
            }
        }
        return maxLen;
    }
}
```
**Complexity:** O(n²) time, O(n²) space (beats O(n³) brute force).

---

## 10. Stone Game II
**Optimized:** `dp[i][M] = max stones current player can get from index i onward with parameter M`.
```java
class StoneGameII {
    public int stoneGameII(int[] piles) {
        int n = piles.length;
        int[] suffixSum = new int[n + 1];
        for (int i = n - 1; i >= 0; i--) suffixSum[i] = suffixSum[i + 1] + piles[i];

        Integer[][] memo = new Integer[n][n + 1];
        return solve(0, 1, piles, suffixSum, memo);
    }

    private int solve(int i, int M, int[] piles, int[] suffixSum, Integer[][] memo) {
        int n = piles.length;
        if (i >= n) return 0;
        if (2 * M >= n - i) return suffixSum[i];
        if (memo[i][M] != null) return memo[i][M];

        int best = 0;
        for (int x = 1; x <= 2 * M; x++) {
            if (i + x > n) break;
            best = Math.max(best, suffixSum[i] - solve(i + x, Math.max(M, x), piles, suffixSum, memo));
        }
        memo[i][M] = best;
        return best;
    }
}
```
**Complexity:** O(n³) time (n positions × n M-values × up to n choices), O(n²) space.

---

## 11. Number of Dice Rolls with Target Sum
**Brute force:** DFS trying every face for every die, no memo → O(faces^dice).
**Optimized:** `dp[d][t] = Σ dp[d-1][t-face]` for each face.
```java
class NumberOfDiceRollsWithTargetSum {
    public int numRollsToTarget(int n, int k, int target) {
        long MOD = 1_000_000_007;
        long[] dp = new long[target + 1];
        dp[0] = 1;

        for (int die = 1; die <= n; die++) {
            long[] next = new long[target + 1];
            for (int t = 1; t <= target; t++) {
                for (int face = 1; face <= k && face <= t; face++) {
                    next[t] = (next[t] + dp[t - face]) % MOD;
                }
            }
            dp = next;
        }
        return (int) dp[target];
    }
}
```
**Complexity:** O(n · target · k) time, O(target) space.

---

## 12. Dice Roll Simulation
**Optimized:** `dp[i][num][count] = ways to reach roll i, last number num, count consecutive`.
```java
class DiceRollSimulation {
    public int dieSimulator(int n, int[] rollMax) {
        long MOD = 1_000_000_007;
        long[][] dp = new long[6][16]; // dp[num][streak]
        for (int num = 0; num < 6; num++) dp[num][1] = 1;

        for (int roll = 2; roll <= n; roll++) {
            long[][] next = new long[6][16];
            for (int num = 0; num < 6; num++) {
                for (int streak = 1; streak <= rollMax[num]; streak++) {
                    if (dp[num][streak] == 0) continue;
                    for (int newNum = 0; newNum < 6; newNum++) {
                        if (newNum == num) {
                            if (streak + 1 <= rollMax[num]) {
                                next[newNum][streak + 1] = (next[newNum][streak + 1] + dp[num][streak]) % MOD;
                            }
                        } else {
                            next[newNum][1] = (next[newNum][1] + dp[num][streak]) % MOD;
                        }
                    }
                }
            }
            dp = next;
        }

        long total = 0;
        for (long[] row : dp) for (long v : row) total = (total + v) % MOD;
        return (int) total;
    }
}
```
**Complexity:** O(n · 6 · 15 · 6) ≈ O(n) time, O(1) extra space (fixed-size state).

---

## 13. Number of Sets of K Non-overlapping Line Segments
**Optimized:** `dp[i][k][used]` — position, segments placed, currently inside a segment or not.
```java
class NumberOfSetsOfKSegments {
    public int numberOfSets(int n, int k) {
        long MOD = 1_000_000_007;
        // dp[i][j] = ways to choose j segments using first i points, dpUsed for "extend current segment"
        long[][] dp = new long[n][k + 1];
        long[][] dpUsed = new long[n][k + 1]; // ending exactly at i, segment ongoing

        for (int j = 0; j <= k; j++) dp[0][j] = (j == 0) ? 1 : 0;
        dpUsed[0][0] = 1;

        for (int i = 1; i < n; i++) {
            dp[i][0] = 1;
            for (int j = 1; j <= k; j++) {
                dpUsed[i][j] = (dpUsed[i - 1][j] + dp[i - 1][j - 1]) % MOD;
                dp[i][j] = (dp[i - 1][j] + dpUsed[i][j]) % MOD;
            }
        }
        return (int) dp[n - 1][k];
    }
}
```
**Complexity:** O(n·k) time, O(n·k) space.

---

## 14. Best Time to Buy and Sell Stock IV
**Brute force:** try all combinations of up to k buy/sell pairs → exponential.
**Optimized:** `dp[k][day]` state DP generalizing Stock III's 4-state approach to k transactions.
```java
class BuySellStockIV {
    public int maxProfit(int k, int[] prices) {
        if (prices.length == 0) return 0;
        int[] buy = new int[k + 1];
        int[] sell = new int[k + 1];
        Arrays.fill(buy, Integer.MIN_VALUE);

        for (int price : prices) {
            for (int t = 1; t <= k; t++) {
                buy[t] = Math.max(buy[t], sell[t - 1] - price);
                sell[t] = Math.max(sell[t], buy[t] + price);
            }
        }
        return sell[k];
    }
}
```
**Complexity:** O(n · k) time, O(k) space.

---

## 15. Create Maximum Number
**Problem:** From two arrays, pick k digits total (preserving relative order within each array) forming the largest number.
**Optimized:** for each split (i from array1, k-i from array2): get max subsequence of length i from array1 (monotonic stack), same for array2, merge greedily, keep best overall.
```java
class CreateMaximumNumber {
    public int[] maxNumber(int[] nums1, int[] nums2, int k) {
        int[] best = new int[k];
        for (int i = Math.max(0, k - nums2.length); i <= Math.min(k, nums1.length); i++) {
            int[] candidate1 = maxSubsequence(nums1, i);
            int[] candidate2 = maxSubsequence(nums2, k - i);
            int[] merged = merge(candidate1, candidate2);
            if (greater(merged, 0, best, 0)) best = merged;
        }
        return best;
    }

    private int[] maxSubsequence(int[] nums, int k) {
        int[] stack = new int[k];
        int top = -1;
        int toRemove = nums.length - k;
        for (int num : nums) {
            while (top >= 0 && stack[top] < num && toRemove > 0) { top--; toRemove--; }
            if (top < k - 1) stack[++top] = num;
            else toRemove--;
        }
        return stack;
    }

    private int[] merge(int[] a, int[] b) {
        int[] result = new int[a.length + b.length];
        int i = 0, j = 0, idx = 0;
        while (i < a.length || j < b.length) {
            if (greater(a, i, b, j)) result[idx++] = a[i++];
            else result[idx++] = b[j++];
        }
        return result;
    }

    private boolean greater(int[] a, int i, int[] b, int j) {
        while (i < a.length && j < b.length && a[i] == b[j]) { i++; j++; }
        return j == b.length || (i < a.length && a[i] > b[j]);
    }
}
```
**Complexity:** O((n+m)³) time (k splits × O((n+m)) merge × O(n+m) comparisons worst case), O(n+m) space. A hard problem combining greedy monotonic-stack subsequence extraction with merge logic.

---

## 16. Frog Jump
**Brute force:** DFS trying jump-1/jump/jump+1 at every stone, no memo → exponential.
**Optimized:** `dp[stone] = set of jump sizes that can reach this stone`.
```java
class FrogJump {
    public boolean canCross(int[] stones) {
        Map<Integer, Set<Integer>> dp = new HashMap<>();
        for (int stone : stones) dp.put(stone, new HashSet<>());
        dp.get(stones[0]).add(0);

        for (int stone : stones) {
            for (int jump : dp.get(stone)) {
                for (int next = jump - 1; next <= jump + 1; next++) {
                    if (next > 0 && dp.containsKey(stone + next)) {
                        dp.get(stone + next).add(next);
                    }
                }
            }
        }
        return !dp.get(stones[stones.length - 1]).isEmpty();
    }
}
```
**Complexity:** O(n²) time (n stones × up to n jump sizes each), O(n²) space.

---

## 17. Split Array Largest Sum
**Brute force:** try every split point combination recursively → exponential.
**Optimized Solution 1 — DP:** `dp[i][k] = min largest-subarray-sum splitting first i elements into k parts`.
```java
class SplitArrayLargestSumDP {
    public int splitArray(int[] nums, int k) {
        int n = nums.length;
        int[] prefix = new int[n + 1];
        for (int i = 0; i < n; i++) prefix[i + 1] = prefix[i] + nums[i];

        int[][] dp = new int[n + 1][k + 1];
        for (int[] row : dp) Arrays.fill(row, Integer.MAX_VALUE);
        dp[0][0] = 0;

        for (int i = 1; i <= n; i++) {
            for (int parts = 1; parts <= Math.min(i, k); parts++) {
                for (int j = parts - 1; j < i; j++) {
                    if (dp[j][parts - 1] == Integer.MAX_VALUE) continue;
                    int subSum = prefix[i] - prefix[j];
                    dp[i][parts] = Math.min(dp[i][parts], Math.max(dp[j][parts - 1], subSum));
                }
            }
        }
        return dp[n][k];
    }
}
```
**Complexity:** O(n² · k) time, O(n·k) space.

### Optimized Solution 2 — Binary Search on Answer (better complexity)
```java
class SplitArrayLargestSumBS {
    public int splitArray(int[] nums, int k) {
        int lo = Arrays.stream(nums).max().getAsInt();
        int hi = Arrays.stream(nums).sum();

        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (canSplit(nums, k, mid)) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

    private boolean canSplit(int[] nums, int k, int maxSum) {
        int parts = 1, currSum = 0;
        for (int num : nums) {
            if (currSum + num > maxSum) { parts++; currSum = 0; }
            currSum += num;
        }
        return parts <= k;
    }
}
```
**Complexity:** O(n log(sum)) time, O(1) space — significantly better than the O(n²k) DP.

---

## 18. Freedom Trail
**Optimized:** `dp[i][pos] = min steps to spell first i chars of key, ring pointer at pos`.
```java
class FreedomTrail {
    public int findRotateSteps(String ring, String key) {
        int n = ring.length(), m = key.length();
        Map<Character, List<Integer>> charPositions = new HashMap<>();
        for (int i = 0; i < n; i++) {
            charPositions.computeIfAbsent(ring.charAt(i), c -> new ArrayList<>()).add(i);
        }

        int[] dp = new int[n];
        dp[0] = 0; // ring pointer starts at 0
        for (int i = 1; i < n; i++) dp[i] = Integer.MAX_VALUE / 2;

        for (int i = 0; i < m; i++) {
            int[] next = new int[n];
            Arrays.fill(next, Integer.MAX_VALUE / 2);
            for (int pos : charPositions.get(key.charAt(i))) {
                for (int prevPos = 0; prevPos < n; prevPos++) {
                    if (dp[prevPos] == Integer.MAX_VALUE / 2) continue;
                    int dist = Math.min(Math.abs(pos - prevPos), n - Math.abs(pos - prevPos));
                    next[pos] = Math.min(next[pos], dp[prevPos] + dist + 1);
                }
            }
            dp = next;
        }
        return Arrays.stream(dp).min().getAsInt();
    }
}
```
**Complexity:** O(m · n²) time (can be optimized to O(m·n·occurrences) by only iterating relevant prev positions), O(n) space.

---

## 19. Minimum Number of Refueling Stops
**Brute force:** try every subset of stations to stop at → O(2ⁿ).
**Optimized:** `dp[stops] = max distance reachable using exactly `stops` refuels` — greedy-DP hybrid.
```java
class MinRefuelingStops {
    public int minRefuelStops(int target, int startFuel, int[][] stations) {
        int n = stations.length;
        long[] dp = new long[n + 1];
        dp[0] = startFuel;

        for (int i = 0; i < n; i++) {
            for (int stops = i; stops >= 0; stops--) {
                if (dp[stops] >= stations[i][0]) {
                    dp[stops + 1] = Math.max(dp[stops + 1], dp[stops] + stations[i][1]);
                }
            }
        }

        for (int stops = 0; stops <= n; stops++) {
            if (dp[stops] >= target) return stops;
        }
        return -1;
    }
}
```
**Complexity:** O(n²) time, O(n) space. (An O(n log n) greedy-with-max-heap alternative also exists: pass stations, push fuel into a heap, pop largest fuel amounts as needed when stuck.)

---

## 20. Number of Music Playlists
**Optimized:** `dp[i][j] = ways to build playlist of length i using j unique songs`.
```java
class NumMusicPlaylists {
    public int numMusicPlaylists(int n, int goal, int k) {
        long MOD = 1_000_000_007;
        long[][] dp = new long[goal + 1][n + 1];
        dp[0][0] = 1;

        for (int i = 1; i <= goal; i++) {
            for (int j = 1; j <= n; j++) {
                // add a new song
                dp[i][j] = (dp[i - 1][j - 1] * (n - j + 1)) % MOD;
                // replay an old song (must wait k songs)
                if (j > k) {
                    dp[i][j] = (dp[i][j] + dp[i - 1][j] * (j - k)) % MOD;
                }
            }
        }
        return (int) dp[goal][n];
    }
}
```
**Complexity:** O(goal · n) time, O(goal · n) space.

---

## 21. Count Vowels Permutation
**Optimized:** `dp[i][vowel] = count of valid strings of length i ending in `vowel``, per allowed-transition rules.
```java
class CountVowelsPermutation {
    public int countVowelPermutation(int n) {
        long MOD = 1_000_000_007;
        long a = 1, e = 1, i = 1, o = 1, u = 1; // a,e,i,o,u counts

        for (int len = 1; len < n; len++) {
            long na = (e + i + u) % MOD;
            long ne = (a + i) % MOD;
            long ni = (e + o) % MOD;
            long no = i;
            long nu = (i + o) % MOD;
            a = na; e = ne; i = ni; o = no; u = nu;
        }
        return (int) ((a + e + i + o + u) % MOD);
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 22. Minimum Falling Path Sum II
**Optimized:** track top-2 minimum values per row (avoiding same-column reuse) instead of scanning all columns each time.
```java
class MinFallingPathSumII {
    public int minFallingPathSum(int[][] grid) {
        int n = grid.length;
        int[] dp = grid[0].clone();

        for (int row = 1; row < n; row++) {
            int min1 = Integer.MAX_VALUE, min1Idx = -1, min2 = Integer.MAX_VALUE;
            for (int c = 0; c < n; c++) {
                if (dp[c] < min1) { min2 = min1; min1 = dp[c]; min1Idx = c; }
                else if (dp[c] < min2) { min2 = dp[c]; }
            }
            int[] next = new int[n];
            for (int c = 0; c < n; c++) {
                next[c] = grid[row][c] + (c == min1Idx ? min2 : min1);
            }
            dp = next;
        }
        return Arrays.stream(dp).min().getAsInt();
    }
}
```
**Complexity:** O(n²) time (still scan each row, but O(n) per row instead of O(n) neighbors-only — this is actually the SAME order as the naive 3-neighbor version, but here every column is a valid "non-same" transition so we need the running top-2 trick to stay O(n) per row instead of O(n²) per row). O(n) space.

---

## 23. Minimum Distance to Type a Word Using Two Fingers
**Optimized:** `dp[otherFingerPos] = min distance`, one finger implicitly at the last-typed character.
```java
class MinDistanceTwoFingers {
    public int minimumDistance(String word) {
        int n = word.length();
        int[][] dp = new int[27][27]; // dp[f1][f2], 26 = "unused" finger
        for (int[] row : dp) Arrays.fill(row, -1);
        dp[26][26] = 0;

        Map<Long, Integer> memo = new HashMap<>();
        return solve(word, 0, 26, 26, memo);
    }

    private int solve(String word, int idx, int f1, int f2, Map<Long, Integer> memo) {
        if (idx == word.length()) return 0;
        long key = ((long) idx * 27 + f1) * 27 + f2;
        if (memo.containsKey(key)) return memo.get(key);

        int target = word.charAt(idx) - 'A';
        int cost1 = (f1 == 26) ? 0 : dist(f1, target);
        int cost2 = (f2 == 26) ? 0 : dist(f2, target);

        int result = Math.min(
            cost1 + solve(word, idx + 1, target, f2, memo),
            cost2 + solve(word, idx + 1, f1, target, memo)
        );
        memo.put(key, result);
        return result;
    }

    private int dist(int a, int b) {
        int r1 = a / 6, c1 = a % 6, r2 = b / 6, c2 = b % 6;
        return Math.abs(r1 - r2) + Math.abs(c1 - c2);
    }
}
```
**Complexity:** O(n · 26²) time, O(n · 26²) space.

---

## 24. Minimum Difficulty of a Job Schedule
**Brute force:** try every way to partition jobs into d days → exponential.
**Optimized:** `dp[i][d] = min difficulty scheduling first i jobs into d days`.
```java
class MinDifficultyJobSchedule {
    public int minDifficulty(int[] jobDifficulty, int d) {
        int n = jobDifficulty.length;
        if (n < d) return -1;

        int[][] dp = new int[n + 1][d + 1];
        for (int[] row : dp) Arrays.fill(row, Integer.MAX_VALUE / 2);
        dp[0][0] = 0;

        for (int i = 1; i <= n; i++) {
            for (int day = 1; day <= Math.min(i, d); day++) {
                int maxDiff = 0;
                for (int j = i; j >= day; j--) {
                    maxDiff = Math.max(maxDiff, jobDifficulty[j - 1]);
                    dp[i][day] = Math.min(dp[i][day], dp[j - 1][day - 1] + maxDiff);
                }
            }
        }
        return dp[n][d];
    }
}
```
**Complexity:** O(n² · d) time, O(n·d) space.

---

## 25. Number of Ways to Paint N x 3 Grid
**Optimized:** `dp[pattern]` for the 12 valid row colorings (3 colors, no adjacent same), transition via compatibility check.
```java
class NumWaysPaintNx3Grid {
    public int numOfWays(int n) {
        long MOD = 1_000_000_007;
        long same = 6, diff = 6; // "same" = patterns like ABA, "diff" = patterns like ABC

        for (int row = 1; row < n; row++) {
            long newSame = (same * 3 + diff * 2) % MOD;
            long newDiff = (same * 2 + diff * 2) % MOD;
            same = newSame;
            diff = newDiff;
        }
        return (int) ((same + diff) % MOD);
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 26. Build Array Where You Can Find The Maximum Exactly K Comparisons
**Optimized:** `dp[i][maxVal][k] = ways to build first i elements, running max = maxVal, k search-cost used so far`.
```java
class BuildArrayMaxKComparisons {
    public int numOfArrays(int n, int m, int k) {
        long MOD = 1_000_000_007;
        // dp[i][maxVal][cost]
        long[][][] dp = new long[n + 1][m + 1][k + 1];
        for (int v = 1; v <= m; v++) dp[1][v][1] = 1;

        for (int i = 2; i <= n; i++) {
            for (int maxVal = 1; maxVal <= m; maxVal++) {
                for (int cost = 1; cost <= k; cost++) {
                    // place a value <= maxVal (doesn't change max or cost)
                    dp[i][maxVal][cost] = (dp[i][maxVal][cost] + dp[i - 1][maxVal][cost] * maxVal) % MOD;
                    // place a new max value > previous max (increases cost by 1)
                    for (int prevMax = 1; prevMax < maxVal; prevMax++) {
                        dp[i][maxVal][cost] = (dp[i][maxVal][cost] + dp[i - 1][prevMax][cost - 1]) % MOD;
                    }
                }
            }
        }

        long total = 0;
        for (int v = 1; v <= m; v++) total = (total + dp[n][v][k]) % MOD;
        return (int) total;
    }
}
```
**Complexity:** O(n · m² · k) time, O(n·m·k) space.

---

## 27. Number of Ways of Cutting a Pizza
**Optimized:** `dp[k][r][c] = ways to make k cuts starting from (r,c) to bottom-right`, with 2D prefix sum for O(1) apple-count checks.
```java
class NumWaysCuttingPizza {
    public int ways(String[] pizza, int k) {
        long MOD = 1_000_000_007;
        int rows = pizza.length, cols = pizza[0].length();
        int[][] apples = new int[rows + 1][cols + 1]; // suffix count of apples from (r,c) to bottom-right
        for (int r = rows - 1; r >= 0; r--) {
            for (int c = cols - 1; c >= 0; c--) {
                apples[r][c] = (pizza[r].charAt(c) == 'A' ? 1 : 0)
                    + apples[r + 1][c] + apples[r][c + 1] - apples[r + 1][c + 1];
            }
        }

        Integer[][][] memo = new Integer[k][rows][cols];
        return solve(0, 0, k - 1, rows, cols, apples, memo);
    }

    private int solve(int r, int c, int cutsLeft, int rows, int cols, int[][] apples, Integer[][][] memo) {
        long MOD = 1_000_000_007;
        if (apples[r][c] == 0) return 0; // no apple in remaining piece
        if (cutsLeft == 0) return 1;
        if (memo[cutsLeft][r][c] != null) return memo[cutsLeft][r][c];

        long ways = 0;
        for (int nr = r + 1; nr < rows; nr++) {
            if (apples[r][c] - apples[nr][c] > 0) {
                ways = (ways + solve(nr, c, cutsLeft - 1, rows, cols, apples, memo)) % MOD;
            }
        }
        for (int nc = c + 1; nc < cols; nc++) {
            if (apples[r][c] - apples[r][nc] > 0) {
                ways = (ways + solve(r, nc, cutsLeft - 1, rows, cols, apples, memo)) % MOD;
            }
        }
        memo[cutsLeft][r][c] = (int) ways;
        return (int) ways;
    }
}
```
**Complexity:** O(k · rows · cols · (rows+cols)) time, O(k·rows·cols) space.

---

## 28. Paint House III
**Optimized:** `dp[house][color][neighborhoods] = min cost`.
```java
class PaintHouseIII {
    public int minCost(int[] houses, int[][] cost, int m, int n, int target) {
        int INF = Integer.MAX_VALUE / 2;
        // dp[house][color][groups]
        int[][][] dp = new int[m][n + 1][target + 1];
        for (int[][] a : dp) for (int[] b : a) Arrays.fill(b, INF);

        if (houses[0] != 0) {
            dp[0][houses[0]][1] = 0;
        } else {
            for (int c = 1; c <= n; c++) dp[0][c][1] = cost[0][c - 1];
        }

        for (int h = 1; h < m; h++) {
            for (int c = 1; c <= n; c++) {
                if (houses[h] != 0 && houses[h] != c) continue;
                int paintCost = (houses[h] == 0) ? cost[h][c - 1] : 0;
                for (int g = 1; g <= target; g++) {
                    for (int prevC = 1; prevC <= n; prevC++) {
                        int prevGroups = (prevC == c) ? g : g - 1;
                        if (prevGroups < 1) continue;
                        if (dp[h - 1][prevC][prevGroups] == INF) continue;
                        dp[h][c][g] = Math.min(dp[h][c][g], dp[h - 1][prevC][prevGroups] + paintCost);
                    }
                }
            }
        }

        int result = INF;
        for (int c = 1; c <= n; c++) result = Math.min(result, dp[m - 1][c][target]);
        return result == INF ? -1 : result;
    }
}
```
**Complexity:** O(m · n² · target) time, O(m·n·target) space.

---

## 29. Count All Possible Routes
**Optimized:** `dp[city][fuel] = number of routes`, memoized recursion (fuel monotonically decreases so no cycles).
```java
class CountAllPossibleRoutes {
    public int countRoutes(int[] locations, int start, int finish, int fuel) {
        long MOD = 1_000_000_007;
        int n = locations.length;
        Integer[][] memo = new Integer[n][fuel + 1];
        return (int) solve(locations, start, finish, fuel, memo);
    }

    private long solve(int[] locations, int curr, int finish, int fuel, Integer[][] memo) {
        long MOD = 1_000_000_007;
        if (fuel < 0) return 0;
        if (memo[curr][fuel] != null) return memo[curr][fuel];

        long ways = (curr == finish) ? 1 : 0;
        for (int next = 0; next < locations.length; next++) {
            if (next == curr) continue;
            int cost = Math.abs(locations[curr] - locations[next]);
            if (cost <= fuel) {
                ways = (ways + solve(locations, next, finish, fuel - cost, memo)) % MOD;
            }
        }
        memo[curr][fuel] = (int) ways;
        return ways;
    }
}
```
**Complexity:** O(n² · fuel) time, O(n·fuel) space.

---

## 🎯 Part 3 Summary Table

| # | Problem | Time | Space |
|---|---|---|---|
| 1 | Triangle | O(n²) | O(n) |
| 2 | Combination Sum IV | O(target·n) | O(target) |
| 3 | Out of Boundary Paths | O(moves·m·n) | O(m·n) |
| 4 | Knight Probability | O(k·n²) | O(n²) |
| 5 | Champagne Tower | O(row²) | O(row) |
| 6 | Largest Sum of Averages | O(n²·k) | O(n·k) |
| 7 | Min Falling Path Sum | O(n²) | O(n) |
| 8 | Video Stitching | O(n+time) | O(time) |
| 9 | Longest Arith Subsequence | O(n²) | O(n²) |
| 10 | Stone Game II | O(n³) | O(n²) |
| 11 | Dice Rolls Target Sum | O(n·target·k) | O(target) |
| 12 | Dice Roll Simulation | O(n) | O(1) |
| 13 | K Non-overlapping Segments | O(n·k) | O(n·k) |
| 14 | Buy/Sell Stock IV | O(n·k) | O(k) |
| 15 | Create Maximum Number | O((n+m)³) | O(n+m) |
| 16 | Frog Jump | O(n²) | O(n²) |
| 17 | Split Array Largest Sum (DP) | O(n²·k) | O(n·k) |
| 17b | Split Array Largest Sum (BS) | O(n log sum) | O(1) |
| 18 | Freedom Trail | O(m·n²) | O(n) |
| 19 | Min Refueling Stops | O(n²) | O(n) |
| 20 | Num Music Playlists | O(goal·n) | O(goal·n) |
| 21 | Count Vowels Permutation | O(n) | O(1) |
| 22 | Min Falling Path Sum II | O(n²) | O(n) |
| 23 | Min Distance Two Fingers | O(n·26²) | O(n·26²) |
| 24 | Min Difficulty Job Schedule | O(n²·d) | O(n·d) |
| 25 | Paint N×3 Grid | O(n) | O(1) |
| 26 | Build Array K Comparisons | O(n·m²·k) | O(n·m·k) |
| 27 | Ways of Cutting Pizza | O(k·r·c·(r+c)) | O(k·r·c) |
| 28 | Paint House III | O(m·n²·target) | O(m·n·target) |
| 29 | Count All Possible Routes | O(n²·fuel) | O(n·fuel) |

---

**Next: Part 4 — Interval DP (17 problems).** Say "continue" to proceed, or name a category to jump to.
