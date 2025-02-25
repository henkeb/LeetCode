// Given an array of integers arr, return the number of subarrays with an odd sum.
//
// Since the answer can be very large, return it modulo 109 + 7.
//
//
//
// Example 1:
//
// Input: arr = [1,3,5]
// Output: 4
// Explanation: All subarrays are [[1],[1,3],[1,3,5],[3],[3,5],[5]]
// All sub-arrays sum are [1,4,9,3,8,5].
// Odd sums are [1,9,3,5] so the answer is 4.
// Example 2:
//
// Input: arr = [2,4,6]
// Output: 0
// Explanation: All subarrays are [[2],[2,4],[2,4,6],[4],[4,6],[6]]
// All sub-arrays sum are [2,6,12,4,10,6].
// All sub-arrays have even sum and the answer is 0.
// Example 3:
//
// Input: arr = [1,2,3,4,5,6,7]
// Output: 16
//
//
// Constraints:
//
// 1 <= arr.length <= 105
// 1 <= arr[i] <= 100
struct Solution;
impl Solution {
    pub fn num_of_subarrays(mut arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1e9 as i32 + 7;
        arr.iter_mut().for_each(|val| *val &= 1);
        let n = arr.len();
        let mut dp_odd: Vec<i32> = vec![0; n];
        let mut dp_even: Vec<i32> = vec![0; n];

        if arr[n - 1] == 0 {
            dp_even[n - 1] = 1;
        } else {
            dp_odd[n - 1] = 1;
        }

        if n > 1 {
            for i in (0..=n - 2).rev() {
                if arr[i] == 1 {
                    dp_odd[i] = (1 + dp_even[i + 1]) % MOD;
                    dp_even[i] = dp_odd[i + 1];
                } else {
                    dp_even[i] = (1 + dp_even[i + 1]) % MOD;
                    dp_odd[i] = dp_odd[i + 1];
                }
            }
        }

        dp_odd.iter().fold(0, |acc, val| (acc + val) % MOD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]), 16)
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::num_of_subarrays(vec![2, 4, 6]), 0)
    }
}
