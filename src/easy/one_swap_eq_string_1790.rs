// You are given two strings s1 and s2 of equal length. A string swap is an operation where you choose two indices in a string (not necessarily different) and swap the characters at these indices.
//
// Return true if it is possible to make both strings equal by performing at most one string swap on exactly one of the strings. Otherwise, return false.
//
//
//
// Example 1:
//
// Input: s1 = "bank", s2 = "kanb"
// Output: true
// Explanation: For example, swap the first character with the last character of s2 to make "bank".
// Example 2:
//
// Input: s1 = "attack", s2 = "defend"
// Output: false
// Explanation: It is impossible to make them equal with one string swap.
// Example 3:
//
// Input: s1 = "kelb", s2 = "kelb"
// Output: true
// Explanation: The two strings are already equal, so no string swap operation is required.
//
//
// Constraints:
//
// 1 <= s1.length, s2.length <= 100
// s1.length == s2.length
// s1 and s2 consist of only lowercase English letters.
//
// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
struct Solution;
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let mut freq: [i32; 26] = [0; 26];
        for ch in s1.chars() {
            let ch_ascii = (ch as u8 - b'a') as usize;
            freq[ch_ascii] += 1;
        }
        for ch in s2.chars() {
            let ch_ascii = (ch as u8 - b'a') as usize;
            freq[ch_ascii] -= 1;
        }
        for &ch in freq.iter() {
            if ch != 0 {
                return false;
            }
        }
        let diff = s1.chars().zip(s2.chars()).fold(
            0,
            |acc, (c1, c2)| {
                if c1 != c2 {
                    acc + 1
                } else {
                    acc
                }
            },
        );
        diff == 0 || diff == 2
    }
}
