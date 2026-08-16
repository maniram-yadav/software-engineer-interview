/**
 * LeetCode Top Interview 150 -- #20. Longest Common Prefix (Easy)
 *
 * Given an array of strings, find the longest common prefix string among
 * all of them. Return "" if there is none.
 *
 * Example:
 *   Input: strs = ["flower","flow","flight"]
 *   Output: "fl"
 */
public class P20_LongestCommonPrefix {

    public String longestCommonPrefix(String[] strs) {
        if (strs == null || strs.length == 0) return "";
        String prefix = strs[0];
        for (int i = 1; i < strs.length; i++) {
            while (!strs[i].startsWith(prefix)) {
                prefix = prefix.substring(0, prefix.length() - 1);
                if (prefix.isEmpty()) return "";
            }
        }
        return prefix;
    }

    public static void main(String[] args) {
        P20_LongestCommonPrefix sol = new P20_LongestCommonPrefix();
        test(sol, new String[]{"flower", "flow", "flight"}, "fl");
        test(sol, new String[]{"dog", "racecar", "car"}, "");
        test(sol, new String[]{"single"}, "single");
        test(sol, new String[]{"", "b"}, "");
        System.out.println("All tests passed.");
    }

    private static void test(P20_LongestCommonPrefix sol, String[] strs, String expected) {
        String actual = sol.longestCommonPrefix(strs);
        if (!actual.equals(expected)) {
            throw new AssertionError("Expected \"" + expected + "\" but got \"" + actual + "\"");
        }
        System.out.println("PASS: " + java.util.Arrays.toString(strs) + " -> \"" + actual + "\"");
    }
}
