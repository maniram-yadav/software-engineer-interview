/**
 * LeetCode Top Interview 150 -- #3. Remove Duplicates from Sorted Array (Easy)
 *
 * Given a sorted array nums, remove duplicates in place so each unique
 * element appears once, keeping relative order, and return the new length.
 *
 * Example:
 *   Input: nums = [0,0,1,1,1,2,2,3,3,4]
 *   Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
 */
public class P03_RemoveDuplicatesFromSortedArray {

    public int removeDuplicates(int[] nums) {
        if (nums.length == 0) return 0;
        int k = 1;
        for (int i = 1; i < nums.length; i++) {
            if (nums[i] != nums[k - 1]) {
                nums[k++] = nums[i];
            }
        }
        return k;
    }

    public static void main(String[] args) {
        P03_RemoveDuplicatesFromSortedArray sol = new P03_RemoveDuplicatesFromSortedArray();
        test(sol, new int[]{0, 0, 1, 1, 1, 2, 2, 3, 3, 4}, new int[]{0, 1, 2, 3, 4});
        test(sol, new int[]{1, 1, 2}, new int[]{1, 2});
        test(sol, new int[]{}, new int[]{});
        test(sol, new int[]{1, 2, 3}, new int[]{1, 2, 3});
        System.out.println("All tests passed.");
    }

    private static void test(P03_RemoveDuplicatesFromSortedArray sol, int[] nums, int[] expected) {
        int k = sol.removeDuplicates(nums);
        int[] actual = java.util.Arrays.copyOf(nums, k);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: length=" + k + " nums=" + java.util.Arrays.toString(actual));
    }
}
