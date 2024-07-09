// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
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
type ValTilt = (i32, i32);
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> ValTilt {
            if let Some(node) = node {
                let l = dfs(&node.as_ref().borrow().left);
                let r = dfs(&node.as_ref().borrow().right);
                (
                    l.0 + r.0 + node.as_ref().borrow().val,
                    (l.1 - r.0).abs() + l.1 + r.1,
                )
            } else {
                (0, 0)
            }
        }
        dfs(&root).1
    }
}
