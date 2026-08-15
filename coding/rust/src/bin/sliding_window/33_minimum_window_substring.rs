//! LeetCode Top Interview 150 — #33 Minimum Window Substring (Hard)
//!
//! Given strings `s` and `t`, return the smallest substring of `s` that
//! contains every character of `t` (with multiplicity). Return "" if no
//! such substring exists. Solved with a classic shrink/grow sliding
//! window tracking how many required distinct characters are satisfied.
//!
//! Example:
//!   Input: s = "ADOBECODEBANC", t = "ABC"
//!   Output: "BANC"

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.is_empty() || s.len() < t.len() {
            return String::new();
        }

        let mut need: HashMap<char, i32> = HashMap::new();
        for c in t.chars() {
            *need.entry(c).or_insert(0) += 1;
        }
        let required = need.len();

        let mut window: HashMap<char, i32> = HashMap::new();
        let mut formed = 0;
        let s_chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut best_len = usize::MAX;
        let mut best_left = 0;

        for right in 0..s_chars.len() {
            let c = s_chars[right];
            *window.entry(c).or_insert(0) += 1;
            if let Some(&need_count) = need.get(&c) {
                if window[&c] == need_count {
                    formed += 1;
                }
            }

            while formed == required {
                if right - left + 1 < best_len {
                    best_len = right - left + 1;
                    best_left = left;
                }
                let lc = s_chars[left];
                if let Some(cnt) = window.get_mut(&lc) {
                    *cnt -= 1;
                    if let Some(&need_count) = need.get(&lc) {
                        if *cnt < need_count {
                            formed -= 1;
                        }
                    }
                }
                left += 1;
            }
        }

        if best_len == usize::MAX {
            String::new()
        } else {
            s_chars[best_left..best_left + best_len].iter().collect()
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
    }

    #[test]
    fn example_2_no_valid_window() {
        assert_eq!(
            Solution::min_window("a".to_string(), "a".to_string()),
            "a".to_string()
        );
    }

    #[test]
    fn example_3_impossible() {
        assert_eq!(
            Solution::min_window("a".to_string(), "aa".to_string()),
            "".to_string()
        );
    }

    #[test]
    fn whole_string_needed() {
        assert_eq!(
            Solution::min_window("aa".to_string(), "aa".to_string()),
            "aa".to_string()
        );
    }
}
