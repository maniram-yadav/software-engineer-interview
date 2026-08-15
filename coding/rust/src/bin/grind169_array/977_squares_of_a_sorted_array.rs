//! Grind 169 — LeetCode #977 Squares of a Sorted Array (Easy)
//!
//! Given an integer array nums sorted in non-decreasing order, return an
//! array of the squares of each number, also sorted in non-decreasing
//! order. Solved with two pointers from both ends, since the largest
//! square always comes from one of the extremes.
//!
//! Example:
//!   Input: nums = [-4,-1,0,3,10]
//!   Output: [0,1,9,16,100]

struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        let (mut l, mut r) = (0i32, n as i32 - 1);
        let mut pos = n as i32 - 1;

        while l <= r {
            let left_sq = nums[l as usize] * nums[l as usize];
            let right_sq = nums[r as usize] * nums[r as usize];
            if left_sq > right_sq {
                result[pos as usize] = left_sq;
                l += 1;
            } else {
                result[pos as usize] = right_sq;
                r -= 1;
            }
            pos -= 1;
        }

        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::sorted_squares(vec![-4, -1, 0, 3, 10])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }

    #[test]
    fn all_non_negative() {
        assert_eq!(Solution::sorted_squares(vec![0, 2, 3]), vec![0, 4, 9]);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::sorted_squares(vec![-5]), vec![25]);
    }
}
