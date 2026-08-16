/**
 * LeetCode Top Interview 150 -- #38. Game of Life (Medium)
 *
 * Given an m x n board representing Conway's Game of Life (1 = live, 0 =
 * dead), compute the next state in place using the standard rules (a live
 * cell with 2-3 live neighbors survives; a dead cell with exactly 3 live
 * neighbors becomes alive; otherwise dies/stays dead).
 *
 * Example:
 *   Input: board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
 *   Output: [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]
 */
public class P38_GameOfLife {

    private static final int[][] DIRS = {
            {-1, -1}, {-1, 0}, {-1, 1},
            {0, -1}, {0, 1},
            {1, -1}, {1, 0}, {1, 1}
    };

    public void gameOfLife(int[][] board) {
        int rows = board.length, cols = board[0].length;

        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                int liveNeighbors = 0;
                for (int[] d : DIRS) {
                    int nr = r + d[0], nc = c + d[1];
                    if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && Math.abs(board[nr][nc]) == 1) {
                        liveNeighbors++;
                    }
                }
                if (board[r][c] == 1 && (liveNeighbors < 2 || liveNeighbors > 3)) {
                    board[r][c] = -1;
                } else if (board[r][c] == 0 && liveNeighbors == 3) {
                    board[r][c] = 2;
                }
            }
        }

        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (board[r][c] == -1) board[r][c] = 0;
                else if (board[r][c] == 2) board[r][c] = 1;
            }
        }
    }

    public static void main(String[] args) {
        P38_GameOfLife sol = new P38_GameOfLife();
        test(sol, new int[][]{{0, 1, 0}, {0, 0, 1}, {1, 1, 1}, {0, 0, 0}},
                new int[][]{{0, 0, 0}, {1, 0, 1}, {0, 1, 1}, {0, 1, 0}});
        test(sol, new int[][]{{1, 1}, {1, 0}}, new int[][]{{1, 1}, {1, 1}});
        System.out.println("All tests passed.");
    }

    private static void test(P38_GameOfLife sol, int[][] board, int[][] expected) {
        sol.gameOfLife(board);
        if (!java.util.Arrays.deepEquals(board, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.deepToString(expected) + " but got " + java.util.Arrays.deepToString(board));
        }
        System.out.println("PASS: " + java.util.Arrays.deepToString(board));
    }
}
