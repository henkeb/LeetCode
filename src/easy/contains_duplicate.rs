// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
//Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
struct Solution;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut hash_set = HashSet::new();
        nums.iter().any(|element| !hash_set.insert(*element))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn it_works2() {
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn it_works3() {
        assert!(Solution::contains_duplicate(vec![
            1, 1, 1, 3, 3, 4, 3, 2, 4, 2
        ]));
    }
}
