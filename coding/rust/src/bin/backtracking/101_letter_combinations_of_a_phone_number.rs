//! LeetCode Top Interview 150 — #101 Letter Combinations of a Phone
//! Number (Medium)
//!
//! Given a string of digits 2-9, return all possible letter combinations
//! the number could represent (standard phone keypad mapping).
//!
//! Example:
//!   Input: digits = "23"
//!   Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let mapping: HashMap<char, &str> = HashMap::from([
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]);

        fn backtrack(
            digits: &[char],
            idx: usize,
            current: &mut String,
            mapping: &HashMap<char, &str>,
            result: &mut Vec<String>,
        ) {
            if idx == digits.len() {
                result.push(current.clone());
                return;
            }
            if let Some(letters) = mapping.get(&digits[idx]) {
                for c in letters.chars() {
                    current.push(c);
                    backtrack(digits, idx + 1, current, mapping, result);
                    current.pop();
                }
            }
        }

        let chars: Vec<char> = digits.chars().collect();
        let mut result = Vec::new();
        let mut current = String::new();
        backtrack(&chars, 0, &mut current, &mapping, &mut result);
        result
    }
}

fn main() {
    println!("{:?}", Solution::letter_combinations("23".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_1() {
        let mut result = Solution::letter_combinations("23".to_string());
        result.sort();
        let mut expected = v(&["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn example_2_empty() {
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            Vec::<String>::new()
        );
    }

    #[test]
    fn example_3_single_digit() {
        let mut result = Solution::letter_combinations("2".to_string());
        result.sort();
        assert_eq!(result, v(&["a", "b", "c"]));
    }
}
