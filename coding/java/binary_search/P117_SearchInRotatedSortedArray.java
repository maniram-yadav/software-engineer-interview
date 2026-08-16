/**
 * LeetCode Top Interview 150 -- #117. Search in Rotated Sorted Array (Medium)
 *
 * Given a sorted array that has been rotated at an unknown pivot, and a
 * target, search for the target in O(log n) and return its index, or -1.
 *
 * Example:
 *   Input: nums = [4,5,6,7,0,1,2], target = 0
 *   Output: 4
 */
public class P117_SearchInRotatedSortedArray {

    public int search(int[] nums, int target) {
        int left = 0, right = nums.length - 1;
        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (nums[mid] == target) return mid;

            if (nums[left] <= nums[mid]) {
                if (nums[left] <= target && target < nums[mid]) right = mid - 1;
                else left = mid + 1;
            } else {
                if (nums[mid] < target && target <= nums[right]) left = mid + 1;
                else right = mid - 1;
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        P117_SearchInRotatedSortedArray sol = new P117_SearchInRotatedSortedArray();
        test(sol, new int[]{4, 5, 6, 7, 0, 1, 2}, 0, 4);
        test(sol, new int[]{4, 5, 6, 7, 0, 1, 2}, 3, -1);
        test(sol, new int[]{1}, 0, -1);
        System.out.println("All tests passed.");
    }

    private static void test(P117_SearchInRotatedSortedArray sol, int[] nums, int target, int expected) {
        int actual = sol.search(nums, target);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " target=" + target + " -> " + actual);
    }
}
