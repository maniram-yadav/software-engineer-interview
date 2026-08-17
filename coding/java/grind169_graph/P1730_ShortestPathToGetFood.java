/**
 * Grind 169 -- #1730. Shortest Path to Get Food (Medium)
 *
 * Given a grid with your position (*), food cells (#), obstacles (X), and
 * empty cells (O), return the shortest path length to any food cell, or
 * -1 if unreachable.
 *
 * Example:
 *   Input: grid = [["X","X","X","X","X","X"],["X","*","O","O","O","X"],["X","O","O","#","O","X"],["X","X","X","X","X","X"]]
 *   Output: 3
 */
public class P1730_ShortestPathToGetFood {

    private static final int[][] DIRS = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};

    public int getFood(char[][] grid) {
        int rows = grid.length, cols = grid[0].length;
        int sr = -1, sc = -1;
        outer:
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (grid[r][c] == '*') {
                    sr = r;
                    sc = c;
                    break outer;
                }
            }
        }

        java.util.Queue<int[]> queue = new java.util.LinkedList<>();
        queue.add(new int[]{sr, sc});
        boolean[][] visited = new boolean[rows][cols];
        visited[sr][sc] = true;

        int steps = 0;
        while (!queue.isEmpty()) {
            int size = queue.size();
            for (int i = 0; i < size; i++) {
                int[] cell = queue.poll();
                if (grid[cell[0]][cell[1]] == '#') return steps;
                for (int[] d : DIRS) {
                    int nr = cell[0] + d[0], nc = cell[1] + d[1];
                    if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && !visited[nr][nc] && grid[nr][nc] != 'X') {
                        visited[nr][nc] = true;
                        queue.add(new int[]{nr, nc});
                    }
                }
            }
            steps++;
        }
        return -1;
    }

    public static void main(String[] args) {
        P1730_ShortestPathToGetFood sol = new P1730_ShortestPathToGetFood();
        test(sol, new char[][]{
                {'X', 'X', 'X', 'X', 'X', 'X'},
                {'X', '*', 'O', 'O', 'O', 'X'},
                {'X', 'O', 'O', '#', 'O', 'X'},
                {'X', 'X', 'X', 'X', 'X', 'X'}
        }, 3);
        test(sol, new char[][]{{'X', '*'}, {'X', 'X'}, {'O', '#'}}, -1);
        System.out.println("All tests passed.");
    }

    private static void test(P1730_ShortestPathToGetFood sol, char[][] grid, int expected) {
        int actual = sol.getFood(grid);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
