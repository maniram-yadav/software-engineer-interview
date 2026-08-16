/**
 * LeetCode Top Interview 150 -- #143. Minimum Path Sum (Medium)
 *
 * Given an m x n grid filled with non-negative numbers, find a path from
 * top-left to bottom-right (moving only down or right) that minimizes the
 * sum of numbers along the path.
 *
 * Example:
 *   Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
 *   Output: 7
 */
public class P143_MinimumPathSum {

    public int minPathSum(int[][] grid) {
        int rows = grid.length, cols = grid[0].length;
        int[] dp = new int[cols];
        dp[0] = grid[0][0];
        for (int c = 1; c < cols; c++) dp[c] = dp[c - 1] + grid[0][c];
        for (int r = 1; r < rows; r++) {
            dp[0] += grid[r][0];
            for (int c = 1; c < cols; c++) {
                dp[c] = Math.min(dp[c - 1], dp[c]) + grid[r][c];
            }
        }
        return dp[cols - 1];
    }

    public static void main(String[] args) {
        P143_MinimumPathSum sol = new P143_MinimumPathSum();
        test(sol, new int[][]{{1, 3, 1}, {1, 5, 1}, {4, 2, 1}}, 7);
        test(sol, new int[][]{{1, 2, 3}, {4, 5, 6}}, 12);
        System.out.println("All tests passed.");
    }

    private static void test(P143_MinimumPathSum sol, int[][] grid, int expected) {
        int actual = sol.minPathSum(grid);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
