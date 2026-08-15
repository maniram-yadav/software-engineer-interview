//! LeetCode Top Interview 150 — #49 Merge Intervals (Medium)
//!
//! Given an array of intervals, merge all overlapping intervals and
//! return the non-overlapping intervals covering all input intervals.
//!
//! Example:
//!   Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
//!   Output: [[1,6],[8,10],[15,18]]

struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|iv| iv[0]);
        let mut result: Vec<Vec<i32>> = Vec::new();

        for iv in intervals {
            if let Some(last) = result.last_mut() {
                if iv[0] <= last[1] {
                    last[1] = last[1].max(iv[1]);
                    continue;
                }
            }
            result.push(iv);
        }

        result
    }
}

fn main() {
    let intervals = vec![
        vec![1, 3],
        vec![2, 6],
        vec![8, 10],
        vec![15, 18],
    ];
    println!("{:?}", Solution::merge(intervals));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(
            Solution::merge(intervals),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn example_2_touching_intervals() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        assert_eq!(Solution::merge(intervals), vec![vec![1, 5]]);
    }

    #[test]
    fn no_overlap() {
        let intervals = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::merge(intervals), vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn unsorted_input() {
        let intervals = vec![vec![5, 6], vec![1, 3], vec![2, 4]];
        assert_eq!(Solution::merge(intervals), vec![vec![1, 4], vec![5, 6]]);
    }
}
