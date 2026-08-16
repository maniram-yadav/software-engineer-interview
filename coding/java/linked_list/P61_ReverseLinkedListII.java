/**
 * LeetCode Top Interview 150 -- #61. Reverse Linked List II (Medium)
 *
 * Given the head of a singly linked list and positions left/right, reverse
 * the nodes between those positions (1-indexed) and return the head.
 *
 * Example:
 *   Input: head = [1,2,3,4,5], left = 2, right = 4
 *   Output: [1,4,3,2,5]
 */
public class P61_ReverseLinkedListII {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public ListNode reverseBetween(ListNode head, int left, int right) {
        ListNode dummy = new ListNode(0);
        dummy.next = head;
        ListNode prev = dummy;
        for (int i = 1; i < left; i++) prev = prev.next;

        ListNode cur = prev.next;
        for (int i = 0; i < right - left; i++) {
            ListNode next = cur.next;
            cur.next = next.next;
            next.next = prev.next;
            prev.next = next;
        }
        return dummy.next;
    }

    public static void main(String[] args) {
        P61_ReverseLinkedListII sol = new P61_ReverseLinkedListII();
        test(sol, new int[]{1, 2, 3, 4, 5}, 2, 4, new int[]{1, 4, 3, 2, 5});
        test(sol, new int[]{5}, 1, 1, new int[]{5});
        test(sol, new int[]{1, 2, 3}, 1, 3, new int[]{3, 2, 1});
        System.out.println("All tests passed.");
    }

    private static void test(P61_ReverseLinkedListII sol, int[] vals, int left, int right, int[] expected) {
        ListNode result = sol.reverseBetween(build(vals), left, right);
        int[] actual = toArray(result);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(vals) + " left=" + left + " right=" + right + " -> " + java.util.Arrays.toString(actual));
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
