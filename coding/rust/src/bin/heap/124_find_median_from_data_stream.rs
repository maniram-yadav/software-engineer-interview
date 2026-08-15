//! LeetCode Top Interview 150 — #124 Find Median from Data Stream (Hard)
//!
//! Design a data structure that supports adding integers from a stream
//! and finding the median of all elements so far efficiently. Solved
//! with two heaps: a max-heap `low` holding the smaller half, and a
//! min-heap `high` holding the larger half, rebalanced after every
//! insertion so `low` has either the same count as `high` or exactly one
//! more.
//!
//! Example:
//!   addNum(1); addNum(2);
//!   findMedian(); // 1.5
//!   addNum(3);
//!   findMedian(); // 2.0

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    low: BinaryHeap<i32>,
    high: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            low: BinaryHeap::new(),
            high: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.low.push(num);
        let moved = self.low.pop().unwrap();
        self.high.push(Reverse(moved));

        if self.high.len() > self.low.len() {
            let Reverse(back) = self.high.pop().unwrap();
            self.low.push(back);
        }
    }

    fn find_median(&self) -> f64 {
        if self.low.len() > self.high.len() {
            *self.low.peek().unwrap() as f64
        } else {
            let l = *self.low.peek().unwrap() as f64;
            let h = self.high.peek().unwrap().0 as f64;
            (l + h) / 2.0
        }
    }
}

fn main() {
    let mut mf = MedianFinder::new();
    mf.add_num(1);
    mf.add_num(2);
    println!("{}", mf.find_median());
    mf.add_num(3);
    println!("{}", mf.find_median());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);
    }

    #[test]
    fn single_value() {
        let mut mf = MedianFinder::new();
        mf.add_num(5);
        assert_eq!(mf.find_median(), 5.0);
    }

    #[test]
    fn descending_insertion_order() {
        let mut mf = MedianFinder::new();
        for n in [5, 4, 3, 2, 1] {
            mf.add_num(n);
        }
        assert_eq!(mf.find_median(), 3.0);
    }

    #[test]
    fn even_count_averages_middle_two() {
        let mut mf = MedianFinder::new();
        for n in [1, 2, 3, 4] {
            mf.add_num(n);
        }
        assert_eq!(mf.find_median(), 2.5);
    }
}
