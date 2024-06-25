// The complement of an integer is the integer you get when you flip all the 0's to 1's and all the 1's to 0's in its binary representation.
//
//     For example, The integer 5 is "101" in binary and its complement is "010" which is the integer 2.
//
// Given an integer num, return its complement.
struct Solution;
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut mask = 1 << 31;
        let mut msb = 0;
        for pos_from_right in 0..32 {
            if num & mask != 0 {
                msb = 32 - pos_from_right;
                break;
            }
            mask >>= 1;
        }
        !num & i32::MAX >> (31 - msb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_complement() {
        assert_eq!(Solution::find_complement(5), 2);
    }

    #[test]
    fn test_find_complement2() {
        assert_eq!(Solution::find_complement(1), 0);
    }
}
