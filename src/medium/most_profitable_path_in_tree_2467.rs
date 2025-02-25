use std::collections::{HashMap, HashSet, VecDeque};

// There is an undirected tree with n nodes labeled from 0 to n - 1, rooted at node 0. You are given a 2D integer array edges of length n - 1 where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.
//
// At every node i, there is a gate. You are also given an array of even integers amount, where amount[i] represents:
//
// the price needed to open the gate at node i, if amount[i] is negative, or,
// the cash reward obtained on opening the gate at node i, otherwise.
// The game goes on as follows:
//
// Initially, Alice is at node 0 and Bob is at node bob.
// At every second, Alice and Bob each move to an adjacent node. Alice moves towards some leaf node, while Bob moves towards node 0.
// For every node along their path, Alice and Bob either spend money to open the gate at that node, or accept the reward. Note that:
// If the gate is already open, no price will be required, nor will there be any cash reward.
// If Alice and Bob reach the node simultaneously, they share the price/reward for opening the gate there. In other words, if the price to open the gate is c, then both Alice and Bob pay c / 2 each. Similarly, if the reward at the gate is c, both of them receive c / 2 each.
// If Alice reaches a leaf node, she stops moving. Similarly, if Bob reaches node 0, he stops moving. Note that these events are independent of each other.
// Return the maximum net income Alice can have if she travels towards the optimal leaf node.
//
//
//
// Example 1:
//
//
// Input: edges = [[0,1],[1,2],[1,3],[3,4]], bob = 3, amount = [-2,4,2,-4,6]
// Output: 6
// Explanation:
// The above diagram represents the given tree. The game goes as follows:
// - Alice is initially on node 0, Bob on node 3. They open the gates of their respective nodes.
//   Alice's net income is now -2.
// - Both Alice and Bob move to node 1.
//   Since they reach here simultaneously, they open the gate together and share the reward.
//   Alice's net income becomes -2 + (4 / 2) = 0.
// - Alice moves on to node 3. Since Bob already opened its gate, Alice's income remains unchanged.
//   Bob moves on to node 0, and stops moving.
// - Alice moves on to node 4 and opens the gate there. Her net income becomes 0 + 6 = 6.
// Now, neither Alice nor Bob can make any further moves, and the game ends.
// It is not possible for Alice to get a higher net income.
// Example 2:
//
//
// Input: edges = [[0,1]], bob = 1, amount = [-7280,2350]
// Output: -7280
// Explanation:
// Alice follows the path 0->1 whereas Bob follows the path 1->0.
// Thus, Alice opens the gate at node 0 only. Hence, her net income is -7280.
//
//
// Constraints:
//
// 2 <= n <= 105
// edges.length == n - 1
// edges[i].length == 2
// 0 <= ai, bi < n
// ai != bi
// edges represents a valid tree.
// 1 <= bob < n
// amount.length == n
// amount[i] is an even integer in the range [-104, 104].
struct Solution;
impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::default();
        for edge in edges.iter() {
            adj_list
                .entry(edge[0])
                .and_modify(|list| list.push(edge[1]))
                .or_insert(vec![edge[1]]);
            adj_list
                .entry(edge[1])
                .and_modify(|list| list.push(edge[0]))
                .or_insert(vec![edge[0]]);
        }

        // Get bob path
        let mut path = vec![bob];
        let mut visited = HashSet::new();
        visited.insert(bob);
        Solution::bob_dfs(bob, &adj_list, &mut path, &mut visited);
        let mut bob_time: HashMap<i32, usize> = HashMap::new();
        for (i, val) in path.iter().enumerate() {
            bob_time.insert(*val, i);
        }

        Solution::alice_bfs(&bob_time, &amount, &adj_list)
    }

    fn alice_bfs(
        bob_time: &HashMap<i32, usize>,
        amount: &Vec<i32>,
        adj_list: &HashMap<i32, Vec<i32>>,
    ) -> i32 {
        let mut visited = HashSet::new();
        visited.insert(&0);
        let mut queue = VecDeque::new();

        queue.push_back((0, 0, 0));

        let mut max_profit = i32::min_value();
        while let Some((alice_node, alice_time, mut profit)) = queue.pop_front() {
            if let Some(&bob_time) = bob_time.get(&alice_node) {
                if bob_time == alice_time {
                    println!("here, alice_node {alice_node}");
                    println!("bob_time: {bob_time}");
                    println!("alice_time: {alice_time}");
                    profit += amount[alice_node as usize] / 2;
                } else if bob_time > alice_time {
                    profit += amount[alice_node as usize];
                }
            } else {
                profit += amount[alice_node as usize];
            }

            let mut count = 0;
            for neighbour in adj_list.get(&alice_node).unwrap().iter() {
                if !visited.contains(neighbour) {
                    queue.push_back((*neighbour, alice_time + 1, profit));
                    visited.insert(neighbour);
                    count += 1;
                }
            }

            if count == 0 {
                max_profit = max_profit.max(profit);
            }
        }
        max_profit
    }

    fn bob_dfs(
        node: i32,
        adj_list: &HashMap<i32, Vec<i32>>,
        path: &mut Vec<i32>,
        visited: &mut HashSet<i32>,
    ) -> bool {
        if node == 0 {
            return true;
        }

        if let Some(nodes) = adj_list.get(&node) {
            for node in nodes.iter() {
                if !visited.contains(&node) {
                    path.push(*node);
                    visited.insert(*node);
                    if Solution::bob_dfs(*node, adj_list, path, visited) {
                        return true;
                    }
                    path.pop();
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::most_profitable_path(
                vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]],
                3,
                vec![-2, 4, 2, -4, 6]
            ),
            6
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::most_profitable_path(vec![vec![0, 1]], 1, vec![-7280, 2350]),
            -7280
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::most_profitable_path(
                vec![vec![0, 1], vec![0, 2]],
                2,
                vec![-3360, -5394, -1146]
            ),
            -3360
        )
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::most_profitable_path(
                vec![
                    vec![0, 2],
                    vec![1, 4],
                    vec![1, 6],
                    vec![2, 4],
                    vec![3, 6],
                    vec![3, 7],
                    vec![5, 7]
                ],
                4,
                vec![-6896, -1216, -1208, -1108, 1606, -7704, -9212, -8258]
            ),
            -34998
        )
    }
}
