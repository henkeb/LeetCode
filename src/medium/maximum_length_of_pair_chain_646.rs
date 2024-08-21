// Solution has for greedy
// Time complexity: O(nlogn)
// Space complexity: O(1)
//
// Solution has for dp
// Time complexity: O(n²)
// Space complexity: O(n)
//
// You are given an array of n pairs pairs where pairs[i] = [lefti, righti] and lefti < righti.
//
// A pair p2 = [c, d] follows a pair p1 = [a, b] if b < c. A chain of pairs can be formed in this fashion.
//
// Return the length longest chain which can be formed.
//
// You do not need to use up all the given intervals. You can select pairs in any order.

struct Solution;
impl Solution {
    pub fn find_longest_chain_greedy(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_by_key(|v| v[1]);

        let mut cur = i32::MIN;
        let mut ans = 0;
        for pair in &pairs {
            if cur < pair[0] {
                cur = pair[1];
                ans += 1;
            }
        }
        ans
    }

    pub fn find_longest_chain_dp(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_by_key(|v| v[1]);
        let mut dp = vec![1; pairs.len()];

        for i in 1..pairs.len() {
            for j in 0..i {
                if pairs[i][0] > pairs[j][1] {
                    dp[i] += 1;
                }
            }
        }
        *dp.iter().max().unwrap_or(&0)
    }
}
