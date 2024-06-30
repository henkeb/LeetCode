// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Our hero Teemo is attacking an enemy Ashe with poison attacks! When Teemo attacks Ashe, Ashe gets poisoned for a exactly duration seconds. More formally, an attack at second t will mean Ashe is poisoned during the inclusive time interval [t, t + duration - 1]. If Teemo attacks again before the poison effect ends, the timer for it is reset, and the poison effect will end duration seconds after the new attack.
//
// You are given a non-decreasing integer array timeSeries, where timeSeries[i] denotes that Teemo attacks Ashe at second timeSeries[i], and an integer duration.
//
// Return the total number of seconds that Ashe is poisoned.
struct Solution;
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        use std::cmp::Ordering;
        (1..time_series.len()).fold(0, |acc, i| {
            let dt = time_series[i] - time_series[i - 1];
            match dt.cmp(&duration) {
                Ordering::Greater => acc + duration,
                Ordering::Equal => acc + duration,
                Ordering::Less => acc + dt,
            }
        }) + duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 4], 2), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 2], 2), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::find_poisoned_duration(vec![1], 2), 2);
    }
}
