// Given two positive integers num1 and num2, find the positive integer x such that:
//
// x has the same number of set bits as num2, and
// The value x XOR num1 is minimal.
// Note that XOR is the bitwise XOR operation.
//
// Return the integer x. The test cases are generated such that x is uniquely determined.
//
// The number of set bits of an integer is the number of 1's in its binary representation.
//
//
//
// Example 1:
//
// Input: num1 = 3, num2 = 5
// Output: 3
// Explanation:
// The binary representations of num1 and num2 are 0011 and 0101, respectively.
// The integer 3 has the same number of set bits as num2, and the value 3 XOR 3 = 0 is minimal.
// Example 2:
//
// Input: num1 = 1, num2 = 12
// Output: 3
// Explanation:
// The binary representations of num1 and num2 are 0001 and 1100, respectively.
// The integer 3 has the same number of set bits as num2, and the value 3 XOR 1 = 2 is minimal.
//
//
// Constraints:
//
// 1 <= num1, num2 <= 109
// Truth table XOR
// A  B  R
// 0  0  0
// 0  1  1
// 1  0  1
// 1  1  0
//
// Solution has
// Time complexity: O(1)
// Space complexity: O(1)
struct Solution;
impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut ones_num2 = 0;
        let mut ones_num1 = 0;
        let mut output = num1;
        for i in 0..32 {
            if (num2 >> i) & 1 == 1 {
                ones_num2 += 1;
            }
            if (num1 >> i) & 1 == 1 {
                ones_num1 += 1;
            }
        }
        if ones_num1 != ones_num2 {
            if ones_num2 > ones_num1 {
                let mut diff = ones_num2 - ones_num1;
                if diff != 0 {
                    for i in 0..32 {
                        if diff == 0 {
                            break;
                        }
                        if (num1 >> i) & 1 == 0 {
                            output |= 1 << i;
                            diff -= 1;
                        }
                    }
                }
            } else {
                let mut ones = ones_num2;
                output = num1;
                for i in (0..32).rev() {
                    if (num1 >> i) & 1 == 1 {
                        if ones != 0 {
                            ones -= 1;
                        } else {
                            output &= !(1 << i);
                        }
                    }
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
        assert_eq!(Solution::minimize_xor(3, 5), 3)
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::minimize_xor(1, 12), 3)
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::minimize_xor(25, 72), 24)
    }
}
