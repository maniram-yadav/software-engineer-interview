//! LeetCode Top Interview 150 — #74 Populating Next Right Pointers in
//! Each Node II (Medium)
//!
//! Given a binary tree (not necessarily perfect) where each node has a
//! `next` pointer, populate each `next` pointer to point to its next
//! right node on the same level, or null. Since `next` points to a node
//! not owned by the current node (not a tree edge), nodes use
//! `Rc<RefCell<Node>>` instead of `Box`. Solved with a standard BFS,
//! linking each level's nodes left to right as they're dequeued.
//!
//! Example:
//!   Input: root = [1,2,3,4,5,null,7]
//!   Output: [1,#,2,3,#,4,5,7,#]   (# marks end of each level)

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;

struct Node {
    val: i32,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
    next: Option<NodeRef>,
}

impl Node {
    fn new(val: i32) -> NodeRef {
        Rc::new(RefCell::new(Node {
            val,
            left: None,
            right: None,
            next: None,
        }))
    }
}

struct Solution;

impl Solution {
    pub fn connect(root: Option<NodeRef>) -> Option<NodeRef> {
        let mut queue: VecDeque<NodeRef> = VecDeque::new();
        if let Some(r) = root.clone() {
            queue.push_back(r);
        }

        while !queue.is_empty() {
            let level_len = queue.len();
            let mut prev: Option<NodeRef> = None;
            for _ in 0..level_len {
                let cur = queue.pop_front().unwrap();
                if let Some(p) = &prev {
                    p.borrow_mut().next = Some(cur.clone());
                }
                prev = Some(cur.clone());

                if let Some(l) = cur.borrow().left.clone() {
                    queue.push_back(l);
                }
                if let Some(r) = cur.borrow().right.clone() {
                    queue.push_back(r);
                }
            }
        }

        root
    }
}

fn set_children(parent: &NodeRef, left: Option<NodeRef>, right: Option<NodeRef>) {
    parent.borrow_mut().left = left;
    parent.borrow_mut().right = right;
}

fn next_chain_vals(mut cur: Option<NodeRef>) -> Vec<i32> {
    let mut vals = Vec::new();
    while let Some(n) = cur {
        vals.push(n.borrow().val);
        cur = n.borrow().next.clone();
    }
    vals
}

fn main() {
    let n7 = Node::new(7);
    let n5 = Node::new(5);
    let n4 = Node::new(4);
    let n3 = Node::new(3);
    let n2 = Node::new(2);
    set_children(&n2, Some(n4.clone()), Some(n5.clone()));
    set_children(&n3, None, Some(n7.clone()));
    let root = Node::new(1);
    set_children(&root, Some(n2.clone()), Some(n3.clone()));

    Solution::connect(Some(root.clone()));
    println!("level 1: {:?}", next_chain_vals(Some(root)));
    println!("level 2: {:?}", next_chain_vals(Some(n2)));
    println!("level 3: {:?}", next_chain_vals(Some(n4)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n7 = Node::new(7);
        let n5 = Node::new(5);
        let n4 = Node::new(4);
        let n3 = Node::new(3);
        let n2 = Node::new(2);
        set_children(&n2, Some(n4.clone()), Some(n5.clone()));
        set_children(&n3, None, Some(n7.clone()));
        let root = Node::new(1);
        set_children(&root, Some(n2.clone()), Some(n3.clone()));

        Solution::connect(Some(root.clone()));

        assert_eq!(next_chain_vals(Some(root)), vec![1]);
        assert_eq!(next_chain_vals(Some(n2)), vec![2, 3]);
        assert_eq!(next_chain_vals(Some(n4)), vec![4, 5, 7]);
    }

    #[test]
    fn empty_tree() {
        assert!(Solution::connect(None).is_none());
    }

    #[test]
    fn single_node() {
        let root = Node::new(1);
        Solution::connect(Some(root.clone()));
        assert_eq!(next_chain_vals(Some(root)), vec![1]);
    }

    #[test]
    fn perfect_two_levels() {
        let n2 = Node::new(2);
        let n3 = Node::new(3);
        let root = Node::new(1);
        set_children(&root, Some(n2.clone()), Some(n3.clone()));
        Solution::connect(Some(root));
        assert_eq!(next_chain_vals(Some(n2)), vec![2, 3]);
    }
}
