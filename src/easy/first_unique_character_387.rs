// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given a string s, find the first non-repeating character in it and return its index. If it does not exist, return -1.

struct Solution;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        if s.is_empty() {
            return -1;
        }

        use std::collections::HashMap;
        let mut char_map: HashMap<char, (usize, usize)> = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            match char_map.get(&c) {
                None => {
                    char_map.insert(c, (i, 1));
                }
                Some((char, count)) => {
                    char_map.insert(c, (*char, count + 1));
                }
            }
        }
        let mut first_non_repeating = usize::MAX;
        for (_, (position, count)) in char_map.iter() {
            if *count == 1 && *position < first_non_repeating {
                first_non_repeating = *position;
            }
        }
        if first_non_repeating == usize::MAX {
            return -1;
        }
        first_non_repeating as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(Solution::first_uniq_char("bubba".to_string()), 1);
    }

    #[test]
    fn second() {
        assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);
    }
}
