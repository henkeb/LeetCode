// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given an array of integers nums and an integer k, return the number of unique k-diff pairs in the array.
//
// A k-diff pair is an integer pair (nums[i], nums[j]), where the following are true:
//
//     0 <= i, j < nums.length
//     i != j
//     |nums[i] - nums[j]| == k
//
// Notice that |val| denotes the absolute value of val.
struct Solution;
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut num_hm: HashMap<i32, usize> = HashMap::new();
        for &num in nums.iter() {
            *num_hm.entry(num).or_insert(0) += 1;
        }
        let mut count = 0;
        for (&num, _) in num_hm.iter() {
            if let Some(&val) = num_hm.get(&(num + k)) {
                if k != 0 || val > 1 {
                    count += 1;
                }
            }
        }
        count
    }
}
