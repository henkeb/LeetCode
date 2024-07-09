// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// A perfect number is a positive integer that is equal to the sum of its positive divisors, excluding the number itself. A divisor of an integer x is an integer that can divide x evenly.
//
// Given an integer n, return true if n is a perfect number, otherwise return false.
//

struct Solution;
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        (1..=num / 2)
            .filter(|divisor| num % divisor == 0)
            .sum::<i32>()
            == num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_perfect() {
        assert!(Solution::check_perfect_number(28))
    }

    #[test]
    fn is_not_perfect() {
        assert!(!Solution::check_perfect_number(30))
    }
}
