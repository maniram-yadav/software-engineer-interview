//! LeetCode Top Interview 150 — #48 Summary Ranges (Easy)
//!
//! Given a sorted unique array of integers, return the smallest sorted
//! list of ranges that exactly cover all numbers.
//!
//! Example:
//!   Input: nums = [0,1,2,4,5,7]
//!   Output: ["0->2","4->5","7"]

struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        let n = nums.len();
        let mut i = 0;

        while i < n {
            let mut j = i;
            while j + 1 < n && nums[j + 1] == nums[j] + 1 {
                j += 1;
            }
            if i == j {
                result.push(format!("{}", nums[i]));
            } else {
                result.push(format!("{}->{}", nums[i], nums[j]));
            }
            i = j + 1;
        }

        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            v(&["0->2", "4->5", "7"])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            v(&["0", "2->4", "6", "8->9"])
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(Solution::summary_ranges(vec![]), Vec::<String>::new());
    }

    #[test]
    fn single_range() {
        assert_eq!(
            Solution::summary_ranges(vec![1, 2, 3, 4]),
            v(&["1->4"])
        );
    }
}
