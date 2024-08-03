// Solution has
// Time complexity: O(n*target)
// Space complexity: O(target)
//
// Given an array of distinct integers nums and a target integer target, return the number of possible combinations that add up to target.
//
// The test cases are generated so that the answer can fit in a 32-bit integer.
struct Solution;
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        for i in 1..=target {
            for &num in nums.iter() {
                if i - num >= 0 {
                    dp[i as usize] += dp[(i - num) as usize];
                }
            }
        }
        dp[target as usize]
    }
}
