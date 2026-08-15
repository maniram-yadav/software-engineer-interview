//! LeetCode Top Interview 150 — #81 Lowest Common Ancestor of a Binary
//! Tree (Medium)
//!
//! Given a binary tree and two nodes p and q, find their lowest common
//! ancestor. LeetCode's actual signature identifies p/q by node
//! reference; since this file's tree uses plain `Box` ownership (no
//! external aliasing), p and q are identified by value instead
//! (values are assumed unique, as in the standard examples). Solved with
//! post-order recursion: a node is the LCA if p and q are found in
//! different subtrees (or the node itself is p or q and the other is
//! found below it).
//!
//! Example:
//!   Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
//!   Output: 3

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
    pub fn lowest_common_ancestor(root: &Option<Box<TreeNode>>, p: i32, q: i32) -> Option<i32> {
        match root {
            None => None,
            Some(n) => {
                if n.val == p || n.val == q {
                    return Some(n.val);
                }
                let left = Solution::lowest_common_ancestor(&n.left, p, q);
                let right = Solution::lowest_common_ancestor(&n.right, p, q);
                match (left, right) {
                    (Some(_), Some(_)) => Some(n.val),
                    (Some(l), None) => Some(l),
                    (None, Some(r)) => Some(r),
                    (None, None) => None,
                }
            }
        }
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn example_tree() -> Option<Box<TreeNode>> {
    node(
        3,
        node(5, leaf(6), node(2, leaf(7), leaf(4))),
        node(1, leaf(0), leaf(8)),
    )
}

fn main() {
    let root = example_tree();
    println!("{:?}", Solution::lowest_common_ancestor(&root, 5, 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = example_tree();
        assert_eq!(Solution::lowest_common_ancestor(&root, 5, 1), Some(3));
    }

    #[test]
    fn example_2_ancestor_is_descendant() {
        let root = example_tree();
        assert_eq!(Solution::lowest_common_ancestor(&root, 5, 4), Some(5));
    }

    #[test]
    fn example_3_two_node_tree() {
        let root = node(1, leaf(2), None);
        assert_eq!(Solution::lowest_common_ancestor(&root, 1, 2), Some(1));
    }

    #[test]
    fn root_is_lca_of_direct_children() {
        let root = node(1, leaf(2), leaf(3));
        assert_eq!(Solution::lowest_common_ancestor(&root, 2, 3), Some(1));
    }
}
