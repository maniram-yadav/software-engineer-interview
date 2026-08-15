//! LeetCode Top Interview 150 — #60 Copy List with Random Pointer (Medium)
//!
//! A linked list has each node containing an extra `random` pointer that
//! can point to any node or null. Create a deep copy of the list. Nodes
//! use `Rc<RefCell<Node>>` since `random` can point anywhere (shared,
//! non-tree references), which plain `Box` ownership can't express.
//! Solved in two passes with a HashMap from old-node pointer to new node:
//! first clone every node's value, then wire up `next`/`random`.
//!
//! Example:
//!   Input: head = [[7,null],[13,0],[11,4],[10,2],[1,0]]
//!   Output: deep copy with identical val/random structure but all new nodes

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;

struct Node {
    val: i32,
    next: Option<NodeRef>,
    random: Option<NodeRef>,
}

impl Node {
    fn new(val: i32) -> NodeRef {
        Rc::new(RefCell::new(Node {
            val,
            next: None,
            random: None,
        }))
    }
}

struct Solution;

impl Solution {
    pub fn copy_random_list(head: Option<NodeRef>) -> Option<NodeRef> {
        head.as_ref()?;

        let mut old_to_new: HashMap<*const RefCell<Node>, NodeRef> = HashMap::new();

        let mut cur = head.clone();
        while let Some(node) = cur {
            let ptr = Rc::as_ptr(&node);
            old_to_new.insert(ptr, Node::new(node.borrow().val));
            cur = node.borrow().next.clone();
        }

        let mut cur = head.clone();
        while let Some(node) = cur {
            let ptr = Rc::as_ptr(&node);
            let new_node = old_to_new.get(&ptr).unwrap().clone();

            let next_ptr = node.borrow().next.as_ref().map(Rc::as_ptr);
            if let Some(np) = next_ptr {
                new_node.borrow_mut().next = old_to_new.get(&np).cloned();
            }

            let random_ptr = node.borrow().random.as_ref().map(Rc::as_ptr);
            if let Some(rp) = random_ptr {
                new_node.borrow_mut().random = old_to_new.get(&rp).cloned();
            }

            cur = node.borrow().next.clone();
        }

        old_to_new.get(&Rc::as_ptr(&head.unwrap())).cloned()
    }
}

// Builds a list from (val, random_index) pairs, where random_index is an
// index into the same list (or None for a null random pointer).
fn build(pairs: &[(i32, Option<usize>)]) -> Option<NodeRef> {
    if pairs.is_empty() {
        return None;
    }
    let nodes: Vec<NodeRef> = pairs.iter().map(|&(v, _)| Node::new(v)).collect();
    for i in 0..nodes.len() {
        if i + 1 < nodes.len() {
            nodes[i].borrow_mut().next = Some(nodes[i + 1].clone());
        }
        if let Some(r) = pairs[i].1 {
            nodes[i].borrow_mut().random = Some(nodes[r].clone());
        }
    }
    Some(nodes[0].clone())
}

// Flattens a list into (val, random_val) pairs for easy comparison,
// following `next` pointers.
fn flatten(head: Option<NodeRef>) -> Vec<(i32, Option<i32>)> {
    let mut order = Vec::new();
    let mut cur = head;
    while let Some(node) = cur {
        order.push(node.clone());
        cur = node.borrow().next.clone();
    }
    order
        .iter()
        .map(|node| {
            let val = node.borrow().val;
            let random_val = node.borrow().random.as_ref().map(|r| r.borrow().val);
            (val, random_val)
        })
        .collect()
}

fn main() {
    let head = build(&[(7, None), (13, Some(0)), (11, Some(4)), (10, Some(2)), (1, Some(0))]);
    let copy = Solution::copy_random_list(head);
    println!("{:?}", flatten(copy));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let head = build(&[
            (7, None),
            (13, Some(0)),
            (11, Some(4)),
            (10, Some(2)),
            (1, Some(0)),
        ]);
        let copy = Solution::copy_random_list(head);
        assert_eq!(
            flatten(copy),
            vec![
                (7, None),
                (13, Some(7)),
                (11, Some(1)),
                (10, Some(11)),
                (1, Some(7))
            ]
        );
    }

    #[test]
    fn example_2_two_nodes() {
        let head = build(&[(1, Some(1)), (2, Some(1))]);
        let copy = Solution::copy_random_list(head);
        assert_eq!(flatten(copy), vec![(1, Some(2)), (2, Some(2))]);
    }

    #[test]
    fn empty_list() {
        assert_eq!(Solution::copy_random_list(None), None);
    }

    #[test]
    fn self_referencing_random() {
        let head = build(&[(5, Some(0))]);
        assert_eq!(flatten(Solution::copy_random_list(head)), vec![(5, Some(5))]);
    }

    #[test]
    fn returns_a_true_deep_copy() {
        let head = build(&[(1, None), (2, None)]);
        let original_head_ptr = Rc::as_ptr(head.as_ref().unwrap());
        let copy = Solution::copy_random_list(head);
        let copy_head_ptr = Rc::as_ptr(copy.as_ref().unwrap());
        assert_ne!(original_head_ptr, copy_head_ptr);
    }
}
