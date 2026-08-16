/**
 * LeetCode Top Interview 150 -- #70. Invert Binary Tree (Easy)
 *
 * Given the root of a binary tree, invert it (mirror left/right children
 * recursively) and return the root.
 *
 * Example:
 *   Input: root = [4,2,7,1,3,6,9]
 *   Output: [4,7,2,9,6,3,1]
 */
public class P70_InvertBinaryTree {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public TreeNode invertTree(TreeNode root) {
        if (root == null) return null;
        TreeNode left = invertTree(root.left);
        TreeNode right = invertTree(root.right);
        root.left = right;
        root.right = left;
        return root;
    }

    public static void main(String[] args) {
        P70_InvertBinaryTree sol = new P70_InvertBinaryTree();
        test(sol, build(4, 2, 7, 1, 3, 6, 9), new Integer[]{4, 7, 2, 9, 6, 3, 1});
        test(sol, build(2, 1, 3), new Integer[]{2, 3, 1});
        test(sol, null, new Integer[]{});
        System.out.println("All tests passed.");
    }

    private static void test(P70_InvertBinaryTree sol, TreeNode root, Integer[] expected) {
        TreeNode result = sol.invertTree(root);
        java.util.List<Integer> actual = toLevelOrder(result);
        java.util.List<Integer> expectedList = java.util.Arrays.asList(expected);
        if (!actual.equals(expectedList)) {
            throw new AssertionError("Expected " + expectedList + " but got " + actual);
        }
        System.out.println("PASS: " + actual);
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

    private static java.util.List<Integer> toLevelOrder(TreeNode root) {
        java.util.List<Integer> result = new java.util.ArrayList<>();
        if (root == null) return result;
        java.util.Queue<TreeNode> queue = new java.util.LinkedList<>();
        queue.add(root);
        while (!queue.isEmpty()) {
            TreeNode node = queue.poll();
            if (node == null) {
                result.add(null);
            } else {
                result.add(node.val);
                queue.add(node.left);
                queue.add(node.right);
            }
        }
        while (!result.isEmpty() && result.get(result.size() - 1) == null) {
            result.remove(result.size() - 1);
        }
        return result;
    }
}
