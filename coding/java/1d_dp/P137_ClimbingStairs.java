/**
 * LeetCode Top Interview 150 -- #137. Climbing Stairs (Easy)
 *
 * You're climbing a staircase of n steps, taking 1 or 2 steps at a time.
 * Return the number of distinct ways to reach the top.
 *
 * Example:
 *   Input: n = 3
 *   Output: 3   (1+1+1, 1+2, 2+1)
 */
public class P137_ClimbingStairs {

    public int climbStairs(int n) {
        if (n <= 2) return n;
        int prev2 = 1, prev1 = 2;
        for (int i = 3; i <= n; i++) {
            int cur = prev1 + prev2;
            prev2 = prev1;
            prev1 = cur;
        }
        return prev1;
    }

    public static void main(String[] args) {
        P137_ClimbingStairs sol = new P137_ClimbingStairs();
        test(sol, 3, 3);
        test(sol, 2, 2);
        test(sol, 1, 1);
        test(sol, 5, 8);
        System.out.println("All tests passed.");
    }

    private static void test(P137_ClimbingStairs sol, int n, int expected) {
        int actual = sol.climbStairs(n);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: n=" + n + " -> " + actual);
    }
}
