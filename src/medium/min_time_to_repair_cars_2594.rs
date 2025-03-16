// You are given an integer array ranks representing the ranks of some mechanics. ranks_i is the rank of the ith mechanic. A mechanic with a rank r can repair n cars in r * n2 minutes.
//
// You are also given an integer cars representing the total number of cars waiting in the garage to be repaired.
//
// Return the minimum time taken to repair all the cars.
//
// Note: All the mechanics can repair the cars simultaneously.
//
//
//
// Example 1:
//
// Input: ranks = [4,2,3,1], cars = 10
// Output: 16
// Explanation:
// - The first mechanic will repair two cars. The time required is 4 * 2 * 2 = 16 minutes.
// - The second mechanic will repair two cars. The time required is 2 * 2 * 2 = 8 minutes.
// - The third mechanic will repair two cars. The time required is 3 * 2 * 2 = 12 minutes.
// - The fourth mechanic will repair four cars. The time required is 1 * 4 * 4 = 16 minutes.
// It can be proved that the cars cannot be repaired in less than 16 minutes.
// Example 2:
//
// Input: ranks = [5,1,8], cars = 6
// Output: 16
// Explanation:
// - The first mechanic will repair one car. The time required is 5 * 1 * 1 = 5 minutes.
// - The second mechanic will repair four cars. The time required is 1 * 4 * 4 = 16 minutes.
// - The third mechanic will repair one car. The time required is 8 * 1 * 1 = 8 minutes.
// It can be proved that the cars cannot be repaired in less than 16 minutes.
//
//
// Constraints:
//
// 1 <= ranks.length <= 105
// 1 <= ranks[i] <= 100
// 1 <= cars <= 106
struct Solution;
impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let cars = cars as usize;
        let max_rank = *ranks.iter().max().unwrap() as usize;
        let mut h = max_rank * cars * cars;
        let mut l = 1;
        let mut ans = i64::max_value();
        while l <= h {
            let mid = l + (h - l) / 2;
            if Solution::can_repair_in_time(&ranks, mid, cars as usize) {
                ans = std::cmp::min(ans, mid as i64);
                h = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        ans
    }

    fn can_repair_in_time(ranks: &[i32], time: usize, cars: usize) -> bool {
        let mut tot_cars = 0;
        for &rank in ranks.iter() {
            tot_cars += (time / rank as usize).isqrt() as usize;
            if tot_cars >= cars {
                return true;
            }
        }
        false
    }
}
