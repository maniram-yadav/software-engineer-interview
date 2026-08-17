/**
 * Grind 169 -- #588. Design In-Memory File System (Hard)
 *
 * Design an in-memory file system supporting ls, mkdir, addContentToFile,
 * and readContentFromFile, mimicking Unix-style paths.
 *
 * Example:
 *   fs.mkdir("/a/b/c");
 *   fs.addContentToFile("/a/b/c/d","hello");
 *   fs.ls("/");        // ["a"]
 *   fs.readContentFromFile("/a/b/c/d"); // "hello"
 */
public class P588_DesignInMemoryFileSystem {

    static class FileSystem {
        static class Node {
            boolean isFile = false;
            String content = "";
            java.util.Map<String, Node> children = new java.util.TreeMap<>();
        }

        private final Node root = new Node();

        public java.util.List<String> ls(String path) {
            Node node = traverse(path);
            if (node.isFile) {
                String[] parts = path.split("/");
                return java.util.List.of(parts[parts.length - 1]);
            }
            return new java.util.ArrayList<>(node.children.keySet());
        }

        public void mkdir(String path) {
            getOrCreate(path, false);
        }

        public void addContentToFile(String filePath, String content) {
            Node node = getOrCreate(filePath, true);
            node.content += content;
        }

        public String readContentFromFile(String filePath) {
            return traverse(filePath).content;
        }

        private Node traverse(String path) {
            Node cur = root;
            for (String part : path.split("/")) {
                if (part.isEmpty()) continue;
                cur = cur.children.get(part);
            }
            return cur;
        }

        private Node getOrCreate(String path, boolean isFile) {
            Node cur = root;
            String[] parts = path.split("/");
            for (int i = 0; i < parts.length; i++) {
                String part = parts[i];
                if (part.isEmpty()) continue;
                boolean last = (i == parts.length - 1);
                cur = cur.children.computeIfAbsent(part, k -> new Node());
                if (last && isFile) cur.isFile = true;
            }
            return cur;
        }
    }

    public static void main(String[] args) {
        FileSystem fs = new FileSystem();
        fs.mkdir("/a/b/c");
        fs.addContentToFile("/a/b/c/d", "hello");
        checkList(fs.ls("/"), java.util.List.of("a"), "ls(/)");
        check(fs.readContentFromFile("/a/b/c/d"), "hello", "readContentFromFile");
        checkList(fs.ls("/a/b/c"), java.util.List.of("d"), "ls(/a/b/c)");
        fs.addContentToFile("/a/b/c/d", " world");
        check(fs.readContentFromFile("/a/b/c/d"), "hello world", "readContentFromFile after append");
        System.out.println("All tests passed.");
    }

    private static void check(String actual, String expected, String label) {
        if (!actual.equals(expected)) {
            throw new AssertionError(label + ": expected \"" + expected + "\" but got \"" + actual + "\"");
        }
        System.out.println("PASS: " + label + " -> \"" + actual + "\"");
    }

    private static void checkList(java.util.List<String> actual, java.util.List<String> expected, String label) {
        if (!actual.equals(expected)) {
            throw new AssertionError(label + ": expected " + expected + " but got " + actual);
        }
        System.out.println("PASS: " + label + " -> " + actual);
    }
}
