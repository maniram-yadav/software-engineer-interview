//! LeetCode Top Interview 150 — #71 Symmetric Tree (Easy)
//!
//! Given the root of a binary tree, check whether it is a mirror of
//! itself around its center.
//!
//! Example:
//!   Input: root = [1,2,2,3,4,4,3]
//!   Output: true

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
    pub fn is_symmetric(root: Option<Box<TreeNode>>) -> bool {
        fn mirror(a: &Option<Box<TreeNode>>, b: &Option<Box<TreeNode>>) -> bool {
            match (a, b) {
                (None, None) => true,
                (Some(x), Some(y)) => {
                    x.val == y.val && mirror(&x.left, &y.right) && mirror(&x.right, &y.left)
                }
                _ => false,
            }
        }

        match &root {
            None => true,
            Some(node) => mirror(&node.left, &node.right),
        }
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let root = node(1, node(2, leaf(3), leaf(4)), node(2, leaf(4), leaf(3)));
    println!("{}", Solution::is_symmetric(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_symmetric() {
        let root = node(1, node(2, leaf(3), leaf(4)), node(2, leaf(4), leaf(3)));
        assert_eq!(Solution::is_symmetric(root), true);
    }

    #[test]
    fn example_2_not_symmetric() {
        let root = node(1, node(2, None, leaf(3)), node(2, None, leaf(3)));
        assert_eq!(Solution::is_symmetric(root), false);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(Solution::is_symmetric(None), true);
    }

    #[test]
    fn single_node() {
        assert_eq!(Solution::is_symmetric(leaf(1)), true);
    }
}
