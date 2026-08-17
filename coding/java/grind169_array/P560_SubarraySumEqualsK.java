/**
 * Grind 169 -- #560. Subarray Sum Equals K (Medium)
 *
 * Given an integer array nums and an integer k, return the total number of
 * contiguous subarrays whose sum equals k.
 *
 * Example:
 *   Input: nums = [1,1,1], k = 2
 *   Output: 2
 */
public class P560_SubarraySumEqualsK {

    public int subarraySum(int[] nums, int k) {
        java.util.Map<Integer, Integer> prefixCount = new java.util.HashMap<>();
        prefixCount.put(0, 1);
        int sum = 0, count = 0;
        for (int n : nums) {
            sum += n;
            count += prefixCount.getOrDefault(sum - k, 0);
            prefixCount.merge(sum, 1, Integer::sum);
        }
        return count;
    }

    public static void main(String[] args) {
        P560_SubarraySumEqualsK sol = new P560_SubarraySumEqualsK();
        test(sol, new int[]{1, 1, 1}, 2, 2);
        test(sol, new int[]{1, 2, 3}, 3, 2);
        test(sol, new int[]{1}, 0, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P560_SubarraySumEqualsK sol, int[] nums, int k, int expected) {
        int actual = sol.subarraySum(nums, k);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " k=" + k + " -> " + actual);
    }
}
