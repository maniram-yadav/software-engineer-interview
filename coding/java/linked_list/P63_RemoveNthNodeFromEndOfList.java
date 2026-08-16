/**
 * LeetCode Top Interview 150 -- #63. Remove Nth Node From End of List (Medium)
 *
 * Given the head of a linked list, remove the n-th node from the end and
 * return the head, in one pass.
 *
 * Example:
 *   Input: head = [1,2,3,4,5], n = 2
 *   Output: [1,2,3,5]
 */
public class P63_RemoveNthNodeFromEndOfList {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public ListNode removeNthFromEnd(ListNode head, int n) {
        ListNode dummy = new ListNode(0);
        dummy.next = head;
        ListNode fast = dummy, slow = dummy;

        for (int i = 0; i < n; i++) fast = fast.next;
        while (fast.next != null) {
            fast = fast.next;
            slow = slow.next;
        }
        slow.next = slow.next.next;
        return dummy.next;
    }

    public static void main(String[] args) {
        P63_RemoveNthNodeFromEndOfList sol = new P63_RemoveNthNodeFromEndOfList();
        test(sol, new int[]{1, 2, 3, 4, 5}, 2, new int[]{1, 2, 3, 5});
        test(sol, new int[]{1}, 1, new int[]{});
        test(sol, new int[]{1, 2}, 1, new int[]{1});
        System.out.println("All tests passed.");
    }

    private static void test(P63_RemoveNthNodeFromEndOfList sol, int[] vals, int n, int[] expected) {
        ListNode result = sol.removeNthFromEnd(build(vals), n);
        int[] actual = toArray(result);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(vals) + " n=" + n + " -> " + java.util.Arrays.toString(actual));
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
