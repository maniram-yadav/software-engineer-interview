//! LeetCode Top Interview 150 — #134 Sqrt(x) (Easy)
//!
//! Given a non-negative integer x, return the integer square root of x
//! (truncated), without using built-in power/sqrt functions. Solved with
//! binary search over candidate roots, using i64 to avoid overflow when
//! squaring.
//!
//! Example:
//!   Input: x = 8
//!   Output: 2

struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let target = x as i64;
        let (mut lo, mut hi) = (1i64, target);
        let mut best = 1i64;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let sq = mid * mid;
            if sq == target {
                return mid as i32;
            } else if sq < target {
                best = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        best as i32
    }
}

fn main() {
    println!("{}", Solution::my_sqrt(8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_perfect_square() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }

    #[test]
    fn example_2_truncates() {
        assert_eq!(Solution::my_sqrt(8), 2);
    }

    #[test]
    fn zero_and_one() {
        assert_eq!(Solution::my_sqrt(0), 0);
        assert_eq!(Solution::my_sqrt(1), 1);
    }

    #[test]
    fn large_value() {
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
    }
}
