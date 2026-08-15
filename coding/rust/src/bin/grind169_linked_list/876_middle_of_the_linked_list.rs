//! Grind 169 — LeetCode #876 Middle of the Linked List (Easy)
//!
//! Given the head of a singly linked list, return the middle node (if
//! two middle nodes, return the second one).
//!
//! Example:
//!   Input: head = [1,2,3,4,5]
//!   Output: [3,4,5]

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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let vals = to_vec(head);
        let mid = vals.len() / 2;
        from_vec(&vals[mid..])
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
    println!("{:?}", to_vec(Solution::middle_node(head)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_odd_length() {
        let head = from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(
            to_vec(Solution::middle_node(head)),
            vec![3, 4, 5]
        );
    }

    #[test]
    fn example_2_even_length_returns_second_middle() {
        let head = from_vec(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(
            to_vec(Solution::middle_node(head)),
            vec![4, 5, 6]
        );
    }

    #[test]
    fn single_node() {
        let head = from_vec(&[1]);
        assert_eq!(to_vec(Solution::middle_node(head)), vec![1]);
    }

    #[test]
    fn two_nodes() {
        let head = from_vec(&[1, 2]);
        assert_eq!(to_vec(Solution::middle_node(head)), vec![2]);
    }
}
