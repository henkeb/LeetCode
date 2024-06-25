// Given an integer n, return a string array answer (1-indexed) where:
//
//     answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
//     answer[i] == "Fizz" if i is divisible by 3.
//     answer[i] == "Buzz" if i is divisible by 5.
//     answer[i] == i (as a string) if none of the above conditions are true.

struct Solution;
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for i in 1..=n {
            match (i % 3, i % 5) {
                (0, 0) => output.push("FizzBuzz".to_string()),
                (0, _) => output.push("Fizz".to_string()),
                (_, 0) => output.push("Buzz".to_string()),
                (_, _) => output.push(i.to_string()),
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizz_buzz_3() {
        assert_eq!(Solution::fizz_buzz(3), vec!["1", "2", "Fizz"]);
    }

    #[test]
    fn fizz_buzz_5() {
        assert_eq!(Solution::fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    }
}
