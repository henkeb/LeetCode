// Solution has
// Time complexity: O(2^n)
// Space complexity: O(n)
//
// Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sum to target.
//
// Each number in candidates may only be used once in the combination.
//
// Note: The solution set must not contain duplicate combinations.
struct Solution;
impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut combinations = Vec::new();
        let mut combination = Vec::new();

        Self::backtrack(&candidates, &mut combinations, &mut combination, 0, target);
        combinations
    }

    fn backtrack(
        candidates: &[i32],
        combinations: &mut Vec<Vec<i32>>,
        combination: &mut Vec<i32>,
        start: usize,
        remain: i32,
    ) {
        if remain == 0 {
            combinations.push(combination.clone());
            return;
        }
        for i in start..candidates.len() {
            if i > start && candidates[i] == candidates[i - 1] {
                continue;
            }
            if candidates[i] > remain {
                break;
            }
            combination.push(candidates[i]);
            Self::backtrack(
                candidates,
                combinations,
                combination,
                i + 1,
                remain - candidates[i],
            );
            combination.pop();
        }
    }
}
