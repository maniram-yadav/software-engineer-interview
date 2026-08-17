/**
 * Grind 169 -- #994. Rotting Oranges (Medium)
 *
 * Given a grid where cells are empty (0), fresh orange (1), or rotten
 * orange (2), each minute a fresh orange adjacent to a rotten one becomes
 * rotten. Return the minimum minutes until no cell has a fresh orange, or
 * -1 if impossible.
 *
 * Example:
 *   Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
 *   Output: 4
 */
public class P994_RottingOranges {

    private static final int[][] DIRS = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

    public int orangesRotting(int[][] grid) {
        int rows = grid.length, cols = grid[0].length;
        java.util.Queue<int[]> queue = new java.util.LinkedList<>();
        int fresh = 0;
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (grid[r][c] == 2) queue.add(new int[]{r, c});
                else if (grid[r][c] == 1) fresh++;
            }
        }

        int minutes = 0;
        while (!queue.isEmpty() && fresh > 0) {
            int size = queue.size();
            for (int i = 0; i < size; i++) {
                int[] cell = queue.poll();
                for (int[] d : DIRS) {
                    int nr = cell[0] + d[0], nc = cell[1] + d[1];
                    if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr][nc] == 1) {
                        grid[nr][nc] = 2;
                        fresh--;
                        queue.add(new int[]{nr, nc});
                    }
                }
            }
            minutes++;
        }
        return fresh == 0 ? minutes : -1;
    }

    public static void main(String[] args) {
        P994_RottingOranges sol = new P994_RottingOranges();
        test(sol, new int[][]{{2, 1, 1}, {1, 1, 0}, {0, 1, 1}}, 4);
        test(sol, new int[][]{{0, 2}}, 0);
        test(sol, new int[][]{{2, 1, 1}, {0, 1, 1}, {1, 0, 1}}, -1);
        System.out.println("All tests passed.");
    }

    private static void test(P994_RottingOranges sol, int[][] grid, int expected) {
        int actual = sol.orangesRotting(grid);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
