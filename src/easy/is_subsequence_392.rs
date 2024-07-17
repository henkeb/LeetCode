// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
//
// A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).
//
struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        if t.is_empty() {
            return false;
        }

        let mut s_ptr = 0;

        for c in t.chars() {
            if let Some(s_char) = s.chars().nth(s_ptr) {
                if s_char == c {
                    s_ptr += 1;
                }
            }
        }
        s.len() == s_ptr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Solution::is_subsequence(
            "abc".to_string(),
            "ahbgdc".to_string()
        ));
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_subsequence(
            "axc".to_string(),
            "ahbgdc".to_string()
        ));
    }
    #[test]
    fn test_3() {
        assert!(!Solution::is_subsequence(
            "acb".to_string(),
            "ahbgdc".to_string()
        ))
    }
}
