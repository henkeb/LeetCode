// A happy string is a string that:
//
// consists only of letters of the set ['a', 'b', 'c'].
// s[i] != s[i + 1] for all values of i from 1 to s.length - 1 (string is 1-indexed).
// For example, strings "abc", "ac", "b" and "abcbabcbcb" are all happy strings and strings "aa", "baa" and "ababbc" are not happy strings.
//
// Given two integers n and k, consider a list of all happy strings of length n sorted in lexicographical order.
//
// Return the kth string of this list or return an empty string if there are less than k happy strings of length n.
//
//
//
// Example 1:
//
// Input: n = 1, k = 3
// Output: "c"
// Explanation: The list ["a", "b", "c"] contains all happy strings of length 1. The third string is "c".
// Example 2:
//
// Input: n = 1, k = 4
// Output: ""
// Explanation: There are only 3 happy strings of length 1.
// Example 3:
//
// Input: n = 3, k = 9
// Output: "cab"
// Explanation: There are 12 different happy string of length 3 ["aba", "abc", "aca", "acb", "bab", "bac", "bca", "bcb", "cab", "cac", "cba", "cbc"]. You will find the 9th string = "cab"
//
//
// Constraints:
//
// 1 <= n <= 10
// 1 <= k <= 100
struct Solution;
impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        fn backtrack(n: i32, active_string: &mut String, k: &mut i32) {
            if n == 0 {
                *k -= 1;
                return;
            }
            for ch in "abc".chars() {
                if let Some(prev) = active_string.chars().next_back() {
                    if prev == ch {
                        continue;
                    }
                }
                active_string.push(ch);
                backtrack(n - 1, active_string, k);
                if *k == 0 {
                    return;
                }
                active_string.pop();
            }
        }
        let mut active_string: String = String::new();
        let mut k = k;
        backtrack(n, &mut active_string, &mut k);
        active_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_happy_string(3, 9), "cab".to_string())
    }
}
