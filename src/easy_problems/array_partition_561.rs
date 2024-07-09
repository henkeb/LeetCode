// Solution has
// Time complexity: O(n log(n))
// Space complexity: O(1)
//
// Given an integer array nums of 2n integers, group these integers into n pairs (a1, b1), (a2, b2), ..., (an, bn) such that the sum of min(ai, bi) for all i is maximized. Return the maximized sum.
struct Solution;
impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.into_iter().step_by(2).sum()
    }
}
