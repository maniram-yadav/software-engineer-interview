//! Grind 169 — LeetCode #394 Decode String (Medium)
//!
//! Given an encoded string with the pattern k[encoded_string] (repeat
//! encoded_string k times), return the fully decoded string. Solved with
//! two stacks: one for pending repeat counts, one for the string built
//! so far at each nesting level.
//!
//! Example:
//!   Input: s = "3[a]2[bc]"
//!   Output: "aaabcbc"

struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut count_stack: Vec<i32> = Vec::new();
        let mut str_stack: Vec<String> = Vec::new();
        let mut current = String::new();
        let mut num = 0;

        for c in s.chars() {
            if c.is_ascii_digit() {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            } else if c == '[' {
                count_stack.push(num);
                str_stack.push(current.clone());
                current.clear();
                num = 0;
            } else if c == ']' {
                let cnt = count_stack.pop().unwrap();
                let prev = str_stack.pop().unwrap();
                current = prev + &current.repeat(cnt as usize);
            } else {
                current.push(c);
            }
        }

        current
    }
}

fn main() {
    println!(
        "{}",
        Solution::decode_string("3[a]2[bc]".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::decode_string("3[a]2[bc]".to_string()),
            "aaabcbc".to_string()
        );
    }

    #[test]
    fn example_2_nested() {
        assert_eq!(
            Solution::decode_string("3[a2[c]]".to_string()),
            "accaccacc".to_string()
        );
    }

    #[test]
    fn example_3_mixed_with_plain_text() {
        assert_eq!(
            Solution::decode_string("2[abc]3[cd]ef".to_string()),
            "abcabccdcdcdef".to_string()
        );
    }

    #[test]
    fn no_encoding() {
        assert_eq!(
            Solution::decode_string("abc".to_string()),
            "abc".to_string()
        );
    }
}
