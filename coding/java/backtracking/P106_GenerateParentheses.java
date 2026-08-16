/**
 * LeetCode Top Interview 150 -- #106. Generate Parentheses (Medium)
 *
 * Given n pairs of parentheses, generate all combinations of well-formed
 * parentheses.
 *
 * Example:
 *   Input: n = 3
 *   Output: ["((()))","(()())","(())()","()(())","()()()"]
 */
public class P106_GenerateParentheses {

    public java.util.List<String> generateParenthesis(int n) {
        java.util.List<String> result = new java.util.ArrayList<>();
        backtrack(n, 0, 0, new StringBuilder(), result);
        return result;
    }

    private void backtrack(int n, int open, int close, StringBuilder current, java.util.List<String> result) {
        if (current.length() == 2 * n) {
            result.add(current.toString());
            return;
        }
        if (open < n) {
            current.append('(');
            backtrack(n, open + 1, close, current, result);
            current.deleteCharAt(current.length() - 1);
        }
        if (close < open) {
            current.append(')');
            backtrack(n, open, close + 1, current, result);
            current.deleteCharAt(current.length() - 1);
        }
    }

    public static void main(String[] args) {
        P106_GenerateParentheses sol = new P106_GenerateParentheses();
        test(sol, 3, new String[]{"((()))", "(()())", "(())()", "()(())", "()()()"});
        test(sol, 1, new String[]{"()"});
        System.out.println("All tests passed.");
    }

    private static void test(P106_GenerateParentheses sol, int n, String[] expected) {
        java.util.List<String> actual = sol.generateParenthesis(n);
        java.util.List<String> expectedList = java.util.Arrays.asList(expected);
        if (!actual.equals(expectedList)) {
            throw new AssertionError("Expected " + expectedList + " but got " + actual);
        }
        System.out.println("PASS: n=" + n + " -> " + actual);
    }
}
