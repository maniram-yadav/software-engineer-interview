/**
 * LeetCode Top Interview 150 -- #23. Find the Index of the First Occurrence in a String (Easy)
 *
 * Given strings haystack and needle, return the index of the first
 * occurrence of needle in haystack, or -1 if it doesn't occur.
 *
 * Example:
 *   Input: haystack = "sadbutsad", needle = "sad"
 *   Output: 0
 */
public class P23_FindTheIndexOfTheFirstOccurrenceInAString {

    public int strStr(String haystack, String needle) {
        int n = haystack.length(), m = needle.length();
        if (m == 0) return 0;
        for (int i = 0; i + m <= n; i++) {
            if (haystack.substring(i, i + m).equals(needle)) {
                return i;
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        P23_FindTheIndexOfTheFirstOccurrenceInAString sol = new P23_FindTheIndexOfTheFirstOccurrenceInAString();
        test(sol, "sadbutsad", "sad", 0);
        test(sol, "leetcode", "leeto", -1);
        test(sol, "hello", "ll", 2);
        test(sol, "a", "a", 0);
        System.out.println("All tests passed.");
    }

    private static void test(P23_FindTheIndexOfTheFirstOccurrenceInAString sol, String haystack, String needle, int expected) {
        int actual = sol.strStr(haystack, needle);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: haystack=\"" + haystack + "\" needle=\"" + needle + "\" -> " + actual);
    }
}
