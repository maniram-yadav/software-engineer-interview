/**
 * Grind 169 -- #287. Find the Duplicate Number (Medium)
 *
 * Given an array of n + 1 integers where each value is in [1, n], and
 * exactly one number repeats (possibly multiple times), find the
 * duplicate without modifying the array and using O(1) extra space
 * (Floyd's cycle detection).
 *
 * Example:
 *   Input: nums = [1,3,4,2,2]
 *   Output: 2
 */
public class P287_FindTheDuplicateNumber {

    public int findDuplicate(int[] nums) {
        int slow = nums[0], fast = nums[0];
        do {
            slow = nums[slow];
            fast = nums[nums[fast]];
        } while (slow != fast);

        slow = nums[0];
        while (slow != fast) {
            slow = nums[slow];
            fast = nums[fast];
        }
        return slow;
    }

    public static void main(String[] args) {
        P287_FindTheDuplicateNumber sol = new P287_FindTheDuplicateNumber();
        test(sol, new int[]{1, 3, 4, 2, 2}, 2);
        test(sol, new int[]{3, 1, 3, 4, 2}, 3);
        System.out.println("All tests passed.");
    }

    private static void test(P287_FindTheDuplicateNumber sol, int[] nums, int expected) {
        int actual = sol.findDuplicate(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
