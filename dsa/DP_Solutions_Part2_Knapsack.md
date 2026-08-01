# DP Solutions — Part 2: Knapsack (Java)
### 11 Problems · Brute Force Note + Optimized Approach + Complexity

---

## 1. House Robber II
**Problem:** Houses arranged in a circle — first and last are adjacent.
**Brute force:** try all subsets respecting adjacency and the circular constraint → O(2ⁿ).
**Optimized:** run linear House Robber twice — once excluding the last house, once excluding the first — take the max (breaks the circular dependency into two linear subproblems).
```java
class HouseRobberII {
    public int rob(int[] nums) {
        int n = nums.length;
        if (n == 1) return nums[0];
        return Math.max(robLinear(nums, 0, n - 2), robLinear(nums, 1, n - 1));
    }

    private int robLinear(int[] nums, int start, int end) {
        int prev2 = 0, prev1 = 0;
        for (int i = start; i <= end; i++) {
            int curr = Math.max(prev1, prev2 + nums[i]);
            prev2 = prev1;
            prev1 = curr;
        }
        return prev1;
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 2. Ones and Zeroes
**Problem:** Pick max number of strings from list, using at most `m` zeros and `n` ones total.
**Brute force:** try every subset of strings, check zero/one budget → O(2^k · L).
**Optimized:** 2D 0/1 knapsack — `dp[i][j] = max strings using ≤i zeros, ≤j ones`.
```java
class OnesAndZeroes {
    public int findMaxForm(String[] strs, int m, int n) {
        int[][] dp = new int[m + 1][n + 1];
        for (String s : strs) {
            int zeros = 0, ones = 0;
            for (char c : s.toCharArray()) {
                if (c == '0') zeros++; else ones++;
            }
            for (int i = m; i >= zeros; i--) {
                for (int j = n; j >= ones; j--) {
                    dp[i][j] = Math.max(dp[i][j], dp[i - zeros][j - ones] + 1);
                }
            }
        }
        return dp[m][n];
    }
}
```
**Complexity:** O(k · m · n) time (k = number of strings), O(m·n) space. This is a 2-constraint 0/1 knapsack — same pattern as classic knapsack but with a 2D capacity.

---

## 3. Target Sum
**Problem:** Assign + or − to each number so the sum equals target; count ways.
**Brute force:** try both signs for every number recursively, no memo → O(2ⁿ).
**Optimized insight:** reduces to subset-sum knapsack. If P = positive subset sum, N = negative subset sum: P - N = target, P + N = totalSum → P = (target + totalSum) / 2. Count subsets summing to P.
```java
class TargetSum {
    public int findTargetSumWays(int[] nums, int target) {
        int totalSum = Arrays.stream(nums).sum();
        if (Math.abs(target) > totalSum || (totalSum + target) % 2 != 0) return 0;
        int P = (totalSum + target) / 2;

        int[] dp = new int[P + 1];
        dp[0] = 1;
        for (int num : nums) {
            for (int s = P; s >= num; s--) {
                dp[s] += dp[s - num];
            }
        }
        return dp[P];
    }
}
```
**Complexity:** O(n · P) time, O(P) space — a major improvement over O(2ⁿ) brute-force sign enumeration.

---

## 4. Shopping Offers
**Problem:** Buy items either individually or via special offers (bundles), minimize total cost.
**Brute force / actual approach:** this problem's "brute force" and "optimized" are the same technique — DFS + memoization over the needs-vector state, since the state space (bounded small quantities) is what makes memoization tractable (a pure iterative DP table isn't natural here due to the multi-dimensional continuous-ish state).
```java
class ShoppingOffers {
    public int shoppingOffers(List<Integer> price, List<List<Integer>> special, List<Integer> needs) {
        Map<String, Integer> memo = new HashMap<>();
        return dfs(price, special, needs, memo);
    }

    private int dfs(List<Integer> price, List<List<Integer>> special, List<Integer> needs, Map<String, Integer> memo) {
        String key = needs.toString();
        if (memo.containsKey(key)) return memo.get(key);

        // baseline: buy everything individually
        int minCost = 0;
        for (int i = 0; i < needs.size(); i++) minCost += needs.get(i) * price.get(i);

        for (List<Integer> offer : special) {
            List<Integer> newNeeds = new ArrayList<>();
            boolean valid = true;
            for (int i = 0; i < needs.size(); i++) {
                int remaining = needs.get(i) - offer.get(i);
                if (remaining < 0) { valid = false; break; }
                newNeeds.add(remaining);
            }
            if (valid) {
                int offerCost = offer.get(offer.size() - 1) + dfs(price, special, newNeeds, memo);
                minCost = Math.min(minCost, offerCost);
            }
        }
        memo.put(key, minCost);
        return minCost;
    }
}
```
**Complexity:** O(states · offers · items) where states = distinct needs-vectors reachable (bounded in practice by small item counts), pseudo-polynomial. Memoization is essential — without it, this is exponential in the number of offers applied.

---

## 5. 2 Keys Keyboard
**Problem:** Starting with 1 'A', use Copy-All and Paste to reach exactly n 'A's in min operations.
**Brute force:** BFS/DFS over (count, clipboard) states without exploiting structure → can blow up.
**Optimized insight:** this is prime factorization — answer = sum of prime factors of n (each factor f means "copy then paste f-1 times").
```java
class TwoKeysKeyboard {
    public int minSteps(int n) {
        int result = 0;
        for (int factor = 2; factor <= n; factor++) {
            while (n % factor == 0) {
                result += factor;
                n /= factor;
            }
        }
        return result;
    }
}
```
**Complexity:** O(√n) time (trial division), O(1) space. (A DP formulation `dp[i] = min steps` also works in O(n√n), but the factorization insight is strictly better.)

---

## 6. Minimum Swaps to Make Sequences Increasing
**Problem:** Two arrays, swap `nums1[i]` and `nums2[i]` at same index to make both strictly increasing, minimize swaps.
**Optimized:** DP with two states per index — `keep[i]` (no swap at i) and `swap[i]` (swap at i).
```java
class MinSwapsIncreasing {
    public int minSwap(int[] nums1, int[] nums2) {
        int n = nums1.length;
        int keep = 0, swap = 1;
        for (int i = 1; i < n; i++) {
            int newKeep = Integer.MAX_VALUE, newSwap = Integer.MAX_VALUE;
            if (nums1[i] > nums1[i - 1] && nums2[i] > nums2[i - 1]) {
                newKeep = Math.min(newKeep, keep);
                newSwap = Math.min(newSwap, swap + 1);
            }
            if (nums1[i] > nums2[i - 1] && nums2[i] > nums1[i - 1]) {
                newKeep = Math.min(newKeep, swap);
                newSwap = Math.min(newSwap, keep + 1);
            }
            keep = newKeep;
            swap = newSwap;
        }
        return Math.min(keep, swap);
    }
}
```
**Complexity:** O(n) time, O(1) space.

---

## 7. Best Team With No Conflicts
**Problem:** Pick max-score subset of players such that no younger player has strictly higher score than an older one.
**Brute force:** try every subset, validate constraint → O(2ⁿ).
**Optimized:** sort by age (then score), then it's LIS-style DP — `dp[i] = scores[i] + max(dp[j])` for valid j < i.
```java
class BestTeamNoConflicts {
    public int bestTeamScore(int[] scores, int[] ages) {
        int n = scores.length;
        Integer[] idx = new Integer[n];
        for (int i = 0; i < n; i++) idx[i] = i;
        Arrays.sort(idx, (a, b) -> ages[a] != ages[b] ? ages[a] - ages[b] : scores[a] - scores[b]);

        int[] dp = new int[n];
        int best = 0;
        for (int i = 0; i < n; i++) {
            dp[i] = scores[idx[i]];
            for (int j = 0; j < i; j++) {
                if (scores[idx[j]] <= scores[idx[i]]) {
                    dp[i] = Math.max(dp[i], dp[j] + scores[idx[i]]);
                }
            }
            best = Math.max(best, dp[i]);
        }
        return best;
    }
}
```
**Complexity:** O(n²) time (LIS-style DP after sort), O(n) space.

---

## 8. Profitable Schemes
**Problem:** Count subsets of crimes achieving profit ≥ minProfit while using ≤ n total members.
**Brute force:** try every subset of crimes → O(2^crimes).
**Optimized:** 2D knapsack — `dp[people][profit] = number of schemes`, profit dimension capped at minProfit (overflow profit collapses to the minProfit bucket).
```java
class ProfitableSchemes {
    public int profitableSchemes(int n, int minProfit, int[] group, int[] profit) {
        long MOD = 1_000_000_007;
        long[][] dp = new long[n + 1][minProfit + 1];
        dp[0][0] = 1;

        for (int i = 0; i < group.length; i++) {
            int members = group[i], gain = profit[i];
            for (int people = n; people >= members; people--) {
                for (int p = minProfit; p >= 0; p--) {
                    int newProfit = Math.min(minProfit, p + gain);
                    dp[people][newProfit] = (dp[people][newProfit] + dp[people - members][p]) % MOD;
                }
            }
        }

        long total = 0;
        for (int people = 0; people <= n; people++) total = (total + dp[people][minProfit]) % MOD;
        return (int) total;
    }
}
```
**Complexity:** O(crimes · n · minProfit) time, O(n · minProfit) space.

---

## 9. Tallest Billboard
**Problem:** Choose subset of rods to split into two groups with equal height sum, maximize that height.
**Brute force:** try every subset assignment to left/right/unused → O(3ⁿ).
**Optimized:** DP over the *difference* between two sides — `dp[diff] = max(shorter side height)` for a given height difference.
```java
class TallestBillboard {
    public int tallestBillboard(int[] rods) {
        Map<Integer, Integer> dp = new HashMap<>(); // diff -> max of the shorter side
        dp.put(0, 0);

        for (int rod : rods) {
            Map<Integer, Integer> next = new HashMap<>(dp);
            for (Map.Entry<Integer, Integer> entry : dp.entrySet()) {
                int diff = entry.getKey(), shorter = entry.getValue();
                // add rod to taller side
                int newDiff1 = diff + rod;
                next.put(newDiff1, Math.max(next.getOrDefault(newDiff1, 0), shorter));
                // add rod to shorter side
                int newDiff2 = Math.abs(diff - rod);
                int newShorter2 = shorter + Math.min(rod, diff);
                next.put(newDiff2, Math.max(next.getOrDefault(newDiff2, 0), newShorter2));
            }
            dp = next;
        }
        return dp.getOrDefault(0, 0);
    }
}
```
**Complexity:** O(n · D) time where D = number of distinct achievable differences (bounded by total rod sum), O(D) space — a massive improvement over O(3ⁿ) brute enumeration.

---

## 10. Pizza With 3n Slices
**Problem:** Pick n slices (you, then friend, then you alternating in a circle) — maximize your total picking optimally, equivalent to: from the circular array, pick n non-adjacent slices maximizing sum (with the circular constraint handled like House Robber II).
**Brute force:** try all valid pick combinations → exponential.
**Optimized:** break circle into two linear cases (exclude first, exclude last), each solved via "max sum picking n non-adjacent elements" DP.
```java
class PizzaWith3NSlices {
    public int maxSizeSlices(int[] slices) {
        int n = slices.length / 3;
        return Math.max(
            maxSumNonAdjacent(slices, 0, slices.length - 2, n),
            maxSumNonAdjacent(slices, 1, slices.length - 1, n)
        );
    }

    private int maxSumNonAdjacent(int[] slices, int start, int end, int k) {
        int len = end - start + 1;
        int[][] dp = new int[len + 1][k + 1]; // dp[i][j] = max sum picking j non-adjacent from first i
        for (int i = 1; i <= len; i++) {
            for (int j = 1; j <= k; j++) {
                int skip = dp[i - 1][j];
                int take = (i >= 2 ? dp[i - 2][j - 1] : (j == 1 ? 0 : Integer.MIN_VALUE));
                if (take != Integer.MIN_VALUE) take += slices[start + i - 1];
                dp[i][j] = Math.max(skip, take);
            }
        }
        return dp[len][k];
    }
}
```
**Complexity:** O(n²) time (each linear pass is O(len·k) ≈ O(n²)), O(n²) space.

---

## 11. Reducing Dishes
**Problem:** Order dishes to maximize Σ(time[i] × satisfaction[i]) where time increments per dish cooked in order.
**Brute force:** try all orderings/subsets → O(n!) or O(2ⁿ).
**Optimized insight:** sort ascending, then greedily decide whether including each dish (from largest satisfaction down) increases total — equivalent to a running-sum DP.
```java
class ReducingDishes {
    public int maxSatisfaction(int[] satisfaction) {
        Arrays.sort(satisfaction);
        int n = satisfaction.length;
        int total = 0, runningSum = 0;

        for (int i = n - 1; i >= 0; i--) {
            runningSum += satisfaction[i];
            if (runningSum <= 0) break;      // adding more (smaller) dishes stops helping
            total += runningSum;
        }
        return total;
    }
}
```
**Complexity:** O(n log n) time (sort-dominated), O(1) extra space. This greedy-with-running-sum works because once `runningSum` (the marginal contribution of adding one more dish at the front) turns non-positive, no further prefix extension can help — proven via exchange argument, equivalent in spirit to a DP over "how many trailing dishes to include."

---

## 🎯 Part 2 Summary Table

| # | Problem | Time | Space |
|---|---|---|---|
| 1 | House Robber II | O(n) | O(1) |
| 2 | Ones and Zeroes | O(k·m·n) | O(m·n) |
| 3 | Target Sum | O(n·P) | O(P) |
| 4 | Shopping Offers | O(states·offers·items) | O(states) |
| 5 | 2 Keys Keyboard | O(√n) | O(1) |
| 6 | Min Swaps Increasing | O(n) | O(1) |
| 7 | Best Team No Conflicts | O(n²) | O(n) |
| 8 | Profitable Schemes | O(crimes·n·minProfit) | O(n·minProfit) |
| 9 | Tallest Billboard | O(n·D) | O(D) |
| 10 | Pizza With 3N Slices | O(n²) | O(n²) |
| 11 | Reducing Dishes | O(n log n) | O(1) |

---

**Next: Part 3 — Multi-Dimension DP (28 problems).** Say "continue" to proceed, or name a category to jump to.
