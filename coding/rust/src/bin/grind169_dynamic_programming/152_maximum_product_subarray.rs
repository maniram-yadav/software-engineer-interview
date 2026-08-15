//! Grind 169 — LeetCode #152 Maximum Product Subarray (Medium)
//!
//! Given an integer array nums, find a contiguous subarray with the
//! largest product, and return that product. Tracks both the running
//! max and min product ending at each position, since a negative number
//! can flip the min into the new max.
//!
//! Example:
//!   Input: nums = [2,3,-2,4]
//!   Output: 6   ([2,3])

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_prod = nums[0];
        let mut min_prod = nums[0];
        let mut best = nums[0];

        for &n in &nums[1..] {
            if n < 0 {
                std::mem::swap(&mut max_prod, &mut min_prod);
            }
            max_prod = n.max(max_prod * n);
            min_prod = n.min(min_prod * n);
            best = best.max(max_prod);
        }

        best
    }
}

fn main() {
    println!("{}", Solution::max_product(vec![2, 3, -2, 4]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    }

    #[test]
    fn example_2_zero_resets() {
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }

    #[test]
    fn single_negative() {
        assert_eq!(Solution::max_product(vec![-2]), -2);
    }

    #[test]
    fn two_negatives_multiply_positive() {
        assert_eq!(Solution::max_product(vec![-2, 3, -4]), 24);
    }
}
