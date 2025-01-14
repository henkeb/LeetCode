use std::collections::HashMap;

// Solution has
// Time complexity: O(?)
// Space complexity: O(?)
//
// Given an array of positive integers nums, remove the smallest subarray (possibly empty) such that the sum of the remaining elements is divisible by p. It is not allowed to remove the whole array.
//
// Return the length of the smallest subarray that you need to remove, or -1 if it's impossible.
//
// A subarray is defined as a contiguous block of elements in the array.
//
//
//
// Example 1:
//
// Input: nums = [3,1,4,2], p = 6
// Output: 1
// Explanation: The sum of the elements in nums is 10, which is not divisible by 6. We can remove the subarray [4], and the sum of the remaining elements is 6, which is divisible by 6.
// Example 2:
//
// Input: nums = [6,3,5,2], p = 9
// Output: 2
// Explanation: We cannot remove a single element to get a sum divisible by 9. The best way is to remove the subarray [5,2], leaving us with [6,3] with sum 9.
// Example 3:
//
// Input: nums = [1,2,3], p = 3
// Output: 0
// Explanation: Here the sum is 6. which is already divisible by 3. Thus we do not need to remove anything.
//
//
// Constraints:
//
// 1 <= nums.length <= 105
// 1 <= nums[i] <= 109
// 1 <= p <= 109
struct Solution;
impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let sum: i64 = nums.iter().map(|val| *val as i64).sum();
        let p = p as i64;
        let remainder = sum % p;

        if remainder == 0 {
            return 0;
        }

        // Find shortest subarray which sums up to remainder
        // Calculate prefix sum
        let mut prefix_sum = vec![0; nums.len()];
        let _ = nums.iter().enumerate().scan(0, |state, (i, value)| {
            *state += *value;
            prefix_sum[i] = *state;
            Some(*state)
        });

        let mut mod_map: HashMap<i64, i32> = HashMap::new();
        mod_map.insert(0, -1);
        let mut current_sum: i64 = 0;
        let mut min_len = nums.len() as i32;

        for (i, num) in nums.iter().enumerate() {
            current_sum = (current_sum + *num as i64) % p;
            let needed = (current_sum - remainder + p) % p;
            if let Some(min) = mod_map.get(&needed) {
                min_len = std::cmp::min(min_len, i as i32 - *min);
            }
            mod_map.insert(current_sum, i as i32);
        }
        if min_len == nums.len() as i32 {
            -1
        } else {
            min_len
        }
    }
}
