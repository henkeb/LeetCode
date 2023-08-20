// Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.
//
// Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:
//
//     Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
//     Return k.

struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut unique_pos = 0;

        for i in 1..nums.len() {
            if nums[unique_pos] != nums[i] {
                unique_pos += 1;
                nums[unique_pos] = nums[i];
            }
        }

        (unique_pos + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut input = vec![1, 1, 2];

        assert_eq!(Solution::remove_duplicates(&mut input), 2);
        assert_eq!(input[0..=1], [1, 2]);
    }

    #[test]
    fn test2() {
        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

        assert_eq!(Solution::remove_duplicates(&mut input), 5);
        assert_eq!(input[0..=4], [0, 1, 2, 3, 4]);
    }
}
