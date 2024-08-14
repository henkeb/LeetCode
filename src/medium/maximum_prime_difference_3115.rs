// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// You are given an integer array nums.
//
// Return an integer that is the maximum distance between the indices of two (not necessarily different) prime numbers in nums.
struct Solution;
impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        let mut min: Option<i32> = None;
        let mut max: Option<i32> = None;
        for (i, &num) in nums.iter().enumerate() {
            if Self::is_prime(num) {
                if min.is_none() {
                    min = Some(i as i32);
                } else {
                    max = Some(i as i32);
                }
            }
        }
        match (min, max) {
            (Some(_), None) => 0,
            (Some(min), Some(max)) => max - min,
            _ => unreachable!(),
        }
    }

    fn is_prime(n: i32) -> bool {
        if n == 1 {
            false
        } else if n == 2 || n == 3 || n == 5 || n == 7 {
            true
        } else {
            n % 2 != 0 && n % 3 != 0 && n % 5 != 0 && n % 7 != 0
        }
    }
}
