/**
 * LeetCode Top Interview 150 -- #79. Binary Search Tree Iterator (Medium)
 *
 * Design an iterator over a BST that returns the next smallest number in
 * order via next(), with hasNext(), both amortized O(1).
 *
 * Example:
 *   BSTIterator it = new BSTIterator(root); // root = [7,3,15,null,null,9,20]
 *   it.next();    // 3
 *   it.next();    // 7
 *   it.hasNext(); // true
 *   it.next();    // 9
 */
public class P79_BinarySearchTreeIterator {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    static class BSTIterator {
        private final java.util.Deque<TreeNode> stack = new java.util.ArrayDeque<>();

        public BSTIterator(TreeNode root) {
            pushLeft(root);
        }

        public int next() {
            TreeNode node = stack.pop();
            pushLeft(node.right);
            return node.val;
        }

        public boolean hasNext() {
            return !stack.isEmpty();
        }

        private void pushLeft(TreeNode node) {
            while (node != null) {
                stack.push(node);
                node = node.left;
            }
        }
    }

    public static void main(String[] args) {
        TreeNode root = build(7, 3, 15, null, null, 9, 20);
        BSTIterator it = new BSTIterator(root);
        check(it.next(), 3, "next()");
        check(it.next(), 7, "next()");
        checkBool(it.hasNext(), true, "hasNext()");
        check(it.next(), 9, "next()");
        checkBool(it.hasNext(), true, "hasNext()");
        check(it.next(), 15, "next()");
        checkBool(it.hasNext(), true, "hasNext()");
        check(it.next(), 20, "next()");
        checkBool(it.hasNext(), false, "hasNext() at end");
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

    private static TreeNode build(Integer... values) {
        if (values.length == 0 || values[0] == null) return null;
        TreeNode root = new TreeNode(values[0]);
        java.util.Queue<TreeNode> queue = new java.util.LinkedList<>();
        queue.add(root);
        int i = 1;
        while (!queue.isEmpty() && i < values.length) {
            TreeNode node = queue.poll();
            if (i < values.length) {
                Integer leftVal = values[i++];
                if (leftVal != null) {
                    node.left = new TreeNode(leftVal);
                    queue.add(node.left);
                }
            }
            if (i < values.length) {
                Integer rightVal = values[i++];
                if (rightVal != null) {
                    node.right = new TreeNode(rightVal);
                    queue.add(node.right);
                }
            }
        }
        return root;
    }
}
