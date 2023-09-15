// Solution has:
// time complexity O(n)
// space complexity O(1)
struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut current_max = 0;
        let mut minimum = i32::MAX;

        for val in prices.iter() {
            if val < &minimum {
                minimum = *val;
            }

            match val - minimum {
                x if x > current_max => current_max = x,
                _ => (),
            }
        }
        current_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(input), 5);
    }

    #[test]
    fn test2() {
        let input = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(input), 0);
    }
}
