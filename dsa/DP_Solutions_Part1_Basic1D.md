# DP Solutions — Part 1: Basic 1D DP (Java)
### 37 Problems · Brute Force Note + Optimized Approach + Complexity

---

## 1. Climbing Stairs
**Brute force:** recursion trying 1 or 2 steps each call → O(2ⁿ).
**Optimized:** `dp[i] = dp[i-1] + dp[i-2]` (Fibonacci), space-optimized to two variables.
```java
class ClimbingStairs {
    public int climbStairs(int n) {
        if (n <= 2) return n;
        int prev2 = 1, prev1 = 2;
        for (int i = 3; i <= n; i++) {
            int curr = prev1 + prev2;
            prev2 = prev1;
            prev1 = curr;
        }
        return prev1;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 2. Best Time to Buy and Sell Stock
**Brute force:** check every pair (i,j), i<j → O(n²).
**Optimized:** track running minimum price, update max profit in one pass.
```java
class BuySellStock {
    public int maxProfit(int[] prices) {
        int minPrice = Integer.MAX_VALUE, maxProfit = 0;
        for (int p : prices) {
            minPrice = Math.min(minPrice, p);
            maxProfit = Math.max(maxProfit, p - minPrice);
        }
        return maxProfit;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 3. Min Cost Climbing Stairs
**Optimized:** `dp[i] = cost[i] + min(dp[i-1], dp[i-2])`, answer = min(dp[n-1], dp[n-2]).
```java
class MinCostClimbingStairs {
    public int minCostClimbingStairs(int[] cost) {
        int n = cost.length;
        int prev2 = 0, prev1 = 0;
        for (int i = 2; i <= n; i++) {
            int curr = Math.min(prev1 + cost[i - 1], prev2 + cost[i - 2]);
            prev2 = prev1;
            prev1 = curr;
        }
        return prev1;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 4. Divisor Game
**Brute force:** `dp[i] = true if exists x<i, i%x==0, dp[i-x]==false` → O(n²).
**Optimized insight:** Alice wins iff n is even (provable by induction).
```java
class DivisorGame {
    public boolean divisorGame(int n) {
        return n % 2 == 0;
    }
}
```
**Complexity:** O(1) time/space (O(n²) if solved via literal DP without the parity insight).

---

## 5. Decode Ways
**Optimized:** `dp[i] = dp[i-1] (if s[i-1] valid 1-digit) + dp[i-2] (if s[i-2..i-1] valid 2-digit)`.
```java
class DecodeWays {
    public int numDecodings(String s) {
        int n = s.length();
        if (n == 0 || s.charAt(0) == '0') return 0;
        int prev2 = 1, prev1 = 1;
        for (int i = 1; i < n; i++) {
            int curr = 0;
            if (s.charAt(i) != '0') curr += prev1;
            int twoDigit = Integer.parseInt(s.substring(i - 1, i + 1));
            if (twoDigit >= 10 && twoDigit <= 26) curr += prev2;
            prev2 = prev1;
            prev1 = curr;
        }
        return prev1;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 6. Unique Binary Search Trees
**Brute force:** recursively count trees for every root split, no memo → exponential.
**Optimized:** `dp[i] = Σ dp[j] * dp[i-1-j]` for j in [0, i-1] (Catalan number recurrence).
```java
class UniqueBST {
    public int numTrees(int n) {
        int[] dp = new int[n + 1];
        dp[0] = 1;
        for (int nodes = 1; nodes <= n; nodes++) {
            for (int root = 1; root <= nodes; root++) {
                dp[nodes] += dp[root - 1] * dp[nodes - root];
            }
        }
        return dp[n];
    }
}
```
**Complexity:** O(n²) time, O(n) space.

---

## 7. House Robber
**Optimized:** `dp[i] = max(dp[i-1], dp[i-2] + nums[i])`.
```java
class HouseRobber {
    public int rob(int[] nums) {
        int prev2 = 0, prev1 = 0;
        for (int num : nums) {
            int curr = Math.max(prev1, prev2 + num);
            prev2 = prev1;
            prev1 = curr;
        }
        return prev1;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 8. Perfect Squares
**Brute force:** BFS/DFS trying every square ≤ n at each step without memo → exponential.
**Optimized:** `dp[i] = 1 + min(dp[i - j*j])` for all j*j ≤ i.
```java
class PerfectSquares {
    public int numSquares(int n) {
        int[] dp = new int[n + 1];
        Arrays.fill(dp, Integer.MAX_VALUE);
        dp[0] = 0;
        for (int i = 1; i <= n; i++) {
            for (int j = 1; j * j <= i; j++) {
                dp[i] = Math.min(dp[i], dp[i - j * j] + 1);
            }
        }
        return dp[n];
    }
}
```
**Complexity:** O(n·√n) time, O(n) space.

---

## 9. Best Time to Buy/Sell Stock with Cooldown
**Optimized:** state machine DP — `hold`, `sold`, `rest` states.
```java
class BuySellCooldown {
    public int maxProfit(int[] prices) {
        if (prices.length == 0) return 0;
        int hold = -prices[0], sold = 0, rest = 0;
        for (int i = 1; i < prices.length; i++) {
            int prevSold = sold;
            sold = hold + prices[i];
            hold = Math.max(hold, rest - prices[i]);
            rest = Math.max(rest, prevSold);
        }
        return Math.max(sold, rest);
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 10. Coin Change
**Brute force:** try every coin recursively at each amount, no memo → O(coins^amount).
**Optimized:** `dp[i] = min(dp[i - coin] + 1)` over all coins.
```java
class CoinChange {
    public int coinChange(int[] coins, int amount) {
        int[] dp = new int[amount + 1];
        Arrays.fill(dp, amount + 1);
        dp[0] = 0;
        for (int i = 1; i <= amount; i++) {
            for (int coin : coins) {
                if (coin <= i) dp[i] = Math.min(dp[i], dp[i - coin] + 1);
            }
        }
        return dp[amount] > amount ? -1 : dp[amount];
    }
}
```
**Complexity:** O(amount · coins) time, O(amount) space.

---

## 11. Counting Bits
**Brute force:** count set bits per number independently via loop → O(n log n).
**Optimized:** `dp[i] = dp[i >> 1] + (i & 1)`.
```java
class CountingBits {
    public int[] countBits(int n) {
        int[] dp = new int[n + 1];
        for (int i = 1; i <= n; i++) dp[i] = dp[i >> 1] + (i & 1);
        return dp;
    }
}
```
**Complexity:** O(n) time, O(n) space.

---

## 12. Integer Break
**Optimized:** `dp[i] = max over j of max(j*(i-j), j*dp[i-j])`.
```java
class IntegerBreak {
    public int integerBreak(int n) {
        int[] dp = new int[n + 1];
        dp[1] = 1;
        for (int i = 2; i <= n; i++) {
            for (int j = 1; j < i; j++) {
                dp[i] = Math.max(dp[i], Math.max(j * (i - j), j * dp[i - j]));
            }
        }
        return dp[n];
    }
}
```
**Complexity:** O(n²) time, O(n) space.

---

## 13. Count Numbers with Unique Digits
**Optimized:** combinatorial counting per digit length (permutation count), summed.
```java
class UniqueDigits {
    public int countNumbersWithUniqueDigits(int n) {
        if (n == 0) return 1;
        int result = 10, uniqueDigits = 9, availableDigits = 9;
        for (int i = 2; i <= n && availableDigits > 0; i++) {
            uniqueDigits *= availableDigits;
            result += uniqueDigits;
            availableDigits--;
        }
        return result;
    }
}
```
**Complexity:** O(n) time, O(1) space (n ≤ 10 practically bounded).

---

## 14. Wiggle Subsequence
**Brute force:** try all subsequences, check alternation → O(2ⁿ).
**Optimized:** track `up[i]` (longest ending in an up-swing) and `down[i]`.
```java
class WiggleSubsequence {
    public int wiggleMaxLength(int[] nums) {
        int up = 1, down = 1;
        for (int i = 1; i < nums.length; i++) {
            if (nums[i] > nums[i - 1]) up = down + 1;
            else if (nums[i] < nums[i - 1]) down = up + 1;
        }
        return Math.max(up, down);
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 15. Partition Equal Subset Sum
**Brute force:** try including/excluding every element → O(2ⁿ).
**Optimized:** 0/1 knapsack — `dp[s] = dp[s] || dp[s - num]`, target = sum/2.
```java
class PartitionEqualSubsetSum {
    public boolean canPartition(int[] nums) {
        int sum = Arrays.stream(nums).sum();
        if (sum % 2 != 0) return false;
        int target = sum / 2;
        boolean[] dp = new boolean[target + 1];
        dp[0] = true;
        for (int num : nums) {
            for (int s = target; s >= num; s--) {
                dp[s] = dp[s] || dp[s - num];
            }
        }
        return dp[target];
    }
}
```
**Complexity:** O(n · sum) time, O(sum) space.

---

## 16. Maximum Length of Pair Chain
**Brute force:** LIS-style O(n²) DP after sorting.
**Optimized:** greedy — sort by end value, count non-overlapping chain (interval scheduling).
```java
class PairChain {
    public int findLongestChain(int[][] pairs) {
        Arrays.sort(pairs, (a, b) -> a[1] - b[1]);
        int count = 0, currEnd = Integer.MIN_VALUE;
        for (int[] p : pairs) {
            if (p[0] > currEnd) {
                count++;
                currEnd = p[1];
            }
        }
        return count;
    }
}
```
**Complexity:** O(n log n) time (sort-dominated), O(1) extra space. Beats the O(n²) LIS-style DP.

---

## 17. Best Time to Buy/Sell Stock with Transaction Fee
**Optimized:** state DP tracking `cash` (no stock) and `hold` (holding stock).
```java
class BuySellFee {
    public int maxProfit(int[] prices, int fee) {
        int cash = 0, hold = -prices[0];
        for (int i = 1; i < prices.length; i++) {
            cash = Math.max(cash, hold + prices[i] - fee);
            hold = Math.max(hold, cash - prices[i]);
        }
        return cash;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 18. Delete and Earn
**Optimized:** bucket by value into sums, reduce to House Robber on adjacent values.
```java
class DeleteAndEarn {
    public int deleteAndEarn(int[] nums) {
        int maxVal = Arrays.stream(nums).max().getAsInt();
        int[] sums = new int[maxVal + 1];
        for (int num : nums) sums[num] += num;

        int prev2 = 0, prev1 = 0;
        for (int val : sums) {
            int curr = Math.max(prev1, prev2 + val);
            prev2 = prev1;
            prev1 = curr;
        }
        return prev1;
    }
}
```
**Complexity:** O(n + maxVal) time, O(maxVal) space.

---

## 19. Domino and Tromino Tiling
**Optimized:** `dp[i] = 2*dp[i-1] + dp[i-3]`, derived from tiling recurrence.
```java
class DominoTromino {
    public int numTilings(int n) {
        long MOD = 1_000_000_007;
        if (n <= 2) return n;
        long[] dp = new long[n + 1];
        dp[0] = 1; dp[1] = 1; dp[2] = 2;
        for (int i = 3; i <= n; i++) {
            dp[i] = (2 * dp[i - 1] % MOD + dp[i - 3]) % MOD;
        }
        return (int) dp[n];
    }
}
```
**Complexity:** O(n) time, O(n) space (reducible to O(1)).

---

## 20. Knight Dialer
**Brute force:** DFS trying all knight moves for n hops, no memo → O(8ⁿ) roughly.
**Optimized:** `dp[i][digit] = Σ dp[i-1][neighbor]` for valid knight-move neighbors.
```java
class KnightDialer {
    private static final int[][] MOVES = {
        {4,6}, {6,8}, {7,9}, {4,8}, {0,3,9}, {}, {0,1,7}, {2,6}, {1,3}, {2,4}
    };

    public int knightDialer(int n) {
        long MOD = 1_000_000_007;
        long[] dp = new long[10];
        Arrays.fill(dp, 1);
        for (int step = 1; step < n; step++) {
            long[] next = new long[10];
            for (int digit = 0; digit < 10; digit++) {
                for (int nei : MOVES[digit]) {
                    next[nei] = (next[nei] + dp[digit]) % MOD;
                }
            }
            dp = next;
        }
        long total = 0;
        for (long v : dp) total = (total + v) % MOD;
        return (int) total;
    }
}
```
**Complexity:** O(n) time, O(1) space (fixed 10 digits).

---

## 21. Minimum Cost For Tickets
**Optimized:** `dp[day] = min cost` trying 1/7/30-day passes, skipping non-travel days.
```java
class MinCostTickets {
    public int mincostTickets(int[] days, int[] costs) {
        Set<Integer> travelDays = new HashSet<>();
        for (int d : days) travelDays.add(d);
        int lastDay = days[days.length - 1];
        int[] dp = new int[lastDay + 1];

        for (int day = 1; day <= lastDay; day++) {
            if (!travelDays.contains(day)) {
                dp[day] = dp[day - 1];
                continue;
            }
            int opt1 = dp[day - 1] + costs[0];
            int opt7 = dp[Math.max(0, day - 7)] + costs[1];
            int opt30 = dp[Math.max(0, day - 30)] + costs[2];
            dp[day] = Math.min(opt1, Math.min(opt7, opt30));
        }
        return dp[lastDay];
    }
}
```
**Complexity:** O(lastDay) time, O(lastDay) space.

---

## 22. Partition Array for Maximum Sum
**Optimized:** `dp[i] = max over k≤maxLen of dp[i-k] + k*max(arr[i-k..i-1])`.
```java
class PartitionArrayMaxSum {
    public int maxSumAfterPartitioning(int[] arr, int k) {
        int n = arr.length;
        int[] dp = new int[n + 1];
        for (int i = 1; i <= n; i++) {
            int currMax = 0;
            for (int len = 1; len <= k && i - len >= 0; len++) {
                currMax = Math.max(currMax, arr[i - len]);
                dp[i] = Math.max(dp[i], dp[i - len] + currMax * len);
            }
        }
        return dp[n];
    }
}
```
**Complexity:** O(n·k) time, O(n) space.

---

## 23. Filling Bookcase Shelves
**Optimized:** `dp[i] = min over j of dp[j] + shelfHeight`, trying every valid split.
```java
class FillingBookcaseShelves {
    public int minHeightShelves(int[][] books, int shelfWidth) {
        int n = books.length;
        int[] dp = new int[n + 1];
        Arrays.fill(dp, Integer.MAX_VALUE);
        dp[0] = 0;

        for (int i = 1; i <= n; i++) {
            int width = 0, height = 0;
            for (int j = i; j >= 1; j--) {
                width += books[j - 1][0];
                if (width > shelfWidth) break;
                height = Math.max(height, books[j - 1][1]);
                dp[i] = Math.min(dp[i], dp[j - 1] + height);
            }
        }
        return dp[n];
    }
}
```
**Complexity:** O(n²) time, O(n) space.

---

## 24. Longest Arithmetic Subsequence of Given Difference
**Brute force:** O(n²) LIS-style DP checking all pairs.
**Optimized:** hashmap — `dp[num] = dp[num - difference] + 1`.
```java
class LongestArithSeqGivenDiff {
    public int longestSubsequence(int[] arr, int difference) {
        Map<Integer, Integer> dp = new HashMap<>();
        int maxLen = 1;
        for (int num : arr) {
            int len = dp.getOrDefault(num - difference, 0) + 1;
            dp.put(num, len);
            maxLen = Math.max(maxLen, len);
        }
        return maxLen;
    }
}
```
**Complexity:** O(n) time, O(n) space. Beats the O(n²) generic LAS approach by exploiting the fixed difference.

---

## 25. Greatest Sum Divisible by Three
**Optimized:** `dp[r]` = max sum with remainder r mod 3, updated per element.
```java
class GreatestSumDivisibleByThree {
    public int maxSumDivThree(int[] nums) {
        int[] dp = new int[3]; // dp[r] = max sum with sum % 3 == r
        for (int num : nums) {
            int[] next = dp.clone();
            for (int r = 0; r < 3; r++) {
                if (dp[r] > 0 || r == 0) {
                    int newRem = (dp[r] + num) % 3;
                    next[newRem] = Math.max(next[newRem], dp[r] + num);
                }
            }
            dp = next;
        }
        return dp[0];
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 26. Best Time to Buy and Sell Stock III
**Optimized:** track 4 states — buy1, sell1, buy2, sell2 (at most 2 transactions).
```java
class BuySellStockIII {
    public int maxProfit(int[] prices) {
        int buy1 = Integer.MIN_VALUE, sell1 = 0, buy2 = Integer.MIN_VALUE, sell2 = 0;
        for (int p : prices) {
            buy1 = Math.max(buy1, -p);
            sell1 = Math.max(sell1, buy1 + p);
            buy2 = Math.max(buy2, sell1 - p);
            sell2 = Math.max(sell2, buy2 + p);
        }
        return sell2;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 27. Student Attendance Record II
**Optimized:** `dp[n][absences][consecutiveLates]` state DP.
```java
class StudentAttendanceII {
    public int checkRecord(int n) {
        long MOD = 1_000_000_007;
        // dp[a][l]: number of valid records of current length with a absences, l trailing lates
        long[][] dp = new long[2][3];
        dp[0][0] = 1;
        for (int day = 0; day < n; day++) {
            long[][] next = new long[2][3];
            for (int a = 0; a < 2; a++) {
                for (int l = 0; l < 3; l++) {
                    if (dp[a][l] == 0) continue;
                    // append 'P'
                    next[a][0] = (next[a][0] + dp[a][l]) % MOD;
                    // append 'A'
                    if (a < 1) next[a + 1][0] = (next[a + 1][0] + dp[a][l]) % MOD;
                    // append 'L'
                    if (l < 2) next[a][l + 1] = (next[a][l + 1] + dp[a][l]) % MOD;
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
**Complexity:** O(n) time, O(1) space (fixed 2×3 state).

---

## 28. Decode Ways II
**Optimized:** extends Decode Ways with `*` wildcard, careful case enumeration per transition.
```java
class DecodeWaysII {
    public int numDecodings(String s) {
        long MOD = 1_000_000_007;
        long prev2 = 1, prev1 = ways1(s.charAt(0));
        for (int i = 1; i < s.length(); i++) {
            char c1 = s.charAt(i - 1), c2 = s.charAt(i);
            long curr = (ways1(c2) * prev1) % MOD;
            curr = (curr + ways2(c1, c2) * prev2) % MOD;
            prev2 = prev1;
            prev1 = curr;
        }
        return (int) prev1;
    }

    private long ways1(char c) {
        if (c == '*') return 9;
        if (c == '0') return 0;
        return 1;
    }

    private long ways2(char c1, char c2) {
        if (c1 == '*' && c2 == '*') return 15; // 11-19, 21-26
        if (c1 == '*') return c2 <= '6' ? 2 : 1; // 1x or 2x
        if (c2 == '*') {
            if (c1 == '1') return 9;
            if (c1 == '2') return 6;
            return 0;
        }
        int val = (c1 - '0') * 10 + (c2 - '0');
        return (val >= 10 && val <= 26) ? 1 : 0;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 29. Triples with Bitwise AND Equal To Zero
**Brute force:** try all triples directly → O(n³).
**Optimized:** precompute pairwise AND frequency (O(n²)), then for each element count pairs whose AND shares no bit with it (O(n · 2^16) using frequency map over AND results).
```java
class TriplesBitwiseAndZero {
    public int countTriplets(int[] nums) {
        Map<Integer, Integer> pairAndCount = new HashMap<>();
        for (int a : nums) {
            for (int b : nums) {
                pairAndCount.merge(a & b, 1, Integer::sum);
            }
        }
        int count = 0;
        for (int c : nums) {
            for (Map.Entry<Integer, Integer> entry : pairAndCount.entrySet()) {
                if ((entry.getKey() & c) == 0) count += entry.getValue();
            }
        }
        return count;
    }
}
```
**Complexity:** O(n² + n·D) time where D = distinct AND results (≤ 2^16 for typical constraints), O(n²) space — beats the naive O(n³) triple loop.

---

## 30. Maximum Profit in Job Scheduling
**Brute force:** try include/exclude every job recursively, no memo → O(2ⁿ).
**Optimized:** sort by end time, `dp[i] = max(dp[i-1], profit[i] + dp[latest non-overlapping])` found via binary search.
```java
class MaxProfitJobScheduling {
    public int jobScheduling(int[] startTime, int[] endTime, int[] profit) {
        int n = startTime.length;
        int[][] jobs = new int[n][3];
        for (int i = 0; i < n; i++) jobs[i] = new int[]{startTime[i], endTime[i], profit[i]};
        Arrays.sort(jobs, (a, b) -> a[1] - b[1]);

        int[] dp = new int[n + 1];
        int[] ends = new int[n];
        for (int i = 0; i < n; i++) ends[i] = jobs[i][1];

        for (int i = 1; i <= n; i++) {
            int[] job = jobs[i - 1];
            int idx = upperBound(ends, i - 2, job[0]); // last index with end <= job start
            dp[i] = Math.max(dp[i - 1], job[2] + (idx == -1 ? 0 : dp[idx + 1]));
        }
        return dp[n];
    }

    private int upperBound(int[] ends, int hi, int target) {
        int lo = 0, result = -1;
        while (lo <= hi) {
            int mid = (lo + hi) / 2;
            if (ends[mid] <= target) { result = mid; lo = mid + 1; }
            else hi = mid - 1;
        }
        return result;
    }
}
```
**Complexity:** O(n log n) time (sort + binary search per job), O(n) space.

---

## 31. Minimum Number of Taps to Open to Water a Garden
**Brute force:** try all subsets of taps → O(2ⁿ).
**Optimized:** reduce to Jump Game II — convert taps to max-reach intervals, greedy jump.
```java
class MinTaps {
    public int minTaps(int n, int[] ranges) {
        int[] maxReach = new int[n + 1];
        for (int i = 0; i <= n; i++) {
            int left = Math.max(0, i - ranges[i]);
            int right = Math.min(n, i + ranges[i]);
            maxReach[left] = Math.max(maxReach[left], right);
        }

        int taps = 0, currEnd = 0, farthest = 0;
        for (int i = 0; i <= n; i++) {
            if (i > farthest) return -1;
            if (i > currEnd) {
                taps++;
                currEnd = farthest;
            }
            farthest = Math.max(farthest, maxReach[i]);
        }
        return taps;
    }
}
```
**Complexity:** O(n) time, O(n) space.

---

## 32. Count All Valid Pickup and Delivery Options
**Optimized:** combinatorial recurrence — `dp[i] = dp[i-1] * i * (2i-1)`.
```java
class ValidPickupDelivery {
    public int countOrders(int n) {
        long MOD = 1_000_000_007;
        long result = 1;
        for (int i = 1; i <= n; i++) {
            result = (result * i % MOD) * (2 * i - 1) % MOD;
        }
        return (int) result;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 33. Stone Game III
**Optimized:** `dp[i]` = best score difference achievable from index i onward, trying taking 1/2/3 stones.
```java
class StoneGameIII {
    public String stoneGameIII(int[] stoneValue) {
        int n = stoneValue.length;
        int[] dp = new int[n + 1]; // dp[i] = best score diff (current player - opponent) from i onward
        for (int i = n - 1; i >= 0; i--) {
            int take = 0;
            dp[i] = Integer.MIN_VALUE;
            for (int k = 0; k < 3 && i + k < n; k++) {
                take += stoneValue[i + k];
                dp[i] = Math.max(dp[i], take - dp[i + k + 1]);
            }
        }
        if (dp[0] > 0) return "Alice";
        if (dp[0] < 0) return "Bob";
        return "Tie";
    }
}
```
**Complexity:** O(n) time, O(n) space.

---

## 34. Restore the Array
**Optimized:** `dp[i] = Σ dp[j]` for valid split points, using a sliding window sum for O(n) instead of O(n·k).
```java
class RestoreTheArray {
    public int numberOfArrays(String s, int k) {
        long MOD = 1_000_000_007;
        int n = s.length();
        long[] dp = new long[n + 1];
        dp[n] = 1;
        for (int i = n - 1; i >= 0; i--) {
            if (s.charAt(i) == '0') continue; // no valid number can start with 0
            long num = 0;
            for (int j = i; j < n; j++) {
                num = num * 10 + (s.charAt(j) - '0');
                if (num > k) break;
                dp[i] = (dp[i] + dp[j + 1]) % MOD;
            }
        }
        return (int) dp[0];
    }
}
```
**Complexity:** O(n · log₁₀(k)) time — inner loop bounded by digit length of k, O(n) space.

---

## 35. Form Largest Integer With Digits That Add Up to Target
**Optimized:** knapsack DP tracking max digit-count for each target sum, then greedily build largest number.
```java
class LargestNumberDigitSumTarget {
    public String largestNumber(int[] cost, int target) {
        int[] dp = new int[target + 1];
        Arrays.fill(dp, Integer.MIN_VALUE);
        dp[0] = 0;

        for (int c : cost) {
            for (int t = c; t <= target; t++) {
                dp[t] = Math.max(dp[t], dp[t - c] + 1);
            }
        }
        if (dp[target] < 0) return "0";

        StringBuilder sb = new StringBuilder();
        int t = target;
        for (int digit = 9; digit >= 1; digit--) {
            int c = cost[digit - 1];
            while (t >= c && dp[t] == dp[t - c] + 1) {
                sb.append(digit);
                t -= c;
            }
        }
        return sb.toString();
    }
}
```
**Complexity:** O(9 · target) time, O(target) space.

---

## 36. Stone Game IV
**Optimized:** `dp[i] = true if exists perfect square k² ≤ i with dp[i - k²] == false`.
```java
class StoneGameIV {
    public boolean winnerSquareGame(int n) {
        boolean[] dp = new boolean[n + 1];
        for (int i = 1; i <= n; i++) {
            for (int k = 1; k * k <= i; k++) {
                if (!dp[i - k * k]) { dp[i] = true; break; }
            }
        }
        return dp[n];
    }
}
```
**Complexity:** O(n·√n) time, O(n) space.

---

## 37. Coin Change 2
**Brute force:** recursive try-every-coin without memo → exponential.
**Optimized:** `dp[amount] += dp[amount - coin]`, iterating coins in the OUTER loop (ensures combinations, not permutations, are counted).
```java
class CoinChange2 {
    public int change(int amount, int[] coins) {
        int[] dp = new int[amount + 1];
        dp[0] = 1;
        for (int coin : coins) {           // outer loop over coins = count combinations
            for (int a = coin; a <= amount; a++) {
                dp[a] += dp[a - coin];
            }
        }
        return dp[amount];
    }
}
```
**Complexity:** O(amount · coins) time, O(amount) space.

---

## 🎯 Part 1 Summary Table

| # | Problem | Time | Space |
|---|---|---|---|
| 1 | Climbing Stairs | O(n) | O(1) |
| 2 | Best Time Buy/Sell Stock | O(n) | O(1) |
| 3 | Min Cost Climbing Stairs | O(n) | O(1) |
| 4 | Divisor Game | O(1) | O(1) |
| 5 | Decode Ways | O(n) | O(1) |
| 6 | Unique BST | O(n²) | O(n) |
| 7 | House Robber | O(n) | O(1) |
| 8 | Perfect Squares | O(n√n) | O(n) |
| 9 | Buy/Sell w/ Cooldown | O(n) | O(1) |
| 10 | Coin Change | O(n·k) | O(n) |
| 11 | Counting Bits | O(n) | O(n) |
| 12 | Integer Break | O(n²) | O(n) |
| 13 | Unique Digits Count | O(n) | O(1) |
| 14 | Wiggle Subsequence | O(n) | O(1) |
| 15 | Partition Equal Subset Sum | O(n·sum) | O(sum) |
| 16 | Max Length Pair Chain | O(n log n) | O(1) |
| 17 | Buy/Sell w/ Fee | O(n) | O(1) |
| 18 | Delete and Earn | O(n+max) | O(max) |
| 19 | Domino/Tromino Tiling | O(n) | O(n) |
| 20 | Knight Dialer | O(n) | O(1) |
| 21 | Min Cost Tickets | O(days) | O(days) |
| 22 | Partition Array Max Sum | O(n·k) | O(n) |
| 23 | Filling Bookcase Shelves | O(n²) | O(n) |
| 24 | Longest Arith Subseq (fixed diff) | O(n) | O(n) |
| 25 | Greatest Sum Div by 3 | O(n) | O(1) |
| 26 | Buy/Sell Stock III | O(n) | O(1) |
| 27 | Student Attendance II | O(n) | O(1) |
| 28 | Decode Ways II | O(n) | O(1) |
| 29 | Triples AND = 0 | O(n²) | O(n²) |
| 30 | Max Profit Job Scheduling | O(n log n) | O(n) |
| 31 | Min Taps Water Garden | O(n) | O(n) |
| 32 | Valid Pickup/Delivery | O(n) | O(1) |
| 33 | Stone Game III | O(n) | O(n) |
| 34 | Restore The Array | O(n log k) | O(n) |
| 35 | Largest Number Digit Sum | O(9·target) | O(target) |
| 36 | Stone Game IV | O(n√n) | O(n) |
| 37 | Coin Change 2 | O(n·k) | O(n) |

---

**Next parts (say "continue" for any of these):**
- Part 2: Knapsack (11 problems)
- Part 3: Multi-Dimension DP (28 problems)
- Part 4: Interval DP (17 problems)
- Part 5: Bitmask DP (12 problems)
- Part 6: Digit DP (3 problems)
- Part 7: DP on Trees (8 problems)
- Part 8: String DP (20 problems)
- Part 9: Probability DP (3 problems)
- Part 10: Classic DPs — Kadane's, LCS, LIS, Grid, Prefix Sum, Hashmap Subarray (~50 problems)
- Part 11–16: DP+Tricks, Insertion DP, Graph DP, Memoization, Binary Lifting, Math (~20 problems)
