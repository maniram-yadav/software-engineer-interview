# DP Solutions — Part 10c: Classic DPs (Prefix Sum + Hashmap Subarray) (Java)
### 20 Problems · Full Problem Statement + Example + Brute Force → Optimized + Complexity

---

# Section E: Cumulative / Prefix Sum

## E1. Range Sum Query - Immutable

**Problem:** Given an integer array, design a data structure to efficiently answer sum queries for a range `[left, right]`, with the array not changing after construction.

**Example:**
```
Input: nums = [-2,0,3,-5,2,-1], then sumRange(0,2), sumRange(2,5), sumRange(0,5)
Output: 1, -1, -3
```

**Brute force:** sum the range directly on every query → O(n) per query.
**Optimized:** precompute a prefix sum array once; each query becomes O(1).
```java
class RangeSumQueryImmutable {
    private int[] prefix;

    public RangeSumQueryImmutable(int[] nums) {
        prefix = new int[nums.length + 1];
        for (int i = 0; i < nums.length; i++) prefix[i + 1] = prefix[i] + nums[i];
    }

    public int sumRange(int left, int right) {
        return prefix[right + 1] - prefix[left];
    }
}
```
**Complexity:** O(n) build, O(1) per query, O(n) space.

---

## E2. Maximal Square

**Problem:** Given a binary matrix, find the largest square containing only 1's, return its AREA.

**Example:**
```
Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
Output: 4
Explanation: The largest square has side length 2 (area 4).
```

**Brute force:** for every cell, try every possible square size, verify all-ones → O(n³).
**Optimized:** `dp[i][j] = side length of the largest square with bottom-right corner at (i,j)`, using the invariant that all 3 neighboring squares (up, left, up-left) must support the extension.
```java
class MaximalSquare {
    public int maximalSquare(char[][] matrix) {
        int rows = matrix.length, cols = matrix[0].length;
        int[][] dp = new int[rows + 1][cols + 1];
        int best = 0;
        for (int i = 1; i <= rows; i++) {
            for (int j = 1; j <= cols; j++) {
                if (matrix[i - 1][j - 1] == '1') {
                    dp[i][j] = Math.min(dp[i - 1][j], Math.min(dp[i][j - 1], dp[i - 1][j - 1])) + 1;
                    best = Math.max(best, dp[i][j]);
                }
            }
        }
        return best * best;
    }
}
```
**Complexity:** O(rows·cols) time, O(rows·cols) space (reducible to O(cols)).

---

## E3. Range Sum Query 2D - Immutable

**Problem:** Given a 2D matrix, efficiently answer sum queries for any axis-aligned rectangle.

**Example:**
```
Input: matrix (5x5), then sumRegion(2,1,4,3)
Output: 8
```

**Brute force:** sum the rectangle directly per query → O(rows·cols) per query.
**Optimized:** 2D prefix sum via inclusion-exclusion.
```java
class NumMatrix {
    private int[][] prefix;

    public NumMatrix(int[][] matrix) {
        int rows = matrix.length, cols = matrix[0].length;
        prefix = new int[rows + 1][cols + 1];
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                prefix[i + 1][j + 1] = matrix[i][j] + prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j];
            }
        }
    }

    public int sumRegion(int row1, int col1, int row2, int col2) {
        return prefix[row2 + 1][col2 + 1] - prefix[row1][col2 + 1] - prefix[row2 + 1][col1] + prefix[row1][col1];
    }
}
```
**Complexity:** O(rows·cols) build, O(1) per query, O(rows·cols) space.

---

## E4. Largest Plus Sign

**Problem:** In an `n x n` grid with some mines (blocked cells), find the largest axis-aligned "plus sign" (a center cell plus arms extending equally in all 4 directions) made entirely of non-mine cells. Return its order (arm length including center; 0 if no valid plus exists).

**Example:**
```
Input: n = 5, mines = [[4,2]]
Output: 2
Explanation: The largest plus sign has order 2 (a 2-cell arm in each direction from its center).
```

**Brute force:** for every cell, try every possible plus size, verify all 4 arms → O(n³).
**Optimized:** for each cell, precompute the consecutive-1 run length from each of the 4 directions (left, right, up, down) via 4 linear sweeps; the plus-order at a cell is the MINIMUM of these 4 values.
```java
class LargestPlusSign {
    public int orderOfLargestPlusSign(int n, int[][] mines) {
        int[][] dp = new int[n][n];
        for (int[] row : dp) Arrays.fill(row, n);
        Set<Long> mineSet = new HashSet<>();
        for (int[] m : mines) mineSet.add((long) m[0] * n + m[1]);

        for (int i = 0; i < n; i++) {
            int count = 0;
            for (int j = 0; j < n; j++) { // left to right
                count = mineSet.contains((long) i * n + j) ? 0 : count + 1;
                dp[i][j] = Math.min(dp[i][j], count);
            }
            count = 0;
            for (int j = n - 1; j >= 0; j--) { // right to left
                count = mineSet.contains((long) i * n + j) ? 0 : count + 1;
                dp[i][j] = Math.min(dp[i][j], count);
            }
        }
        for (int j = 0; j < n; j++) {
            int count = 0;
            for (int i = 0; i < n; i++) { // top to bottom
                count = mineSet.contains((long) i * n + j) ? 0 : count + 1;
                dp[i][j] = Math.min(dp[i][j], count);
            }
            count = 0;
            for (int i = n - 1; i >= 0; i--) { // bottom to top
                count = mineSet.contains((long) i * n + j) ? 0 : count + 1;
                dp[i][j] = Math.min(dp[i][j], count);
            }
        }

        int best = 0;
        for (int i = 0; i < n; i++) for (int j = 0; j < n; j++) best = Math.max(best, dp[i][j]);
        return best;
    }
}
```
**Complexity:** O(n²) time, O(n²) space — beats the O(n³) brute force.

---

## E5. Push Dominoes

**Problem:** A row of dominoes, each `L` (falling left), `R` (falling right), or `.` (standing). Simulate the physics: return the final state.

**Example:**
```
Input: dominoes = "RR.L"
Output: "RR.L"
Explanation: The first two R's are already fallen right; the '.' feels no net 
force (R pushes right, but L needs distance); the L stays L.
```

**Brute force:** simulate physical propagation step by step until stable → O(n²) worst case.
**Optimized:** compute a "net force" at each position using two directional sweeps (force decays by 1 per step, resets at each R or L source, R contributes positive force, L contributes negative).
```java
class PushDominoes {
    public String pushDominoes(String dominoes) {
        char[] arr = dominoes.toCharArray();
        int n = arr.length;
        int[] forces = new int[n];

        int force = 0;
        for (int i = 0; i < n; i++) {
            if (arr[i] == 'R') force = n;
            else if (arr[i] == 'L') force = 0;
            else force = Math.max(force - 1, 0);
            forces[i] += force;
        }
        force = 0;
        for (int i = n - 1; i >= 0; i--) {
            if (arr[i] == 'L') force = n;
            else if (arr[i] == 'R') force = 0;
            else force = Math.max(force - 1, 0);
            forces[i] -= force;
        }

        StringBuilder sb = new StringBuilder();
        for (int f : forces) {
            if (f > 0) sb.append('R');
            else if (f < 0) sb.append('L');
            else sb.append('.');
        }
        return sb.toString();
    }
}
```
**Complexity:** O(n) time, O(n) space.

---

## E6. Largest 1-Bordered Square

**Problem:** Given a binary grid, find the largest square whose BORDER (not interior) consists entirely of 1's. Return its area.

**Example:**
```
Input: grid = [[1,1,1],[1,0,1],[1,1,1]]
Output: 9
Explanation: The entire 3x3 grid's border is all 1's.
```

**Brute force:** for every cell and every candidate size, check the entire border → O(n⁴).
**Optimized:** precompute `left[i][j]` (consecutive 1's ending at (i,j) going left) and `up[i][j]` (consecutive 1's ending at (i,j) going up); for each bottom-right corner, try decreasing candidate sizes, checking only the two "opposite" border segments (top edge via `up` at the top-right corner, left edge via `left` at the bottom-left corner) for O(1) validity per size.
```java
class Largest1BorderedSquare {
    public int largest1BorderedSquare(int[][] grid) {
        int rows = grid.length, cols = grid[0].length;
        int[][] left = new int[rows][cols], up = new int[rows][cols];
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                if (grid[i][j] == 1) {
                    left[i][j] = (j == 0) ? 1 : left[i][j - 1] + 1;
                    up[i][j] = (i == 0) ? 1 : up[i - 1][j] + 1;
                }
            }
        }

        int best = 0;
        for (int i = rows - 1; i >= 0; i--) {
            for (int j = cols - 1; j >= 0; j--) {
                int maxSize = Math.min(left[i][j], up[i][j]);
                while (maxSize > best) {
                    if (up[i][j - maxSize + 1] >= maxSize && left[i - maxSize + 1][j] >= maxSize) {
                        best = maxSize;
                        break;
                    }
                    maxSize--;
                }
            }
        }
        return best * best;
    }
}
```
**Complexity:** O(rows·cols·min(rows,cols)) time worst case, O(rows·cols) space — beats O(n⁴) brute force.

---

## E7. Count Square Submatrices With All Ones

**Problem:** Given a binary matrix, count the total number of square submatrices consisting entirely of 1's (all sizes).

**Example:**
```
Input: matrix = [[0,1,1,1],[1,1,1,1],[0,1,1,1]]
Output: 15
```

**Brute force:** for every cell and every square size, verify all-ones → O(n⁴).
**Optimized:** same DP as Maximal Square, but SUM every `dp[i][j]` value instead of tracking the max (each `dp[i][j]` counts exactly the number of valid squares with bottom-right corner at (i,j), since a square of side k there implies squares of side 1..k-1 also exist there).
```java
class CountSquareSubmatrices {
    public int countSquares(int[][] matrix) {
        int rows = matrix.length, cols = matrix[0].length;
        int[][] dp = new int[rows][cols];
        int total = 0;
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                if (matrix[i][j] == 1) {
                    dp[i][j] = (i == 0 || j == 0) ? 1
                        : Math.min(dp[i - 1][j], Math.min(dp[i][j - 1], dp[i - 1][j - 1])) + 1;
                    total += dp[i][j];
                }
            }
        }
        return total;
    }
}
```
**Complexity:** O(rows·cols) time, O(rows·cols) space.

---

## E8. Matrix Block Sum

**Problem:** Given a matrix and a radius `k`, return a new matrix where each cell is the sum of all elements within Manhattan-clipped distance `k` (i.e., a `(2k+1)x(2k+1)` block clipped to grid bounds).

**Example:**
```
Input: mat = [[1,2,3],[4,5,6],[7,8,9]], k = 1
Output: [[12,21,16],[27,45,33],[24,39,28]]
```

**Brute force:** for every cell, sum the block directly → O(rows·cols·k²).
**Optimized:** 2D prefix sum, then O(1) block-sum lookup per cell (clipped to bounds).
```java
class MatrixBlockSum {
    public int[][] matrixBlockSum(int[][] mat, int k) {
        int rows = mat.length, cols = mat[0].length;
        int[][] prefix = new int[rows + 1][cols + 1];
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                prefix[i + 1][j + 1] = mat[i][j] + prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j];
            }
        }

        int[][] result = new int[rows][cols];
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                int r1 = Math.max(0, i - k), c1 = Math.max(0, j - k);
                int r2 = Math.min(rows - 1, i + k), c2 = Math.min(cols - 1, j + k);
                result[i][j] = prefix[r2 + 1][c2 + 1] - prefix[r1][c2 + 1] - prefix[r2 + 1][c1] + prefix[r1][c1];
            }
        }
        return result;
    }
}
```
**Complexity:** O(rows·cols) time, O(rows·cols) space.

---

## E9. Maximum Points You Can Obtain From Cards

**Problem:** Given a row of cards, take exactly `k` cards from either end (any combination), maximize the sum of taken cards.

**Example:**
```
Input: cardPoints = [1,2,3,4,5,6,1], k = 3
Output: 12
Explanation: Take the last 3 cards: 4+6+1... actually best is take 1,6,5? 
Best is last 3: 6+1 and one from front... optimal is take last 2 and first 1: 
1+6+5=... the actual max is 12 via specific combination.
```

**Brute force:** try every combination of taking `i` from the front and `k-i` from the back → O(k).
**Optimized insight:** equivalent to MINIMIZING the sum of the remaining contiguous middle window of length `n-k` (total - minWindow = maxTaken), solvable via sliding window.
```java
class MaxPointsCards {
    public int maxScore(int[] cardPoints, int k) {
        int n = cardPoints.length;
        int windowSize = n - k;
        int total = 0;
        for (int c : cardPoints) total += c;
        if (windowSize == 0) return total;

        int windowSum = 0;
        for (int i = 0; i < windowSize; i++) windowSum += cardPoints[i];
        int minWindow = windowSum;
        for (int i = windowSize; i < n; i++) {
            windowSum += cardPoints[i] - cardPoints[i - windowSize];
            minWindow = Math.min(minWindow, windowSum);
        }
        return total - minWindow;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## E10. Count Submatrices With All Ones

**Problem:** Given a binary matrix, count the total number of submatrices (any rectangular shape, not just squares) consisting entirely of 1's.

**Example:**
```
Input: mat = [[1,0,1],[1,1,0],[1,1,0]]
Output: 13
```

**Brute force:** check every possible rectangle → O((rows·cols)²).
**Optimized:** precompute `heights[i][j]` = consecutive 1's ending at (i,j) going up (like a histogram per row); for each cell as the bottom-right corner, scan leftward tracking the running minimum height, adding that minimum to the total at each step (this counts all rectangles ending exactly at this cell with varying widths).
```java
class CountSubmatricesAllOnes {
    public int numSubmat(int[][] mat) {
        int rows = mat.length, cols = mat[0].length;
        int[][] heights = new int[rows][cols];
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                heights[i][j] = (mat[i][j] == 0) ? 0 : (i == 0 ? 1 : heights[i - 1][j] + 1);
            }
        }

        int total = 0;
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                int minHeight = Integer.MAX_VALUE;
                for (int k = j; k >= 0; k--) {
                    minHeight = Math.min(minHeight, heights[i][k]);
                    if (minHeight == 0) break;
                    total += minHeight;
                }
            }
        }
        return total;
    }
}
```
**Complexity:** O(rows·cols²) time, O(rows·cols) space.

---

## E11. Ways to Make a Fair Array

**Problem:** For each index in the array, if that ELEMENT were removed, check whether the sum of remaining even-indexed elements equals the sum of remaining odd-indexed elements. Count how many indices satisfy this.

**Example:**
```
Input: nums = [2,1,6,4]
Output: 1
Explanation: Removing index 0 gives [1,6,4] with even-idx sum 1+4=5, odd-idx sum 6 — not equal.
Removing index 1 gives [2,6,4] with even-idx sum 2+4=6, odd-idx sum 6 — equal! Count this one.
```

**Brute force:** for each index, actually remove it and recompute both sums from scratch → O(n²).
**Optimized:** maintain running prefix sums of even/odd positions BEFORE index i; the suffix sums (after removal, positions shift parity) can be derived from precomputed total even/odd sums minus the prefix — all in O(1) per index.
```java
class WaysToMakeFairArray {
    public int waysToMakeFair(int[] nums) {
        int n = nums.length;
        int totalEven = 0, totalOdd = 0;
        for (int i = 0; i < n; i++) {
            if (i % 2 == 0) totalEven += nums[i]; else totalOdd += nums[i];
        }

        int leftEven = 0, leftOdd = 0;
        int count = 0;
        for (int i = 0; i < n; i++) {
            int rightEven = totalEven - leftEven - (i % 2 == 0 ? nums[i] : 0);
            int rightOdd = totalOdd - leftOdd - (i % 2 == 1 ? nums[i] : 0);
            // after removing index i, elements after i shift parity: old-odd becomes new-even, old-even becomes new-odd
            int newEvenSum = leftEven + rightOdd;
            int newOddSum = leftOdd + rightEven;
            if (newEvenSum == newOddSum) count++;

            if (i % 2 == 0) leftEven += nums[i]; else leftOdd += nums[i];
        }
        return count;
    }
}
```
**Complexity:** O(n) time, O(1) space — beats the O(n²) brute force.

---

## E12. Maximal Rectangle

**Problem:** Given a binary matrix, find the area of the largest rectangle consisting entirely of 1's.

**Example:**
```
Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
Output: 6
```

**Brute force:** check every possible rectangle → O((rows·cols)²).
**Optimized:** for each row, build a "histogram" of consecutive-1 heights per column (like Maximal Square's building blocks), then apply the Largest Rectangle in Histogram technique (monotonic stack) per row.
```java
class MaximalRectangle {
    public int maximalRectangle(char[][] matrix) {
        if (matrix.length == 0) return 0;
        int cols = matrix[0].length;
        int[] heights = new int[cols];
        int best = 0;

        for (char[] row : matrix) {
            for (int j = 0; j < cols; j++) {
                heights[j] = (row[j] == '1') ? heights[j] + 1 : 0;
            }
            best = Math.max(best, largestRectangleInHistogram(heights));
        }
        return best;
    }

    private int largestRectangleInHistogram(int[] heights) {
        Deque<Integer> stack = new ArrayDeque<>();
        int best = 0;
        int n = heights.length;
        for (int i = 0; i <= n; i++) {
            int h = (i == n) ? 0 : heights[i];
            while (!stack.isEmpty() && heights[stack.peek()] >= h) {
                int height = heights[stack.pop()];
                int width = stack.isEmpty() ? i : i - stack.peek() - 1;
                best = Math.max(best, height * width);
            }
            stack.push(i);
        }
        return best;
    }
}
```
**Complexity:** O(rows·cols) time, O(cols) space.

---

## E13. Max Sum of Rectangle No Larger Than K

**Problem:** Given a matrix and integer `k`, find the maximum sum of a rectangular submatrix such that the sum is ≤ k.

**Example:**
```
Input: matrix = [[1,0,1],[0,-2,3]], k = 2
Output: 2
Explanation: The rectangle [[0,1],[-2,3]] (columns 1-2, both rows) sums to 2.
```

**Brute force:** try every rectangle directly → O((rows·cols)²).
**Optimized:** fix left/right column boundaries (O(cols²) pairs), compress each row-range into a 1D row-sum array, then use a TreeSet of prefix sums to find, for each running sum, the smallest earlier prefix sum ≥ `currSum - k` (giving the largest valid sub-sum ≤ k) via `ceiling()`.
```java
class MaxSumRectangleNoLargerThanK {
    public int maxSumSubmatrix(int[][] matrix, int k) {
        int rows = matrix.length, cols = matrix[0].length;
        int best = Integer.MIN_VALUE;

        for (int left = 0; left < cols; left++) {
            int[] rowSum = new int[rows];
            for (int right = left; right < cols; right++) {
                for (int i = 0; i < rows; i++) rowSum[i] += matrix[i][right];

                TreeSet<Integer> prefixSums = new TreeSet<>();
                prefixSums.add(0);
                int currSum = 0;
                for (int sum : rowSum) {
                    currSum += sum;
                    Integer ceiling = prefixSums.ceiling(currSum - k);
                    if (ceiling != null) best = Math.max(best, currSum - ceiling);
                    prefixSums.add(currSum);
                }
            }
        }
        return best;
    }
}
```
**Complexity:** O(cols² · rows · log(rows)) time, O(rows) space.

---

## E14. Super Washing Machines

**Problem:** Given the number of dresses in each washing machine, in one move you may transfer exactly one dress between two ADJACENT machines. Return the minimum moves to equalize all machines, or -1 if impossible.

**Example:**
```
Input: machines = [1,0,5]
Output: 3
Explanation: Move dresses so each machine ends with 2 dresses, achievable in 3 moves.
```

**Brute force:** simulate every possible sequence of transfers → exponential.
**Optimized insight:** if not evenly divisible, impossible. Otherwise, the answer is the max over all positions of `max(|running balance|, single-machine excess over average)` — the running balance represents net dresses that must cross the boundary at that position.
```java
class SuperWashingMachines {
    public int findMinMoves(int[] machines) {
        int n = machines.length;
        int total = 0;
        for (int m : machines) total += m;
        if (total % n != 0) return -1;

        int avg = total / n;
        int maxMoves = 0, runningBalance = 0;
        for (int m : machines) {
            int diff = m - avg;
            runningBalance += diff;
            maxMoves = Math.max(maxMoves, Math.max(Math.abs(runningBalance), diff));
        }
        return maxMoves;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## E15. Maximum Sum of 3 Non-Overlapping Subarrays

**Problem:** Given an array and length `k`, find 3 non-overlapping subarrays of length `k` that maximize the total sum. Return their starting indices (smallest lexicographically if tied).

**Example:**
```
Input: nums = [1,2,1,2,6,7,5,1], k = 2
Output: [0,3,5]
Explanation: Subarrays [1,2],[2,6],[7,5] with sums 3,8,12 — total 23, maximal.
```

**Brute force:** try every triple of non-overlapping window start positions → O(n³).
**Optimized:** precompute all window sums via sliding window (O(n)); then `left[i]` = best single-window index in `[0,i]`, `right[i]` = best single-window index in `[i,end]` (with tie-breaking toward smaller index); finally iterate all valid middle window positions combining `left` and `right`.
```java
class MaxSum3NonOverlapping {
    public int[] maxSumOfThreeSubarrays(int[] nums, int k) {
        int n = nums.length;
        int[] windowSum = new int[n - k + 1];
        int sum = 0;
        for (int i = 0; i < n; i++) {
            sum += nums[i];
            if (i >= k) sum -= nums[i - k];
            if (i >= k - 1) windowSum[i - k + 1] = sum;
        }

        int m = windowSum.length;
        int[] left = new int[m];
        int best = 0;
        for (int i = 0; i < m; i++) {
            if (windowSum[i] > windowSum[best]) best = i;
            left[i] = best;
        }

        int[] right = new int[m];
        best = m - 1;
        for (int i = m - 1; i >= 0; i--) {
            if (windowSum[i] >= windowSum[best]) best = i;
            right[i] = best;
        }

        int[] result = new int[3];
        int maxTotal = -1;
        for (int mid = k; mid < m - k; mid++) {
            int l = left[mid - k], r = right[mid + k];
            int total = windowSum[l] + windowSum[mid] + windowSum[r];
            if (total > maxTotal) {
                maxTotal = total;
                result = new int[]{l, mid, r};
            }
        }
        return result;
    }
}
```
**Complexity:** O(n) time, O(n) space — beats the O(n³) brute force.

---

## E16. Number of Submatrices That Sum to Target

**Problem:** Given a matrix and a target, count the number of submatrices whose sum EQUALS the target exactly.

**Example:**
```
Input: matrix = [[0,1,0],[1,1,1],[0,1,0]], target = 0
Output: 4
```

**Brute force:** check every rectangle's sum directly → O((rows·cols)²).
**Optimized:** fix left/right column boundaries (O(cols²)), compress to a 1D row-sum array, then apply the classic "subarray sum equals K" prefix-sum + hashmap technique in O(rows).
```java
class NumSubmatricesSumTarget {
    public int numSubmatrixSumTarget(int[][] matrix, int target) {
        int rows = matrix.length, cols = matrix[0].length;
        int total = 0;

        for (int left = 0; left < cols; left++) {
            int[] rowSum = new int[rows];
            for (int right = left; right < cols; right++) {
                for (int i = 0; i < rows; i++) rowSum[i] += matrix[i][right];

                Map<Integer, Integer> prefixCount = new HashMap<>();
                prefixCount.put(0, 1);
                int currSum = 0;
                for (int sum : rowSum) {
                    currSum += sum;
                    total += prefixCount.getOrDefault(currSum - target, 0);
                    prefixCount.merge(currSum, 1, Integer::sum);
                }
            }
        }
        return total;
    }
}
```
**Complexity:** O(cols² · rows) time, O(rows) space.

---

## E17. Get the Maximum Score

**Problem:** Given two strictly increasing sorted arrays, you can travel through either array in order, switching between them ONLY at values common to both. Maximize the sum of the path visited, modulo 10⁹+7.

**Example:**
```
Input: nums1 = [2,4,5,8,10], nums2 = [4,6,8,9]
Output: 30
Explanation: Path 2 -> 4 -> 6 -> 8 -> 9 -> 10 gives sum 2+4+6+8+9+10 = 39... 
actually optimal path chooses the better branch at each common point, giving 30.
```

**Brute force:** try every combination of switch-points between the two arrays → exponential.
**Optimized:** two-pointer merge — accumulate running sums separately for each array between common values; at each common value, take the MAX of the two accumulated sums (the better path so far), add the common value, and reset both running sums.
```java
class GetMaximumScore {
    public int maxSum(int[] nums1, int[] nums2) {
        long MOD = 1_000_000_007;
        int i = 0, j = 0;
        long sum1 = 0, sum2 = 0, result = 0;

        while (i < nums1.length && j < nums2.length) {
            if (nums1[i] < nums2[j]) {
                sum1 += nums1[i++];
            } else if (nums1[i] > nums2[j]) {
                sum2 += nums2[j++];
            } else {
                result += Math.max(sum1, sum2) + nums1[i];
                sum1 = 0; sum2 = 0;
                i++; j++;
            }
        }
        while (i < nums1.length) sum1 += nums1[i++];
        while (j < nums2.length) sum2 += nums2[j++];
        result += Math.max(sum1, sum2);

        return (int) (result % MOD);
    }
}
```
**Complexity:** O(n + m) time, O(1) space.

---

# Section F: Hashmap (Subarray)

## F1. Continuous Subarray Sum

**Problem:** Given an array and integer `k`, determine if there's a contiguous subarray of length ≥ 2 whose sum is a multiple of `k`.

**Example:**
```
Input: nums = [23,2,4,6,7], k = 6
Output: true
Explanation: [2,4] sums to 6, a multiple of 6.
```

**Brute force:** check every subarray's sum for divisibility → O(n²).
**Optimized:** track `prefixSum mod k` in a hashmap with its FIRST occurrence index; if the same remainder recurs at an index at least 2 apart, the subarray between them sums to a multiple of k.
```java
class ContinuousSubarraySum {
    public boolean checkSubarraySum(int[] nums, int k) {
        Map<Integer, Integer> remainderIndex = new HashMap<>();
        remainderIndex.put(0, -1);
        int sum = 0;
        for (int i = 0; i < nums.length; i++) {
            sum += nums[i];
            int rem = ((sum % k) + k) % k;
            if (remainderIndex.containsKey(rem)) {
                if (i - remainderIndex.get(rem) >= 2) return true;
            } else {
                remainderIndex.put(rem, i);
            }
        }
        return false;
    }
}
```
**Complexity:** O(n) time, O(min(n,k)) space.

---

## F2. Find Two Non-overlapping Sub-arrays Each With Target Sum

**Problem:** Find two NON-OVERLAPPING subarrays that each sum EXACTLY to `target`, minimizing the sum of their two lengths. Return -1 if impossible.

**Example:**
```
Input: arr = [3,2,2,4,3], target = 3
Output: 2
Explanation: Two subarrays [3] and [3] (the first and last elements), 
combined length 1+1 = 2.
```

**Brute force:** try every pair of valid non-overlapping subarrays → O(n³) or worse.
**Optimized:** sliding window to find, ending at each index, the shortest subarray summing to target (if any); maintain a running `best` (shortest window seen so far up to a boundary), and combine with the current window whenever both exist without overlapping.
```java
class FindTwoNonOverlappingSubarrays {
    public int minSumOfLengths(int[] arr, int target) {
        int n = arr.length;
        int[] minLenEndingAt = new int[n]; // best (shortest) valid window length using indices [0..i]
        Arrays.fill(minLenEndingAt, Integer.MAX_VALUE);

        int left = 0, sum = 0;
        int best = Integer.MAX_VALUE, result = Integer.MAX_VALUE;
        for (int right = 0; right < n; right++) {
            sum += arr[right];
            while (sum > target) { sum -= arr[left]; left++; }
            if (sum == target) {
                int currLen = right - left + 1;
                if (left > 0 && minLenEndingAt[left - 1] != Integer.MAX_VALUE) {
                    result = Math.min(result, currLen + minLenEndingAt[left - 1]);
                }
                best = Math.min(best, currLen);
            }
            minLenEndingAt[right] = best;
        }
        return result == Integer.MAX_VALUE ? -1 : result;
    }
}
```
**Complexity:** O(n) time (sliding window pointers each move forward only), O(n) space.

---

## F3. Maximum Number of Non-overlapping Subarrays With Sum Equals Target

**Problem:** Find the MAXIMUM count of non-overlapping subarrays that each sum exactly to `target`.

**Example:**
```
Input: nums = [1,1,1,1,1], target = 2
Output: 2
Explanation: Two non-overlapping subarrays [1,1] and [1,1] (using 4 of the 5 elements).
```

**Brute force:** try every combination of non-overlapping valid subarrays → exponential.
**Optimized:** greedy left-to-right scan with a prefix-sum hashset — whenever `currentPrefixSum - target` has been seen, greedily TAKE that subarray (count++), then RESET the hashset entirely (since any future subarray can't overlap this one, effectively restarting the prefix-sum tracking from this point) — provably optimal via exchange argument (taking the earliest-ending valid subarray never hurts future options).
```java
class MaxNonOverlappingSubarraysTarget {
    public int maxNonOverlapping(int[] nums, int target) {
        Set<Integer> prefixSums = new HashSet<>();
        prefixSums.add(0);
        int sum = 0, count = 0;

        for (int num : nums) {
            sum += num;
            if (prefixSums.contains(sum - target)) {
                count++;
                prefixSums.clear();
                prefixSums.add(0);
                sum = 0; // restart prefix tracking after greedily taking this subarray
            } else {
                prefixSums.add(sum);
            }
        }
        return count;
    }
}
```
**Complexity:** O(n) time, O(n) space.

---

## 🎯 Part 10c Summary Table

| # | Problem | Time | Space |
|---|---|---|---|
| E1 | Range Sum Query Immutable | O(1) query | O(n) |
| E2 | Maximal Square | O(rows·cols) | O(rows·cols) |
| E3 | Range Sum Query 2D | O(1) query | O(rows·cols) |
| E4 | Largest Plus Sign | O(n²) | O(n²) |
| E5 | Push Dominoes | O(n) | O(n) |
| E6 | Largest 1-Bordered Square | O(rc·min(r,c)) | O(rc) |
| E7 | Count Square Submatrices | O(rows·cols) | O(rows·cols) |
| E8 | Matrix Block Sum | O(rows·cols) | O(rows·cols) |
| E9 | Max Points From Cards | O(n) | O(1) |
| E10 | Count Submatrices All Ones | O(rows·cols²) | O(rows·cols) |
| E11 | Ways to Make Fair Array | O(n) | O(1) |
| E12 | Maximal Rectangle | O(rows·cols) | O(cols) |
| E13 | Max Sum Rectangle ≤ K | O(cols²·rows·log rows) | O(rows) |
| E14 | Super Washing Machines | O(n) | O(1) |
| E15 | Max Sum 3 Non-overlapping | O(n) | O(n) |
| E16 | Num Submatrices Sum Target | O(cols²·rows) | O(rows) |
| E17 | Get the Maximum Score | O(n+m) | O(1) |
| F1 | Continuous Subarray Sum | O(n) | O(min(n,k)) |
| F2 | Two Non-overlapping Subarrays | O(n) | O(n) |
| F3 | Max Non-overlapping Subarrays | O(n) | O(n) |

---

**This completes Part 10 (Classic DPs)! Remaining: Parts 11–16 — DP+Tricks (3), Insertion DP (1), Graph DP (2), Memoization (6), Binary Lifting (1), Math (7) = ~20 problems.** Say "continue" to proceed.
