// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Reverse a string recursively
//
struct Solution;
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        match s.len() {
            0 => (),
            l => Self::swap(s, 0, l - 1),
        }
    }
    fn swap(s: &mut Vec<char>, begin: usize, end: usize) {
        if begin >= end {
            return;
        }
        s.swap(begin, end);
        Self::swap(s, begin + 1, end - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }

    #[test]
    fn test_reverse_string2() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }

    #[test]
    fn test_reverse_string3() {
        let mut s = vec![];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec![]);
    }
}
