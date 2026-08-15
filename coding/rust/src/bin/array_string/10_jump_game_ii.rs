//! LeetCode Top Interview 150 — #10 Jump Game II (Medium)
//!
//! Given an array `nums` where `nums[i]` is the max jump length from index
//! i, starting at index 0, return the minimum number of jumps needed to
//! reach the last index. A valid path is guaranteed to exist.
//!
//! Example:
//!   Input: nums = [2,3,1,1,4]
//!   Output: 2

struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }
        let mut jumps = 0;
        let mut cur_end = 0usize;
        let mut farthest = 0usize;
        for i in 0..n - 1 {
            farthest = farthest.max(i + nums[i] as usize);
            if i == cur_end {
                jumps += 1;
                cur_end = farthest;
                if cur_end >= n - 1 {
                    break;
                }
            }
        }
        jumps
    }
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4];
    println!("min jumps: {}", Solution::jump(nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::jump(vec![0]), 0);
    }

    #[test]
    fn two_elements() {
        assert_eq!(Solution::jump(vec![1, 2]), 1);
    }
}
