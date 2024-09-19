// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given a list of non-negative integers nums, arrange them such that they form the largest number and return it.
//
// Since the result may be very large, so you need to return a string instead of an integer.
//
//
//
// Example 1:
//
// Input: nums = [10,2]
// Output: "210"
//
// Example 2:
//
// Input: nums = [3,30,34,5,9]
// Output: "9534330"
//
//
//
// Constraints:
//
//     1 <= nums.length <= 100
//     0 <= nums[i] <= 109
//
struct Solution;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        if nums.iter().all(|&x| x == 0) {
            return "0".to_string();
        }
        let mut a = nums.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        a.sort_unstable_by(Self::compare);
        a.concat()
    }

    fn compare(a: &String, b: &String) -> std::cmp::Ordering {
        let s1 = a.chars().chain(b.chars());
        let s2 = b.chars().chain(a.chars());

        s2.cmp(s1)
    }
}
