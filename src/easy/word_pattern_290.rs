// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given a pattern and a string s, find if s follows the same pattern.
//
// Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s.
struct Solution;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        use std::collections::HashMap;
        let mut p_map: HashMap<char, &str> = HashMap::new();
        let mut s_map: HashMap<&str, char> = HashMap::new();

        if pattern.len() != s.split_whitespace().count() {
            return false;
        }

        for (i, word) in s.split_whitespace().enumerate() {
            let s_char = pattern.chars().nth(i).unwrap();
            if let Some(mapped_char) = s_map.insert(word, s_char) {
                if mapped_char != s_char {
                    return false;
                }
            }

            if let Some(mapped_word) = p_map.insert(s_char, word) {
                if mapped_word != word {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_a() {
        assert!(Solution::word_pattern(
            "abba".to_string(),
            "dog cat cat dog".to_string()
        ))
    }

    #[test]
    fn it_works2() {
        assert!(!Solution::word_pattern(
            "abba".to_string(),
            "dog cat cat fish".to_string()
        ))
    }

    #[test]
    fn it_works3() {
        assert!(!Solution::word_pattern(
            "aaaa".to_string(),
            "dog cat cat dog".to_string()
        ))
    }

    #[test]
    fn it_works4() {
        assert!(!Solution::word_pattern(
            "abba".to_string(),
            "dog dog dog dog".to_string()
        ))
    }
}
