/**
 * Grind 169 -- #417. Pacific Atlantic Water Flow (Medium)
 *
 * Given an m x n grid of heights, find all cells from which water can flow
 * to both the Pacific (top/left edges) and Atlantic (bottom/right edges)
 * oceans (water flows to equal or lower neighbors).
 *
 * Example:
 *   Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
 *   Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
 */
public class P417_PacificAtlanticWaterFlow {

    private static final int[][] DIRS = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

    public java.util.List<java.util.List<Integer>> pacificAtlantic(int[][] heights) {
        int rows = heights.length, cols = heights[0].length;
        boolean[][] pacific = new boolean[rows][cols];
        boolean[][] atlantic = new boolean[rows][cols];

        for (int r = 0; r < rows; r++) {
            dfs(heights, pacific, r, 0);
            dfs(heights, atlantic, r, cols - 1);
        }
        for (int c = 0; c < cols; c++) {
            dfs(heights, pacific, 0, c);
            dfs(heights, atlantic, rows - 1, c);
        }

        java.util.List<java.util.List<Integer>> result = new java.util.ArrayList<>();
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (pacific[r][c] && atlantic[r][c]) result.add(java.util.List.of(r, c));
            }
        }
        return result;
    }

    private void dfs(int[][] heights, boolean[][] visited, int r, int c) {
        visited[r][c] = true;
        for (int[] d : DIRS) {
            int nr = r + d[0], nc = c + d[1];
            if (nr >= 0 && nr < heights.length && nc >= 0 && nc < heights[0].length
                    && !visited[nr][nc] && heights[nr][nc] >= heights[r][c]) {
                dfs(heights, visited, nr, nc);
            }
        }
    }

    public static void main(String[] args) {
        P417_PacificAtlanticWaterFlow sol = new P417_PacificAtlanticWaterFlow();

        int[][] heights = {
                {1, 2, 2, 3, 5},
                {3, 2, 3, 4, 4},
                {2, 4, 5, 3, 1},
                {6, 7, 1, 4, 5},
                {5, 1, 1, 2, 4}
        };
        int[][] expected = {{0, 4}, {1, 3}, {1, 4}, {2, 2}, {3, 0}, {3, 1}, {4, 0}};
        test(sol, heights, expected);

        System.out.println("All tests passed.");
    }

    private static void test(P417_PacificAtlanticWaterFlow sol, int[][] heights, int[][] expected) {
        java.util.List<java.util.List<Integer>> actual = sol.pacificAtlantic(heights);
        java.util.Set<java.util.List<Integer>> actualSet = new java.util.HashSet<>(actual);
        java.util.Set<java.util.List<Integer>> expectedSet = new java.util.HashSet<>();
        for (int[] e : expected) expectedSet.add(java.util.List.of(e[0], e[1]));
        if (!actualSet.equals(expectedSet)) {
            throw new AssertionError("Expected " + expectedSet + " but got " + actualSet);
        }
        System.out.println("PASS: " + actual.size() + " cells match");
    }
}
