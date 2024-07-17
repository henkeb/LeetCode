// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// We define the usage of capitals in a word to be right when one of the following cases holds:
//
//     All letters in this word are capitals, like "USA".
//     All letters in this word are not capitals, like "leetcode".
//     Only the first letter in this word is capital, like "Google".
//
// Given a string word, return true if the usage of capitals in it is right.

struct Solution;
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        word == word.to_ascii_uppercase()
            || word == word.to_ascii_lowercase()
            || (word.chars().next().unwrap().is_ascii_uppercase()
                && !word.chars().skip(1).any(|ch| ch.is_ascii_uppercase()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cap_ok() {
        assert!(Solution::detect_capital_use("USA".to_string()))
    }

    #[test]
    fn cap_false() {
        assert!(!Solution::detect_capital_use("FlaG".to_string()))
    }
}
