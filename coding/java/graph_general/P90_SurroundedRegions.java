/**
 * LeetCode Top Interview 150 -- #90. Surrounded Regions (Medium)
 *
 * Given an m x n board of 'X' and 'O', capture (flip to 'X') all regions
 * of 'O' that are fully surrounded and not connected to the border.
 *
 * Example:
 *   Input: board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
 *   Output: [["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
 */
public class P90_SurroundedRegions {

    public void solve(char[][] board) {
        int rows = board.length, cols = board[0].length;

        for (int r = 0; r < rows; r++) {
            markSafe(board, r, 0);
            markSafe(board, r, cols - 1);
        }
        for (int c = 0; c < cols; c++) {
            markSafe(board, 0, c);
            markSafe(board, rows - 1, c);
        }

        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (board[r][c] == 'O') board[r][c] = 'X';
                else if (board[r][c] == '#') board[r][c] = 'O';
            }
        }
    }

    private void markSafe(char[][] board, int r, int c) {
        if (r < 0 || r >= board.length || c < 0 || c >= board[0].length || board[r][c] != 'O') return;
        board[r][c] = '#';
        markSafe(board, r + 1, c);
        markSafe(board, r - 1, c);
        markSafe(board, r, c + 1);
        markSafe(board, r, c - 1);
    }

    public static void main(String[] args) {
        P90_SurroundedRegions sol = new P90_SurroundedRegions();

        char[][] board1 = {
                {'X', 'X', 'X', 'X'},
                {'X', 'O', 'O', 'X'},
                {'X', 'X', 'O', 'X'},
                {'X', 'O', 'X', 'X'}
        };
        char[][] expected1 = {
                {'X', 'X', 'X', 'X'},
                {'X', 'X', 'X', 'X'},
                {'X', 'X', 'X', 'X'},
                {'X', 'O', 'X', 'X'}
        };
        test(sol, board1, expected1);

        char[][] board2 = {{'X'}};
        char[][] expected2 = {{'X'}};
        test(sol, board2, expected2);

        System.out.println("All tests passed.");
    }

    private static void test(P90_SurroundedRegions sol, char[][] board, char[][] expected) {
        sol.solve(board);
        if (!java.util.Arrays.deepEquals(board, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.deepToString(expected) + " but got " + java.util.Arrays.deepToString(board));
        }
        System.out.println("PASS: " + java.util.Arrays.deepToString(board));
    }
}
