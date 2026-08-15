//! LeetCode Top Interview 150 — #76 Path Sum (Easy)
//!
//! Given the root of a binary tree and an integer targetSum, return true
//! if the tree has a root-to-leaf path such that the values sum to
//! targetSum.
//!
//! Example:
//!   Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
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
    pub fn has_path_sum(root: Option<Box<TreeNode>>, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(n) => {
                let remaining = target_sum - n.val;
                if n.left.is_none() && n.right.is_none() {
                    remaining == 0
                } else {
                    Solution::has_path_sum(n.left, remaining)
                        || Solution::has_path_sum(n.right, remaining)
                }
            }
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
    let root = node(
        5,
        node(4, node(11, leaf(7), leaf(2)), None),
        node(8, leaf(13), node(4, None, leaf(1))),
    );
    println!("{}", Solution::has_path_sum(root, 22));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(
            5,
            node(4, node(11, leaf(7), leaf(2)), None),
            node(8, leaf(13), node(4, None, leaf(1))),
        );
        assert_eq!(Solution::has_path_sum(root, 22), true);
    }

    #[test]
    fn example_2_no_matching_path() {
        let root = node(1, leaf(2), leaf(3));
        assert_eq!(Solution::has_path_sum(root, 5), false);
    }

    #[test]
    fn example_3_empty_tree() {
        assert_eq!(Solution::has_path_sum(None, 0), false);
    }

    #[test]
    fn single_node_matching() {
        assert_eq!(Solution::has_path_sum(leaf(5), 5), true);
    }
}
