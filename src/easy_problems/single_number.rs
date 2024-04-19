// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
//
// You must implement a solution with a linear runtime complexity and use only constant extra space.
//
use std::ops::BitXor;
struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .reduce(|result, element| result.bitxor(element))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn it_works2() {
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn it_works3() {
        assert_eq!(Solution::single_number(vec![1]), 1);
    }
}
