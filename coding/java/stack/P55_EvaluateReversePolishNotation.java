/**
 * LeetCode Top Interview 150 -- #55. Evaluate Reverse Polish Notation (Medium)
 *
 * Evaluate an arithmetic expression given in Reverse Polish (postfix)
 * notation as an array of tokens.
 *
 * Example:
 *   Input: tokens = ["2","1","+","3","*"]
 *   Output: 9   ((2 + 1) * 3)
 */
public class P55_EvaluateReversePolishNotation {

    private static final java.util.Set<String> OPS = java.util.Set.of("+", "-", "*", "/");

    public int evalRPN(String[] tokens) {
        java.util.Deque<Integer> stack = new java.util.ArrayDeque<>();
        for (String token : tokens) {
            if (OPS.contains(token)) {
                int b = stack.pop();
                int a = stack.pop();
                switch (token) {
                    case "+": stack.push(a + b); break;
                    case "-": stack.push(a - b); break;
                    case "*": stack.push(a * b); break;
                    case "/": stack.push(a / b); break;
                }
            } else {
                stack.push(Integer.parseInt(token));
            }
        }
        return stack.pop();
    }

    public static void main(String[] args) {
        P55_EvaluateReversePolishNotation sol = new P55_EvaluateReversePolishNotation();
        test(sol, new String[]{"2", "1", "+", "3", "*"}, 9);
        test(sol, new String[]{"4", "13", "5", "/", "+"}, 6);
        test(sol, new String[]{"10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"}, 22);
        System.out.println("All tests passed.");
    }

    private static void test(P55_EvaluateReversePolishNotation sol, String[] tokens, int expected) {
        int actual = sol.evalRPN(tokens);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(tokens) + " -> " + actual);
    }
}
