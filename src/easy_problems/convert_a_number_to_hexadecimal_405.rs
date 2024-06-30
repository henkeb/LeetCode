// Solution has
// Time complexity: O(log16(n))
// Space complexity: O(log16(n))
//
use core::panic;

struct Solution;
impl Solution {
    pub fn to_hex(num: i32) -> String {
        let mut dec = num as u32;
        let mut output: Vec<char> = Vec::new();
        while dec > 0 {
            output.push(Self::base_sixteen_to_hex(dec % 16));
            dec /= 16;
        }
        if output.is_empty() {
            output.push('0');
        }
        output.into_iter().rev().collect()
    }

    fn base_sixteen_to_hex(num: u32) -> char {
        match num {
            val if val < 10 => (val as u8 + 48u8) as char,
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            14 => 'e',
            15 => 'f',
            _ => panic!("Should never go here"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_26() {
        assert_eq!(Solution::to_hex(26), "1a");
    }

    #[test]
    fn test_negative_one() {
        assert_eq!(Solution::to_hex(-1), "ffffffff");
    }
}
