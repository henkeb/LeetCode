// Given an integer array nums sorted in non-decreasing order, remove some duplicates in-place such that each unique element appears at most twice. The relative order of the elements should be kept the same.
//
// Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.
//
// Return k after placing the final result in the first k slots of nums.
//
// Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.

struct Solution {}

const DUPLICATES: u8 = 2;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = 1;
        let mut j = 1;

        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                count += 1;
            } else {
                count = 1;
            }

            if count <= DUPLICATES {
                nums[j] = nums[i];
                j += 1;
            }
        }
        j as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut input = vec![1, 1, 1, 2, 2, 3];
        let target = 5;

        assert_eq!(Solution::remove_duplicates(&mut input), target);
        assert_eq!(input[0..target as usize], [1, 1, 2, 2, 3]);
    }

    #[test]
    fn test2() {
        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let target = 9;

        assert_eq!(Solution::remove_duplicates(&mut input), target);
        assert_eq!(input[0..target as usize], [0, 0, 1, 1, 2, 2, 3, 3, 4]);
    }

    #[test]
    fn test3() {
        let mut input = vec![0, 1, 1, 1];
        let target = 3;

        assert_eq!(Solution::remove_duplicates(&mut input), target);
        assert_eq!(input[0..target as usize], [0, 1, 1]);
    }
}
