//! LeetCode Top Interview 150 — #92 Evaluate Division (Medium)
//!
//! Given equations like a / b = 2.0 and query pairs, evaluate each query
//! using the equations as a weighted graph. Return -1.0 if undeterminable.
//! Solved by building a weighted adjacency list (edge weight w for a->b
//! means a/b=w, plus the inverse edge 1/w for b->a), then BFS per query
//! accumulating the product of edge weights along the path.
//!
//! Example:
//!   Input: equations = [["a","b"],["b","c"]], values = [2.0,3.0],
//!          queries = [["a","c"],["b","a"],["a","e"]]
//!   Output: [6.0,0.5,-1.0]

use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();
        for (eq, &val) in equations.iter().zip(values.iter()) {
            let (a, b) = (eq[0].clone(), eq[1].clone());
            graph.entry(a.clone()).or_insert_with(Vec::new).push((b.clone(), val));
            graph.entry(b).or_insert_with(Vec::new).push((a, 1.0 / val));
        }

        queries
            .iter()
            .map(|q| {
                let (src, dst) = (&q[0], &q[1]);
                if !graph.contains_key(src) || !graph.contains_key(dst) {
                    return -1.0;
                }
                if src == dst {
                    return 1.0;
                }

                let mut visited: HashSet<String> = HashSet::new();
                let mut queue: VecDeque<(String, f64)> = VecDeque::new();
                visited.insert(src.clone());
                queue.push_back((src.clone(), 1.0));

                while let Some((cur, acc)) = queue.pop_front() {
                    if &cur == dst {
                        return acc;
                    }
                    if let Some(neighbors) = graph.get(&cur) {
                        for (next, weight) in neighbors {
                            if !visited.contains(next) {
                                visited.insert(next.clone());
                                queue.push_back((next.clone(), acc * weight));
                            }
                        }
                    }
                }
                -1.0
            })
            .collect()
    }
}

fn eqs(pairs: &[(&str, &str)]) -> Vec<Vec<String>> {
    pairs
        .iter()
        .map(|&(a, b)| vec![a.to_string(), b.to_string()])
        .collect()
}

fn approx_eq(a: &[f64], b: &[f64]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| (x - y).abs() < 1e-6)
}

fn main() {
    let result = Solution::calc_equation(
        eqs(&[("a", "b"), ("b", "c")]),
        vec![2.0, 3.0],
        eqs(&[("a", "c"), ("b", "a"), ("a", "e")]),
    );
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = Solution::calc_equation(
            eqs(&[("a", "b"), ("b", "c")]),
            vec![2.0, 3.0],
            eqs(&[("a", "c"), ("b", "a"), ("a", "e"), ("a", "a"), ("x", "x")]),
        );
        assert!(approx_eq(&result, &[6.0, 0.5, -1.0, 1.0, -1.0]));
    }

    #[test]
    fn example_2_multi_hop() {
        let result = Solution::calc_equation(
            eqs(&[("a", "b"), ("b", "c"), ("bc", "cd")]),
            vec![1.5, 2.5, 5.0],
            eqs(&[("a", "c"), ("c", "b"), ("bc", "cd"), ("cd", "bc")]),
        );
        assert!(approx_eq(&result, &[3.75, 0.4, 5.0, 0.2]));
    }

    #[test]
    fn example_3_disconnected() {
        let result = Solution::calc_equation(
            eqs(&[("a", "b")]),
            vec![0.5],
            eqs(&[("a", "b"), ("b", "a"), ("a", "c"), ("x", "y")]),
        );
        assert!(approx_eq(&result, &[0.5, 2.0, -1.0, -1.0]));
    }
}
