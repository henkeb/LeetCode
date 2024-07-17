// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given a string s, reverse only all the vowels in the string and return it.
//
// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.
struct Solution;
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.is_empty() {
            return "".to_string();
        }

        let mut chars: Vec<char> = s.chars().collect();

        let mut start = 0;
        let mut end = chars.len() - 1;
        while start < end {
            if Self::check_wovel(&chars[start]) {
                while start < end {
                    if Self::check_wovel(&chars[end]) {
                        chars.swap(start, end);
                        end -= 1;
                        break;
                    }
                    end -= 1;
                }
            }
            start += 1;
        }
        chars.into_iter().collect()
    }

    fn check_wovel(c: &char) -> bool {
        matches!(c, 'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reverse_hello() {
        assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle");
    }

    fn reverse_leetcode() {
        assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede");
    }
}
