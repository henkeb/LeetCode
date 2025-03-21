// There is an undirected weighted graph with n vertices labeled from 0 to n - 1.
//
// You are given the integer n and an array edges, where edges[i] = [ui, vi, wi] indicates that there is an edge between vertices ui and vi with a weight of wi.
//
// A walk on a graph is a sequence of vertices and edges. The walk starts and ends with a vertex, and each edge connects the vertex that comes before it and the vertex that comes after it. It's important to note that a walk may visit the same edge or vertex more than once.
//
// The cost of a walk starting at node u and ending at node v is defined as the bitwise AND of the weights of the edges traversed during the walk. In other words, if the sequence of edge weights encountered during the walk is w0, w1, w2, ..., wk, then the cost is calculated as w0 & w1 & w2 & ... & wk, where & denotes the bitwise AND operator.
//
// You are also given a 2D array query, where query[i] = [si, ti]. For each query, you need to find the minimum cost of the walk starting at vertex si and ending at vertex ti. If there exists no such walk, the answer is -1.
//
// Return the array answer, where answer[i] denotes the minimum cost of a walk for query i.
//
//
//
// Example 1:
//
// Input: n = 5, edges = [[0,1,7],[1,3,7],[1,2,1]], query = [[0,3],[3,4]]
//
// Output: [1,-1]
//
// Explanation:
//
//
// To achieve the cost of 1 in the first query, we need to move on the following edges: 0->1 (weight 7), 1->2 (weight 1), 2->1 (weight 1), 1->3 (weight 7).
//
// In the second query, there is no walk between nodes 3 and 4, so the answer is -1.
//
// Example 2:
//
// Input: n = 3, edges = [[0,2,7],[0,1,15],[1,2,6],[1,2,1]], query = [[1,2]]
//
// Output: [0]
//
// Explanation:
//
//
// To achieve the cost of 0 in the first query, we need to move on the following edges: 1->2 (weight 1), 2->1 (weight 6), 1->2 (weight 1).
//
//
//
// Constraints:
//
// 2 <= n <= 105
// 0 <= edges.length <= 105
// edges[i].length == 3
// 0 <= ui, vi <= n - 1
// ui != vi
// 0 <= wi <= 105
// 1 <= query.length <= 105
// query[i].length == 2
// 0 <= si, ti <= n - 1
// si != ti
struct Solution;
impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let mut parent: Vec<i32> = (0..n).collect();
        let mut min_path_cost: Vec<i32> = vec![-1; n as usize];

        fn find_root(parent: &mut Vec<i32>, node: i32) -> i32 {
            if parent[node as usize] != node {
                parent[node as usize] = find_root(parent, parent[node as usize]);
            }
            parent[node as usize]
        }

        for edge in edges.iter() {
            let source = edge[0];
            let target = edge[1];
            let weight = edge[2];

            let source_root = find_root(&mut parent, source);
            let target_root = find_root(&mut parent, target);

            min_path_cost[target_root as usize] &= weight;

            if source_root != target_root {
                min_path_cost[target_root as usize] &= min_path_cost[source_root as usize];
                parent[source_root as usize] = target_root;
            }
        }

        let mut result = Vec::new();
        for q in query.iter() {
            let start = q[0];
            let end = q[1];

            if start == end {
                result.push(0);
            } else if find_root(&mut parent, start) != find_root(&mut parent, end) {
                result.push(-1);
            } else {
                result.push(min_path_cost[find_root(&mut parent, start) as usize]);
            }
        }

        result
    }
}
