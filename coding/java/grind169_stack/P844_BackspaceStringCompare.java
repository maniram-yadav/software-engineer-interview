/**
 * Grind 169 -- #844. Backspace String Compare (Easy)
 *
 * Given two strings s and t containing lowercase letters and # (backspace),
 * return true if they're equal after applying the backspaces.
 *
 * Example:
 *   Input: s = "ab#c", t = "ad#c"
 *   Output: true   (both become "ac")
 */
public class P844_BackspaceStringCompare {

    public boolean backspaceCompare(String s, String t) {
        return build(s).equals(build(t));
    }

    private String build(String s) {
        StringBuilder sb = new StringBuilder();
        for (char c : s.toCharArray()) {
            if (c == '#') {
                if (sb.length() > 0) sb.deleteCharAt(sb.length() - 1);
            } else {
                sb.append(c);
            }
        }
        return sb.toString();
    }

    public static void main(String[] args) {
        P844_BackspaceStringCompare sol = new P844_BackspaceStringCompare();
        test(sol, "ab#c", "ad#c", true);
        test(sol, "ab##", "c#d#", true);
        test(sol, "a#c", "b", false);
        System.out.println("All tests passed.");
    }

    private static void test(P844_BackspaceStringCompare sol, String s, String t, boolean expected) {
        boolean actual = sol.backspaceCompare(s, t);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: s=\"" + s + "\" t=\"" + t + "\" -> " + actual);
    }
}
