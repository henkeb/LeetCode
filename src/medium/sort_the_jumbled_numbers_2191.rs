// Solution has
// Time complexity: O(1)
// Space complexity: O(1)
//
// You are given a 0-indexed integer array mapping which represents the mapping rule of a shuffled decimal system. mapping[i] = j means digit i should be mapped to digit j in this system.
//
// The mapped value of an integer is the new integer obtained by replacing each occurrence of digit i in the integer with mapping[i] for all 0 <= i <= 9.
//
// You are also given another integer array nums. Return the array nums sorted in non-decreasing order based on the mapped values of its elements.
//
// Notes:
//
//     Elements with the same mapped values should appear in the same relative order as in the input.
//     The elements of nums should only be sorted based on their mapped values and not be replaced by them.

struct Solution;
impl Solution {
    fn mapping(mut num: i32, mapping: &[i32]) -> i32 {
        if num == 0 {
            return mapping[0];
        }
        let mut exp = 1;
        let mut temp = 0;
        while num > 0 {
            let digit = num % 10;
            num /= 10;
            temp += mapping[digit as usize] * exp;
            exp *= 10;
        }
        temp
    }
    pub fn sort_jumbled(mapping: Vec<i32>, mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_cached_key(|n| Self::mapping(*n, &mapping));
        nums
    }
}
