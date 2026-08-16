/**
 * LeetCode Top Interview 150 -- #82. Binary Tree Right Side View (Medium)
 *
 * Given the root of a binary tree, return the values visible from the
 * right side, ordered top to bottom.
 *
 * Example:
 *   Input: root = [1,2,3,null,5,null,4]
 *   Output: [1,3,4]
 */
public class P82_BinaryTreeRightSideView {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public java.util.List<Integer> rightSideView(TreeNode root) {
        java.util.List<Integer> result = new java.util.ArrayList<>();
        if (root == null) return result;

        java.util.Queue<TreeNode> queue = new java.util.LinkedList<>();
        queue.add(root);
        while (!queue.isEmpty()) {
            int size = queue.size();
            for (int i = 0; i < size; i++) {
                TreeNode node = queue.poll();
                if (i == size - 1) result.add(node.val);
                if (node.left != null) queue.add(node.left);
                if (node.right != null) queue.add(node.right);
            }
        }
        return result;
    }

    public static void main(String[] args) {
        P82_BinaryTreeRightSideView sol = new P82_BinaryTreeRightSideView();
        test(sol, build(1, 2, 3, null, 5, null, 4), new int[]{1, 3, 4});
        test(sol, build(1, null, 3), new int[]{1, 3});
        test(sol, null, new int[]{});
        System.out.println("All tests passed.");
    }

    private static void test(P82_BinaryTreeRightSideView sol, TreeNode root, int[] expected) {
        java.util.List<Integer> actual = sol.rightSideView(root);
        int[] actualArr = actual.stream().mapToInt(Integer::intValue).toArray();
        if (!java.util.Arrays.equals(actualArr, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + actual);
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
}
