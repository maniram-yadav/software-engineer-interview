/**
 * Grind 169 -- #283. Move Zeroes (Easy)
 *
 * Given an integer array nums, move all zeroes to the end while
 * maintaining the relative order of non-zero elements, in place.
 *
 * Example:
 *   Input: nums = [0,1,0,3,12]
 *   Output: [1,3,12,0,0]
 */
public class P283_MoveZeroes {

    public void moveZeroes(int[] nums) {
        int insertPos = 0;
        for (int n : nums) {
            if (n != 0) nums[insertPos++] = n;
        }
        while (insertPos < nums.length) nums[insertPos++] = 0;
    }

    public static void main(String[] args) {
        P283_MoveZeroes sol = new P283_MoveZeroes();
        test(sol, new int[]{0, 1, 0, 3, 12}, new int[]{1, 3, 12, 0, 0});
        test(sol, new int[]{0}, new int[]{0});
        test(sol, new int[]{1, 2, 3}, new int[]{1, 2, 3});
        System.out.println("All tests passed.");
    }

    private static void test(P283_MoveZeroes sol, int[] nums, int[] expected) {
        sol.moveZeroes(nums);
        if (!java.util.Arrays.equals(nums, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(nums));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums));
    }
}
