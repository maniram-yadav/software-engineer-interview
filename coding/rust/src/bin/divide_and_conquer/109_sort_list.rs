//! LeetCode Top Interview 150 — #109 Sort List (Medium)
//!
//! Given the head of a linked list, sort it in ascending order and
//! return it, in O(n log n) time.
//!
//! Example:
//!   Input: head = [4,2,1,3]
//!   Output: [1,2,3,4]

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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vals = to_vec(head);
        vals.sort_unstable();
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
    let head = from_vec(&[4, 2, 1, 3]);
    println!("{:?}", to_vec(Solution::sort_list(head)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let head = from_vec(&[4, 2, 1, 3]);
        assert_eq!(to_vec(Solution::sort_list(head)), vec![1, 2, 3, 4]);
    }

    #[test]
    fn example_2_with_negatives() {
        let head = from_vec(&[-1, 5, 3, 4, 0]);
        assert_eq!(
            to_vec(Solution::sort_list(head)),
            vec![-1, 0, 3, 4, 5]
        );
    }

    #[test]
    fn example_3_empty() {
        let head = from_vec(&[]);
        assert_eq!(to_vec(Solution::sort_list(head)), Vec::<i32>::new());
    }

    #[test]
    fn already_sorted() {
        let head = from_vec(&[1, 2, 3]);
        assert_eq!(to_vec(Solution::sort_list(head)), vec![1, 2, 3]);
    }
}
