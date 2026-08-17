/**
 * Grind 169 -- #143. Reorder List (Medium)
 *
 * Given the head of a linked list L0 -> L1 -> ... -> Ln-1 -> Ln, reorder
 * it in place to L0 -> Ln -> L1 -> Ln-1 -> L2 -> Ln-2 -> ...
 *
 * Example:
 *   Input: head = [1,2,3,4]
 *   Output: [1,4,2,3]
 */
public class P143_ReorderList {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public void reorderList(ListNode head) {
        if (head == null || head.next == null) return;

        ListNode slow = head, fast = head;
        while (fast.next != null && fast.next.next != null) {
            slow = slow.next;
            fast = fast.next.next;
        }

        ListNode second = slow.next;
        slow.next = null;
        ListNode prev = null;
        while (second != null) {
            ListNode next = second.next;
            second.next = prev;
            prev = second;
            second = next;
        }

        ListNode first = head;
        second = prev;
        while (second != null) {
            ListNode n1 = first.next, n2 = second.next;
            first.next = second;
            second.next = n1;
            first = n1;
            second = n2;
        }
    }

    public static void main(String[] args) {
        P143_ReorderList sol = new P143_ReorderList();
        test(sol, new int[]{1, 2, 3, 4}, new int[]{1, 4, 2, 3});
        test(sol, new int[]{1, 2, 3, 4, 5}, new int[]{1, 5, 2, 4, 3});
        test(sol, new int[]{1}, new int[]{1});
        System.out.println("All tests passed.");
    }

    private static void test(P143_ReorderList sol, int[] vals, int[] expected) {
        ListNode head = build(vals);
        sol.reorderList(head);
        int[] actual = toArray(head);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(vals) + " -> " + java.util.Arrays.toString(actual));
    }

    private static ListNode build(int[] vals) {
        ListNode dummy = new ListNode(0);
        ListNode cur = dummy;
        for (int v : vals) {
            cur.next = new ListNode(v);
            cur = cur.next;
        }
        return dummy.next;
    }

    private static int[] toArray(ListNode head) {
        java.util.List<Integer> list = new java.util.ArrayList<>();
        while (head != null) {
            list.add(head.val);
            head = head.next;
        }
        return list.stream().mapToInt(Integer::intValue).toArray();
    }
}
