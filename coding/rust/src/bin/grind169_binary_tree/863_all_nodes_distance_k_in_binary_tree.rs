//! Grind 169 — LeetCode #863 All Nodes Distance K in Binary Tree (Medium)
//!
//! Given the root of a binary tree, a target node, and an integer k,
//! return the values of all nodes that are exactly distance k from the
//! target. Since this file's tree uses plain `Box` (no parent pointers),
//! `target` is identified by value (values assumed unique). Solved by
//! first building a value->parent map via one DFS, then BFS outward
//! from the target treating the tree as an undirected graph (via
//! left/right/parent edges).
//!
//! Example:
//!   Input: root = [3,5,1,6,2,0,8,null,null,7,4], target = 5, k = 2
//!   Output: [7,4,1]

use std::collections::{HashMap, HashSet, VecDeque};

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
    pub fn distance_k(root: Option<Box<TreeNode>>, target: i32, k: i32) -> Vec<i32> {
        let mut parent: HashMap<i32, i32> = HashMap::new();
        let mut children: HashMap<i32, Vec<i32>> = HashMap::new();

        fn build(
            node: &Option<Box<TreeNode>>,
            par: Option<i32>,
            parent: &mut HashMap<i32, i32>,
            children: &mut HashMap<i32, Vec<i32>>,
        ) {
            if let Some(n) = node {
                if let Some(p) = par {
                    parent.insert(n.val, p);
                }
                let mut kids = Vec::new();
                if let Some(l) = &n.left {
                    kids.push(l.val);
                }
                if let Some(r) = &n.right {
                    kids.push(r.val);
                }
                children.insert(n.val, kids);
                build(&n.left, Some(n.val), parent, children);
                build(&n.right, Some(n.val), parent, children);
            }
        }
        build(&root, None, &mut parent, &mut children);

        let mut visited: HashSet<i32> = HashSet::new();
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        queue.push_back((target, 0));
        visited.insert(target);
        let mut result = Vec::new();

        while let Some((val, dist)) = queue.pop_front() {
            if dist == k {
                result.push(val);
                continue;
            }
            let mut neighbors = children.get(&val).cloned().unwrap_or_default();
            if let Some(&p) = parent.get(&val) {
                neighbors.push(p);
            }
            for nb in neighbors {
                if !visited.contains(&nb) {
                    visited.insert(nb);
                    queue.push_back((nb, dist + 1));
                }
            }
        }
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
        3,
        node(5, leaf(6), node(2, leaf(7), leaf(4))),
        node(1, leaf(0), leaf(8)),
    );
    let mut result = Solution::distance_k(root, 5, 2);
    result.sort();
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let root = node(
            3,
            node(5, leaf(6), node(2, leaf(7), leaf(4))),
            node(1, leaf(0), leaf(8)),
        );
        let mut result = Solution::distance_k(root, 5, 2);
        result.sort();
        assert_eq!(result, vec![1, 4, 7]);
    }

    #[test]
    fn distance_zero_is_target_itself() {
        let root = node(1, leaf(2), leaf(3));
        assert_eq!(Solution::distance_k(root, 1, 0), vec![1]);
    }

    #[test]
    fn single_node() {
        assert_eq!(Solution::distance_k(leaf(1), 1, 0), vec![1]);
    }

    #[test]
    fn distance_beyond_tree_size_is_empty() {
        let root = node(1, leaf(2), None);
        assert_eq!(
            Solution::distance_k(root, 1, 5),
            Vec::<i32>::new()
        );
    }
}
