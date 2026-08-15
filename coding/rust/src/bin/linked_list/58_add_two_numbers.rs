//! LeetCode Top Interview 150 — #58 Add Two Numbers (Medium)
//!
//! Two non-empty linked lists represent non-negative integers in reverse
//! digit order. Add the two numbers and return the sum as a linked list
//! in the same format.
//!
//! Example:
//!   Input: l1 = [2,4,3], l2 = [5,6,4]
//!   Output: [7,0,8]   (342 + 465 = 807)

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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }
            carry = sum / 10;
            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();
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
    let l1 = from_vec(&[2, 4, 3]);
    let l2 = from_vec(&[5, 6, 4]);
    let sum = Solution::add_two_numbers(l1, l2);
    println!("{:?}", to_vec(sum));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let l1 = from_vec(&[2, 4, 3]);
        let l2 = from_vec(&[5, 6, 4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(to_vec(result), vec![7, 0, 8]);
    }

    #[test]
    fn example_2_both_zero() {
        let l1 = from_vec(&[0]);
        let l2 = from_vec(&[0]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(to_vec(result), vec![0]);
    }

    #[test]
    fn example_3_carry_propagation() {
        let l1 = from_vec(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = from_vec(&[9, 9, 9, 9]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(to_vec(result), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }

    #[test]
    fn different_lengths() {
        let l1 = from_vec(&[1]);
        let l2 = from_vec(&[9, 9]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(to_vec(result), vec![0, 0, 1]);
    }
}
