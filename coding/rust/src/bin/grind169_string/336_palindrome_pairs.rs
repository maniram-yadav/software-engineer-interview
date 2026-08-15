//! Grind 169 — LeetCode #336 Palindrome Pairs (Hard)
//!
//! Given a list of unique words, return all pairs of indices (i, j) such
//! that concatenating words[i] + words[j] forms a palindrome. For each
//! word, split it at every position into (left, right); if `left` is
//! itself a palindrome and the reverse of `right` exists as another
//! word, that word + this word is a palindrome pair (and symmetrically
//! for `right` being a palindrome).
//!
//! Example:
//!   Input: words = ["abcd","dcba","lls","s","sssll"]
//!   Output: [[0,1],[1,0],[3,2],[2,4]]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        fn is_pal(chars: &[char]) -> bool {
            let n = chars.len();
            for i in 0..n / 2 {
                if chars[i] != chars[n - 1 - i] {
                    return false;
                }
            }
            true
        }

        let index: HashMap<&str, usize> =
            words.iter().enumerate().map(|(i, w)| (w.as_str(), i)).collect();
        let mut result = Vec::new();

        for (i, w) in words.iter().enumerate() {
            let chars: Vec<char> = w.chars().collect();
            let n = chars.len();
            for j in 0..=n {
                let left = &chars[..j];
                let right = &chars[j..];

                if is_pal(left) {
                    let rev_right: String = right.iter().rev().collect();
                    if let Some(&k) = index.get(rev_right.as_str()) {
                        if k != i {
                            result.push(vec![k as i32, i as i32]);
                        }
                    }
                }

                if j != n && is_pal(right) {
                    let rev_left: String = left.iter().rev().collect();
                    if let Some(&k) = index.get(rev_left.as_str()) {
                        if k != i {
                            result.push(vec![i as i32, k as i32]);
                        }
                    }
                }
            }
        }

        result
    }
}

fn v(strs: &[&str]) -> Vec<String> {
    strs.iter().map(|s| s.to_string()).collect()
}

fn normalize(mut pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    pairs.sort();
    pairs
}

fn main() {
    let words = v(&["abcd", "dcba", "lls", "s", "sssll"]);
    println!("{:?}", normalize(Solution::palindrome_pairs(words)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let words = v(&["abcd", "dcba", "lls", "s", "sssll"]);
        let result = normalize(Solution::palindrome_pairs(words));
        let expected = normalize(vec![
            vec![0, 1],
            vec![1, 0],
            vec![3, 2],
            vec![2, 4],
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let words = v(&["bat", "tab", "cat"]);
        let result = normalize(Solution::palindrome_pairs(words));
        assert_eq!(result, normalize(vec![vec![0, 1], vec![1, 0]]));
    }

    #[test]
    fn example_3_empty_string_pairs_with_any_palindrome() {
        let words = v(&["a", ""]);
        let result = normalize(Solution::palindrome_pairs(words));
        assert_eq!(result, normalize(vec![vec![0, 1], vec![1, 0]]));
    }

    #[test]
    fn no_pairs() {
        let words = v(&["abc", "def"]);
        assert_eq!(Solution::palindrome_pairs(words), Vec::<Vec<i32>>::new());
    }
}
