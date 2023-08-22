// Given an array nums of size n, return the majority element.
//
// The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // let majority = nums.len() / 2;
        let mut map: HashMap<i32, usize> = HashMap::new();

        nums.iter().for_each(|element| match map.get_mut(element) {
            Some(value) => *value += 1,
            None => {
                map.insert(*element, 1);
            }
        });

        match map.iter().max_by_key(|pair| pair.1) {
            Some(pair) => *pair.0,
            None => unreachable!(),
        }
    }

    pub fn majority_element_alternate(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums[nums.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::majority_element_alternate(vec![3, 2, 3]), 3);
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::majority_element_alternate(vec![2, 2, 1, 1, 1, 2, 2]),
            2
        );
    }
}
