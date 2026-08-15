//! LeetCode Top Interview 150 — #125 Add Binary (Easy)
//!
//! Given two binary strings a and b, return their sum as a binary
//! string.
//!
//! Example:
//!   Input: a = "11", b = "1"
//!   Output: "100"

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        let mut i = a_bytes.len() as i32 - 1;
        let mut j = b_bytes.len() as i32 - 1;
        let mut carry = 0;
        let mut result = Vec::new();

        while i >= 0 || j >= 0 || carry != 0 {
            let mut sum = carry;
            if i >= 0 {
                sum += (a_bytes[i as usize] - b'0') as i32;
                i -= 1;
            }
            if j >= 0 {
                sum += (b_bytes[j as usize] - b'0') as i32;
                j -= 1;
            }
            result.push((b'0' + (sum % 2) as u8) as char);
            carry = sum / 2;
        }

        result.iter().rev().collect()
    }
}

fn main() {
    println!("{}", Solution::add_binary("11".to_string(), "1".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }

    #[test]
    fn different_lengths() {
        assert_eq!(
            Solution::add_binary("100".to_string(), "1".to_string()),
            "101".to_string()
        );
    }

    #[test]
    fn all_ones_carries_through() {
        assert_eq!(
            Solution::add_binary("111".to_string(), "1".to_string()),
            "1000".to_string()
        );
    }
}
