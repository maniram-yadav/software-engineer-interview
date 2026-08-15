//! LeetCode Top Interview 150 — #69 Same Tree (Easy)
//!
//! Given the roots of two binary trees, check if they are structurally
//! identical with the same node values.
//!
//! Example:
//!   Input: p = [1,2,3], q = [1,2,3]
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
    pub fn is_same_tree(p: Option<Box<TreeNode>>, q: Option<Box<TreeNode>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                a.val == b.val
                    && Solution::is_same_tree(a.left, b.left)
                    && Solution::is_same_tree(a.right, b.right)
            }
            _ => false,
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
    let p = node(1, leaf(2), leaf(3));
    let q = node(1, leaf(2), leaf(3));
    println!("{}", Solution::is_same_tree(p, q));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_equal() {
        let p = node(1, leaf(2), leaf(3));
        let q = node(1, leaf(2), leaf(3));
        assert_eq!(Solution::is_same_tree(p, q), true);
    }

    #[test]
    fn example_2_different_structure() {
        let p = node(1, leaf(2), None);
        let q = node(1, None, leaf(2));
        assert_eq!(Solution::is_same_tree(p, q), false);
    }

    #[test]
    fn example_3_different_values() {
        let p = node(1, leaf(2), leaf(1));
        let q = node(1, leaf(1), leaf(2));
        assert_eq!(Solution::is_same_tree(p, q), false);
    }

    #[test]
    fn both_empty() {
        assert_eq!(Solution::is_same_tree(None, None), true);
    }
}
