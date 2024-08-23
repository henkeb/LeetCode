// Solution has
// Time complexity: O(n²)
// Space complexity: O(n)
//
// Given an integer array nums, return the length of the longest strictly increasing
// subsequence
// .
//
//
//
// Example 1:
//
// Input: nums = [10,9,2,5,3,7,101,18]
// Output: 4
// Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
//
// Example 2:
//
// Input: nums = [0,1,0,3,2,3]
// Output: 4
//
// Example 3:
//
// Input: nums = [7,7,7,7,7,7,7]
// Output: 1
//
//
//
// Constraints:
//
//     1 <= nums.length <= 2500
//     -104 <= nums[i] <= 104
struct Solution;
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = std::cmp::max(dp[i], dp[j] + 1);
                }
            }
        }
        *dp.iter().max().unwrap_or(&0)
    }
}
