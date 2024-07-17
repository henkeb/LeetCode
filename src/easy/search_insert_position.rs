// Solution has
// Time complexity: O(log n)
// Space complexity: O(1)
//
// Given a sorted array of distinct integers and a target value,
// return the index if the target is found.
// If not, return the index where it would be if it were inserted in order.
//
// You must write an algorithm with O(log n) runtime complexity.
struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;
        if nums.is_empty() {
            return 0;
        }
        let mut low: i32 = 0;
        let mut high: i32 = (nums.len() - 1) as i32;
        let mut mid: i32 = 0;
        while low <= high {
            mid = low + (high - low) / 2;
            match nums[mid as usize].cmp(&target) {
                Ordering::Equal => return mid,
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid - 1,
            }
        }
        match nums[mid as usize].cmp(&target) {
            Ordering::Less => mid + 1,
            Ordering::Greater => mid,
            Ordering::Equal => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn test_search_insert2() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn test_search_insert3() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn test_search_insert4() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }

    #[test]
    fn test_search_insert5() {
        assert_eq!(Solution::search_insert(vec![1], 2), 1);
    }
}
