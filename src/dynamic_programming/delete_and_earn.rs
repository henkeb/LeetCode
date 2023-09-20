// Solution has
// Time complexity: O(?)
// Space complexity: O(?)

// You are given an integer array nums. You want to maximize the number of points you get by performing the following operation any number of times:
//
//     Pick any nums[i] and delete it to earn nums[i] points. Afterwards, you must delete every element equal to nums[i] - 1 and every element equal to nums[i] + 1.
//
// Return the maximum number of points you can earn by applying the above operation some number of times.

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut max_number = 0;

        for i in nums.iter() {
            match map.get_mut(i) {
                None => {
                    map.insert(*i, *i);
                }
                Some(x) => *x += i,
            };
            max_number = std::cmp::max(max_number, *i);
        }

        let mut dp: Vec<i32> = vec![0; (max_number + 1) as usize];
        dp[1] = *map.get(&1).unwrap_or(&0);
        for i in 2..dp.len() {
            dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + *map.get(&(i as i32)).unwrap_or(&0));
        }
        dp[max_number as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::delete_and_earn([3, 4, 2].to_vec()), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::delete_and_earn([2, 2, 3, 3, 3, 4].to_vec()), 9);
    }
}
