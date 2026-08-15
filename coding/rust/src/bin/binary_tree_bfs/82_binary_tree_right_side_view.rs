//! LeetCode Top Interview 150 — #82 Binary Tree Right Side View (Medium)
//!
//! Given the root of a binary tree, return the values visible from the
//! right side, ordered top to bottom. Solved with BFS level order,
//! keeping only the last node visited at each level.
//!
//! Example:
//!   Input: root = [1,2,3,null,5,null,4]
//!   Output: [1,3,4]

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
    pub fn right_side_view(root: Option<Box<TreeNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut queue: VecDeque<Box<TreeNode>> = VecDeque::new();
        if let Some(r) = root {
            queue.push_back(r);
        }

        while !queue.is_empty() {
            let level_len = queue.len();
            for i in 0..level_len {
                let mut n = queue.pop_front().unwrap();
                if i == level_len - 1 {
                    result.push(n.val);
                }
                if let Some(l) = n.left.take() {
                    queue.push_back(l);
                }
                if let Some(r) = n.right.take() {
                    queue.push_back(r);
                }
            }
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
    let root = node(1, node(2, None, leaf(5)), node(3, None, leaf(4)));
    println!("{:?}", Solution::right_side_view(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(1, node(2, None, leaf(5)), node(3, None, leaf(4)));
        assert_eq!(Solution::right_side_view(root), vec![1, 3, 4]);
    }

    #[test]
    fn example_2_single_node() {
        assert_eq!(Solution::right_side_view(leaf(1)), vec![1]);
    }

    #[test]
    fn example_3_empty_tree() {
        assert_eq!(Solution::right_side_view(None), Vec::<i32>::new());
    }

    #[test]
    fn left_only_chain() {
        let root = node(1, node(2, leaf(3), None), None);
        assert_eq!(Solution::right_side_view(root), vec![1, 2, 3]);
    }
}
