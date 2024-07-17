// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
//
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
struct Solution;
impl Solution {
    pub fn is_anagram2(s: String, t: String) -> bool {
        use std::collections::HashMap;
        let mut s_hmap: HashMap<char, u32> = HashMap::new();
        let mut t_hmap: HashMap<char, u32> = HashMap::new();
        for s_chars in s.chars() {
            s_hmap
                .entry(s_chars)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        for t_chars in t.chars() {
            t_hmap
                .entry(t_chars)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        s_hmap == t_hmap
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut frequency: [i8; 128] = [0; 128];

        for s_char in s.chars() {
            frequency[s_char as usize] += 1;
        }

        for t_char in t.chars() {
            frequency[t_char as usize] -= 1;
            if frequency[t_char as usize] < 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
    }

    #[test]
    fn it_works2() {
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
    }

    #[test]
    fn it_works3() {
        assert!(Solution::is_anagram("a".to_string(), "a".to_string()));
    }

    #[test]
    fn it_works4() {
        assert!(!Solution::is_anagram("a".to_string(), "ab".to_string()));
    }

    #[test]
    fn it_works5() {
        assert!(!Solution::is_anagram("abc".to_string(), "ab".to_string()));
    }
}
