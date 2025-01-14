// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given two strings s1 and s2, return true if s2 contains a
// permutation
//  of s1, or false otherwise.
//
// In other words, return true if one of s1's permutations is the substring of s2.
//
//
//
// Example 1:
//
// Input: s1 = "ab", s2 = "eidbaooo"
// Output: true
// Explanation: s2 contains one permutation of s1 ("ba").
// Example 2:
//
// Input: s1 = "ab", s2 = "eidboaoo"
// Output: false
//
//
// Constraints:
//
// 1 <= s1.length, s2.length <= 104
// s1 and s2 consist of lowercase English letters.
struct Solution;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let mut s1_freq = [0; 26];
        for ch in s1.chars() {
            s1_freq[(ch as u8 - b'a') as usize] += 1;
        }
        let mut s2_freq = [0; 26];
        for ch in &s2.as_bytes()[0..s1.len()] {
            s2_freq[(ch - b'a') as usize] += 1;
        }
        if s2_freq == s1_freq {
            return true;
        }
        let mut l: usize = 0;
        for r in s1.len()..s2.len() {
            s2_freq[(s2.as_bytes()[l] - b'a') as usize] -= 1;
            l += 1;
            s2_freq[(s2.as_bytes()[r] - b'a') as usize] += 1;
            if s2_freq == s1_freq {
                return true;
            }
        }
        false
    }
}
