//! Grind 169 — LeetCode #328 Odd Even Linked List (Medium)
//!
//! Given the head of a singly linked list, group all nodes at odd
//! indices together followed by nodes at even indices (1-indexed),
//! preserving relative order within each group.
//!
//! Example:
//!   Input: head = [1,2,3,4,5]
//!   Output: [1,3,5,2,4]

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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let vals = to_vec(head);
        let mut odds = Vec::new();
        let mut evens = Vec::new();
        for (i, v) in vals.into_iter().enumerate() {
            if i % 2 == 0 {
                odds.push(v);
            } else {
                evens.push(v);
            }
        }
        odds.extend(evens);
        from_vec(&odds)
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
    println!("{:?}", to_vec(Solution::odd_even_list(head)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let head = from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(
            to_vec(Solution::odd_even_list(head)),
            vec![1, 3, 5, 2, 4]
        );
    }

    #[test]
    fn example_2() {
        let head = from_vec(&[2, 1, 3, 5, 6, 4, 7]);
        assert_eq!(
            to_vec(Solution::odd_even_list(head)),
            vec![2, 3, 6, 7, 1, 5, 4]
        );
    }

    #[test]
    fn empty_list() {
        let head = from_vec(&[]);
        assert_eq!(to_vec(Solution::odd_even_list(head)), Vec::<i32>::new());
    }

    #[test]
    fn single_node() {
        let head = from_vec(&[1]);
        assert_eq!(to_vec(Solution::odd_even_list(head)), vec![1]);
    }
}
