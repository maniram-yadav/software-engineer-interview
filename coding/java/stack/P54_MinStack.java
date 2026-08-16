/**
 * LeetCode Top Interview 150 -- #54. Min Stack (Medium)
 *
 * Design a stack supporting push, pop, top, and getMin -- all in O(1) time.
 *
 * Example:
 *   MinStack ms = new MinStack();
 *   ms.push(-2); ms.push(0); ms.push(-3);
 *   ms.getMin(); // -3
 *   ms.pop();
 *   ms.top();    // 0
 *   ms.getMin(); // -2
 */
public class P54_MinStack {

    static class MinStack {
        private final java.util.Deque<Integer> stack = new java.util.ArrayDeque<>();
        private final java.util.Deque<Integer> minStack = new java.util.ArrayDeque<>();

        public void push(int val) {
            stack.push(val);
            minStack.push(minStack.isEmpty() ? val : Math.min(val, minStack.peek()));
        }

        public void pop() {
            stack.pop();
            minStack.pop();
        }

        public int top() {
            return stack.peek();
        }

        public int getMin() {
            return minStack.peek();
        }
    }

    public static void main(String[] args) {
        MinStack ms = new MinStack();
        ms.push(-2);
        ms.push(0);
        ms.push(-3);
        check(ms.getMin(), -3, "getMin after push(-2,0,-3)");
        ms.pop();
        check(ms.top(), 0, "top after pop");
        check(ms.getMin(), -2, "getMin after pop");
        System.out.println("All tests passed.");
    }

    private static void check(int actual, int expected, String label) {
        if (actual != expected) {
            throw new AssertionError(label + ": expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + label + " -> " + actual);
    }
}
