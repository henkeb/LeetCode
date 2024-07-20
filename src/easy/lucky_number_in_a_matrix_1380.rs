// Solution has
// Time complexity: O(?)
// Space complexity: O(?)
//
//Given an m x n matrix of distinct numbers, return all lucky numbers in the matrix in any order.
//
// A lucky number is an element of the matrix such that it is the minimum element in its row and maximum in its column.
struct Solution;
impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut lucky_numbers: Vec<i32> = Vec::new();
        let mut column_max: Vec<i32> = Vec::with_capacity(n);
        for j in 0..n {
            let mut max = i32::MIN;
            (0..m).for_each(|i| {
                if matrix[i][j] > max {
                    max = matrix[i][j];
                }
            });
            column_max.push(max);
        }

        (0..m).for_each(|i| {
            let mut min = (0usize, i32::MAX);
            for j in 0..n {
                if matrix[i][j] < min.1 {
                    min.1 = matrix[i][j];
                    min.0 = j;
                }
            }
            println!("min: {min:?}");
            if min.1 == column_max[min.0] {
                lucky_numbers.push(min.1);
            }
        });
        lucky_numbers
    }
}
