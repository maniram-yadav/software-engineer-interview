/**
 * LeetCode Top Interview 150 -- #118. Find First and Last Position of Element in Sorted Array (Medium)
 *
 * Given a sorted array of integers and a target, find the starting and
 * ending index of the target's occurrences in O(log n). Return [-1,-1] if
 * not found.
 *
 * Example:
 *   Input: nums = [5,7,7,8,8,10], target = 8
 *   Output: [3,4]
 */
public class P118_FindFirstAndLastPositionOfElementInSortedArray {

    public int[] searchRange(int[] nums, int target) {
        int first = findBound(nums, target, true);
        if (first == -1) return new int[]{-1, -1};
        int last = findBound(nums, target, false);
        return new int[]{first, last};
    }

    private int findBound(int[] nums, int target, boolean findFirst) {
        int left = 0, right = nums.length - 1, result = -1;
        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (nums[mid] == target) {
                result = mid;
                if (findFirst) right = mid - 1;
                else left = mid + 1;
            } else if (nums[mid] < target) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        return result;
    }

    public static void main(String[] args) {
        P118_FindFirstAndLastPositionOfElementInSortedArray sol = new P118_FindFirstAndLastPositionOfElementInSortedArray();
        test(sol, new int[]{5, 7, 7, 8, 8, 10}, 8, new int[]{3, 4});
        test(sol, new int[]{5, 7, 7, 8, 8, 10}, 6, new int[]{-1, -1});
        test(sol, new int[]{}, 0, new int[]{-1, -1});
        System.out.println("All tests passed.");
    }

    private static void test(P118_FindFirstAndLastPositionOfElementInSortedArray sol, int[] nums, int target, int[] expected) {
        int[] actual = sol.searchRange(nums, target);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " target=" + target + " -> " + java.util.Arrays.toString(actual));
    }
}
