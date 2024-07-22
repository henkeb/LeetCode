// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// You are given a positive integer k. You are also given:
//
//     a 2D integer array rowConditions of size n where rowConditions[i] = [abovei, belowi], and
//     a 2D integer array colConditions of size m where colConditions[i] = [lefti, righti].
//
// The two arrays contain integers from 1 to k.
//
// You have to build a k x k matrix that contains each of the numbers from 1 to k exactly once. The remaining cells should have the value 0.
//
// The matrix should also satisfy the following conditions:
//
//     The number abovei should appear in a row that is strictly above the row at which the number belowi appears for all i from 0 to n - 1.
//     The number lefti should appear in a column that is strictly left of the column at which the number righti appears for all i from 0 to m - 1.
//
// Return any matrix that satisfies the conditions. If no answer exists, return an empty matrix.
struct Solution;
impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut matrix = vec![vec![0; k as usize]; k as usize];

        let row_sequence = Self::kahns_algo(&row_conditions, k);
        let col_sequence = Self::kahns_algo(&col_conditions, k);

        if row_sequence.is_empty() || col_sequence.is_empty() {
            return vec![];
        }
        let mut row_map = HashMap::new();
        for i in 0..row_sequence.len() {
            matrix[i][0] = row_sequence[i];
            row_map.insert(row_sequence[i], i);
        }
        (0..col_sequence.len()).for_each(|i| {
            let row = row_map.get(&col_sequence[i]).unwrap();
            matrix[*row][0] = 0;
            matrix[*row][i] = col_sequence[i];
        });
        matrix
    }

    fn kahns_algo(conditions: &Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut adj: Vec<Vec<i32>> = vec![Vec::new(); k as usize + 1];
        let mut in_degree: Vec<i32> = vec![0; k as usize + 1];
        for v in conditions {
            adj[v[0] as usize].push(v[1]);
            in_degree[v[1] as usize] += 1;
        }

        let mut queue: Vec<i32> = Vec::new();
        (1..in_degree.len()).for_each(|i| {
            if in_degree[i] == 0 {
                queue.push(i as i32);
            }
        });

        let mut topological_sort: Vec<i32> = Vec::new();
        while let Some(v) = queue.pop() {
            topological_sort.push(v);
            for &v in adj[v as usize].iter() {
                in_degree[v as usize] -= 1;
                if in_degree[v as usize] == 0 {
                    queue.push(v);
                }
            }
        }
        if topological_sort.len() != k as usize {
            return vec![];
        }
        topological_sort
    }
}
