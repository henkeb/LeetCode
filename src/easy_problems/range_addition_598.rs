// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
struct Solution;
// impl Solution {
//     pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
//         if ops.is_empty() {
//             return m * n;
//         }
//         let mut mat = vec![0; (m * n) as usize];
//         ops.iter().for_each(|op| {
//             for x in 0..op[0] {
//                 for y in 0..op[1] {
//                     Self::increment(&x, &y, &m, mat.as_mut_slice());
//                 }
//             }
//         });
//         let max = mat.iter().max().unwrap_or(&0);
//         mat.iter().filter(|elem| *elem == max).count() as i32
//     }
//     fn increment(x: &i32, y: &i32, width: &i32, mat: &mut [i32]) {
//         mat[(*x * *width + *y) as usize] += 1;
//     }
// }
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        ops.iter().map(|op| op[0]).min().unwrap_or(m)
            * ops.iter().map(|op| op[1]).min().unwrap_or(n)
    }
}
