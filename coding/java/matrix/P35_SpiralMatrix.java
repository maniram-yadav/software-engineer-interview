/**
 * LeetCode Top Interview 150 -- #35. Spiral Matrix (Medium)
 *
 * Given an m x n matrix, return all elements in spiral order.
 *
 * Example:
 *   Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
 *   Output: [1,2,3,6,9,8,7,4,5]
 */
public class P35_SpiralMatrix {

    public java.util.List<Integer> spiralOrder(int[][] matrix) {
        java.util.List<Integer> result = new java.util.ArrayList<>();
        if (matrix.length == 0) return result;

        int top = 0, bottom = matrix.length - 1;
        int left = 0, right = matrix[0].length - 1;

        while (top <= bottom && left <= right) {
            for (int c = left; c <= right; c++) result.add(matrix[top][c]);
            top++;
            for (int r = top; r <= bottom; r++) result.add(matrix[r][right]);
            right--;
            if (top <= bottom) {
                for (int c = right; c >= left; c--) result.add(matrix[bottom][c]);
                bottom--;
            }
            if (left <= right) {
                for (int r = bottom; r >= top; r--) result.add(matrix[r][left]);
                left++;
            }
        }
        return result;
    }

    public static void main(String[] args) {
        P35_SpiralMatrix sol = new P35_SpiralMatrix();
        test(sol, new int[][]{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}, new int[]{1, 2, 3, 6, 9, 8, 7, 4, 5});
        test(sol, new int[][]{{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10, 11, 12}}, new int[]{1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7});
        test(sol, new int[][]{{1}}, new int[]{1});
        System.out.println("All tests passed.");
    }

    private static void test(P35_SpiralMatrix sol, int[][] matrix, int[] expected) {
        java.util.List<Integer> actual = sol.spiralOrder(matrix);
        int[] actualArr = actual.stream().mapToInt(Integer::intValue).toArray();
        if (!java.util.Arrays.equals(actualArr, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + actual);
        }
        System.out.println("PASS: " + actual);
    }
}
