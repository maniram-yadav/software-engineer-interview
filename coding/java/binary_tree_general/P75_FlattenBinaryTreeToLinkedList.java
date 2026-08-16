/**
 * LeetCode Top Interview 150 -- #75. Flatten Binary Tree to Linked List (Medium)
 *
 * Given the root of a binary tree, flatten it in place into a "linked
 * list" following preorder traversal, using the right child pointers only.
 *
 * Example:
 *   Input: root = [1,2,5,3,4,null,6]
 *   Output: [1,null,2,null,3,null,4,null,5,null,6]
 */
public class P75_FlattenBinaryTreeToLinkedList {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    private TreeNode prev;

    public void flatten(TreeNode root) {
        prev = null;
        flattenHelper(root);
    }

    private void flattenHelper(TreeNode node) {
        if (node == null) return;
        flattenHelper(node.right);
        flattenHelper(node.left);
        node.right = prev;
        node.left = null;
        prev = node;
    }

    public static void main(String[] args) {
        P75_FlattenBinaryTreeToLinkedList sol = new P75_FlattenBinaryTreeToLinkedList();

        TreeNode root = build(1, 2, 5, 3, 4, null, 6);
        sol.flatten(root);
        test(root, new int[]{1, 2, 3, 4, 5, 6});

        TreeNode single = build(1);
        sol.flatten(single);
        test(single, new int[]{1});

        System.out.println("All tests passed.");
    }

    private static void test(TreeNode root, int[] expected) {
        java.util.List<Integer> actual = new java.util.ArrayList<>();
        TreeNode node = root;
        while (node != null) {
            if (node.left != null) {
                throw new AssertionError("left child should be null after flattening");
            }
            actual.add(node.val);
            node = node.right;
        }
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
