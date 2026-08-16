/**
 * LeetCode Top Interview 150 -- #85. Binary Tree Zigzag Level Order Traversal (Medium)
 *
 * Same as level order, but alternate direction each level (left-to-right,
 * then right-to-left, ...).
 *
 * Example:
 *   Input: root = [3,9,20,null,null,15,7]
 *   Output: [[3],[20,9],[15,7]]
 */
public class P85_BinaryTreeZigzagLevelOrderTraversal {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public java.util.List<java.util.List<Integer>> zigzagLevelOrder(TreeNode root) {
        java.util.List<java.util.List<Integer>> result = new java.util.ArrayList<>();
        if (root == null) return result;

        java.util.Queue<TreeNode> queue = new java.util.LinkedList<>();
        queue.add(root);
        boolean leftToRight = true;
        while (!queue.isEmpty()) {
            int size = queue.size();
            java.util.LinkedList<Integer> level = new java.util.LinkedList<>();
            for (int i = 0; i < size; i++) {
                TreeNode node = queue.poll();
                if (leftToRight) level.addLast(node.val);
                else level.addFirst(node.val);
                if (node.left != null) queue.add(node.left);
                if (node.right != null) queue.add(node.right);
            }
            result.add(level);
            leftToRight = !leftToRight;
        }
        return result;
    }

    public static void main(String[] args) {
        P85_BinaryTreeZigzagLevelOrderTraversal sol = new P85_BinaryTreeZigzagLevelOrderTraversal();
        test(sol, build(3, 9, 20, null, null, 15, 7), "[[3], [20, 9], [15, 7]]");
        test(sol, build(1), "[[1]]");
        test(sol, null, "[]");
        System.out.println("All tests passed.");
    }

    private static void test(P85_BinaryTreeZigzagLevelOrderTraversal sol, TreeNode root, String expected) {
        java.util.List<java.util.List<Integer>> actual = sol.zigzagLevelOrder(root);
        if (!actual.toString().equals(expected)) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
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
