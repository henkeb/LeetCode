// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
struct Solution;
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        nums.windows(k as usize)
            .map(|window| window.iter().sum::<i32>())
            .max()
            .unwrap_or(0) as f64
            / k as f64
    }
}
