// Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
//
// You must do it in place.
//
//
//
// Example 1:
//
//
// Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
// Output: [[1,0,1],[0,0,0],[1,0,1]]
// Example 2:
//
//
// Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
// Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
//
//
// Constraints:
//
// m == matrix.length
// n == matrix[0].length
// 1 <= m, n <= 200
// -231 <= matrix[i][j] <= 231 - 1
//
//
// Follow up:
//
// A straightforward solution using O(mn) space is probably a bad idea.
// A simple improvement uses O(m + n) space, but still not the best solution.
// Could you devise a constant space solution?
struct Solution;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        use std::collections::HashSet;

        let m = matrix.len();
        let n = matrix[0].len();
        let mut z_set: HashSet<(usize, usize)> = HashSet::new();
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    z_set.insert((i, j));
                }
            }
        }

        for (i, j) in z_set.into_iter() {
            for row in 0..n {
                matrix[i][row] = 0;
            }
            for col in 0..m {
                matrix[col][j] = 0;
            }
        }
    }
}
