// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given an integer array nums, move all the even integers at the beginning of the array followed by all the odd integers.
//
// Return any array that satisfies this condition.
//
//
//
// Example 1:
//
// Input: nums = [3,1,2,4]
// Output: [2,4,3,1]
// Explanation: The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.
//
// Example 2:
//
// Input: nums = [0]
// Output: [0]
//
//
//
// Constraints:
//
//     1 <= nums.length <= 5000
//     0 <= nums[i] <= 5000
struct Solution;
impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut left = 0usize;
        let mut right = nums.len() - 1;
        while left != right {
            if nums[left] & 1 == 1 {
                nums.swap(left, right);
                right -= 1;
            } else {
                left += 1;
            }
        }
        nums
    }
}
