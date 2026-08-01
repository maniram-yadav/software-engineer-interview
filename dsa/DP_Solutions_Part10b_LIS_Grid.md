# DP Solutions — Part 10b: Classic DPs (LIS + 2D Grid Traversal) (Java)
### 17 Problems · Full Problem Statement + Example + Brute Force → Optimized + Complexity

---

# Section C: Longest Increasing Subsequence (LIS) Family

## C1. Longest Increasing Subsequence

**Problem:** Find the length of the longest strictly increasing subsequence in an array.

**Example:**
```
Input: nums = [10,9,2,5,3,7,101,18]
Output: 4
Explanation: The LIS is [2,3,7,101] (or [2,3,7,18]), length 4.
```

**Brute force:** try every subsequence, check increasing property → O(2ⁿ).
**Optimized Solution 1 — DP:** `dp[i] = LIS ending at i`.
```java
class LISBasicDP {
    public int lengthOfLIS(int[] nums) {
        int n = nums.length;
        int[] dp = new int[n];
        Arrays.fill(dp, 1);
        int best = 1;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < i; j++) {
                if (nums[j] < nums[i]) dp[i] = Math.max(dp[i], dp[j] + 1);
            }
            best = Math.max(best, dp[i]);
        }
        return best;
    }
}
```
**Complexity:** O(n²) time, O(n) space.

### Optimized Solution 2 — Patience Sorting + Binary Search
```java
class LISBinarySearch {
    public int lengthOfLIS(int[] nums) {
        List<Integer> tails = new ArrayList<>();
        for (int num : nums) {
            int idx = Collections.binarySearch(tails, num);
            if (idx < 0) idx = -(idx + 1);
            if (idx == tails.size()) tails.add(num);
            else tails.set(idx, num);
        }
        return tails.size();
    }
}
```
**Complexity:** O(n log n) time, O(n) space — the far better choice for large inputs.

---

## C2. Number of Longest Increasing Subsequences

**Problem:** Return the number of DISTINCT longest increasing subsequences.

**Example:**
```
Input: nums = [1,3,5,4,7]
Output: 2
Explanation: The two LIS of length 4 are [1,3,4,7] and [1,3,5,7].
```

**Brute force:** enumerate all increasing subsequences, track max length and count → O(2ⁿ).
**Optimized:** `len[i] = LIS length ending at i`, `cnt[i] = number of LIS of that length ending at i`.
```java
class NumberOfLIS {
    public int findNumberOfLIS(int[] nums) {
        int n = nums.length;
        int[] len = new int[n], cnt = new int[n];
        Arrays.fill(len, 1);
        Arrays.fill(cnt, 1);
        int maxLen = 1;

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < i; j++) {
                if (nums[j] < nums[i]) {
                    if (len[j] + 1 > len[i]) { len[i] = len[j] + 1; cnt[i] = cnt[j]; }
                    else if (len[j] + 1 == len[i]) cnt[i] += cnt[j];
                }
            }
            maxLen = Math.max(maxLen, len[i]);
        }

        int result = 0;
        for (int i = 0; i < n; i++) if (len[i] == maxLen) result += cnt[i];
        return result;
    }
}
```
**Complexity:** O(n²) time, O(n) space.

---

## C3. Russian Doll Envelopes

**Problem:** Given envelope dimensions `(w,h)`, one envelope can nest inside another if both its width and height are strictly smaller. Find the maximum number of envelopes that can be nested (forming a chain).

**Example:**
```
Input: envelopes = [[5,4],[6,4],[6,7],[2,3]]
Output: 3
Explanation: [2,3] -> [5,4] -> [6,7] — a chain of 3.
```

**Brute force:** try every ordering/subset of envelopes, check chain validity → O(2ⁿ).
**Optimized:** sort by width ascending (height DESCENDING for tied widths, to prevent same-width envelopes from being counted together), then run LIS on the height sequence.
```java
class RussianDollEnvelopes {
    public int maxEnvelopes(int[][] envelopes) {
        Arrays.sort(envelopes, (a, b) -> a[0] != b[0] ? a[0] - b[0] : b[1] - a[1]);
        List<Integer> tails = new ArrayList<>();
        for (int[] e : envelopes) {
            int h = e[1];
            int idx = Collections.binarySearch(tails, h);
            if (idx < 0) idx = -(idx + 1);
            if (idx == tails.size()) tails.add(h);
            else tails.set(idx, h);
        }
        return tails.size();
    }
}
```
**Complexity:** O(n log n) time (sort + LIS via binary search), O(n) space.

---

## C4. Delete Columns to Make Sorted III

**Problem:** Given an array of equal-length strings, choose a set of column indices to KEEP such that reading each string restricted to those columns (in order) is non-decreasing. Return the minimum number of columns to delete (maximize kept columns).

**Example:**
```
Input: strs = ["babca","bbazb"]
Output: 3
Explanation: Keeping columns {0,1,4} gives "bba" and "bbb" — both non-decreasing 
per-row. That's 2 columns kept... actually keeping 2 columns is optimal here, 
requiring 3 deletions out of 5 total columns.
```

**Brute force:** try every subset of columns to keep, check validity for all rows → O(2^cols · rows · cols).
**Optimized:** this is LIS generalized across MULTIPLE strings simultaneously — `dp[j] = max columns kept ending at column j`, where column `i` can precede column `j` only if `strs[row][i] <= strs[row][j]` for EVERY row.
```java
class DeleteColumnsToMakeSortedIII {
    public int minDeletionSize(String[] strs) {
        int n = strs[0].length();
        int[] dp = new int[n];
        Arrays.fill(dp, 1);
        int maxKeep = 1;

        for (int j = 1; j < n; j++) {
            for (int i = 0; i < j; i++) {
                boolean valid = true;
                for (String s : strs) {
                    if (s.charAt(i) > s.charAt(j)) { valid = false; break; }
                }
                if (valid) dp[j] = Math.max(dp[j], dp[i] + 1);
            }
            maxKeep = Math.max(maxKeep, dp[j]);
        }
        return n - maxKeep;
    }
}
```
**Complexity:** O(cols² · rows) time, O(cols) space — beats the O(2^cols) brute force.

---

## C5. Minimum Number of Removals to Make Mountain Array

**Problem:** A mountain array strictly increases then strictly decreases (both parts non-empty). Given an array, return the minimum number of elements to remove so it becomes a mountain array.

**Example:**
```
Input: nums = [2,1,1,5,6,2,3,1]
Output: 3
Explanation: Remove elements to leave [1,5,6,3,1] or similar — a valid mountain 
of length 5, removing 3 elements.
```

**Brute force:** try every subset, check the mountain property → O(2ⁿ).
**Optimized:** compute `left[i]` = LIS ending at i (increasing), `right[i]` = longest strictly decreasing run STARTING at i; for every valid peak `i` (where both > 1), the best mountain length is `left[i] + right[i] - 1`.
```java
class MinRemovalsMountainArray {
    public int minimumMountainRemovals(int[] nums) {
        int n = nums.length;
        int[] left = lisLength(nums);
        int[] rightReversed = lisLength(reverseArray(nums));
        int[] right = reverseArray(rightReversed); // right[i] = longest decreasing run starting at i

        int best = 0;
        for (int i = 0; i < n; i++) {
            if (left[i] > 1 && right[i] > 1) best = Math.max(best, left[i] + right[i] - 1);
        }
        return n - best;
    }

    private int[] lisLength(int[] nums) {
        int n = nums.length;
        int[] dp = new int[n];
        Arrays.fill(dp, 1);
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < i; j++) {
                if (nums[j] < nums[i]) dp[i] = Math.max(dp[i], dp[j] + 1);
            }
        }
        return dp;
    }

    private int[] reverseArray(int[] nums) {
        int n = nums.length;
        int[] r = new int[n];
        for (int i = 0; i < n; i++) r[i] = nums[n - 1 - i];
        return r;
    }
}
```
**Complexity:** O(n²) time (two LIS passes), O(n) space — beats O(2ⁿ) brute force.

---

## C6. Maximum Height by Stacking Cuboids

**Problem:** Given cuboid dimensions, you may freely rotate each cuboid's 3 dimensions. Stack cuboid B on top of A only if every dimension of B ≤ the corresponding dimension of A. Maximize total stacked height.

**Example:**
```
Input: cuboids = [[50,45,20],[95,37,53],[45,23,12]]
Output: 190
Explanation: Orient and stack cuboids so their combined heights sum to 190 
while respecting the dimension-fits-inside constraint at every level.
```

**Brute force:** try every rotation × every stacking order → exponential.
**Optimized insight:** sort each cuboid's own 3 dimensions ascending (this is always optimal — using the largest dimension as "height" maximizes total height while the other two act as the "footprint" for stacking checks); then sort cuboids lexicographically; finally run an LIS-style DP where cuboid `j` can sit under cuboid `i` if ALL 3 sorted dimensions of `j` ≤ those of `i`.
```java
class MaxHeightStackingCuboids {
    public int maxHeight(int[][] cuboids) {
        for (int[] c : cuboids) Arrays.sort(c);
        Arrays.sort(cuboids, (a, b) -> {
            if (a[0] != b[0]) return a[0] - b[0];
            if (a[1] != b[1]) return a[1] - b[1];
            return a[2] - b[2];
        });

        int n = cuboids.length;
        int[] dp = new int[n];
        int best = 0;
        for (int i = 0; i < n; i++) {
            dp[i] = cuboids[i][2];
            for (int j = 0; j < i; j++) {
                if (cuboids[j][0] <= cuboids[i][0] && cuboids[j][1] <= cuboids[i][1] && cuboids[j][2] <= cuboids[i][2]) {
                    dp[i] = Math.max(dp[i], dp[j] + cuboids[i][2]);
                }
            }
            best = Math.max(best, dp[i]);
        }
        return best;
    }
}
```
**Complexity:** O(n²) time (plus O(n log n) sort), O(n) space.

---

## C7. Make Array Strictly Increasing

**Problem:** Given `arr1` and `arr2`, you may replace any element of `arr1` with any element of `arr2` (each choice independent, elements of arr2 reusable across different replacements). Return the minimum number of replacements to make `arr1` strictly increasing, or -1 if impossible.

**Example:**
```
Input: arr1 = [1,5,3,6,7], arr2 = [1,3,2,4]
Output: 1
Explanation: Replace 5 with 2: [1,2,3,6,7] — strictly increasing with 1 change.
```

**Brute force:** try every combination of replace/keep decisions → exponential.
**Optimized:** DP over "last value used" — `dp[value] = min operations to reach this point ending with `value``, using binary search on sorted+deduplicated arr2 to find the cheapest valid replacement.
```java
class MakeArrayStrictlyIncreasing {
    public int makeArrayIncreasing(int[] arr1, int[] arr2) {
        Arrays.sort(arr2);
        int[] arr2Unique = dedupe(arr2);

        Map<Integer, Integer> dp = new HashMap<>();
        dp.put(-1, 0); // sentinel: "previous value" = -1, 0 operations so far

        for (int num : arr1) {
            Map<Integer, Integer> next = new HashMap<>();
            for (Map.Entry<Integer, Integer> entry : dp.entrySet()) {
                int prevVal = entry.getKey(), ops = entry.getValue();

                if (num > prevVal) next.merge(num, ops, Math::min); // keep num as-is

                int idx = upperBound(arr2Unique, prevVal);
                if (idx < arr2Unique.length) {
                    next.merge(arr2Unique[idx], ops + 1, Math::min); // replace with smallest valid arr2 value
                }
            }
            if (next.isEmpty()) return -1;
            dp = next;
        }

        int best = Integer.MAX_VALUE;
        for (int v : dp.values()) best = Math.min(best, v);
        return best;
    }

    private int[] dedupe(int[] arr) {
        List<Integer> list = new ArrayList<>();
        for (int i = 0; i < arr.length; i++) if (i == 0 || arr[i] != arr[i - 1]) list.add(arr[i]);
        int[] result = new int[list.size()];
        for (int i = 0; i < result.length; i++) result[i] = list.get(i);
        return result;
    }

    private int upperBound(int[] arr, int val) {
        int lo = 0, hi = arr.length;
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (arr[mid] <= val) lo = mid + 1;
            else hi = mid;
        }
        return lo;
    }
}
```
**Complexity:** O(n · m log m) time (n = arr1 length, m = arr2 length; per-element hashmap size bounded, binary search per entry), O(m) space for arr2, O(states) for dp map.

---

# Section D: 2D Grid Traversal DP

## D1. Unique Paths

**Problem:** A robot starts at the top-left of an `m x n` grid and can only move right or down. Count the number of unique paths to the bottom-right corner.

**Example:**
```
Input: m = 3, n = 7
Output: 28
```

**Brute force:** DFS trying both directions at every cell without memo → O(2^(m+n)).
**Optimized:** `dp[i][j] = dp[i-1][j] + dp[i][j-1]`.
```java
class UniquePaths {
    public int uniquePaths(int m, int n) {
        int[] dp = new int[n];
        Arrays.fill(dp, 1);
        for (int i = 1; i < m; i++) {
            for (int j = 1; j < n; j++) {
                dp[j] += dp[j - 1];
            }
        }
        return dp[n - 1];
    }
}
```
**Complexity:** O(m·n) time, O(n) space. (A pure combinatorial formula `C(m+n-2, m-1)` also works in O(m+n) time.)

---

## D2. Unique Paths II

**Problem:** Same as Unique Paths, but the grid contains obstacles (marked 1) that block movement.

**Example:**
```
Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
Output: 2
```

**Brute force:** DFS avoiding obstacles without memo → exponential.
**Optimized:** same DP as Unique Paths, but zero out any cell that's an obstacle.
```java
class UniquePathsII {
    public int uniquePathsWithObstacles(int[][] obstacleGrid) {
        int rows = obstacleGrid.length, cols = obstacleGrid[0].length;
        int[] dp = new int[cols];
        dp[0] = (obstacleGrid[0][0] == 0) ? 1 : 0;

        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                if (obstacleGrid[i][j] == 1) {
                    dp[j] = 0;
                } else if (j > 0) {
                    dp[j] += dp[j - 1];
                }
            }
        }
        return dp[cols - 1];
    }
}
```
**Complexity:** O(m·n) time, O(n) space.

---

## D3. Minimum Path Sum

**Problem:** Given a grid of non-negative numbers, find the minimum sum path from top-left to bottom-right (moving only right or down).

**Example:**
```
Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
Output: 7
Explanation: Path 1->3->1->1->1 = 7.
```

**Brute force:** DFS trying both directions without memo → exponential.
**Optimized:** `dp[i][j] = grid[i][j] + min(dp[i-1][j], dp[i][j-1])`.
```java
class MinimumPathSum {
    public int minPathSum(int[][] grid) {
        int rows = grid.length, cols = grid[0].length;
        int[] dp = new int[cols];
        dp[0] = grid[0][0];
        for (int j = 1; j < cols; j++) dp[j] = dp[j - 1] + grid[0][j];

        for (int i = 1; i < rows; i++) {
            dp[0] += grid[i][0];
            for (int j = 1; j < cols; j++) {
                dp[j] = grid[i][j] + Math.min(dp[j], dp[j - 1]);
            }
        }
        return dp[cols - 1];
    }
}
```
**Complexity:** O(m·n) time, O(n) space.

---

## D4. Maximum Non-negative Product in a Matrix

**Problem:** From top-left to bottom-right (moving right/down only), maximize the PRODUCT of numbers on the path. Return the max product mod 10⁹+7, or -1 if every path yields a negative product.

**Example:**
```
Input: grid = [[-1,-2,-3],[-2,-3,-3],[-3,-3,-2]]
Output: -1
Explanation: Every path results in a negative product.
```

**Brute force:** try every path, compute product for each → O(2^(m+n)).
**Optimized:** track BOTH `maxProd[i][j]` and `minProd[i][j]` (since multiplying by a negative flips which is best), combining from both directions.
```java
class MaxNonNegativeProduct {
    public int maxProductPath(int[][] grid) {
        long MOD = 1_000_000_007;
        int rows = grid.length, cols = grid[0].length;
        long[][] maxP = new long[rows][cols];
        long[][] minP = new long[rows][cols];
        maxP[0][0] = minP[0][0] = grid[0][0];

        for (int j = 1; j < cols; j++) maxP[0][j] = minP[0][j] = maxP[0][j - 1] * grid[0][j];
        for (int i = 1; i < rows; i++) maxP[i][0] = minP[i][0] = maxP[i - 1][0] * grid[i][0];

        for (int i = 1; i < rows; i++) {
            for (int j = 1; j < cols; j++) {
                long val = grid[i][j];
                long a = maxP[i - 1][j] * val, b = minP[i - 1][j] * val;
                long c = maxP[i][j - 1] * val, d = minP[i][j - 1] * val;
                maxP[i][j] = Math.max(Math.max(a, b), Math.max(c, d));
                minP[i][j] = Math.min(Math.min(a, b), Math.min(c, d));
            }
        }

        long result = maxP[rows - 1][cols - 1];
        return result < 0 ? -1 : (int) (result % MOD);
    }
}
```
**Complexity:** O(m·n) time, O(m·n) space (reducible to O(n) with rolling arrays).

---

## D5. Where Will the Ball Fall

**Problem:** A grid represents diagonal deflectors (1 = deflects right, -1 = deflects left). For each starting column, drop a ball and determine which column it exits from (or -1 if it gets stuck in a "V" formed by two opposite deflectors).

**Example:**
```
Input: grid = [[1,1,1,-1,-1],[1,1,1,-1,-1],[-1,-1,-1,1,1],[1,1,1,1,-1],[-1,-1,-1,-1,-1]]
Output: [1,-1,-1,-1,-1]
```

**Brute force / actual approach:** simulate each ball independently row by row — there's no shortcut avoiding per-ball simulation since each ball's path depends on the specific deflector sequence it encounters.
```java
class WhereWillTheBallFall {
    public int[] findBall(int[][] grid) {
        int rows = grid.length, cols = grid[0].length;
        int[] result = new int[cols];

        for (int start = 0; start < cols; start++) {
            int col = start;
            for (int row = 0; row < rows; row++) {
                int dir = grid[row][col];
                int nextCol = col + dir;
                if (nextCol < 0 || nextCol >= cols || grid[row][nextCol] != dir) {
                    col = -1;
                    break;
                }
                col = nextCol;
            }
            result[start] = col;
        }
        return result;
    }
}
```
**Complexity:** O(rows · cols) time, O(cols) space.

---

## D6. Dungeon Game

**Problem:** A knight must traverse a dungeon grid (negative = damage, positive = heal) from top-left to bottom-right (moving right/down), with health never allowed to drop below 1. Return the minimum initial health needed.

**Example:**
```
Input: dungeon = [[-2,-3,3],[-5,-10,1],[10,30,-5]]
Output: 7
Explanation: Starting with 7 HP and following the optimal path keeps HP ≥ 1 
throughout, and 7 is the minimum such starting value.
```

**Brute force:** try every path forward, tracking minimum health needed — but this doesn't decompose forward (the health requirement depends on what's AHEAD, not behind) → naive forward DP doesn't work; brute-force enumeration of all paths is O(2^(m+n)).
**Optimized:** work BACKWARD from the destination — `dp[i][j] = minimum health needed ENTERING cell (i,j) to survive the rest of the journey`.
```java
class DungeonGame {
    public int calculateMinimumHP(int[][] dungeon) {
        int rows = dungeon.length, cols = dungeon[0].length;
        int[][] dp = new int[rows + 1][cols + 1];
        for (int[] row : dp) Arrays.fill(row, Integer.MAX_VALUE);
        dp[rows][cols - 1] = 1;
        dp[rows - 1][cols] = 1;

        for (int i = rows - 1; i >= 0; i--) {
            for (int j = cols - 1; j >= 0; j--) {
                int need = Math.min(dp[i + 1][j], dp[i][j + 1]) - dungeon[i][j];
                dp[i][j] = Math.max(1, need);
            }
        }
        return dp[0][0];
    }
}
```
**Complexity:** O(m·n) time, O(m·n) space (reducible to O(n) with a rolling array) — the backward direction is the key insight making this a clean DP instead of requiring path enumeration.

---

## D7. Cherry Pickup

**Problem:** A grid contains cherries (1), thorns (-1, impassable), and empty cells (0). Starting at top-left, travel to bottom-right (right/down only), then travel back to top-left (equivalently: up/left only, or modeled as TWO people walking forward simultaneously). Maximize total cherries collected (each cell's cherry collected at most once).

**Example:**
```
Input: grid = [[0,1,-1],[1,0,-1],[1,1,1]]
Output: 5
```

**Brute force:** simulate every pair of forward/backward path combinations independently → exponential.
**Optimized:** model as TWO people walking simultaneously from (0,0) to (n-1,n-1); since both take the same number of steps, track state `(row1, col1, row2)` — `col2` is derived as `row1+col1-row2` (both have moved the same total distance).
```java
class CherryPickup {
    public int cherryPickup(int[][] grid) {
        int n = grid.length;
        Integer[][][] memo = new Integer[n][n][n];
        int result = solve(0, 0, 0, grid, n, memo);
        return Math.max(result, 0);
    }

    private int solve(int r1, int c1, int r2, int[][] grid, int n, Integer[][][] memo) {
        int c2 = r1 + c1 - r2;
        if (r1 >= n || c1 >= n || r2 >= n || c2 >= n || grid[r1][c1] == -1 || grid[r2][c2] == -1) {
            return Integer.MIN_VALUE;
        }
        if (r1 == n - 1 && c1 == n - 1) return grid[r1][c1];
        if (memo[r1][c1][r2] != null) return memo[r1][c1][r2];

        int cherries = grid[r1][c1];
        if (r1 != r2) cherries += grid[r2][c2];

        int best = Integer.MIN_VALUE;
        best = Math.max(best, solve(r1 + 1, c1, r2 + 1, grid, n, memo));
        best = Math.max(best, solve(r1 + 1, c1, r2, grid, n, memo));
        best = Math.max(best, solve(r1, c1 + 1, r2 + 1, grid, n, memo));
        best = Math.max(best, solve(r1, c1 + 1, r2, grid, n, memo));

        int result = (best == Integer.MIN_VALUE) ? Integer.MIN_VALUE : best + cherries;
        memo[r1][c1][r2] = result;
        return result;
    }
}
```
**Complexity:** O(n³) time (n³ states, O(1) transitions), O(n³) space — reframing "there and back" as "two simultaneous forward paths" is the key insight avoiding a much harder direct formulation.

---

## D8. Number of Paths with Max Score

**Problem:** Given a board with digits (points), 'S' (start, bottom-right), 'E' (end, top-left), and 'X' (obstacle), move up/left/up-left from S to E, maximizing the sum of digits collected. Return `[maxScore, numberOfWaysToAchieveIt mod 1e9+7]`, or `[0,0]` if no path exists.

**Example:**
```
Input: board = ["E23","2X2","12S"]
Output: [7,1]
Explanation: The unique path with max score 7 collects digits summing to 7.
```

**Brute force:** DFS trying all directions without memo → exponential.
**Optimized:** `dp[i][j] = {maxScore, wayCount}` combining the three valid predecessor cells (down, right, down-right, since we conceptually move from S to E).
```java
class NumberOfPathsWithMaxScore {
    public int[] pathsWithMaxScore(List<String> board) {
        int MOD = 1_000_000_007;
        int n = board.size();
        int[][] dpScore = new int[n][n];
        long[][] dpCount = new long[n][n];
        for (int[] row : dpScore) Arrays.fill(row, -1);

        dpScore[n - 1][n - 1] = 0;
        dpCount[n - 1][n - 1] = 1;

        for (int i = n - 1; i >= 0; i--) {
            for (int j = n - 1; j >= 0; j--) {
                if (i == n - 1 && j == n - 1) continue;
                char ch = board.get(i).charAt(j);
                if (ch == 'X') continue;
                int val = (ch == 'S' || ch == 'E') ? 0 : (ch - '0');

                int bestScore = -1;
                long bestCount = 0;
                int[][] preds = {{i + 1, j}, {i, j + 1}, {i + 1, j + 1}};
                for (int[] p : preds) {
                    int pi = p[0], pj = p[1];
                    if (pi < n && pj < n && dpScore[pi][pj] != -1) {
                        int score = dpScore[pi][pj] + val;
                        if (score > bestScore) { bestScore = score; bestCount = dpCount[pi][pj]; }
                        else if (score == bestScore) { bestCount = (bestCount + dpCount[pi][pj]) % MOD; }
                    }
                }
                dpScore[i][j] = bestScore;
                dpCount[i][j] = bestCount;
            }
        }
        if (dpScore[0][0] == -1) return new int[]{0, 0};
        return new int[]{dpScore[0][0], (int) dpCount[0][0]};
    }
}
```
**Complexity:** O(n²) time, O(n²) space.

---

## D9. Cherry Pickup II

**Problem:** A grid has two robots starting at the top row (columns 0 and n-1). Both move down each step (choosing down-left, down, or down-right), collecting cherries (shared cells count once). Maximize total cherries collected by both robots after reaching the bottom row.

**Example:**
```
Input: grid = [[3,1,1],[2,5,1],[1,5,5],[2,1,1]]
Output: 24
```

**Brute force:** try every combination of moves for both robots independently at each row → exponential.
**Optimized:** `dp[row][col1][col2] = max cherries collectable from this row to the bottom`, trying all 3×3 = 9 move combinations per step.
```java
class CherryPickupII {
    public int cherryPickup(int[][] grid) {
        int rows = grid.length, cols = grid[0].length;
        Integer[][][] memo = new Integer[rows][cols][cols];
        return solve(0, 0, cols - 1, grid, rows, cols, memo);
    }

    private int solve(int row, int col1, int col2, int[][] grid, int rows, int cols, Integer[][][] memo) {
        if (col1 < 0 || col1 >= cols || col2 < 0 || col2 >= cols) return Integer.MIN_VALUE;
        if (memo[row][col1][col2] != null) return memo[row][col1][col2];

        int cherries = grid[row][col1];
        if (col1 != col2) cherries += grid[row][col2];

        if (row == rows - 1) {
            memo[row][col1][col2] = cherries;
            return cherries;
        }

        int best = Integer.MIN_VALUE;
        for (int d1 = -1; d1 <= 1; d1++) {
            for (int d2 = -1; d2 <= 1; d2++) {
                int nc1 = col1 + d1, nc2 = col2 + d2;
                if (nc1 >= 0 && nc1 < cols && nc2 >= 0 && nc2 < cols) {
                    best = Math.max(best, solve(row + 1, nc1, nc2, grid, rows, cols, memo));
                }
            }
        }
        int result = cherries + best;
        memo[row][col1][col2] = result;
        return result;
    }
}
```
**Complexity:** O(rows · cols² · 9) time = O(rows·cols²), O(rows·cols²) space.

---

## D10. Kth Smallest Instructions

**Problem:** To reach a destination `(row, col)` from the origin using only 'H' (horizontal/right) and 'V' (vertical/down) moves, find the kth lexicographically smallest valid instruction string.

**Example:**
```
Input: destination = [2,3], k = 1
Output: "HHVVV"
Explanation: The lexicographically smallest path (all H's before V's where possible) 
uses 3 H's and 2 V's — "HHVVV" is smallest since 'H' < 'V'.
```

**Brute force:** generate all C(row+col, row) permutations, sort, pick kth → factorial/combinatorial blowup, infeasible for large inputs.
**Optimized:** greedily decide each character — at each step, count how many valid completions START with 'H' using the combinatorial formula `C(remaining H's - 1 + remaining V's, remaining V's)`; if `k` falls within that count, choose 'H', otherwise choose 'V' and subtract that count from `k`.
```java
class KthSmallestInstructions {
    public String kthSmallestPath(int[] destination, int k) {
        int row = destination[0], col = destination[1];
        long[][] comb = buildPascal(row + col + 1);
        StringBuilder sb = new StringBuilder();
        int h = col, v = row;

        for (int i = 0; i < row + col; i++) {
            if (h == 0) { sb.append('V'); v--; continue; }
            if (v == 0) { sb.append('H'); h--; continue; }

            long countH = comb[h - 1 + v][v]; // number of valid completions if we place 'H' now
            if (k <= countH) {
                sb.append('H');
                h--;
            } else {
                sb.append('V');
                k -= countH;
                v--;
            }
        }
        return sb.toString();
    }

    private long[][] buildPascal(int n) {
        long[][] c = new long[n + 1][n + 1];
        for (int i = 0; i <= n; i++) {
            c[i][0] = 1;
            for (int j = 1; j <= i; j++) c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
        }
        return c;
    }
}
```
**Complexity:** O((row+col)²) time (Pascal's triangle precompute) + O(row+col) for the greedy construction, O((row+col)²) space.

---

## 🎯 Part 10b Summary Table

| # | Problem | Time | Space |
|---|---|---|---|
| C1 | LIS (DP) | O(n²) | O(n) |
| C1b | LIS (Binary Search) | O(n log n) | O(n) |
| C2 | Number of LIS | O(n²) | O(n) |
| C3 | Russian Doll Envelopes | O(n log n) | O(n) |
| C4 | Delete Columns III | O(cols²·rows) | O(cols) |
| C5 | Min Removals Mountain Array | O(n²) | O(n) |
| C6 | Max Height Stacking Cuboids | O(n²) | O(n) |
| C7 | Make Array Strictly Increasing | O(n·m log m) | O(m) |
| D1 | Unique Paths | O(m·n) | O(n) |
| D2 | Unique Paths II | O(m·n) | O(n) |
| D3 | Minimum Path Sum | O(m·n) | O(n) |
| D4 | Max Non-negative Product | O(m·n) | O(m·n) |
| D5 | Where Will Ball Fall | O(rows·cols) | O(cols) |
| D6 | Dungeon Game | O(m·n) | O(m·n) |
| D7 | Cherry Pickup | O(n³) | O(n³) |
| D8 | Paths With Max Score | O(n²) | O(n²) |
| D9 | Cherry Pickup II | O(rows·cols²) | O(rows·cols²) |
| D10 | Kth Smallest Instructions | O((row+col)²) | O((row+col)²) |

---

**Next: Part 10c — Prefix Sum + Hashmap Subarray (~23 problems), completing the "Classic DPs" category.** Say "continue" to proceed.
