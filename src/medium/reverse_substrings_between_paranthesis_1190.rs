// Solution has
// Time complexity: O(n²)
// Space complexity: O(n)
//
// You are given a string s that consists of lower case English letters and brackets.
//
// Reverse the strings in each pair of matching parentheses, starting from the innermost one.
//
// Your result should not contain any brackets.
struct Solution;
impl Solution {
    pub fn reverse_parentheses(mut s: String) -> String {
        let mut stack: Vec<usize> = Vec::new();
        let mut reverse_substrings: Vec<(usize, usize)> = Vec::new();

        s.chars().enumerate().for_each(|(i, c)| match c {
            '(' => stack.push(i),
            ')' => reverse_substrings.push((stack.pop().unwrap(), i)),
            _ => (),
        });

        for (start, end) in reverse_substrings {
            s.replace_range(
                start..=end,
                &s[start..=end].chars().rev().collect::<String>(),
            );
        }

        s = s.replace('(', "");
        s = s.replace(')', "");
        s
    }
}
