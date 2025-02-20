use std::collections::HashMap;

// You are given a 0-indexed array nums consisting of positive integers. You can choose two indices i and j, such that i != j, and the sum of digits of the number nums[i] is equal to that of nums[j].
//
// Return the maximum value of nums[i] + nums[j] that you can obtain over all possible indices i and j that satisfy the conditions.
//
//
//
// Example 1:
//
// Input: nums = [18,43,36,13,7]
// Output: 54
// Explanation: The pairs (i, j) that satisfy the conditions are:
// - (0, 2), both numbers have a sum of digits equal to 9, and their sum is 18 + 36 = 54.
// - (1, 4), both numbers have a sum of digits equal to 7, and their sum is 43 + 7 = 50.
// So the maximum sum that we can obtain is 54.
// Example 2:
//
// Input: nums = [10,12,19,14]
// Output: -1
// Explanation: There are no two numbers that satisfy the conditions, so we return -1.
//
//
// Constraints:
//
// 1 <= nums.length <= 105
// 1 <= nums[i] <= 109
struct Solution;
impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut num_map: HashMap<i32, i32> = HashMap::new();
        let mut max_num = -1;
        for (i, num) in nums.iter().enumerate() {
            let sum = Solution::sum_digits(num);
            num_map
                .entry(sum)
                .and_modify(|val| {
                    let new_val = nums[i];
                    max_num = max_num.max(new_val + *val);
                    if new_val > *val {
                        *val = new_val;
                    }
                })
                .or_insert(nums[i]);
        }
        max_num
    }
    fn sum_digits(num: &i32) -> i32 {
        let mut num_cp = *num;
        let mut res = 0;
        while num_cp != 0 {
            res += num_cp % 10;
            num_cp /= 10;
        }
        res
    }
}
