//! Grind 169 — LeetCode #437 Path Sum III (Medium)
//!
//! Given the root of a binary tree and an integer targetSum, return the
//! number of downward paths (not necessarily root-to-leaf) where the
//! sum equals targetSum. Solved with a running prefix-sum count (like
//! Subarray Sum Equals K, adapted to a tree): at each node, the number
//! of valid paths ending here equals how many ancestors had a prefix sum
//! of (current prefix sum - targetSum).
//!
//! Example:
//!   Input: root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8
//!   Output: 3

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Box<TreeNode>>, target_sum: i32) -> i32 {
        fn dfs(
            node: &Option<Box<TreeNode>>,
            sum: i64,
            target: i64,
            prefix: &mut HashMap<i64, i32>,
        ) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let sum = sum + n.val as i64;
                    let mut count = *prefix.get(&(sum - target)).unwrap_or(&0);
                    *prefix.entry(sum).or_insert(0) += 1;
                    count += dfs(&n.left, sum, target, prefix);
                    count += dfs(&n.right, sum, target, prefix);
                    *prefix.get_mut(&sum).unwrap() -= 1;
                    count
                }
            }
        }
        let mut prefix: HashMap<i64, i32> = HashMap::new();
        prefix.insert(0, 1);
        dfs(&root, 0, target_sum as i64, &mut prefix)
    }
}

fn leaf(val: i32) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode::new(val)))
}

fn node(val: i32, left: Option<Box<TreeNode>>, right: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    Some(Box::new(TreeNode { val, left, right }))
}

fn main() {
    let root = node(
        10,
        node(5, node(3, leaf(3), leaf(-2)), node(2, None, leaf(1))),
        node(-3, None, leaf(11)),
    );
    println!("{}", Solution::path_sum(root, 8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // Matches: [5,3], [-3,11], [5,2,1] — three downward paths summing to 8.
        let root = node(
            10,
            node(5, node(3, leaf(3), leaf(-2)), node(2, None, leaf(1))),
            node(-3, None, leaf(11)),
        );
        assert_eq!(Solution::path_sum(root, 8), 3);
    }

    #[test]
    fn example_2() {
        // root = 1 with children 2, 3; target 3.
        // Matches: [3] alone, and [1,2] (root through left child).
        let root = node(1, leaf(2), leaf(3));
        assert_eq!(Solution::path_sum(root, 3), 2);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(Solution::path_sum(None, 0), 0);
    }

    #[test]
    fn single_node_matching() {
        assert_eq!(Solution::path_sum(leaf(1), 1), 1);
    }
}
