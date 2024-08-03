// Solution has
// Time complexity: O(nlogn)
// Space complexity: O(1)
//
// You are given a 0-indexed integer array nums. You are allowed to permute nums into a new array perm of your choosing.
//
// We define the greatness of nums be the number of indices 0 <= i < nums.length for which perm[i] > nums[i].
//
// Return the maximum possible greatness you can achieve after permuting nums.
struct Solution;

impl Solution {
    pub fn maximize_greatness(mut nums: Vec<i32>) -> i32 {
        let mut score = 0;
        let mut i = 0;
        let mut j = 0;
        let len = nums.len();
        nums.sort_unstable();
        while j < len {
            if nums[i] < nums[j] {
                i += 1;
                score += 1;
            }
            j += 1;
        }
        score
    }
}
