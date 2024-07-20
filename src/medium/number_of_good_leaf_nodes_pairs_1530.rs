// Solution has
// Time complexity: O(n²)
// Space complexity: O(n)
//
//You are given the root of a binary tree and an integer distance. A pair of two different leaf nodes of a binary tree is said to be good if the length of the shortest path between them is less than or equal to distance.

// Return the number of good leaf node pairs in the tree.
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
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        let mut node_pairs: i32 = 0;
        Self::dfs(root, distance, &mut node_pairs);
        node_pairs
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, distance: i32, node_pairs: &mut i32) -> Vec<i32> {
        if let Some(node) = node {
            let mut left = Self::dfs(node.borrow().left.clone(), distance, node_pairs);
            let mut right = Self::dfs(node.borrow().right.clone(), distance, node_pairs);
            match (!left.is_empty(), !right.is_empty()) {
                (true, true) => {
                    for i in left.iter() {
                        for j in right.iter() {
                            if i + j <= distance {
                                *node_pairs += 1;
                            }
                        }
                    }
                    left.append(&mut right);
                    left.iter_mut().for_each(|val| *val += 1);
                    left
                }
                (true, false) => {
                    left.iter_mut().for_each(|val| *val += 1);
                    left
                }
                (false, true) => {
                    right.iter_mut().for_each(|val| *val += 1);
                    right
                }
                (false, false) => vec![1],
            }
        } else {
            vec![]
        }
    }
}
