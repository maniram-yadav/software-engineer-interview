/**
 * LeetCode Top Interview 150 -- #56. Basic Calculator (Hard)
 *
 * Given a string expression containing +, -, (, ), digits, and spaces,
 * implement a basic calculator to evaluate it (no * or /).
 *
 * Example:
 *   Input: s = "(1+(4+5+2)-3)+(6+8)"
 *   Output: 23
 */
public class P56_BasicCalculator {

    public int calculate(String s) {
        java.util.Deque<Integer> stack = new java.util.ArrayDeque<>();
        int result = 0, number = 0, sign = 1;

        for (char c : s.toCharArray()) {
            if (Character.isDigit(c)) {
                number = number * 10 + (c - '0');
            } else if (c == '+') {
                result += sign * number;
                number = 0;
                sign = 1;
            } else if (c == '-') {
                result += sign * number;
                number = 0;
                sign = -1;
            } else if (c == '(') {
                stack.push(result);
                stack.push(sign);
                result = 0;
                sign = 1;
            } else if (c == ')') {
                result += sign * number;
                number = 0;
                result *= stack.pop();
                result += stack.pop();
            }
        }
        result += sign * number;
        return result;
    }

    public static void main(String[] args) {
        P56_BasicCalculator sol = new P56_BasicCalculator();
        test(sol, "(1+(4+5+2)-3)+(6+8)", 23);
        test(sol, "1 + 1", 2);
        test(sol, " 2-1 + 2 ", 3);
        test(sol, "2-(5-6)", 3);
        System.out.println("All tests passed.");
    }

    private static void test(P56_BasicCalculator sol, String s, int expected) {
        int actual = sol.calculate(s);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: \"" + s + "\" -> " + actual);
    }
}
