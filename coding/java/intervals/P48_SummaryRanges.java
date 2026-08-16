/**
 * LeetCode Top Interview 150 -- #48. Summary Ranges (Easy)
 *
 * Given a sorted unique array of integers, return the smallest sorted list
 * of ranges that exactly cover all numbers.
 *
 * Example:
 *   Input: nums = [0,1,2,4,5,7]
 *   Output: ["0->2","4->5","7"]
 */
public class P48_SummaryRanges {

    public java.util.List<String> summaryRanges(int[] nums) {
        java.util.List<String> result = new java.util.ArrayList<>();
        int n = nums.length;
        int i = 0;
        while (i < n) {
            int start = i;
            while (i + 1 < n && nums[i + 1] == nums[i] + 1) i++;
            if (start == i) {
                result.add(String.valueOf(nums[start]));
            } else {
                result.add(nums[start] + "->" + nums[i]);
            }
            i++;
        }
        return result;
    }

    public static void main(String[] args) {
        P48_SummaryRanges sol = new P48_SummaryRanges();
        test(sol, new int[]{0, 1, 2, 4, 5, 7}, new String[]{"0->2", "4->5", "7"});
        test(sol, new int[]{}, new String[]{});
        test(sol, new int[]{1}, new String[]{"1"});
        test(sol, new int[]{0, 2, 3, 4, 6, 8, 9}, new String[]{"0", "2->4", "6", "8->9"});
        System.out.println("All tests passed.");
    }

    private static void test(P48_SummaryRanges sol, int[] nums, String[] expected) {
        java.util.List<String> actual = sol.summaryRanges(nums);
        java.util.List<String> expectedList = java.util.Arrays.asList(expected);
        if (!actual.equals(expectedList)) {
            throw new AssertionError("Expected " + expectedList + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
