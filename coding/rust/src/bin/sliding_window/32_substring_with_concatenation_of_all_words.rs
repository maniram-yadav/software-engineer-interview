//! LeetCode Top Interview 150 — #32 Substring with Concatenation of All
//! Words (Hard)
//!
//! Given a string `s` and an array of same-length `words`, return the
//! starting indices of all substrings in `s` that are a concatenation of
//! every word in `words` exactly once, in any order. Solved with a
//! sliding window run once per possible starting offset (0..word_len).
//!
//! Example:
//!   Input: s = "barfoothefoobarman", words = ["foo","bar"]
//!   Output: [0,9]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.is_empty() {
            return vec![];
        }
        let word_len = words[0].len();
        let num_words = words.len();
        let total_len = word_len * num_words;
        let n = s.len();
        if word_len == 0 || n < total_len {
            return vec![];
        }

        let mut word_count: HashMap<String, i32> = HashMap::new();
        for w in &words {
            *word_count.entry(w.clone()).or_insert(0) += 1;
        }

        let s_bytes = s.as_bytes();
        let mut result = Vec::new();

        for offset in 0..word_len {
            let mut left = offset;
            let mut count = 0;
            let mut window: HashMap<String, i32> = HashMap::new();
            let mut j = offset;

            while j + word_len <= n {
                let word = String::from_utf8(s_bytes[j..j + word_len].to_vec()).unwrap();
                j += word_len;

                if let Some(&needed) = word_count.get(&word) {
                    *window.entry(word.clone()).or_insert(0) += 1;
                    count += 1;

                    while window[&word] > needed {
                        let left_word =
                            String::from_utf8(s_bytes[left..left + word_len].to_vec()).unwrap();
                        *window.get_mut(&left_word).unwrap() -= 1;
                        left += word_len;
                        count -= 1;
                    }

                    if count == num_words {
                        result.push(left as i32);
                        let left_word =
                            String::from_utf8(s_bytes[left..left + word_len].to_vec()).unwrap();
                        *window.get_mut(&left_word).unwrap() -= 1;
                        left += word_len;
                        count -= 1;
                    }
                } else {
                    window.clear();
                    count = 0;
                    left = j;
                }
            }
        }

        result
    }
}

fn main() {
    let words = vec!["foo".to_string(), "bar".to_string()];
    println!(
        "{:?}",
        Solution::find_substring("barfoothefoobarman".to_string(), words)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_1() {
        let mut result = Solution::find_substring(
            "barfoothefoobarman".to_string(),
            v(&["foo", "bar"]),
        );
        result.sort();
        assert_eq!(result, vec![0, 9]);
    }

    #[test]
    fn example_2_no_match() {
        let result = Solution::find_substring(
            "wordgoodgoodgoodbestword".to_string(),
            v(&["word", "good", "best", "word"]),
        );
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn example_3() {
        let mut result = Solution::find_substring(
            "barfoofoobarthefoobarman".to_string(),
            v(&["bar", "foo", "the"]),
        );
        result.sort();
        assert_eq!(result, vec![6, 9, 12]);
    }
}
