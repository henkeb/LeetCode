// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given the root of a binary search tree (BST) with duplicates, return all the mode(s) (i.e., the most frequently occurred element) in it.
//
// If the tree has more than one mode, return them in any order.
//
// Assume a BST is defined as follows:
//
//     The left subtree of a node contains only nodes with keys less than or equal to the node's key.
//     The right subtree of a node contains only nodes with keys greater than or equal to the node's key.
//     Both the left and right subtrees must also be binary search trees.
//
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
use std::collections::HashMap;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut mode_map: HashMap<i32, i32> = HashMap::new();
        if root.is_some() {
            Self::dfs(&root, &mut mode_map);
        } else {
            return vec![];
        }
        let max = &mode_map.values().max().unwrap().clone();
        mode_map
            .into_iter()
            .filter(|(_, value)| *value == *max)
            .map(|(key, _value)| key)
            .collect()
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, mode_map: &mut HashMap<i32, i32>) {
        if let Some(node) = node {
            let node = node.as_ref().borrow();
            Self::dfs(&node.left, mode_map);
            match mode_map.get(&node.val) {
                Some(val) => {
                    mode_map.insert(node.val, val + 1);
                }
                None => {
                    mode_map.insert(node.val, 1);
                }
            }
            Self::dfs(&node.right, mode_map);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::find_mode(None), vec![]);
    }

    #[test]
    fn it_works2() {
        assert_eq!(
            Solution::find_mode(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            vec![1]
        );
    }
}
