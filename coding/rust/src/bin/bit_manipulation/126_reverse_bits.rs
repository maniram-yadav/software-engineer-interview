//! LeetCode Top Interview 150 — #126 Reverse Bits (Easy)
//!
//! Reverse the bits of a given 32-bit unsigned integer.
//!
//! Example:
//!   Input: n = 00000010100101000001111010011100
//!   Output: 964176192 (00111001011110000010100101000000)

struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut result: u32 = 0;
        let mut x = x;
        for _ in 0..32 {
            result = (result << 1) | (x & 1);
            x >>= 1;
        }
        result
    }
}

fn main() {
    println!("{}", Solution::reverse_bits(43261596));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::reverse_bits(0b00000010100101000001111010011100), 964176192);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::reverse_bits(0b11111111111111111111111111111101), 3221225471);
    }

    #[test]
    fn all_zeros() {
        assert_eq!(Solution::reverse_bits(0), 0);
    }

    #[test]
    fn all_ones() {
        assert_eq!(Solution::reverse_bits(u32::MAX), u32::MAX);
    }

    #[test]
    fn single_bit() {
        assert_eq!(Solution::reverse_bits(1), 1u32 << 31);
    }
}
