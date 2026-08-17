/**
 * Grind 169 -- #285. Inorder Successor in BST (Medium, LeetCode Premium)
 *
 * Given a BST and a node p, find the in-order successor of p (the node
 * with the smallest value greater than p.val), or null if none.
 *
 * Example:
 *   Input: root = [2,1,3], p = 1
 *   Output: 2
 */
public class P285_InorderSuccessorInBST {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public TreeNode inorderSuccessor(TreeNode root, TreeNode p) {
        TreeNode successor = null;
        TreeNode node = root;
        while (node != null) {
            if (p.val < node.val) {
                successor = node;
                node = node.left;
            } else {
                node = node.right;
            }
        }
        return successor;
    }

    public static void main(String[] args) {
        P285_InorderSuccessorInBST sol = new P285_InorderSuccessorInBST();

        TreeNode root1 = build(2, 1, 3);
        test(sol, root1, find(root1, 1), 2);

        TreeNode root2 = build(5, 3, 6, 2, 4, null, null, 1);
        test(sol, root2, find(root2, 6), -1);

        System.out.println("All tests passed.");
    }

    private static void test(P285_InorderSuccessorInBST sol, TreeNode root, TreeNode p, int expected) {
        TreeNode result = sol.inorderSuccessor(root, p);
        int actual = result == null ? -1 : result.val;
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: successor(" + p.val + ") -> " + actual);
    }

    private static TreeNode find(TreeNode root, int val) {
        if (root == null) return null;
        if (root.val == val) return root;
        return val < root.val ? find(root.left, val) : find(root.right, val);
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
