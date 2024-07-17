// Solution has
// Time complexity: O(log n)
// Space complexity: O(1)
//
// Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.
//
// You must write an algorithm with O(log n) runtime complexity.
struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut hi = nums.len() as i32;
        let mut lo = -1;

        while lo + 1 < hi {
            let mid = lo + (hi - lo) / 2;

            if nums[mid as usize] >= target {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        if hi == nums.len() as i32 || nums[hi as usize] != target {
            return -1;
        }
        hi
    }
}
