//! LeetCode Top Interview 150 — #88 Validate Binary Search Tree (Medium)
//!
//! Given the root of a binary tree, determine if it is a valid BST (left
//! subtree values < node < right subtree values, for every node).
//! Solved by threading a valid (lower, upper) bound down through the
//! recursion; bounds use i64 to safely exceed i32::MIN/MAX.
//!
//! Example:
//!   Input: root = [2,1,3]
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
    pub fn is_valid_bst(root: Option<Box<TreeNode>>) -> bool {
        fn helper(node: &Option<Box<TreeNode>>, lower: Option<i64>, upper: Option<i64>) -> bool {
            match node {
                None => true,
                Some(n) => {
                    let val = n.val as i64;
                    if let Some(low) = lower {
                        if val <= low {
                            return false;
                        }
                    }
                    if let Some(up) = upper {
                        if val >= up {
                            return false;
                        }
                    }
                    helper(&n.left, lower, Some(val)) && helper(&n.right, Some(val), upper)
                }
            }
        }

        helper(&root, None, None)
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let root = node(2, leaf(1), leaf(3));
    println!("{}", Solution::is_valid_bst(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_valid() {
        let root = node(2, leaf(1), leaf(3));
        assert_eq!(Solution::is_valid_bst(root), true);
    }

    #[test]
    fn example_2_invalid() {
        let root = node(5, leaf(1), node(4, leaf(3), leaf(6)));
        assert_eq!(Solution::is_valid_bst(root), false);
    }

    #[test]
    fn duplicate_values_invalid() {
        let root = node(1, leaf(1), None);
        assert_eq!(Solution::is_valid_bst(root), false);
    }

    #[test]
    fn boundary_values() {
        let root = node(i32::MIN, None, leaf(i32::MAX));
        assert_eq!(Solution::is_valid_bst(root), true);
    }

    #[test]
    fn single_node() {
        assert_eq!(Solution::is_valid_bst(leaf(0)), true);
    }
}
