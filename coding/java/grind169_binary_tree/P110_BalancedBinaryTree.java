/**
 * Grind 169 -- #110. Balanced Binary Tree (Easy)
 *
 * Given a binary tree, determine if it is height-balanced (the depth of
 * the two subtrees of every node never differs by more than 1).
 *
 * Example:
 *   Input: root = [3,9,20,null,null,15,7]
 *   Output: true
 */
public class P110_BalancedBinaryTree {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public boolean isBalanced(TreeNode root) {
        return height(root) != -1;
    }

    private int height(TreeNode node) {
        if (node == null) return 0;
        int left = height(node.left);
        if (left == -1) return -1;
        int right = height(node.right);
        if (right == -1) return -1;
        if (Math.abs(left - right) > 1) return -1;
        return 1 + Math.max(left, right);
    }

    public static void main(String[] args) {
        P110_BalancedBinaryTree sol = new P110_BalancedBinaryTree();
        test(sol, build(3, 9, 20, null, null, 15, 7), true);
        test(sol, build(1, 2, 2, 3, 3, null, null, 4, 4), false);
        test(sol, null, true);
        System.out.println("All tests passed.");
    }

    private static void test(P110_BalancedBinaryTree sol, TreeNode root, boolean expected) {
        boolean actual = sol.isBalanced(root);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: -> " + actual);
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
