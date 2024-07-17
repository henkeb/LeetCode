// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// You are given a license key represented as a string s that consists of only alphanumeric characters and dashes. The string is separated into n + 1 groups by n dashes. You are also given an integer k.
//
// We want to reformat the string s such that each group contains exactly k characters, except for the first group, which could be shorter than k but still must contain at least one character. Furthermore, there must be a dash inserted between two groups, and you should convert all lowercase letters to uppercase.
//
// Return the reformatted license key.
struct Solution;
impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let no_dashes = s.replace('-', "").to_ascii_uppercase();
        let mut output = String::new();
        let mut count = 0;

        no_dashes.chars().rev().for_each(|ch| {
            if count == k {
                output.push('-');
                count = 0;
            }
            output.push(ch);
            count += 1;
        });
        output.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn license_key_formatting() {
        assert_eq!(
            Solution::license_key_formatting(String::from("5F3Z-2e-9-w"), 4),
            String::from("5F3Z-2E9W")
        );
    }

    #[test]
    fn license_key_formatting2() {
        assert_eq!(
            Solution::license_key_formatting(String::from("2-5g-3-J"), 2),
            String::from("2-5G-3J")
        );
    }
}
