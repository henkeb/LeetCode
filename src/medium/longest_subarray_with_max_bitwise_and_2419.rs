// Solution has
// Time complexity: O(?)
// Space complexity: O(?)
//

// You are given an integer array nums of size n.
//
// Consider a non-empty subarray from nums that has the maximum possible bitwise AND.
//
//     In other words, let k be the maximum value of the bitwise AND of any subarray of nums. Then, only subarrays with a bitwise AND equal to k should be considered.
//
// Return the length of the longest such subarray.
//
// The bitwise AND of an array is the bitwise AND of all the numbers in it.
//
// A subarray is a contiguous sequence of elements within an array.
//
//
//
// Example 1:
//
// Input: nums = [1,2,3,3,2,2]
// Output: 2
// Explanation:
// The maximum possible bitwise AND of a subarray is 3.
// The longest subarray with that value is [3,3], so we return 2.
//
// Example 2:
//
// Input: nums = [1,2,3,4]
// Output: 1
// Explanation:
// The maximum possible bitwise AND of a subarray is 4.
// The longest subarray with that value is [4], so we return 1.
//
//
//
// Constraints:
//
//     1 <= nums.length <= 105
//     1 <= nums[i] <= 106
//
struct Solution;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max_element = 0;
        let mut max_count = 0;
        let mut i = 0;
        while i < nums.len() {
            if nums[i] >= max_element {
                let prev_element = max_element;
                max_element = nums[i];
                let mut j = i + 1;
                let mut current_count = 1;
                while j < nums.len() && nums[j] == nums[i] {
                    current_count += 1;
                    j += 1;
                }
                if prev_element == max_element {
                    max_count = std::cmp::max(max_count, current_count);
                } else {
                    max_count = current_count;
                }
                i = j;
            } else {
                i += 1;
            }
        }
        max_count
    }
}
