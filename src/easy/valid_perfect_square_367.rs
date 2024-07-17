// Solution has
// Time complexity: O(log(n))
// Space complexity: O(1)
//
// Given a positive integer num, return true if num is a perfect square or false otherwise.
//
// A perfect square is an integer that is the square of an integer. In other words, it is the product of some integer with itself.
//
// You must not use any built-in library function, such as sqrt.
struct Solution;
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 1 {
            return true;
        }
        let n: usize = num as usize;
        use std::cmp::Ordering;
        let mut l = 1;
        let mut h = n / 2;
        while l <= h {
            let mid = l + (h - l) / 2;
            match n.cmp(&(mid * mid)) {
                Ordering::Equal => return true,
                Ordering::Greater => l = mid + 1,
                Ordering::Less => h = mid - 1,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_16() {
        assert!(Solution::is_perfect_square(16));
    }

    #[test]
    fn square_14() {
        assert!(!Solution::is_perfect_square(14));
    }

    #[test]
    fn square_808201() {
        assert!(Solution::is_perfect_square(808201));
    }
}
