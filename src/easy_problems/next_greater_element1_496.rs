// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// The next greater element of some element x in an array is the first greater element that is to the right of x in the same array.
// You are given two distinct 0-indexed integer arrays nums1 and nums2, where nums1 is a subset of nums2.
//
// For each 0 <= i < nums1.length, find the index j such that nums1[i] == nums2[j] and determine the next greater element of nums2[j] in nums2. If there is no next greater element, then the answer for this query is -1.
//
// Return an array ans of length nums1.length such that ans[i] is the next greater element as described above.
struct Solution;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut output = vec![-1; nums1.len()];
        (0..nums1.len()).for_each(|i| {
            let mut iter = nums2.iter();
            iter.find(|&&val| val == nums1[i]);
            for element in iter {
                if *element > nums1[i] {
                    output[i] = *element;
                    break;
                }
            }
        });
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }
}
