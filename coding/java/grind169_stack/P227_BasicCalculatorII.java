/**
 * Grind 169 -- #227. Basic Calculator II (Medium)
 *
 * Given a string expression containing non-negative integers and
 * + - * / (no parentheses), evaluate it following normal operator
 * precedence.
 *
 * Example:
 *   Input: s = "3+2*2"
 *   Output: 7
 */
public class P227_BasicCalculatorII {

    public int calculate(String s) {
        java.util.Deque<Integer> stack = new java.util.ArrayDeque<>();
        int num = 0;
        char sign = '+';
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (Character.isDigit(c)) num = num * 10 + (c - '0');
            if ((!Character.isDigit(c) && c != ' ') || i == s.length() - 1) {
                if (sign == '+') stack.push(num);
                else if (sign == '-') stack.push(-num);
                else if (sign == '*') stack.push(stack.pop() * num);
                else if (sign == '/') stack.push(stack.pop() / num);
                sign = c;
                num = 0;
            }
        }
        int result = 0;
        for (int n : stack) result += n;
        return result;
    }

    public static void main(String[] args) {
        P227_BasicCalculatorII sol = new P227_BasicCalculatorII();
        test(sol, "3+2*2", 7);
        test(sol, " 3/2 ", 1);
        test(sol, " 3+5 / 2 ", 5);
        System.out.println("All tests passed.");
    }

    private static void test(P227_BasicCalculatorII sol, String s, int expected) {
        int actual = sol.calculate(s);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: \"" + s + "\" -> " + actual);
    }
}
