// You are given two positive integers low and high.
//
// An integer x consisting of 2 * n digits is symmetric if the sum of the first n digits of x is equal to the sum of the last n digits of x. Numbers with an odd number of digits are never symmetric.
//
// Return the number of symmetric integers in the range [low, high].
//
//
//
// Example 1:
//
// Input: low = 1, high = 100
// Output: 9
// Explanation: There are 9 symmetric integers between 1 and 100: 11, 22, 33, 44, 55, 66, 77, 88, and 99.
// Example 2:
//
// Input: low = 1200, high = 1230
// Output: 4
// Explanation: There are 4 symmetric integers between 1200 and 1230: 1203, 1212, 1221, and 1230.
//
//
// Constraints:
//
// 1 <= low <= high <= 104
struct Solution;
impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut output = 0;
        for mut i in low..=high {
            let digits = f32::log10(i as f32) as i32 + 1;
            if digits % 2 == 0 {
                let (mut left_sum, mut right_sum) = (0, 0);
                let half_digits = digits / 2;
                let mut pos = digits;
                while i != 0 {
                    let digit = i % 10;
                    if pos > half_digits {
                        right_sum += digit;
                    } else {
                        left_sum += digit;
                    }
                    pos -= 1;
                    i /= 10;
                }
                if left_sum == right_sum {
                    output += 1;
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_symmetric_integers(1, 100), 9);
    }
}
