//! LeetCode Top Interview 150 — #111 Merge k Sorted Lists (Hard)
//!
//! Given an array of k sorted linked lists, merge them into one sorted
//! linked list.
//!
//! Example:
//!   Input: lists = [[1,4,5],[1,3,4],[2,6]]
//!   Output: [1,1,2,3,4,4,5,6]

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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut vals: Vec<i32> = lists.into_iter().flat_map(to_vec).collect();
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
    let lists = vec![
        from_vec(&[1, 4, 5]),
        from_vec(&[1, 3, 4]),
        from_vec(&[2, 6]),
    ];
    println!("{:?}", to_vec(Solution::merge_k_lists(lists)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let lists = vec![from_vec(&[1, 4, 5]), from_vec(&[1, 3, 4]), from_vec(&[2, 6])];
        assert_eq!(
            to_vec(Solution::merge_k_lists(lists)),
            vec![1, 1, 2, 3, 4, 4, 5, 6]
        );
    }

    #[test]
    fn example_2_empty_list_of_lists() {
        assert_eq!(
            to_vec(Solution::merge_k_lists(vec![])),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn example_3_single_empty_list() {
        assert_eq!(
            to_vec(Solution::merge_k_lists(vec![from_vec(&[])])),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn mixed_empty_and_nonempty() {
        let lists = vec![from_vec(&[]), from_vec(&[2, 1])];
        assert_eq!(to_vec(Solution::merge_k_lists(lists)), vec![1, 2]);
    }
}
