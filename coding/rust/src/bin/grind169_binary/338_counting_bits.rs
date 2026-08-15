//! Grind 169 — LeetCode #338 Counting Bits (Easy)
//!
//! Given an integer n, return an array ans of length n+1 where ans[i] is
//! the number of 1's in the binary representation of i. Uses the
//! recurrence ans[i] = ans[i >> 1] + (i & 1): dropping the last bit
//! either keeps or reduces the popcount by exactly that bit.
//!
//! Example:
//!   Input: n = 5
//!   Output: [0,1,1,2,1,2]

struct Solution;

impl Solution {
    pub fn counting_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans = vec![0; n + 1];
        for i in 1..=n {
            ans[i] = ans[i >> 1] + (i & 1) as i32;
        }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::counting_bits(5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::counting_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::counting_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn zero() {
        assert_eq!(Solution::counting_bits(0), vec![0]);
    }

    #[test]
    fn power_of_two() {
        assert_eq!(Solution::counting_bits(8)[8], 1);
    }
}
