// You are given an array nums consisting of positive integers.
//
// We call a subarray of nums nice if the bitwise AND of every pair of elements that are in different positions in the subarray is equal to 0.
//
// Return the length of the longest nice subarray.
//
// A subarray is a contiguous part of an array.
//
// Note that subarrays of length 1 are always considered nice.
//
//
//
// Example 1:
//
// Input: nums = [1,3,8,48,10]
// Output: 3
// Explanation: The longest nice subarray is [3,8,48]. This subarray satisfies the conditions:
// - 3 AND 8 = 0.
// - 3 AND 48 = 0.
// - 8 AND 48 = 0.
// It can be proven that no longer nice subarray can be obtained, so we return 3.
// Example 2:
//
// Input: nums = [3,1,5,11,13]
// Output: 1
// Explanation: The length of the longest nice subarray is 1. Any subarray of length 1 can be chosen.
//
//
// Constraints:
//
// 1 <= nums.length <= 105
// 1 <= nums[i] <= 109
struct Solution;
impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut used_bits = 0;
        let mut start = 0;
        let mut max_nice = 0;

        for end in 0..nums.len() {
            while used_bits & nums[end] != 0 {
                used_bits ^= nums[start];
                start += 1;
            }

            used_bits |= nums[end];

            max_nice = std::cmp::max(max_nice, end - start + 1);
        }
        max_nice as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_nice_subarray(vec![1, 3, 8, 48, 10]), 3)
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::longest_nice_subarray(vec![
                744437702, 379056602, 145555074, 392756761, 560864007, 934981918, 113312475, 1090,
                16384, 33, 217313281, 117883195, 978927664
            ]),
            3
        );
    }
}
