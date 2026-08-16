/**
 * LeetCode Top Interview 150 -- #81. Lowest Common Ancestor of a Binary Tree (Medium)
 *
 * Given a binary tree and two nodes p and q, find their lowest common
 * ancestor.
 *
 * Example:
 *   Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
 *   Output: 3
 */
public class P81_LowestCommonAncestorOfABinaryTree {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
        if (root == null || root == p || root == q) return root;
        TreeNode left = lowestCommonAncestor(root.left, p, q);
        TreeNode right = lowestCommonAncestor(root.right, p, q);
        if (left != null && right != null) return root;
        return left != null ? left : right;
    }

    public static void main(String[] args) {
        P81_LowestCommonAncestorOfABinaryTree sol = new P81_LowestCommonAncestorOfABinaryTree();

        TreeNode root = build(3, 5, 1, 6, 2, 0, 8, null, null, 7, 4);
        test(sol, root, find(root, 5), find(root, 1), 3);
        test(sol, root, find(root, 5), find(root, 4), 5);

        TreeNode small = build(1, 2);
        test(sol, small, find(small, 1), find(small, 2), 1);

        System.out.println("All tests passed.");
    }

    private static void test(P81_LowestCommonAncestorOfABinaryTree sol, TreeNode root, TreeNode p, TreeNode q, int expected) {
        TreeNode result = sol.lowestCommonAncestor(root, p, q);
        if (result == null || result.val != expected) {
            throw new AssertionError("Expected " + expected + " but got " + (result == null ? "null" : result.val));
        }
        System.out.println("PASS: LCA(" + p.val + ", " + q.val + ") -> " + result.val);
    }

    private static TreeNode find(TreeNode root, int val) {
        if (root == null) return null;
        if (root.val == val) return root;
        TreeNode left = find(root.left, val);
        return left != null ? left : find(root.right, val);
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
