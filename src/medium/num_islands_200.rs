// Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.
//
// An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
//
//
//
// Example 1:
//
// Input: grid = [
//   ["1","1","1","1","0"],
//   ["1","1","0","1","0"],
//   ["1","1","0","0","0"],
//   ["0","0","0","0","0"]
// ]
// Output: 1
// Example 2:
//
// Input: grid = [
//   ["1","1","0","0","0"],
//   ["1","1","0","0","0"],
//   ["0","0","1","0","0"],
//   ["0","0","0","1","1"]
// ]
// Output: 3
//
//
// Constraints:
//
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 300
// grid[i][j] is '0' or '1'.
struct Solution;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        use std::collections::HashSet;

        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut queue: Vec<(i32, i32)> = Vec::new();

        let row = grid.len() as i32;
        let col = grid[0].len() as i32;

        let mut islands = 0;

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == '1' {
                    if !visited.contains(&(x as i32, y as i32)) {
                        islands += 1;
                        let pos_x = x as i32;
                        let pos_y = y as i32;
                        queue.push((pos_x, pos_y));

                        while let Some((x, y)) = queue.pop() {
                            visited.insert((x, y));
                            for (mut new_x, mut new_y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                                new_x += x;
                                new_y += y;
                                if new_x >= 0 && new_x < col && new_y >= 0 && new_y < row {
                                    if grid[new_y as usize][new_x as usize] == '1'
                                        && !visited.contains(&(new_x, new_y))
                                    {
                                        queue.push((new_x, new_y));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        islands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1'],
            ]),
            3
        )
    }
}
