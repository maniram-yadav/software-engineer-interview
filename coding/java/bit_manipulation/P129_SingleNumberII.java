/**
 * LeetCode Top Interview 150 -- #129. Single Number II (Medium)
 *
 * Given an integer array where every element appears three times except
 * for one, find that single one, in O(n) time and O(1) space.
 *
 * Example:
 *   Input: nums = [2,2,3,2]
 *   Output: 3
 */
public class P129_SingleNumberII {

    public int singleNumber(int[] nums) {
        int ones = 0, twos = 0;
        for (int n : nums) {
            ones = (ones ^ n) & ~twos;
            twos = (twos ^ n) & ~ones;
        }
        return ones;
    }

    public static void main(String[] args) {
        P129_SingleNumberII sol = new P129_SingleNumberII();
        test(sol, new int[]{2, 2, 3, 2}, 3);
        test(sol, new int[]{0, 1, 0, 1, 0, 1, 99}, 99);
        System.out.println("All tests passed.");
    }

    private static void test(P129_SingleNumberII sol, int[] nums, int expected) {
        int actual = sol.singleNumber(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
