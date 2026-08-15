//! LeetCode Top Interview 150 — #20 Longest Common Prefix (Easy)
//!
//! Given an array of strings, find the longest common prefix string among
//! all of them. Return "" if there is none.
//!
//! Example:
//!   Input: strs = ["flower","flow","flight"]
//!   Output: "fl"

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        let first = &strs[0];
        let mut prefix_len = first.len();

        for s in &strs[1..] {
            let mut len = 0;
            for (a, b) in first.bytes().zip(s.bytes()) {
                if a == b {
                    len += 1;
                } else {
                    break;
                }
            }
            prefix_len = prefix_len.min(len);
        }

        first[..prefix_len].to_string()
    }
}

fn main() {
    let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    println!("prefix: {:?}", Solution::longest_common_prefix(strs));
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
            Solution::longest_common_prefix(v(&["flower", "flow", "flight"])),
            "fl".to_string()
        );
    }

    #[test]
    fn example_2_no_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(v(&["dog", "racecar", "car"])),
            "".to_string()
        );
    }

    #[test]
    fn single_string() {
        assert_eq!(
            Solution::longest_common_prefix(v(&["alone"])),
            "alone".to_string()
        );
    }

    #[test]
    fn identical_strings() {
        assert_eq!(
            Solution::longest_common_prefix(v(&["same", "same", "same"])),
            "same".to_string()
        );
    }
}
