/**
 * Grind 169 -- #62. Unique Paths (Medium)
 *
 * A robot on an m x n grid starts at the top-left corner and can only move
 * down or right. Return the number of unique paths to the bottom-right
 * corner.
 *
 * Example:
 *   Input: m = 3, n = 7
 *   Output: 28
 */
public class P62_UniquePaths {

    public int uniquePaths(int m, int n) {
        int[] dp = new int[n];
        java.util.Arrays.fill(dp, 1);
        for (int i = 1; i < m; i++) {
            for (int j = 1; j < n; j++) {
                dp[j] += dp[j - 1];
            }
        }
        return dp[n - 1];
    }

    public static void main(String[] args) {
        P62_UniquePaths sol = new P62_UniquePaths();
        test(sol, 3, 7, 28);
        test(sol, 3, 2, 3);
        test(sol, 1, 1, 1);
        System.out.println("All tests passed.");
    }

    private static void test(P62_UniquePaths sol, int m, int n, int expected) {
        int actual = sol.uniquePaths(m, n);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: m=" + m + " n=" + n + " -> " + actual);
    }
}
