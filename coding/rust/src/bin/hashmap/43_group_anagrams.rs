//! LeetCode Top Interview 150 — #43 Group Anagrams (Medium)
//!
//! Given an array of strings, group the anagrams together (any order).
//! Solved by keying a HashMap on each word's sorted character sequence.
//!
//! Example:
//!   Input: strs = ["eat","tea","tan","ate","nat","bat"]
//!   Output: [["bat"],["nat","tan"],["ate","eat","tea"]]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            let key: String = chars.into_iter().collect();
            groups.entry(key).or_insert_with(Vec::new).push(s);
        }

        groups.into_iter().map(|(_, v)| v).collect()
    }
}

fn main() {
    let strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    println!("{:?}", Solution::group_anagrams(strs));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    // Group order and in-group order are unspecified, so normalize both
    // before comparing.
    fn normalize(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for g in groups.iter_mut() {
            g.sort();
        }
        groups.sort();
        groups
    }

    #[test]
    fn example_1() {
        let strs = v(&["eat", "tea", "tan", "ate", "nat", "bat"]);
        let result = normalize(Solution::group_anagrams(strs));
        let expected = normalize(vec![
            v(&["bat"]),
            v(&["nat", "tan"]),
            v(&["ate", "eat", "tea"]),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2_empty_string() {
        let strs = v(&[""]);
        let result = normalize(Solution::group_anagrams(strs));
        assert_eq!(result, vec![v(&[""])]);
    }

    #[test]
    fn example_3_single_char() {
        let strs = v(&["a"]);
        let result = normalize(Solution::group_anagrams(strs));
        assert_eq!(result, vec![v(&["a"])]);
    }

    #[test]
    fn no_anagrams_each_own_group() {
        let strs = v(&["abc", "def", "ghi"]);
        let result = normalize(Solution::group_anagrams(strs));
        let expected = normalize(vec![v(&["abc"]), v(&["def"]), v(&["ghi"])]);
        assert_eq!(result, expected);
    }
}
