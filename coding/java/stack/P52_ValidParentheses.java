/**
 * LeetCode Top Interview 150 -- #52. Valid Parentheses (Easy)
 *
 * Given a string of ()[]{} characters, determine if brackets are properly
 * matched and nested.
 *
 * Example:
 *   Input: s = "()[]{}"
 *   Output: true
 */
public class P52_ValidParentheses {

    public boolean isValid(String s) {
        java.util.Deque<Character> stack = new java.util.ArrayDeque<>();
        java.util.Map<Character, Character> pairs = java.util.Map.of(')', '(', ']', '[', '}', '{');

        for (char c : s.toCharArray()) {
            if (pairs.containsKey(c)) {
                if (stack.isEmpty() || stack.pop() != pairs.get(c)) return false;
            } else {
                stack.push(c);
            }
        }
        return stack.isEmpty();
    }

    public static void main(String[] args) {
        P52_ValidParentheses sol = new P52_ValidParentheses();
        test(sol, "()[]{}", true);
        test(sol, "(]", false);
        test(sol, "([)]", false);
        test(sol, "{[]}", true);
        System.out.println("All tests passed.");
    }

    private static void test(P52_ValidParentheses sol, String s, boolean expected) {
        boolean actual = sol.isValid(s);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: \"" + s + "\" -> " + actual);
    }
}
