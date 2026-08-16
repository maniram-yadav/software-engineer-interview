/**
 * LeetCode Top Interview 150 -- #68. Maximum Depth of Binary Tree (Easy)
 *
 * Given the root of a binary tree, return its maximum depth (number of
 * nodes along the longest path from root to leaf).
 *
 * Example:
 *   Input: root = [3,9,20,null,null,15,7]
 *   Output: 3
 */
public class P68_MaximumDepthOfBinaryTree {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public int maxDepth(TreeNode root) {
        if (root == null) return 0;
        return 1 + Math.max(maxDepth(root.left), maxDepth(root.right));
    }

    public static void main(String[] args) {
        P68_MaximumDepthOfBinaryTree sol = new P68_MaximumDepthOfBinaryTree();
        test(sol, build(3, 9, 20, null, null, 15, 7), 3);
        test(sol, build(1, null, 2), 2);
        test(sol, null, 0);
        System.out.println("All tests passed.");
    }

    private static void test(P68_MaximumDepthOfBinaryTree sol, TreeNode root, int expected) {
        int actual = sol.maxDepth(root);
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
