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
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert_node(root.as_ref());

        root
    }

    fn invert_node(node: Option<&Rc<RefCell<TreeNode>>>) {
        if let Some(node) = node {
            let mut inner = node.as_ref().borrow_mut();

            // Invert children.
            Self::invert_node(inner.left.as_ref());
            Self::invert_node(inner.right.as_ref());

            // Swap children.
            let l = inner.left.take();
            let r = inner.right.take();

            inner.left = r;
            inner.right = l;
        };
    }
}
