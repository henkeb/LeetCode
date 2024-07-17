// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1. That is,
//
// F(0) = 0, F(1) = 1
// F(n) = F(n - 1) + F(n - 2), for n > 1.
//
// Given n, calculate F(n).

struct Solution;
impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut fib_2 = 0;
        let mut fib_1 = 1;
        if n == 0 {
            return fib_2;
        }

        if n == 1 {
            return fib_1;
        }

        let mut current_fib = 0;
        (2..=n).for_each(|_| {
            current_fib = fib_2 + fib_1;
            fib_2 = fib_1;
            fib_1 = current_fib;
        });
        current_fib
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibbi() {
        assert_eq!(Solution::fib(2), 1)
    }
    #[test]
    fn fibbi_dos() {
        assert_eq!(Solution::fib(4), 3)
    }
}
