// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
//Given an integer array nums and an integer k, return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.
struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        let mut pool = HashMap::with_capacity(nums.len());

        nums.into_iter()
            .enumerate()
            .any(|(j, x)| Some(true) == pool.insert(x, j).map(|i| j - i <= k as usize))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
    }

    #[test]
    fn it_works2() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
    }

    #[test]
    fn it_works3() {
        assert!(!Solution::contains_nearby_duplicate(
            vec![1, 2, 3, 1, 2, 3],
            2
        ));
    }
}
