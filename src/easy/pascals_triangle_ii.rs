// Solution has
// Time complexity: O(n²)
// Space complexity: O(n)
//
// Given an integer rowIndex, return the rowIndexth (0-indexed) row of the Pascal's triangle.
//
// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:

use std::u128;

struct Solution;
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut current_row: Vec<i32> = vec![];

        for row in 0..=row_index {
            current_row = vec![];
            for col in 0..=row {
                current_row.push(Self::comb(row, col));
            }
        }
        current_row
    }
    fn comb(n: i32, k: i32) -> i32 {
        (((n - k + 1) as u128..=n as u128).product::<u128>() / (1..=k as u128).product::<u128>())
            as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::get_row(0), vec![1]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::get_row(1), vec![1, 1]);
    }

    #[test]
    fn testcomb() {
        assert_eq!(Solution::comb(10, 5), 252);
    }
}
