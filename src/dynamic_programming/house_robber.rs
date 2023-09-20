// Solution has
// Time complexity: O(n)
// Space complexity: O(n)

struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut dp: Vec<i32> = vec![0; nums.len()];

        // Base cases
        dp[0] = nums[0];
        dp[1] = std::cmp::max(nums[0], nums[1]);

        for i in 2..nums.len() {
            dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + nums[i]);
        }

        dp[nums.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::rob([1, 2, 3, 1].to_vec()), 4);
        assert_eq!(Solution::rob([2, 7, 9, 3, 1].to_vec()), 12);
    }
}
