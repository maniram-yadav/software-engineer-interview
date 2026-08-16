/**
 * LeetCode Top Interview 150 -- #114. Search Insert Position (Easy)
 *
 * Given a sorted array of distinct integers and a target, return the index
 * if found; otherwise the index where it would be inserted, in order.
 * O(log n).
 *
 * Example:
 *   Input: nums = [1,3,5,6], target = 5
 *   Output: 2
 */
public class P114_SearchInsertPosition {

    public int searchInsert(int[] nums, int target) {
        int left = 0, right = nums.length - 1;
        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (nums[mid] == target) return mid;
            else if (nums[mid] < target) left = mid + 1;
            else right = mid - 1;
        }
        return left;
    }

    public static void main(String[] args) {
        P114_SearchInsertPosition sol = new P114_SearchInsertPosition();
        test(sol, new int[]{1, 3, 5, 6}, 5, 2);
        test(sol, new int[]{1, 3, 5, 6}, 2, 1);
        test(sol, new int[]{1, 3, 5, 6}, 7, 4);
        test(sol, new int[]{1, 3, 5, 6}, 0, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P114_SearchInsertPosition sol, int[] nums, int target, int expected) {
        int actual = sol.searchInsert(nums, target);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " target=" + target + " -> " + actual);
    }
}
