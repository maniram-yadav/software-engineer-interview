/**
 * LeetCode Top Interview 150 -- #36. Rotate Image (Medium)
 *
 * Given an n x n 2D matrix representing an image, rotate it 90 degrees
 * clockwise, in place.
 *
 * Example:
 *   Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
 *   Output: [[7,4,1],[8,5,2],[9,6,3]]
 */
public class P36_RotateImage {

    public void rotate(int[][] matrix) {
        int n = matrix.length;
        for (int i = 0; i < n; i++) {
            for (int j = i + 1; j < n; j++) {
                int tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
        for (int[] row : matrix) {
            for (int left = 0, right = row.length - 1; left < right; left++, right--) {
                int tmp = row[left];
                row[left] = row[right];
                row[right] = tmp;
            }
        }
    }

    public static void main(String[] args) {
        P36_RotateImage sol = new P36_RotateImage();
        test(sol, new int[][]{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}, new int[][]{{7, 4, 1}, {8, 5, 2}, {9, 6, 3}});
        test(sol, new int[][]{{5}}, new int[][]{{5}});
        test(sol, new int[][]{{1, 2}, {3, 4}}, new int[][]{{3, 1}, {4, 2}});
        System.out.println("All tests passed.");
    }

    private static void test(P36_RotateImage sol, int[][] matrix, int[][] expected) {
        sol.rotate(matrix);
        if (!java.util.Arrays.deepEquals(matrix, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.deepToString(expected) + " but got " + java.util.Arrays.deepToString(matrix));
        }
        System.out.println("PASS: " + java.util.Arrays.deepToString(matrix));
    }
}
