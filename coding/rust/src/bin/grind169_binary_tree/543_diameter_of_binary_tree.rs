//! Grind 169 — LeetCode #543 Diameter of Binary Tree (Easy)
//!
//! Given the root of a binary tree, return the length (in edges) of the
//! longest path between any two nodes. A post-order DFS computes each
//! subtree's depth while tracking the best "left depth + right depth"
//! seen at any node along the way.
//!
//! Example:
//!   Input: root = [1,2,3,4,5]
//!   Output: 3   (path [4,2,1,3] or [5,2,1,3])

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
    pub fn diameter_of_binary_tree(root: Option<Box<TreeNode>>) -> i32 {
        fn depth(node: &Option<Box<TreeNode>>, best: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let l = depth(&n.left, best);
                    let r = depth(&n.right, best);
                    *best = (*best).max(l + r);
                    1 + l.max(r)
                }
            }
        }
        let mut best = 0;
        depth(&root, &mut best);
        best
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let root = node(1, node(2, leaf(4), leaf(5)), leaf(3));
    println!("{}", Solution::diameter_of_binary_tree(root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(1, node(2, leaf(4), leaf(5)), leaf(3));
        assert_eq!(Solution::diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn example_2_two_nodes() {
        let root = leaf(1).map(|mut n| {
            n.left = leaf(2);
            n
        });
        assert_eq!(Solution::diameter_of_binary_tree(root), 1);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(Solution::diameter_of_binary_tree(None), 0);
    }

    #[test]
    fn single_node() {
        assert_eq!(Solution::diameter_of_binary_tree(leaf(1)), 0);
    }
}
