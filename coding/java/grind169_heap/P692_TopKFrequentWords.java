/**
 * Grind 169 -- #692. Top K Frequent Words (Medium)
 *
 * Given an array of strings words and an integer k, return the k most
 * frequent words, sorted by frequency (descending) then lexicographically
 * for ties.
 *
 * Example:
 *   Input: words = ["i","love","leetcode","i","love","coding"], k = 2
 *   Output: ["i","love"]
 */
public class P692_TopKFrequentWords {

    public java.util.List<String> topKFrequent(String[] words, int k) {
        java.util.Map<String, Integer> count = new java.util.HashMap<>();
        for (String w : words) count.merge(w, 1, Integer::sum);

        java.util.PriorityQueue<String> heap = new java.util.PriorityQueue<>(
                (a, b) -> count.get(a).equals(count.get(b)) ? b.compareTo(a) : count.get(a) - count.get(b));
        for (String w : count.keySet()) {
            heap.add(w);
            if (heap.size() > k) heap.poll();
        }

        java.util.List<String> result = new java.util.ArrayList<>();
        while (!heap.isEmpty()) result.add(heap.poll());
        java.util.Collections.reverse(result);
        return result;
    }

    public static void main(String[] args) {
        P692_TopKFrequentWords sol = new P692_TopKFrequentWords();
        test(sol, new String[]{"i", "love", "leetcode", "i", "love", "coding"}, 2, new String[]{"i", "love"});
        test(sol, new String[]{"the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"}, 4,
                new String[]{"the", "is", "sunny", "day"});
        System.out.println("All tests passed.");
    }

    private static void test(P692_TopKFrequentWords sol, String[] words, int k, String[] expected) {
        java.util.List<String> actual = sol.topKFrequent(words, k);
        java.util.List<String> expectedList = java.util.Arrays.asList(expected);
        if (!actual.equals(expectedList)) {
            throw new AssertionError("Expected " + expectedList + " but got " + actual);
        }
        System.out.println("PASS: k=" + k + " -> " + actual);
    }
}
