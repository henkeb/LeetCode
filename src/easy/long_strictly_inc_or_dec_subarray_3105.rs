// You are given an array of integers nums. Return the length of the longest
// subarray
//  of nums which is either
// strictly increasing
//  or
// strictly decreasing
// .
//
//
//
// Example 1:
//
// Input: nums = [1,4,3,3,2]
//
// Output: 2
//
// Explanation:
//
// The strictly increasing subarrays of nums are [1], [2], [3], [3], [4], and [1,4].
//
// The strictly decreasing subarrays of nums are [1], [2], [3], [3], [4], [3,2], and [4,3].
//
// Hence, we return 2.
//
// Example 2:
//
// Input: nums = [3,3,3,3]
//
// Output: 1
//
// Explanation:
//
// The strictly increasing subarrays of nums are [3], [3], [3], and [3].
//
// The strictly decreasing subarrays of nums are [3], [3], [3], and [3].
//
// Hence, we return 1.
//
// Example 3:
//
// Input: nums = [3,2,1]
//
// Output: 3
//
// Explanation:
//
// The strictly increasing subarrays of nums are [3], [2], and [1].
//
// The strictly decreasing subarrays of nums are [3], [2], [1], [3,2], [2,1], and [3,2,1].
//
// Hence, we return 3.
//
//
//
// Constraints:
//
// 1 <= nums.length <= 50
// 1 <= nums[i] <= 50
struct Solution;
impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut longest = 1;
        let mut increasing = 1;
        let mut decreasing = 1;

        for i in 1..nums.len() {
            match nums[i - 1].cmp(&nums[i]) {
                std::cmp::Ordering::Less => {
                    decreasing += 1;
                    increasing = 1;
                }
                std::cmp::Ordering::Greater => {
                    increasing += 1;
                    decreasing = 1;
                }
                std::cmp::Ordering::Equal => {
                    increasing = 1;
                    decreasing = 1;
                }
            }
            longest = longest.max(decreasing).max(increasing);
        }
        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 2, 1]), 3);
    }
}
