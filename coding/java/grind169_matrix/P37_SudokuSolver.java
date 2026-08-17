/**
 * Grind 169 -- #37. Sudoku Solver (Hard)
 *
 * Write a program to solve a Sudoku puzzle by filling the empty cells in
 * place (backtracking).
 *
 * Example:
 *   Input: board = partially filled 9x9 Sudoku grid
 *   Output: fully solved 9x9 Sudoku grid
 */
public class P37_SudokuSolver {

    public void solveSudoku(char[][] board) {
        solve(board);
    }

    private boolean solve(char[][] board) {
        for (int r = 0; r < 9; r++) {
            for (int c = 0; c < 9; c++) {
                if (board[r][c] == '.') {
                    for (char d = '1'; d <= '9'; d++) {
                        if (isValid(board, r, c, d)) {
                            board[r][c] = d;
                            if (solve(board)) return true;
                            board[r][c] = '.';
                        }
                    }
                    return false;
                }
            }
        }
        return true;
    }

    private boolean isValid(char[][] board, int row, int col, char d) {
        for (int i = 0; i < 9; i++) {
            if (board[row][i] == d) return false;
            if (board[i][col] == d) return false;
            if (board[3 * (row / 3) + i / 3][3 * (col / 3) + i % 3] == d) return false;
        }
        return true;
    }

    public static void main(String[] args) {
        P37_SudokuSolver sol = new P37_SudokuSolver();

        char[][] board = {
                {'5', '3', '.', '.', '7', '.', '.', '.', '.'},
                {'6', '.', '.', '1', '9', '5', '.', '.', '.'},
                {'.', '9', '8', '.', '.', '.', '.', '6', '.'},
                {'8', '.', '.', '.', '6', '.', '.', '.', '3'},
                {'4', '.', '.', '8', '.', '3', '.', '.', '1'},
                {'7', '.', '.', '.', '2', '.', '.', '.', '6'},
                {'.', '6', '.', '.', '.', '.', '2', '8', '.'},
                {'.', '.', '.', '4', '1', '9', '.', '.', '5'},
                {'.', '.', '.', '.', '8', '.', '.', '7', '9'}
        };
        sol.solveSudoku(board);
        checkSolved(board);
        System.out.println("PASS: sudoku solved -> " + java.util.Arrays.deepToString(board));
        System.out.println("All tests passed.");
    }

    private static void checkSolved(char[][] board) {
        for (int r = 0; r < 9; r++) {
            for (int c = 0; c < 9; c++) {
                if (board[r][c] == '.') {
                    throw new AssertionError("Cell (" + r + "," + c + ") left unfilled");
                }
            }
        }
        for (int r = 0; r < 9; r++) {
            for (int c = 0; c < 9; c++) {
                char d = board[r][c];
                board[r][c] = '.';
                boolean valid = new P37_SudokuSolver().isValid(board, r, c, d);
                board[r][c] = d;
                if (!valid) {
                    throw new AssertionError("Invalid placement at (" + r + "," + c + ")");
                }
            }
        }
    }
}
