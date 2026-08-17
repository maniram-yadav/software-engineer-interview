/**
 * Grind 169 -- #268. Missing Number (Easy)
 *
 * Given an array nums containing n distinct numbers in range [0, n],
 * return the one number missing from the range.
 *
 * Example:
 *   Input: nums = [3,0,1]
 *   Output: 2
 */
public class P268_MissingNumber {

    public int missingNumber(int[] nums) {
        int n = nums.length;
        int expectedSum = n * (n + 1) / 2;
        int actualSum = 0;
        for (int x : nums) actualSum += x;
        return expectedSum - actualSum;
    }

    public static void main(String[] args) {
        P268_MissingNumber sol = new P268_MissingNumber();
        test(sol, new int[]{3, 0, 1}, 2);
        test(sol, new int[]{0, 1}, 2);
        test(sol, new int[]{9, 6, 4, 2, 3, 5, 7, 0, 1}, 8);
        System.out.println("All tests passed.");
    }

    private static void test(P268_MissingNumber sol, int[] nums, int expected) {
        int actual = sol.missingNumber(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
