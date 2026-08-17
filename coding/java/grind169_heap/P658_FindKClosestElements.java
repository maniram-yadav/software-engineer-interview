/**
 * Grind 169 -- #658. Find K Closest Elements (Medium)
 *
 * Given a sorted integer array arr, and integers k and x, return the k
 * closest integers to x in the array, sorted ascending.
 *
 * Example:
 *   Input: arr = [1,2,3,4,5], k = 4, x = 3
 *   Output: [1,2,3,4]
 */
public class P658_FindKClosestElements {

    public java.util.List<Integer> findClosestElements(int[] arr, int k, int x) {
        int left = 0, right = arr.length - k;
        while (left < right) {
            int mid = left + (right - left) / 2;
            if (x - arr[mid] > arr[mid + k] - x) left = mid + 1;
            else right = mid;
        }
        java.util.List<Integer> result = new java.util.ArrayList<>();
        for (int i = left; i < left + k; i++) result.add(arr[i]);
        return result;
    }

    public static void main(String[] args) {
        P658_FindKClosestElements sol = new P658_FindKClosestElements();
        test(sol, new int[]{1, 2, 3, 4, 5}, 4, 3, new int[]{1, 2, 3, 4});
        test(sol, new int[]{1, 2, 3, 4, 5}, 4, -1, new int[]{1, 2, 3, 4});
        System.out.println("All tests passed.");
    }

    private static void test(P658_FindKClosestElements sol, int[] arr, int k, int x, int[] expected) {
        java.util.List<Integer> actual = sol.findClosestElements(arr, k, x);
        int[] actualArr = actual.stream().mapToInt(Integer::intValue).toArray();
        if (!java.util.Arrays.equals(actualArr, expected)) {
            throw new AssertionError("Expected " + java.util.Arrays.toString(expected) + " but got " + actual);
        }
        System.out.println("PASS: k=" + k + " x=" + x + " -> " + actual);
    }
}
