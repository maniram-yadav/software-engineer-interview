//! LeetCode Top Interview 150 — #17 Roman to Integer (Easy)
//!
//! Convert a Roman numeral string to an integer.
//!
//! Example:
//!   Input: s = "MCMXCIV"
//!   Output: 1994

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn value(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        }

        let chars: Vec<char> = s.chars().collect();
        let mut total = 0;
        for i in 0..chars.len() {
            let cur = value(chars[i]);
            if i + 1 < chars.len() && cur < value(chars[i + 1]) {
                total -= cur;
            } else {
                total += cur;
            }
        }
        total
    }
}

fn main() {
    println!("value: {}", Solution::roman_to_int("MCMXCIV".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }

    #[test]
    fn single_char() {
        assert_eq!(Solution::roman_to_int("M".to_string()), 1000);
    }
}
