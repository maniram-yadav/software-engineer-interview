/**
 * LeetCode Top Interview 150 -- #96. Minimum Genetic Mutation (Medium)
 *
 * A gene string has 8 characters from {A,C,G,T}. Given startGene, endGene,
 * and a bank of valid intermediate genes (one character differs per
 * mutation), return the minimum number of mutations to get from start to
 * end using only bank genes, or -1.
 *
 * Example:
 *   Input: startGene = "AACCGGTT", endGene = "AACCGGTA", bank = ["AACCGGTA"]
 *   Output: 1
 */
public class P96_MinimumGeneticMutation {

    private static final char[] GENES = {'A', 'C', 'G', 'T'};

    public int minMutation(String startGene, String endGene, String[] bank) {
        java.util.Set<String> bankSet = new java.util.HashSet<>(java.util.Arrays.asList(bank));
        if (!bankSet.contains(endGene)) return -1;

        java.util.Queue<String> queue = new java.util.LinkedList<>();
        queue.add(startGene);
        java.util.Set<String> visited = new java.util.HashSet<>();
        visited.add(startGene);

        int mutations = 0;
        while (!queue.isEmpty()) {
            int size = queue.size();
            for (int i = 0; i < size; i++) {
                String cur = queue.poll();
                if (cur.equals(endGene)) return mutations;

                char[] chars = cur.toCharArray();
                for (int j = 0; j < chars.length; j++) {
                    char orig = chars[j];
                    for (char g : GENES) {
                        if (g == orig) continue;
                        chars[j] = g;
                        String next = new String(chars);
                        if (bankSet.contains(next) && !visited.contains(next)) {
                            visited.add(next);
                            queue.add(next);
                        }
                    }
                    chars[j] = orig;
                }
            }
            mutations++;
        }
        return -1;
    }

    public static void main(String[] args) {
        P96_MinimumGeneticMutation sol = new P96_MinimumGeneticMutation();
        test(sol, "AACCGGTT", "AACCGGTA", new String[]{"AACCGGTA"}, 1);
        test(sol, "AACCGGTT", "AAACGGTA", new String[]{"AACCGGTA", "AACCGCTA", "AAACGGTA"}, 2);
        test(sol, "AAAAACCC", "AACCCCCC", new String[]{"AAAACCCC", "AAACCCCC", "AACCCCCC"}, 3);
        System.out.println("All tests passed.");
    }

    private static void test(P96_MinimumGeneticMutation sol, String start, String end, String[] bank, int expected) {
        int actual = sol.minMutation(start, end, bank);
        if (actual != expected) {
            throw new AssertionError("Expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + start + " -> " + end + " = " + actual);
    }
}
