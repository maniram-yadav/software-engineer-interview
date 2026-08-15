//! Grind 169 — LeetCode #232 Implement Queue using Stacks (Easy)
//!
//! Implement a first-in-first-out (FIFO) queue using only two stacks.
//! `in_stack` accepts pushes; when `out_stack` runs dry, everything is
//! poured over from `in_stack`, reversing it into FIFO order.
//!
//! Example:
//!   push(1); push(2); peek() -> 1; pop() -> 1; empty() -> false

struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            in_stack: Vec::new(),
            out_stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.in_stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.peek();
        self.out_stack.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.out_stack.is_empty() {
            while let Some(v) = self.in_stack.pop() {
                self.out_stack.push(v);
            }
        }
        *self.out_stack.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.in_stack.is_empty() && self.out_stack.is_empty()
    }
}

fn main() {
    let mut q = MyQueue::new();
    q.push(1);
    q.push(2);
    println!("peek: {}", q.peek());
    println!("pop: {}", q.pop());
    println!("empty: {}", q.empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert_eq!(q.empty(), false);
    }

    #[test]
    fn fifo_order_preserved() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        q.push(3);
        assert_eq!(q.pop(), 1);
        assert_eq!(q.pop(), 2);
        assert_eq!(q.pop(), 3);
        assert_eq!(q.empty(), true);
    }

    #[test]
    fn interleaved_push_pop() {
        let mut q = MyQueue::new();
        q.push(1);
        assert_eq!(q.pop(), 1);
        q.push(2);
        q.push(3);
        assert_eq!(q.pop(), 2);
        q.push(4);
        assert_eq!(q.pop(), 3);
        assert_eq!(q.pop(), 4);
    }
}
