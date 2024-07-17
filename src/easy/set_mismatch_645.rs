// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
struct Solution;
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut frequency = vec![0; nums.len() + 1];
        let (mut num_twice, mut missingno) = (0, 0);
        for num in nums {
            frequency[num as usize] += 1;
        }
        for (i, count) in frequency.iter().skip(1).enumerate() {
            if *count == 0 {
                missingno = i + 1;
            } else if *count == 2 {
                num_twice = i + 1;
            }
        }
        vec![num_twice as i32, missingno as i32]
    }
}

// 1 1 3 4: target = 10, sum = 9, num_twice = 1
// 1 2 1 4: target = 10, sum = 8, num_twice = 1
// 4 2 3 4: target = 10, sum = 13, num_twice = 4
