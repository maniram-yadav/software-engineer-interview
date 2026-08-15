//! Grind 169 — LeetCode #323 Number of Connected Components in an
//! Undirected Graph (Medium)
//!
//! Given n nodes and a list of undirected edges, return the number of
//! connected components. Union-find: start with n components, and each
//! edge that unites two previously-separate roots reduces the count by
//! one.
//!
//! Example:
//!   Input: n = 5, edges = [[0,1],[1,2],[3,4]]
//!   Output: 2

struct Solution;

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut parent: Vec<usize> = (0..n).collect();
        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        let mut count = n as i32;
        for e in edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            let (ra, rb) = (find(&mut parent, a), find(&mut parent, b));
            if ra != rb {
                parent[ra] = rb;
                count -= 1;
            }
        }
        count
    }
}

fn main() {
    let edges = vec![vec![0, 1], vec![1, 2], vec![3, 4]];
    println!("{}", Solution::count_components(5, edges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::count_components(5, edges), 2);
    }

    #[test]
    fn example_2_fully_connected() {
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(Solution::count_components(5, edges), 1);
    }

    #[test]
    fn no_edges() {
        assert_eq!(Solution::count_components(4, vec![]), 4);
    }

    #[test]
    fn redundant_edge_still_one_component() {
        let edges = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(Solution::count_components(2, edges), 1);
    }
}
