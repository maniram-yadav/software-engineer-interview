//! LeetCode Top Interview 150 — #14 Gas Station (Medium)
//!
//! There are n gas stations in a circle. `gas[i]` is the fuel available at
//! station i, `cost[i]` is the fuel to travel from station i to i+1.
//! Return the starting station index from which the circuit can be
//! completed, or -1 if impossible (the answer is unique if it exists).
//!
//! Example:
//!   Input: gas = [1,2,3,4,5], cost = [3,4,5,1,2]
//!   Output: 3

struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut tank = 0;
        let mut start = 0usize;
        for i in 0..gas.len() {
            let diff = gas[i] - cost[i];
            total += diff;
            tank += diff;
            if tank < 0 {
                start = i + 1;
                tank = 0;
            }
        }
        if total >= 0 {
            start as i32
        } else {
            -1
        }
    }
}

fn main() {
    let gas = vec![1, 2, 3, 4, 5];
    let cost = vec![3, 4, 5, 1, 2];
    println!("start index: {}", Solution::can_complete_circuit(gas, cost));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
    }

    #[test]
    fn example_2_impossible() {
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }

    #[test]
    fn single_station_possible() {
        assert_eq!(Solution::can_complete_circuit(vec![5], vec![4]), 0);
    }

    #[test]
    fn single_station_impossible() {
        assert_eq!(Solution::can_complete_circuit(vec![1], vec![2]), -1);
    }
}
