// Given an array of points on the X-Y plane points where points[i] = [xi, yi], return the area of the largest triangle that can be formed by any three different points. Answers within 10-5 of the actual answer will be accepted.
//
//
//
// Example 1:
//
//
// Input: points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
// Output: 2.00000
// Explanation: The five points are shown in the above figure. The red triangle is the largest.
// Example 2:
//
// Input: points = [[1,0],[0,0],[0,1]]
// Output: 0.50000
//
//
// Constraints:
//
// 3 <= points.length <= 50
// -50 <= xi, yi <= 50
// All the given points are unique.
struct Solution;
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut area = 0.0;
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                for k in (j + 1)..points.len() {
                    area = f64::max(
                        area,
                        Solution::calculate_area(&points[i], &points[j], &points[k]),
                    );
                }
            }
        }
        area
    }

    fn calculate_area(p1: &[i32], p2: &[i32], p3: &[i32]) -> f64 {
        // Area of triangle ABC = (1/2) | [x1 (y2 - y3) + x2 (y3 - y1) + x3 (y1 - y2)] |
        (p1[0] * (p2[1] - p3[1]) + p2[0] * (p3[1] - p1[1]) + p3[0] * (p1[1] - p2[1])).abs() as f64
            / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::largest_triangle_area(vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![0, 2],
                vec![2, 0]
            ]),
            2.0
        )
    }
}
