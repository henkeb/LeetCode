// Solution has
// Time complexity: O(1)
// Space complexity: O(1)
//
// You have n coins and you want to build a staircase with these coins. The staircase consists of k rows where the ith row has exactly i coins. The last row of the staircase may be incomplete.
//
// Given the integer n, return the number of complete rows of the staircase you will build.
struct Solution;
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        // x(x+1)/2
        // n = x²/2 + x/2
        // 0 = x²/2 + x/2 - n
        // a = 1/2; b = 1/2; c = -n
        // positive solution: x = (-b + sqrt(b²-4ac))/2a
        // x = (-1/2 + sqrt(1/2 * 1/2 - 4 * 1/2 * -n)) / 2 * 1/2
        // x = -1/2 + sqrt(1/4 + 2 * n)
        // x = sqrt(0.25 + 2.0 * n) - 0.25
        ((0.25 + 2.0 * n as f64).sqrt() - 0.5).floor() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::arrange_coins(5), 2);
    }

    #[test]
    fn it_works2() {
        assert_eq!(Solution::arrange_coins(8), 3);
    }

    #[test]
    fn it_works3() {
        assert_eq!(Solution::arrange_coins(1), 1);
    }

    #[test]
    fn it_works4() {
        assert_eq!(Solution::arrange_coins(2), 1);
    }

    #[test]
    fn it_works5() {
        assert_eq!(Solution::arrange_coins(3), 2);
    }
}
