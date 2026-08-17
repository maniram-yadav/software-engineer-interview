/**
 * Grind 169 -- #542. 01 Matrix (Medium)
 *
 * Given an m x n binary matrix, return the distance to the nearest 0 for
 * each cell (multi-source BFS).
 *
 * Example:
 *   Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
 *   Output: [[0,0,0],[0,1,0],[1,2,1]]
 */
public class P542_01Matrix {

    private static final int[][] DIRS = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

    public int[][] updateMatrix(int[][] mat) {
        int rows = mat.length, cols = mat[0].length;
        int[][] dist = new int[rows][cols];
        boolean[][] visited = new boolean[rows][cols];
        java.util.Queue<int[]> queue = new java.util.LinkedList<>();

        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (mat[r][c] == 0) {
                    queue.add(new int[]{r, c});
                    visited[r][c] = true;
                }
            }
        }

        while (!queue.isEmpty()) {
            int[] cell = queue.poll();
            for (int[] d : DIRS) {
                int nr = cell[0] + d[0], nc = cell[1] + d[1];
                if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && !visited[nr][nc]) {
                    dist[nr][nc] = dist[cell[0]][cell[1]] + 1;
                    visited[nr][nc] = true;
                    queue.add(new int[]{nr, nc});
                }
            }
        }
        return dist;
    }

    public static void main(String[] args) {
        P542_01Matrix sol = new P542_01Matrix();
        test(sol, new int[][]{{0, 0, 0}, {0, 1, 0}, {1, 1, 1}}, new int[][]{{0, 0, 0}, {0, 1, 0}, {1, 2, 1}});
        test(sol, new int[][]{{0}}, new int[][]{{0}});
        System.out.println("All tests passed.");
    }

    private static void test(P542_01Matrix sol, int[][] mat, int[][] expected) {
        int[][] actual = sol.updateMatrix(mat);
        if (!java.util.Arrays.deepEquals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.deepToString(expected) + " but got " + java.util.Arrays.deepToString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.deepToString(actual));
    }
}
