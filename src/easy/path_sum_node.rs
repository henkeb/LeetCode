// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
//Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.

// A leaf is a node with no children.
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
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(..) => Self::dfs(&root, target_sum),
            None => false,
        }
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = node {
            let diff = target_sum - node.as_ref().borrow().val;
            let left = &node.as_ref().borrow().left;
            let right = &node.as_ref().borrow().right;
            match (left, right) {
                (None, None) => diff == 0,
                (Some(..), None) => Self::dfs(left, diff),
                (None, Some(..)) => Self::dfs(right, diff),
                (Some(..), Some(..)) => Self::dfs(left, diff) || Self::dfs(right, diff),
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert!(!Solution::has_path_sum(None, 0));
    }

    #[test]
    fn test2() {
        assert!(Solution::has_path_sum(
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            1
        ));
    }
}
