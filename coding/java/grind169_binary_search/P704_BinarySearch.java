/**
 * Grind 169 -- #704. Binary Search (Easy)
 *
 * Given a sorted array of unique integers and a target, return its index
 * using binary search, or -1 if absent.
 *
 * Example:
 *   Input: nums = [-1,0,3,5,9,12], target = 9
 *   Output: 4
 */
public class P704_BinarySearch {

    public int search(int[] nums, int target) {
        int left = 0, right = nums.length - 1;
        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (nums[mid] == target) return mid;
            else if (nums[mid] < target) left = mid + 1;
            else right = mid - 1;
        }
        return -1;
    }

    public static void main(String[] args) {
        P704_BinarySearch sol = new P704_BinarySearch();
        test(sol, new int[]{-1, 0, 3, 5, 9, 12}, 9, 4);
        test(sol, new int[]{-1, 0, 3, 5, 9, 12}, 2, -1);
        System.out.println("All tests passed.");
    }

    private static void test(P704_BinarySearch sol, int[] nums, int target, int expected) {
        int actual = sol.search(nums, target);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: target=" + target + " -> " + actual);
    }
}
