// Solution has
// Time complexity: O(n³)
// Space complexity: O(1)
//
// You are given two 0-indexed strings source and target, both of length n and consisting of lowercase English letters. You are also given two 0-indexed character arrays original and changed, and an integer array cost, where cost[i] represents the cost of changing the character original[i] to the character changed[i].
//
// You start with the string source. In one operation, you can pick a character x from the string and change it to the character y at a cost of z if there exists any index j such that cost[j] == z, original[j] == x, and changed[j] == y.
//
// Return the minimum cost to convert the string source to the string target using any number of operations. If it is impossible to convert source to target, return -1.
//
// Note that there may exist indices i, j such that original[j] == original[i] and changed[j] == changed[i].
struct Solution;
impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        const ALPHABET: usize = 26;
        const A: usize = 97;
        const MAX_VAL: usize = 1e9 as usize;
        let len = original.len();
        // initialize distance with "inf"
        let mut distance_matrix = [[MAX_VAL; ALPHABET]; ALPHABET];
        // fill distance matrix;
        (0..len).for_each(|i| {
            distance_matrix[original[i] as usize - A][changed[i] as usize - A] = std::cmp::min(
                distance_matrix[original[i] as usize - A][changed[i] as usize - A],
                cost[i] as usize,
            );
        });
        (0..ALPHABET).for_each(|i| distance_matrix[i][i] = 0);

        // Floyd Warshalls algo
        for k in 0..ALPHABET {
            for i in 0..ALPHABET {
                for j in 0..ALPHABET {
                    distance_matrix[i][j] =
                        distance_matrix[i][j].min(distance_matrix[i][k] + distance_matrix[k][j]);
                }
            }
        }

        // find out the total cost
        let mut cost = 0;
        for i in 0..source.len() {
            let cost_pair = distance_matrix[source.as_bytes()[i] as usize - A]
                [target.as_bytes()[i] as usize - A];
            if cost_pair >= MAX_VAL {
                return -1;
            }
            cost += cost_pair;
        }
        cost as i64
    }
}
