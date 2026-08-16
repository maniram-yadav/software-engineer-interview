/**
 * LeetCode Top Interview 150 -- #58. Add Two Numbers (Medium)
 *
 * Two non-empty linked lists represent non-negative integers in reverse
 * digit order. Add the two numbers and return the sum as a linked list in
 * the same format.
 *
 * Example:
 *   Input: l1 = [2,4,3], l2 = [5,6,4]
 *   Output: [7,0,8]   (342 + 465 = 807)
 */
public class P58_AddTwoNumbers {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        ListNode dummy = new ListNode(0);
        ListNode cur = dummy;
        int carry = 0;

        while (l1 != null || l2 != null || carry != 0) {
            int sum = carry;
            if (l1 != null) {
                sum += l1.val;
                l1 = l1.next;
            }
            if (l2 != null) {
                sum += l2.val;
                l2 = l2.next;
            }
            carry = sum / 10;
            cur.next = new ListNode(sum % 10);
            cur = cur.next;
        }
        return dummy.next;
    }

    public static void main(String[] args) {
        P58_AddTwoNumbers sol = new P58_AddTwoNumbers();
        test(sol, new int[]{2, 4, 3}, new int[]{5, 6, 4}, new int[]{7, 0, 8});
        test(sol, new int[]{0}, new int[]{0}, new int[]{0});
        test(sol, new int[]{9, 9, 9, 9, 9, 9, 9}, new int[]{9, 9, 9, 9}, new int[]{8, 9, 9, 9, 0, 0, 0, 1});
        System.out.println("All tests passed.");
    }

    private static void test(P58_AddTwoNumbers sol, int[] l1Vals, int[] l2Vals, int[] expected) {
        ListNode result = sol.addTwoNumbers(build(l1Vals), build(l2Vals));
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
