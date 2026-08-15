//! LeetCode Top Interview 150 — #59 Merge Two Sorted Lists (Easy)
//!
//! Merge two sorted linked lists into one sorted list by splicing their
//! nodes.
//!
//! Example:
//!   Input: list1 = [1,2,4], list2 = [1,3,4]
//!   Output: [1,1,2,3,4,4]

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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(mut n1), Some(mut n2)) => {
                if n1.val <= n2.val {
                    n1.next = Solution::merge_two_lists(n1.next, Some(n2));
                    Some(n1)
                } else {
                    n2.next = Solution::merge_two_lists(Some(n1), n2.next);
                    Some(n2)
                }
            }
        }
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
    let list1 = from_vec(&[1, 2, 4]);
    let list2 = from_vec(&[1, 3, 4]);
    let merged = Solution::merge_two_lists(list1, list2);
    println!("{:?}", to_vec(merged));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let list1 = from_vec(&[1, 2, 4]);
        let list2 = from_vec(&[1, 3, 4]);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(to_vec(result), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn example_2_both_empty() {
        let result = Solution::merge_two_lists(from_vec(&[]), from_vec(&[]));
        assert_eq!(to_vec(result), Vec::<i32>::new());
    }

    #[test]
    fn example_3_one_empty() {
        let result = Solution::merge_two_lists(from_vec(&[]), from_vec(&[0]));
        assert_eq!(to_vec(result), vec![0]);
    }

    #[test]
    fn disjoint_ranges() {
        let list1 = from_vec(&[1, 2, 3]);
        let list2 = from_vec(&[4, 5, 6]);
        let result = Solution::merge_two_lists(list1, list2);
        assert_eq!(to_vec(result), vec![1, 2, 3, 4, 5, 6]);
    }
}
