//! Grind 169 — LeetCode #1235 Maximum Profit in Job Scheduling (Hard)
//!
//! Given startTime, endTime, and profit arrays for jobs, find the
//! maximum profit achievable by scheduling non-overlapping jobs. Sort
//! jobs by end time; dp is a list of (end_time, best_profit_by_then)
//! pairs sorted by end_time, and each job's best profit is found via
//! binary search for the latest end_time <= this job's start_time.
//!
//! Example:
//!   Input: startTime = [1,2,3,3], endTime = [3,4,5,6], profit = [50,10,40,70]
//!   Output: 120

struct Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut jobs: Vec<(i32, i32, i32)> = (0..n)
            .map(|i| (end_time[i], start_time[i], profit[i]))
            .collect();
        jobs.sort_unstable();

        let mut dp: Vec<(i32, i32)> = vec![(0, 0)];
        for (end, start, p) in jobs {
            let idx = dp.partition_point(|&(e, _)| e <= start) - 1;
            let cur_profit = dp[idx].1 + p;
            if cur_profit > dp.last().unwrap().1 {
                dp.push((end, cur_profit));
            }
        }

        dp.last().unwrap().1
    }
}

fn main() {
    println!(
        "{}",
        Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
            120
        );
    }

    #[test]
    fn example_2_all_fit() {
        assert_eq!(
            Solution::job_scheduling(
                vec![1, 2, 3, 4, 6],
                vec![3, 5, 10, 6, 9],
                vec![20, 20, 100, 70, 60]
            ),
            150
        );
    }

    #[test]
    fn example_3_all_overlap_pick_best_single() {
        assert_eq!(
            Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]),
            6
        );
    }

    #[test]
    fn single_job() {
        assert_eq!(
            Solution::job_scheduling(vec![1], vec![2], vec![10]),
            10
        );
    }
}
