//! Grind 169 — LeetCode #252 Meeting Rooms (Easy, Premium)
//!
//! Given an array of meeting time intervals, determine if a person could
//! attend all meetings (no overlaps). Sort by start time; any interval
//! starting before the previous one ends is a conflict.
//!
//! Example:
//!   Input: intervals = [[0,30],[5,10],[15,20]]
//!   Output: false

struct Solution;

impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_unstable_by_key(|iv| iv[0]);
        for w in intervals.windows(2) {
            if w[1][0] < w[0][1] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
    println!("{}", Solution::can_attend_meetings(intervals));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        assert_eq!(Solution::can_attend_meetings(intervals), false);
    }

    #[test]
    fn example_2_no_overlap() {
        let intervals = vec![vec![7, 10], vec![2, 4]];
        assert_eq!(Solution::can_attend_meetings(intervals), true);
    }

    #[test]
    fn touching_intervals_are_ok() {
        let intervals = vec![vec![1, 5], vec![5, 10]];
        assert_eq!(Solution::can_attend_meetings(intervals), true);
    }

    #[test]
    fn empty_schedule() {
        assert_eq!(Solution::can_attend_meetings(vec![]), true);
    }
}
