/**
 * Grind 169 -- #662. Maximum Width of Binary Tree (Medium)
 *
 * Given the root of a binary tree, return the maximum width of any level
 * (distance between leftmost and rightmost non-null nodes, counting nulls
 * in between as if the tree were complete).
 *
 * Example:
 *   Input: root = [1,3,2,5,3,null,9]
 *   Output: 4
 */
public class P662_MaximumWidthOfBinaryTree {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public int widthOfBinaryTree(TreeNode root) {
        if (root == null) return 0;
        int maxWidth = 0;
        java.util.Queue<TreeNode> nodeQueue = new java.util.LinkedList<>();
        java.util.Queue<Long> indexQueue = new java.util.LinkedList<>();
        nodeQueue.add(root);
        indexQueue.add(0L);

        while (!nodeQueue.isEmpty()) {
            int size = nodeQueue.size();
            long first = 0, last = 0;
            for (int i = 0; i < size; i++) {
                TreeNode node = nodeQueue.poll();
                long idx = indexQueue.poll();
                if (i == 0) first = idx;
                if (i == size - 1) last = idx;
                if (node.left != null) {
                    nodeQueue.add(node.left);
                    indexQueue.add(idx * 2);
                }
                if (node.right != null) {
                    nodeQueue.add(node.right);
                    indexQueue.add(idx * 2 + 1);
                }
            }
            maxWidth = Math.max(maxWidth, (int) (last - first + 1));
        }
        return maxWidth;
    }

    public static void main(String[] args) {
        P662_MaximumWidthOfBinaryTree sol = new P662_MaximumWidthOfBinaryTree();
        test(sol, build(1, 3, 2, 5, 3, null, 9), 4);
        test(sol, build(1, 3, 2, 5), 2);
        System.out.println("All tests passed.");
    }

    private static void test(P662_MaximumWidthOfBinaryTree sol, TreeNode root, int expected) {
        int actual = sol.widthOfBinaryTree(root);
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
