/**
 * Grind 169 -- #437. Path Sum III (Medium)
 *
 * Given the root of a binary tree and an integer targetSum, return the
 * number of paths (not necessarily root-to-leaf, but must go downward)
 * where the sum equals targetSum.
 *
 * Example:
 *   Input: root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8
 *   Output: 3
 */
public class P437_PathSumIII {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public int pathSum(TreeNode root, int targetSum) {
        java.util.Map<Long, Integer> prefixCount = new java.util.HashMap<>();
        prefixCount.put(0L, 1);
        return dfs(root, 0L, targetSum, prefixCount);
    }

    private int dfs(TreeNode node, long currentSum, int targetSum, java.util.Map<Long, Integer> prefixCount) {
        if (node == null) return 0;
        currentSum += node.val;
        int count = prefixCount.getOrDefault(currentSum - targetSum, 0);
        prefixCount.merge(currentSum, 1, Integer::sum);
        count += dfs(node.left, currentSum, targetSum, prefixCount);
        count += dfs(node.right, currentSum, targetSum, prefixCount);
        prefixCount.merge(currentSum, -1, Integer::sum);
        return count;
    }

    public static void main(String[] args) {
        P437_PathSumIII sol = new P437_PathSumIII();
        test(sol, build(10, 5, -3, 3, 2, null, 11, 3, -2, null, 1), 8, 3);
        test(sol, build(5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1), 22, 3);
        System.out.println("All tests passed.");
    }

    private static void test(P437_PathSumIII sol, TreeNode root, int targetSum, int expected) {
        int actual = sol.pathSum(root, targetSum);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: targetSum=" + targetSum + " -> " + actual);
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
