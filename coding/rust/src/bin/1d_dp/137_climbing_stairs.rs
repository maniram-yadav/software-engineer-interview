//! LeetCode Top Interview 150 — #137 Climbing Stairs (Easy)
//!
//! You're climbing a staircase of n steps, taking 1 or 2 steps at a
//! time. Return the number of distinct ways to reach the top. This is
//! the Fibonacci recurrence: ways(n) = ways(n-1) + ways(n-2).
//!
//! Example:
//!   Input: n = 3
//!   Output: 3

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        let (mut a, mut b) = (1, 2);
        for _ in 3..=n {
            let c = a + b;
            a = b;
            b = c;
        }
        b
    }
}

fn main() {
    println!("{}", Solution::climb_stairs(5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn single_step() {
        assert_eq!(Solution::climb_stairs(1), 1);
    }

    #[test]
    fn larger_n() {
        assert_eq!(Solution::climb_stairs(5), 8);
    }
}
