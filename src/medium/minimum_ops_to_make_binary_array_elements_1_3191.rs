// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// You are given a
// binary array
// nums.
//
// You can do the following operation on the array any number of times (possibly zero):
//
//     Choose any 3 consecutive elements from the array and flip all of them.
//
// Flipping an element means changing its value from 0 to 1, and from 1 to 0.
//
// Return the minimum number of operations required to make all elements in nums equal to 1. If it is impossible, return -1.
struct Solution;
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut ops = 0;
        let len = nums.len();
        if len < 3 {
            return ops;
        }

        for i in 0..(len - 2) {
            if nums[i] == 0 {
                nums[i + 1] = 1 - nums[i + 1];
                nums[i + 2] = 1 - nums[i + 2];
                ops += 1;
            }
        }
        if nums[len - 1] == 0 || nums[len - 2] == 0 {
            return -1;
        }
        ops
    }

    fn flip(bit: &i32) -> i32 {
        if *bit == 0 {
            1
        } else {
            0
        }
    }
}
