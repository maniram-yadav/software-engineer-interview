//! LeetCode Top Interview 150 — #50 Insert Interval (Medium)
//!
//! Given a sorted, non-overlapping list of intervals and a new interval,
//! insert it and merge as necessary, returning the sorted, non-overlapping
//! result.
//!
//! Example:
//!   Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
//!   Output: [[1,5],[6,9]]

struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let n = intervals.len();
        let mut i = 0;
        let (mut start, mut end) = (new_interval[0], new_interval[1]);

        while i < n && intervals[i][1] < start {
            result.push(intervals[i].clone());
            i += 1;
        }

        while i < n && intervals[i][0] <= end {
            start = start.min(intervals[i][0]);
            end = end.max(intervals[i][1]);
            i += 1;
        }
        result.push(vec![start, end]);

        while i < n {
            result.push(intervals[i].clone());
            i += 1;
        }

        result
    }
}

fn main() {
    let intervals = vec![vec![1, 3], vec![6, 9]];
    println!("{:?}", Solution::insert(intervals, vec![2, 5]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        assert_eq!(
            Solution::insert(intervals, vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
    }

    #[test]
    fn example_2() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        assert_eq!(
            Solution::insert(intervals, vec![4, 8]),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }

    #[test]
    fn insert_into_empty() {
        assert_eq!(
            Solution::insert(vec![], vec![5, 7]),
            vec![vec![5, 7]]
        );
    }

    #[test]
    fn insert_before_all() {
        let intervals = vec![vec![3, 5], vec![6, 9]];
        assert_eq!(
            Solution::insert(intervals, vec![0, 1]),
            vec![vec![0, 1], vec![3, 5], vec![6, 9]]
        );
    }
}
