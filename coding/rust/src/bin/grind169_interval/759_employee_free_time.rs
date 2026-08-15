//! Grind 169 — LeetCode #759 Employee Free Time (Hard, Premium)
//!
//! Given a list of schedules (each employee's list of non-overlapping
//! intervals, sorted), return the list of finite intervals representing
//! common, positive-length free time for all employees. Flatten every
//! employee's intervals into one list, sort by start, then scan for gaps
//! between the running merged-end and the next interval's start.
//!
//! Example:
//!   Input: schedule = [[[1,2],[5,6]],[[1,3]],[[4,10]]]
//!   Output: [[3,4]]

#[derive(Debug, Clone, PartialEq, Eq)]
struct Interval {
    start: i32,
    end: i32,
}

struct Solution;

impl Solution {
    pub fn employee_free_time(schedule: Vec<Vec<Interval>>) -> Vec<Interval> {
        let mut all: Vec<Interval> = schedule.into_iter().flatten().collect();
        if all.is_empty() {
            return vec![];
        }
        all.sort_by_key(|iv| iv.start);

        let mut result = Vec::new();
        let mut end = all[0].end;
        for iv in &all[1..] {
            if iv.start > end {
                result.push(Interval {
                    start: end,
                    end: iv.start,
                });
            }
            end = end.max(iv.end);
        }
        result
    }
}

fn iv(start: i32, end: i32) -> Interval {
    Interval { start, end }
}

fn main() {
    let schedule = vec![
        vec![iv(1, 2), iv(5, 6)],
        vec![iv(1, 3)],
        vec![iv(4, 10)],
    ];
    println!("{:?}", Solution::employee_free_time(schedule));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let schedule = vec![
            vec![iv(1, 2), iv(5, 6)],
            vec![iv(1, 3)],
            vec![iv(4, 10)],
        ];
        assert_eq!(Solution::employee_free_time(schedule), vec![iv(3, 4)]);
    }

    #[test]
    fn example_2_multiple_gaps() {
        let schedule = vec![vec![iv(1, 3), iv(6, 7)], vec![iv(2, 4)], vec![iv(2, 5), iv(9, 12)]];
        assert_eq!(
            Solution::employee_free_time(schedule),
            vec![iv(5, 6), iv(7, 9)]
        );
    }

    #[test]
    fn no_free_time() {
        let schedule = vec![vec![iv(1, 5)], vec![iv(2, 6)]];
        assert_eq!(Solution::employee_free_time(schedule), vec![]);
    }

    #[test]
    fn empty_schedule() {
        assert_eq!(Solution::employee_free_time(vec![]), vec![]);
    }
}
