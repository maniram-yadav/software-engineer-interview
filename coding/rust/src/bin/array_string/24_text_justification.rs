//! LeetCode Top Interview 150 — #24 Text Justification (Hard)
//!
//! Given an array of words and a width `max_width`, format the text so
//! each line has exactly `max_width` characters, fully justified (extra
//! spaces distributed as evenly as possible, left-heavy); the last line
//! and any line with a single word are left-justified with single spaces.
//!
//! Example:
//!   Input: words = ["This","is","an","example","of","text","justification."], maxWidth = 16
//!   Output: ["This    is    an","example  of text","justification.  "]

struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let n = words.len();
        let mut result = Vec::new();
        let mut i = 0;

        while i < n {
            let mut j = i;
            let mut line_len = 0;
            while j < n && line_len + words[j].len() + (j - i) <= max_width {
                line_len += words[j].len();
                j += 1;
            }
            let num_words = j - i;
            let mut line = String::new();

            if j == n || num_words == 1 {
                for k in i..j {
                    if k > i {
                        line.push(' ');
                    }
                    line.push_str(&words[k]);
                }
                while line.len() < max_width {
                    line.push(' ');
                }
            } else {
                let total_spaces = max_width - line_len;
                let gaps = num_words - 1;
                let space_each = total_spaces / gaps;
                let extra = total_spaces % gaps;
                for k in i..j {
                    line.push_str(&words[k]);
                    if k < j - 1 {
                        let mut spaces = space_each;
                        if (k - i) < extra {
                            spaces += 1;
                        }
                        for _ in 0..spaces {
                            line.push(' ');
                        }
                    }
                }
            }

            result.push(line);
            i = j;
        }

        result
    }
}

fn main() {
    let words = ["This", "is", "an", "example", "of", "text", "justification."]
        .iter()
        .map(|s| s.to_string())
        .collect();
    for line in Solution::full_justify(words, 16) {
        println!("{:?}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_1() {
        let words = v(&[
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification.",
        ]);
        assert_eq!(
            Solution::full_justify(words, 16),
            v(&[
                "This    is    an",
                "example  of text",
                "justification.  "
            ])
        );
    }

    #[test]
    fn example_2() {
        let words = v(&["What", "must", "be", "acknowledgment", "shall", "be"]);
        assert_eq!(
            Solution::full_justify(words, 16),
            v(&["What   must   be", "acknowledgment  ", "shall be        "])
        );
    }

    #[test]
    fn single_word_line_is_left_justified() {
        let words = v(&["Listen"]);
        assert_eq!(
            Solution::full_justify(words, 10),
            v(&["Listen    "])
        );
    }
}
