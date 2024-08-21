// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// You are given m arrays, where each array is sorted in ascending order.
//
// You can pick up two integers from two different arrays (each array picks one) and calculate the distance. We define the distance between two integers a and b to be their absolute difference |a - b|.
//
// Return the maximum distance.
struct Solution;
impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut min_val = arrays[0][0];
        let mut max_val = arrays[0][arrays[0].len() - 1];
        let mut output = 0;
        arrays
            .iter()
            .skip(1)
            .map(|v| (v[0], v[v.len() - 1]))
            .for_each(|(curr_min, curr_max)| {
                output = output.max((curr_max - min_val).abs());
                output = output.max((max_val - curr_min).abs());
                min_val = min_val.min(curr_min);
                max_val = max_val.max(curr_max);
            });
        output
    }
}
