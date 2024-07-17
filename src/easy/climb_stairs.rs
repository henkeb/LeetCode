// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// You are climbing a staircase. It takes n steps to reach the top.
//
// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // use fibonacci sequence
        let mut n_minus_2 = 0;
        let mut n_minus_1 = 1;
        let mut number_of_different_steps = 0;

        for _ in 1..=n {
            // calculate current steps
            number_of_different_steps = n_minus_1 + n_minus_2;
            // update previous steps for next iteration
            n_minus_2 = n_minus_1;
            n_minus_1 = number_of_different_steps;
        }
        number_of_different_steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn test_solution3() {
        assert_eq!(Solution::climb_stairs(4), 5);
    }
}
