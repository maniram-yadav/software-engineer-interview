/**
 * Grind 169 -- #41. First Missing Positive (Hard)
 *
 * Given an unsorted integer array nums, return the smallest missing
 * positive integer, in O(n) time and O(1) extra space.
 *
 * Example:
 *   Input: nums = [3,4,-1,1]
 *   Output: 2
 */
public class P41_FirstMissingPositive {

    public int firstMissingPositive(int[] nums) {
        int n = nums.length;
        for (int i = 0; i < n; i++) {
            while (nums[i] > 0 && nums[i] <= n && nums[nums[i] - 1] != nums[i]) {
                int temp = nums[nums[i] - 1];
                nums[nums[i] - 1] = nums[i];
                nums[i] = temp;
            }
        }
        for (int i = 0; i < n; i++) {
            if (nums[i] != i + 1) return i + 1;
        }
        return n + 1;
    }

    public static void main(String[] args) {
        P41_FirstMissingPositive sol = new P41_FirstMissingPositive();
        test(sol, new int[]{3, 4, -1, 1}, 2);
        test(sol, new int[]{1, 2, 0}, 3);
        test(sol, new int[]{7, 8, 9, 11, 12}, 1);
        System.out.println("All tests passed.");
    }

    private static void test(P41_FirstMissingPositive sol, int[] nums, int expected) {
        int actual = sol.firstMissingPositive(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
    }
}
