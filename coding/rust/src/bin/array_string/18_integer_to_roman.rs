//! LeetCode Top Interview 150 — #18 Integer to Roman (Medium)
//!
//! Convert an integer (1 to 3999) to a Roman numeral string.
//!
//! Example:
//!   Input: num = 1994
//!   Output: "MCMXCIV"

struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        const VALUES: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        const SYMBOLS: [&str; 13] = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];

        let mut result = String::new();
        for i in 0..VALUES.len() {
            while num >= VALUES[i] {
                num -= VALUES[i];
                result.push_str(SYMBOLS[i]);
            }
        }
        result
    }
}

fn main() {
    println!("roman: {}", Solution::int_to_roman(1994));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX".to_string());
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
    }

    #[test]
    fn smallest_value() {
        assert_eq!(Solution::int_to_roman(1), "I".to_string());
    }

    #[test]
    fn largest_value() {
        assert_eq!(Solution::int_to_roman(3999), "MMMCMXCIX".to_string());
    }
}
