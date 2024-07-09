// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given an integer array arr, return true if there are three consecutive odd numbers in the array. Otherwise, return false.
struct Solution;
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        arr.windows(3)
            .any(|window| window[0] % 2 == 1 && window[1] % 2 == 1 && window[2] % 2 == 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(!Solution::three_consecutive_odds(vec![2, 6, 4, 1]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::three_consecutive_odds(vec![1, 2, 34, 3, 4, 5]));
    }

    #[test]
    fn test3() {
        assert!(!Solution::three_consecutive_odds(vec![
            1, 2, 34, 3, 4, 5, 7
        ]));
    }

    #[test]
    fn test4() {
        assert!(Solution::three_consecutive_odds(vec![1, 3, 4, 3, 5, 7]))
    }
}
