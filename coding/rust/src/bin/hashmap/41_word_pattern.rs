//! LeetCode Top Interview 150 — #41 Word Pattern (Easy)
//!
//! Given a `pattern` and a string `s`, find if `s` follows the same
//! pattern — a full bijection between letters in `pattern` and words in
//! `s`.
//!
//! Example:
//!   Input: pattern = "abba", s = "dog cat cat dog"
//!   Output: true

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split(' ').collect();
        if pattern.len() != words.len() {
            return false;
        }

        let mut p2w: HashMap<char, &str> = HashMap::new();
        let mut w2p: HashMap<&str, char> = HashMap::new();

        for (c, w) in pattern.chars().zip(words.iter()) {
            match (p2w.get(&c), w2p.get(w)) {
                (Some(&mapped), _) if mapped != *w => return false,
                (_, Some(&mapped)) if mapped != c => return false,
                _ => {
                    p2w.insert(c, w);
                    w2p.insert(w, c);
                }
            }
        }

        true
    }
}

fn main() {
    println!(
        "{}",
        Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
            false
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()),
            false
        );
    }

    #[test]
    fn mismatched_lengths() {
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat dog dog".to_string()),
            false
        );
    }

    #[test]
    fn not_injective() {
        // Two different pattern letters mapping to the same word.
        assert_eq!(
            Solution::word_pattern("ab".to_string(), "dog dog".to_string()),
            false
        );
    }
}
