//! Grind 169 — LeetCode #735 Asteroid Collision (Medium)
//!
//! Given an array of asteroids (sign = direction, magnitude = size)
//! moving in a row, simulate collisions (larger survives, equal both
//! explode) and return the state after all collisions. A stack holds
//! surviving asteroids moving right; a new left-moving asteroid resolves
//! collisions against the stack's top before either exploding or
//! settling.
//!
//! Example:
//!   Input: asteroids = [5,10,-5]
//!   Output: [5,10]

struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();

        for a in asteroids {
            let mut a = a;
            let mut alive = true;
            while alive && a < 0 && !stack.is_empty() && *stack.last().unwrap() > 0 {
                let top = *stack.last().unwrap();
                if top < -a {
                    stack.pop();
                } else if top == -a {
                    stack.pop();
                    alive = false;
                } else {
                    alive = false;
                }
            }
            if alive {
                stack.push(a);
            }
        }

        stack
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::asteroid_collision(vec![5, 10, -5])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::asteroid_collision(vec![5, 10, -5]),
            vec![5, 10]
        );
    }

    #[test]
    fn example_2_mutual_destruction() {
        assert_eq!(
            Solution::asteroid_collision(vec![8, -8]),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn example_3_larger_survives() {
        assert_eq!(
            Solution::asteroid_collision(vec![10, 2, -5]),
            vec![10]
        );
    }

    #[test]
    fn same_direction_no_collision() {
        assert_eq!(
            Solution::asteroid_collision(vec![1, 2, 3]),
            vec![1, 2, 3]
        );
    }
}
