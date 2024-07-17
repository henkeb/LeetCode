// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.
//
// Increment the large integer by one and return the resulting array of digits.

struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        for digit in digits.iter_mut().rev() {
            let sum = *digit + carry;
            *digit = sum % 10;
            match sum / 10 {
                0 => return digits,
                _ => carry = 1,
            };
        }
        digits.insert(0, 1);
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn test_plus_one_2() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn test_plus_one_3() {
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }
}
