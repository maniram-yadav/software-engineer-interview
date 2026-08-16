/**
 * LeetCode Top Interview 150 -- #65. Rotate List (Medium)
 *
 * Given the head of a linked list, rotate the list to the right by k
 * places.
 *
 * Example:
 *   Input: head = [1,2,3,4,5], k = 2
 *   Output: [4,5,1,2,3]
 */
public class P65_RotateList {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public ListNode rotateRight(ListNode head, int k) {
        if (head == null || head.next == null) return head;

        int length = 1;
        ListNode tail = head;
        while (tail.next != null) {
            tail = tail.next;
            length++;
        }
        k %= length;
        if (k == 0) return head;

        tail.next = head;
        int stepsToNewTail = length - k;
        ListNode newTail = head;
        for (int i = 1; i < stepsToNewTail; i++) newTail = newTail.next;

        ListNode newHead = newTail.next;
        newTail.next = null;
        return newHead;
    }

    public static void main(String[] args) {
        P65_RotateList sol = new P65_RotateList();
        test(sol, new int[]{1, 2, 3, 4, 5}, 2, new int[]{4, 5, 1, 2, 3});
        test(sol, new int[]{0, 1, 2}, 4, new int[]{2, 0, 1});
        test(sol, new int[]{1}, 5, new int[]{1});
        System.out.println("All tests passed.");
    }

    private static void test(P65_RotateList sol, int[] vals, int k, int[] expected) {
        ListNode result = sol.rotateRight(build(vals), k);
        int[] actual = toArray(result);
        if (!java.util.Arrays.equals(actual, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + java.util.Arrays.toString(actual));
        }
        System.out.println("PASS: " + java.util.Arrays.toString(vals) + " k=" + k + " -> " + java.util.Arrays.toString(actual));
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
