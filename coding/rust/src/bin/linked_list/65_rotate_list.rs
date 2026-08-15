//! LeetCode Top Interview 150 — #65 Rotate List (Medium)
//!
//! Given the head of a linked list, rotate the list to the right by k
//! places.
//!
//! Example:
//!   Input: head = [1,2,3,4,5], k = 2
//!   Output: [4,5,1,2,3]

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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let vals = to_vec(head);
        let n = vals.len();
        if n == 0 {
            return None;
        }
        let k = (k as usize) % n;
        if k == 0 {
            return from_vec(&vals);
        }
        let mut rotated = Vec::with_capacity(n);
        rotated.extend_from_slice(&vals[n - k..]);
        rotated.extend_from_slice(&vals[..n - k]);
        from_vec(&rotated)
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
    let result = Solution::rotate_right(head, 2);
    println!("{:?}", to_vec(result));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let head = from_vec(&[1, 2, 3, 4, 5]);
        let result = Solution::rotate_right(head, 2);
        assert_eq!(to_vec(result), vec![4, 5, 1, 2, 3]);
    }

    #[test]
    fn example_2_k_larger_than_length() {
        let head = from_vec(&[0, 1, 2]);
        let result = Solution::rotate_right(head, 4);
        assert_eq!(to_vec(result), vec![2, 0, 1]);
    }

    #[test]
    fn k_zero_is_noop() {
        let head = from_vec(&[1, 2, 3]);
        let result = Solution::rotate_right(head, 0);
        assert_eq!(to_vec(result), vec![1, 2, 3]);
    }

    #[test]
    fn k_multiple_of_length_is_noop() {
        let head = from_vec(&[1, 2, 3]);
        let result = Solution::rotate_right(head, 3);
        assert_eq!(to_vec(result), vec![1, 2, 3]);
    }

    #[test]
    fn empty_list() {
        let head = from_vec(&[]);
        let result = Solution::rotate_right(head, 5);
        assert_eq!(to_vec(result), Vec::<i32>::new());
    }
}
