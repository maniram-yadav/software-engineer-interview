/**
 * Grind 169 -- #271. Encode and Decode Strings (Medium, LeetCode Premium)
 *
 * Design an algorithm to encode a list of strings into one string, and
 * decode it back into the original list of strings.
 *
 * Example:
 *   Input: ["lint","code","love","you"]
 *   Output (encoded then decoded): ["lint","code","love","you"]
 */
public class P271_EncodeAndDecodeStrings {

    public String encode(java.util.List<String> strs) {
        StringBuilder sb = new StringBuilder();
        for (String s : strs) {
            sb.append(s.length()).append('#').append(s);
        }
        return sb.toString();
    }

    public java.util.List<String> decode(String s) {
        java.util.List<String> result = new java.util.ArrayList<>();
        int i = 0;
        while (i < s.length()) {
            int j = i;
            while (s.charAt(j) != '#') j++;
            int len = Integer.parseInt(s.substring(i, j));
            result.add(s.substring(j + 1, j + 1 + len));
            i = j + 1 + len;
        }
        return result;
    }

    public static void main(String[] args) {
        P271_EncodeAndDecodeStrings sol = new P271_EncodeAndDecodeStrings();
        test(sol, java.util.List.of("lint", "code", "love", "you"));
        test(sol, java.util.List.of());
        test(sol, java.util.List.of(""));
        test(sol, java.util.List.of("a#b", "10#c"));
        System.out.println("All tests passed.");
    }

    private static void test(P271_EncodeAndDecodeStrings sol, java.util.List<String> strs) {
        String encoded = sol.encode(strs);
        java.util.List<String> decoded = sol.decode(encoded);
        if (!decoded.equals(strs)) {
            throw new AssertionError("Expected " + strs + " but got " + decoded);
        }
        System.out.println("PASS: " + strs + " -> \"" + encoded + "\" -> " + decoded);
    }
}
