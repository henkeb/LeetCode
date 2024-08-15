// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// At a lemonade stand, each lemonade costs $5. Customers are standing in a queue to buy from you and order one at a time (in the order specified by bills). Each customer will only buy one lemonade and pay with either a $5, $10, or $20 bill. You must provide the correct change to each customer so that the net transaction is that the customer pays $5.
//
// Note that you do not have any change in hand at first.
//
// Given an integer array bills where bills[i] is the bill the ith customer pays, return true if you can provide every customer with the correct change, or false otherwise.
struct Solution;
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        if bills[0] != 5 {
            return false;
        }
        let (mut fives, mut tens) = (1, 0);
        for &bill in bills.iter().skip(1) {
            match bill {
                5 => fives += 1,
                10 => {
                    if fives > 0 {
                        fives -= 1;
                        tens += 1;
                    } else {
                        return false;
                    }
                }
                20 => {
                    if tens > 0 && fives > 0 {
                        tens -= 1;
                        fives -= 1;
                    } else if fives >= 3 {
                        fives -= 3;
                    } else {
                        return false;
                    }
                }
                _ => (),
            }
        }
        true
    }
}
