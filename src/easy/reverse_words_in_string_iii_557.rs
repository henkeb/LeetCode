// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.
struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .map(|word| word.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
