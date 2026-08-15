//! LeetCode Top Interview 150 — #80 Count Complete Tree Nodes (Easy)
//!
//! Given the root of a complete binary tree, count the number of nodes.
//! A straightforward O(n) recursive count — correct for any binary tree,
//! not just complete ones (the O(log^2 n) shortcut that exploits
//! completeness is left as an optimization).
//!
//! Example:
//!   Input: root = [1,2,3,4,5,6]
//!   Output: 6

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
    pub fn count_nodes(root: Option<Box<TreeNode>>) -> i32 {
        match root {
            None => 0,
            Some(n) => 1 + Solution::count_nodes(n.left) + Solution::count_nodes(n.right),
        }
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let root = node(1, node(2, leaf(4), leaf(5)), node(3, leaf(6), None));
    println!("{}", Solution::count_nodes(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(1, node(2, leaf(4), leaf(5)), node(3, leaf(6), None));
        assert_eq!(Solution::count_nodes(root), 6);
    }

    #[test]
    fn example_2_empty() {
        assert_eq!(Solution::count_nodes(None), 0);
    }

    #[test]
    fn example_3_single_node() {
        assert_eq!(Solution::count_nodes(leaf(1)), 1);
    }

    #[test]
    fn perfect_tree_of_seven() {
        let root = node(
            1,
            node(2, leaf(4), leaf(5)),
            node(3, leaf(6), leaf(7)),
        );
        assert_eq!(Solution::count_nodes(root), 7);
    }
}
