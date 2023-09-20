// Solution has
// Time complexity: O(n)
// Space complexity: O(1)

//You are given an integer array cost where cost[i] is the cost of ith step on a staircase. Once you pay the cost, you can either climb one or two steps.
//
// You can either start from the step with index 0, or the step with index 1.
//
// Return the minimum cost to reach the top of the floor.
struct Solution {}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut t0 = 0;
        let mut t1 = 1;
        let mut t2 = 1;
        let mut next_value;
        match n {
            0 => return t0,
            1 | 2 => return t1,
            _ => (),
        }

        for _ in 3..=n as usize {
            next_value = t0 + t1 + t2;
            t0 = t1;
            t1 = t2;
            t2 = next_value;
        }
        t2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::tribonacci(4), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
