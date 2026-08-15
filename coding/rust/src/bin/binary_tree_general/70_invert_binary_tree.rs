//! LeetCode Top Interview 150 — #70 Invert Binary Tree (Easy)
//!
//! Given the root of a binary tree, invert it (mirror left/right children
//! recursively) and return the root.
//!
//! Example:
//!   Input: root = [4,2,7,1,3,6,9]
//!   Output: [4,7,2,9,6,3,1]

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
    pub fn invert_tree(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        root.map(|mut n| {
            let left = Solution::invert_tree(n.left.take());
            let right = Solution::invert_tree(n.right.take());
            n.left = right;
            n.right = left;
            n
        })
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let root = node(4, node(2, leaf(1), leaf(3)), node(7, leaf(6), leaf(9)));
    println!("{:?}", Solution::invert_tree(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(4, node(2, leaf(1), leaf(3)), node(7, leaf(6), leaf(9)));
        let expected = node(4, node(7, leaf(9), leaf(6)), node(2, leaf(3), leaf(1)));
        assert_eq!(Solution::invert_tree(root), expected);
    }

    #[test]
    fn example_2_two_nodes() {
        let root = node(2, leaf(1), None);
        let expected = node(2, None, leaf(1));
        assert_eq!(Solution::invert_tree(root), expected);
    }

    #[test]
    fn example_3_empty() {
        assert_eq!(Solution::invert_tree(None), None);
    }

    #[test]
    fn single_node_unchanged() {
        assert_eq!(Solution::invert_tree(leaf(1)), leaf(1));
    }
}
