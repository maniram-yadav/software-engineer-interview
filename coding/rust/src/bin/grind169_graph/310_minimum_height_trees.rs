//! Grind 169 — LeetCode #310 Minimum Height Trees (Medium)
//!
//! Given a tree with n nodes, return all roots that produce minimum
//! height trees (the "centroids"). Solved by repeatedly peeling off
//! leaves (topological layers from the outside in) until 1 or 2 nodes
//! remain — those are the centroids.
//!
//! Example:
//!   Input: n = 4, edges = [[1,0],[1,2],[1,3]]
//!   Output: [1]

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        if n == 1 {
            return vec![0];
        }
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut degree = vec![0i32; n];
        for e in &edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            adj[a].push(b);
            adj[b].push(a);
            degree[a] += 1;
            degree[b] += 1;
        }

        let mut leaves: VecDeque<usize> = (0..n).filter(|&i| degree[i] == 1).collect();
        let mut remaining = n;
        while remaining > 2 {
            let leaf_count = leaves.len();
            remaining -= leaf_count;
            for _ in 0..leaf_count {
                let leaf = leaves.pop_front().unwrap();
                for &next in &adj[leaf] {
                    degree[next] -= 1;
                    if degree[next] == 1 {
                        leaves.push_back(next);
                    }
                }
            }
        }

        leaves.into_iter().map(|x| x as i32).collect()
    }
}

fn main() {
    let edges = vec![vec![1, 0], vec![1, 2], vec![1, 3]];
    println!("{:?}", Solution::find_min_height_trees(4, edges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let edges = vec![vec![1, 0], vec![1, 2], vec![1, 3]];
        assert_eq!(Solution::find_min_height_trees(4, edges), vec![1]);
    }

    #[test]
    fn example_2_two_centroids() {
        let edges = vec![
            vec![3, 0],
            vec![3, 1],
            vec![3, 2],
            vec![3, 4],
            vec![5, 4],
        ];
        let mut result = Solution::find_min_height_trees(6, edges);
        result.sort();
        assert_eq!(result, vec![3, 4]);
    }

    #[test]
    fn single_node() {
        assert_eq!(Solution::find_min_height_trees(1, vec![]), vec![0]);
    }

    #[test]
    fn two_nodes() {
        let mut result = Solution::find_min_height_trees(2, vec![vec![0, 1]]);
        result.sort();
        assert_eq!(result, vec![0, 1]);
    }
}
