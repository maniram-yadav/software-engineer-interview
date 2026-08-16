/**
 * LeetCode Top Interview 150 -- #144. Unique Paths II (Medium)
 *
 * Given an m x n grid with obstacles (marked 1), find the number of unique
 * paths from top-left to bottom-right moving only down or right.
 *
 * Example:
 *   Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
 *   Output: 2
 */
public class P144_UniquePathsII {

    public int uniquePathsWithObstacles(int[][] obstacleGrid) {
        int rows = obstacleGrid.length, cols = obstacleGrid[0].length;
        int[] dp = new int[cols];
        dp[0] = (obstacleGrid[0][0] == 0) ? 1 : 0;
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (obstacleGrid[r][c] == 1) {
                    dp[c] = 0;
                } else if (c > 0) {
                    dp[c] += dp[c - 1];
                }
            }
        }
        return dp[cols - 1];
    }

    public static void main(String[] args) {
        P144_UniquePathsII sol = new P144_UniquePathsII();
        test(sol, new int[][]{{0, 0, 0}, {0, 1, 0}, {0, 0, 0}}, 2);
        test(sol, new int[][]{{0, 1}, {0, 0}}, 1);
        test(sol, new int[][]{{1}}, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P144_UniquePathsII sol, int[][] grid, int expected) {
        int actual = sol.uniquePathsWithObstacles(grid);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
