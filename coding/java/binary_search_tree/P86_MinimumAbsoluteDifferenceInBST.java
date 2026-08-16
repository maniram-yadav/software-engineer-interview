/**
 * LeetCode Top Interview 150 -- #86. Minimum Absolute Difference in BST (Easy)
 *
 * Given the root of a BST, return the minimum absolute difference between
 * the values of any two distinct nodes.
 *
 * Example:
 *   Input: root = [4,2,6,1,3]
 *   Output: 1
 */
public class P86_MinimumAbsoluteDifferenceInBST {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    private Integer prevVal;
    private int minDiff;

    public int getMinimumDifference(TreeNode root) {
        prevVal = null;
        minDiff = Integer.MAX_VALUE;
        inorder(root);
        return minDiff;
    }

    private void inorder(TreeNode node) {
        if (node == null) return;
        inorder(node.left);
        if (prevVal != null) minDiff = Math.min(minDiff, node.val - prevVal);
        prevVal = node.val;
        inorder(node.right);
    }

    public static void main(String[] args) {
        P86_MinimumAbsoluteDifferenceInBST sol = new P86_MinimumAbsoluteDifferenceInBST();
        test(sol, build(4, 2, 6, 1, 3), 1);
        test(sol, build(1, 0, 48, null, null, 12, 49), 1);
        System.out.println("All tests passed.");
    }

    private static void test(P86_MinimumAbsoluteDifferenceInBST sol, TreeNode root, int expected) {
        int actual = sol.getMinimumDifference(root);
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
