//! Grind 169 — LeetCode #110 Balanced Binary Tree (Easy)
//!
//! Given a binary tree, determine if it is height-balanced (the depth of
//! the two subtrees of every node never differs by more than 1). A
//! single post-order pass returns -1 up the call stack the moment any
//! subtree is found unbalanced, short-circuiting further work.
//!
//! Example:
//!   Input: root = [3,9,20,null,null,15,7]
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
    pub fn is_balanced(root: Option<Box<TreeNode>>) -> bool {
        fn height(node: &Option<Box<TreeNode>>) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let lh = height(&n.left);
                    if lh == -1 {
                        return -1;
                    }
                    let rh = height(&n.right);
                    if rh == -1 {
                        return -1;
                    }
                    if (lh - rh).abs() > 1 {
                        return -1;
                    }
                    1 + lh.max(rh)
                }
            }
        }
        height(&root) != -1
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
    println!("{}", Solution::is_balanced(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(3, leaf(9), node(20, leaf(15), leaf(7)));
        assert_eq!(Solution::is_balanced(root), true);
    }

    #[test]
    fn example_2_unbalanced() {
        let root = node(
            1,
            node(2, node(3, leaf(4), leaf(4)), leaf(3)),
            leaf(2),
        );
        assert_eq!(Solution::is_balanced(root), false);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(Solution::is_balanced(None), true);
    }

    #[test]
    fn single_node() {
        assert_eq!(Solution::is_balanced(leaf(1)), true);
    }
}
