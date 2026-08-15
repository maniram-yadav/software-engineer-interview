//! LeetCode Top Interview 150 — #83 Average of Levels in Binary Tree (Easy)
//!
//! Given the root of a binary tree, return the average value of nodes at
//! each level.
//!
//! Example:
//!   Input: root = [3,9,20,null,null,15,7]
//!   Output: [3.0,14.5,11.0]

use std::collections::VecDeque;

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
    pub fn average_of_levels(root: Option<Box<TreeNode>>) -> Vec<f64> {
        let mut result = Vec::new();
        let mut queue: VecDeque<Box<TreeNode>> = VecDeque::new();
        if let Some(r) = root {
            queue.push_back(r);
        }

        while !queue.is_empty() {
            let level_len = queue.len();
            let mut sum: i64 = 0;
            for _ in 0..level_len {
                let mut n = queue.pop_front().unwrap();
                sum += n.val as i64;
                if let Some(l) = n.left.take() {
                    queue.push_back(l);
                }
                if let Some(r) = n.right.take() {
                    queue.push_back(r);
                }
            }
            result.push(sum as f64 / level_len as f64);
        }

        result
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let root = node(3, leaf(9), node(20, leaf(15), leaf(7)));
    println!("{:?}", Solution::average_of_levels(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(3, leaf(9), node(20, leaf(15), leaf(7)));
        assert_eq!(Solution::average_of_levels(root), vec![3.0, 14.5, 11.0]);
    }

    #[test]
    fn example_2_single_node() {
        assert_eq!(Solution::average_of_levels(leaf(5)), vec![5.0]);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(Solution::average_of_levels(None), Vec::<f64>::new());
    }

    #[test]
    fn negative_values() {
        let root = node(-1, leaf(-2), leaf(-3));
        assert_eq!(Solution::average_of_levels(root), vec![-1.0, -2.5]);
    }
}
