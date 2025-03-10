use std::f64;

// Solution has
// Time complexity: O(?)
// Space complexity: O(?)
//
// You are given a positive integer array skill of even length n where skill[i] denotes the skill of the ith player. Divide the players into n / 2 teams of size 2 such that the total skill of each team is equal.
//
// The chemistry of a team is equal to the product of the skills of the players on that team.
//
// Return the sum of the chemistry of all the teams, or return -1 if there is no way to divide the players into teams such that the total skill of each team is equal.
//
//
//
// Example 1:
//
// Input: skill = [3,2,5,1,3,4]
// Output: 22
// Explanation:
// Divide the players into the following teams: (1, 5), (2, 4), (3, 3), where each team has a total skill of 6.
// The sum of the chemistry of all the teams is: 1 * 5 + 2 * 4 + 3 * 3 = 5 + 8 + 9 = 22.
// Example 2:
//
// Input: skill = [3,4]
// Output: 12
// Explanation:
// The two players form a team with a total skill of 7.
// The chemistry of the team is 3 * 4 = 12.
// Example 3:
//
// Input: skill = [1,1,2,3]
// Output: -1
// Explanation:
// There is no way to divide the players into teams such that the total skill of each team is equal.
//
//
// Constraints:
//
// 2 <= skill.length <= 10^5
// skill.length is even.
// 1 <= skill[i] <= 1000
struct Solution;
// impl Solution {
//     pub fn divide_players(skill: Vec<i32>) -> i64 {
//         let n = skill.len();
//         let mut skill = skill.iter().map(|val| *val as i64).collect::<Vec<i64>>();
//         skill.sort_unstable();
//
//         let sum = skill[0] + skill[n - 1];
//         let mut total_chemistry = skill[0] * skill[n - 1];
//
//         for l in 1..n / 2 {
//             let r = n - l - 1;
//             if skill[l] + skill[r] == sum {
//                 total_chemistry += skill[l] * skill[r];
//             } else {
//                 return -1;
//             }
//         }
//         total_chemistry
//     }
// }
impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut frequency = vec![0; 1001];
        let mut cumulative_average: f64 = skill[0] as f64;
        let mut n: f64 = 1.;
        //New average = old average * (n-1)/n + new value /n
        //new_average = (old_average * (n-1) + new_value) / n
        for val in skill.iter().skip(1) {
            cumulative_average = (*val as f64 * cumulative_average * n) / (n + 1.);
            frequency[*val as usize] += 1;
            n += 1.0;
        }

        if (cumulative_average * 2.).fract() > 1e-10 {
            return -1;
        }

        let pair_product: i64 = (cumulative_average * 2.0) as i64;

        for &val in frequency.iter() {
            if val == 0 {
                continue;
            }
            if pair_product % val != 0 {
                return -1;
            }
            if frequency[(pair_product / val) as usize] != frequency[val as usize] {
                return -1;
            }
        }
        pair_product * skill.len() as i64 / 2
    }
}
