// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Reverse bits of a given 32 bits unsigned integer.

struct Solution;
impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut rev: u32 = 0;
        for _ in 0..32 {
            rev <<= 1;
            if x & 1 == 1 {
                rev ^= 1;
            }
            x >>= 1;
            println!("Rev: {rev:b}");
            println!("X: {x:b}");
        }
        rev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse_bits(43261596), 964176192);
    }

    #[test]
    fn it_works2() {
        assert_eq!(Solution::reverse_bits(1), 2147483648);
    }

    #[test]
    fn it_works3() {
        assert_eq!(Solution::reverse_bits(0), 0);
    }
}
