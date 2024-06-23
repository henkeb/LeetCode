// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
//
// Note that you must do this in-place without making a copy of the array.
struct Solution;
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut swap_idx = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(swap_idx, i);
                swap_idx += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn it_works2() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn it_works3() {
        let mut nums = vec![1];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn it_works4() {
        let mut nums = vec![0, 1];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 0]);
    }

    #[test]
    fn it_works5() {
        let mut nums = vec![1, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 0]);
    }

    #[test]
    fn it_works6() {
        let mut nums = vec![0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0, 0]);
    }

    #[test]
    fn it_works7() {
        let mut nums = vec![1, 2, 3];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn it_works8() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }
}
