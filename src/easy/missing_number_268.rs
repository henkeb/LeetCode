// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.
struct Solution;
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = n * (n + 1) / 2;
        nums.iter().for_each(|val| sum -= *val as usize);
        sum as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn it_works2() {
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn it_works3() {
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
