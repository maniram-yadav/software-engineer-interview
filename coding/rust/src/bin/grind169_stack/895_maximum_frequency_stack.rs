//! Grind 169 — LeetCode #895 Maximum Frequency Stack (Hard)
//!
//! Design a stack-like data structure FreqStack where pop() removes and
//! returns the most frequent element, breaking ties by most recently
//! pushed. Solved by tracking each value's frequency, and grouping
//! values by frequency into their own stacks (so the most-recently-added
//! value at the max frequency is always the top of `group[max_freq]`).
//!
//! Example:
//!   push(5); push(7); push(5); push(7); push(4); push(5);
//!   pop() -> 5

use std::collections::HashMap;

struct FreqStack {
    freq: HashMap<i32, i32>,
    group: HashMap<i32, Vec<i32>>,
    max_freq: i32,
}

impl FreqStack {
    fn new() -> Self {
        FreqStack {
            freq: HashMap::new(),
            group: HashMap::new(),
            max_freq: 0,
        }
    }

    fn push(&mut self, val: i32) {
        let f = *self.freq.get(&val).unwrap_or(&0) + 1;
        self.freq.insert(val, f);
        if f > self.max_freq {
            self.max_freq = f;
        }
        self.group.entry(f).or_insert_with(Vec::new).push(val);
    }

    fn pop(&mut self) -> i32 {
        let val = self.group.get_mut(&self.max_freq).unwrap().pop().unwrap();
        *self.freq.get_mut(&val).unwrap() -= 1;
        if self.group[&self.max_freq].is_empty() {
            self.max_freq -= 1;
        }
        val
    }
}

fn main() {
    let mut fs = FreqStack::new();
    for v in [5, 7, 5, 7, 4, 5] {
        fs.push(v);
    }
    println!("{}", fs.pop());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut fs = FreqStack::new();
        for v in [5, 7, 5, 7, 4, 5] {
            fs.push(v);
        }
        assert_eq!(fs.pop(), 5);
        assert_eq!(fs.pop(), 7);
        assert_eq!(fs.pop(), 5);
        assert_eq!(fs.pop(), 4);
    }

    #[test]
    fn ties_broken_by_recency() {
        let mut fs = FreqStack::new();
        fs.push(1);
        fs.push(2);
        assert_eq!(fs.pop(), 2);
        assert_eq!(fs.pop(), 1);
    }

    #[test]
    fn single_element() {
        let mut fs = FreqStack::new();
        fs.push(9);
        assert_eq!(fs.pop(), 9);
    }

    #[test]
    fn frequency_recomputed_after_pop() {
        let mut fs = FreqStack::new();
        fs.push(1);
        fs.push(1);
        fs.push(2);
        assert_eq!(fs.pop(), 1); // freq 2, most frequent
        assert_eq!(fs.pop(), 2); // freq 1, tie with the remaining 1, but 2 pushed later
    }
}
