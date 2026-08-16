/**
 * LeetCode Top Interview 150 -- #72. Construct Binary Tree from Preorder and Inorder Traversal (Medium)
 *
 * Given two integer arrays preorder and inorder (no duplicate values)
 * representing a binary tree's traversals, construct and return the tree.
 *
 * Example:
 *   Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
 *   Output: [3,9,20,null,null,15,7]
 */
public class P72_ConstructBinaryTreeFromPreorderAndInorderTraversal {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    private int preIndex;

    public TreeNode buildTree(int[] preorder, int[] inorder) {
        preIndex = 0;
        java.util.Map<Integer, Integer> inorderIndex = new java.util.HashMap<>();
        for (int i = 0; i < inorder.length; i++) inorderIndex.put(inorder[i], i);
        return build(preorder, inorderIndex, 0, inorder.length - 1);
    }

    private TreeNode build(int[] preorder, java.util.Map<Integer, Integer> inorderIndex, int left, int right) {
        if (left > right) return null;
        int rootVal = preorder[preIndex++];
        TreeNode root = new TreeNode(rootVal);
        int mid = inorderIndex.get(rootVal);
        root.left = build(preorder, inorderIndex, left, mid - 1);
        root.right = build(preorder, inorderIndex, mid + 1, right);
        return root;
    }

    public static void main(String[] args) {
        P72_ConstructBinaryTreeFromPreorderAndInorderTraversal sol = new P72_ConstructBinaryTreeFromPreorderAndInorderTraversal();
        test(sol, new int[]{3, 9, 20, 15, 7}, new int[]{9, 3, 15, 20, 7}, new Integer[]{3, 9, 20, null, null, 15, 7});
        test(sol, new int[]{-1}, new int[]{-1}, new Integer[]{-1});
        test(sol, new int[]{1, 2}, new int[]{2, 1}, new Integer[]{1, 2});
        System.out.println("All tests passed.");
    }

    private static void test(P72_ConstructBinaryTreeFromPreorderAndInorderTraversal sol, int[] preorder, int[] inorder, Integer[] expected) {
        TreeNode result = sol.buildTree(preorder, inorder);
        java.util.List<Integer> actual = toLevelOrder(result);
        java.util.List<Integer> expectedList = java.util.Arrays.asList(expected);
        if (!actual.equals(expectedList)) {
            throw new AssertionError("Expected " + expectedList + " but got " + actual);
        }
        System.out.println("PASS: " + actual);
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
