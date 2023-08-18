// You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.
//
// Merge nums1 and nums2 into a single array sorted in non-decreasing order.
//
// The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.

struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.iter_mut().for_each(|val| {
            if val < 
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

    fn example_2() {
        let mut nums1: Vec<i32> = Vec::from([1]);
        let mut nums2: Vec<i32> = Vec::from([]);
        Solution::merge(&mut nums1, 1, &mut nums2, 0);

        assert_eq!(nums1, vec![1]);
    }

    fn example_3() {
        let mut nums1: Vec<i32> = Vec::from([]);
        let mut nums2: Vec<i32> = Vec::from([1]);
        Solution::merge(&mut nums1, 0, &mut nums2, 1);

        assert_eq!(nums1, vec![1]);
    }
}
