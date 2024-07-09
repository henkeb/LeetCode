// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// You are given an integer array score of size n, where score[i] is the score of the ith athlete in a competition. All the scores are guaranteed to be unique.
//
// The athletes are placed based on their scores, where the 1st place athlete has the highest score, the 2nd place athlete has the 2nd highest score, and so on. The placement of each athlete determines their rank:
//
//     The 1st place athlete's rank is "Gold Medal".
//     The 2nd place athlete's rank is "Silver Medal".
//     The 3rd place athlete's rank is "Bronze Medal".
//     For the 4th place to the nth place athlete, their rank is their placement number (i.e., the xth place athlete's rank is "x").
//
// Return an array answer of size n where answer[i] is the rank of the ith athlete.

struct Solution;
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut sorted_score: Vec<i32> = score.clone();
        sorted_score.sort_by(|a, b| b.cmp(a));
        let mut output: Vec<String> = vec![];

        score
            .iter()
            .for_each(|&val| match sorted_score.iter().position(|x| x == &val) {
                Some(0) => output.push("Gold Medal".to_string()),
                Some(1) => output.push("Silver Medal".to_string()),
                Some(2) => output.push("Bronze Medal".to_string()),
                Some(val) => output.push(format!("{}", val + 1)),
                None => unreachable!(),
            });

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_ok() {
        assert_eq!(
            Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
            vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
        )
    }

    #[test]
    fn relative_2nd() {
        assert_eq!(
            Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
            vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
        )
    }
}
