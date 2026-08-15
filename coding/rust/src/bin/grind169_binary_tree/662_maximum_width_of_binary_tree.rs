//! Grind 169 — LeetCode #662 Maximum Width of Binary Tree (Medium)
//!
//! Given the root of a binary tree, return the maximum width of any
//! level (distance between leftmost and rightmost non-null nodes,
//! counting nulls in between as if the tree were complete). Solved with
//! BFS, assigning each node a positional index as in a complete binary
//! tree's array representation (children of index i are 2i and 2i+1).
//!
//! Example:
//!   Input: root = [1,3,2,5,3,null,9]
//!   Output: 4

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
    pub fn width_of_binary_tree(root: Option<Box<TreeNode>>) -> i32 {
        let root = match root {
            Some(r) => r,
            None => return 0,
        };
        let mut queue: VecDeque<(Box<TreeNode>, u64)> = VecDeque::new();
        queue.push_back((root, 0));
        let mut best: u64 = 0;

        while !queue.is_empty() {
            let level_len = queue.len();
            let front_idx = queue.front().unwrap().1;
            let mut last_idx = front_idx;
            for _ in 0..level_len {
                let (mut n, idx) = queue.pop_front().unwrap();
                last_idx = idx;
                if let Some(l) = n.left.take() {
                    queue.push_back((l, idx * 2));
                }
                if let Some(r) = n.right.take() {
                    queue.push_back((r, idx * 2 + 1));
                }
            }
            best = best.max(last_idx - front_idx + 1);
        }

        best as i32
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let root = node(1, node(3, leaf(5), leaf(3)), node(2, None, leaf(9)));
    println!("{}", Solution::width_of_binary_tree(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(1, node(3, leaf(5), leaf(3)), node(2, None, leaf(9)));
        assert_eq!(Solution::width_of_binary_tree(root), 4);
    }

    #[test]
    fn example_2_single_node() {
        assert_eq!(Solution::width_of_binary_tree(leaf(1)), 1);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(Solution::width_of_binary_tree(None), 0);
    }

    #[test]
    fn left_skewed() {
        let root = node(1, node(2, leaf(3), None), None);
        assert_eq!(Solution::width_of_binary_tree(root), 1);
    }
}
