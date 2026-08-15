//! Grind 169 — LeetCode #234 Palindrome Linked List (Easy)
//!
//! Given the head of a singly linked list, return true if it reads the
//! same forward and backward.
//!
//! Example:
//!   Input: head = [1,2,2,1]
//!   Output: true

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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let vals = to_vec(head);
        let rev: Vec<i32> = vals.iter().rev().copied().collect();
        vals == rev
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
    let head = from_vec(&[1, 2, 2, 1]);
    println!("{}", Solution::is_palindrome(head));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let head = from_vec(&[1, 2, 2, 1]);
        assert_eq!(Solution::is_palindrome(head), true);
    }

    #[test]
    fn example_2_not_palindrome() {
        let head = from_vec(&[1, 2]);
        assert_eq!(Solution::is_palindrome(head), false);
    }

    #[test]
    fn single_node() {
        let head = from_vec(&[1]);
        assert_eq!(Solution::is_palindrome(head), true);
    }

    #[test]
    fn odd_length_palindrome() {
        let head = from_vec(&[1, 2, 3, 2, 1]);
        assert_eq!(Solution::is_palindrome(head), true);
    }
}
