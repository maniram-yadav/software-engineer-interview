//! LeetCode Top Interview 150 — #63 Remove Nth Node From End of List (Medium)
//!
//! Given the head of a linked list, remove the n-th node from the end and
//! return the head. Solved with a dummy head, a first pass to measure
//! length, then skipping to the node just before the removal point.
//!
//! Example:
//!   Input: head = [1,2,3,4,5], n = 2
//!   Output: [1,2,3,5]

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;

        let mut length = 0;
        let mut cur = dummy.next.as_ref();
        while let Some(node) = cur {
            length += 1;
            cur = node.next.as_ref();
        }

        let steps_before = length - n;
        let mut node = &mut dummy;
        for _ in 0..steps_before {
            node = node.next.as_mut().unwrap();
        }
        node.next = node.next.take().unwrap().next;

        dummy.next
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
    let result = Solution::remove_nth_from_end(head, 2);
    println!("{:?}", to_vec(result));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let head = from_vec(&[1, 2, 3, 4, 5]);
        let result = Solution::remove_nth_from_end(head, 2);
        assert_eq!(to_vec(result), vec![1, 2, 3, 5]);
    }

    #[test]
    fn example_2_single_node() {
        let head = from_vec(&[1]);
        let result = Solution::remove_nth_from_end(head, 1);
        assert_eq!(to_vec(result), Vec::<i32>::new());
    }

    #[test]
    fn example_3_remove_last() {
        let head = from_vec(&[1, 2]);
        let result = Solution::remove_nth_from_end(head, 1);
        assert_eq!(to_vec(result), vec![1]);
    }

    #[test]
    fn remove_head() {
        let head = from_vec(&[1, 2, 3]);
        let result = Solution::remove_nth_from_end(head, 3);
        assert_eq!(to_vec(result), vec![2, 3]);
    }
}
