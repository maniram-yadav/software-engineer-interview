/**
 * Grind 169 -- #232. Implement Queue using Stacks (Easy)
 *
 * Implement a first-in-first-out (FIFO) queue using only two stacks,
 * supporting push, pop, peek, and empty.
 *
 * Example:
 *   MyQueue q = new MyQueue();
 *   q.push(1); q.push(2);
 *   q.peek(); // 1
 *   q.pop();  // 1
 *   q.empty();// false
 */
public class P232_ImplementQueueUsingStacks {

    static class MyQueue {
        private final java.util.Deque<Integer> in = new java.util.ArrayDeque<>();
        private final java.util.Deque<Integer> out = new java.util.ArrayDeque<>();

        public void push(int x) {
            in.push(x);
        }

        public int pop() {
            peek();
            return out.pop();
        }

        public int peek() {
            if (out.isEmpty()) {
                while (!in.isEmpty()) out.push(in.pop());
            }
            return out.peek();
        }

        public boolean empty() {
            return in.isEmpty() && out.isEmpty();
        }
    }

    public static void main(String[] args) {
        MyQueue q = new MyQueue();
        q.push(1);
        q.push(2);
        check(q.peek(), 1, "peek()");
        check(q.pop(), 1, "pop()");
        checkBool(q.empty(), false, "empty()");
        check(q.pop(), 2, "pop()");
        checkBool(q.empty(), true, "empty() after draining");
        System.out.println("All tests passed.");
    }

    private static void check(int actual, int expected, String label) {
        if (actual != expected) {
            throw new AssertionError(label + ": expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + label + " -> " + actual);
    }

    private static void checkBool(boolean actual, boolean expected, String label) {
        if (actual != expected) {
            throw new AssertionError(label + ": expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + label + " -> " + actual);
    }
}
