/**
 * LeetCode Top Interview 150 -- #97. Word Ladder (Hard)
 *
 * Given beginWord, endWord, and a wordList, return the length of the
 * shortest transformation sequence changing one letter at a time, each
 * intermediate word in wordList, or 0 if none exists.
 *
 * Example:
 *   Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
 *   Output: 5   (hit -> hot -> dot -> dog -> cog)
 */
public class P97_WordLadder {

    public int ladderLength(String beginWord, String endWord, java.util.List<String> wordList) {
        java.util.Set<String> wordSet = new java.util.HashSet<>(wordList);
        if (!wordSet.contains(endWord)) return 0;

        java.util.Queue<String> queue = new java.util.LinkedList<>();
        queue.add(beginWord);
        java.util.Set<String> visited = new java.util.HashSet<>();
        visited.add(beginWord);

        int length = 1;
        while (!queue.isEmpty()) {
            int size = queue.size();
            for (int i = 0; i < size; i++) {
                String cur = queue.poll();
                if (cur.equals(endWord)) return length;

                char[] chars = cur.toCharArray();
                for (int j = 0; j < chars.length; j++) {
                    char orig = chars[j];
                    for (char c = 'a'; c <= 'z'; c++) {
                        if (c == orig) continue;
                        chars[j] = c;
                        String next = new String(chars);
                        if (wordSet.contains(next) && !visited.contains(next)) {
                            visited.add(next);
                            queue.add(next);
                        }
                    }
                    chars[j] = orig;
                }
            }
            length++;
        }
        return 0;
    }

    public static void main(String[] args) {
        P97_WordLadder sol = new P97_WordLadder();
        test(sol, "hit", "cog", java.util.List.of("hot", "dot", "dog", "lot", "log", "cog"), 5);
        test(sol, "hit", "cog", java.util.List.of("hot", "dot", "dog", "lot", "log"), 0);
        test(sol, "a", "c", java.util.List.of("a", "b", "c"), 2);
        System.out.println("All tests passed.");
    }

    private static void test(P97_WordLadder sol, String beginWord, String endWord, java.util.List<String> wordList, int expected) {
        int actual = sol.ladderLength(beginWord, endWord, wordList);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + beginWord + " -> " + endWord + " = " + actual);
    }
}
