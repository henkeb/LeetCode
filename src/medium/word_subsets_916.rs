// You are given two string arrays words1 and words2.
//
// A string b is a subset of string a if every letter in b occurs in a including multiplicity.
//
// For example, "wrr" is a subset of "warrior" but is not a subset of "world".
// A string a from words1 is universal if for every string b in words2, b is a subset of a.
//
// Return an array of all the universal strings in words1. You may return the answer in any order.
//
//
//
// Example 1:
//
// Input: words1 = ["amazon","apple","facebook","google","leetcode"], words2 = ["e","o"]
// Output: ["facebook","google","leetcode"]
// Example 2:
//
// Input: words1 = ["amazon","apple","facebook","google","leetcode"], words2 = ["l","e"]
// Output: ["apple","google","leetcode"]
//
//
// Constraints:
//
// 1 <= words1.length, words2.length <= 104
// 1 <= words1[i].length, words2[i].length <= 10
// words1[i] and words2[i] consist only of lowercase English letters.
// All the strings of words1 are unique.
struct Solution;
impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        let mut subs_frequency = [0_usize; 26];
        for word2 in words2.iter() {
            let mut frequency_subset = [0; 26];
            for c in word2.chars() {
                frequency_subset[(c as u8 - b'a') as usize] += 1;
            }
            for (i, &freq) in frequency_subset.iter().enumerate() {
                if freq > subs_frequency[i] {
                    subs_frequency[i] = freq;
                }
            }
        }
        for word in words1.iter() {
            let mut frequency = [0_usize; 26];
            for c in word.chars() {
                frequency[(c as u8 - b'a') as usize] += 1;
            }
            if subs_frequency
                .iter()
                .zip(frequency.iter())
                .any(|(sub, word)| sub > word)
            {
                continue;
            }

            output.push(word.to_string());
        }
        output
    }
}
