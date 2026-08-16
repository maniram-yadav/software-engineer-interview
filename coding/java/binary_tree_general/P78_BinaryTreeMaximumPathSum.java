/**
 * LeetCode Top Interview 150 -- #78. Binary Tree Maximum Path Sum (Hard)
 *
 * Given the root of a binary tree, find the maximum path sum of any
 * non-empty path (path need not pass through the root, and a node may be
 * used at most once).
 *
 * Example:
 *   Input: root = [-10,9,20,null,null,15,7]
 *   Output: 42   (path 15 -> 20 -> 7)
 */
public class P78_BinaryTreeMaximumPathSum {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    private int maxSum;

    public int maxPathSum(TreeNode root) {
        maxSum = Integer.MIN_VALUE;
        maxGain(root);
        return maxSum;
    }

    private int maxGain(TreeNode node) {
        if (node == null) return 0;
        int leftGain = Math.max(maxGain(node.left), 0);
        int rightGain = Math.max(maxGain(node.right), 0);
        maxSum = Math.max(maxSum, node.val + leftGain + rightGain);
        return node.val + Math.max(leftGain, rightGain);
    }

    public static void main(String[] args) {
        P78_BinaryTreeMaximumPathSum sol = new P78_BinaryTreeMaximumPathSum();
        test(sol, build(1, 2, 3), 6);
        test(sol, build(-10, 9, 20, null, null, 15, 7), 42);
        test(sol, build(-3), -3);
        System.out.println("All tests passed.");
    }

    private static void test(P78_BinaryTreeMaximumPathSum sol, TreeNode root, int expected) {
        int actual = sol.maxPathSum(root);
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
