//! LeetCode Top Interview 150 — #72 Construct Binary Tree from Preorder
//! and Inorder Traversal (Medium)
//!
//! Given two integer arrays preorder and inorder (no duplicate values)
//! representing a binary tree's traversals, construct and return the
//! tree. Solved with a HashMap for O(1) inorder index lookups plus
//! recursion over index ranges (no slicing/copying).
//!
//! Example:
//!   Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
//!   Output: [3,9,20,null,null,15,7]

use std::collections::HashMap;

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Box<TreeNode>> {
        let index_map: HashMap<i32, usize> =
            inorder.iter().enumerate().map(|(i, &v)| (v, i)).collect();

        fn helper(
            preorder: &[i32],
            pre_idx: &mut usize,
            in_left: i32,
            in_right: i32,
            index_map: &HashMap<i32, usize>,
        ) -> Option<Box<TreeNode>> {
            if in_left > in_right {
                return None;
            }
            let root_val = preorder[*pre_idx];
            *pre_idx += 1;
            let mut n = Box::new(TreeNode::new(root_val));
            let mid = index_map[&root_val] as i32;
            n.left = helper(preorder, pre_idx, in_left, mid - 1, index_map);
            n.right = helper(preorder, pre_idx, mid + 1, in_right, index_map);
            Some(n)
        }

        let n = inorder.len() as i32;
        let mut pre_idx = 0usize;
        helper(&preorder, &mut pre_idx, 0, n - 1, &index_map)
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let result = Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
        let expected = node(3, leaf(9), node(20, leaf(15), leaf(7)));
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2_single_node() {
        let result = Solution::build_tree(vec![-1], vec![-1]);
        assert_eq!(result, leaf(-1));
    }

    #[test]
    fn left_skewed() {
        let result = Solution::build_tree(vec![3, 2, 1], vec![1, 2, 3]);
        let expected = node(3, node(2, leaf(1), None), None);
        assert_eq!(result, expected);
    }

    #[test]
    fn right_skewed() {
        let result = Solution::build_tree(vec![1, 2, 3], vec![1, 2, 3]);
        let expected = node(1, None, node(2, None, leaf(3)));
        assert_eq!(result, expected);
    }
}
