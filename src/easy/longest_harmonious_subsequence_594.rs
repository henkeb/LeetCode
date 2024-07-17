// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// We define a harmonious array as an array where the difference between its maximum value and its minimum value is exactly 1.
//
// Given an integer array nums, return the length of its longest harmonious subsequence among all its possible subsequences.
//
// A subsequence of array is a sequence that can be derived from the array by deleting some or no elements without changing the order of the remaining elements.
struct Solution;
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut count_nums: HashMap<i32, i32> = HashMap::new();

        for &num in nums.iter() {
            count_nums
                .entry(num)
                .and_modify(|key| *key += 1)
                .or_insert(1);
        }
        let mut max_count = 0;
        for (&num, &count_num) in count_nums.iter() {
            if count_nums.contains_key(&(num - 1)) {
                max_count =
                    std::cmp::max(max_count, count_num + count_nums.get(&(num - 1)).unwrap());
            }

            if count_nums.contains_key(&(num + 1)) {
                max_count =
                    std::cmp::max(max_count, count_num + count_nums.get(&(num + 1)).unwrap());
            }
        }
        max_count
    }
}
