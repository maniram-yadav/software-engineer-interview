//! LeetCode Top Interview 150 — #86 Minimum Absolute Difference in BST (Easy)
//!
//! Given the root of a BST, return the minimum absolute difference
//! between the values of any two distinct nodes. Solved with an in-order
//! traversal (yields sorted values), tracking the gap to the previous
//! value visited.
//!
//! Example:
//!   Input: root = [4,2,6,1,3]
//!   Output: 1

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
    pub fn get_minimum_difference(root: Option<Box<TreeNode>>) -> i32 {
        fn inorder(node: &Option<Box<TreeNode>>, prev: &mut Option<i32>, best: &mut i32) {
            if let Some(n) = node {
                inorder(&n.left, prev, best);
                if let Some(p) = *prev {
                    *best = (*best).min(n.val - p);
                }
                *prev = Some(n.val);
                inorder(&n.right, prev, best);
            }
        }

        let mut prev = None;
        let mut best = i32::MAX;
        inorder(&root, &mut prev, &mut best);
        best
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let root = node(4, node(2, leaf(1), leaf(3)), leaf(6));
    println!("{}", Solution::get_minimum_difference(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(4, node(2, leaf(1), leaf(3)), leaf(6));
        assert_eq!(Solution::get_minimum_difference(root), 1);
    }

    #[test]
    fn example_2() {
        let root = node(1, None, node(3, leaf(2), None));
        assert_eq!(Solution::get_minimum_difference(root), 1);
    }

    #[test]
    fn two_nodes() {
        let root = node(5, leaf(1), None);
        assert_eq!(Solution::get_minimum_difference(root), 4);
    }

    #[test]
    fn wide_gaps() {
        let root = node(10, leaf(1), leaf(100));
        assert_eq!(Solution::get_minimum_difference(root), 9);
    }
}
