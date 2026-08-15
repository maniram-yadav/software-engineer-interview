//! Grind 169 — LeetCode #269 Alien Dictionary (Hard, Premium)
//!
//! Given a list of words sorted lexicographically according to an
//! unknown alien language's rules, derive a valid character ordering of
//! that alphabet. Solved by building a "comes before" edge for the first
//! differing character between each pair of adjacent words, then
//! topologically sorting (Kahn's algorithm, iterating neighbors in
//! sorted order for a deterministic result). A word that is a proper
//! prefix of an earlier word makes the input invalid.
//!
//! Example:
//!   Input: words = ["wrt","wrf","er","ett","rftt"]
//!   Output: "wertf"

use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut graph: HashMap<char, HashSet<char>> = HashMap::new();
        let mut indegree: HashMap<char, i32> = HashMap::new();
        for w in &words {
            for c in w.chars() {
                indegree.entry(c).or_insert(0);
                graph.entry(c).or_insert_with(HashSet::new);
            }
        }

        for pair in words.windows(2) {
            let c1: Vec<char> = pair[0].chars().collect();
            let c2: Vec<char> = pair[1].chars().collect();
            let min_len = c1.len().min(c2.len());
            let mut found_diff = false;
            for i in 0..min_len {
                if c1[i] != c2[i] {
                    if graph.get_mut(&c1[i]).unwrap().insert(c2[i]) {
                        *indegree.get_mut(&c2[i]).unwrap() += 1;
                    }
                    found_diff = true;
                    break;
                }
            }
            if !found_diff && c1.len() > c2.len() {
                return String::new();
            }
        }

        let mut queue_vec: Vec<char> = indegree
            .iter()
            .filter(|&(_, &d)| d == 0)
            .map(|(&c, _)| c)
            .collect();
        queue_vec.sort();
        let mut queue: VecDeque<char> = queue_vec.into();

        let mut result = String::new();
        while let Some(c) = queue.pop_front() {
            result.push(c);
            let mut next_chars: Vec<char> = graph[&c].iter().copied().collect();
            next_chars.sort();
            for nc in next_chars {
                let d = indegree.get_mut(&nc).unwrap();
                *d -= 1;
                if *d == 0 {
                    queue.push_back(nc);
                }
            }
        }

        if result.len() != indegree.len() {
            String::new()
        } else {
            result
        }
    }
}

fn v(strs: &[&str]) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{}",
        Solution::alien_order(v(&["wrt", "wrf", "er", "ett", "rftt"]))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::alien_order(v(&["wrt", "wrf", "er", "ett", "rftt"])),
            "wertf".to_string()
        );
    }

    #[test]
    fn example_2_simple_order() {
        let result = Solution::alien_order(v(&["z", "x"]));
        assert_eq!(result, "zx".to_string());
    }

    #[test]
    fn example_3_invalid_prefix_order() {
        assert_eq!(
            Solution::alien_order(v(&["z", "x", "z"])),
            String::new()
        );
    }

    #[test]
    fn single_word() {
        let result = Solution::alien_order(v(&["abc"]));
        let mut chars: Vec<char> = result.chars().collect();
        chars.sort();
        assert_eq!(chars, vec!['a', 'b', 'c']);
    }
}
