// Solution has
// Time complexity: O(n²)
// Space complexity: O(1)
//
// There are n soldiers standing in a line. Each soldier is assigned a unique rating value.
//
// You have to form a team of 3 soldiers amongst them under the following rules:
//
//     Choose 3 soldiers with index (i, j, k) with rating (rating[i], rating[j], rating[k]).
//     A team is valid if: (rating[i] < rating[j] < rating[k]) or (rating[i] > rating[j] > rating[k]) where (0 <= i < j < k < n).
//
// Return the number of teams you can form given the conditions. (soldiers can be part of multiple teams).
struct Solution;
impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let len = rating.len();
        let mut teams = 0;
        for i in 1..(len - 1) {
            let (mut left_gt, mut left_st) = (0, 0);
            let (mut right_gt, mut right_st) = (0, 0);
            for j in 0..i {
                if rating[j] < rating[i] {
                    left_st += 1;
                } else {
                    left_gt += 1;
                }
            }
            for j in (i + 1)..len {
                if rating[j] < rating[i] {
                    right_st += 1;
                } else {
                    right_gt += 1;
                }
            }
            teams += left_st * right_gt + left_gt * right_st;
        }
        teams
    }
}
