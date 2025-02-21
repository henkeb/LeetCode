use std::collections::HashSet;

// Given an array of strings nums containing n unique binary strings each of length n, return a binary string of length n that does not appear in nums. If there are multiple answers, you may return any of them.
//
//
//
// Example 1:
//
// Input: nums = ["01","10"]
// Output: "11"
// Explanation: "11" does not appear in nums. "00" would also be correct.
// Example 2:
//
// Input: nums = ["00","01"]
// Output: "11"
// Explanation: "11" does not appear in nums. "10" would also be correct.
// Example 3:
//
// Input: nums = ["111","011","001"]
// Output: "101"
// Explanation: "101" does not appear in nums. "000", "010", "100", and "110" would also be correct.
//
//
// Constraints:
//
// n == nums.length
// 1 <= n <= 16
// nums[i].length == n
// nums[i] is either '0' or '1'.
// All the strings of nums are unique.
struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let nums_set: HashSet<&String> = HashSet::from_iter(nums.iter());
        let n = nums[0].len();
        let mut output = String::new();
        Solution::backtrack(&nums_set, &mut output, &n);
        output
    }

    fn backtrack(nums_set: &HashSet<&String>, number_string: &mut String, n: &usize) -> bool {
        if number_string.len() == *n {
            match nums_set.get(number_string) {
                Some(_) => return false,
                None => return true,
            }
        }

        for i in 0..=1 {
            number_string.push(char::from_digit(i, 10).unwrap());
            if Solution::backtrack(&nums_set, number_string, n) {
                return true;
            }
            number_string.pop();
        }
        false
    }
}

struct Solution2;
impl Solution2 {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut ans = String::new();
        for i in 0..nums.len() {
            if let Some(num) = nums[i].chars().nth(i) {
                if num == '1' {
                    ans.push('0');
                } else {
                    ans.push('1');
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        Solution2::find_different_binary_string(vec![
            "111".to_string(),
            "011".to_string(),
            "001".to_string(),
        ]);
    }
}
