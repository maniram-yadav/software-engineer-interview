//! Grind 169 — LeetCode #84 Largest Rectangle in Histogram (Hard)
//!
//! Given an array of bar heights of a histogram (width 1 each), return
//! the area of the largest rectangle that fits within it. Solved with a
//! monotonic increasing stack of indices: when a shorter bar is found,
//! the popped bar's maximal rectangle width spans from the new stack top
//! (exclusive) to the current index (exclusive).
//!
//! Example:
//!   Input: heights = [2,1,5,6,2,3]
//!   Output: 10

struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut max_area = 0;
        let n = heights.len();

        for i in 0..=n {
            let h = if i == n { 0 } else { heights[i] };
            while let Some(&top) = stack.last() {
                if h < heights[top] {
                    stack.pop();
                    let height = heights[top];
                    let width = match stack.last() {
                        Some(&new_top) => (i - new_top - 1) as i32,
                        None => i as i32,
                    };
                    max_area = max_area.max(height * width);
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        max_area
    }
}

fn main() {
    println!(
        "{}",
        Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]),
            10
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }

    #[test]
    fn single_bar() {
        assert_eq!(Solution::largest_rectangle_area(vec![5]), 5);
    }

    #[test]
    fn all_same_height() {
        assert_eq!(Solution::largest_rectangle_area(vec![3, 3, 3]), 9);
    }
}
