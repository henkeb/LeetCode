// You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.
//
// Return true if you can reach the last index, or false otherwise.
//
//
//
// Example 1:
//
// Input: nums = [2,3,1,1,4]
// Output: true
// Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
// Example 2:
//
// Input: nums = [3,2,1,0,4]
// Output: false
// Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.
//
//
// Constraints:
//
// 1 <= nums.length <= 104
// 0 <= nums[i] <= 105
struct Solution;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // false --> not jumped on, true --> jumped on --> return early (we cant get to end from
        // here)
        let end_point = (nums.len() - 1) as i32;
        let mut visited = vec![false; nums.len()];
        visited[0] = true;

        let mut queue: Vec<i32> = Vec::new();
        queue.push(nums[0]);

        while let Some(jump) = queue.pop() {
            if jump >= end_point || jump + nums[jump as usize] >= end_point {
                visited[end_point as usize] = true;
                break;
            }
            if visited[jump as usize] {
                continue;
            }
            visited[jump as usize] = true;

            queue.push(jump - 1);
            queue.push(jump + nums[jump as usize]);
        }
        visited[end_point as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::can_jump([2, 5, 0, 0].to_vec()));
    }
}
