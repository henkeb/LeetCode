// Write a function that reverses a string. The input string is given as an array of characters s.
// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
//
// You must do this by modifying the input array in-place with O(1) extra memory.
struct Solution;
impl Solution {
    pub fn reverse_string(s: &mut [char]) {
        let len = s.len();
        for i in 0..len / 2 {
            s.swap(i, len - 1 - i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_hello() {
        let mut input = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut input);
        assert_eq!(input, vec!['o', 'l', 'l', 'e', 'h']);
    }
}
