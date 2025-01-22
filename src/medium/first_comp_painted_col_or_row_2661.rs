use std::collections::HashMap;

// You are given a 0-indexed integer array arr, and an m x n integer matrix mat. arr and mat both contain all the integers in the range [1, m * n].
//
// Go through each index i in arr starting from index 0 and paint the cell in mat containing the integer arr[i].
//
// Return the smallest index i at which either a row or a column will be completely painted in mat.
//
//
//
// Example 1:
//
// image explanation for example 1
// Input: arr = [1,3,4,2], mat = [[1,4],[2,3]]
// Output: 2
// Explanation: The moves are shown in order, and both the first row and second column of the matrix become fully painted at arr[2].
// Example 2:
//
// image explanation for example 2
// Input: arr = [2,8,7,4,1,3,5,6,9], mat = [[3,2,5],[1,4,6],[8,7,9]]
// Output: 3
// Explanation: The second column becomes fully painted at arr[3].
//
//
// Constraints:
//
// m == mat.length
// n = mat[i].length
// arr.length == m * n
// 1 <= m, n <= 105
// 1 <= m * n <= 105
// 1 <= arr[i], mat[r][c] <= m * n
// All the integers of arr are unique.
// All the integers of mat are unique.
//
// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
struct Solution;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let mut map: HashMap<i32, (usize, usize)> = HashMap::new();
        let row_len = mat.len();
        let col_len = mat[0].len();
        let mut row_freq = vec![0; row_len];
        let mut col_freq = vec![0; col_len];
        for (i, row) in mat.iter().enumerate() {
            for (j, col_val) in row.iter().enumerate() {
                map.insert(*col_val, (i, j));
            }
        }

        for (i, val) in arr.iter().enumerate() {
            if let Some((row_val, col_val)) = map.get(val) {
                row_freq[*row_val] += 1;
                if row_freq[*row_val] == col_len {
                    return i as i32;
                }
                col_freq[*col_val] += 1;
                if col_freq[*col_val] == row_len {
                    return i as i32;
                }
            }
        }
        panic!("Can never happen");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::first_complete_index(vec![1, 3, 4, 2], vec![vec![1, 4], vec![2, 3]]),
            2
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::first_complete_index(
                vec![2, 8, 7, 4, 1, 3, 5, 6, 9],
                vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]]
            ),
            3
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::first_complete_index(
                vec![8, 2, 4, 9, 3, 5, 7, 10, 1, 6],
                vec![vec![8, 2, 9, 10, 4], vec![1, 7, 6, 3, 5]]
            ),
            5
        );
    }
}
