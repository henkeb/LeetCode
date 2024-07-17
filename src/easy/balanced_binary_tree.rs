// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//Given a binary tree, determine if it is hight balanced:
//A height-balanced binary tree is a binary tree in which the depth of the two subtrees of every node never differs by more than one.
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
    fn bfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
        if let Some(node) = node {
            let n = node.as_ref().borrow();
            let (is_balanced_left, height_left) = Self::bfs(&n.left);
            if !is_balanced_left {
                return (false, 0);
            }
            let (is_balanced_right, height_right) = Self::bfs(&n.right);
            if !is_balanced_right {
                return (false, 0);
            }
            let abs = (height_left - height_right).abs();
            (abs <= 1, 1 + std::cmp::max(&height_right, &height_left))
        } else {
            (true, 0)
        }
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::bfs(&root).0
    }
}
