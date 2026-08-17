/**
 * Grind 169 -- #31. Next Permutation (Medium)
 *
 * Given an array of integers representing a permutation, rearrange it in
 * place to the next lexicographically greater permutation; if none
 * exists, rearrange to the lowest order (sorted ascending).
 *
 * Example:
 *   Input: nums = [1,2,3]
 *   Output: [1,3,2]
 */
public class P31_NextPermutation {

    public void nextPermutation(int[] nums) {
        int n = nums.length;
        int i = n - 2;
        while (i >= 0 && nums[i] >= nums[i + 1]) i--;
        if (i >= 0) {
            int j = n - 1;
            while (nums[j] <= nums[i]) j--;
            swap(nums, i, j);
        }
        reverse(nums, i + 1, n - 1);
    }

    private void swap(int[] nums, int i, int j) {
        int t = nums[i];
        nums[i] = nums[j];
        nums[j] = t;
    }

    private void reverse(int[] nums, int left, int right) {
        while (left < right) swap(nums, left++, right--);
    }

    public static void main(String[] args) {
        P31_NextPermutation sol = new P31_NextPermutation();
        test(sol, new int[]{1, 2, 3}, new int[]{1, 3, 2});
        test(sol, new int[]{3, 2, 1}, new int[]{1, 2, 3});
        test(sol, new int[]{1, 1, 5}, new int[]{1, 5, 1});
        System.out.println("All tests passed.");
    }

    private static void test(P31_NextPermutation sol, int[] nums, int[] expected) {
        sol.nextPermutation(nums);
        if (!java.util.Arrays.equals(nums, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(nums));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums));
    }
}
