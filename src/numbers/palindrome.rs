// Solution has
// Time complexity: O(n)
// Space complexity: O(1)

// Given an integer x, return true if x is a palindrome, and false otherwise.

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let number: &str = &x.to_string();
        number == number.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(Solution::is_palindrome(121));
        assert!(!Solution::is_palindrome(-121));
        assert!(!Solution::is_palindrome(10));
    }
}
