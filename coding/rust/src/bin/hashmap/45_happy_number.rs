//! LeetCode Top Interview 150 — #45 Happy Number (Easy)
//!
//! A happy number is defined by repeatedly replacing it with the sum of
//! the squares of its digits, eventually reaching 1 (unhappy numbers
//! cycle forever). Determine if `n` is happy. Solved with Floyd's cycle
//! detection (slow/fast pointers) over the sequence, avoiding a HashSet.
//!
//! Example:
//!   Input: n = 19
//!   Output: true

struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        fn next(mut x: i32) -> i32 {
            let mut sum = 0;
            while x > 0 {
                let d = x % 10;
                sum += d * d;
                x /= 10;
            }
            sum
        }

        let mut slow = n;
        let mut fast = next(n);
        while fast != 1 && slow != fast {
            slow = next(slow);
            fast = next(next(fast));
        }
        fast == 1
    }
}

fn main() {
    println!("{}", Solution::is_happy(19));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::is_happy(19), true);
    }

    #[test]
    fn example_2_not_happy() {
        assert_eq!(Solution::is_happy(2), false);
    }

    #[test]
    fn one_is_happy() {
        assert_eq!(Solution::is_happy(1), true);
    }

    #[test]
    fn another_happy_number() {
        assert_eq!(Solution::is_happy(7), true);
    }
}
