//! LeetCode Top Interview 150 — #97 Word Ladder (Hard)
//!
//! Given beginWord, endWord, and a wordList, return the length of the
//! shortest transformation sequence changing one letter at a time, each
//! intermediate word in wordList, or 0 if none exists. Solved with BFS
//! over single-character substitutions, only stepping into words present
//! in the word list.
//!
//! Example:
//!   Input: beginWord = "hit", endWord = "cog",
//!          wordList = ["hot","dot","dog","lot","log","cog"]
//!   Output: 5

use std::collections::{HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_set: HashSet<String> = word_list.into_iter().collect();
        if !word_set.contains(&end_word) {
            return 0;
        }

        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(begin_word.clone());
        let mut queue: VecDeque<(String, i32)> = VecDeque::new();
        queue.push_back((begin_word, 1));

        while let Some((word, steps)) = queue.pop_front() {
            if word == end_word {
                return steps;
            }
            let chars: Vec<char> = word.chars().collect();
            for i in 0..chars.len() {
                for c in b'a'..=b'z' {
                    let c = c as char;
                    if c == chars[i] {
                        continue;
                    }
                    let mut next_chars = chars.clone();
                    next_chars[i] = c;
                    let next_word: String = next_chars.into_iter().collect();
                    if word_set.contains(&next_word) && !visited.contains(&next_word) {
                        visited.insert(next_word.clone());
                        queue.push_back((next_word, steps + 1));
                    }
                }
            }
        }

        0
    }
}

fn v(strs: &[&str]) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
}

fn main() {
    let result = Solution::ladder_length(
        "hit".to_string(),
        "cog".to_string(),
        v(&["hot", "dot", "dog", "lot", "log", "cog"]),
    );
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                v(&["hot", "dot", "dog", "lot", "log", "cog"])
            ),
            5
        );
    }

    #[test]
    fn example_2_end_not_reachable() {
        assert_eq!(
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                v(&["hot", "dot", "dog", "lot", "log"])
            ),
            0
        );
    }

    #[test]
    fn direct_neighbor() {
        assert_eq!(
            Solution::ladder_length("hit".to_string(), "hot".to_string(), v(&["hot"])),
            2
        );
    }

    #[test]
    fn begin_equals_end_not_in_list_still_needs_path() {
        assert_eq!(
            Solution::ladder_length("a".to_string(), "c".to_string(), v(&["a", "b", "c"])),
            2
        );
    }
}
