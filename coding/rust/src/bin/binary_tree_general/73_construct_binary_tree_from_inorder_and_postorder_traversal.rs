//! LeetCode Top Interview 150 — #73 Construct Binary Tree from Inorder
//! and Postorder Traversal (Medium)
//!
//! Given inorder and postorder traversals of a binary tree with no
//! duplicate values, construct and return the tree. Mirrors #72's
//! technique, but consumes postorder from the back (root, right, left).
//!
//! Example:
//!   Input: inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Box<TreeNode>> {
        let index_map: HashMap<i32, usize> =
            inorder.iter().enumerate().map(|(i, &v)| (v, i)).collect();

        fn helper(
            postorder: &[i32],
            post_idx: &mut i32,
            in_left: i32,
            in_right: i32,
            index_map: &HashMap<i32, usize>,
        ) -> Option<Box<TreeNode>> {
            if in_left > in_right {
                return None;
            }
            let root_val = postorder[*post_idx as usize];
            *post_idx -= 1;
            let mut n = Box::new(TreeNode::new(root_val));
            let mid = index_map[&root_val] as i32;
            // Postorder is Left, Right, Root, so read from the back: Root, Right, Left.
            n.right = helper(postorder, post_idx, mid + 1, in_right, index_map);
            n.left = helper(postorder, post_idx, in_left, mid - 1, index_map);
            Some(n)
        }

        let n = inorder.len() as i32;
        let mut post_idx = n - 1;
        helper(&postorder, &mut post_idx, 0, n - 1, &index_map)
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let result = Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]);
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
        let result = Solution::build_tree(vec![1, 2, 3], vec![1, 2, 3]);
        let expected = node(3, node(2, leaf(1), None), None);
        assert_eq!(result, expected);
    }

    #[test]
    fn right_skewed() {
        let result = Solution::build_tree(vec![1, 2, 3], vec![3, 2, 1]);
        let expected = node(1, None, node(2, None, leaf(3)));
        assert_eq!(result, expected);
    }
}
