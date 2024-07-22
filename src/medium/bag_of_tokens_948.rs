// Solution has
// Time complexity: O(n*log(n))
// Space complexity: O(1)
//
// You start with an initial power of power, an initial score of 0, and a bag of tokens given as an integer array tokens, where each tokens[i] denotes the value of tokeni.
//
// Your goal is to maximize the total score by strategically playing these tokens. In one move, you can play an unplayed token in one of the two ways (but not both for the same token):
//
//     Face-up: If your current power is at least tokens[i], you may play tokeni, losing tokens[i] power and gaining 1 score.
//     Face-down: If your current score is at least 1, you may play tokeni, gaining tokens[i] power and losing 1 score.
//
// Return the maximum possible score you can achieve after playing any number of tokens.

struct Solution;
impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }
        tokens.sort_unstable();
        let mut front = 0;
        let mut back = tokens.len() - 1;
        let mut score = 0;
        while front <= back {
            if tokens[front] <= power {
                power -= tokens[front];
                front += 1;
                score += 1;
            } else if score > 0 && (back - front > 1) {
                power += tokens[back];
                back -= 1;
                score -= 1;
            } else {
                return score;
            }
        }
        score
    }
}
