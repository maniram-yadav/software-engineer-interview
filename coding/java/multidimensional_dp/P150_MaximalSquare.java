/**
 * LeetCode Top Interview 150 -- #150. Maximal Square (Medium)
 *
 * Given an m x n binary matrix filled with 0s and 1s, find the largest
 * square containing only 1s, and return its area.
 *
 * Example:
 *   Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
 *   Output: 4
 */
public class P150_MaximalSquare {

    public int maximalSquare(char[][] matrix) {
        int rows = matrix.length, cols = matrix[0].length;
        int[][] dp = new int[rows + 1][cols + 1];
        int maxSide = 0;
        for (int r = 1; r <= rows; r++) {
            for (int c = 1; c <= cols; c++) {
                if (matrix[r - 1][c - 1] == '1') {
                    dp[r][c] = Math.min(dp[r - 1][c], Math.min(dp[r][c - 1], dp[r - 1][c - 1])) + 1;
                    maxSide = Math.max(maxSide, dp[r][c]);
                }
            }
        }
        return maxSide * maxSide;
    }

    public static void main(String[] args) {
        P150_MaximalSquare sol = new P150_MaximalSquare();
        test(sol, new char[][]{
                {'1', '0', '1', '0', '0'},
                {'1', '0', '1', '1', '1'},
                {'1', '1', '1', '1', '1'},
                {'1', '0', '0', '1', '0'}
        }, 4);
        test(sol, new char[][]{{'0', '1'}, {'1', '0'}}, 1);
        test(sol, new char[][]{{'0'}}, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P150_MaximalSquare sol, char[][] matrix, int expected) {
        int actual = sol.maximalSquare(matrix);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
