// Solution has
// Time complexity: O(Log(n))
// Space complexity: O(1)

// Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.
//
// You must not use any built-in exponent function or operator.
//
//     For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.
struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut low = 0;
        let mut high = 46340.min(x / 2); // Max value for sqrt(i32::MAX)

        match x {
            0 => return 0,
            1..=3 => return 1,
            _ => (),
        }

        if x >= high * high {
            return high;
        }

        while high - low > 1 {
            let mid = (high + low) / 2;
            let pow = mid * mid;
            match pow.cmp(&x) {
                std::cmp::Ordering::Less => low = mid,
                std::cmp::Ordering::Greater => high = mid,
                std::cmp::Ordering::Equal => return mid,
            }
        }
        low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }

    #[test]
    fn test_sqrt2() {
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}
