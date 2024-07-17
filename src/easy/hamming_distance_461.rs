// The Hamming distance between two integers is the number of positions at which the corresponding bits are different.
// Solution has
// Time complexity: O(1)
// Space complexity: O(1)
//
//
// Given two integers x and y, return the Hamming distance between them.
struct Solution;
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hamm() {
        assert_eq!(Solution::hamming_distance(1, 4), 2)
    }
    #[test]
    fn hammer() {
        assert_eq!(Solution::hamming_distance(3, 1), 1)
    }
}
