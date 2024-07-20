// Solution has
// Time complexity: O(m*n)
// Space complexity: O(m*n)
//
// You are given two arrays rowSum and colSum of non-negative integers where rowSum[i] is the sum of the elements in the ith row and colSum[j] is the sum of the elements of the jth column of a 2D matrix. In other words, you do not know the elements of the matrix, but you do know the sums of each row and column.
//
// Find any matrix of non-negative integers of size rowSum.length x colSum.length that satisfies the rowSum and colSum requirements.
//
// Return a 2D array representing any matrix that fulfills the requirements. It's guaranteed that at least one matrix that fulfills the requirements exists.
struct Solution;
impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let m = row_sum.len();
        let n = col_sum.len();
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; n]; m];
        let mut row = 0;
        let mut col = 0;

        while row < m && col < n {
            matrix[row][col] = std::cmp::min(row_sum[row], col_sum[col]);
            row_sum[row] -= matrix[row][col];
            col_sum[col] -= matrix[row][col];
            match (row_sum[row] == 0, col_sum[col] == 0) {
                (true, true) => {
                    row += 1;
                    col += 1;
                }
                (true, false) => row += 1,
                (false, true) => col += 1,
                _ => (),
            }
        }
        matrix
    }
}
