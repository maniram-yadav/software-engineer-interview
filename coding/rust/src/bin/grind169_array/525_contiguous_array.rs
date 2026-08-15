//! Grind 169 — LeetCode #525 Contiguous Array (Medium)
//!
//! Given a binary array nums, return the maximum length of a contiguous
//! subarray with an equal number of 0s and 1s. Treat 0 as -1 and 1 as
//! +1; a subarray sums to zero exactly when it has equal counts, so the
//! answer is the widest span between two indices with the same running
//! sum (tracked via the first-seen index of each running sum).
//!
//! Example:
//!   Input: nums = [0,1,0,1]
//!   Output: 4

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut first_seen: HashMap<i32, i32> = HashMap::new();
        first_seen.insert(0, -1);
        let mut count = 0;
        let mut best = 0;

        for (i, n) in nums.iter().enumerate() {
            count += if *n == 1 { 1 } else { -1 };
            if let Some(&prev_idx) = first_seen.get(&count) {
                best = best.max(i as i32 - prev_idx);
            } else {
                first_seen.insert(count, i as i32);
            }
        }

        best
    }
}

fn main() {
    println!("{}", Solution::find_max_length(vec![0, 1, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
    }

    #[test]
    fn whole_array_balanced() {
        assert_eq!(Solution::find_max_length(vec![0, 1, 0, 1]), 4);
    }

    #[test]
    fn no_balanced_subarray() {
        assert_eq!(Solution::find_max_length(vec![0, 0, 0]), 0);
    }
}
