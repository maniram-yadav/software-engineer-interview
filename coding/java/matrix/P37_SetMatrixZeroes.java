/**
 * LeetCode Top Interview 150 -- #37. Set Matrix Zeroes (Medium)
 *
 * Given an m x n matrix, if an element is 0, set its entire row and column
 * to 0, in place.
 *
 * Example:
 *   Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
 *   Output: [[1,0,1],[0,0,0],[1,0,1]]
 */
public class P37_SetMatrixZeroes {

    public void setZeroes(int[][] matrix) {
        int rows = matrix.length, cols = matrix[0].length;
        boolean firstRowZero = false, firstColZero = false;

        for (int c = 0; c < cols; c++) {
            if (matrix[0][c] == 0) firstRowZero = true;
        }
        for (int r = 0; r < rows; r++) {
            if (matrix[r][0] == 0) firstColZero = true;
        }

        for (int r = 1; r < rows; r++) {
            for (int c = 1; c < cols; c++) {
                if (matrix[r][c] == 0) {
                    matrix[r][0] = 0;
                    matrix[0][c] = 0;
                }
            }
        }

        for (int r = 1; r < rows; r++) {
            for (int c = 1; c < cols; c++) {
                if (matrix[r][0] == 0 || matrix[0][c] == 0) {
                    matrix[r][c] = 0;
                }
            }
        }

        if (firstRowZero) {
            for (int c = 0; c < cols; c++) matrix[0][c] = 0;
        }
        if (firstColZero) {
            for (int r = 0; r < rows; r++) matrix[r][0] = 0;
        }
    }

    public static void main(String[] args) {
        P37_SetMatrixZeroes sol = new P37_SetMatrixZeroes();
        test(sol, new int[][]{{1, 1, 1}, {1, 0, 1}, {1, 1, 1}}, new int[][]{{1, 0, 1}, {0, 0, 0}, {1, 0, 1}});
        test(sol, new int[][]{{0, 1, 2, 0}, {3, 4, 5, 2}, {1, 3, 1, 5}}, new int[][]{{0, 0, 0, 0}, {0, 4, 5, 0}, {0, 3, 1, 0}});
        System.out.println("All tests passed.");
    }

    private static void test(P37_SetMatrixZeroes sol, int[][] matrix, int[][] expected) {
        sol.setZeroes(matrix);
        if (!java.util.Arrays.deepEquals(matrix, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.deepToString(expected) + " but got " + java.util.Arrays.deepToString(matrix));
        }
        System.out.println("PASS: " + java.util.Arrays.deepToString(matrix));
    }
}
