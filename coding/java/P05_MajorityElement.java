/**
 * LeetCode Top Interview 150 -- #5. Majority Element (Easy)
 *
 * Given an array nums of size n, return the element that appears more than
 * floor(n / 2) times. It's guaranteed to exist (Boyer-Moore voting works
 * in O(1) space).
 *
 * Example:
 *   Input: nums = [2,2,1,1,1,2,2]
 *   Output: 2
 */
public class P05_MajorityElement {

    public int majorityElement(int[] nums) {
        int count = 0, candidate = 0;
        for (int num : nums) {
            if (count == 0) {
                candidate = num;
            }
            count += (num == candidate) ? 1 : -1;
        }
        return candidate;
    }

    public static void main(String[] args) {
        P05_MajorityElement sol = new P05_MajorityElement();
        test(sol, new int[]{2, 2, 1, 1, 1, 2, 2}, 2);
        test(sol, new int[]{3, 2, 3}, 3);
        test(sol, new int[]{1}, 1);
        test(sol, new int[]{6, 5, 5}, 5);
        System.out.println("All tests passed.");
    }

    private static void test(P05_MajorityElement sol, int[] nums, int expected) {
        int actual = sol.majorityElement(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
