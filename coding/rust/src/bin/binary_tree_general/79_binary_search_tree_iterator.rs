//! LeetCode Top Interview 150 — #79 Binary Search Tree Iterator (Medium)
//!
//! Design an iterator over a BST that returns the next smallest number in
//! order via next(), with has_next(), both amortized O(1). Solved with an
//! explicit stack holding the path of "not yet visited" left ancestors.
//!
//! Example:
//!   BSTIterator it = new BSTIterator(root); // root = [7,3,15,null,null,9,20]
//!   it.next();    // 3
//!   it.next();    // 7
//!   it.hasNext(); // true
//!   it.next();    // 9

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

struct BSTIterator {
    stack: Vec<Box<TreeNode>>,
}

impl BSTIterator {
    fn new(root: Option<Box<TreeNode>>) -> Self {
        let mut it = BSTIterator { stack: Vec::new() };
        it.push_left(root);
        it
    }

    fn push_left(&mut self, mut node: Option<Box<TreeNode>>) {
        while let Some(mut n) = node {
            let left = n.left.take();
            self.stack.push(n);
            node = left;
        }
    }

    fn next(&mut self) -> i32 {
        let mut n = self.stack.pop().unwrap();
        let val = n.val;
        let right = n.right.take();
        self.push_left(right);
        val
    }

    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let root = node(7, leaf(3), node(15, leaf(9), leaf(20)));
    let mut it = BSTIterator::new(root);
    while it.has_next() {
        print!("{} ", it.next());
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(7, leaf(3), node(15, leaf(9), leaf(20)));
        let mut it = BSTIterator::new(root);
        assert_eq!(it.next(), 3);
        assert_eq!(it.next(), 7);
        assert_eq!(it.has_next(), true);
        assert_eq!(it.next(), 9);
        assert_eq!(it.has_next(), true);
        assert_eq!(it.next(), 15);
        assert_eq!(it.has_next(), true);
        assert_eq!(it.next(), 20);
        assert_eq!(it.has_next(), false);
    }

    #[test]
    fn single_node() {
        let mut it = BSTIterator::new(leaf(1));
        assert_eq!(it.has_next(), true);
        assert_eq!(it.next(), 1);
        assert_eq!(it.has_next(), false);
    }

    #[test]
    fn left_skewed_yields_ascending_order() {
        let root = node(3, node(2, leaf(1), None), None);
        let mut it = BSTIterator::new(root);
        let mut vals = Vec::new();
        while it.has_next() {
            vals.push(it.next());
        }
        assert_eq!(vals, vec![1, 2, 3]);
    }
}
