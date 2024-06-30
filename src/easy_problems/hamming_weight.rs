// Solution has
// Time complexity: O(1)
// Space complexity: O(1)
//
// Write a function that takes the binary representation of a positive integer and returns the number of
// set bits
// it has (also known as the Hamming weight).

struct Solution;
impl Solution {
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut count = 0;
        for _ in 0..32 {
            if n & 1 == 1 {
                count += 1;
            }
            n >>= 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::hamming_weight(11), 3);
    }

    #[test]
    fn it_works2() {
        assert_eq!(Solution::hamming_weight(128), 1);
    }

    #[test]
    fn it_works3() {
        assert_eq!(Solution::hamming_weight(2147483645), 30);
    }
}
