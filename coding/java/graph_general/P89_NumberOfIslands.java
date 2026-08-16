/**
 * LeetCode Top Interview 150 -- #89. Number of Islands (Medium)
 *
 * Given an m x n 2D binary grid of '1' (land) and '0' (water), return the
 * number of islands (connected land, 4-directionally).
 *
 * Example:
 *   Input: grid = [
 *     ["1","1","0","0","0"],
 *     ["1","1","0","0","0"],
 *     ["0","0","1","0","0"],
 *     ["0","0","0","1","1"]
 *   ]
 *   Output: 3
 */
public class P89_NumberOfIslands {

    public int numIslands(char[][] grid) {
        int count = 0;
        for (int r = 0; r < grid.length; r++) {
            for (int c = 0; c < grid[0].length; c++) {
                if (grid[r][c] == '1') {
                    count++;
                    sink(grid, r, c);
                }
            }
        }
        return count;
    }

    private void sink(char[][] grid, int r, int c) {
        if (r < 0 || r >= grid.length || c < 0 || c >= grid[0].length || grid[r][c] != '1') return;
        grid[r][c] = '0';
        sink(grid, r + 1, c);
        sink(grid, r - 1, c);
        sink(grid, r, c + 1);
        sink(grid, r, c - 1);
    }

    public static void main(String[] args) {
        P89_NumberOfIslands sol = new P89_NumberOfIslands();
        test(sol, new char[][]{
                {'1', '1', '0', '0', '0'},
                {'1', '1', '0', '0', '0'},
                {'0', '0', '1', '0', '0'},
                {'0', '0', '0', '1', '1'}
        }, 3);
        test(sol, new char[][]{{'1'}}, 1);
        test(sol, new char[][]{{'0', '0'}, {'0', '0'}}, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P89_NumberOfIslands sol, char[][] grid, int expected) {
        int actual = sol.numIslands(grid);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
