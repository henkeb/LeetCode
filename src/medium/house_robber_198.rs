// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.
//
// Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.
struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut dp = vec![0; nums.len() + 1];
        dp[1] = nums[0];

        for i in 1..nums.len() {
            dp[i + 1] = std::cmp::max(dp[i], dp[i - 1] + nums[i]);
        }
        dp[nums.len()]
    }
}
