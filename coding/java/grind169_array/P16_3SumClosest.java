/**
 * Grind 169 -- #16. 3Sum Closest (Medium)
 *
 * Given an integer array nums and a target, find three integers whose sum
 * is closest to target and return that sum.
 *
 * Example:
 *   Input: nums = [-1,2,1,-4], target = 1
 *   Output: 2   (-1 + 2 + 1 = 2)
 */
public class P16_3SumClosest {

    public int threeSumClosest(int[] nums, int target) {
        java.util.Arrays.sort(nums);
        int closest = nums[0] + nums[1] + nums[2];
        for (int i = 0; i < nums.length - 2; i++) {
            int left = i + 1, right = nums.length - 1;
            while (left < right) {
                int sum = nums[i] + nums[left] + nums[right];
                if (Math.abs(sum - target) < Math.abs(closest - target)) closest = sum;
                if (sum == target) return sum;
                else if (sum < target) left++;
                else right--;
            }
        }
        return closest;
    }

    public static void main(String[] args) {
        P16_3SumClosest sol = new P16_3SumClosest();
        test(sol, new int[]{-1, 2, 1, -4}, 1, 2);
        test(sol, new int[]{0, 0, 0}, 1, 0);
        test(sol, new int[]{1, 1, 1, 1}, 3, 3);
        System.out.println("All tests passed.");
    }

    private static void test(P16_3SumClosest sol, int[] nums, int target, int expected) {
        int actual = sol.threeSumClosest(nums, target);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " target=" + target + " -> " + actual);
    }
}
