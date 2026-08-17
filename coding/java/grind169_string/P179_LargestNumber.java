/**
 * Grind 169 -- #179. Largest Number (Medium)
 *
 * Given a list of non-negative integers, arrange them so they form the
 * largest possible number, returned as a string.
 *
 * Example:
 *   Input: nums = [3,30,34,5,9]
 *   Output: "9534330"
 */
public class P179_LargestNumber {

    public String largestNumber(int[] nums) {
        String[] strs = new String[nums.length];
        for (int i = 0; i < nums.length; i++) strs[i] = String.valueOf(nums[i]);
        java.util.Arrays.sort(strs, (a, b) -> (b + a).compareTo(a + b));
        if (strs[0].equals("0")) return "0";
        StringBuilder sb = new StringBuilder();
        for (String s : strs) sb.append(s);
        return sb.toString();
    }

    public static void main(String[] args) {
        P179_LargestNumber sol = new P179_LargestNumber();
        test(sol, new int[]{10, 2}, "210");
        test(sol, new int[]{3, 30, 34, 5, 9}, "9534330");
        test(sol, new int[]{0, 0}, "0");
        System.out.println("All tests passed.");
    }

    private static void test(P179_LargestNumber sol, int[] nums, String expected) {
        String actual = sol.largestNumber(nums);
        if (!actual.equals(expected)) {
            throw new AssertionError("Expected \"" + expected + "\" but got \"" + actual + "\"");
        }
        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> \"" + actual + "\"");
    }
}
