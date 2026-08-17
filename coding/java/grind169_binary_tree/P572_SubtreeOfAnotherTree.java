/**
 * Grind 169 -- #572. Subtree of Another Tree (Easy)
 *
 * Given two binary trees root and subRoot, return true if subRoot has the
 * same structure and node values as some subtree of root.
 *
 * Example:
 *   Input: root = [3,4,5,1,2], subRoot = [4,1,2]
 *   Output: true
 */
public class P572_SubtreeOfAnotherTree {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public boolean isSubtree(TreeNode root, TreeNode subRoot) {
        if (root == null) return subRoot == null;
        if (isSameTree(root, subRoot)) return true;
        return isSubtree(root.left, subRoot) || isSubtree(root.right, subRoot);
    }

    private boolean isSameTree(TreeNode a, TreeNode b) {
        if (a == null && b == null) return true;
        if (a == null || b == null || a.val != b.val) return false;
        return isSameTree(a.left, b.left) && isSameTree(a.right, b.right);
    }

    public static void main(String[] args) {
        P572_SubtreeOfAnotherTree sol = new P572_SubtreeOfAnotherTree();
        test(sol, build(3, 4, 5, 1, 2), build(4, 1, 2), true);
        test(sol, build(3, 4, 5, 1, 2, null, null, null, null, 0), build(4, 1, 2), false);
        System.out.println("All tests passed.");
    }

    private static void test(P572_SubtreeOfAnotherTree sol, TreeNode root, TreeNode subRoot, boolean expected) {
        boolean actual = sol.isSubtree(root, subRoot);
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
