//! LeetCode Top Interview 150 — #23 Find the Index of the First Occurrence
//! in a String (Easy)
//!
//! Given strings `haystack` and `needle`, return the index of the first
//! occurrence of `needle` in `haystack`, or -1 if it doesn't occur.
//!
//! Example:
//!   Input: haystack = "sadbutsad", needle = "sad"
//!   Output: 0

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        let h = haystack.as_bytes();
        let n = needle.as_bytes();
        if n.len() > h.len() {
            return -1;
        }
        for i in 0..=(h.len() - n.len()) {
            if &h[i..i + n.len()] == n {
                return i as i32;
            }
        }
        -1
    }
}

fn main() {
    println!(
        "{}",
        Solution::str_str("sadbutsad".to_string(), "sad".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
    }

    #[test]
    fn example_2_not_found() {
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
    }

    #[test]
    fn needle_at_end() {
        assert_eq!(
            Solution::str_str("hello".to_string(), "llo".to_string()),
            2
        );
    }

    #[test]
    fn empty_needle() {
        assert_eq!(Solution::str_str("abc".to_string(), "".to_string()), 0);
    }
}
