/**
 * LeetCode Top Interview 150 -- #64. Remove Duplicates from Sorted List II (Medium)
 *
 * Given the head of a sorted linked list, delete all nodes that have
 * duplicate numbers, leaving only distinct numbers from the original list.
 *
 * Example:
 *   Input: head = [1,2,3,3,4,4,5]
 *   Output: [1,2,5]
 */
public class P64_RemoveDuplicatesFromSortedListII {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public ListNode deleteDuplicates(ListNode head) {
        ListNode dummy = new ListNode(0);
        dummy.next = head;
        ListNode prev = dummy, cur = head;

        while (cur != null) {
            if (cur.next != null && cur.val == cur.next.val) {
                int val = cur.val;
                while (cur != null && cur.val == val) cur = cur.next;
                prev.next = cur;
            } else {
                prev = cur;
                cur = cur.next;
            }
        }
        return dummy.next;
    }

    public static void main(String[] args) {
        P64_RemoveDuplicatesFromSortedListII sol = new P64_RemoveDuplicatesFromSortedListII();
        test(sol, new int[]{1, 2, 3, 3, 4, 4, 5}, new int[]{1, 2, 5});
        test(sol, new int[]{1, 1, 1, 2, 3}, new int[]{2, 3});
        test(sol, new int[]{1, 2, 3}, new int[]{1, 2, 3});
        System.out.println("All tests passed.");
    }

    private static void test(P64_RemoveDuplicatesFromSortedListII sol, int[] vals, int[] expected) {
        ListNode result = sol.deleteDuplicates(build(vals));
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
