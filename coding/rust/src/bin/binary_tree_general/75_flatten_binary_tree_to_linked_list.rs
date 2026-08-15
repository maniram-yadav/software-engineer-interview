//! LeetCode Top Interview 150 — #75 Flatten Binary Tree to Linked List (Medium)
//!
//! Given the root of a binary tree, flatten it in place into a "linked
//! list" following preorder traversal, using the `right` child pointers
//! only. Solved iteratively, O(1) extra space: for each node with a left
//! child, splice the left subtree in between the node and its right
//! subtree (attached at the left subtree's rightmost node).
//!
//! Example:
//!   Input: root = [1,2,5,3,4,null,6]
//!   Output: [1,null,2,null,3,null,4,null,5,null,6]

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
    pub fn flatten(root: &mut Option<Box<TreeNode>>) {
        let mut cur = root;
        while let Some(n) = cur {
            if let Some(mut left) = n.left.take() {
                let mut rightmost = &mut left;
                while rightmost.right.is_some() {
                    rightmost = rightmost.right.as_mut().unwrap();
                }
                rightmost.right = n.right.take();
                n.right = Some(left);
            }
            cur = &mut n.right;
        }
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

// Flattened trees are a right-only chain; collect vals by following `right`.
fn right_chain_vals(mut cur: &Option<Box<TreeNode>>) -> Vec<i32> {
    let mut vals = Vec::new();
    while let Some(n) = cur {
        vals.push(n.val);
        assert!(n.left.is_none(), "flattened tree must have no left children");
        cur = &n.right;
    }
    vals
}

fn main() {
    let mut root = node(
        1,
        node(2, leaf(3), leaf(4)),
        node(5, None, leaf(6)),
    );
    Solution::flatten(&mut root);
    println!("{:?}", right_chain_vals(&root));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut root = node(1, node(2, leaf(3), leaf(4)), node(5, None, leaf(6)));
        Solution::flatten(&mut root);
        assert_eq!(right_chain_vals(&root), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn example_2_empty() {
        let mut root: Option<Box<TreeNode>> = None;
        Solution::flatten(&mut root);
        assert_eq!(right_chain_vals(&root), Vec::<i32>::new());
    }

    #[test]
    fn example_3_single_node() {
        let mut root = leaf(0);
        Solution::flatten(&mut root);
        assert_eq!(right_chain_vals(&root), vec![0]);
    }

    #[test]
    fn already_right_skewed() {
        let mut root = node(1, None, node(2, None, leaf(3)));
        Solution::flatten(&mut root);
        assert_eq!(right_chain_vals(&root), vec![1, 2, 3]);
    }

    #[test]
    fn left_skewed_gets_flattened() {
        let mut root = node(1, node(2, leaf(3), None), None);
        Solution::flatten(&mut root);
        assert_eq!(right_chain_vals(&root), vec![1, 2, 3]);
    }
}
