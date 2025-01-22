// You are given a 0-indexed 2D array grid of size 2 x n, where grid[r][c] represents the number of points at position (r, c) on the matrix. Two robots are playing a game on this matrix.
//
// Both robots initially start at (0, 0) and want to reach (1, n-1). Each robot may only move to the right ((r, c) to (r, c + 1)) or down ((r, c) to (r + 1, c)).
//
// At the start of the game, the first robot moves from (0, 0) to (1, n-1), collecting all the points from the cells on its path. For all cells (r, c) traversed on the path, grid[r][c] is set to 0. Then, the second robot moves from (0, 0) to (1, n-1), collecting the points on its path. Note that their paths may intersect with one another.
//
// The first robot wants to minimize the number of points collected by the second robot. In contrast, the second robot wants to maximize the number of points it collects. If both robots play optimally, return the number of points collected by the second robot.
//
//
//
// Example 1:
//
//
// Input: grid = [[2,5,4],[1,5,1]]
// Output: 4
// Explanation: The optimal path taken by the first robot is shown in red, and the optimal path taken by the second robot is shown in blue.
// The cells visited by the first robot are set to 0.
// The second robot will collect 0 + 0 + 4 + 0 = 4 points.
// Example 2:
//
//
// Input: grid = [[3,3,1],[8,5,2]]
// Output: 4
// Explanation: The optimal path taken by the first robot is shown in red, and the optimal path taken by the second robot is shown in blue.
// The cells visited by the first robot are set to 0.
// The second robot will collect 0 + 3 + 1 + 0 = 4 points.
// Example 3:
//
//
// Input: grid = [[1,3,1,15],[1,3,3,1]]
// Output: 7
// Explanation: The optimal path taken by the first robot is shown in red, and the optimal path taken by the second robot is shown in blue.
// The cells visited by the first robot are set to 0.
// The second robot will collect 0 + 1 + 3 + 3 + 0 = 7 points.
//
//
// Constraints:
//
// grid.length == 2
// n == grid[r].length
// 1 <= n <= 5 * 104
// 1 <= grid[r][c] <= 105
//
struct Solution;
impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let mut prefix1: Vec<i64> = vec![0; grid[0].len() + 1];
        let mut prefix2: Vec<i64> = vec![0; grid[0].len() + 1];

        for i in 1..=grid[0].len() {
            prefix1[i] = prefix1[i - 1] + grid[0][i - 1] as i64;
        }
        let mut second_row_sum: i64 = 0;
        for (_, val) in grid[1].iter().enumerate() {
            second_row_sum += *val as i64;
        }
        prefix2[1] = second_row_sum;
        for i in 2..=grid[0].len() {
            prefix2[i] = prefix2[i - 1] - grid[1][i - 2] as i64;
        }

        let mut first_r: Vec<i64> = vec![0; grid[0].len() + 1];

        for i in 0..grid[0].len() {
            // first_r[i] = prefix1[i] + prefix2[i];
            let right: i64 = prefix1[prefix1.len() - 1] - prefix1[i + 1];
            let left: i64 = grid[1][..i].iter().map(|val| *val as i64).sum();
            first_r[i] = left.max(right);
        }

        *first_r[0..(first_r.len() - 1)].iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::grid_game(vec![vec![2, 5, 4], vec![1, 5, 1]]), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::grid_game(vec![vec![3, 3, 1], vec![8, 5, 2]]), 4);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::grid_game(vec![
                vec![20, 3, 20, 17, 2, 12, 15, 17, 4, 15],
                vec![20, 10, 13, 14, 15, 5, 2, 3, 14, 3]
            ]),
            63
        )
    }
}
