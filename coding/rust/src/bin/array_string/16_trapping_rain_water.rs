//! LeetCode Top Interview 150 — #16 Trapping Rain Water (Hard)
//!
//! Given n non-negative integers representing an elevation map where each
//! bar has width 1, compute how much water it can trap after raining.
//! Solved with the two-pointer / running-max technique in O(n) time,
//! O(1) space.
//!
//! Example:
//!   Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
//!   Output: 6

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let mut left = 0usize;
        let mut right = height.len() - 1;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut water = 0;

        while left < right {
            if height[left] < height[right] {
                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    water += left_max - height[left];
                }
                left += 1;
            } else {
                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    water += right_max - height[right];
                }
                right -= 1;
            }
        }

        water
    }
}

fn main() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    println!("trapped water: {}", Solution::trap(height));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]),
            6
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn no_water_monotonic() {
        assert_eq!(Solution::trap(vec![1, 2, 3, 4, 5]), 0);
    }

    #[test]
    fn empty_input() {
        assert_eq!(Solution::trap(vec![]), 0);
    }
}
