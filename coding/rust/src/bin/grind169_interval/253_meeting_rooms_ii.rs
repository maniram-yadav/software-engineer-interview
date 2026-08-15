//! Grind 169 — LeetCode #253 Meeting Rooms II (Medium, Premium)
//!
//! Given an array of meeting time intervals, return the minimum number
//! of conference rooms required. Solved by sweeping sorted start and end
//! times independently: each start needs a room, each end frees one; the
//! peak concurrent count is the answer.
//!
//! Example:
//!   Input: intervals = [[0,30],[5,10],[15,20]]
//!   Output: 2

struct Solution;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut starts: Vec<i32> = intervals.iter().map(|iv| iv[0]).collect();
        let mut ends: Vec<i32> = intervals.iter().map(|iv| iv[1]).collect();
        starts.sort_unstable();
        ends.sort_unstable();

        let (mut rooms, mut max_rooms) = (0, 0);
        let (mut i, mut j) = (0, 0);
        while i < starts.len() {
            if starts[i] < ends[j] {
                rooms += 1;
                i += 1;
            } else {
                rooms -= 1;
                j += 1;
            }
            max_rooms = max_rooms.max(rooms);
        }
        max_rooms
    }
}

fn main() {
    let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
    println!("{}", Solution::min_meeting_rooms(intervals));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        assert_eq!(Solution::min_meeting_rooms(intervals), 2);
    }

    #[test]
    fn example_2_no_overlap() {
        let intervals = vec![vec![7, 10], vec![2, 4]];
        assert_eq!(Solution::min_meeting_rooms(intervals), 1);
    }

    #[test]
    fn empty_schedule() {
        assert_eq!(Solution::min_meeting_rooms(vec![]), 0);
    }

    #[test]
    fn all_overlapping() {
        let intervals = vec![vec![1, 10], vec![2, 9], vec![3, 8]];
        assert_eq!(Solution::min_meeting_rooms(intervals), 3);
    }
}
