//! LeetCode Top Interview 150 — #27 Two Sum II - Input Array Is Sorted (Medium)
//!
//! Given a 1-indexed array `numbers` sorted in non-decreasing order, find
//! two numbers that add up to `target` and return their (1-indexed)
//! positions, using O(1) extra space.
//!
//! Example:
//!   Input: numbers = [2,7,11,15], target = 9
//!   Output: [1,2]

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0i32;
        let mut right = numbers.len() as i32 - 1;
        while left < right {
            let sum = numbers[left as usize] + numbers[right as usize];
            if sum == target {
                return vec![left + 1, right + 1];
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        vec![]
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }

    #[test]
    fn example_3_negative_numbers() {
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }

    #[test]
    fn duplicates_in_array() {
        assert_eq!(Solution::two_sum(vec![1, 2, 2, 4], 4), vec![2, 3]);
    }
}
