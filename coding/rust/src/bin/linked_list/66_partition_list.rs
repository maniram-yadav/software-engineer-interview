//! LeetCode Top Interview 150 — #66 Partition List (Medium)
//!
//! Given the head of a linked list and a value x, partition it so all
//! nodes less than x come before nodes >= x, preserving the relative
//! order within each partition. Solved by splitting into two lists
//! (less-than and greater-or-equal) then splicing them together.
//!
//! Example:
//!   Input: head = [1,4,3,2,5,2], x = 3
//!   Output: [1,2,2,4,3,5]

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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut less_dummy = Box::new(ListNode::new(0));
        let mut greater_dummy = Box::new(ListNode::new(0));
        let mut less_tail = &mut less_dummy;
        let mut greater_tail = &mut greater_dummy;

        let mut cur = head;
        while let Some(mut node) = cur {
            cur = node.next.take();
            if node.val < x {
                less_tail.next = Some(node);
                less_tail = less_tail.next.as_mut().unwrap();
            } else {
                greater_tail.next = Some(node);
                greater_tail = greater_tail.next.as_mut().unwrap();
            }
        }

        less_tail.next = greater_dummy.next;
        less_dummy.next
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
    let head = from_vec(&[1, 4, 3, 2, 5, 2]);
    let result = Solution::partition(head, 3);
    println!("{:?}", to_vec(result));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let head = from_vec(&[1, 4, 3, 2, 5, 2]);
        let result = Solution::partition(head, 3);
        assert_eq!(to_vec(result), vec![1, 2, 2, 4, 3, 5]);
    }

    #[test]
    fn example_2() {
        let head = from_vec(&[2, 1]);
        let result = Solution::partition(head, 2);
        assert_eq!(to_vec(result), vec![1, 2]);
    }

    #[test]
    fn all_less_than_x() {
        let head = from_vec(&[1, 2, 3]);
        let result = Solution::partition(head, 10);
        assert_eq!(to_vec(result), vec![1, 2, 3]);
    }

    #[test]
    fn all_greater_or_equal() {
        let head = from_vec(&[5, 6, 7]);
        let result = Solution::partition(head, 3);
        assert_eq!(to_vec(result), vec![5, 6, 7]);
    }

    #[test]
    fn empty_list() {
        let head = from_vec(&[]);
        let result = Solution::partition(head, 3);
        assert_eq!(to_vec(result), Vec::<i32>::new());
    }
}
