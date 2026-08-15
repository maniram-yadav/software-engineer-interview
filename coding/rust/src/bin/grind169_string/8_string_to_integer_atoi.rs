//! Grind 169 — LeetCode #8 String to Integer (atoi) (Medium)
//!
//! Implement atoi to convert a string to a 32-bit signed integer,
//! following specific whitespace/sign/overflow rules: skip leading
//! spaces, take an optional sign, consume digits until a non-digit, and
//! clamp to the i32 range.
//!
//! Example:
//!   Input: s = "   -42"
//!   Output: -42

struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut i = 0;
        while i < n && chars[i] == ' ' {
            i += 1;
        }

        let mut sign: i64 = 1;
        if i < n && (chars[i] == '+' || chars[i] == '-') {
            if chars[i] == '-' {
                sign = -1;
            }
            i += 1;
        }

        let mut num: i64 = 0;
        while i < n && chars[i].is_ascii_digit() {
            num = num * 10 + chars[i].to_digit(10).unwrap() as i64;
            if sign * num < i32::MIN as i64 {
                return i32::MIN;
            }
            if sign * num > i32::MAX as i64 {
                return i32::MAX;
            }
            i += 1;
        }

        (sign * num) as i32
    }
}

fn main() {
    println!("{}", Solution::my_atoi("   -42".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }

    #[test]
    fn example_2_leading_whitespace_and_sign() {
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
    }

    #[test]
    fn example_3_stops_at_non_digit() {
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    }

    #[test]
    fn no_digits_is_zero() {
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }

    #[test]
    fn overflow_clamps_to_i32_min() {
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), i32::MIN);
    }
}
