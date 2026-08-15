//! LeetCode Top Interview 150 — #40 Isomorphic Strings (Easy)
//!
//! Given strings `s` and `t`, determine if they are isomorphic —
//! characters in `s` can be replaced to get `t`, with a consistent
//! one-to-one (bijective) mapping in both directions.
//!
//! Example:
//!   Input: s = "egg", t = "add"
//!   Output: true

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map_st: HashMap<char, char> = HashMap::new();
        let mut map_ts: HashMap<char, char> = HashMap::new();

        for (cs, ct) in s.chars().zip(t.chars()) {
            match (map_st.get(&cs), map_ts.get(&ct)) {
                (Some(&mapped), _) if mapped != ct => return false,
                (_, Some(&mapped)) if mapped != cs => return false,
                _ => {
                    map_st.insert(cs, ct);
                    map_ts.insert(ct, cs);
                }
            }
        }

        true
    }
}

fn main() {
    println!(
        "{}",
        Solution::is_isomorphic("egg".to_string(), "add".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::is_isomorphic("egg".to_string(), "add".to_string()),
            true
        );
    }

    #[test]
    fn example_2_not_isomorphic() {
        assert_eq!(
            Solution::is_isomorphic("foo".to_string(), "bar".to_string()),
            false
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::is_isomorphic("paper".to_string(), "title".to_string()),
            true
        );
    }

    #[test]
    fn not_injective_mapping() {
        // Both 'a' and 'b' would map to 'a', which breaks the bijection.
        assert_eq!(
            Solution::is_isomorphic("badc".to_string(), "baba".to_string()),
            false
        );
    }
}
