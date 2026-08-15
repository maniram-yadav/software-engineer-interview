//! LeetCode Top Interview 150 — #64 Remove Duplicates from Sorted List II (Medium)
//!
//! Given the head of a sorted linked list, delete all nodes that have
//! duplicate numbers, leaving only distinct numbers from the original
//! list. Solved with a dummy head and a single pass: whenever the current
//! node's value repeats, skip every node sharing that value entirely.
//!
//! Example:
//!   Input: head = [1,2,3,3,4,4,5]
//!   Output: [1,2,5]

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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut prev = &mut dummy;

        while let Some(node) = prev.next.take() {
            if node.next.is_some() && node.next.as_ref().unwrap().val == node.val {
                let dup_val = node.val;
                let mut rest = node.next;
                while let Some(n) = rest {
                    if n.val == dup_val {
                        rest = n.next;
                    } else {
                        rest = Some(n);
                        break;
                    }
                }
                prev.next = rest;
            } else {
                prev.next = Some(node);
                prev = prev.next.as_mut().unwrap();
            }
        }

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
    let head = from_vec(&[1, 2, 3, 3, 4, 4, 5]);
    let result = Solution::delete_duplicates(head);
    println!("{:?}", to_vec(result));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let head = from_vec(&[1, 2, 3, 3, 4, 4, 5]);
        let result = Solution::delete_duplicates(head);
        assert_eq!(to_vec(result), vec![1, 2, 5]);
    }

    #[test]
    fn example_2_leading_duplicates() {
        let head = from_vec(&[1, 1, 1, 2, 3]);
        let result = Solution::delete_duplicates(head);
        assert_eq!(to_vec(result), vec![2, 3]);
    }

    #[test]
    fn no_duplicates() {
        let head = from_vec(&[1, 2, 3]);
        let result = Solution::delete_duplicates(head);
        assert_eq!(to_vec(result), vec![1, 2, 3]);
    }

    #[test]
    fn all_duplicates() {
        let head = from_vec(&[1, 1]);
        let result = Solution::delete_duplicates(head);
        assert_eq!(to_vec(result), Vec::<i32>::new());
    }

    #[test]
    fn empty_list() {
        let head = from_vec(&[]);
        let result = Solution::delete_duplicates(head);
        assert_eq!(to_vec(result), Vec::<i32>::new());
    }
}
