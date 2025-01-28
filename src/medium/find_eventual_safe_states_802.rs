// There is a directed graph of n nodes with each node labeled from 0 to n - 1. The graph is represented by a 0-indexed 2D integer array graph where graph[i] is an integer array of nodes adjacent to node i, meaning there is an edge from node i to each node in graph[i].
//
// A node is a terminal node if there are no outgoing edges. A node is a safe node if every possible path starting from that node leads to a terminal node (or another safe node).
//
// Return an array containing all the safe nodes of the graph. The answer should be sorted in ascending order.
//
//
//
// Example 1:
//
// Illustration of graph
// Input: graph = [[1,2],[2,3],[5],[0],[5],[],[]]
// Output: [2,4,5,6]
// Explanation: The given graph is shown above.
// Nodes 5 and 6 are terminal nodes as there are no outgoing edges from either of them.
// Every path starting at nodes 2, 4, 5, and 6 all lead to either node 5 or 6.
// Example 2:
//
// Input: graph = [[1,2,3,4],[1,2],[3,4],[0,4],[]]
// Output: [4]
// Explanation:
// Only node 4 is a terminal node, and every path starting at node 4 leads to node 4.
//
//
// Constraints:
//
// n == graph.length
// 1 <= n <= 104
// 0 <= graph[i].length <= n
// 0 <= graph[i][j] <= n - 1
// graph[i] is sorted in a strictly increasing order.
// The graph may contain self-loops.
// The number of edges in the graph will be in the range [1, 4 * 104].
struct Solution;
#[derive(Clone)]
enum State {
    Unvisited,
    Visiting,
    Safe,
    Unsafe,
}

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut state = vec![State::Unvisited; graph.len()];
        let mut safe = Vec::new();
        for node in 0..graph.len() {
            if !has_cycle(node, &mut state, &graph) {
                safe.push(node as i32);
            }
        }
        safe.sort();
        safe
    }
}

fn has_cycle(node: usize, state: &mut Vec<State>, graph: &Vec<Vec<i32>>) -> bool {
    match state[node] {
        State::Safe => false,
        State::Unsafe | State::Visiting => true,
        _ => {
            state[node] = State::Visiting;
            for &neighbour in graph[node].iter() {
                if has_cycle(neighbour as usize, state, graph) {
                    state[node] = State::Unsafe;
                    return true;
                }
            }
            state[node] = State::Safe;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::eventual_safe_nodes(vec![
                vec![1, 2],
                vec![2, 3],
                vec![5],
                vec![0],
                vec![5],
                vec![],
                vec![]
            ]),
            vec![2, 4, 5, 6]
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::eventual_safe_nodes(vec![vec![], vec![0, 2, 3, 4], vec![3], vec![4], vec![]]),
            vec![0, 1, 2, 3, 4]
        );
    }
}
