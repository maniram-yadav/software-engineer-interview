//! LeetCode Top Interview 150 — #28 Container With Most Water (Medium)
//!
//! Given n non-negative integers `height[i]` representing vertical lines at
//! position i, find two lines that together with the x-axis form a
//! container holding the most water.
//!
//! Example:
//!   Input: height = [1,8,6,2,5,4,8,3,7]
//!   Output: 49

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = height.len() - 1;
        let mut best = 0;
        while left < right {
            let h = height[left].min(height[right]);
            let w = (right - left) as i32;
            best = best.max(h * w);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        best
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]),
            49
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn two_elements_uneven() {
        assert_eq!(Solution::max_area(vec![4, 3]), 3);
    }

    #[test]
    fn increasing_heights() {
        assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5]), 6);
    }
}
