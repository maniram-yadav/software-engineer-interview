//! Grind 169 — LeetCode #235 Lowest Common Ancestor of a Binary Search
//! Tree (Medium)
//!
//! Given a BST and two nodes p and q, find their lowest common ancestor,
//! using BST ordering. Nodes are identified by value (values assumed
//! unique, as is standard for this problem). Walk down from the root:
//! if both targets are smaller, go left; if both are larger, go right;
//! otherwise the current node is the split point (the LCA).
//!
//! Example:
//!   Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
//!   Output: 6

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Box<TreeNode>>, p: i32, q: i32) -> Option<i32> {
        let mut node = &root;
        while let Some(n) = node {
            if p < n.val && q < n.val {
                node = &n.left;
            } else if p > n.val && q > n.val {
                node = &n.right;
            } else {
                return Some(n.val);
            }
        }
        None
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn example_tree() -> Option<Box<TreeNode>> {
    node(
        6,
        node(2, leaf(0), node(4, leaf(3), leaf(5))),
        node(8, leaf(7), leaf(9)),
    )
}

fn main() {
    let root = example_tree();
    println!("{:?}", Solution::lowest_common_ancestor(root, 2, 8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = example_tree();
        assert_eq!(Solution::lowest_common_ancestor(root, 2, 8), Some(6));
    }

    #[test]
    fn example_2_one_is_ancestor_of_other() {
        let root = example_tree();
        assert_eq!(Solution::lowest_common_ancestor(root, 2, 4), Some(2));
    }

    #[test]
    fn two_node_tree() {
        let root = node(1, leaf(0), None);
        assert_eq!(Solution::lowest_common_ancestor(root, 0, 1), Some(1));
    }

    #[test]
    fn root_is_lca() {
        let root = node(5, leaf(3), leaf(8));
        assert_eq!(Solution::lowest_common_ancestor(root, 3, 8), Some(5));
    }
}
