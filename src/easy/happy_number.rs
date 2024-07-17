// Solution has
// Time complexity: O(?)
// Space complexity: O(1)
//
// Write an algorithm to determine if a number n is happy.
//
// A happy number is a number defined by the following process:
//
//     Starting with any positive integer, replace the number by the sum of the squares of its digits.
//     Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
//     Those numbers for which this process ends in 1 are happy.
//
// Return true if n is a happy number, and false if not.
//
struct Solution;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut happy: u32 = n.try_into().unwrap();

        while happy != 1 {
            happy = happy
                .to_string()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|num| num.pow(2))
                .sum();
            if happy == 4 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert!(Solution::is_happy(19));
    }

    #[test]
    fn test_is_not_happy() {
        assert!(!Solution::is_happy(2));
    }
}
