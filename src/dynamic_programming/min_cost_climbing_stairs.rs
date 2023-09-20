// Solution has
// Time complexity: O(n)
// Space complexity: O(n)

//You are given an integer array cost where cost[i] is the cost of ith step on a staircase. Once you pay the cost, you can either climb one or two steps.
//
// You can either start from the step with index 0, or the step with index 1.
//
// Return the minimum cost to reach the top of the floor.
struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![0; cost.len() + 2];
        for i in (0..cost.len()).rev() {
            dp[i] = cost[i] + std::cmp::min(dp[i + 1], dp[i + 2]);
        }
        std::cmp::min(dp[0], dp[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::min_cost_climbing_stairs([1, 100, 1, 1, 1, 100, 1, 1, 100, 1].to_vec()),
            6
        );
        assert_eq!(
            Solution::min_cost_climbing_stairs([10, 15, 20].to_vec()),
            15
        );
        assert_eq!(
            Solution::min_cost_climbing_stairs([15, 10, 20].to_vec()),
            10
        );
    }
}
