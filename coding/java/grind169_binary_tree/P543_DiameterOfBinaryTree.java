/**
 * Grind 169 -- #543. Diameter of Binary Tree (Easy)
 *
 * Given the root of a binary tree, return the length (in edges) of the
 * longest path between any two nodes (may or may not pass through the
 * root).
 *
 * Example:
 *   Input: root = [1,2,3,4,5]
 *   Output: 3   (path [4,2,1,3] or [5,2,1,3])
 */
public class P543_DiameterOfBinaryTree {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    private int diameter;

    public int diameterOfBinaryTree(TreeNode root) {
        diameter = 0;
        depth(root);
        return diameter;
    }

    private int depth(TreeNode node) {
        if (node == null) return 0;
        int left = depth(node.left);
        int right = depth(node.right);
        diameter = Math.max(diameter, left + right);
        return 1 + Math.max(left, right);
    }

    public static void main(String[] args) {
        P543_DiameterOfBinaryTree sol = new P543_DiameterOfBinaryTree();
        test(sol, build(1, 2, 3, 4, 5), 3);
        test(sol, build(1, 2), 1);
        System.out.println("All tests passed.");
    }

    private static void test(P543_DiameterOfBinaryTree sol, TreeNode root, int expected) {
        int actual = sol.diameterOfBinaryTree(root);
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
