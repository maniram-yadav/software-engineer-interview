/**
 * LeetCode Top Interview 150 -- #73. Construct Binary Tree from Inorder and Postorder Traversal (Medium)
 *
 * Same idea as building from preorder/inorder, but from inorder and
 * postorder traversals.
 *
 * Example:
 *   Input: inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
 *   Output: [3,9,20,null,null,15,7]
 */
public class P73_ConstructBinaryTreeFromInorderAndPostorderTraversal {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    private int postIndex;

    public TreeNode buildTree(int[] inorder, int[] postorder) {
        postIndex = postorder.length - 1;
        java.util.Map<Integer, Integer> inorderIndex = new java.util.HashMap<>();
        for (int i = 0; i < inorder.length; i++) inorderIndex.put(inorder[i], i);
        return build(postorder, inorderIndex, 0, inorder.length - 1);
    }

    private TreeNode build(int[] postorder, java.util.Map<Integer, Integer> inorderIndex, int left, int right) {
        if (left > right) return null;
        int rootVal = postorder[postIndex--];
        TreeNode root = new TreeNode(rootVal);
        int mid = inorderIndex.get(rootVal);
        root.right = build(postorder, inorderIndex, mid + 1, right);
        root.left = build(postorder, inorderIndex, left, mid - 1);
        return root;
    }

    public static void main(String[] args) {
        P73_ConstructBinaryTreeFromInorderAndPostorderTraversal sol = new P73_ConstructBinaryTreeFromInorderAndPostorderTraversal();
        test(sol, new int[]{9, 3, 15, 20, 7}, new int[]{9, 15, 7, 20, 3}, new Integer[]{3, 9, 20, null, null, 15, 7});
        test(sol, new int[]{-1}, new int[]{-1}, new Integer[]{-1});
        test(sol, new int[]{2, 1}, new int[]{2, 1}, new Integer[]{1, 2});
        System.out.println("All tests passed.");
    }

    private static void test(P73_ConstructBinaryTreeFromInorderAndPostorderTraversal sol, int[] inorder, int[] postorder, Integer[] expected) {
        TreeNode result = sol.buildTree(inorder, postorder);
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
