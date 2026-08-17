/**
 * Grind 169 -- #152. Maximum Product Subarray (Medium)
 *
 * Given an integer array nums, find a contiguous subarray with the largest
 * product, and return that product.
 *
 * Example:
 *   Input: nums = [2,3,-2,4]
 *   Output: 6   ([2,3])
 */
public class P152_MaximumProductSubarray {

    public int maxProduct(int[] nums) {
        int maxProd = nums[0], minProd = nums[0], result = nums[0];
        for (int i = 1; i < nums.length; i++) {
            int n = nums[i];
            if (n < 0) {
                int t = maxProd;
                maxProd = minProd;
                minProd = t;
            }
            maxProd = Math.max(n, maxProd * n);
            minProd = Math.min(n, minProd * n);
            result = Math.max(result, maxProd);
        }
        return result;
    }

    public static void main(String[] args) {
        P152_MaximumProductSubarray sol = new P152_MaximumProductSubarray();
        test(sol, new int[]{2, 3, -2, 4}, 6);
        test(sol, new int[]{-2, 0, -1}, 0);
        test(sol, new int[]{-2, 3, -4}, 24);
        System.out.println("All tests passed.");
    }

    private static void test(P152_MaximumProductSubarray sol, int[] nums, int expected) {
        int actual = sol.maxProduct(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
