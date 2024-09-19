use std::i32;

// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given a list of 24-hour clock time points in "HH:MM" format, return the minimum minutes difference between any two time-points in the list.
//
//
//
// Example 1:
//
// Input: timePoints = ["23:59","00:00"]
// Output: 1
//
// Example 2:
//
// Input: timePoints = ["00:00","23:59","00:00"]
// Output: 0
//
//
//
// Constraints:
//
//     2 <= timePoints.length <= 2 * 104
//     timePoints[i] is in the format "HH:MM".
struct Solution;
impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut time: Vec<i32> = time_points
            .iter()
            .filter_map(|point| {
                if let Some((h, m)) = point.split_once(":") {
                    if let (Some(h), Some(m)) = (h.parse::<i32>().ok(), m.parse::<i32>().ok()) {
                        Some(h * 60 + m)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        time.sort_unstable();
        let mut ans = i32::MAX;
        for i in 1..time.len() {
            ans = std::cmp::min(ans, time[i] - time[i - 1]);
        }
        std::cmp::min(ans, time[0] + (60 * 24 - time[time.len() - 1]))
    }
}
