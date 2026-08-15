//! Grind 169 — LeetCode #261 Graph Valid Tree (Medium)
//!
//! Given n nodes and a list of undirected edges, determine if these
//! edges form a valid tree (connected and acyclic). A tree on n nodes
//! has exactly n-1 edges; union-find additionally confirms no cycles
//! (and thus, combined with the edge count, full connectivity).
//!
//! Example:
//!   Input: n = 5, edges = [[0,1],[0,2],[0,3],[1,4]]
//!   Output: true

struct Solution;

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        if n == 0 {
            return edges.is_empty();
        }
        if edges.len() != n - 1 {
            return false;
        }

        let mut parent: Vec<usize> = (0..n).collect();
        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        for e in edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
            if ra == rb {
                return false;
            }
            parent[ra] = rb;
        }
        true
    }
}

fn main() {
    let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]];
    println!("{}", Solution::valid_tree(5, edges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_valid() {
        let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]];
        assert_eq!(Solution::valid_tree(5, edges), true);
    }

    #[test]
    fn example_2_has_cycle() {
        let edges = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![1, 3],
            vec![1, 4],
        ];
        assert_eq!(Solution::valid_tree(5, edges), false);
    }

    #[test]
    fn disconnected_is_invalid() {
        let edges = vec![vec![0, 1]];
        assert_eq!(Solution::valid_tree(4, edges), false);
    }

    #[test]
    fn single_node_no_edges() {
        assert_eq!(Solution::valid_tree(1, vec![]), true);
    }
}
