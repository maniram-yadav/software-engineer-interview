/**
 * Grind 169 -- #113. Path Sum II (Medium)
 *
 * Given the root of a binary tree and targetSum, return all root-to-leaf
 * paths where the sum of node values equals targetSum.
 *
 * Example:
 *   Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
 *   Output: [[5,4,11,2],[5,8,4,5]]
 */
public class P113_PathSumII {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public java.util.List<java.util.List<Integer>> pathSum(TreeNode root, int targetSum) {
        java.util.List<java.util.List<Integer>> result = new java.util.ArrayList<>();
        dfs(root, targetSum, new java.util.ArrayList<>(), result);
        return result;
    }

    private void dfs(TreeNode node, int remaining, java.util.List<Integer> path, java.util.List<java.util.List<Integer>> result) {
        if (node == null) return;
        path.add(node.val);
        if (node.left == null && node.right == null && remaining == node.val) {
            result.add(new java.util.ArrayList<>(path));
        } else {
            dfs(node.left, remaining - node.val, path, result);
            dfs(node.right, remaining - node.val, path, result);
        }
        path.remove(path.size() - 1);
    }

    public static void main(String[] args) {
        P113_PathSumII sol = new P113_PathSumII();
        TreeNode root = build(5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1);
        java.util.List<java.util.List<Integer>> actual = sol.pathSum(root, 22);
        java.util.Set<java.util.List<Integer>> actualSet = new java.util.HashSet<>(actual);
        java.util.Set<java.util.List<Integer>> expected = java.util.Set.of(
                java.util.List.of(5, 4, 11, 2), java.util.List.of(5, 8, 4, 5));
        if (!actualSet.equals(expected)) {
            throw new AssertionError("Expected " + expected + " but got " + actualSet);
        }
        System.out.println("PASS: " + actual);
        System.out.println("All tests passed.");
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
