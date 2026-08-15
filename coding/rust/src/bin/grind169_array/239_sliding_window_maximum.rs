//! Grind 169 — LeetCode #239 Sliding Window Maximum (Hard)
//!
//! Given an array nums and a sliding window of size k moving from left
//! to right, return the max value in the window at each position.
//! Solved with a monotonic deque of indices holding strictly decreasing
//! values; the front is always the current window's maximum.
//!
//! Example:
//!   Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
//!   Output: [3,3,5,5,6,7]

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut deque: VecDeque<usize> = VecDeque::new();
        let mut result = Vec::new();

        for i in 0..nums.len() {
            while let Some(&back) = deque.back() {
                if nums[back] <= nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back(i);

            if *deque.front().unwrap() + k <= i {
                deque.pop_front();
            }
            if i + 1 >= k {
                result.push(nums[*deque.front().unwrap()]);
            }
        }

        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }

    #[test]
    fn example_2_single_element_window() {
        assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
    }

    #[test]
    fn window_covers_whole_array() {
        assert_eq!(
            Solution::max_sliding_window(vec![9, 11], 2),
            vec![11]
        );
    }

    #[test]
    fn decreasing_values() {
        assert_eq!(
            Solution::max_sliding_window(vec![5, 4, 3, 2, 1], 2),
            vec![5, 4, 3, 2]
        );
    }
}
