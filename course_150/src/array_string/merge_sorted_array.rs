// You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
//
// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
//
// The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.

use std::mem::swap;

struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
            return;
        }

        (0..m as usize).for_each(|i| {
            if nums1[i] > nums2[0] {
                swap(&mut nums1[i], &mut nums2[0]);
                nums2.sort();
            }
        });

        (0..n as usize).for_each(|val| {
            swap(&mut nums1[m as usize + val], &mut nums2[val]);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums1: Vec<i32> = Vec::from([1, 2, 3, 0, 0, 0]);
        let mut nums2: Vec<i32> = Vec::from([2, 5, 6]);
        Solution::merge(&mut nums1, 3, &mut nums2, 3);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn example_2() {
        let mut nums1: Vec<i32> = Vec::from([1]);
        let mut nums2: Vec<i32> = Vec::from([]);
        Solution::merge(&mut nums1, 1, &mut nums2, 0);

        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn example_3() {
        let mut nums1: Vec<i32> = Vec::from([0]);
        let mut nums2: Vec<i32> = Vec::from([1]);
        Solution::merge(&mut nums1, 0, &mut nums2, 1);

        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn example_4() {
        let mut nums1: Vec<i32> = Vec::from([4, 5, 6, 0, 0, 0]);
        let mut nums2: Vec<i32> = Vec::from([1, 2, 3]);
        Solution::merge(&mut nums1, 3, &mut nums2, 3);

        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }
}
