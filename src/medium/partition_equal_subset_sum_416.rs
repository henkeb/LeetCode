// Given an integer array nums, return true if you can partition the array into two subsets such that the sum of the elements in both subsets is equal or false otherwise.
//
//
//
// Example 1:
//
// Input: nums = [1,5,11,5]
// Output: true
// Explanation: The array can be partitioned as [1, 5, 5] and [11].
// Example 2:
//
// Input: nums = [1,2,3,5]
// Output: false
// Explanation: The array cannot be partitioned into equal sum subsets.
//
//
// Constraints:
//
// 1 <= nums.length <= 200
// 1 <= nums[i] <= 100
struct Solution;
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum & 1 == 1 {
            return false;
        }

        let target = sum as usize / 2;

        // Then, we implement dynamic programming to check for a subset with the target sum S/2.
        let mut dp = vec![false; target + 1];
        dp[0] = true;

        for &num in &nums {
            let num = num as usize;
            for j in (num..=target).rev() {
                dp[j] |= dp[j - num]; // If a subset with sum = target - num exists, then a subset with sum = target also exists.
            }
        }

        dp[target] // The result of this problem is the ability to form a subset with the target sum (S/2).
    }
}
