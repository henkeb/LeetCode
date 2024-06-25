struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut frequency: [i32; 52] = [0; 52];
        for c in s.chars() {
            if c.is_lowercase() {
                frequency[c as usize - 97 + 26] += 1;
            } else {
                frequency[c as usize - 65] += 1;
            }
        }
        let max_odd: i32 = *frequency
            .iter()
            .filter(|val| *val % 2 != 0)
            .max()
            .unwrap_or(&0);
        let mut other_odd: i32 = frequency
            .iter()
            .filter(|val| *val % 2 != 0)
            .map(|val| *val - 1)
            .sum::<i32>();
        if other_odd != 0 {
            other_odd -= max_odd - 1;
        }
        let even: i32 = frequency.iter().filter(|val| *val % 2 == 0).sum();
        max_odd + other_odd + even
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_7() {
        assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
    }

    #[test]
    fn long_1() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
    }

    #[test]
    fn long_a() {
        assert_eq!(Solution::longest_palindrome("aaaAaaaa".to_string()), 7);
    }
    #[test]
    fn long_bb() {
        assert_eq!(Solution::longest_palindrome("bb".to_string()), 2);
    }
}
