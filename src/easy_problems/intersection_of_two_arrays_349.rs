// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given two integer arrays nums1 and nums2, return an array of their
// intersection
// . Each element in the result must be unique and you may return the result in any order.
struct Solution;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let nums1_set: HashSet<i32> = HashSet::from_iter(nums1);
        let nums2_set: HashSet<i32> = HashSet::from_iter(nums2);
        let mut output: Vec<i32> = vec![];
        for i in nums1_set.intersection(&nums2_set).collect::<Vec<&i32>>() {
            output.push(*i);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    fn first() {
        assert_eq!(
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2]
        );
    }

    #[cfg(test)]
    fn second() {
        assert_eq!(
            Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![4, 9]
        );
    }
}
