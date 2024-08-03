// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// A swap is defined as taking two distinct positions in an array and swapping the values in them.
//
// A circular array is defined as an array where we consider the first element and the last element to be adjacent.
//
// Given a binary circular array nums, return the minimum number of swaps required to group all 1's present in the array together at any location.
struct Solution;
impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let window_size = nums.iter().sum::<i32>() as usize;
        let mut window_sum = 0;
        nums.iter()
            .take(window_size)
            .for_each(|val| window_sum += val);
        if window_sum == 0 {
            return 0;
        }
        let mut min_swaps = window_size as i32 - window_sum;

        for i in 1..nums.len() {
            if nums.len() - i > window_size - 1 {
                window_sum += nums[i + window_size - 1] - nums[i - 1];
            } else {
                window_sum += nums[i + window_size - nums.len() - 1] - nums[i - 1];
            }
            min_swaps = min_swaps.min(window_size as i32 - window_sum);
        }
        min_swaps
    }
}
