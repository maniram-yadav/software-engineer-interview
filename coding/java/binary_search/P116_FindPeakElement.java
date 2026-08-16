/**
 * LeetCode Top Interview 150 -- #116. Find Peak Element (Medium)
 *
 * Given an integer array nums where nums[i] != nums[i+1], find any peak
 * element (greater than both neighbors, treating out-of-bounds as -inf) in
 * O(log n).
 *
 * Example:
 *   Input: nums = [1,2,3,1]
 *   Output: 2   (index of peak value 3)
 */
public class P116_FindPeakElement {

    public int findPeakElement(int[] nums) {
        int left = 0, right = nums.length - 1;
        while (left < right) {
            int mid = left + (right - left) / 2;
            if (nums[mid] > nums[mid + 1]) right = mid;
            else left = mid + 1;
        }
        return left;
    }

    public static void main(String[] args) {
        P116_FindPeakElement sol = new P116_FindPeakElement();
        test(sol, new int[]{1, 2, 3, 1});
        test(sol, new int[]{1, 2, 1, 3, 5, 6, 4});
        test(sol, new int[]{1});
        System.out.println("All tests passed.");
    }

    private static void test(P116_FindPeakElement sol, int[] nums) {
        int idx = sol.findPeakElement(nums);
        int left = (idx == 0) ? Integer.MIN_VALUE : nums[idx - 1];
        int right = (idx == nums.length - 1) ? Integer.MIN_VALUE : nums[idx + 1];
        if (nums[idx] <= left || nums[idx] <= right) {
            throw new AssertionError("Index " + idx + " is not a peak in " + java.util.Arrays.toString(nums));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> peak index " + idx + " (value " + nums[idx] + ")");
    }
}
