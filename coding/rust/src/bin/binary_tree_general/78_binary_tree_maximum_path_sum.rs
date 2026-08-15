//! LeetCode Top Interview 150 — #78 Binary Tree Maximum Path Sum (Hard)
//!
//! Given the root of a binary tree, find the maximum path sum of any
//! non-empty path (path need not pass through the root, and a node may
//! be used at most once). Solved with post-order DFS: each call returns
//! the best downward extension through the current node, while tracking
//! the best "through" path (left + node + right) in a side variable.
//!
//! Example:
//!   Input: root = [-10,9,20,null,null,15,7]
//!   Output: 42   (path 15 -> 20 -> 7)

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
    pub fn max_path_sum(root: Option<Box<TreeNode>>) -> i32 {
        fn dfs(node: &Option<Box<TreeNode>>, best: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let left = dfs(&n.left, best).max(0);
                    let right = dfs(&n.right, best).max(0);
                    *best = (*best).max(n.val + left + right);
                    n.val + left.max(right)
                }
            }
        }

        let mut best = i32::MIN;
        dfs(&root, &mut best);
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
    let root = node(-10, leaf(9), node(20, leaf(15), leaf(7)));
    println!("{}", Solution::max_path_sum(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(1, leaf(2), leaf(3));
        assert_eq!(Solution::max_path_sum(root), 6);
    }

    #[test]
    fn example_2() {
        let root = node(-10, leaf(9), node(20, leaf(15), leaf(7)));
        assert_eq!(Solution::max_path_sum(root), 42);
    }

    #[test]
    fn single_negative_node() {
        assert_eq!(Solution::max_path_sum(leaf(-3)), -3);
    }

    #[test]
    fn all_negative_picks_least_negative() {
        let root = node(-2, leaf(-1), None);
        assert_eq!(Solution::max_path_sum(root), -1);
    }
}
