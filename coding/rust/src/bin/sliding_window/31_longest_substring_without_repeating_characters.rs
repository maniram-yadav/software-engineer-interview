//! LeetCode Top Interview 150 — #31 Longest Substring Without Repeating
//! Characters (Medium)
//!
//! Given a string `s`, find the length of the longest substring without
//! repeating characters. Solved with a sliding window tracking the last
//! seen index of each character.
//!
//! Example:
//!   Input: s = "abcabcbb"
//!   Output: 3

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen: HashMap<char, i32> = HashMap::new();
        let mut left: i32 = -1;
        let mut best = 0;

        for (right, c) in s.chars().enumerate() {
            if let Some(&idx) = last_seen.get(&c) {
                if idx > left {
                    left = idx;
                }
            }
            last_seen.insert(c, right as i32);
            best = best.max(right as i32 - left);
        }

        best
    }
}

fn main() {
    println!(
        "{}",
        Solution::length_of_longest_substring("abcabcbb".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn example_2_all_same_char() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn empty_string() {
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }
}
