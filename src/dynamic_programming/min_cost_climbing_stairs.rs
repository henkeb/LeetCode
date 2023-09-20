// Solution has
// Time complexity: O(n)
// Space complexity: O(1)

//You are given an integer array cost where cost[i] is the cost of ith step on a staircase. Once you pay the cost, you can either climb one or two steps.
//
// You can either start from the step with index 0, or the step with index 1.
//
// Return the minimum cost to reach the top of the floor.
struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut take_one = 0;
        let mut take_two = 0;
        let mut temp = 0;
        (2..=cost.len()).for_each(|i| {
            temp = take_one;
            take_one = std::cmp::min(take_one + cost[i - 1], take_two + cost[i - 2]);
            take_two = temp;
        });
        take_one
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
