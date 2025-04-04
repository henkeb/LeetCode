// Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
//
// You must write an algorithm that runs in O(n) time.
//
//
//
// Example 1:
//
// Input: nums = [100,4,200,1,3,2]
// Output: 4
// Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
// Example 2:
//
// Input: nums = [0,3,7,2,5,8,4,6,0,1]
// Output: 9
// Example 3:
//
// Input: nums = [1,0,1,2]
// Output: 3
//
//
// Constraints:
//
// 0 <= nums.length <= 105
// -109 <= nums[i] <= 109
//
// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let set: HashSet<i32> = nums.into_iter().collect();
        let mut longest = 0;

        for num in set.iter() {
            if !set.contains(&(num - 1)) {
                let mut count = 1;
                while set.contains(&(num + count)) {
                    count += 1;
                }
                longest = longest.max(count);
            }
        }
        longest as i32
    }
}
