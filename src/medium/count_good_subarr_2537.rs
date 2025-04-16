// Given an integer array nums and an integer k, return the number of good subarrays of nums.
//
// A subarray arr is good if there are at least k pairs of indices (i, j) such that i < j and arr[i] == arr[j].
//
// A subarray is a contiguous non-empty sequence of elements within an array.
//
//
//
// Example 1:
//
// Input: nums = [1,1,1,1,1], k = 10
// Output: 1
// Explanation: The only good subarray is the array nums itself.
// Example 2:
//
// Input: nums = [3,1,4,3,2,2,4], k = 2
// Output: 4
// Explanation: There are 4 different good subarrays:
// - [3,1,4,3,2,2] that has 2 pairs.
// - [3,1,4,3,2,2,4] that has 3 pairs.
// - [1,4,3,2,2,4] that has 2 pairs.
// - [4,3,2,2,4] that has 2 pairs.
//
//
// Constraints:
//
// 1 <= nums.length <= 105
// 1 <= nums[i], k <= 109
struct Solution;
impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::HashMap;

        let n = nums.len();
        let mut right = -1;
        let mut same = 0;
        let mut count = HashMap::new();
        let mut output = 0;
        for left in 0..n {
            while same < k && right + 1 < n as i32 {
                right += 1;
                let num = nums[right as usize];
                count
                    .entry(num)
                    .and_modify(|val| {
                        same += *val;
                        *val += 1;
                    })
                    .or_insert(1);
            }
            if same >= k {
                output += n as i64 - right as i64;
            }
            let num = nums[left];
            count
                .entry(num)
                .and_modify(|val| {
                    *val -= 1;
                    same -= *val;
                })
                .or_insert(0);
        }
        output
    }
}
