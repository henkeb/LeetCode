// You are given a string s.
//
// Your task is to remove all digits by doing this operation repeatedly:
//
// Delete the first digit and the closest non-digit character to its left.
// Return the resulting string after removing all digits.
//
// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
struct Solution;
impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut stack = Vec::new();

        for ch in s.chars() {
            if ch.is_digit(10) {
                stack.pop();
            } else {
                stack.push(ch);
            }
        }

        stack.iter().collect()
    }
}
