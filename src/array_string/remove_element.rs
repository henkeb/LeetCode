// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.
//
// Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:
//
//     Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
//     Return k.

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut positions: VecDeque<i32> = VecDeque::new();
        let mut count = 0;
        let length: i32 = nums.len() as i32;

        nums.iter().enumerate().for_each(|(i, element)| {
            if *element == val {
                positions.push_back(i as i32);
            }
        });

        while !positions.is_empty() {
            nums.swap(
                positions.pop_back().unwrap() as usize,
                (length - count - 1) as usize,
            );
            count += 1;
        }

        length - count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut input: Vec<i32> = vec![3, 2, 2, 3];
        let value = 3;
        assert_eq!(Solution::remove_element(&mut input, value), 2);
        assert_eq!(&input[2..=3], [3, 3]);
    }

    #[test]
    fn test2() {
        let mut input: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let value = 2;
        assert_eq!(Solution::remove_element(&mut input, value), 5);
        assert_eq!(&input[5..=7], [2, 2, 2]);
    }
}
