//! Grind 169 — LeetCode #621 Task Scheduler (Medium)
//!
//! Given a list of CPU tasks (letters) and a cooldown n between two same
//! tasks, return the minimum number of time units (including idle slots)
//! needed to finish all tasks. The most frequent task dictates the frame
//! count: (maxCount-1) frames of size (n+1), plus however many tasks tie
//! for that max frequency; the true answer is never below the total task
//! count (idle-free scheduling is possible when tasks are diverse).
//!
//! Example:
//!   Input: tasks = ["A","A","A","B","B","B"], n = 2
//!   Output: 8   ("A B idle A B idle A B")

struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counts = [0i32; 26];
        for t in &tasks {
            counts[(*t as u8 - b'A') as usize] += 1;
        }
        let max_count = *counts.iter().max().unwrap();
        let max_count_num = counts.iter().filter(|&&c| c == max_count).count() as i32;
        let total = tasks.len() as i32;

        ((max_count - 1) * (n + 1) + max_count_num).max(total)
    }
}

fn main() {
    println!(
        "{}",
        Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
            8
        );
    }

    #[test]
    fn example_2_no_cooldown_needed() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0),
            6
        );
    }

    #[test]
    fn example_3_enough_diversity_to_avoid_idle() {
        let tasks = vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'];
        assert_eq!(Solution::least_interval(tasks, 2), 16);
    }

    #[test]
    fn single_task_type() {
        assert_eq!(Solution::least_interval(vec!['A', 'A'], 2), 5);
    }
}
