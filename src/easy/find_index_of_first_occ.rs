// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given two strings needle and haystack,
// return the index of the first occurrence of needle in haystack,
// or -1 if needle is not part of haystack.
struct Solution;
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let (hay_len, nee_len) = (haystack.len(), needle.len());
        if hay_len < nee_len {
            return -1;
        }
        if nee_len == 0 {
            return 0;
        }
        for i in 0..=hay_len - nee_len {
            if haystack[i..(i + nee_len)] == needle {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
    }
}
