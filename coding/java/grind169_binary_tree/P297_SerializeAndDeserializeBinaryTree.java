/**
 * Grind 169 -- #297. Serialize and Deserialize Binary Tree (Hard)
 *
 * Design an algorithm to serialize a binary tree to a string and
 * deserialize that string back to the original tree structure.
 *
 * Example:
 *   Input: root = [1,2,3,null,null,4,5]
 *   Output (round-trip): [1,2,3,null,null,4,5]
 */
public class P297_SerializeAndDeserializeBinaryTree {

    static class TreeNode {
        int val;
        TreeNode left, right;

        TreeNode(int val) {
            this.val = val;
        }
    }

    public String serialize(TreeNode root) {
        StringBuilder sb = new StringBuilder();
        serializeHelper(root, sb);
        return sb.toString();
    }

    private void serializeHelper(TreeNode node, StringBuilder sb) {
        if (node == null) {
            sb.append("null,");
            return;
        }
        sb.append(node.val).append(',');
        serializeHelper(node.left, sb);
        serializeHelper(node.right, sb);
    }

    public TreeNode deserialize(String data) {
        java.util.Queue<String> queue = new java.util.LinkedList<>(java.util.Arrays.asList(data.split(",")));
        return deserializeHelper(queue);
    }

    private TreeNode deserializeHelper(java.util.Queue<String> queue) {
        String val = queue.poll();
        if (val.equals("null")) return null;
        TreeNode node = new TreeNode(Integer.parseInt(val));
        node.left = deserializeHelper(queue);
        node.right = deserializeHelper(queue);
        return node;
    }

    public static void main(String[] args) {
        P297_SerializeAndDeserializeBinaryTree sol = new P297_SerializeAndDeserializeBinaryTree();
        test(sol, build(1, 2, 3, null, null, 4, 5));
        test(sol, null);
        test(sol, build(1));
        System.out.println("All tests passed.");
    }

    private static void test(P297_SerializeAndDeserializeBinaryTree sol, TreeNode root) {
        String serialized = sol.serialize(root);
        TreeNode restored = sol.deserialize(serialized);
        java.util.List<Integer> original = toLevelOrder(root);
        java.util.List<Integer> roundTrip = toLevelOrder(restored);
        if (!original.equals(roundTrip)) {
            throw new AssertionError("Expected " + original + " but got " + roundTrip);
        }
        System.out.println("PASS: \"" + serialized + "\" -> " + roundTrip);
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

    private static java.util.List<Integer> toLevelOrder(TreeNode root) {
        java.util.List<Integer> result = new java.util.ArrayList<>();
        if (root == null) return result;
        java.util.Queue<TreeNode> queue = new java.util.LinkedList<>();
        queue.add(root);
        while (!queue.isEmpty()) {
            TreeNode node = queue.poll();
            if (node == null) {
                result.add(null);
            } else {
                result.add(node.val);
                queue.add(node.left);
                queue.add(node.right);
            }
        }
        while (!result.isEmpty() && result.get(result.size() - 1) == null) {
            result.remove(result.size() - 1);
        }
        return result;
    }
}
