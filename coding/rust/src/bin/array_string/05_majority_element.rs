//! LeetCode Top Interview 150 — #5 Majority Element (Easy)
//!
//! Given an array `nums` of size n, return the element that appears more
//! than floor(n / 2) times. It's guaranteed to exist (solved here with
//! the Boyer-Moore voting algorithm: O(n) time, O(1) space).
//!
//! Example:
//!   Input: nums = [2,2,1,1,1,2,2]
//!   Output: 2

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;
        for num in nums {
            if count == 0 {
                candidate = num;
            }
            count += if num == candidate { 1 } else { -1 };
        }
        candidate
    }
}

fn main() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    println!("majority element: {}", Solution::majority_element(nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::majority_element(vec![7]), 7);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(Solution::majority_element(vec![-1, -1, -1, 2, 2]), -1);
    }
}
