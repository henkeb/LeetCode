// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Alice has n candies, where the ith candy is of type candyType[i]. Alice noticed that she started to gain weight, so she visited a doctor.
//
// The doctor advised Alice to only eat n / 2 of the candies she has (n is always even). Alice likes her candies very much, and she wants to eat the maximum number of different types of candies while still following the doctor's advice.
//
// Given the integer array candyType of length n, return the maximum number of different types of candies she can eat if she only eats n / 2 of them.
struct Solution;
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let max_candies = candy_type.len() >> 1;
        let different_candies = HashSet::<&i32>::from_iter(candy_type.iter()).len() as i32;
        if different_candies < max_candies as i32 {
            return different_candies;
        }
        max_candies as i32
    }
}
