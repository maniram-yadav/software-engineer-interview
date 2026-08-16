/**
 * LeetCode Top Interview 150 -- #105. N-Queens II (Hard)
 *
 * Return the number of distinct solutions to the n-queens puzzle (placing
 * n queens on an n x n board so no two attack each other).
 *
 * Example:
 *   Input: n = 4
 *   Output: 2
 */
public class P105_NQueensII {

    public int totalNQueens(int n) {
        return solve(n, 0, new boolean[n], new boolean[2 * n - 1], new boolean[2 * n - 1]);
    }

    private int solve(int n, int row, boolean[] cols, boolean[] diag1, boolean[] diag2) {
        if (row == n) return 1;
        int count = 0;
        for (int col = 0; col < n; col++) {
            int d1 = row + col, d2 = row - col + n - 1;
            if (cols[col] || diag1[d1] || diag2[d2]) continue;
            cols[col] = diag1[d1] = diag2[d2] = true;
            count += solve(n, row + 1, cols, diag1, diag2);
            cols[col] = diag1[d1] = diag2[d2] = false;
        }
        return count;
    }

    public static void main(String[] args) {
        P105_NQueensII sol = new P105_NQueensII();
        test(sol, 4, 2);
        test(sol, 1, 1);
        test(sol, 2, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P105_NQueensII sol, int n, int expected) {
        int actual = sol.totalNQueens(n);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: n=" + n + " -> " + actual);
    }
}
