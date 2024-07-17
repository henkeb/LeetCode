// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.
//
// Each letter in magazine can only be used once in ransomNote.

struct Solution;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }
        use std::collections::HashMap;
        let mut mag_map: HashMap<char, usize> = HashMap::new();
        for c in magazine.chars() {
            match mag_map.get(&c) {
                Some(val) => {
                    mag_map.insert(c, val + 1);
                }
                None => {
                    mag_map.insert(c, 1);
                }
            }
        }

        for c in ransom_note.chars() {
            match mag_map.get(&c) {
                Some(val) => {
                    if *val == 0 {
                        return false;
                    }
                    mag_map.insert(c, val - 1);
                }
                None => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ransom_aa() {
        assert!(Solution::can_construct("aa".to_string(), "aab".to_string()));
    }

    #[test]
    fn ransom_fail() {
        assert!(!Solution::can_construct("a".to_string(), "b".to_string()));
    }

    #[test]
    fn ransom_fail_b() {
        assert!(!Solution::can_construct("aa".to_string(), "ab".to_string()));
    }
}
