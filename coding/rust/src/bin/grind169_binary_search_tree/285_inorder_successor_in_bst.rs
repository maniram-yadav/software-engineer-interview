//! Grind 169 — LeetCode #285 Inorder Successor in BST (Medium, Premium)
//!
//! Given a BST and a value p, find the in-order successor of p (the
//! node with the smallest value greater than p), or null if none. Walk
//! down from the root: whenever the current node's value exceeds p, it's
//! a successor candidate (remember it) and search left for something
//! smaller-but-still-greater-than-p; otherwise search right.
//!
//! Example:
//!   Input: root = [2,1,3], p = 1
//!   Output: 2

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
    pub fn inorder_successor(root: Option<Box<TreeNode>>, p: i32) -> Option<i32> {
        let mut node = &root;
        let mut successor = None;
        while let Some(n) = node {
            if p < n.val {
                successor = Some(n.val);
                node = &n.left;
            } else {
                node = &n.right;
            }
        }
        successor
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let root = node(2, leaf(1), leaf(3));
    println!("{:?}", Solution::inorder_successor(root, 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(2, leaf(1), leaf(3));
        assert_eq!(Solution::inorder_successor(root, 1), Some(2));
    }

    #[test]
    fn example_2_no_successor() {
        let root = node(
            5,
            node(3, leaf(2), leaf(4)),
            node(6, None, None),
        );
        assert_eq!(Solution::inorder_successor(root, 6), None);
    }

    #[test]
    fn successor_via_right_subtree() {
        let root = node(2, leaf(1), leaf(3));
        assert_eq!(Solution::inorder_successor(root, 2), Some(3));
    }

    #[test]
    fn single_node_has_no_successor() {
        assert_eq!(Solution::inorder_successor(leaf(1), 1), None);
    }
}
