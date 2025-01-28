use std::collections::HashSet;

// You are given a 0-indexed 2D matrix grid of size m x n, where (r, c) represents:
//
// A land cell if grid[r][c] = 0, or
// A water cell containing grid[r][c] fish, if grid[r][c] > 0.
// A fisher can start at any water cell (r, c) and can do the following operations any number of times:
//
// Catch all the fish at cell (r, c), or
// Move to any adjacent water cell.
// Return the maximum number of fish the fisher can catch if he chooses his starting cell optimally, or 0 if no water cell exists.
//
// An adjacent cell of the cell (r, c), is one of the cells (r, c + 1), (r, c - 1), (r + 1, c) or (r - 1, c) if it exists.
//
//
//
// Example 1:
//
//
// Input: grid = [[0,2,1,0],[4,0,0,3],[1,0,0,4],[0,3,2,0]]
// Output: 7
// Explanation: The fisher can start at cell (1,3) and collect 3 fish, then move to cell (2,3) and collect 4 fish.
// Example 2:
//
//
// Input: grid = [[1,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,1]]
// Output: 1
// Explanation: The fisher can start at cells (0,0) or (3,3) and collect a single fish.
//
//
// Constraints:
//
// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 10
// 0 <= grid[i][j] <= 10
//
// Solution has
// Time complexity: O(n*m)
// Space complexity: O(n*m)
//
struct Solution;
impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        let mut queue: Vec<(i32, i32)> = Vec::new();

        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut max_fish = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != 0 {
                    queue.push((i as i32, j as i32));
                    let mut fish = grid[i][j];
                    visited.insert((i as i32, j as i32));
                    while let Some((prow, pcol)) = queue.pop() {
                        let prev_len = visited.len();
                        for dir in DIRS.iter() {
                            let (nr, nc) = ((prow as i32 + dir.0), (pcol as i32 + dir.1));
                            if (nr >= 0 && nr < m as i32) && (nc >= 0 && nc < n as i32) {
                                if grid[nr as usize][nc as usize] != 0 {
                                    if !visited.contains(&(nr, nc)) {
                                        fish += grid[nr as usize][nc as usize];
                                        queue.push((nr, nc));
                                        visited.insert((nr, nc));
                                    }
                                }
                            }
                        }
                        if visited.len() == prev_len {
                            max_fish = max_fish.max(fish);
                        }
                    }
                }
            }
        }
        max_fish
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_max_fish(vec![
                vec![0, 2, 1, 0],
                vec![4, 0, 0, 3],
                vec![1, 0, 0, 4],
                vec![0, 3, 2, 0]
            ]),
            7
        );
    }
}
