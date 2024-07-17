// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
// Given two binary strings a and b, return their sum as a binary string.
//
struct Solution;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (shortest, longest): (&str, &str) = match a.len().cmp(&b.len()) {
            std::cmp::Ordering::Less => (&a, &b),
            std::cmp::Ordering::Equal => (&a, &b),
            std::cmp::Ordering::Greater => (&b, &a),
        };
        let mut result: String = String::new();
        let mut carry = false;
        let diff = longest.len() - shortest.len();

        for i in (0..(longest.len())).rev() {
            let short = match i.cmp(&diff) {
                std::cmp::Ordering::Less => None,
                _ => shortest.chars().nth(i - diff),
            };
            Self::add_bit(&short, &longest.chars().nth(i), &mut result, &mut carry);
        }
        if carry {
            result.push('1');
        }
        result.chars().rev().collect()
    }

    fn add_bit(bit_a: &Option<char>, bit_b: &Option<char>, result: &mut String, carry: &mut bool) {
        match (bit_a, bit_b) {
            (Some(bit_a), Some(bit_b)) if *bit_a == '1' && *bit_b == '1' => {
                if *carry {
                    result.push('1');
                    *carry = true;
                } else {
                    result.push('0');
                    *carry = true;
                }
            }
            (Some(bit_a), Some(bit_b)) if *bit_a == '0' && *bit_b == '0' => {
                if *carry {
                    result.push('1');
                } else {
                    result.push('0');
                }
                *carry = false;
            }
            (Some(..), Some(..)) => {
                if *carry {
                    result.push('0');
                    *carry = true;
                } else {
                    result.push('1');
                    *carry = false;
                }
            }
            (Some(a_bit), None) => {
                if *carry {
                    if *a_bit == '1' {
                        result.push('0');
                        *carry = true;
                    } else {
                        result.push('1');
                        *carry = false;
                    }
                } else {
                    result.push(*a_bit);
                    *carry = false;
                }
            }
            (None, Some(b_bit)) => {
                if *carry {
                    if *b_bit == '1' {
                        result.push('0');
                        *carry = true;
                    } else {
                        result.push('1');
                        *carry = false;
                    }
                } else {
                    result.push(*b_bit);
                    *carry = false;
                }
            }
            (None, None) => {
                if *carry {
                    result.push('1');
                    *carry = false;
                }
            }
        }
    }

    fn parse_string(s: &str) -> u32 {
        s.chars()
            .rev()
            .enumerate()
            .map(|(place, val)| val.to_digit(10).unwrap_or(0) << place)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        assert_eq!(Solution::parse_string("1"), 1);
        assert_eq!(Solution::parse_string("10"), 2);
        assert_eq!(Solution::parse_string("101"), 5);
    }

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
    }

    #[test]
    fn it_works2() {
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }

    #[test]
    fn it_works3() {
        assert_eq!(
            Solution::add_binary("0".to_string(), "0".to_string()),
            "0".to_string()
        );
    }

    #[test]
    fn it_works4() {
        assert_eq!(
            Solution::add_binary("1".to_string(), "1".to_string()),
            "10".to_string()
        );
    }

    #[test]
    fn testing() {
        assert_eq!(
            Solution::add_binary("100".to_string(), "110010".to_string()),
            "110110".to_string()
        );
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::add_binary("10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string(), "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string()), "110111101100010011000101110110100000011101000101011001000011011000001100011110011010010011000000000".to_string())
    }
}
