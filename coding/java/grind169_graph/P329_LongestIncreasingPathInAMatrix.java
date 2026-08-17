/**
 * Grind 169 -- #329. Longest Increasing Path in a Matrix (Hard)
 *
 * Given an m x n integer matrix, return the length of the longest strictly
 * increasing path (moving in any of 4 directions).
 *
 * Example:
 *   Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
 *   Output: 4   (path 1->2->6->9)
 */
public class P329_LongestIncreasingPathInAMatrix {

    private static final int[][] DIRS = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

    public int longestIncreasingPath(int[][] matrix) {
        int rows = matrix.length, cols = matrix[0].length;
        int[][] memo = new int[rows][cols];
        int best = 0;
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                best = Math.max(best, dfs(matrix, memo, r, c));
            }
        }
        return best;
    }

    private int dfs(int[][] matrix, int[][] memo, int r, int c) {
        if (memo[r][c] != 0) return memo[r][c];
        int max = 1;
        for (int[] d : DIRS) {
            int nr = r + d[0], nc = c + d[1];
            if (nr >= 0 && nr < matrix.length && nc >= 0 && nc < matrix[0].length && matrix[nr][nc] > matrix[r][c]) {
                max = Math.max(max, 1 + dfs(matrix, memo, nr, nc));
            }
        }
        memo[r][c] = max;
        return max;
    }

    public static void main(String[] args) {
        P329_LongestIncreasingPathInAMatrix sol = new P329_LongestIncreasingPathInAMatrix();
        test(sol, new int[][]{{9, 9, 4}, {6, 6, 8}, {2, 1, 1}}, 4);
        test(sol, new int[][]{{1}}, 1);
        test(sol, new int[][]{{3, 4, 5}, {3, 2, 6}, {2, 2, 1}}, 4);
        System.out.println("All tests passed.");
    }

    private static void test(P329_LongestIncreasingPathInAMatrix sol, int[][] matrix, int expected) {
        int actual = sol.longestIncreasingPath(matrix);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
