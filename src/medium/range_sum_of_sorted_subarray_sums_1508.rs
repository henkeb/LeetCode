// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// You are given the array nums consisting of n positive integers. You computed the sum of all non-empty continuous subarrays from the array and then sorted them in non-decreasing order, creating a new array of n * (n + 1) / 2 numbers.
//
// Return the sum of the numbers from index left to index right (indexed from 1), inclusive, in the new array. Since the answer can be a huge number return it modulo 10^9 + 7.
struct Solution;
impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let nusize = n as usize;
        const MOD: i32 = 1e9 as i32 + 7;
        let mut sum_subarray: Vec<i32> = Vec::with_capacity(nusize * (nusize + 1) / 2);
        let max_subarr: i32 = nums.iter().sum();
        let mut add_acc = 0;
        (0..nusize).for_each(|i| {
            add_acc += nums[i];
            sum_subarray.push(add_acc);
            sum_subarray.push(max_subarr - add_acc);
        });
        sum_subarray.push(max_subarr);
        sum_subarray.sort_unstable();
        let mut output = 0;
        ((left as usize - 1)..right as usize).for_each(|i| {
            output += sum_subarray[i];
            if output > MOD {
                output %= MOD;
            }
        });
        output
    }
}
// Input: nums = [1,2,3,4], n = 4, left = 1, right = 5
// Output: 13
// Explanation: All subarray sums are 1, 3, 6, 10, 2, 5, 9, 3, 7, 4.
// After sorting them in non-decreasing order we have the new array [1, 2, 3, 3, 4, 5, 6, 7, 9, 10].
// The sum of the numbers from index le = 1 to ri = 5 is 1 + 2 + 3 + 3 + 4 = 13.
