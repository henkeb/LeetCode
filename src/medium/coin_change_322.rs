// Solution has
// Time complexity: O(n*m) m is amount
// Space complexity: O(n)
//
// You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
//
// Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.
//
// You may assume that you have an infinite number of each kind of coin.
struct Solution;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![i32::MAX; amount + 1];
        dp[0] = 0; // base case

        for a in 1..=amount {
            for coin in &coins {
                if *coin <= a as i32 {
                    dp[a] = i32::min(dp[a], dp[a - *coin as usize].saturating_add(1));
                }
            }
        }
        if dp[amount] == i32::MAX {
            -1
        } else {
            dp[amount]
        }
    }
}

// Example 1:
//
// Input: coins = [1,2,5], amount = 11
// Output: 3
// Explanation: 11 = 5 + 5 + 1
//
// Example 2:
//
// Input: coins = [2], amount = 3
// Output: -1
//
// Example 3:
//
// Input: coins = [1], amount = 0
// Output: 0
