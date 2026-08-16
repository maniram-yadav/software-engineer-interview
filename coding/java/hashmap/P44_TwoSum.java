/**
 * LeetCode Top Interview 150 -- #44. Two Sum (Easy)
 *
 * Given an array nums and an integer target, return the indices of the two
 * numbers that add up to target. Exactly one solution exists.
 *
 * Example:
 *   Input: nums = [2,7,11,15], target = 9
 *   Output: [0,1]
 */
public class P44_TwoSum {

    public int[] twoSum(int[] nums, int target) {
        java.util.Map<Integer, Integer> seen = new java.util.HashMap<>();
        for (int i = 0; i < nums.length; i++) {
            int complement = target - nums[i];
            if (seen.containsKey(complement)) {
                return new int[]{seen.get(complement), i};
            }
            seen.put(nums[i], i);
        }
        throw new IllegalArgumentException("No solution");
    }

    public static void main(String[] args) {
        P44_TwoSum sol = new P44_TwoSum();
        test(sol, new int[]{2, 7, 11, 15}, 9, new int[]{0, 1});
        test(sol, new int[]{3, 2, 4}, 6, new int[]{1, 2});
        test(sol, new int[]{3, 3}, 6, new int[]{0, 1});
        System.out.println("All tests passed.");
    }

    private static void test(P44_TwoSum sol, int[] nums, int target, int[] expected) {
        int[] actual = sol.twoSum(nums, target);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " target=" + target + " -> " + java.util.Arrays.toString(actual));
    }
}
