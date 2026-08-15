//! Grind 169 — LeetCode #113 Path Sum II (Medium)
//!
//! Given the root of a binary tree and targetSum, return all root-to-leaf
//! paths where the sum of node values equals targetSum. DFS with
//! backtracking, appending the current path whenever a qualifying leaf
//! is reached.
//!
//! Example:
//!   Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
//!   Output: [[5,4,11,2],[5,8,4,5]]

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
    pub fn path_sum(root: Option<Box<TreeNode>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn dfs(
            node: &Option<Box<TreeNode>>,
            remaining: i32,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if let Some(n) = node {
                path.push(n.val);
                let remaining = remaining - n.val;
                if n.left.is_none() && n.right.is_none() && remaining == 0 {
                    result.push(path.clone());
                } else {
                    dfs(&n.left, remaining, path, result);
                    dfs(&n.right, remaining, path, result);
                }
                path.pop();
            }
        }
        let mut result = Vec::new();
        let mut path = Vec::new();
        dfs(&root, target_sum, &mut path, &mut result);
        result
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
        5,
        node(4, node(11, leaf(7), leaf(2)), None),
        node(8, node(13, None, None), node(4, leaf(5), leaf(1))),
    );
    println!("{:?}", Solution::path_sum(root, 22));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(
            5,
            node(4, node(11, leaf(7), leaf(2)), None),
            node(8, node(13, None, None), node(4, leaf(5), leaf(1))),
        );
        assert_eq!(
            Solution::path_sum(root, 22),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        );
    }

    #[test]
    fn example_2_no_matching_path() {
        let root = node(1, leaf(2), leaf(3));
        assert_eq!(Solution::path_sum(root, 5), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn example_3_empty_tree() {
        assert_eq!(Solution::path_sum(None, 0), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn single_node_matching() {
        assert_eq!(Solution::path_sum(leaf(5), 5), vec![vec![5]]);
    }
}
