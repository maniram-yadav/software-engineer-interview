/**
 * Grind 169 -- #977. Squares of a Sorted Array (Easy)
 *
 * Given an integer array nums sorted in non-decreasing order, return an
 * array of the squares of each number, also sorted in non-decreasing
 * order.
 *
 * Example:
 *   Input: nums = [-4,-1,0,3,10]
 *   Output: [0,1,9,16,100]
 */
public class P977_SquaresOfASortedArray {

    public int[] sortedSquares(int[] nums) {
        int n = nums.length;
        int[] result = new int[n];
        int left = 0, right = n - 1;
        for (int i = n - 1; i >= 0; i--) {
            int leftSq = nums[left] * nums[left];
            int rightSq = nums[right] * nums[right];
            if (leftSq > rightSq) {
                result[i] = leftSq;
                left++;
            } else {
                result[i] = rightSq;
                right--;
            }
        }
        return result;
    }

    public static void main(String[] args) {
        P977_SquaresOfASortedArray sol = new P977_SquaresOfASortedArray();
        test(sol, new int[]{-4, -1, 0, 3, 10}, new int[]{0, 1, 9, 16, 100});
        test(sol, new int[]{-7, -3, 2, 3, 11}, new int[]{4, 9, 9, 49, 121});
        System.out.println("All tests passed.");
    }

    private static void test(P977_SquaresOfASortedArray sol, int[] nums, int[] expected) {
        int[] actual = sol.sortedSquares(nums);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(actual));
    }
}
