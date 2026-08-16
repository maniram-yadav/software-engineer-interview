/**
 * LeetCode Top Interview 150 -- #115. Search a 2D Matrix (Medium)
 *
 * Given an m x n matrix where each row is sorted ascending and the first
 * integer of each row is greater than the last integer of the previous
 * row, determine if target exists, in O(log(mn)).
 *
 * Example:
 *   Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
 *   Output: true
 */
public class P115_Search2DMatrix {

    public boolean searchMatrix(int[][] matrix, int target) {
        int rows = matrix.length, cols = matrix[0].length;
        int left = 0, right = rows * cols - 1;
        while (left <= right) {
            int mid = left + (right - left) / 2;
            int val = matrix[mid / cols][mid % cols];
            if (val == target) return true;
            else if (val < target) left = mid + 1;
            else right = mid - 1;
        }
        return false;
    }

    public static void main(String[] args) {
        P115_Search2DMatrix sol = new P115_Search2DMatrix();
        int[][] matrix = {{1, 3, 5, 7}, {10, 11, 16, 20}, {23, 30, 34, 60}};
        test(sol, matrix, 3, true);
        test(sol, matrix, 13, false);
        test(sol, matrix, 60, true);
        System.out.println("All tests passed.");
    }

    private static void test(P115_Search2DMatrix sol, int[][] matrix, int target, boolean expected) {
        boolean actual = sol.searchMatrix(matrix, target);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: target=" + target + " -> " + actual);
    }
}
