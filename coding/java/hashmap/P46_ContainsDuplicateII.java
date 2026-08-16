/**
 * LeetCode Top Interview 150 -- #46. Contains Duplicate II (Easy)
 *
 * Given an array nums and an integer k, return true if there are two
 * distinct indices i, j such that nums[i] == nums[j] and |i - j| <= k.
 *
 * Example:
 *   Input: nums = [1,2,3,1], k = 3
 *   Output: true
 */
public class P46_ContainsDuplicateII {

    public boolean containsNearbyDuplicate(int[] nums, int k) {
        java.util.Map<Integer, Integer> lastIndex = new java.util.HashMap<>();
        for (int i = 0; i < nums.length; i++) {
            if (lastIndex.containsKey(nums[i]) && i - lastIndex.get(nums[i]) <= k) {
                return true;
            }
            lastIndex.put(nums[i], i);
        }
        return false;
    }

    public static void main(String[] args) {
        P46_ContainsDuplicateII sol = new P46_ContainsDuplicateII();
        test(sol, new int[]{1, 2, 3, 1}, 3, true);
        test(sol, new int[]{1, 0, 1, 1}, 1, true);
        test(sol, new int[]{1, 2, 3, 1, 2, 3}, 2, false);
        System.out.println("All tests passed.");
    }

    private static void test(P46_ContainsDuplicateII sol, int[] nums, int k, boolean expected) {
        boolean actual = sol.containsNearbyDuplicate(nums, k);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " k=" + k + " -> " + actual);
    }
}
