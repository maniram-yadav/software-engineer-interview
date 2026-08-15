//! LeetCode Top Interview 150 — #54 Min Stack (Medium)
//!
//! Design a stack supporting push, pop, top, and get_min — all in O(1)
//! time. Solved with a companion stack that tracks the running minimum
//! alongside each pushed value.
//!
//! Example:
//!   push(-2); push(0); push(-3);
//!   getMin() -> -3
//!   pop();
//!   top() -> 0
//!   getMin() -> -2

struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        let min = match self.min_stack.last() {
            Some(&m) => m.min(val),
            None => val,
        };
        self.min_stack.push(min);
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

fn main() {
    let mut ms = MinStack::new();
    ms.push(-2);
    ms.push(0);
    ms.push(-3);
    println!("min: {}", ms.get_min());
    ms.pop();
    println!("top: {}", ms.top());
    println!("min: {}", ms.get_min());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut ms = MinStack::new();
        ms.push(-2);
        ms.push(0);
        ms.push(-3);
        assert_eq!(ms.get_min(), -3);
        ms.pop();
        assert_eq!(ms.top(), 0);
        assert_eq!(ms.get_min(), -2);
    }

    #[test]
    fn min_updates_after_multiple_pops() {
        let mut ms = MinStack::new();
        ms.push(5);
        ms.push(3);
        ms.push(7);
        ms.push(1);
        assert_eq!(ms.get_min(), 1);
        ms.pop();
        assert_eq!(ms.get_min(), 3);
        ms.pop();
        assert_eq!(ms.get_min(), 3);
        ms.pop();
        assert_eq!(ms.get_min(), 5);
    }

    #[test]
    fn single_element() {
        let mut ms = MinStack::new();
        ms.push(42);
        assert_eq!(ms.top(), 42);
        assert_eq!(ms.get_min(), 42);
    }

    #[test]
    fn duplicate_minimums() {
        let mut ms = MinStack::new();
        ms.push(1);
        ms.push(1);
        ms.push(1);
        assert_eq!(ms.get_min(), 1);
        ms.pop();
        assert_eq!(ms.get_min(), 1);
    }
}
