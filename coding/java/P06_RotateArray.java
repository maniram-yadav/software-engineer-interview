/**
 * LeetCode Top Interview 150 -- #6. Rotate Array (Medium)
 *
 * Rotate an array nums to the right by k steps, in place.
 *
 * Example:
 *   Input: nums = [1,2,3,4,5,6,7], k = 3
 *   Output: [5,6,7,1,2,3,4]
 */
public class P06_RotateArray {

    public void rotate(int[] nums, int k) {
        int n = nums.length;
        k %= n;
        reverse(nums, 0, n - 1);
        reverse(nums, 0, k - 1);
        reverse(nums, k, n - 1);
    }

    private void reverse(int[] nums, int left, int right) {
        while (left < right) {
            int tmp = nums[left];
            nums[left] = nums[right];
            nums[right] = tmp;
            left++;
            right--;
        }
    }

    public static void main(String[] args) {
        P06_RotateArray sol = new P06_RotateArray();
        test(sol, new int[]{1, 2, 3, 4, 5, 6, 7}, 3, new int[]{5, 6, 7, 1, 2, 3, 4});
        test(sol, new int[]{-1, -100, 3, 99}, 2, new int[]{3, 99, -1, -100});
        test(sol, new int[]{1}, 0, new int[]{1});
        test(sol, new int[]{1, 2}, 3, new int[]{2, 1});
        System.out.println("All tests passed.");
    }

    private static void test(P06_RotateArray sol, int[] nums, int k, int[] expected) {
        sol.rotate(nums, k);
        if (!java.util.Arrays.equals(nums, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(nums));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(expected));
    }
}
