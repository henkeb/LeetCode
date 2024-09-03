// Solution has
// Time complexity: O(n²)
// Space complexity: O(mn)
//
// You are given two m x n binary matrices grid1 and grid2 containing only 0's (representing water) and 1's (representing land). An island is a group of 1's connected 4-directionally (horizontal or vertical). Any cells outside of the grid are considered water cells.
//
// An island in grid2 is considered a sub-island if there is an island in grid1 that contains all the cells that make up this island in grid2.
//
// Return the number of islands in grid2 that are considered sub-islands.
//
//
//
// Example 1:
//
// Input: grid1 = [[1,1,1,0,0],[0,1,1,1,1],[0,0,0,0,0],[1,0,0,0,0],[1,1,0,1,1]], grid2 = [[1,1,1,0,0],[0,0,1,1,1],[0,1,0,0,0],[1,0,1,1,0],[0,1,0,1,0]]
// Output: 3
// Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
// The 1s colored red in grid2 are those considered to be part of a sub-island. There are three sub-islands.
//
// Example 2:
//
// Input: grid1 = [[1,0,1,0,1],[1,1,1,1,1],[0,0,0,0,0],[1,1,1,1,1],[1,0,1,0,1]], grid2 = [[0,0,0,0,0],[1,1,1,1,1],[0,1,0,1,0],[0,1,0,1,0],[1,0,0,0,1]]
// Output: 2
// Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
// The 1s colored red in grid2 are those considered to be part of a sub-island. There are two sub-islands.
//
//
//
// Constraints:
//
//     m == grid1.length == grid2.length
//     n == grid1[i].length == grid2[i].length
//     1 <= m, n <= 500
//     grid1[i][j] and grid2[i][j] are either 0 or 1.
//
const DIR: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
struct Solution;
impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let (y_len, x_len) = (grid1.len(), grid1[0].len());
        let mut visited = vec![vec![false; x_len]; y_len];
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut sub_islands = 0;
        for y in 0..y_len {
            for x in 0..x_len {
                if grid2[y][x] == 1 && !visited[y][x] {
                    let mut is_sub_island = true;
                    stack.push((y, x));
                    while let Some((y, x)) = stack.pop() {
                        if visited[y][x] || grid2[y][x] == 0 {
                            continue;
                        }
                        if grid2[y][x] == 0 || grid1[y][x] == 0 {
                            is_sub_island = false;
                        }
                        visited[y][x] = true;
                        for (dx, dy) in DIR {
                            let new_x: i32 = x as i32 + dx;
                            let new_y: i32 = y as i32 + dy;
                            if new_y >= 0
                                && new_y < y_len as i32
                                && new_x >= 0
                                && new_x < x_len as i32
                            {
                                stack.push((new_y as usize, new_x as usize));
                            }
                        }

                        // if (y as i32 - 1) >= 0 {
                        //     stack.push(((y - 1), x));
                        // }
                        // if (y + 1) < y_len {
                        //     stack.push(((y + 1), x));
                        // }
                        // if (x as i32 - 1) >= 0 {
                        //     stack.push((y, (x - 1)));
                        // }
                        // if (x + 1) < x_len {
                        //     stack.push((y, (x + 1)));
                        // }
                    }
                    if is_sub_island {
                        sub_islands += 1;
                    }
                }
            }
        }
        sub_islands
    }
}
