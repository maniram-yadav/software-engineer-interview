//! LeetCode Top Interview 150 — #11 H-Index (Medium)
//!
//! Given an array `citations` where `citations[i]` is the number of
//! citations for the researcher's i-th paper, return the h-index: the max
//! h such that at least h papers have >= h citations each.
//!
//! Example:
//!   Input: citations = [3,0,6,1,5]
//!   Output: 3

struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable_by(|a, b| b.cmp(a));
        let mut h = 0;
        for (i, &c) in citations.iter().enumerate() {
            if c >= (i as i32 + 1) {
                h = i as i32 + 1;
            } else {
                break;
            }
        }
        h
    }
}

fn main() {
    println!("h-index: {}", Solution::h_index(vec![3, 0, 6, 1, 5]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
    }

    #[test]
    fn all_zero_citations() {
        assert_eq!(Solution::h_index(vec![0, 0, 0]), 0);
    }

    #[test]
    fn single_paper_highly_cited() {
        assert_eq!(Solution::h_index(vec![100]), 1);
    }
}
