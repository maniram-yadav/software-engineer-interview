//! LeetCode Top Interview 150 — #87 Kth Smallest Element in a BST (Medium)
//!
//! Given the root of a BST and an integer k, return the k-th smallest
//! value (1-indexed). Solved with an in-order traversal (yields sorted
//! values) that stops as soon as the k-th value is found.
//!
//! Example:
//!   Input: root = [3,1,4,null,2], k = 1
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
    pub fn kth_smallest(root: Option<Box<TreeNode>>, k: i32) -> i32 {
        fn inorder(node: &Option<Box<TreeNode>>, k: &mut i32, result: &mut Option<i32>) {
            if result.is_some() {
                return;
            }
            if let Some(n) = node {
                inorder(&n.left, k, result);
                if result.is_some() {
                    return;
                }
                *k -= 1;
                if *k == 0 {
                    *result = Some(n.val);
                    return;
                }
                inorder(&n.right, k, result);
            }
        }

        let mut k = k;
        let mut result = None;
        inorder(&root, &mut k, &mut result);
        result.unwrap()
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let root = node(3, node(1, None, leaf(2)), leaf(4));
    println!("{}", Solution::kth_smallest(root, 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(3, node(1, None, leaf(2)), leaf(4));
        assert_eq!(Solution::kth_smallest(root, 1), 1);
    }

    #[test]
    fn example_2() {
        let root = node(
            5,
            node(3, node(2, leaf(1), None), leaf(4)),
            leaf(6),
        );
        assert_eq!(Solution::kth_smallest(root, 3), 3);
    }

    #[test]
    fn last_element() {
        let root = node(2, leaf(1), leaf(3));
        assert_eq!(Solution::kth_smallest(root, 3), 3);
    }

    #[test]
    fn single_node() {
        assert_eq!(Solution::kth_smallest(leaf(1), 1), 1);
    }
}
