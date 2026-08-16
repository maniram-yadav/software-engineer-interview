/**
 * LeetCode Top Interview 150 -- #109. Sort List (Medium)
 *
 * Given the head of a linked list, sort it in ascending order and return
 * it, in O(n log n) time.
 *
 * Example:
 *   Input: head = [4,2,1,3]
 *   Output: [1,2,3,4]
 */
public class P109_SortList {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public ListNode sortList(ListNode head) {
        if (head == null || head.next == null) return head;

        ListNode slow = head, fast = head, prev = null;
        while (fast != null && fast.next != null) {
            prev = slow;
            slow = slow.next;
            fast = fast.next.next;
        }
        prev.next = null;

        ListNode left = sortList(head);
        ListNode right = sortList(slow);
        return merge(left, right);
    }

    private ListNode merge(ListNode l1, ListNode l2) {
        ListNode dummy = new ListNode(0);
        ListNode cur = dummy;
        while (l1 != null && l2 != null) {
            if (l1.val <= l2.val) {
                cur.next = l1;
                l1 = l1.next;
            } else {
                cur.next = l2;
                l2 = l2.next;
            }
            cur = cur.next;
        }
        cur.next = (l1 != null) ? l1 : l2;
        return dummy.next;
    }

    public static void main(String[] args) {
        P109_SortList sol = new P109_SortList();
        test(sol, new int[]{4, 2, 1, 3}, new int[]{1, 2, 3, 4});
        test(sol, new int[]{}, new int[]{});
        test(sol, new int[]{1}, new int[]{1});
        test(sol, new int[]{-1, 5, 3, 4, 0}, new int[]{-1, 0, 3, 4, 5});
        System.out.println("All tests passed.");
    }

    private static void test(P109_SortList sol, int[] vals, int[] expected) {
        ListNode result = sol.sortList(build(vals));
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
