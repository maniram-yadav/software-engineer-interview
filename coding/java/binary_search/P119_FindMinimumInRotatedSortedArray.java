/**
 * LeetCode Top Interview 150 -- #119. Find Minimum in Rotated Sorted Array (Medium)
 *
 * Given a rotated sorted array of unique elements, find the minimum
 * element in O(log n).
 *
 * Example:
 *   Input: nums = [3,4,5,1,2]
 *   Output: 1
 */
public class P119_FindMinimumInRotatedSortedArray {

    public int findMin(int[] nums) {
        int left = 0, right = nums.length - 1;
        while (left < right) {
            int mid = left + (right - left) / 2;
            if (nums[mid] > nums[right]) left = mid + 1;
            else right = mid;
        }
        return nums[left];
    }

    public static void main(String[] args) {
        P119_FindMinimumInRotatedSortedArray sol = new P119_FindMinimumInRotatedSortedArray();
        test(sol, new int[]{3, 4, 5, 1, 2}, 1);
        test(sol, new int[]{4, 5, 6, 7, 0, 1, 2}, 0);
        test(sol, new int[]{11, 13, 15, 17}, 11);
        System.out.println("All tests passed.");
    }

    private static void test(P119_FindMinimumInRotatedSortedArray sol, int[] nums, int expected) {
        int actual = sol.findMin(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
