/**
 * LeetCode Top Interview 150 -- #110. Construct Quad Tree (Medium)
 *
 * Given an n x n binary grid, build a Quad-Tree representation,
 * recursively splitting into 4 quadrants until each region is uniform.
 *
 * Example:
 *   Input: grid = [[0,1],[1,0]]
 *   Output: quad-tree with 4 leaf nodes for each cell
 */
public class P110_ConstructQuadTree {

    static class Node {
        boolean val;
        boolean isLeaf;
        Node topLeft, topRight, bottomLeft, bottomRight;

        Node(boolean val, boolean isLeaf) {
            this.val = val;
            this.isLeaf = isLeaf;
        }
    }

    public Node construct(int[][] grid) {
        return build(grid, 0, 0, grid.length);
    }

    private Node build(int[][] grid, int row, int col, int size) {
        if (size == 1) return new Node(grid[row][col] == 1, true);

        int half = size / 2;
        Node tl = build(grid, row, col, half);
        Node tr = build(grid, row, col + half, half);
        Node bl = build(grid, row + half, col, half);
        Node br = build(grid, row + half, col + half, half);

        if (tl.isLeaf && tr.isLeaf && bl.isLeaf && br.isLeaf
                && tl.val == tr.val && tr.val == bl.val && bl.val == br.val) {
            return new Node(tl.val, true);
        }

        Node node = new Node(true, false);
        node.topLeft = tl;
        node.topRight = tr;
        node.bottomLeft = bl;
        node.bottomRight = br;
        return node;
    }

    public static void main(String[] args) {
        P110_ConstructQuadTree sol = new P110_ConstructQuadTree();

        Node result1 = sol.construct(new int[][]{{0, 1}, {1, 0}});
        checkFalse(result1.isLeaf, "root is internal for mixed grid");
        checkLeaf(result1.topLeft, false, "topLeft");
        checkLeaf(result1.topRight, true, "topRight");
        checkLeaf(result1.bottomLeft, true, "bottomLeft");
        checkLeaf(result1.bottomRight, false, "bottomRight");
        System.out.println("PASS: mixed 2x2 grid -> internal node with 4 leaves");

        Node result2 = sol.construct(new int[][]{{1, 1}, {1, 1}});
        checkLeaf(result2, true, "uniform grid root");
        System.out.println("PASS: uniform grid -> single leaf true");

        Node result3 = sol.construct(new int[][]{{0}});
        checkLeaf(result3, false, "single cell root");
        System.out.println("PASS: single cell -> single leaf false");

        System.out.println("All tests passed.");
    }

    private static void checkLeaf(Node node, boolean expectedVal, String label) {
        if (!node.isLeaf || node.val != expectedVal) {
            throw new AssertionError(label + ": expected leaf with val=" + expectedVal + " but got isLeaf=" + node.isLeaf + " val=" + node.val);
        }
    }

    private static void checkFalse(boolean actual, String label) {
        if (actual) {
            throw new AssertionError(label + ": expected false but got true");
        }
    }
}
