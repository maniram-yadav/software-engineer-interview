/**
 * LeetCode Top Interview 150 -- #62. Reverse Nodes in k-Group (Hard)
 *
 * Given a linked list, reverse the nodes of the list k at a time and
 * return the modified list. If the remaining nodes are fewer than k, leave
 * them as is.
 *
 * Example:
 *   Input: head = [1,2,3,4,5], k = 2
 *   Output: [2,1,4,3,5]
 */
public class P62_ReverseNodesInKGroup {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public ListNode reverseKGroup(ListNode head, int k) {
        ListNode node = head;
        int count = 0;
        while (node != null && count < k) {
            node = node.next;
            count++;
        }
        if (count < k) return head;

        ListNode prev = reverseKGroup(node, k);
        ListNode cur = head;
        for (int i = 0; i < k; i++) {
            ListNode next = cur.next;
            cur.next = prev;
            prev = cur;
            cur = next;
        }
        return prev;
    }

    public static void main(String[] args) {
        P62_ReverseNodesInKGroup sol = new P62_ReverseNodesInKGroup();
        test(sol, new int[]{1, 2, 3, 4, 5}, 2, new int[]{2, 1, 4, 3, 5});
        test(sol, new int[]{1, 2, 3, 4, 5}, 3, new int[]{3, 2, 1, 4, 5});
        test(sol, new int[]{1, 2}, 1, new int[]{1, 2});
        System.out.println("All tests passed.");
    }

    private static void test(P62_ReverseNodesInKGroup sol, int[] vals, int k, int[] expected) {
        ListNode result = sol.reverseKGroup(build(vals), k);
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
