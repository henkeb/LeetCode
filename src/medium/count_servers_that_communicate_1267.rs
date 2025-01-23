// You are given a map of a server center, represented as a m * n integer matrix grid, where 1 means that on that cell there is a server and 0 means that it is no server. Two servers are said to communicate if they are on the same row or on the same column.
//
// Return the number of servers that communicate with any other server.
//
//
//
// Example 1:
//
//
//
// Input: grid = [[1,0],[0,1]]
// Output: 0
// Explanation: No servers can communicate with others.
// Example 2:
//
//
//
// Input: grid = [[1,0],[1,1]]
// Output: 3
// Explanation: All three servers can communicate with at least one other server.
// Example 3:
//
//
//
// Input: grid = [[1,1,0,0],[0,0,1,0],[0,0,1,0],[0,0,0,1]]
// Output: 4
// Explanation: The two servers in the first row can communicate with each other. The two servers in the third column can communicate with each other. The server at right bottom corner can't communicate with any other server.
//
//
// Constraints:
//
// m == grid.length
// n == grid[i].length
// 1 <= m <= 250
// 1 <= n <= 250
// grid[i][j] == 0 or 1
//
// Solution has
// Time complexity: O(m*n)
// Space complexity: O(m+n)
//
struct Solution;
impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut rows = vec![0; m];
        let mut cols = vec![0; n];
        let mut total_servers = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            }
        }

        for i in 0..m {
            if rows[i] == 1 {
                for j in 0..n {
                    if grid[i][j] == 1 && cols[j] > 1 {
                        total_servers += 1;
                    }
                }
            } else {
                total_servers += rows[i];
            }
        }
        total_servers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_servers(vec![vec![1, 0], vec![1, 1]]), 3);
    }
}
