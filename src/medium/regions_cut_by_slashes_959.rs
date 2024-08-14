// Solution has
// Time complexity: O(n²)
// Space complexity: O((n*3)²)
//
// An n x n grid is composed of 1 x 1 squares where each 1 x 1 square consists of a '/', '\', or blank space ' '. These characters divide the square into contiguous regions.
//
// Given the grid grid represented as a string array, return the number of regions.
//
// Note that backslash characters are escaped, so a '\' is represented as '\\'.
struct Solution;
impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();
        let mut graph: Vec<Vec<u8>> = vec![vec![0; n * 3]; n * 3];
        for (i, row) in grid.iter().enumerate() {
            for (j, c) in row.chars().enumerate() {
                if c == '/' {
                    for k in 0..3 {
                        graph[3 * i + 2 - k][3 * j + k] = 1;
                    }
                }
                if c == '\\' {
                    for k in 0..3 {
                        graph[3 * i + k][3 * j + k] = 1;
                    }
                }
            }
        }
        Self::count_regions(&mut graph)
    }

    fn count_regions(graph: &mut [Vec<u8>]) -> i32 {
        let mut regions = 0;
        for y in 0..graph.len() {
            for x in 0..graph.len() {
                if graph[y][x] == 1 {
                    continue;
                }
                Self::dfs(graph, x, y);
                regions += 1;
            }
        }
        regions
    }

    fn dfs(graph: &mut [Vec<u8>], x: usize, y: usize) {
        let mut stack: Vec<(usize, usize)> = vec![(x, y)];
        let n = graph.len();
        while let Some((x, y)) = stack.pop() {
            if graph[y][x] == 1 {
                continue;
            }
            graph[y][x] = 1;

            if x > 0 {
                stack.push((x - 1, y));
            }
            if x + 1 < n {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y + 1 < n {
                stack.push((x, y + 1));
            }
        }
    }
}
