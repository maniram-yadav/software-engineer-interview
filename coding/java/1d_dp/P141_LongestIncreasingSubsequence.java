/**
 * LeetCode Top Interview 150 -- #141. Longest Increasing Subsequence (Medium)
 *
 * Given an integer array nums, return the length of the longest strictly
 * increasing subsequence.
 *
 * Example:
 *   Input: nums = [10,9,2,5,3,7,101,18]
 *   Output: 4   ([2,3,7,101])
 */
public class P141_LongestIncreasingSubsequence {

    public int lengthOfLIS(int[] nums) {
        int[] tails = new int[nums.length];
        int size = 0;
        for (int n : nums) {
            int left = 0, right = size;
            while (left < right) {
                int mid = left + (right - left) / 2;
                if (tails[mid] < n) left = mid + 1;
                else right = mid;
            }
            tails[left] = n;
            if (left == size) size++;
        }
        return size;
    }

    public static void main(String[] args) {
        P141_LongestIncreasingSubsequence sol = new P141_LongestIncreasingSubsequence();
        test(sol, new int[]{10, 9, 2, 5, 3, 7, 101, 18}, 4);
        test(sol, new int[]{0, 1, 0, 3, 2, 3}, 4);
        test(sol, new int[]{7, 7, 7, 7}, 1);
        System.out.println("All tests passed.");
    }

    private static void test(P141_LongestIncreasingSubsequence sol, int[] nums, int expected) {
        int actual = sol.lengthOfLIS(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
