use std::collections::HashMap;

// Given an array nums of distinct positive integers, return the number of tuples (a, b, c, d) such that a * b = c * d where a, b, c, and d are elements of nums, and a != b != c != d.
//
//
//
// Example 1:
//
// Input: nums = [2,3,4,6]
// Output: 8
// Explanation: There are 8 valid tuples:
// (2,6,3,4) , (2,6,4,3) , (6,2,3,4) , (6,2,4,3)
// (3,4,2,6) , (4,3,2,6) , (3,4,6,2) , (4,3,6,2)
// Example 2:
//
// Input: nums = [1,2,4,5,10]
// Output: 16
// Explanation: There are 16 valid tuples:
// (1,10,2,5) , (1,10,5,2) , (10,1,2,5) , (10,1,5,2)
// (2,5,1,10) , (2,5,10,1) , (5,2,1,10) , (5,2,10,1)
// (2,10,4,5) , (2,10,5,4) , (10,2,4,5) , (10,2,5,4)
// (4,5,2,10) , (4,5,10,2) , (5,4,2,10) , (5,4,10,2)
//
//
// Constraints:
//
// 1 <= nums.length <= 1000
// 1 <= nums[i] <= 104
// All elements in nums are distinct.
struct Solution;
impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut prod: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if i == j {
                    continue;
                }
                prod.entry(nums[i] * nums[j])
                    .and_modify(|val| *val += 1)
                    .or_insert(1);
            }
        }

        prod.values()
            .filter(|&value| *value > 1)
            .map(|val| ((val - 1) * val / 2) * 8)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::tuple_same_product(vec![2, 3, 4, 6]), 8);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::tuple_same_product(vec![1, 2, 4, 5, 10]), 16);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::tuple_same_product(vec![2, 3, 4, 6, 8, 12]), 40);
    }
}
