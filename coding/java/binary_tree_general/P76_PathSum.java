/**
 * LeetCode Top Interview 150 -- #76. Path Sum (Easy)
 *
 * Given the root of a binary tree and an integer targetSum, return true if
 * the tree has a root-to-leaf path such that the values sum to targetSum.
 *
 * Example:
 *   Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
 *   Output: true
 */
public class P76_PathSum {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public boolean hasPathSum(TreeNode root, int targetSum) {
        if (root == null) return false;
        if (root.left == null && root.right == null) return root.val == targetSum;
        int remaining = targetSum - root.val;
        return hasPathSum(root.left, remaining) || hasPathSum(root.right, remaining);
    }

    public static void main(String[] args) {
        P76_PathSum sol = new P76_PathSum();
        test(sol, build(5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1), 22, true);
        test(sol, build(1, 2, 3), 5, false);
        test(sol, null, 0, false);
        System.out.println("All tests passed.");
    }

    private static void test(P76_PathSum sol, TreeNode root, int targetSum, boolean expected) {
        boolean actual = sol.hasPathSum(root, targetSum);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: targetSum=" + targetSum + " -> " + actual);
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
