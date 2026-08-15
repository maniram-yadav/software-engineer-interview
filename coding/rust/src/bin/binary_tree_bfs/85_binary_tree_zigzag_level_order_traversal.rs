//! LeetCode Top Interview 150 — #85 Binary Tree Zigzag Level Order
//! Traversal (Medium)
//!
//! Same as level order, but alternate direction each level (left-to-right,
//! then right-to-left, ...).
//!
//! Example:
//!   Input: root = [3,9,20,null,null,15,7]
//!   Output: [[3],[20,9],[15,7]]

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
    pub fn zigzag_level_order(root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut queue: VecDeque<Box<TreeNode>> = VecDeque::new();
        if let Some(r) = root {
            queue.push_back(r);
        }
        let mut left_to_right = true;

        while !queue.is_empty() {
            let level_len = queue.len();
            let mut level = Vec::new();
            for _ in 0..level_len {
                let mut n = queue.pop_front().unwrap();
                level.push(n.val);
                if let Some(l) = n.left.take() {
                    queue.push_back(l);
                }
                if let Some(r) = n.right.take() {
                    queue.push_back(r);
                }
            }
            if !left_to_right {
                level.reverse();
            }
            result.push(level);
            left_to_right = !left_to_right;
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
    println!("{:?}", Solution::zigzag_level_order(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(3, leaf(9), node(20, leaf(15), leaf(7)));
        assert_eq!(
            Solution::zigzag_level_order(root),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }

    #[test]
    fn example_2_single_node() {
        assert_eq!(Solution::zigzag_level_order(leaf(1)), vec![vec![1]]);
    }

    #[test]
    fn example_3_empty_tree() {
        assert_eq!(Solution::zigzag_level_order(None), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn four_levels() {
        let root = node(
            1,
            node(2, leaf(4), None),
            node(3, None, leaf(5)),
        );
        assert_eq!(
            Solution::zigzag_level_order(root),
            vec![vec![1], vec![3, 2], vec![4, 5]]
        );
    }
}
