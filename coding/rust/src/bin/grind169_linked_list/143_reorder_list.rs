//! Grind 169 — LeetCode #143 Reorder List (Medium)
//!
//! Given the head of a linked list L0->L1->...->Ln, reorder it in place
//! to L0->Ln->L1->Ln-1->L2->Ln-2->... Solved by collecting values, then
//! alternately taking from the front and back of the collected list.
//!
//! Example:
//!   Input: head = [1,2,3,4]
//!   Output: [1,4,2,3]

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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let vals = to_vec(head.take());
        let n = vals.len() as i32;
        let mut result = Vec::with_capacity(vals.len());
        let (mut l, mut r) = (0i32, n - 1);
        let mut turn_left = true;

        for _ in 0..n {
            if turn_left {
                result.push(vals[l as usize]);
                l += 1;
            } else {
                result.push(vals[r as usize]);
                r -= 1;
            }
            turn_left = !turn_left;
        }

        *head = from_vec(&result);
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
    let mut head = from_vec(&[1, 2, 3, 4]);
    Solution::reorder_list(&mut head);
    println!("{:?}", to_vec(head));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut head = from_vec(&[1, 2, 3, 4]);
        Solution::reorder_list(&mut head);
        assert_eq!(to_vec(head), vec![1, 4, 2, 3]);
    }

    #[test]
    fn example_2_odd_length() {
        let mut head = from_vec(&[1, 2, 3, 4, 5]);
        Solution::reorder_list(&mut head);
        assert_eq!(to_vec(head), vec![1, 5, 2, 4, 3]);
    }

    #[test]
    fn single_node() {
        let mut head = from_vec(&[1]);
        Solution::reorder_list(&mut head);
        assert_eq!(to_vec(head), vec![1]);
    }

    #[test]
    fn two_nodes() {
        let mut head = from_vec(&[1, 2]);
        Solution::reorder_list(&mut head);
        assert_eq!(to_vec(head), vec![1, 2]);
    }
}
