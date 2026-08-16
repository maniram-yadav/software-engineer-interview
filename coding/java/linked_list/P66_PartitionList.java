/**
 * LeetCode Top Interview 150 -- #66. Partition List (Medium)
 *
 * Given the head of a linked list and a value x, partition it so all nodes
 * less than x come before nodes >= x, preserving the relative order within
 * each partition.
 *
 * Example:
 *   Input: head = [1,4,3,2,5,2], x = 3
 *   Output: [1,2,2,4,3,5]
 */
public class P66_PartitionList {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public ListNode partition(ListNode head, int x) {
        ListNode lessDummy = new ListNode(0);
        ListNode greaterDummy = new ListNode(0);
        ListNode less = lessDummy, greater = greaterDummy;

        while (head != null) {
            if (head.val < x) {
                less.next = head;
                less = less.next;
            } else {
                greater.next = head;
                greater = greater.next;
            }
            head = head.next;
        }
        greater.next = null;
        less.next = greaterDummy.next;
        return lessDummy.next;
    }

    public static void main(String[] args) {
        P66_PartitionList sol = new P66_PartitionList();
        test(sol, new int[]{1, 4, 3, 2, 5, 2}, 3, new int[]{1, 2, 2, 4, 3, 5});
        test(sol, new int[]{2, 1}, 2, new int[]{1, 2});
        test(sol, new int[]{1, 2, 3}, 0, new int[]{1, 2, 3});
        System.out.println("All tests passed.");
    }

    private static void test(P66_PartitionList sol, int[] vals, int x, int[] expected) {
        ListNode result = sol.partition(build(vals), x);
        int[] actual = toArray(result);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(vals) + " x=" + x + " -> " + java.util.Arrays.toString(actual));
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
