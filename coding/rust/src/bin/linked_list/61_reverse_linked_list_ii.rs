//! LeetCode Top Interview 150 — #61 Reverse Linked List II (Medium)
//!
//! Given the head of a singly linked list and positions left/right,
//! reverse the nodes between those positions (1-indexed) and return the
//! head. Solved by converting to a Vec, reversing the target slice, and
//! rebuilding — simple and easy to verify correct.
//!
//! Example:
//!   Input: head = [1,2,3,4,5], left = 2, right = 4
//!   Output: [1,4,3,2,5]

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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut vals = to_vec(head);
        let (l, r) = ((left - 1) as usize, (right - 1) as usize);
        vals[l..=r].reverse();
        from_vec(&vals)
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
    let result = Solution::reverse_between(head, 2, 4);
    println!("{:?}", to_vec(result));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let head = from_vec(&[1, 2, 3, 4, 5]);
        let result = Solution::reverse_between(head, 2, 4);
        assert_eq!(to_vec(result), vec![1, 4, 3, 2, 5]);
    }

    #[test]
    fn example_2_single_node_range() {
        let head = from_vec(&[5]);
        let result = Solution::reverse_between(head, 1, 1);
        assert_eq!(to_vec(result), vec![5]);
    }

    #[test]
    fn reverse_whole_list() {
        let head = from_vec(&[1, 2, 3]);
        let result = Solution::reverse_between(head, 1, 3);
        assert_eq!(to_vec(result), vec![3, 2, 1]);
    }

    #[test]
    fn reverse_tail_segment() {
        let head = from_vec(&[1, 2, 3, 4]);
        let result = Solution::reverse_between(head, 3, 4);
        assert_eq!(to_vec(result), vec![1, 2, 4, 3]);
    }
}
