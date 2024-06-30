// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given a binary array nums, return the maximum number of consecutive 1's in the array.
struct Solution;
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut max = 0;
        nums.iter().for_each(|element| {
            if *element == 0 {
                if count > max {
                    max = count;
                }
                count = 0;
            } else {
                count += 1;
            }
        });
        if max > count {
            max
        } else {
            count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        );
    }
    #[test]
    fn test_0() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![0, 0, 0, 0, 0, 0]),
            0
        );
    }
    #[test]
    fn test_max() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 1, 1, 1, 1]),
            6
        );
    }
}
