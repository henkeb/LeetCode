// You are given an integer array nums. The absolute sum of a subarray [numsl, numsl+1, ..., numsr-1, numsr] is abs(numsl + numsl+1 + ... + numsr-1 + numsr).
//
// Return the maximum absolute sum of any (possibly empty) subarray of nums.
//
// Note that abs(x) is defined as follows:
//
// If x is a negative integer, then abs(x) = -x.
// If x is a non-negative integer, then abs(x) = x.
//
//
// Example 1:
//
// Input: nums = [1,-3,2,3,-4]
// Output: 5
// Explanation: The subarray [2,3] has absolute sum = abs(2+3) = abs(5) = 5.
// Example 2:
//
// Input: nums = [2,-5,1,-4,3,-2]
// Output: 8
// Explanation: The subarray [-5,1,-4] has absolute sum = abs(-5+1-4) = abs(-8) = 8.
//
//
// Constraints:
//
// 1 <= nums.length <= 105
// -104 <= nums[i] <= 104
//
// Solution has
// Time complexity: O(n)
// Space complexity: O(1)

struct Solution;
impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut max_output = nums[0];
        let mut max_sub_arr = nums[0];
        let mut min_output = nums[0];
        let mut min_sub_arr = nums[0];

        // Kadane's algorithm
        for i in 1..nums.len() {
            max_sub_arr = std::cmp::max(nums[i], nums[i] + max_sub_arr);
            min_sub_arr = std::cmp::min(nums[i], nums[i] + min_sub_arr);

            max_output = std::cmp::max(max_output, max_sub_arr);
            min_output = std::cmp::min(min_output, min_sub_arr);
        }
        std::cmp::max(max_output.abs(), min_output.abs())
    }
}
