/**
 * LeetCode Top Interview 150 -- #40. Isomorphic Strings (Easy)
 *
 * Given strings s and t, determine if they are isomorphic -- characters in
 * s can be replaced to get t, with a consistent one-to-one mapping.
 *
 * Example:
 *   Input: s = "egg", t = "add"
 *   Output: true
 */
public class P40_IsomorphicStrings {

    public boolean isIsomorphic(String s, String t) {
        if (s.length() != t.length()) return false;

        java.util.Map<Character, Character> mapST = new java.util.HashMap<>();
        java.util.Map<Character, Character> mapTS = new java.util.HashMap<>();

        for (int i = 0; i < s.length(); i++) {
            char a = s.charAt(i), b = t.charAt(i);
            if (mapST.containsKey(a) && mapST.get(a) != b) return false;
            if (mapTS.containsKey(b) && mapTS.get(b) != a) return false;
            mapST.put(a, b);
            mapTS.put(b, a);
        }
        return true;
    }

    public static void main(String[] args) {
        P40_IsomorphicStrings sol = new P40_IsomorphicStrings();
        test(sol, "egg", "add", true);
        test(sol, "foo", "bar", false);
        test(sol, "paper", "title", true);
        test(sol, "badc", "baba", false);
        System.out.println("All tests passed.");
    }

    private static void test(P40_IsomorphicStrings sol, String s, String t, boolean expected) {
        boolean actual = sol.isIsomorphic(s, t);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: s=\"" + s + "\" t=\"" + t + "\" -> " + actual);
    }
}
