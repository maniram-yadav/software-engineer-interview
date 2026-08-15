//! Grind 169 — LeetCode #572 Subtree of Another Tree (Easy)
//!
//! Given two binary trees root and subRoot, return true if subRoot has
//! the same structure and node values as some subtree of root. Checks
//! for an exact match at every node of root, recursively.
//!
//! Example:
//!   Input: root = [3,4,5,1,2], subRoot = [4,1,2]
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
    pub fn is_subtree(root: Option<Box<TreeNode>>, sub_root: Option<Box<TreeNode>>) -> bool {
        fn same(a: &Option<Box<TreeNode>>, b: &Option<Box<TreeNode>>) -> bool {
            match (a, b) {
                (None, None) => true,
                (Some(x), Some(y)) => {
                    x.val == y.val && same(&x.left, &y.left) && same(&x.right, &y.right)
                }
                _ => false,
            }
        }
        fn helper(root: &Option<Box<TreeNode>>, sub: &Option<Box<TreeNode>>) -> bool {
            match root {
                None => sub.is_none(),
                Some(n) => same(root, sub) || helper(&n.left, sub) || helper(&n.right, sub),
            }
        }
        helper(&root, &sub_root)
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let root = node(3, node(4, leaf(1), leaf(2)), leaf(5));
    let sub_root = node(4, leaf(1), leaf(2));
    println!("{}", Solution::is_subtree(root, sub_root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(3, node(4, leaf(1), leaf(2)), leaf(5));
        let sub_root = node(4, leaf(1), leaf(2));
        assert_eq!(Solution::is_subtree(root, sub_root), true);
    }

    #[test]
    fn example_2_extra_node_breaks_match() {
        let extra = node(4, leaf(1), leaf(2)).map(|mut n| {
            n.left.as_mut().unwrap().left = leaf(0);
            n
        });
        let root = node(3, extra, leaf(5));
        let sub_root = node(4, leaf(1), leaf(2));
        assert_eq!(Solution::is_subtree(root, sub_root), false);
    }

    #[test]
    fn whole_tree_matches_itself() {
        let root = node(1, leaf(2), leaf(3));
        let sub_root = node(1, leaf(2), leaf(3));
        assert_eq!(Solution::is_subtree(root, sub_root), true);
    }

    #[test]
    fn empty_subroot_always_matches() {
        let root = leaf(1);
        assert_eq!(Solution::is_subtree(root, None), true);
    }
}
