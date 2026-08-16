/**
 * LeetCode Top Interview 150 -- #111. Merge k Sorted Lists (Hard)
 *
 * Given an array of k sorted linked lists, merge them into one sorted
 * linked list.
 *
 * Example:
 *   Input: lists = [[1,4,5],[1,3,4],[2,6]]
 *   Output: [1,1,2,3,4,4,5,6]
 */
public class P111_MergeKSortedLists {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public ListNode mergeKLists(ListNode[] lists) {
        if (lists.length == 0) return null;
        return mergeRange(lists, 0, lists.length - 1);
    }

    private ListNode mergeRange(ListNode[] lists, int left, int right) {
        if (left == right) return lists[left];
        int mid = left + (right - left) / 2;
        ListNode l1 = mergeRange(lists, left, mid);
        ListNode l2 = mergeRange(lists, mid + 1, right);
        return merge(l1, l2);
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
        P111_MergeKSortedLists sol = new P111_MergeKSortedLists();
        test(sol, new int[][]{{1, 4, 5}, {1, 3, 4}, {2, 6}}, new int[]{1, 1, 2, 3, 4, 4, 5, 6});
        test(sol, new int[][]{}, new int[]{});
        test(sol, new int[][]{{}}, new int[]{});
        System.out.println("All tests passed.");
    }

    private static void test(P111_MergeKSortedLists sol, int[][] listsVals, int[] expected) {
        ListNode[] lists = new ListNode[listsVals.length];
        for (int i = 0; i < listsVals.length; i++) lists[i] = build(listsVals[i]);

        ListNode result = sol.mergeKLists(lists);
        int[] actual = toArray(result);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.deepToString(listsVals) + " -> " + java.util.Arrays.toString(actual));
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
