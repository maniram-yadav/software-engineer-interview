/**
 * LeetCode Top Interview 150 -- #4. Remove Duplicates from Sorted Array II (Medium)
 *
 * Same as Remove Duplicates from Sorted Array, but each unique element may
 * appear at most twice.
 *
 * Example:
 *   Input: nums = [1,1,1,2,2,3]
 *   Output: 5, nums = [1,1,2,2,3,_]
 */
public class P04_RemoveDuplicatesFromSortedArrayII {

    public int removeDuplicates(int[] nums) {
        if (nums.length <= 2) return nums.length;
        int k = 2;
        for (int i = 2; i < nums.length; i++) {
            if (nums[i] != nums[k - 2]) {
                nums[k++] = nums[i];
            }
        }
        return k;
    }

    public static void main(String[] args) {
        P04_RemoveDuplicatesFromSortedArrayII sol = new P04_RemoveDuplicatesFromSortedArrayII();
        test(sol, new int[]{1, 1, 1, 2, 2, 3}, new int[]{1, 1, 2, 2, 3});
        test(sol, new int[]{0, 0, 1, 1, 1, 1, 2, 3, 3}, new int[]{0, 0, 1, 1, 2, 3, 3});
        test(sol, new int[]{1, 1}, new int[]{1, 1});
        test(sol, new int[]{1}, new int[]{1});
        System.out.println("All tests passed.");
    }

    private static void test(P04_RemoveDuplicatesFromSortedArrayII sol, int[] nums, int[] expected) {
        int k = sol.removeDuplicates(nums);
        int[] actual = java.util.Arrays.copyOf(nums, k);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: length=" + k + " nums=" + java.util.Arrays.toString(actual));
    }
}
