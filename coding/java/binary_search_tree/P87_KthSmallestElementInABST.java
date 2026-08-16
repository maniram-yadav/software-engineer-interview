/**
 * LeetCode Top Interview 150 -- #87. Kth Smallest Element in a BST (Medium)
 *
 * Given the root of a BST and an integer k, return the k-th smallest value
 * (1-indexed).
 *
 * Example:
 *   Input: root = [3,1,4,null,2], k = 1
 *   Output: 1
 */
public class P87_KthSmallestElementInABST {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public int kthSmallest(TreeNode root, int k) {
        java.util.Deque<TreeNode> stack = new java.util.ArrayDeque<>();
        TreeNode node = root;
        while (true) {
            while (node != null) {
                stack.push(node);
                node = node.left;
            }
            node = stack.pop();
            if (--k == 0) return node.val;
            node = node.right;
        }
    }

    public static void main(String[] args) {
        P87_KthSmallestElementInABST sol = new P87_KthSmallestElementInABST();
        test(sol, build(3, 1, 4, null, 2), 1, 1);
        test(sol, build(5, 3, 6, 2, 4, null, null, 1), 3, 3);
        System.out.println("All tests passed.");
    }

    private static void test(P87_KthSmallestElementInABST sol, TreeNode root, int k, int expected) {
        int actual = sol.kthSmallest(root, k);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: k=" + k + " -> " + actual);
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
