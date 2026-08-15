//! LeetCode Top Interview 150 — #145 Longest Palindromic Substring (Medium)
//!
//! Given a string s, return the longest palindromic substring. Solved
//! with the "expand around center" technique: try every odd center (i,i)
//! and even center (i,i+1), expanding outward while characters match.
//!
//! Example:
//!   Input: s = "babad"
//!   Output: "bab"  (or "aba")

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        if n == 0 {
            return String::new();
        }

        fn expand(chars: &[char], l: i32, r: i32) -> (i32, i32) {
            let mut l = l;
            let mut r = r;
            while l >= 0 && (r as usize) < chars.len() && chars[l as usize] == chars[r as usize] {
                l -= 1;
                r += 1;
            }
            (l + 1, r - 1)
        }

        let mut start = 0usize;
        let mut max_len = 1usize;

        for i in 0..n {
            let (l1, r1) = expand(&chars, i as i32, i as i32);
            let len1 = (r1 - l1 + 1) as usize;
            if len1 > max_len {
                max_len = len1;
                start = l1 as usize;
            }

            if i + 1 < n {
                let (l2, r2) = expand(&chars, i as i32, i as i32 + 1);
                if r2 >= l2 {
                    let len2 = (r2 - l2 + 1) as usize;
                    if len2 > max_len {
                        max_len = len2;
                        start = l2 as usize;
                    }
                }
            }
        }

        chars[start..start + max_len].iter().collect()
    }
}

fn main() {
    println!("{}", Solution::longest_palindrome("babad".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
    }

    #[test]
    fn example_2_even_length() {
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }

    #[test]
    fn single_char() {
        assert_eq!(
            Solution::longest_palindrome("a".to_string()),
            "a".to_string()
        );
    }

    #[test]
    fn whole_string_is_palindrome() {
        assert_eq!(
            Solution::longest_palindrome("racecar".to_string()),
            "racecar".to_string()
        );
    }
}
