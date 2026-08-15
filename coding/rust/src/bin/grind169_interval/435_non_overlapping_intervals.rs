//! Grind 169 — LeetCode #435 Non-overlapping Intervals (Medium)
//!
//! Given an array of intervals, return the minimum number of intervals
//! to remove so the rest are non-overlapping. Classic activity-selection
//! greedy: sort by end time, keep an interval whenever it starts at or
//! after the last kept interval's end, otherwise it must be removed.
//!
//! Example:
//!   Input: intervals = [[1,2],[2,3],[3,4],[1,3]]
//!   Output: 1

struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|iv| iv[1]);
        let mut count = 0;
        let mut end = i32::MIN;
        for iv in intervals {
            if iv[0] >= end {
                end = iv[1];
            } else {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
    println!("{}", Solution::erase_overlap_intervals(intervals));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
        assert_eq!(Solution::erase_overlap_intervals(intervals), 1);
    }

    #[test]
    fn example_2_all_overlap() {
        let intervals = vec![vec![1, 2], vec![1, 2], vec![1, 2]];
        assert_eq!(Solution::erase_overlap_intervals(intervals), 2);
    }

    #[test]
    fn example_3_touching_is_fine() {
        let intervals = vec![vec![1, 2], vec![2, 3]];
        assert_eq!(Solution::erase_overlap_intervals(intervals), 0);
    }

    #[test]
    fn empty_input() {
        assert_eq!(Solution::erase_overlap_intervals(vec![]), 0);
    }
}
