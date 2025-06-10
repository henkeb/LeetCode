// You are given a string s consisting of lowercase English letters.
//
// Your task is to find the maximum difference diff = a1 - a2 between the frequency of characters a1 and a2 in the string such that:
//
// a1 has an odd frequency in the string.
// a2 has an even frequency in the string.
// Return this maximum difference.
//
//
//
// Example 1:
//
// Input: s = "aaaaabbc"
//
// Output: 3
//
// Explanation:
//
// The character 'a' has an odd frequency of 5, and 'b' has an even frequency of 2.
// The maximum difference is 5 - 2 = 3.
// Example 2:
//
// Input: s = "abcabcab"
//
// Output: 1
//
// Explanation:
//
// The character 'a' has an odd frequency of 3, and 'c' has an even frequency of 2.
// The maximum difference is 3 - 2 = 1.
//
//
// Constraints:
//
// 3 <= s.length <= 100
// s consists only of lowercase English letters.
// s contains at least one character with an odd frequency and one with an even frequency.
//
struct Solution;
impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut freq = [0i8; 26];
        s.chars().for_each(|c| {
            freq[(c as u8 - b'a') as usize] += 1;
        });
        let min = freq
            .iter()
            .filter(|&elem| *elem != 0 && *elem % 2 == 0)
            .min();
        let max = freq.iter().filter(|&elem| *elem % 2 != 0).max();

        match (max, min) {
            (Some(max), Some(min)) => (*max - *min) as i32,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_difference("aaaaabbc".to_string()), 3)
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_difference("abcabcab".to_string()), 1)
    }
}
