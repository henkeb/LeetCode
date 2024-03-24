// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given a string s consisting of words and spaces, return the length of the last word in the string.
//
// A word is a maximal
// substring
// consisting of non-space characters only.
struct Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace()
            .last()
            .map(|word| word.len())
            .unwrap_or(0) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }
    #[test]
    fn test_length_of_last_word2() {
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }
    #[test]
    fn test_length_of_last_word3() {
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }
}
