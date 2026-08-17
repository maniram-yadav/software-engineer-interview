/**
 * Grind 169 -- #234. Palindrome Linked List (Easy)
 *
 * Given the head of a singly linked list, return true if it reads the
 * same forward and backward.
 *
 * Example:
 *   Input: head = [1,2,2,1]
 *   Output: true
 */
public class P234_PalindromeLinkedList {

    static class ListNode {
        int val;
        ListNode next;

        ListNode(int val) {
            this.val = val;
        }
    }

    public boolean isPalindrome(ListNode head) {
        java.util.List<Integer> vals = new java.util.ArrayList<>();
        while (head != null) {
            vals.add(head.val);
            head = head.next;
        }
        int left = 0, right = vals.size() - 1;
        while (left < right) {
            if (!vals.get(left).equals(vals.get(right))) return false;
            left++;
            right--;
        }
        return true;
    }

    public static void main(String[] args) {
        P234_PalindromeLinkedList sol = new P234_PalindromeLinkedList();
        test(sol, new int[]{1, 2, 2, 1}, true);
        test(sol, new int[]{1, 2}, false);
        test(sol, new int[]{1}, true);
        System.out.println("All tests passed.");
    }

    private static void test(P234_PalindromeLinkedList sol, int[] vals, boolean expected) {
        boolean actual = sol.isPalindrome(build(vals));
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + java.util.Arrays.toString(vals) + " -> " + actual);
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
}
