// Solution has
// Time complexity: O(log7 n)
// Space complexity: O(1)
//
// Given an integer num, return a string of its base 7 representation.
struct Solution;
impl Solution {
    pub fn convert_to_base7(mut num: i32) -> String {
        let mut base = 1;
        let mut output = 0;
        while num != 0 {
            output += base * (num % 7);
            num /= 7;
            base *= 10;
        }
        output.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_to_base7() {
        assert_eq!(Solution::convert_to_base7(100), "202");
    }

    #[test]
    fn test_convert_to_base7_2() {
        assert_eq!(Solution::convert_to_base7(-7), "-10");
    }
}
