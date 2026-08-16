/**
 * LeetCode Top Interview 150 -- #83. Average of Levels in Binary Tree (Easy)
 *
 * Given the root of a binary tree, return the average value of nodes at
 * each level.
 *
 * Example:
 *   Input: root = [3,9,20,null,null,15,7]
 *   Output: [3.0,14.5,11.0]
 */
public class P83_AverageOfLevelsInBinaryTree {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public java.util.List<Double> averageOfLevels(TreeNode root) {
        java.util.List<Double> result = new java.util.ArrayList<>();
        java.util.Queue<TreeNode> queue = new java.util.LinkedList<>();
        queue.add(root);
        while (!queue.isEmpty()) {
            int size = queue.size();
            double sum = 0;
            for (int i = 0; i < size; i++) {
                TreeNode node = queue.poll();
                sum += node.val;
                if (node.left != null) queue.add(node.left);
                if (node.right != null) queue.add(node.right);
            }
            result.add(sum / size);
        }
        return result;
    }

    public static void main(String[] args) {
        P83_AverageOfLevelsInBinaryTree sol = new P83_AverageOfLevelsInBinaryTree();
        test(sol, build(3, 9, 20, null, null, 15, 7), new double[]{3.0, 14.5, 11.0});
        test(sol, build(1), new double[]{1.0});
        test(sol, build(1, 2, 3, 4), new double[]{1.0, 2.5, 4.0});
        System.out.println("All tests passed.");
    }

    private static void test(P83_AverageOfLevelsInBinaryTree sol, TreeNode root, double[] expected) {
        java.util.List<Double> actual = sol.averageOfLevels(root);
        for (int i = 0; i < expected.length; i++) {
            if (Math.abs(actual.get(i) - expected[i]) > 1e-9) {
                throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + actual);
            }
        }
        if (actual.size() != expected.length) {
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
