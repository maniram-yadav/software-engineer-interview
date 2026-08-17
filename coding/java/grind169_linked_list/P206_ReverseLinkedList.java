/**
 * Grind 169 -- #206. Reverse Linked List (Easy)
 *
 * Given the head of a singly linked list, reverse the list and return the
 * new head.
 *
 * Example:
 *   Input: head = [1,2,3,4,5]
 *   Output: [5,4,3,2,1]
 */
public class P206_ReverseLinkedList {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public ListNode reverseList(ListNode head) {
        ListNode prev = null, cur = head;
        while (cur != null) {
            ListNode next = cur.next;
            cur.next = prev;
            prev = cur;
            cur = next;
        }
        return prev;
    }

    public static void main(String[] args) {
        P206_ReverseLinkedList sol = new P206_ReverseLinkedList();
        test(sol, new int[]{1, 2, 3, 4, 5}, new int[]{5, 4, 3, 2, 1});
        test(sol, new int[]{}, new int[]{});
        test(sol, new int[]{1}, new int[]{1});
        System.out.println("All tests passed.");
    }

    private static void test(P206_ReverseLinkedList sol, int[] vals, int[] expected) {
        ListNode result = sol.reverseList(build(vals));
        int[] actual = toArray(result);
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
