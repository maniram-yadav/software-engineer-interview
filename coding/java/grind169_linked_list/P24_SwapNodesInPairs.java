/**
 * Grind 169 -- #24. Swap Nodes in Pairs (Medium)
 *
 * Given a linked list, swap every two adjacent nodes and return its head
 * (swap the nodes themselves, not just values).
 *
 * Example:
 *   Input: head = [1,2,3,4]
 *   Output: [2,1,4,3]
 */
public class P24_SwapNodesInPairs {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public ListNode swapPairs(ListNode head) {
        ListNode dummy = new ListNode(0);
        dummy.next = head;
        ListNode prev = dummy;
        while (prev.next != null && prev.next.next != null) {
            ListNode first = prev.next, second = first.next;
            first.next = second.next;
            second.next = first;
            prev.next = second;
            prev = first;
        }
        return dummy.next;
    }

    public static void main(String[] args) {
        P24_SwapNodesInPairs sol = new P24_SwapNodesInPairs();
        test(sol, new int[]{1, 2, 3, 4}, new int[]{2, 1, 4, 3});
        test(sol, new int[]{}, new int[]{});
        test(sol, new int[]{1}, new int[]{1});
        System.out.println("All tests passed.");
    }

    private static void test(P24_SwapNodesInPairs sol, int[] vals, int[] expected) {
        ListNode result = sol.swapPairs(build(vals));
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
