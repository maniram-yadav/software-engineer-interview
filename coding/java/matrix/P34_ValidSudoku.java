/**
 * LeetCode Top Interview 150 -- #34. Valid Sudoku (Medium)
 *
 * Determine if a partially filled 9x9 Sudoku board is valid: each row,
 * column, and 3x3 sub-box must contain no repeated digits 1-9 (empty cells
 * are '.', and validity only needs to hold for the filled cells, not
 * solvability).
 *
 * Example:
 *   Input: board = 9x9 grid with some digits and '.'
 *   Output: true
 */
public class P34_ValidSudoku {

    public boolean isValidSudoku(char[][] board) {
        java.util.Set<String> seen = new java.util.HashSet<>();
        for (int r = 0; r < 9; r++) {
            for (int c = 0; c < 9; c++) {
                char val = board[r][c];
                if (val == '.') continue;
                String rowKey = "row" + r + val;
                String colKey = "col" + c + val;
                String boxKey = "box" + (r / 3) + (c / 3) + val;
                if (!seen.add(rowKey) || !seen.add(colKey) || !seen.add(boxKey)) {
                    return false;
                }
            }
        }
        return true;
    }

    public static void main(String[] args) {
        P34_ValidSudoku sol = new P34_ValidSudoku();

        char[][] valid = {
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
        test(sol, valid, true);

        char[][] invalid = {
                {'8', '3', '.', '.', '7', '.', '.', '.', '.'},
                {'6', '.', '.', '1', '9', '5', '.', '.', '.'},
                {'.', '9', '8', '.', '.', '.', '.', '6', '.'},
                {'8', '.', '.', '.', '6', '.', '.', '.', '3'},
                {'4', '.', '.', '8', '.', '3', '.', '.', '1'},
                {'7', '.', '.', '.', '2', '.', '.', '.', '6'},
                {'.', '6', '.', '.', '.', '.', '2', '8', '.'},
                {'.', '.', '.', '4', '1', '9', '.', '.', '5'},
                {'.', '.', '.', '.', '8', '.', '.', '7', '9'}
        };
        test(sol, invalid, false);

        char[][] empty = new char[9][9];
        for (char[] row : empty) java.util.Arrays.fill(row, '.');
        test(sol, empty, true);

        System.out.println("All tests passed.");
    }

    private static void test(P34_ValidSudoku sol, char[][] board, boolean expected) {
        boolean actual = sol.isValidSudoku(board);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
