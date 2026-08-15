//! Grind 169 — LeetCode #206 Reverse Linked List (Easy)
//!
//! Given the head of a singly linked list, reverse the list and return
//! the new head. Solved iteratively, relinking each node to point at
//! the previously-processed node.
//!
//! Example:
//!   Input: head = [1,2,3,4,5]
//!   Output: [5,4,3,2,1]

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut cur = head;
        while let Some(mut node) = cur {
            cur = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

fn from_vec(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;
    for &v in vals {
        tail.next = Some(Box::new(ListNode::new(v)));
        tail = tail.next.as_mut().unwrap();
    }
    dummy.next
}

fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    while let Some(node) = head {
        result.push(node.val);
        head = node.next;
    }
    result
}

fn main() {
    let head = from_vec(&[1, 2, 3, 4, 5]);
    println!("{:?}", to_vec(Solution::reverse_list(head)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let head = from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(
            to_vec(Solution::reverse_list(head)),
            vec![5, 4, 3, 2, 1]
        );
    }

    #[test]
    fn example_2_two_nodes() {
        let head = from_vec(&[1, 2]);
        assert_eq!(to_vec(Solution::reverse_list(head)), vec![2, 1]);
    }

    #[test]
    fn example_3_empty() {
        let head = from_vec(&[]);
        assert_eq!(to_vec(Solution::reverse_list(head)), Vec::<i32>::new());
    }

    #[test]
    fn single_node() {
        let head = from_vec(&[1]);
        assert_eq!(to_vec(Solution::reverse_list(head)), vec![1]);
    }
}
