// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given the string s, return the size of the longest substring containing each vowel an even number of times. That is, 'a', 'e', 'i', 'o', and 'u' must appear an even number of times.
//
//
//
// Example 1:
//
// Input: s = "eleetminicoworoep"
// Output: 13
// Explanation: The longest substring is "leetminicowor" which contains two each of the vowels: e, i and o and zero of the vowels: a and u.
//
// Example 2:
//
// Input: s = "leetcodeisgreat"
// Output: 5
// Explanation: The longest substring is "leetc" which contains two e's.
//
// Example 3:
//
// Input: s = "bcbcbc"
// Output: 6
// Explanation: In this case, the given string "bcbcbc" is the longest because all vowels: a, e, i, o and u appear zero times.
//
//
//
// Constraints:
//
//     1 <= s.length <= 5 x 10^5
//     s contains only lowercase English letters.
//
struct Solution;
impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        const CHARACTER_MAP: [i32; 26] = [
            1, 0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0,
        ];
        let mut mp = [-1; 32];
        let mut prefix_xor = 0;
        let mut longest_substring = 0;
        mp[0] = 0;

        for (i, c) in s.chars().enumerate() {
            prefix_xor ^= CHARACTER_MAP[(c as u8 - b'a') as usize];
            if mp[prefix_xor as usize] == -1 {
                mp[prefix_xor as usize] = i as i32 + 1;
            } else {
                longest_substring = longest_substring.max(i as i32 - mp[prefix_xor as usize] + 1);
            }
        }

        longest_substring
    }
}
