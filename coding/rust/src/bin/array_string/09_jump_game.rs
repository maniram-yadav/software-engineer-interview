//! LeetCode Top Interview 150 — #9 Jump Game (Medium)
//!
//! Given an array `nums` where `nums[i]` is the max jump length from index
//! i, starting at index 0, return true if you can reach the last index.
//!
//! Example:
//!   Input: nums = [2,3,1,1,4]
//!   Output: true

struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach: i32 = 0;
        for (i, &step) in nums.iter().enumerate() {
            if i as i32 > max_reach {
                return false;
            }
            max_reach = max_reach.max(i as i32 + step);
        }
        true
    }
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4];
    println!("can jump: {}", Solution::can_jump(nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    }

    #[test]
    fn example_2_stuck() {
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::can_jump(vec![0]), true);
    }

    #[test]
    fn reachable_with_zero_in_between() {
        assert_eq!(Solution::can_jump(vec![1, 0]), true);
    }
}
