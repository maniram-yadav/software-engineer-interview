/**
 * LeetCode Top Interview 150 -- #108. Convert Sorted Array to Binary Search Tree (Easy)
 *
 * Given an integer array sorted ascending, convert it to a height-balanced
 * BST.
 *
 * Example:
 *   Input: nums = [-10,-3,0,5,9]
 *   Output: [0,-3,9,-10,null,5]  (one valid height-balanced BST)
 */
public class P108_ConvertSortedArrayToBinarySearchTree {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public TreeNode sortedArrayToBST(int[] nums) {
        return build(nums, 0, nums.length - 1);
    }

    private TreeNode build(int[] nums, int left, int right) {
        if (left > right) return null;
        int mid = left + (right - left) / 2;
        TreeNode node = new TreeNode(nums[mid]);
        node.left = build(nums, left, mid - 1);
        node.right = build(nums, mid + 1, right);
        return node;
    }

    public static void main(String[] args) {
        P108_ConvertSortedArrayToBinarySearchTree sol = new P108_ConvertSortedArrayToBinarySearchTree();
        test(sol, new int[]{-10, -3, 0, 5, 9});
        test(sol, new int[]{1, 3});
        test(sol, new int[]{1});
        System.out.println("All tests passed.");
    }

    private static void test(P108_ConvertSortedArrayToBinarySearchTree sol, int[] nums) {
        TreeNode root = sol.sortedArrayToBST(nums);

        java.util.List<Integer> inorder = new java.util.ArrayList<>();
        collectInorder(root, inorder);
        int[] actual = inorder.stream().mapToInt(Integer::intValue).toArray();
        if (!java.util.Arrays.equals(actual, nums)) {
            throw new AssertionError("Inorder traversal " + inorder + " does not match sorted input " + java.util.Arrays.toString(nums));
        }

        if (!isBalanced(root)) {
            throw new AssertionError("Tree is not height-balanced for input " + java.util.Arrays.toString(nums));
        }

        System.out.println("PASS: " + java.util.Arrays.toString(nums) + " -> balanced BST with inorder " + inorder);
    }

    private static void collectInorder(TreeNode node, java.util.List<Integer> result) {
        if (node == null) return;
        collectInorder(node.left, result);
        result.add(node.val);
        collectInorder(node.right, result);
    }

    private static boolean isBalanced(TreeNode node) {
        return height(node) != -1;
    }

    private static int height(TreeNode node) {
        if (node == null) return 0;
        int left = height(node.left);
        if (left == -1) return -1;
        int right = height(node.right);
        if (right == -1) return -1;
        if (Math.abs(left - right) > 1) return -1;
        return 1 + Math.max(left, right);
    }
}
