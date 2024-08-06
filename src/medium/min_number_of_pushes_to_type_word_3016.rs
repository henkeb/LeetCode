// Solution has
// Time complexity: O(nlog(n))
// Space complexity: O(1)
//
// You are given a string word containing lowercase English letters.
//
// Telephone keypads have keys mapped with distinct collections of lowercase English letters, which can be used to form words by pushing them. For example, the key 2 is mapped with ["a","b","c"], we need to push the key one time to type "a", two times to type "b", and three times to type "c" .
//
// It is allowed to remap the keys numbered 2 to 9 to distinct collections of letters. The keys can be remapped to any amount of letters, but each letter must be mapped to exactly one key. You need to find the minimum number of times the keys will be pushed to type the string word.
//
// Return the minimum number of pushes needed to type word after remapping the keys.
//
// An example mapping of letters to keys on a telephone keypad is given below. Note that 1, *, #, and 0 do not map to any letters.
struct Solution;
impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut frequency = [0; 26];
        const BUTTON: usize = 8;
        let mut pushes: usize = 0;
        const A: u8 = b'a';
        for c in word.chars() {
            frequency[(c as u8 - A) as usize] += 1;
        }
        frequency.sort_unstable_by(|a, b| b.cmp(a));
        for (i, freq) in frequency.iter().enumerate() {
            pushes += ((i / BUTTON) + 1) * freq;
        }
        pushes as i32
    }
}
