# DP Solutions — Part 9: Probability DP (Java)
### 3 Problems · Full Problem Statement + Example + Brute Force → Optimized + Complexity

---

## 1. Soup Servings

**Problem:** You have soup A and soup B, each starting with `n` mL. Every operation picks one of 4 equally likely options: serve (100 mL A, 0 B), (75 mL A, 25 mL B), (50 mL A, 50 mL B), (25 mL A, 75 mL B) — capped at what's available if a soup would go negative. Return the probability that soup A empties first, plus half the probability both empty on the exact same operation.

**Example:**
```
Input: n = 50
Output: 0.62500
Explanation: Considering all paths of soup-serving operations, A empties strictly 
first with some probability, and both empty simultaneously with another — 
combining as (P(A first) + 0.5·P(tie)) = 0.625.
```

**Brute force:** simulate every possible sequence of operations without memoizing repeated `(a,b)` states → exponential (branching factor 4 per step).
**Optimized:** memoized recursion `dp[a][b] = probability of the described outcome starting from remaining amounts (a,b)` — with the key practical optimization that for large n, the probability provably converges to 1.0 (soup A's serving sizes are stochastically dominant), so we cap computation and short-circuit.
```java
class SoupServings {
    private Double[][] memo;

    public double soupServings(int n) {
        int m = (int) Math.ceil(n / 25.0); // work in units of 25 mL to shrink state space
        if (m >= 500) return 1.0; // converges to 1.0 for large n — avoids huge/slow DP table
        memo = new Double[m + 1][m + 1];
        return dfs(m, m);
    }

    private double dfs(int a, int b) {
        a = Math.max(a, 0);
        b = Math.max(b, 0);
        if (a == 0 && b == 0) return 0.5;
        if (a == 0) return 1.0;
        if (b == 0) return 0.0;
        if (memo[a][b] != null) return memo[a][b];

        double result = 0.25 * (dfs(a - 4, b) + dfs(a - 3, b - 1) + dfs(a - 2, b - 2) + dfs(a - 1, b - 3));
        memo[a][b] = result;
        return result;
    }
}
```
**Complexity:** O(m²) time and space where `m = n/25` (capped at 500), effectively O(1) practically due to the early-exit convergence check — beats the unbounded exponential brute force.

---

## 2. New 21 Game

**Problem:** Start with 0 points. While your point total is < `k`, draw a uniformly random integer from `1` to `maxPts` and add it to your total, then stop as soon as your total reaches `k` or more. Return the probability your final total is ≤ `n`.

**Example:**
```
Input: n = 10, k = 1, maxPts = 10
Output: 1.00000
Explanation: Since k=1, you draw exactly once (stopping immediately once total >= 1),
and any single draw from 1-10 is automatically ≤ n=10 — probability 1.
```

**Brute force:** recursively branch over every possible draw sequence without memo → exponential (though bounded by k stopping condition, still O(maxPts^(k/1)) in the worst case).
**Optimized:** `dp[i] = probability of having EXACTLY i points at some point during the game (before stopping)`, computed via a SLIDING WINDOW SUM over the last `maxPts` values of dp (since each draw is uniform over that range) — avoids the naive O(k·maxPts) transition cost, reducing to O(k+n).
```java
class New21Game {
    public double new21Game(int n, int k, int maxPts) {
        if (k == 0 || n >= k + maxPts - 1) return 1.0; // can't overshoot past n even in the worst case

        double[] dp = new double[k + maxPts];
        dp[0] = 1.0;
        double windowSum = 1.0; // running sum of dp[i-maxPts..i-1], the valid "draw from" range
        double result = 0.0;

        for (int i = 1; i <= n; i++) {
            dp[i] = windowSum / maxPts;
            if (i < k) windowSum += dp[i];       // still drawing — this state can be drawn FROM
            else result += dp[i];                 // stopped here — counts toward final answer
            if (i - maxPts >= 0 && i - maxPts < k) windowSum -= dp[i - maxPts]; // slide window
        }
        return result;
    }
}
```
**Complexity:** O(n + k) time, O(k + maxPts) space — a major improvement over the naive O(k·maxPts) transition-per-state DP (itself already far better than exponential brute force).

---

## 3. Airplane Seat Assignment Probability

**Problem:** `n` passengers board a plane with `n` seats, each with an assigned seat matching their ticket. The FIRST passenger lost their ticket and picks a random seat. Every subsequent passenger sits in their own seat if available; otherwise, they pick a random unoccupied seat. Return the probability that the LAST (nth) passenger ends up in their own assigned seat.

**Example:**
```
Input: n = 2
Output: 0.50000
Explanation: The first passenger picks their own seat (prob 0.5, then passenger 2 
gets their own seat) or the second passenger's seat (prob 0.5, then passenger 2 
does NOT get their own seat) — overall probability = 0.5.
```

**Brute force:** simulate every possible seating permutation resulting from the random-choice cascade → O(n!) paths.
**Optimized insight:** this is a classic result solvable by induction/symmetry rather than literal DP — for n=1 the answer is trivially 1; for any n≥2, by symmetry the last passenger's own seat and the first passenger's own seat are equally likely to be the "last seat remaining" when it matters, giving exactly 0.5 regardless of n.
```java
class AirplaneSeatAssignmentProbability {
    public double nthPersonGetsNthSeat(int n) {
        return n == 1 ? 1.0 : 0.5;
    }
}
```
**Complexity:** O(1) time, O(1) space — the mathematical insight (provable via induction on the recursive structure of the seating process, which is itself expressible as a DP recurrence `f(n) = f(n-1)` for n≥2 with `f(1)=1`, `f(2)=0.5`) collapses what would otherwise be an O(n!) simulation into a constant-time answer.

---

## 🎯 Part 9 Summary Table

| # | Problem | Time | Space |
|---|---|---|---|
| 1 | Soup Servings | O(m²), m=min(n/25,500) | O(m²) |
| 2 | New 21 Game | O(n+k) | O(k+maxPts) |
| 3 | Airplane Seat Assignment | O(1) | O(1) |

---

**Next: Part 10 — Classic DPs (Kadane's, LCS, LIS, 2D Grid, Prefix Sum, Hashmap Subarray) — the largest remaining category (~50 problems).** Given the size, I'll likely split this into 2-3 sub-parts. Say "continue" to proceed, or name a category to jump to.
