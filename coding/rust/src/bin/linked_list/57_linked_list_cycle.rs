//! LeetCode Top Interview 150 — #57 Linked List Cycle (Easy)
//!
//! Given the head of a linked list, determine if it has a cycle, using
//! O(1) extra space (Floyd's tortoise-and-hare). A cyclic list can't be
//! represented with plain `Box` ownership (a cycle needs shared
//! ownership), so nodes here use `Rc<RefCell<ListNode>>`.
//!
//! Example:
//!   Input: head = 3 -> 2 -> 0 -> -4 -> (back to node with value 2)
//!   Output: true

use std::cell::RefCell;
use std::rc::Rc;

type NodeRef = Rc<RefCell<ListNode>>;

struct ListNode {
    #[allow(dead_code)]
    val: i32,
    next: Option<NodeRef>,
}

impl ListNode {
    fn new(val: i32) -> NodeRef {
        Rc::new(RefCell::new(ListNode { val, next: None }))
    }
}

struct Solution;

impl Solution {
    pub fn has_cycle(head: Option<NodeRef>) -> bool {
        let mut slow = head.clone();
        let mut fast = head;

        loop {
            fast = match fast {
                Some(node) => node.borrow().next.clone(),
                None => return false,
            };
            fast = match fast {
                Some(node) => node.borrow().next.clone(),
                None => return false,
            };
            slow = match slow {
                Some(node) => node.borrow().next.clone(),
                None => return false,
            };

            if let (Some(s), Some(f)) = (&slow, &fast) {
                if Rc::ptr_eq(s, f) {
                    return true;
                }
            }
        }
    }
}

// Builds vals[0] -> vals[1] -> ... -> vals[n-1], and if `pos` is Some(i),
// makes the last node point back to vals[i] to form a cycle.
fn build_list(vals: &[i32], pos: Option<usize>) -> Option<NodeRef> {
    if vals.is_empty() {
        return None;
    }
    let nodes: Vec<NodeRef> = vals.iter().map(|&v| ListNode::new(v)).collect();
    for i in 0..nodes.len() - 1 {
        nodes[i].borrow_mut().next = Some(nodes[i + 1].clone());
    }
    if let Some(p) = pos {
        nodes.last().unwrap().borrow_mut().next = Some(nodes[p].clone());
    }
    Some(nodes[0].clone())
}

fn main() {
    let head = build_list(&[3, 2, 0, -4], Some(1));
    println!("has cycle: {}", Solution::has_cycle(head));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_cycle_at_index_1() {
        let head = build_list(&[3, 2, 0, -4], Some(1));
        assert_eq!(Solution::has_cycle(head), true);
    }

    #[test]
    fn example_2_self_loop() {
        let head = build_list(&[1, 2], Some(0));
        assert_eq!(Solution::has_cycle(head), true);
    }

    #[test]
    fn example_3_no_cycle_single_node() {
        let head = build_list(&[1], None);
        assert_eq!(Solution::has_cycle(head), false);
    }

    #[test]
    fn no_cycle_multiple_nodes() {
        let head = build_list(&[1, 2, 3, 4, 5], None);
        assert_eq!(Solution::has_cycle(head), false);
    }

    #[test]
    fn empty_list() {
        let head = build_list(&[], None);
        assert_eq!(Solution::has_cycle(head), false);
    }
}
