// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// A city is represented as a bi-directional connected graph with n vertices where each vertex is labeled from 1 to n (inclusive). The edges in the graph are represented as a 2D integer array edges, where each edges[i] = [ui, vi] denotes a bi-directional edge between vertex ui and vertex vi. Every vertex pair is connected by at most one edge, and no vertex has an edge to itself. The time taken to traverse any edge is time minutes.
//
// Each vertex has a traffic signal which changes its color from green to red and vice versa every change minutes. All signals change at the same time. You can enter a vertex at any time, but can leave a vertex only when the signal is green. You cannot wait at a vertex if the signal is green.
//
// The second minimum value is defined as the smallest value strictly larger than the minimum value.
//
//     For example the second minimum value of [2, 3, 4] is 3, and the second minimum value of [2, 2, 4] is 4.
//
// Given n, edges, time, and change, return the second minimum time it will take to go from vertex 1 to vertex n.
//
// Notes:
//
//     You can go through any vertex any number of times, including 1 and n.
//     You can assume that when the journey starts, all signals have just turned green.

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n = n as usize;
        let mut adj_list: Vec<Vec<usize>> = vec![vec![]; n];
        for (v, u) in edges
            .into_iter()
            .map(|edge| (edge[0] as usize - 1, edge[1] as usize - 1))
        {
            adj_list[v].push(u);
            adj_list[u].push(v);
        }
        let mut queue: VecDeque<(i32, usize)> = VecDeque::from([(0, 0)]);
        let mut visited: Vec<[Option<i32>; 2]> = vec![[None, None]; n];
        visited[0][0] = Some(0);
        while let Some((cost, v)) = queue.pop_front() {
            if v == n - 1 {
                if let Some(cost_1) = visited[v][1] {
                    return cost_1;
                }
            }
            let wait_time = if cost % (2 * change) < change {
                0
            } else {
                2 * change - cost % (2 * change)
            };
            let next_cost = cost + time + wait_time;
            for &u in adj_list[v].iter() {
                match (visited[u][0], visited[u][1]) {
                    (None, None) => {
                        visited[u][0] = Some(next_cost);
                    }
                    (Some(cost_0), None) if cost_0 < next_cost => {
                        visited[u][1] = Some(next_cost);
                    }
                    (_, _) => {
                        continue;
                    }
                };
                queue.push_back((next_cost, u));
            }
        }
        unreachable!()
    }
}
