/**
 * Grind 169 -- #863. All Nodes Distance K in Binary Tree (Medium)
 *
 * Given the root of a binary tree, a target node, and an integer k, return
 * the values of all nodes that are exactly distance k from the target
 * node.
 *
 * Example:
 *   Input: root = [3,5,1,6,2,0,8,null,null,7,4], target = 5, k = 2
 *   Output: [7,4,1]
 */
public class P863_AllNodesDistanceKInBinaryTree {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public java.util.List<Integer> distanceK(TreeNode root, TreeNode target, int k) {
        java.util.Map<TreeNode, TreeNode> parents = new java.util.HashMap<>();
        buildParents(root, null, parents);

        java.util.Queue<TreeNode> queue = new java.util.LinkedList<>();
        java.util.Set<TreeNode> visited = new java.util.HashSet<>();
        queue.add(target);
        visited.add(target);

        int dist = 0;
        while (!queue.isEmpty()) {
            if (dist == k) {
                java.util.List<Integer> result = new java.util.ArrayList<>();
                for (TreeNode node : queue) result.add(node.val);
                return result;
            }
            int size = queue.size();
            for (int i = 0; i < size; i++) {
                TreeNode node = queue.poll();
                TreeNode[] neighbors = {node.left, node.right, parents.get(node)};
                for (TreeNode neighbor : neighbors) {
                    if (neighbor != null && visited.add(neighbor)) queue.add(neighbor);
                }
            }
            dist++;
        }
        return new java.util.ArrayList<>();
    }

    private void buildParents(TreeNode node, TreeNode parent, java.util.Map<TreeNode, TreeNode> parents) {
        if (node == null) return;
        parents.put(node, parent);
        buildParents(node.left, node, parents);
        buildParents(node.right, node, parents);
    }

    public static void main(String[] args) {
        P863_AllNodesDistanceKInBinaryTree sol = new P863_AllNodesDistanceKInBinaryTree();

        TreeNode root = build(3, 5, 1, 6, 2, 0, 8, null, null, 7, 4);
        TreeNode target = find(root, 5);
        java.util.List<Integer> actual = sol.distanceK(root, target, 2);
        java.util.Set<Integer> actualSet = new java.util.HashSet<>(actual);
        java.util.Set<Integer> expected = java.util.Set.of(7, 4, 1);
        if (!actualSet.equals(expected)) {
            throw new AssertionError("Expected " + expected + " but got " + actualSet);
        }
        System.out.println("PASS: " + actual);

        java.util.List<Integer> actual0 = sol.distanceK(root, target, 0);
        if (!actual0.equals(java.util.List.of(5))) {
            throw new AssertionError("Expected [5] but got " + actual0);
        }
        System.out.println("PASS: " + actual0);

        System.out.println("All tests passed.");
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
