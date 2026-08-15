//! LeetCode Top Interview 150 — #13 Product of Array Except Self (Medium)
//!
//! Given an array `nums`, return an array `answer` where `answer[i]` is
//! the product of all elements except `nums[i]`, without using division,
//! in O(n) time via prefix/suffix product passes.
//!
//! Example:
//!   Input: nums = [1,2,3,4]
//!   Output: [24,12,8,6]

struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![1; n];

        let mut prefix = 1;
        for i in 0..n {
            res[i] = prefix;
            prefix *= nums[i];
        }

        let mut suffix = 1;
        for i in (0..n).rev() {
            res[i] *= suffix;
            suffix *= nums[i];
        }

        res
    }
}

fn main() {
    println!("{:?}", Solution::product_except_self(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }

    #[test]
    fn example_2_with_zero() {
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }

    #[test]
    fn two_elements() {
        assert_eq!(Solution::product_except_self(vec![3, 5]), vec![5, 3]);
    }

    #[test]
    fn two_zeros() {
        assert_eq!(Solution::product_except_self(vec![0, 0, 2]), vec![0, 0, 0]);
    }
}
