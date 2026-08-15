//! Grind 169 — LeetCode #692 Top K Frequent Words (Medium)
//!
//! Given an array of strings words and an integer k, return the k most
//! frequent words, sorted by frequency (descending) then lexicographically
//! for ties.
//!
//! Example:
//!   Input: words = ["i","love","leetcode","i","love","coding"], k = 2
//!   Output: ["i","love"]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut counts: HashMap<String, i32> = HashMap::new();
        for w in words {
            *counts.entry(w).or_insert(0) += 1;
        }
        let mut entries: Vec<(String, i32)> = counts.into_iter().collect();
        entries.sort_by(|a, b| {
            if a.1 != b.1 {
                b.1.cmp(&a.1)
            } else {
                a.0.cmp(&b.0)
            }
        });
        entries.into_iter().take(k as usize).map(|(w, _)| w).collect()
    }
}

fn v(strs: &[&str]) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
}

fn main() {
    let words = v(&["i", "love", "leetcode", "i", "love", "coding"]);
    println!("{:?}", Solution::top_k_frequent(words, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let words = v(&["i", "love", "leetcode", "i", "love", "coding"]);
        assert_eq!(Solution::top_k_frequent(words, 2), v(&["i", "love"]));
    }

    #[test]
    fn example_2_tie_broken_lexicographically() {
        let words = v(&[
            "the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is",
        ]);
        assert_eq!(
            Solution::top_k_frequent(words, 4),
            v(&["the", "is", "sunny", "day"])
        );
    }

    #[test]
    fn k_equals_unique_word_count() {
        let words = v(&["a", "b", "c"]);
        let mut result = Solution::top_k_frequent(words, 3);
        result.sort();
        assert_eq!(result, v(&["a", "b", "c"]));
    }

    #[test]
    fn single_word() {
        assert_eq!(Solution::top_k_frequent(v(&["a"]), 1), v(&["a"]));
    }
}
