// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given an integer n, return an array ans of length n + 1 such that for each i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.
struct Solution;
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut output: Vec<i32> = vec![0; n as usize + 1];
        (0..=n).for_each(|val| output[val as usize] = output[(val >> 1) as usize] + (val & 1));
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn test_count_bits2() {
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn test_count_bits3() {
        assert_eq!(Solution::count_bits(0), vec![0]);
    }

    #[test]
    fn test_count_bits4() {
        assert_eq!(Solution::count_bits(1), vec![0, 1]);
    }
}
