/**
 * LeetCode Top Interview 150 -- #57. Linked List Cycle (Easy)
 *
 * Given the head of a linked list, determine if it has a cycle, using O(1)
 * extra space (Floyd's cycle detection).
 *
 * Example:
 *   Input: head = 3 -> 2 -> 0 -> -4 -> (back to node with value 2)
 *   Output: true
 */
public class P57_LinkedListCycle {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public boolean hasCycle(ListNode head) {
        ListNode slow = head, fast = head;
        while (fast != null && fast.next != null) {
            slow = slow.next;
            fast = fast.next.next;
            if (slow == fast) return true;
        }
        return false;
    }

    public static void main(String[] args) {
        P57_LinkedListCycle sol = new P57_LinkedListCycle();

        ListNode a = new ListNode(3);
        ListNode b = new ListNode(2);
        ListNode c = new ListNode(0);
        ListNode d = new ListNode(-4);
        a.next = b;
        b.next = c;
        c.next = d;
        d.next = b;
        test(sol, a, true, "3->2->0->-4->(cycle to 2)");

        ListNode e = new ListNode(1);
        ListNode f = new ListNode(2);
        e.next = f;
        test(sol, e, false, "1->2");

        test(sol, null, false, "empty list");
        test(sol, new ListNode(1), false, "single node, no cycle");

        System.out.println("All tests passed.");
    }

    private static void test(P57_LinkedListCycle sol, ListNode head, boolean expected, String label) {
        boolean actual = sol.hasCycle(head);
        if (actual != expected) {
            throw new AssertionError(label + ": expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + label + " -> " + actual);
    }
}
