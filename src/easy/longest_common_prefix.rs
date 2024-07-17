// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Write a function to find the longest common prefix string amongst an array of strings.
//
// If there is no common prefix, return an empty string "".
struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs[0].clone();
        for s in strs.iter().skip(1) {
            while !s.starts_with(&prefix) {
                prefix.pop();
                if prefix.is_empty() {
                    return String::new();
                }
            }
        }
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl".to_string()
        );
    }

    #[test]
    fn test_longest_common_prefix_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            "".to_string()
        );
    }
    #[test]
    fn test_longest_common_prefix_3() {
        assert_eq!(
            Solution::longest_common_prefix(vec!["ab".to_string(), "a".to_string()]),
            "a"
        )
    }
}
