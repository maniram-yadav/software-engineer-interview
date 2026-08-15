//! LeetCode Top Interview 150 — #135 Pow(x, n) (Medium)
//!
//! Implement pow(x, n), computing x raised to the power n, in O(log n)
//! time via fast (binary) exponentiation. Negative n is handled by
//! inverting the base and negating the exponent; the exponent is widened
//! to i64 since -n can overflow i32 when n == i32::MIN.
//!
//! Example:
//!   Input: x = 2.00000, n = 10
//!   Output: 1024.00000

struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut base = x;
        let mut exp = n as i64;
        if exp < 0 {
            base = 1.0 / base;
            exp = -exp;
        }
        let mut result = 1.0;
        while exp > 0 {
            if exp % 2 == 1 {
                result *= base;
            }
            base *= base;
            exp /= 2;
        }
        result
    }
}

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

fn main() {
    println!("{}", Solution::my_pow(2.0, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(approx_eq(Solution::my_pow(2.0, 10), 1024.0));
    }

    #[test]
    fn example_2() {
        assert!(approx_eq(Solution::my_pow(2.1, 3), 9.261));
    }

    #[test]
    fn example_3_negative_exponent() {
        assert!(approx_eq(Solution::my_pow(2.0, -2), 0.25));
    }

    #[test]
    fn zero_exponent_is_one() {
        assert!(approx_eq(Solution::my_pow(5.0, 0), 1.0));
    }

    #[test]
    fn min_i32_exponent_does_not_overflow() {
        // -i32::MIN overflows i32, so this exercises the i64 widening.
        let result = Solution::my_pow(1.0, i32::MIN);
        assert!(approx_eq(result, 1.0));
    }
}
