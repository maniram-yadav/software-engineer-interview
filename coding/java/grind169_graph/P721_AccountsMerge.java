/**
 * Grind 169 -- #721. Accounts Merge (Medium)
 *
 * Given a list of accounts, each with a name and a list of emails, merge
 * accounts that share at least one email (same person), returning merged
 * accounts with sorted emails.
 *
 * Example:
 *   Input: accounts = [["John","johnsmith@mail.com","john_newyork@mail.com"],["John","johnsmith@mail.com","john00@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
 *   Output: [["John","john00@mail.com","john_newyork@mail.com","johnsmith@mail.com"],["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]
 */
public class P721_AccountsMerge {

    public java.util.List<java.util.List<String>> accountsMerge(java.util.List<java.util.List<String>> accounts) {
        java.util.Map<String, String> parent = new java.util.HashMap<>();
        java.util.Map<String, String> emailToName = new java.util.HashMap<>();

        for (java.util.List<String> account : accounts) {
            String name = account.get(0);
            for (int i = 1; i < account.size(); i++) {
                String email = account.get(i);
                parent.putIfAbsent(email, email);
                emailToName.put(email, name);
                union(parent, account.get(1), email);
            }
        }

        java.util.Map<String, java.util.List<String>> groups = new java.util.TreeMap<>();
        for (String email : parent.keySet()) {
            String root = find(parent, email);
            groups.computeIfAbsent(root, k -> new java.util.ArrayList<>()).add(email);
        }

        java.util.List<java.util.List<String>> result = new java.util.ArrayList<>();
        for (java.util.Map.Entry<String, java.util.List<String>> entry : groups.entrySet()) {
            java.util.List<String> emails = entry.getValue();
            java.util.Collections.sort(emails);
            java.util.List<String> merged = new java.util.ArrayList<>();
            merged.add(emailToName.get(entry.getKey()));
            merged.addAll(emails);
            result.add(merged);
        }
        return result;
    }

    private String find(java.util.Map<String, String> parent, String x) {
        while (!parent.get(x).equals(x)) {
            parent.put(x, parent.get(parent.get(x)));
            x = parent.get(x);
        }
        return x;
    }

    private void union(java.util.Map<String, String> parent, String a, String b) {
        String rootA = find(parent, a), rootB = find(parent, b);
        if (!rootA.equals(rootB)) parent.put(rootA, rootB);
    }

    public static void main(String[] args) {
        P721_AccountsMerge sol = new P721_AccountsMerge();

        java.util.List<java.util.List<String>> accounts = java.util.List.of(
                java.util.List.of("John", "johnsmith@mail.com", "john_newyork@mail.com"),
                java.util.List.of("John", "johnsmith@mail.com", "john00@mail.com"),
                java.util.List.of("Mary", "mary@mail.com"),
                java.util.List.of("John", "johnnybravo@mail.com"));

        java.util.List<String> expected1 = java.util.List.of("John", "john00@mail.com", "john_newyork@mail.com", "johnsmith@mail.com");
        java.util.List<String> expected2 = java.util.List.of("Mary", "mary@mail.com");
        java.util.List<String> expected3 = java.util.List.of("John", "johnnybravo@mail.com");

        java.util.List<java.util.List<String>> actual = sol.accountsMerge(accounts);
        java.util.Set<java.util.List<String>> actualSet = new java.util.HashSet<>(actual);
        java.util.Set<java.util.List<String>> expectedSet = java.util.Set.of(expected1, expected2, expected3);
        if (!actualSet.equals(expectedSet)) {
            throw new AssertionError("Expected " + expectedSet + " but got " + actualSet);
        }
        System.out.println("PASS: " + actual);
        System.out.println("All tests passed.");
    }
}
