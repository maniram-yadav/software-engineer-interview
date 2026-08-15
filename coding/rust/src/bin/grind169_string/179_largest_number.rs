//! Grind 169 — LeetCode #179 Largest Number (Medium)
//!
//! Given a list of non-negative integers, arrange them so they form the
//! largest possible number, returned as a string. Sort by a custom
//! comparator: a should come before b when "a+b" > "b+a" as strings.
//!
//! Example:
//!   Input: nums = [3,30,34,5,9]
//!   Output: "9534330"

struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut strs: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
        strs.sort_by(|a, b| {
            let ab = format!("{}{}", a, b);
            let ba = format!("{}{}", b, a);
            ba.cmp(&ab)
        });

        if strs[0] == "0" {
            return "0".to_string();
        }
        strs.concat()
    }
}

fn main() {
    println!(
        "{}",
        Solution::largest_number(vec![3, 30, 34, 5, 9])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::largest_number(vec![10, 2]),
            "210".to_string()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::largest_number(vec![3, 30, 34, 5, 9]),
            "9534330".to_string()
        );
    }

    #[test]
    fn all_zeros() {
        assert_eq!(
            Solution::largest_number(vec![0, 0]),
            "0".to_string()
        );
    }

    #[test]
    fn single_number() {
        assert_eq!(
            Solution::largest_number(vec![5]),
            "5".to_string()
        );
    }
}
