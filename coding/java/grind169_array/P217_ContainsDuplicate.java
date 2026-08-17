/**
 * Grind 169 -- #217. Contains Duplicate (Easy)
 *
 * Given an integer array nums, return true if any value appears at least
 * twice.
 *
 * Example:
 *   Input: nums = [1,2,3,1]
 *   Output: true
 */
public class P217_ContainsDuplicate {

    public boolean containsDuplicate(int[] nums) {
        java.util.Set<Integer> seen = new java.util.HashSet<>();
        for (int n : nums) {
            if (!seen.add(n)) return true;
        }
        return false;
    }

    public static void main(String[] args) {
        P217_ContainsDuplicate sol = new P217_ContainsDuplicate();
        test(sol, new int[]{1, 2, 3, 1}, true);
        test(sol, new int[]{1, 2, 3, 4}, false);
        test(sol, new int[]{1, 1, 1, 3, 3, 4, 3, 2, 4, 2}, true);
        System.out.println("All tests passed.");
    }

    private static void test(P217_ContainsDuplicate sol, int[] nums, boolean expected) {
        boolean actual = sol.containsDuplicate(nums);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> " + actual);
    }
}
