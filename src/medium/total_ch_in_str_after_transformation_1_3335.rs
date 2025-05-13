// You are given a string s and an integer t, representing the number of transformations to perform. In one transformation, every character in s is replaced according to the following rules:
//
// If the character is 'z', replace it with the string "ab".
// Otherwise, replace it with the next character in the alphabet. For example, 'a' is replaced with 'b', 'b' is replaced with 'c', and so on.
// Return the length of the resulting string after exactly t transformations.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
//
//
// Example 1:
//
// Input: s = "abcyy", t = 2
//
// Output: 7
//
// Explanation:
//
// First Transformation (t = 1):
// 'a' becomes 'b'
// 'b' becomes 'c'
// 'c' becomes 'd'
// 'y' becomes 'z'
// 'y' becomes 'z'
// String after the first transformation: "bcdzz"
// Second Transformation (t = 2):
// 'b' becomes 'c'
// 'c' becomes 'd'
// 'd' becomes 'e'
// 'z' becomes "ab"
// 'z' becomes "ab"
// String after the second transformation: "cdeabab"
// Final Length of the string: The string is "cdeabab", which has 7 characters.
// Example 2:
//
// Input: s = "azbk", t = 1
//
// Output: 5
//
// Explanation:
//
// First Transformation (t = 1):
// 'a' becomes 'b'
// 'z' becomes "ab"
// 'b' becomes 'c'
// 'k' becomes 'l'
// String after the first transformation: "babcl"
// Final Length of the string: The string is "babcl", which has 5 characters.
//
//
// Constraints:
//
// 1 <= s.length <= 105
// s consists only of lowercase English letters.
// 1 <= t <= 105
struct Solution;
impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let mut ch_freq: [usize; 26] = [0; 26];
        const LARGE: usize = 1_000_000_007;
        for ch in s.chars() {
            ch_freq[(ch as usize) - b'a' as usize] += 1;
        }
        for _ in 0..t {
            let mut update: [usize; 26] = [0; 26];
            for i in 0..26 {
                if ch_freq[i] > 0 {
                    update[(i + 1) % 26] += ch_freq[i] % LARGE;
                    if i + 1 == 26 {
                        update[1] += ch_freq[i] % LARGE;
                    }
                }
            }
            ch_freq = update;
        }
        (ch_freq.iter().sum::<usize>() % LARGE) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::length_after_transformations("abcyy".to_string(), 2),
            7
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::length_after_transformations("jqktcurgdvlibczdsvnsg".to_string(), 7517),
            79_033_769
        );
    }
}
