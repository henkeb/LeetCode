// You are given an integer n.
//
// Each number from 1 to n is grouped according to the sum of its digits.
//
// Return the number of groups that have the largest size.
//
//
//
// Example 1:
//
// Input: n = 13
// Output: 4
// Explanation: There are 9 groups in total, they are grouped according sum of its digits of numbers from 1 to 13:
// [1,10], [2,11], [3,12], [4,13], [5], [6], [7], [8], [9].
// There are 4 groups with largest size.
// Example 2:
//
// Input: n = 2
// Output: 2
// Explanation: There are 2 groups [1], [2] of size 1.
//
//
// Constraints:
//
// 1 <= n <= 104
struct Solution;
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        use std::collections::HashMap;
        let mut num_map: HashMap<i32, usize> = HashMap::new();
        for mut num in 1..=n {
            let mut sum = 0;
            while num != 0 {
                sum += num % 10;
                num /= 10;
            }
            num_map.entry(sum).and_modify(|val| *val += 1).or_insert(1);
        }
        if let Some(max) = num_map.iter().map(|(_, val)| val).max() {
            num_map
                .iter()
                .map(|(_, val)| val)
                .fold(0, |acc, val| if val == max { acc + 1 } else { acc })
        } else {
            0
        }
    }
}
