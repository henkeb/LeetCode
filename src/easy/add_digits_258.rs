// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given an integer num, repeatedly add all its digits until the result has only one digit, and return it.
struct Solution;
impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
            if num == 0 && sum >= 10 {
                num = sum;
                sum = 0;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::add_digits(38), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::add_digits(0), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::add_digits(1), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::add_digits(123), 6);
    }
}
