/**
 * LeetCode Top Interview 150 -- #29. 3Sum (Medium)
 *
 * Given an integer array nums, return all unique triplets
 * [nums[i], nums[j], nums[k]] that sum to 0.
 *
 * Example:
 *   Input: nums = [-1,0,1,2,-1,-4]
 *   Output: [[-1,-1,2],[-1,0,1]]
 */
public class P29_3Sum {

    public java.util.List<java.util.List<Integer>> threeSum(int[] nums) {
        java.util.Arrays.sort(nums);
        java.util.List<java.util.List<Integer>> result = new java.util.ArrayList<>();

        for (int i = 0; i < nums.length - 2; i++) {
            if (i > 0 && nums[i] == nums[i - 1]) continue;
            int left = i + 1, right = nums.length - 1;
            while (left < right) {
                int sum = nums[i] + nums[left] + nums[right];
                if (sum == 0) {
                    result.add(java.util.Arrays.asList(nums[i], nums[left], nums[right]));
                    left++;
                    right--;
                    while (left < right && nums[left] == nums[left - 1]) left++;
                    while (left < right && nums[right] == nums[right + 1]) right--;
                } else if (sum < 0) {
                    left++;
                } else {
                    right--;
                }
            }
        }
        return result;
    }

    public static void main(String[] args) {
        P29_3Sum sol = new P29_3Sum();
        test(sol, new int[]{-1, 0, 1, 2, -1, -4}, "[[-1, -1, 2], [-1, 0, 1]]");
        test(sol, new int[]{0, 1, 1}, "[]");
        test(sol, new int[]{0, 0, 0}, "[[0, 0, 0]]");
        System.out.println("All tests passed.");
    }

    private static void test(P29_3Sum sol, int[] nums, String expected) {
        java.util.List<java.util.List<Integer>> actual = sol.threeSum(nums);
        if (!actual.toString().equals(expected)) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
