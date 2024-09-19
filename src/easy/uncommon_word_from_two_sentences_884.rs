// Solution has
// Time complexity: O(n+m)
// Space complexity: O(n+m)
//
// A sentence is a string of single-space separated words where each word consists only of lowercase letters.
//
// A word is uncommon if it appears exactly once in one of the sentences, and does not appear in the other sentence.
//
// Given two sentences s1 and s2, return a list of all the uncommon words. You may return the answer in any order.
//
//
//
// Example 1:
//
// Input: s1 = "this apple is sweet", s2 = "this apple is sour"
//
// Output: ["sweet","sour"]
//
// Explanation:
//
// The word "sweet" appears only in s1, while the word "sour" appears only in s2.
//
// Example 2:
//
// Input: s1 = "apple apple", s2 = "banana"
//
// Output: ["banana"]
//
//
//
// Constraints:
//
//     1 <= s1.length, s2.length <= 200
//     s1 and s2 consist of lowercase English letters and spaces.
//     s1 and s2 do not have leading or trailing spaces.
//     All the words in s1 and s2 are separated by a single space.
//
struct Solution;
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        use std::collections::HashMap;

        let mut m: HashMap<&str, usize> = HashMap::new();
        s1.split_whitespace()
            .chain(s2.split_whitespace())
            .fold(&mut m, |map, word| {
                map.entry(word)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
                map
            });
        m.iter()
            .filter(|(_, value)| **value == 1_usize)
            .map(|(key, _)| key.to_string())
            .collect()
    }
}
