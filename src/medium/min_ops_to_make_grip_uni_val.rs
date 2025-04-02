// You are given a 2D integer grid of size m x n and an integer x. In one operation, you can add x to or subtract x from any element in the grid.
//
// A uni-value grid is a grid where all the elements of it are equal.
//
// Return the minimum number of operations to make the grid uni-value. If it is not possible, return -1.
//
//
//
// Example 1:
//
//
// Input: grid = [[2,4],[6,8]], x = 2
// Output: 4
// Explanation: We can make every element equal to 4 by doing the following:
// - Add x to 2 once.
// - Subtract x from 6 once.
// - Subtract x from 8 twice.
// A total of 4 operations were used.
// Example 2:
//
//
// Input: grid = [[1,5],[2,3]], x = 1
// Output: 5
// Explanation: We can make every element equal to 3.
// Example 3:
//
//
// Input: grid = [[1,2],[3,4]], x = 2
// Output: -1
// Explanation: It is impossible to make every element equal.
//
//
// Constraints:
//
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 105
// 1 <= m * n <= 105
// 1 <= x, grid[i][j] <= 104
struct Solution;
impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut values = grid.into_iter().flatten().collect::<Vec<i32>>();
        values.sort();
        let target = values[values.len() / 2];
        let mut output = 0;

        for val in values.into_iter() {
            match val.cmp(&target) {
                std::cmp::Ordering::Less => {
                    if (target - val) % x != 0 {
                        return -1;
                    }
                    output += (target - val) / x;
                }
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => {
                    if (val - target) % x != 0 {
                        return -1;
                    }
                    output += (val - target) / x;
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_operations(
                vec![
                    vec![1],
                    vec![2],
                    vec![3],
                    vec![2],
                    vec![6],
                    vec![7],
                    vec![9],
                    vec![9],
                    vec![5]
                ],
                2
            ),
            -1
        );
    }
}
