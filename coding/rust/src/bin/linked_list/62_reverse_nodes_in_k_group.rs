//! LeetCode Top Interview 150 — #62 Reverse Nodes in k-Group (Hard)
//!
//! Given a linked list, reverse the nodes of the list k at a time and
//! return the modified list. If the remaining nodes are fewer than k,
//! leave them as is. Solved by converting to a Vec, reversing each
//! complete group of k in place, and rebuilding.
//!
//! Example:
//!   Input: head = [1,2,3,4,5], k = 2
//!   Output: [2,1,4,3,5]

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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut vals = to_vec(head);
        let k = k as usize;
        let n = vals.len();
        let mut i = 0;
        while i + k <= n {
            vals[i..i + k].reverse();
            i += k;
        }
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
    let result = Solution::reverse_k_group(head, 2);
    println!("{:?}", to_vec(result));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_k_two() {
        let head = from_vec(&[1, 2, 3, 4, 5]);
        let result = Solution::reverse_k_group(head, 2);
        assert_eq!(to_vec(result), vec![2, 1, 4, 3, 5]);
    }

    #[test]
    fn example_2_k_three() {
        let head = from_vec(&[1, 2, 3, 4, 5]);
        let result = Solution::reverse_k_group(head, 3);
        assert_eq!(to_vec(result), vec![3, 2, 1, 4, 5]);
    }

    #[test]
    fn k_equals_length() {
        let head = from_vec(&[1, 2, 3, 4]);
        let result = Solution::reverse_k_group(head, 4);
        assert_eq!(to_vec(result), vec![4, 3, 2, 1]);
    }

    #[test]
    fn k_one_is_noop() {
        let head = from_vec(&[1, 2, 3]);
        let result = Solution::reverse_k_group(head, 1);
        assert_eq!(to_vec(result), vec![1, 2, 3]);
    }
}
