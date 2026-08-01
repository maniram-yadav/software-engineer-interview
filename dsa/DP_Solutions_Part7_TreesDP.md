# DP Solutions — Part 7: DP on Trees (Java)
### 8 Problems · Full Problem Statement + Example + Brute Force → Optimized + Complexity

> Shared helper class used throughout:
> ```java
> class TreeNode {
>     int val;
>     TreeNode left, right;
>     TreeNode(int val) { this.val = val; }
> }
> ```

---

## 1. Unique Binary Search Trees II

**Problem:** Given an integer `n`, return ALL the structurally unique BSTs that store values `1` to `n` (not just the count — the actual tree structures).

**Example:**
```
Input: n = 3
Output: [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
Explanation: These are the 5 structurally distinct BSTs using values {1,2,3}.
```

**Brute force:** this problem inherently requires generating every structure — there's no shortcut avoiding construction, but the naive way (without splitting into independent left/right subproblems) would rebuild overlapping subtree shapes repeatedly.
**Optimized:** recursive divide — for each value `i` as root, recursively generate all valid left subtrees (from range [lo,i-1]) and right subtrees (from range [i+1,hi]), then combine every left/right pair.
```java
class UniqueBSTII {
    public List<TreeNode> generateTrees(int n) {
        if (n == 0) return new ArrayList<>();
        return build(1, n);
    }

    private List<TreeNode> build(int lo, int hi) {
        List<TreeNode> result = new ArrayList<>();
        if (lo > hi) {
            result.add(null);
            return result;
        }
        for (int i = lo; i <= hi; i++) {
            List<TreeNode> lefts = build(lo, i - 1);
            List<TreeNode> rights = build(i + 1, hi);
            for (TreeNode l : lefts) {
                for (TreeNode r : rights) {
                    TreeNode root = new TreeNode(i);
                    root.left = l;
                    root.right = r;
                    result.add(root);
                }
            }
        }
        return result;
    }
}
```
**Complexity:** O(Catalan(n) · n) time and space — inherently exponential since the OUTPUT itself is Catalan(n)-many trees; this is optimal since we must construct every tree.

---

## 2. House Robber III

**Problem:** Houses are arranged in a binary tree; each node has a value. A thief cannot rob two directly-connected houses (parent and child). Maximize the total value robbed.

**Example:**
```
Input: root = [3,2,3,null,3,null,1]
        3
       / \
      2   3
       \   \
        3   1
Output: 7
Explanation: Rob nodes 3, 3, 1 (values at the leaves + root's non-adjacent path) = 7.
Robbing 3+3+1 = 7, versus robbing just 3+2 = 5 for the other option — 7 wins.
```

**Brute force:** try every combination of "rob"/"don't rob" per node, checking the no-adjacent-robbery constraint → O(2ⁿ).
**Optimized:** post-order DFS returning a pair `{maxIfNotRobbed, maxIfRobbed}` per subtree, combining at the parent.
```java
class HouseRobberIII {
    public int rob(TreeNode root) {
        int[] result = dfs(root);
        return Math.max(result[0], result[1]);
    }

    // returns {max if this node is NOT robbed, max if this node IS robbed}
    private int[] dfs(TreeNode node) {
        if (node == null) return new int[]{0, 0};
        int[] left = dfs(node.left);
        int[] right = dfs(node.right);

        int notRob = Math.max(left[0], left[1]) + Math.max(right[0], right[1]);
        int rob = node.val + left[0] + right[0]; // children can't be robbed if this node is
        return new int[]{notRob, rob};
    }
}
```
**Complexity:** O(n) time, O(h) space (recursion depth = tree height).

---

## 3. Maximum Product of Splitted Binary Tree

**Problem:** Given a binary tree, remove exactly one edge to split it into two subtrees, maximizing the product of the sums of the two resulting subtrees. Return the max product mod 10⁹+7.

**Example:**
```
Input: root = [1,2,3,4,5,6]
        1
       / \
      2   3
     / \ / \
    4  5 6  (implicit)
Output: 110
Explanation: Removing the edge between 1 and 2 splits into subtree {2,4,5} (sum=11)
and subtree {1,3,6} (sum=10) → product = 110, the maximum achievable.
```

**Brute force:** try removing every possible edge, recompute both subtree sums from scratch each time → O(n²).
**Optimized:** compute the total tree sum once; then in a single DFS pass, compute each subtree's sum and evaluate `subtreeSum * (totalSum - subtreeSum)` at every node (this evaluates the "cut the edge above this node" option for all n possible edges in one pass).
```java
class MaxProductSplittedBinaryTree {
    private long totalSum = 0;
    private long best = 0;
    private static final int MOD = 1_000_000_007;

    public int maxProduct(TreeNode root) {
        totalSum = computeSum(root);
        subtreeSum(root);
        return (int) (best % MOD);
    }

    private long computeSum(TreeNode node) {
        if (node == null) return 0;
        return node.val + computeSum(node.left) + computeSum(node.right);
    }

    private long subtreeSum(TreeNode node) {
        if (node == null) return 0;
        long s = node.val + subtreeSum(node.left) + subtreeSum(node.right);
        best = Math.max(best, s * (totalSum - s));
        return s;
    }
}
```
**Complexity:** O(n) time (two linear passes), O(h) space — beats the O(n²) brute-force re-summation approach.

---

## 4. Linked List in Binary Tree

**Problem:** Given a binary tree and the head of a linked list, determine if the linked list's values form a downward root-to-leaf-or-partial path in the tree (path can start at ANY node, not just the root, but must follow parent→child edges continuously).

**Example:**
```
Input: head = [4,2,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
Output: true
Explanation: There's a downward path 4 -> 2 -> 8 somewhere in the tree matching
the linked list exactly.
```

**Brute force / actual approach:** at every node in the tree, attempt to match the linked list starting from there — this dual-recursion (outer tree traversal + inner list-matching DFS) IS the standard and necessary approach; there's no way to avoid checking multiple starting points, but memoization doesn't help since match state doesn't overlap between different starting nodes.
```java
class LinkedListNode {
    int val;
    LinkedListNode next;
}

class LinkedListInBinaryTree {
    public boolean isSubPath(LinkedListNode head, TreeNode root) {
        if (root == null) return false;
        return matchFromHere(head, root) || isSubPath(head, root.left) || isSubPath(head, root.right);
    }

    private boolean matchFromHere(LinkedListNode head, TreeNode node) {
        if (head == null) return true; // matched the whole list
        if (node == null || node.val != head.val) return false;
        return matchFromHere(head.next, node.left) || matchFromHere(head.next, node.right);
    }
}
```
**Complexity:** O(n · min(L, h)) time (n = tree nodes, L = list length, h = tree height — matching stops early on mismatch), O(h) space.

---

## 5. Longest Zigzag Path in a Binary Tree

**Problem:** A zigzag path alternates left/right moves at every step (e.g., left, right, left, right, ...). Return the length (number of edges) of the longest zigzag path anywhere in the binary tree.

**Example:**
```
Input: root = [1,null,1,1,1,null,null,1,1,null,1,null,null,null,1,null,1]
Output: 3
Explanation: The longest zigzag path alternates directions for 3 edges 
(e.g., right, left, right).
```

**Brute force:** at every node, try extending zigzag paths in both directions with separate DFS calls per starting node → redundant work, effectively O(n²) in the worst case (skewed tree).
**Optimized:** single post-order DFS returning `{longest zigzag starting by going LEFT from here, longest zigzag starting by going RIGHT from here}` — each node's answer is derived directly from its children's answers in O(1).
```java
class LongestZigzagPath {
    private int best = 0;

    public int longestZigZag(TreeNode root) {
        dfs(root);
        return best;
    }

    // returns {longest zigzag path starting by moving LEFT, starting by moving RIGHT}
    private int[] dfs(TreeNode node) {
        if (node == null) return new int[]{-1, -1};
        int[] left = dfs(node.left);
        int[] right = dfs(node.right);

        int goLeft = left[1] + 1;   // move left, then must continue right at child
        int goRight = right[0] + 1; // move right, then must continue left at child

        best = Math.max(best, Math.max(goLeft, goRight));
        return new int[]{goLeft, goRight};
    }
}
```
**Complexity:** O(n) time, O(h) space — beats the O(n²) worst-case naive per-node restart approach.

---

## 6. Binary Tree Cameras

**Problem:** Place the minimum number of cameras on tree nodes such that every node is monitored. A camera at a node monitors itself, its parent, and its direct children.

**Example:**
```
Input: root = [0,0,null,0,0]
       0
      /
     0
    / \
   0   0
Output: 1
Explanation: A single camera placed at the second node (index 1, the middle one) 
covers itself, its parent, and both its children.
```

**Brute force:** try every subset of nodes as camera placements, check full coverage → O(2ⁿ).
**Optimized:** greedy post-order DFS with 3 states per node — `NOT_COVERED`, `COVERED_NO_CAMERA`, `HAS_CAMERA`. Greedily place a camera at a parent whenever a child is NOT_COVERED (must act before it's too late), which provably yields the optimal count.
```java
class BinaryTreeCameras {
    private int cameras = 0;
    private static final int NOT_COVERED = 0, COVERED_NO_CAMERA = 1, HAS_CAMERA = 2;

    public int minCameraCover(TreeNode root) {
        if (dfs(root) == NOT_COVERED) cameras++; // root itself needs covering
        return cameras;
    }

    private int dfs(TreeNode node) {
        if (node == null) return COVERED_NO_CAMERA; // null nodes don't need coverage
        int left = dfs(node.left);
        int right = dfs(node.right);

        if (left == NOT_COVERED || right == NOT_COVERED) {
            cameras++;
            return HAS_CAMERA;
        }
        if (left == HAS_CAMERA || right == HAS_CAMERA) return COVERED_NO_CAMERA;
        return NOT_COVERED;
    }
}
```
**Complexity:** O(n) time, O(h) space — the greedy strategy (place camera as late as possible, at the parent of any uncovered leaf-ward node) is provably optimal, beating O(2ⁿ) brute force.

---

## 7. Maximum Sum BST in Binary Tree

**Problem:** Given a binary tree (not necessarily a BST), find the maximum sum of all keys in any subtree that IS a valid binary search tree.

**Example:**
```
Input: root = [1,4,3,2,4,2,5,null,null,null,null,null,null,4,6]
Output: 20
Explanation: The subtree rooted at the node with value 3 (containing 3,2,4,2,5,4,6...)
forms a valid BST with the maximum achievable sum of 20.
```

**Brute force:** for every node, check if its subtree is a valid BST (via full inorder traversal + sum), independently → O(n²).
**Optimized:** single post-order DFS returning `{isBST, minVal, maxVal, sum}` per subtree — validity and sum are computed bottom-up in one pass, using children's min/max to validate the BST property at the parent in O(1).
```java
class MaximumSumBST {
    private int maxSum = 0;

    public int maxSumBST(TreeNode root) {
        dfs(root);
        return maxSum;
    }

    // returns {isBST (1/0), minVal, maxVal, sum}
    private int[] dfs(TreeNode node) {
        if (node == null) return new int[]{1, Integer.MAX_VALUE, Integer.MIN_VALUE, 0};

        int[] left = dfs(node.left);
        int[] right = dfs(node.right);

        if (left[0] == 1 && right[0] == 1 && node.val > left[2] && node.val < right[1]) {
            int sum = left[3] + right[3] + node.val;
            maxSum = Math.max(maxSum, sum);
            return new int[]{1, Math.min(left[1], node.val), Math.max(right[2], node.val), sum};
        }
        return new int[]{0, 0, 0, 0}; // not a valid BST — values irrelevant, isBST=0 blocks parent
    }
}
```
**Complexity:** O(n) time, O(h) space — beats the O(n²) brute-force per-node validation.

---

## 8. Number of Ways to Reorder Array to Get Same BST

**Problem:** Given an array of distinct integers representing an insertion sequence into a BST, count how many OTHER permutations of the same array would build an IDENTICAL BST structure (not counting the original array itself).

**Example:**
```
Input: nums = [2,1,3]
Output: 1
Explanation: [2,3,1] is the only other permutation producing the same BST 
(root=2, left=1, right=3).
```

**Brute force:** try every permutation of the array, build a BST for each, compare structures → O(n! · n).
**Optimized:** recursive divide by BST structure — for the root (first element), split remaining elements into those going left (< root) and right (> root); the total arrangements = `C(leftSize+rightSize, leftSize) × ways(left) × ways(right)` (choose which relative positions go to the left subtree's insertion order, recursively count each side's internal arrangements).
```java
class NumWaysReorderSameBST {
    private static final int MOD = 1_000_000_007;
    private long[] fact, invFact;

    public int numOfWays(int[] nums) {
        int n = nums.length;
        fact = new long[n + 1];
        invFact = new long[n + 1];
        fact[0] = 1;
        for (int i = 1; i <= n; i++) fact[i] = fact[i - 1] * i % MOD;
        invFact[n] = power(fact[n], MOD - 2, MOD);
        for (int i = n - 1; i >= 0; i--) invFact[i] = invFact[i + 1] * (i + 1) % MOD;

        return (int) ((count(nums) - 1 + MOD) % MOD); // -1 excludes the original arrangement
    }

    private long count(int[] nums) {
        int n = nums.length;
        if (n <= 2) return 1;

        List<Integer> left = new ArrayList<>();
        List<Integer> right = new ArrayList<>();
        for (int i = 1; i < n; i++) {
            if (nums[i] < nums[0]) left.add(nums[i]);
            else right.add(nums[i]);
        }

        long leftWays = count(toArray(left));
        long rightWays = count(toArray(right));
        long comb = binomial(left.size() + right.size(), left.size());
        return leftWays * rightWays % MOD * comb % MOD;
    }

    private int[] toArray(List<Integer> list) {
        int[] arr = new int[list.size()];
        for (int i = 0; i < arr.length; i++) arr[i] = list.get(i);
        return arr;
    }

    private long binomial(int n, int k) {
        return fact[n] * invFact[k] % MOD * invFact[n - k] % MOD;
    }

    private long power(long base, long exp, long mod) {
        long result = 1;
        base %= mod;
        while (exp > 0) {
            if ((exp & 1) == 1) result = result * base % mod;
            base = base * base % mod;
            exp >>= 1;
        }
        return result;
    }
}
```
**Complexity:** O(n²) time (n levels of recursion, each doing O(n) work to split + O(1) binomial lookup after O(n) precompute), O(n) space — vastly better than O(n!·n) brute force.

---

## 🎯 Part 7 Summary Table

| # | Problem | Time | Space |
|---|---|---|---|
| 1 | Unique BST II | O(Catalan(n)·n) | O(Catalan(n)·n) |
| 2 | House Robber III | O(n) | O(h) |
| 3 | Max Product Splitted Tree | O(n) | O(h) |
| 4 | Linked List in Binary Tree | O(n·min(L,h)) | O(h) |
| 5 | Longest Zigzag Path | O(n) | O(h) |
| 6 | Binary Tree Cameras | O(n) | O(h) |
| 7 | Maximum Sum BST | O(n) | O(h) |
| 8 | Ways to Reorder Same BST | O(n²) | O(n) |

---

**Next: Part 8 — String DP (20 problems).** Say "continue" to proceed, or name a category to jump to.
