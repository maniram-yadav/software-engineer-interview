//! Grind 169 — LeetCode #438 Find All Anagrams in a String (Medium)
//!
//! Given strings s and p, return all start indices of p's anagrams in
//! s. Solved with a fixed-size sliding window of length |p|, comparing
//! 26-letter frequency counts.
//!
//! Example:
//!   Input: s = "cbaebabacd", p = "abc"
//!   Output: [0,6]

struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return vec![];
        }
        let mut need = [0i32; 26];
        for b in p.bytes() {
            need[(b - b'a') as usize] += 1;
        }
        let mut window = [0i32; 26];
        let s_bytes = s.as_bytes();
        let plen = p.len();
        let mut result = Vec::new();

        for i in 0..s_bytes.len() {
            window[(s_bytes[i] - b'a') as usize] += 1;
            if i >= plen {
                window[(s_bytes[i - plen] - b'a') as usize] -= 1;
            }
            if i >= plen - 1 && window == need {
                result.push((i + 1 - plen) as i32);
            }
        }

        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
            vec![0, 6]
        );
    }

    #[test]
    fn example_2_overlapping_matches() {
        assert_eq!(
            Solution::find_anagrams("abab".to_string(), "ab".to_string()),
            vec![0, 1, 2]
        );
    }

    #[test]
    fn p_longer_than_s() {
        assert_eq!(
            Solution::find_anagrams("a".to_string(), "ab".to_string()),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn no_anagrams_present() {
        assert_eq!(
            Solution::find_anagrams("xyz".to_string(), "ab".to_string()),
            Vec::<i32>::new()
        );
    }
}
