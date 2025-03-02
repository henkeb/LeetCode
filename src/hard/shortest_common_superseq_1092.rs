// Given two strings str1 and str2, return the shortest string that has both str1 and str2 as subsequences. If there are multiple valid strings, return any of them.
//
// A string s is a subsequence of string t if deleting some number of characters from t (possibly 0) results in the string s.
//
//
//
// Example 1:
//
// Input: str1 = "abac", str2 = "cab"
// Output: "cabac"
// Explanation:
// str1 = "abac" is a subsequence of "cabac" because we can delete the first "c".
// str2 = "cab" is a subsequence of "cabac" because we can delete the last "ac".
// The answer provided is the shortest such string that satisfies these properties.
// Example 2:
//
// Input: str1 = "aaaaaaaa", str2 = "aaaaaaaa"
// Output: "aaaaaaaa"
//
//
// Constraints:
//
// 1 <= str1.length, str2.length <= 1000
// str1 and str2 consist of lowercase English letters.
struct Solution;
impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let chars1: Vec<char> = str1.chars().collect();
        let chars2: Vec<char> = str2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();

        let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

        // Calculate the length of the shortest common supersequence
        for i in 0..=len1 {
            for j in 0..=len2 {
                if i == 0 || j == 0 {
                    dp[i][j] = i + j;
                } else if chars1[i - 1] == chars2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = std::cmp::min(dp[i - 1][j] + 1, dp[i][j - 1] + 1);
                }
            }
        }

        // Reconstruct the shortest common supersequence
        let mut i = len1;
        let mut j = len2;
        let mut result = Vec::new();

        while i > 0 && j > 0 {
            if chars1[i - 1] == chars2[j - 1] {
                result.push(chars1[i - 1]);
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] < dp[i][j - 1] {
                result.push(chars1[i - 1]);
                i -= 1;
            } else {
                result.push(chars2[j - 1]);
                j -= 1;
            }
        }

        while i > 0 {
            result.push(chars1[i - 1]);
            i -= 1;
        }

        while j > 0 {
            result.push(chars2[j - 1]);
            j -= 1;
        }

        result.reverse();
        result.iter().collect()
    }
}
