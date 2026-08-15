//! Grind 169 — LeetCode #278 First Bad Version (Easy)
//!
//! You have n versions and want to find the first bad one, given an API
//! isBadVersion(version). Minimize the number of calls. LeetCode's
//! actual signature calls a global `isBadVersion`; adapted here to take
//! the check as a closure so it's directly testable without global state.
//!
//! Example:
//!   Input: n = 5, bad = 4
//!   Output: 4

struct Solution;

impl Solution {
    pub fn first_bad_version(n: i32, is_bad_version: impl Fn(i32) -> bool) -> i32 {
        let (mut lo, mut hi) = (1i32, n);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if is_bad_version(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}

fn main() {
    println!("{}", Solution::first_bad_version(5, |v| v >= 4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::first_bad_version(5, |v| v >= 4), 4);
    }

    #[test]
    fn example_2_first_version_is_bad() {
        assert_eq!(Solution::first_bad_version(1, |v| v >= 1), 1);
    }

    #[test]
    fn last_version_is_first_bad() {
        assert_eq!(Solution::first_bad_version(10, |v| v >= 10), 10);
    }

    #[test]
    fn large_n() {
        assert_eq!(
            Solution::first_bad_version(2_147_483_647, |v| v >= 1_702_766_719),
            1_702_766_719
        );
    }
}
