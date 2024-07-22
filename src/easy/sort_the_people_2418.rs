// Solution has
// Time complexity: O(n log(n))
// Space complexity: O(n)
//
// You are given an array of strings names, and an array heights that consists of distinct positive integers. Both arrays are of length n.
//
// For each index i, names[i] and heights[i] denote the name and height of the ith person.
//
// Return names sorted in descending order by the people's heights.

struct Solution;
impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut heights_i: Vec<_> = heights.iter().enumerate().collect();
        heights_i.sort_unstable_by_key(|heights| heights.1);
        heights_i
            .iter()
            .rev()
            .map(|heights| names[heights.0].clone())
            .collect()
    }
}
