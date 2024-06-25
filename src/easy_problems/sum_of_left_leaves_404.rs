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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, false)
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        if let Some(node) = node {
            let node = node.as_ref().borrow();
            if node.left.is_none() && node.right.is_none() && is_left {
                return node.val;
            }

            return Self::dfs(&node.left, true) + Self::dfs(&node.right, false);
        }
        0
    }
}
