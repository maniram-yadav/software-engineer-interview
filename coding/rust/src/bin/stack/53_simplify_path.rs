//! LeetCode Top Interview 150 — #53 Simplify Path (Medium)
//!
//! Given an absolute Unix-style file path, simplify it to its canonical
//! form (resolve `.`, `..`, and redundant slashes).
//!
//! Example:
//!   Input: path = "/a/./b/../../c/"
//!   Output: "/c"

struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = Vec::new();
        for part in path.split('/') {
            match part {
                "" | "." => continue,
                ".." => {
                    stack.pop();
                }
                _ => stack.push(part),
            }
        }
        format!("/{}", stack.join("/"))
    }
}

fn main() {
    println!(
        "{}",
        Solution::simplify_path("/a/./b/../../c/".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::simplify_path("/home/".to_string()),
            "/home".to_string()
        );
    }

    #[test]
    fn example_2_dotdot_at_root() {
        assert_eq!(
            Solution::simplify_path("/../".to_string()),
            "/".to_string()
        );
    }

    #[test]
    fn example_3_redundant_slashes() {
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_string()),
            "/home/foo".to_string()
        );
    }

    #[test]
    fn example_4_mixed() {
        assert_eq!(
            Solution::simplify_path("/a/./b/../../c/".to_string()),
            "/c".to_string()
        );
    }

    #[test]
    fn root_only() {
        assert_eq!(Solution::simplify_path("/".to_string()), "/".to_string());
    }
}
