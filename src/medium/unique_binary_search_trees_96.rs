// Solution has
// Time complexity: O(n²)
// Space complexity: O(n)
//
// Given an integer n, return the number of structurally unique BST's (binary search trees) which has exactly n nodes of unique values from 1 to n.
struct Solution;
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut catalan = [0; 20];
        catalan[0] = 1;
        catalan[1] = 1;
        for i in 2..=n as usize {
            for j in 0..i {
                catalan[i] += catalan[j] * catalan[i - j - 1];
            }
        }
        catalan[n as usize]
    }
}
