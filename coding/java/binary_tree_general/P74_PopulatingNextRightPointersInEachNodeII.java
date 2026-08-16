/**
 * LeetCode Top Interview 150 -- #74. Populating Next Right Pointers in Each Node II (Medium)
 *
 * Given a binary tree (not necessarily perfect) where each node has a next
 * pointer, populate each next pointer to point to its next right node on
 * the same level, or null.
 *
 * Example:
 *   Input: root = [1,2,3,4,5,null,7]
 *   Output: [1,#,2,3,#,4,5,7,#]   (# marks end of each level)
 */
public class P74_PopulatingNextRightPointersInEachNodeII {

    static class Node {
        int val;
        Node left, right, next;

        Node(int val) {
            this.val = val;
        }
    }

    public Node connect(Node root) {
        Node levelStart = root;
        while (levelStart != null) {
            Node dummy = new Node(0);
            Node tail = dummy;
            Node node = levelStart;
            while (node != null) {
                if (node.left != null) {
                    tail.next = node.left;
                    tail = tail.next;
                }
                if (node.right != null) {
                    tail.next = node.right;
                    tail = tail.next;
                }
                node = node.next;
            }
            levelStart = dummy.next;
        }
        return root;
    }

    public static void main(String[] args) {
        P74_PopulatingNextRightPointersInEachNodeII sol = new P74_PopulatingNextRightPointersInEachNodeII();

        Node root = build(1, 2, 3, 4, 5, null, 7);
        sol.connect(root);
        test(root, new java.util.List[]{
                java.util.List.of(1),
                java.util.List.of(2, 3),
                java.util.List.of(4, 5, 7)
        });

        Node single = build(1);
        sol.connect(single);
        test(single, new java.util.List[]{java.util.List.of(1)});

        System.out.println("All tests passed.");
    }

    private static void test(Node root, java.util.List<Integer>[] expectedLevels) {
        java.util.List<java.util.List<Integer>> actual = new java.util.ArrayList<>();
        Node levelStart = root;
        while (levelStart != null) {
            java.util.List<Integer> level = new java.util.ArrayList<>();
            Node node = levelStart;
            while (node != null) {
                level.add(node.val);
                node = node.next;
            }
            actual.add(level);
            levelStart = findNextLevelStart(levelStart);
        }
        java.util.List<java.util.List<Integer>> expected = java.util.Arrays.asList(expectedLevels);
        if (!actual.equals(expected)) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + actual);
    }

    private static Node findNextLevelStart(Node levelStart) {
        Node node = levelStart;
        while (node != null) {
            if (node.left != null) return node.left;
            if (node.right != null) return node.right;
            node = node.next;
        }
        return null;
    }

    private static Node build(Integer... values) {
        if (values.length == 0 || values[0] == null) return null;
        Node root = new Node(values[0]);
        java.util.Queue<Node> queue = new java.util.LinkedList<>();
        queue.add(root);
        int i = 1;
        while (!queue.isEmpty() && i < values.length) {
            Node node = queue.poll();
            if (i < values.length) {
                Integer leftVal = values[i++];
                if (leftVal != null) {
                    node.left = new Node(leftVal);
                    queue.add(node.left);
                }
            }
            if (i < values.length) {
                Integer rightVal = values[i++];
                if (rightVal != null) {
                    node.right = new Node(rightVal);
                    queue.add(node.right);
                }
            }
        }
        return root;
    }
}
