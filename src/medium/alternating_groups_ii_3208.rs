// There is a circle of red and blue tiles. You are given an array of integers colors and an integer k. The color of tile i is represented by colors[i]:
//
// colors[i] == 0 means that tile i is red.
// colors[i] == 1 means that tile i is blue.
// An alternating group is every k contiguous tiles in the circle with alternating colors (each tile in the group except the first and last one has a different color from its left and right tiles).
//
// Return the number of alternating groups.
//
// Note that since colors represents a circle, the first and the last tiles are considered to be next to each other.
//
//
//
// Example 1:
//
// Input: colors = [0,1,0,1,0], k = 3
//
// Output: 3
//
// Example 2:
//
// Input: colors = [0,1,0,0,1,0,1], k = 6
//
// Output: 2
//
// Example 3:
//
// Input: colors = [1,1,0,1], k = 4
//
// Output: 0
//
// Constraints:
//
// 3 <= colors.length <= 105
// 0 <= colors[i] <= 1
// 3 <= k <= colors.length
struct Solution;
impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = colors.len();
        let mut prev = colors[0];
        let mut alternating = 1;
        let mut output = 0;
        for i in 1..(n + k - 1) {
            if colors[i % n] == prev {
                alternating = 1;
                prev = colors[i % n];
                continue;
            }

            alternating += 1;

            if alternating >= k {
                output += 1;
            }

            prev = colors[i % n];
        }
        output
    }
}
