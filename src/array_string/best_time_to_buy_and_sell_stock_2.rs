// Solution has:
// time complexity O(n)
// space complexity O(1)
struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        prices
            .iter()
            .enumerate()
            .skip(1)
            .for_each(|(i, val)| match val - prices[i - 1] {
                x if x > 0 => profit += x,
                _ => (),
            });
        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(input), 7);
    }

    #[test]
    fn test2() {
        let input = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(input), 0);
    }

    #[test]
    fn test3() {
        let input = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_profit(input), 4);
    }
}
