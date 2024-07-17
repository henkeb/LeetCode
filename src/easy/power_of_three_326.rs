// Solution has
// Time complexity: O(log(n))
// Space complexity: O(1)
//
// Given an integer n, return true if it is a power of three. Otherwise, return false.
//
// An integer n is a power of three, if there exists an integer x such that n == 3^x.
struct Solution;
impl Solution {
    pub fn is_power_of_three(mut n: i32) -> bool {
        if n == 1 {
            return true;
        }
        if n.is_negative() || n % 3 != 0 {
            return false;
        }
        while n % 3 == 0 && n != 0 {
            n /= 3;
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_three() {
        assert!(Solution::is_power_of_three(27));
    }

    #[test]
    fn test_is_power_of_three_2() {
        assert!(!Solution::is_power_of_three(0));
    }

    #[test]
    fn test_is_power_of_three_3() {
        assert!(!Solution::is_power_of_three(-27));
    }

    #[test]
    fn test_is_power_of_three_9() {
        assert!(Solution::is_power_of_three(9));
    }

    #[test]
    fn test_0() {
        assert!(!Solution::is_power_of_three(0));
    }
}
