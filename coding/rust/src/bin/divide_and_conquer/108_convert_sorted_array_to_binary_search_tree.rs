//! LeetCode Top Interview 150 — #108 Convert Sorted Array to Binary
//! Search Tree (Easy)
//!
//! Given an integer array sorted ascending, convert it to a
//! height-balanced BST. Solved by always picking the middle element as
//! root and recursing on each half — any valid answer is accepted, so
//! tests check the invariants (in-order matches input, height-balanced)
//! rather than one specific shape.
//!
//! Example:
//!   Input: nums = [-10,-3,0,5,9]
//!   Output: one valid height-balanced BST

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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Box<TreeNode>> {
        fn build(nums: &[i32]) -> Option<Box<TreeNode>> {
            if nums.is_empty() {
                return None;
            }
            let mid = nums.len() / 2;
            let mut n = Box::new(TreeNode::new(nums[mid]));
            n.left = build(&nums[..mid]);
            n.right = build(&nums[mid + 1..]);
            Some(n)
        }
        build(&nums)
    }
}

fn inorder_vals(node: &Option<Box<TreeNode>>, out: &mut Vec<i32>) {
    if let Some(n) = node {
        inorder_vals(&n.left, out);
        out.push(n.val);
        inorder_vals(&n.right, out);
    }
}

fn height(node: &Option<Box<TreeNode>>) -> i32 {
    match node {
        None => 0,
        Some(n) => 1 + height(&n.left).max(height(&n.right)),
    }
}

fn is_balanced(node: &Option<Box<TreeNode>>) -> bool {
    match node {
        None => true,
        Some(n) => {
            (height(&n.left) - height(&n.right)).abs() <= 1
                && is_balanced(&n.left)
                && is_balanced(&n.right)
        }
    }
}

fn main() {
    let tree = Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]);
    let mut vals = Vec::new();
    inorder_vals(&tree, &mut vals);
    println!("inorder: {:?}, balanced: {}", vals, is_balanced(&tree));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_preserves_order_and_is_balanced() {
        let nums = vec![-10, -3, 0, 5, 9];
        let tree = Solution::sorted_array_to_bst(nums.clone());
        let mut vals = Vec::new();
        inorder_vals(&tree, &mut vals);
        assert_eq!(vals, nums);
        assert!(is_balanced(&tree));
    }

    #[test]
    fn example_2_two_elements() {
        let nums = vec![1, 3];
        let tree = Solution::sorted_array_to_bst(nums.clone());
        let mut vals = Vec::new();
        inorder_vals(&tree, &mut vals);
        assert_eq!(vals, nums);
        assert!(is_balanced(&tree));
    }

    #[test]
    fn empty_input() {
        assert_eq!(Solution::sorted_array_to_bst(vec![]), None);
    }

    #[test]
    fn single_element() {
        let tree = Solution::sorted_array_to_bst(vec![5]);
        assert_eq!(tree, Some(Box::new(TreeNode::new(5))));
    }
}
