// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given an array of integers nums, sort the array in increasing order based on the frequency of the values. If multiple values have the same frequency, sort them in decreasing order.
//
// Return the sorted array.
struct Solution;
impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut num_map = HashMap::new();
        for &num in nums.iter() {
            num_map
                .entry(num)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut map_vec = num_map.into_iter().collect::<Vec<(i32, i32)>>();
        map_vec.sort_by(|&a, &b| {
            if a.1 == b.1 {
                b.0.cmp(&a.0)
            } else {
                a.1.cmp(&b.1)
            }
        });
        map_vec
            .into_iter()
            .flat_map(|(num, count)| vec![num; count as usize])
            .collect()
    }
}
