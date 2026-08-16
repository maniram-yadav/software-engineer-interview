/**
 * LeetCode Top Interview 150 -- #77. Sum Root to Leaf Numbers (Medium)
 *
 * Each root-to-leaf path represents a number (digits left to right).
 * Return the total sum of all root-to-leaf numbers.
 *
 * Example:
 *   Input: root = [4,9,0,5,1]
 *   Output: 1026   (495 + 491 + 40)
 */
public class P77_SumRootToLeafNumbers {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public int sumNumbers(TreeNode root) {
        return dfs(root, 0);
    }

    private int dfs(TreeNode node, int current) {
        if (node == null) return 0;
        current = current * 10 + node.val;
        if (node.left == null && node.right == null) return current;
        return dfs(node.left, current) + dfs(node.right, current);
    }

    public static void main(String[] args) {
        P77_SumRootToLeafNumbers sol = new P77_SumRootToLeafNumbers();
        test(sol, build(4, 9, 0, 5, 1), 1026);
        test(sol, build(1, 2, 3), 25);
        test(sol, build(0), 0);
        System.out.println("All tests passed.");
    }

    private static void test(P77_SumRootToLeafNumbers sol, TreeNode root, int expected) {
        int actual = sol.sumNumbers(root);
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
