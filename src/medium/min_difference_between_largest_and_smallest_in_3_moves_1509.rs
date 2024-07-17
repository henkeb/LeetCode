// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// You are given an integer array nums.
//
// In one move, you can choose one element of nums and change it to any value.
//
// Return the minimum difference between the largest and smallest value of nums after performing at most three moves.
struct Solution;
impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        if nums.len() <= 4 {
            return 0;
        }
        let mut smallest = [i32::MAX; 4];
        let mut largest = [i32::MIN; 4];

        for val in nums {
            if val <= smallest[0] {
                smallest = [val, smallest[0], smallest[1], smallest[2]];
            } else if val <= smallest[1] {
                smallest = [smallest[0], val, smallest[1], smallest[2]];
            } else if val <= smallest[2] {
                smallest = [smallest[0], smallest[1], val, smallest[2]];
            } else if val < smallest[3] {
                smallest = [smallest[0], smallest[1], smallest[2], val];
            }

            if val >= largest[3] {
                largest = [largest[1], largest[2], largest[3], val];
            } else if val >= largest[2] {
                largest = [largest[1], largest[2], val, largest[3]];
            } else if val >= largest[1] {
                largest = [largest[1], val, largest[2], largest[3]];
            } else if val > largest[0] {
                largest = [val, largest[1], largest[2], largest[3]];
            }
        }
        smallest
            .iter()
            .zip(largest.iter())
            .map(|(s, l)| l - s)
            .min()
            .unwrap()
    }
}
