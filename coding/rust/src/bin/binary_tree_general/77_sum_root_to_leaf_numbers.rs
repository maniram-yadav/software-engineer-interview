//! LeetCode Top Interview 150 — #77 Sum Root to Leaf Numbers (Medium)
//!
//! Each root-to-leaf path represents a number (digits left to right).
//! Return the total sum of all root-to-leaf numbers.
//!
//! Example:
//!   Input: root = [4,9,0,5,1]
//!   Output: 1026

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
    pub fn sum_numbers(root: Option<Box<TreeNode>>) -> i32 {
        fn dfs(node: &Option<Box<TreeNode>>, current: i32) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let current = current * 10 + n.val;
                    if n.left.is_none() && n.right.is_none() {
                        current
                    } else {
                        dfs(&n.left, current) + dfs(&n.right, current)
                    }
                }
            }
        }
        dfs(&root, 0)
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let root = node(4, node(9, leaf(5), leaf(1)), leaf(0));
    println!("{}", Solution::sum_numbers(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(1, leaf(2), leaf(3));
        assert_eq!(Solution::sum_numbers(root), 25);
    }

    #[test]
    fn example_2() {
        let root = node(4, node(9, leaf(5), leaf(1)), leaf(0));
        assert_eq!(Solution::sum_numbers(root), 1026);
    }

    #[test]
    fn single_node() {
        assert_eq!(Solution::sum_numbers(leaf(7)), 7);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(Solution::sum_numbers(None), 0);
    }
}
