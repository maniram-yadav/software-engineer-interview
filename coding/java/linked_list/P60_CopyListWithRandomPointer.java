/**
 * LeetCode Top Interview 150 -- #60. Copy List with Random Pointer (Medium)
 *
 * A linked list has each node containing an extra random pointer that can
 * point to any node or null. Create a deep copy of the list.
 *
 * Example:
 *   Input: head = [[7,null],[13,0],[11,4],[10,2],[1,0]]
 *   Output: deep copy with identical val/random structure but all new nodes
 */
public class P60_CopyListWithRandomPointer {

    static class Node {
        int val;
        Node next, random;

        Node(int val) {
            this.val = val;
        }
    }

    public Node copyRandomList(Node head) {
        if (head == null) return null;

        java.util.Map<Node, Node> map = new java.util.HashMap<>();
        Node cur = head;
        while (cur != null) {
            map.put(cur, new Node(cur.val));
            cur = cur.next;
        }

        cur = head;
        while (cur != null) {
            map.get(cur).next = map.get(cur.next);
            map.get(cur).random = map.get(cur.random);
            cur = cur.next;
        }

        return map.get(head);
    }

    public static void main(String[] args) {
        P60_CopyListWithRandomPointer sol = new P60_CopyListWithRandomPointer();

        // vals[i] with randomIndex[i] (-1 means null)
        test(sol, new int[]{7, 13, 11, 10, 1}, new int[]{-1, 0, 4, 2, 0});
        test(sol, new int[]{1, 2}, new int[]{1, 0});
        test(sol, new int[]{1}, new int[]{-1});

        System.out.println("All tests passed.");
    }

    private static void test(P60_CopyListWithRandomPointer sol, int[] vals, int[] randomIndex) {
        Node head = build(vals, randomIndex);
        Node copy = sol.copyRandomList(head);

        Node origCur = head, copyCur = copy;
        int i = 0;
        while (origCur != null) {
            if (origCur == copyCur) {
                throw new AssertionError("Copy reused original node at index " + i);
            }
            if (origCur.val != copyCur.val) {
                throw new AssertionError("Value mismatch at index " + i);
            }
            int origRandomVal = origCur.random == null ? -1 : origCur.random.val;
            int copyRandomVal = copyCur.random == null ? -1 : copyCur.random.val;
            if (origRandomVal != copyRandomVal) {
                throw new AssertionError("Random pointer mismatch at index " + i);
            }
            origCur = origCur.next;
            copyCur = copyCur.next;
            i++;
        }
        if (copyCur != null) {
            throw new AssertionError("Copy list longer than original");
        }
        System.out.println("PASS: " + java.util.Arrays.toString(vals) + " random=" + java.util.Arrays.toString(randomIndex));
    }

    private static Node build(int[] vals, int[] randomIndex) {
        if (vals.length == 0) return null;
        Node[] nodes = new Node[vals.length];
        for (int i = 0; i < vals.length; i++) nodes[i] = new Node(vals[i]);
        for (int i = 0; i < vals.length - 1; i++) nodes[i].next = nodes[i + 1];
        for (int i = 0; i < vals.length; i++) {
            if (randomIndex[i] != -1) nodes[i].random = nodes[randomIndex[i]];
        }
        return nodes[0];
    }
}
