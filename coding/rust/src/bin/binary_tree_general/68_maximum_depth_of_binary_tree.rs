//! LeetCode Top Interview 150 — #68 Maximum Depth of Binary Tree (Easy)
//!
//! Given the root of a binary tree, return its maximum depth (number of
//! nodes along the longest path from root to leaf).
//!
//! Example:
//!   Input: root = [3,9,20,null,null,15,7]
//!   Output: 3

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
    pub fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                1 + Solution::max_depth(node.left).max(Solution::max_depth(node.right))
            }
        }
    }
}

// Test-only helpers for building small trees by hand.
fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let root = node(3, leaf(9), node(20, leaf(15), leaf(7)));
    println!("{}", Solution::max_depth(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(3, leaf(9), node(20, leaf(15), leaf(7)));
        assert_eq!(Solution::max_depth(root), 3);
    }

    #[test]
    fn example_2_single_node() {
        assert_eq!(Solution::max_depth(leaf(1)), 1);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(Solution::max_depth(None), 0);
    }

    #[test]
    fn left_skewed() {
        let root = node(1, node(2, leaf(3), None), None);
        assert_eq!(Solution::max_depth(root), 3);
    }
}
