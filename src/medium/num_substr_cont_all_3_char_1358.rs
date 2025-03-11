// Given a string s consisting only of characters a, b and c.
//
// Return the number of substrings containing at least one occurrence of all these characters a, b and c.
//
//
//
// Example 1:
//
// Input: s = "abcabc"
// Output: 10
// Explanation: The substrings containing at least one occurrence of the characters a, b and c are "abc", "abca", "abcab", "abcabc", "bca", "bcab", "bcabc", "cab", "cabc" and "abc" (again).
// Example 2:
//
// Input: s = "aaacb"
// Output: 3
// Explanation: The substrings containing at least one occurrence of the characters a, b and c are "aaacb", "aacb" and "acb".
// Example 3:
//
// Input: s = "abc"
// Output: 1
//
//
// Constraints:
//
// 3 <= s.length <= 5 x 10^4
// s only consists of a, b or c characters.
//
// Solution has
// Time complexity: O(n^2)
// Space complexity: O(1)
//
struct Solution;
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut abc_freq = [0; 3];
        let (mut l, mut r) = (0, 0);
        let mut output = 0;
        while r < s.len() {
            abc_freq[(s.as_bytes()[r] - b'a') as usize] += 1;
            while Self::is_abc_valid(&abc_freq) {
                output += s.len() - r;
                abc_freq[(s.as_bytes()[l] - b'a') as usize] -= 1;
                l += 1;
            }
            r += 1;
        }
        output as i32
    }

    fn is_abc_valid(abc_freq: &[i32]) -> bool {
        for val in abc_freq {
            if *val == 0 {
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
    fn test1() {
        assert_eq!(Solution::number_of_substrings("aaacb".to_string()), 3)
    }
}
