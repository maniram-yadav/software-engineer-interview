/**
 * Grind 169 -- #75. Sort Colors (Medium)
 *
 * Given an array with n objects colored red, white, or blue (represented
 * as 0, 1, 2), sort them in place so objects of the same color are
 * adjacent, in the order red, white, blue (Dutch national flag problem).
 *
 * Example:
 *   Input: nums = [2,0,2,1,1,0]
 *   Output: [0,0,1,1,2,2]
 */
public class P75_SortColors {

    public void sortColors(int[] nums) {
        int low = 0, mid = 0, high = nums.length - 1;
        while (mid <= high) {
            if (nums[mid] == 0) {
                swap(nums, low++, mid++);
            } else if (nums[mid] == 1) {
                mid++;
            } else {
                swap(nums, mid, high--);
            }
        }
    }

    private void swap(int[] nums, int i, int j) {
        int t = nums[i];
        nums[i] = nums[j];
        nums[j] = t;
    }

    public static void main(String[] args) {
        P75_SortColors sol = new P75_SortColors();
        test(sol, new int[]{2, 0, 2, 1, 1, 0}, new int[]{0, 0, 1, 1, 2, 2});
        test(sol, new int[]{2, 0, 1}, new int[]{0, 1, 2});
        test(sol, new int[]{0}, new int[]{0});
        System.out.println("All tests passed.");
    }

    private static void test(P75_SortColors sol, int[] nums, int[] expected) {
        sol.sortColors(nums);
        if (!java.util.Arrays.equals(nums, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(nums));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums));
    }
}
