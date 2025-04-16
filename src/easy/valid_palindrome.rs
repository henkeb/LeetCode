// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
//
// Given a string s, return true if it is a palindrome, or false otherwise.
//
struct Solution;
impl Solution {
    pub fn is_palindrome_old(s: String) -> bool {
        s.char_indices()
            .filter(|(_, c)| c.is_ascii_alphanumeric())
            .zip(
                s.char_indices()
                    .rev()
                    .filter(|(_, c)| c.is_ascii_alphanumeric()),
            )
            .take_while(|((first_count, _), (last_count, _))| first_count < last_count)
            .all(|((_, first_char), (_, last_char))| {
                first_char.to_ascii_lowercase() == last_char.to_ascii_lowercase()
            })
    }

    pub fn is_palindrome(s: String) -> bool {
        s.chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .zip(s.chars().rev().filter(|c| c.is_ascii_alphanumeric()))
            .all(|(a, b)| a.to_ascii_lowercase() == b.to_ascii_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
    }

    #[test]
    fn test_is_not_palindrome() {
        assert!(!Solution::is_palindrome("race a car".to_string()));
    }

    #[test]
    fn test_is_empty() {
        assert!(Solution::is_palindrome("".to_string()));
    }

    #[test]
    fn test_0p() {
        assert!(!Solution::is_palindrome("0P".to_string()));
    }
}
