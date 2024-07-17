// Solution has
// Time complexity: O(log(n))
// Space complexity: O(1)
//
// An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.
//
// Given an integer n, return true if n is an ugly number.
struct Solution;
impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n == 1 {
            return true;
        }

        if n > 0 {
            while n != 2 && n != 3 && n != 5 {
                println!("n = {}", n);
                if n % 2 == 0 {
                    n /= 2;
                } else if n % 3 == 0 {
                    n /= 3;
                } else if n % 5 == 0 {
                    n /= 5;
                } else {
                    return false;
                }
            }
        } else {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Solution::is_ugly(6));
    }

    #[test]
    fn it_works2() {
        assert!(Solution::is_ugly(1));
    }

    #[test]
    fn it_works3() {
        assert!(!Solution::is_ugly(14));
    }

    #[test]
    fn it_works4() {
        assert!(Solution::is_ugly(25));
    }
}
