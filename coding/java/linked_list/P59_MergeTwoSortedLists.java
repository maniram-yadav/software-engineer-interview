/**
 * LeetCode Top Interview 150 -- #59. Merge Two Sorted Lists (Easy)
 *
 * Merge two sorted linked lists into one sorted list by splicing their
 * nodes.
 *
 * Example:
 *   Input: list1 = [1,2,4], list2 = [1,3,4]
 *   Output: [1,1,2,3,4,4]
 */
public class P59_MergeTwoSortedLists {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public ListNode mergeTwoLists(ListNode list1, ListNode list2) {
        ListNode dummy = new ListNode(0);
        ListNode cur = dummy;

        while (list1 != null && list2 != null) {
            if (list1.val <= list2.val) {
                cur.next = list1;
                list1 = list1.next;
            } else {
                cur.next = list2;
                list2 = list2.next;
            }
            cur = cur.next;
        }
        cur.next = (list1 != null) ? list1 : list2;
        return dummy.next;
    }

    public static void main(String[] args) {
        P59_MergeTwoSortedLists sol = new P59_MergeTwoSortedLists();
        test(sol, new int[]{1, 2, 4}, new int[]{1, 3, 4}, new int[]{1, 1, 2, 3, 4, 4});
        test(sol, new int[]{}, new int[]{}, new int[]{});
        test(sol, new int[]{}, new int[]{0}, new int[]{0});
        System.out.println("All tests passed.");
    }

    private static void test(P59_MergeTwoSortedLists sol, int[] l1Vals, int[] l2Vals, int[] expected) {
        ListNode result = sol.mergeTwoLists(build(l1Vals), build(l2Vals));
        int[] actual = toArray(result);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(l1Vals) + " + " + java.util.Arrays.toString(l2Vals) + " -> " + java.util.Arrays.toString(actual));
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
