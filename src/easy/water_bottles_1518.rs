// Solution has
// Time complexity: O(log_{num_exchange} n)
// Space complexity: O(1)
//
// There are numBottles water bottles that are initially full of water. You can exchange numExchange empty water bottles from the market with one full water bottle.
//
// The operation of drinking a full water bottle turns it into an empty bottle.
//
// Given the two integers numBottles and numExchange, return the maximum number of water bottles you can drink.
struct Solution;
impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut total_drinks = 0;
        let mut empty_bottles = 0;
        while num_bottles > 0 {
            total_drinks += num_bottles;
            empty_bottles += num_bottles;

            num_bottles = empty_bottles / num_exchange;
            empty_bottles %= num_exchange;
        }
        total_drinks
    }
}
