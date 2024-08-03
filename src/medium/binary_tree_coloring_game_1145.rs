// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Two players play a turn based game on a binary tree. We are given the root of this binary tree, and the number of nodes n in the tree. n is odd, and each node has a distinct value from 1 to n.
//
// Initially, the first player names a value x with 1 <= x <= n, and the second player names a value y with 1 <= y <= n and y != x. The first player colors the node with value x red, and the second player colors the node with value y blue.
//
// Then, the players take turns starting with the first player. In each turn, that player chooses a node of their color (red if player 1, blue if player 2) and colors an uncolored neighbor of the chosen node (either the left child, right child, or parent of the chosen node.)
//
// If (and only if) a player cannot choose such a node in this way, they must pass their turn. If both players pass their turn, the game ends, and the winner is the player that colored more nodes.
//
// You are the second player. If it is possible to choose such a y to ensure you win the game, return true. If it is not possible, return false.
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        let first_node = Self::find_opponent_starting_node(root, x).unwrap();
        let left_count = Self::count_nodes(first_node.borrow().left.clone());
        let right_count = Self::count_nodes(first_node.borrow().right.clone());
        let half_nodes = n / 2;
        if left_count > half_nodes || right_count > half_nodes {
            return true;
        }
        if left_count + right_count + 1 < (n + 1) / 2 {
            return true;
        }
        false
    }
    fn find_opponent_starting_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        x: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            if root.borrow().val == x {
                return Some(root);
            }
            let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![
                root.clone().borrow().left.clone(),
                root.clone().borrow().right.clone(),
            ];
            while let Some(child) = stack.pop() {
                if let Some(child) = child {
                    if child.borrow().val == x {
                        return Some(child);
                    }
                    stack.push(child.borrow().left.clone());
                    stack.push(child.borrow().right.clone());
                }
            }
        }
        None
    }
    fn count_nodes(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if node.is_none() {
            return 0;
        }
        let mut count = 1;
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        stack.push(node.clone().unwrap().borrow().left.clone());
        stack.push(node.clone().unwrap().borrow().right.clone());
        while let Some(child) = stack.pop() {
            if let Some(child) = child {
                count += 1;
                stack.push(child.borrow().left.clone());
                stack.push(child.borrow().right.clone());
            }
        }
        count
    }
}
