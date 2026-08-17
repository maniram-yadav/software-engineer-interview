/**
 * Grind 169 -- #876. Middle of the Linked List (Easy)
 *
 * Given the head of a singly linked list, return the middle node (if two
 * middle nodes, return the second one).
 *
 * Example:
 *   Input: head = [1,2,3,4,5]
 *   Output: [3,4,5]
 */
public class P876_MiddleOfTheLinkedList {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public ListNode middleNode(ListNode head) {
        ListNode slow = head, fast = head;
        while (fast != null && fast.next != null) {
            slow = slow.next;
            fast = fast.next.next;
        }
        return slow;
    }

    public static void main(String[] args) {
        P876_MiddleOfTheLinkedList sol = new P876_MiddleOfTheLinkedList();
        test(sol, new int[]{1, 2, 3, 4, 5}, new int[]{3, 4, 5});
        test(sol, new int[]{1, 2, 3, 4, 5, 6}, new int[]{4, 5, 6});
        test(sol, new int[]{1}, new int[]{1});
        System.out.println("All tests passed.");
    }

    private static void test(P876_MiddleOfTheLinkedList sol, int[] vals, int[] expected) {
        ListNode result = sol.middleNode(build(vals));
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
