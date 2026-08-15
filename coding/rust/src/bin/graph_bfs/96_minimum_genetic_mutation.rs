//! LeetCode Top Interview 150 — #96 Minimum Genetic Mutation (Medium)
//!
//! A gene string has 8 characters from {A,C,G,T}. Given startGene,
//! endGene, and a bank of valid intermediate genes (one character
//! differs per mutation), return the minimum number of mutations to get
//! from start to end using only bank genes, or -1. Solved with BFS over
//! single-character mutations, only stepping into genes present in the
//! bank.
//!
//! Example:
//!   Input: startGene = "AACCGGTT", endGene = "AACCGGTA", bank = ["AACCGGTA"]
//!   Output: 1

use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let bank_set: HashSet<String> = bank.into_iter().collect();
        if !bank_set.contains(&end_gene) {
            return -1;
        }

        let genes = ['A', 'C', 'G', 'T'];
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(start_gene.clone());
        let mut queue: VecDeque<(String, i32)> = VecDeque::new();
        queue.push_back((start_gene, 0));

        while let Some((gene, steps)) = queue.pop_front() {
            if gene == end_gene {
                return steps;
            }
            let chars: Vec<char> = gene.chars().collect();
            for i in 0..chars.len() {
                for &g in genes.iter() {
                    if g == chars[i] {
                        continue;
                    }
                    let mut next_chars = chars.clone();
                    next_chars[i] = g;
                    let next_gene: String = next_chars.into_iter().collect();
                    if bank_set.contains(&next_gene) && !visited.contains(&next_gene) {
                        visited.insert(next_gene.clone());
                        queue.push_back((next_gene, steps + 1));
                    }
                }
            }
        }

        -1
    }
}

fn main() {
    let result = Solution::min_mutation(
        "AACCGGTT".to_string(),
        "AACCGGTA".to_string(),
        vec!["AACCGGTA".to_string()],
    );
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::min_mutation(
                "AACCGGTT".to_string(),
                "AACCGGTA".to_string(),
                v(&["AACCGGTA"])
            ),
            1
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::min_mutation(
                "AACCGGTT".to_string(),
                "AAACGGTA".to_string(),
                v(&["AACCGGTA", "AACCGCTA", "AAACGGTA"])
            ),
            2
        );
    }

    #[test]
    fn end_not_in_bank_is_impossible() {
        assert_eq!(
            Solution::min_mutation("AACCGGTT".to_string(), "AACCGGTA".to_string(), v(&[])),
            -1
        );
    }

    #[test]
    fn start_equals_end() {
        assert_eq!(
            Solution::min_mutation(
                "AACCGGTT".to_string(),
                "AACCGGTT".to_string(),
                v(&["AACCGGTT"])
            ),
            0
        );
    }
}
