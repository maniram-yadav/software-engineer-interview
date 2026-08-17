/**
 * Grind 169 -- #235. Lowest Common Ancestor of a Binary Search Tree (Medium)
 *
 * Given a BST and two nodes p and q, find their lowest common ancestor,
 * using BST ordering properties.
 *
 * Example:
 *   Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
 *   Output: 6
 */
public class P235_LowestCommonAncestorOfABinarySearchTree {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
        TreeNode node = root;
        while (node != null) {
            if (p.val < node.val && q.val < node.val) node = node.left;
            else if (p.val > node.val && q.val > node.val) node = node.right;
            else return node;
        }
        return null;
    }

    public static void main(String[] args) {
        P235_LowestCommonAncestorOfABinarySearchTree sol = new P235_LowestCommonAncestorOfABinarySearchTree();

        TreeNode root = build(6, 2, 8, 0, 4, 7, 9, null, null, 3, 5);
        test(sol, root, find(root, 2), find(root, 8), 6);
        test(sol, root, find(root, 2), find(root, 4), 2);

        System.out.println("All tests passed.");
    }

    private static void test(P235_LowestCommonAncestorOfABinarySearchTree sol, TreeNode root, TreeNode p, TreeNode q, int expected) {
        TreeNode result = sol.lowestCommonAncestor(root, p, q);
        if (result == null || result.val != expected) {
            throw new AssertionError("Expected " + expected + " but got " + (result == null ? "null" : result.val));
        }
        System.out.println("PASS: LCA(" + p.val + "," + q.val + ") -> " + result.val);
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
