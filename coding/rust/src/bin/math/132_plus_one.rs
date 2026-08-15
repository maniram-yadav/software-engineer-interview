//! LeetCode Top Interview 150 — #132 Plus One (Easy)
//!
//! Given a non-empty array of digits representing a non-negative
//! integer, increment the integer by one and return the resulting digit
//! array.
//!
//! Example:
//!   Input: digits = [1,2,3]
//!   Output: [1,2,4]

struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            digits[i] = 0;
        }
        let mut result = vec![1];
        result.extend(digits);
        result
    }
}

fn main() {
    println!("{:?}", Solution::plus_one(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn example_2_all_nines() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn example_3_single_nine() {
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }

    #[test]
    fn carries_through_all_nines() {
        assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }
}
