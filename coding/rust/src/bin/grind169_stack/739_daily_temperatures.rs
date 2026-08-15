//! Grind 169 — LeetCode #739 Daily Temperatures (Medium)
//!
//! Given an array of daily temperatures, return an array answer where
//! answer[i] is the number of days you'd have to wait for a warmer
//! temperature; 0 if none. Solved with a monotonic decreasing stack of
//! indices awaiting a warmer day.
//!
//! Example:
//!   Input: temperatures = [73,74,75,71,69,72,76,73]
//!   Output: [1,1,4,2,1,1,0,0]

struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut result = vec![0; n];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..n {
            while let Some(&top) = stack.last() {
                if temperatures[i] > temperatures[top] {
                    result[top] = (i - top) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn example_2_strictly_decreasing() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
    }

    #[test]
    fn example_3_never_gets_warmer() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }

    #[test]
    fn single_day() {
        assert_eq!(Solution::daily_temperatures(vec![70]), vec![0]);
    }
}
