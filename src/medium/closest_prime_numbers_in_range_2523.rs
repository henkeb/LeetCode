// Given two positive integers left and right, find the two integers num1 and num2 such that:
//
// left <= num1 < num2 <= right .
// Both num1 and num2 are prime numbers.
// num2 - num1 is the minimum amongst all other pairs satisfying the above conditions.
// Return the positive integer array ans = [num1, num2]. If there are multiple pairs satisfying these conditions, return the one with the smallest num1 value. If no such numbers exist, return [-1, -1].
//
//
//
// Example 1:
//
// Input: left = 10, right = 19
// Output: [11,13]
// Explanation: The prime numbers between 10 and 19 are 11, 13, 17, and 19.
// The closest gap between any pair is 2, which can be achieved by [11,13] or [17,19].
// Since 11 is smaller than 17, we return the first pair.
// Example 2:
//
// Input: left = 4, right = 6
// Output: [-1,-1]
// Explanation: There exists only one prime number in the given range, so the conditions cannot be satisfied.
//
//
// Constraints:
//
// 1 <= left <= right <= 106
struct Solution;
impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut prev_prime = 0;
        let mut min = i32::max_value();
        let mut output = [-1, -1];
        for i in left..=right {
            if Self::is_prime(i) {
                let diff = i - prev_prime;
                if prev_prime != 0 {
                    if diff.cmp(&min) == std::cmp::Ordering::Less {
                        min = diff;
                        output[0] = prev_prime;
                        output[1] = i;
                    }
                    if diff == 2 || diff == 1 {
                        return vec![prev_prime, i];
                    }
                }
                prev_prime = i;
            }
        }

        output.to_vec()
    }

    fn is_prime(n: i32) -> bool {
        if n <= 1 {
            return false;
        }

        if n == 2 {
            return true;
        }

        if n % 2 == 0 {
            return false;
        }

        for i in (3..n).step_by(2) {
            if i * i > n {
                break;
            }
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}
