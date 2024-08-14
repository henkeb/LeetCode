// Solution has
// Time complexity: O(nlogM + nlogn) M = max possible distance
// Space complexity: O(1)
//
// The distance of a pair of integers a and b is defined as the absolute difference between a and b.
//
// Given an integer array nums and an integer k, return the kth smallest distance among all the pairs nums[i] and nums[j] where 0 <= i < j < nums.length.
struct Solution;
impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let (mut l, mut h) = (0, nums[nums.len() - 1] - nums[0]);
        while l < h {
            let m = (h + l) / 2;

            let count = Self::count_pairs_with_max_d(&nums, m);
            if count < k {
                l = m + 1;
            } else {
                h = m;
            }
        }
        l
    }

    fn count_pairs_with_max_d(nums: &[i32], max_d: i32) -> i32 {
        let mut count = 0;
        let nums_len = nums.len();
        let mut left: usize = 0;
        for right in 0..nums_len {
            while nums[right] - nums[left] > max_d {
                left += 1;
            }
            count += right - left;
        }
        count as i32
    }
}
