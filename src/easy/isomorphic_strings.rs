// Given two strings s and t, determine if they are isomorphic.
// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
//
// Two strings s and t are isomorphic if the characters in s can be replaced to get t.
//
// All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.

const UNMAPPED: u8 = 129;
struct Solution;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (s, t) = (s.into_bytes(), t.into_bytes());
        // 0-127 basic ascii table - 129 is unmapped char
        let mut s_table: [u8; 128] = [UNMAPPED; 128];
        let mut t_table: [u8; 128] = [UNMAPPED; 128];

        for i in 0..s.len() {
            let (s_c, t_c) = (s[i] as usize, t[i] as usize);
            if s_table[s_c] == UNMAPPED && t_table[t_c] == UNMAPPED {
                s_table[s_c] = t[i];
                t_table[t_c] = s[i];
            } else if s_table[s_c] != t[i] || t_table[t_c] != s[i] {
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
        assert!(Solution::is_isomorphic(
            "egg".to_string(),
            "add".to_string()
        ));
    }

    #[test]
    fn it_works2() {
        assert!(!Solution::is_isomorphic(
            "foo".to_string(),
            "bar".to_string()
        ));
    }

    #[test]
    fn it_works3() {
        assert!(Solution::is_isomorphic(
            "paper".to_string(),
            "title".to_string()
        ));
    }

    #[test]
    fn it_works4() {
        assert!(!Solution::is_isomorphic(
            "badc".to_string(),
            "baba".to_string()
        ));
    }
}
