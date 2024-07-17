// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given an array nums of n integers where nums[i] is in the range [1, n], return an array of all the integers in the range [1, n] that do not appear in nums.
struct Solution;
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut mask: Vec<bool> = vec![false; nums.len() + 1];
        let mut output: Vec<i32> = Vec::new();
        for i in &nums {
            if !mask[*i as usize] {
                mask[*i as usize] = true;
            }
        }
        for (i, m) in mask.iter().enumerate().skip(1) {
            if !m {
                output.push(i as i32);
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        )
    }
    #[test]
    fn it_works2() {
        assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2])
    }
}
