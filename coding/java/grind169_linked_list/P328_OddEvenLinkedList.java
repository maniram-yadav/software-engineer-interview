/**
 * Grind 169 -- #328. Odd Even Linked List (Medium)
 *
 * Given the head of a singly linked list, group all nodes at odd indices
 * together followed by nodes at even indices (1-indexed), preserving
 * relative order within each group, in O(1) extra space.
 *
 * Example:
 *   Input: head = [1,2,3,4,5]
 *   Output: [1,3,5,2,4]
 */
public class P328_OddEvenLinkedList {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public ListNode oddEvenList(ListNode head) {
        if (head == null) return null;
        ListNode odd = head, even = head.next, evenHead = even;
        while (even != null && even.next != null) {
            odd.next = even.next;
            odd = odd.next;
            even.next = odd.next;
            even = even.next;
        }
        odd.next = evenHead;
        return head;
    }

    public static void main(String[] args) {
        P328_OddEvenLinkedList sol = new P328_OddEvenLinkedList();
        test(sol, new int[]{1, 2, 3, 4, 5}, new int[]{1, 3, 5, 2, 4});
        test(sol, new int[]{2, 1, 3, 5, 6, 4, 7}, new int[]{2, 3, 6, 7, 1, 5, 4});
        test(sol, new int[]{}, new int[]{});
        System.out.println("All tests passed.");
    }

    private static void test(P328_OddEvenLinkedList sol, int[] vals, int[] expected) {
        ListNode result = sol.oddEvenList(build(vals));
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
