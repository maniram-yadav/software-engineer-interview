/**
 * Grind 169 -- #51. N-Queens (Hard)
 *
 * Place n queens on an n x n chessboard so that no two attack each other;
 * return all distinct board configurations.
 *
 * Example:
 *   Input: n = 4
 *   Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
 */
public class P51_NQueens {

    public java.util.List<java.util.List<String>> solveNQueens(int n) {
        java.util.List<java.util.List<String>> result = new java.util.ArrayList<>();
        int[] queens = new int[n];
        solve(n, 0, new boolean[n], new boolean[2 * n - 1], new boolean[2 * n - 1], queens, result);
        return result;
    }

    private void solve(int n, int row, boolean[] cols, boolean[] diag1, boolean[] diag2, int[] queens, java.util.List<java.util.List<String>> result) {
        if (row == n) {
            result.add(buildBoard(n, queens));
            return;
        }
        for (int col = 0; col < n; col++) {
            int d1 = row + col, d2 = row - col + n - 1;
            if (cols[col] || diag1[d1] || diag2[d2]) continue;
            cols[col] = diag1[d1] = diag2[d2] = true;
            queens[row] = col;
            solve(n, row + 1, cols, diag1, diag2, queens, result);
            cols[col] = diag1[d1] = diag2[d2] = false;
        }
    }

    private java.util.List<String> buildBoard(int n, int[] queens) {
        java.util.List<String> board = new java.util.ArrayList<>();
        for (int r = 0; r < n; r++) {
            char[] row = new char[n];
            java.util.Arrays.fill(row, '.');
            row[queens[r]] = 'Q';
            board.add(new String(row));
        }
        return board;
    }

    public static void main(String[] args) {
        P51_NQueens sol = new P51_NQueens();

        java.util.List<java.util.List<String>> result4 = sol.solveNQueens(4);
        java.util.Set<java.util.List<String>> actualSet = new java.util.HashSet<>(result4);
        java.util.Set<java.util.List<String>> expectedSet = java.util.Set.of(
                java.util.List.of(".Q..", "...Q", "Q...", "..Q."),
                java.util.List.of("..Q.", "Q...", "...Q", ".Q.."));
        if (!actualSet.equals(expectedSet)) {
            throw new AssertionError("Expected " + expectedSet + " but got " + actualSet);
        }
        System.out.println("PASS: n=4 -> " + result4);

        java.util.List<java.util.List<String>> result1 = sol.solveNQueens(1);
        if (!result1.equals(java.util.List.of(java.util.List.of("Q")))) {
            throw new AssertionError("Expected [[\"Q\"]] but got " + result1);
        }
        System.out.println("PASS: n=1 -> " + result1);

        System.out.println("All tests passed.");
    }
}
