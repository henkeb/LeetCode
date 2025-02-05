use std::collections::{HashMap, HashSet};

// In this problem, a tree is an undirected graph that is connected and has no cycles.
//
// You are given a graph that started as a tree with n nodes labeled from 1 to n, with one additional edge added. The added edge has two different vertices chosen from 1 to n, and was not an edge that already existed. The graph is represented as an array edges of length n where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the graph.
//
// Return an edge that can be removed so that the resulting graph is a tree of n nodes. If there are multiple answers, return the answer that occurs last in the input.
//
//
//
// Example 1:
//
//
// Input: edges = [[1,2],[1,3],[2,3]]
// Output: [2,3]
// Example 2:
//
//
// Input: edges = [[1,2],[2,3],[3,4],[1,4],[1,5]]
// Output: [1,4]
//
//
// Constraints:
//
// n == edges.length
// 3 <= n <= 1000
// edges[i].length == 2
// 1 <= ai < bi <= edges.length
// ai != bi
// There are no repeated edges.
// The given graph is connected.
//
// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
struct Solution;
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let is_connected = |u: i32, v: i32, graph: &HashMap<i32, Vec<i32>>| -> bool {
            let mut visited: HashSet<i32> = HashSet::new();
            let mut stack: Vec<i32> = Vec::new();

            stack.push(u);

            while let Some(node) = stack.pop() {
                if visited.contains(&node) {
                    continue;
                }
                visited.insert(node);
                if node == v {
                    return true;
                }

                for neighbour in graph.get(&node).unwrap().iter() {
                    stack.push(*neighbour);
                }
            }
            false
        };
        for edge in edges.into_iter() {
            let u = edge[0];
            let v = edge[1];
            if graph.contains_key(&u) && graph.contains_key(&v) && is_connected(u, v, &graph) {
                return edge;
            }
            graph
                .entry(u)
                .and_modify(|edges| edges.push(v))
                .or_insert(vec![v]);
            graph
                .entry(v)
                .and_modify(|edges| edges.push(u))
                .or_insert(vec![u]);
        }
        vec![]
    }
}
