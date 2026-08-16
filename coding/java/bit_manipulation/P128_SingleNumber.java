/**
 * LeetCode Top Interview 150 -- #128. Single Number (Easy)
 *
 * Given a non-empty array of integers where every element appears twice
 * except for one, find that single one, in O(n) time and O(1) space (XOR
 * trick).
 *
 * Example:
 *   Input: nums = [4,1,2,1,2]
 *   Output: 4
 */
public class P128_SingleNumber {

    public int singleNumber(int[] nums) {
        int result = 0;
        for (int n : nums) result ^= n;
        return result;
    }

    public static void main(String[] args) {
        P128_SingleNumber sol = new P128_SingleNumber();
        test(sol, new int[]{4, 1, 2, 1, 2}, 4);
        test(sol, new int[]{2, 2, 1}, 1);
        test(sol, new int[]{1}, 1);
        System.out.println("All tests passed.");
    }

    private static void test(P128_SingleNumber sol, int[] nums, int expected) {
        int actual = sol.singleNumber(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
