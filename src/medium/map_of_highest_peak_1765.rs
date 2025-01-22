use std::collections::{HashSet, VecDeque};

// You are given an integer matrix isWater of size m x n that represents a map of land and water cells.
//
// If isWater[i][j] == 0, cell (i, j) is a land cell.
// If isWater[i][j] == 1, cell (i, j) is a water cell.
// You must assign each cell a height in a way that follows these rules:
//
// The height of each cell must be non-negative.
// If the cell is a water cell, its height must be 0.
// Any two adjacent cells must have an absolute height difference of at most 1. A cell is adjacent to another cell if the former is directly north, east, south, or west of the latter (i.e., their sides are touching).
// Find an assignment of heights such that the maximum height in the matrix is maximized.
//
// Return an integer matrix height of size m x n where height[i][j] is cell (i, j)'s height. If there are multiple solutions, return any of them.
//
//
//
// Example 1:
//
//
//
// Input: isWater = [[0,1],[0,0]]
// Output: [[1,0],[2,1]]
// Explanation: The image shows the assigned heights of each cell.
// The blue cell is the water cell, and the green cells are the land cells.
// Example 2:
//
//
//
// Input: isWater = [[0,0,1],[1,0,0],[0,0,0]]
// Output: [[1,1,0],[0,1,1],[1,2,2]]
// Explanation: A height of 2 is the maximum possible height of any assignment.
// Any height assignment that has a maximum height of 2 while still meeting the rules will also be accepted.
//
//
// Constraints:
//
// m == isWater.length
// n == isWater[i].length
// 1 <= m, n <= 1000
// isWater[i][j] is 0 or 1.
// There is at least one water cell.
struct Solution;
impl Solution {
    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = is_water.len();
        let n = is_water[0].len();
        let mut height_map: Vec<Vec<i32>> = vec![vec![-1; n]; m];
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: VecDeque<(i32, Vec<(usize, usize)>)> = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    queue.push_back((0, vec![(i, j)]));
                }
            }
        }
        let m = m as isize;
        let n = n as isize;
        while let Some((height, mut positions)) = queue.pop_front() {
            while let Some((i, j)) = positions.pop() {
                if visited.contains(&(i, j)) {
                    continue;
                }

                visited.insert((i, j));
                height_map[i][j] = height;

                let mut new_positions: Vec<(usize, usize)> = Vec::new();

                for dir in Self::DIRS {
                    let new_pos = ((i as isize + dir.0), (j as isize + dir.1));
                    if new_pos.0 >= 0 && new_pos.0 < m {
                        if new_pos.1 >= 0 && new_pos.1 < n {
                            new_positions.push((new_pos.0 as usize, new_pos.1 as usize));
                        }
                    }
                }
                if !new_positions.is_empty() {
                    queue.push_back((height + 1, new_positions));
                }
            }
        }
        height_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::highest_peak(vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]]),
            vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]]
        )
    }
}
