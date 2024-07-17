// Solution has
// Time complexity: O(n²)
// Space complexity: O(n²)
//
//Given an integer numRows, return the first numRows of Pascal's triangle.
// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
//

struct Solution;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        match num_rows {
            0 => return vec![vec![]],
            1 => return vec![vec![1]],
            2 => return vec![vec![1], vec![1, 1]],
            _ => (),
        }

        let mut triangle: Vec<Vec<i32>> = vec![vec![1], vec![1, 1]];

        for row in 2..num_rows {
            let mut new_row: Vec<i32> = vec![1];
            for col in 1..row {
                new_row.push(
                    triangle[row as usize - 1][col as usize - 1]
                        + triangle[row as usize - 1][col as usize],
                );
            }
            new_row.push(1);
            triangle.push(new_row);
        }
        triangle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::generate(0), vec![vec![]]);
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::generate(6),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
                vec![1, 5, 10, 10, 5, 1]
            ]
        );
    }
}
