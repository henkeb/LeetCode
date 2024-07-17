// Solution has
// Time complexity: O(1)
// Space complexity: O(1)
//
// Given an integer n, return true if it is a power of two. Otherwise, return false.
//
// An integer n is a power of two, if there exists an integer x such that n == 2x.
// Explanation of solution:
//
// 8 = 1000 in binary
// 8 - 1 = 7 = 0111 in binary
// 8 & 7 = 0 since bits are flipped in each expression
struct Solution;
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n & (n - 1) == 0 && n.is_positive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_two() {
        assert!(Solution::is_power_of_two(1));
        assert!(Solution::is_power_of_two(16));
        assert!(!Solution::is_power_of_two(3));
        assert!(Solution::is_power_of_two(4));
        assert!(!Solution::is_power_of_two(5));
        assert!(Solution::is_power_of_two(8));
        assert!(!Solution::is_power_of_two(9));
        assert!(!Solution::is_power_of_two(10));
    }
}
