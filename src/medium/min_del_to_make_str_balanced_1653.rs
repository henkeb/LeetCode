// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// You are given a string s consisting only of characters 'a' and 'b'​​​​.
//
// You can delete any number of characters in s to make s balanced. s is balanced if there is no pair of indices (i,j) such that i < j and s[i] = 'b' and s[j]= 'a'.
//
// Return the minimum number of deletions needed to make s balanced.
struct Solution;
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut current = s
            .chars()
            .fold(0, |acc, ch| if ch == 'a' { acc + 1 } else { acc });
        let mut output = current;
        for ch in s.chars() {
            if ch == 'a' {
                current -= 1;
            } else {
                current += 1;
            }
            output = output.min(current);
        }
        output
    }
}
