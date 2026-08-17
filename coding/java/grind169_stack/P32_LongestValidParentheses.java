/**
 * Grind 169 -- #32. Longest Valid Parentheses (Hard)
 *
 * Given a string containing just ( and ), find the length of the longest
 * valid (well-formed) parentheses substring.
 *
 * Example:
 *   Input: s = ")()())"
 *   Output: 4   ("()()")
 */
public class P32_LongestValidParentheses {

    public int longestValidParentheses(String s) {
        java.util.Deque<Integer> stack = new java.util.ArrayDeque<>();
        stack.push(-1);
        int maxLen = 0;
        for (int i = 0; i < s.length(); i++) {
            if (s.charAt(i) == '(') {
                stack.push(i);
            } else {
                stack.pop();
                if (stack.isEmpty()) {
                    stack.push(i);
                } else {
                    maxLen = Math.max(maxLen, i - stack.peek());
                }
            }
        }
        return maxLen;
    }

    public static void main(String[] args) {
        P32_LongestValidParentheses sol = new P32_LongestValidParentheses();
        test(sol, ")()())", 4);
        test(sol, "(()", 2);
        test(sol, "", 0);
        System.out.println("All tests passed.");
    }

    private static void test(P32_LongestValidParentheses sol, String s, int expected) {
        int actual = sol.longestValidParentheses(s);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: \"" + s + "\" -> " + actual);
    }
}
