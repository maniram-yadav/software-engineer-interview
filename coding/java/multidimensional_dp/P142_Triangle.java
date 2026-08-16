/**
 * LeetCode Top Interview 150 -- #142. Triangle (Medium)
 *
 * Given a triangle array, return the minimum path sum from top to bottom
 * (each step moves to an adjacent number on the row below).
 *
 * Example:
 *   Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
 *   Output: 11   (2 + 3 + 5 + 1)
 */
public class P142_Triangle {

    public int minimumTotal(java.util.List<java.util.List<Integer>> triangle) {
        int n = triangle.size();
        int[] dp = new int[n + 1];
        for (int i = n - 1; i >= 0; i--) {
            java.util.List<Integer> row = triangle.get(i);
            for (int j = 0; j <= i; j++) {
                dp[j] = row.get(j) + Math.min(dp[j], dp[j + 1]);
            }
        }
        return dp[0];
    }

    public static void main(String[] args) {
        P142_Triangle sol = new P142_Triangle();
        test(sol, java.util.List.of(
                java.util.List.of(2),
                java.util.List.of(3, 4),
                java.util.List.of(6, 5, 7),
                java.util.List.of(4, 1, 8, 3)), 11);
        test(sol, java.util.List.of(java.util.List.of(-10)), -10);
        System.out.println("All tests passed.");
    }

    private static void test(P142_Triangle sol, java.util.List<java.util.List<Integer>> triangle, int expected) {
        int actual = sol.minimumTotal(triangle);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + triangle + " -> " + actual);
    }
}
