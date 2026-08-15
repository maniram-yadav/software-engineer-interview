//! LeetCode Top Interview 150 — #127 Number of 1 Bits (Easy)
//!
//! Given a 32-bit unsigned integer, return the number of set bits
//! (Hamming weight).
//!
//! Example:
//!   Input: n = 00000000000000000000000000001011
//!   Output: 3

struct Solution;

impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        n.count_ones() as i32
    }
}

fn main() {
    println!("{}", Solution::hamming_weight(0b1011));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::hamming_weight(0b00000000000000000000000000001011), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::hamming_weight(0b00000000000000000000000010000000), 1);
    }

    #[test]
    fn example_3_all_ones() {
        assert_eq!(Solution::hamming_weight(0b11111111111111111111111111111101), 31);
    }

    #[test]
    fn zero_has_no_bits() {
        assert_eq!(Solution::hamming_weight(0), 0);
    }
}
