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
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min_diff = i32::MAX;
        let mut prev_node: Option<i32> = None;
        Self::in_order(&root, &mut min_diff, &mut prev_node);
        min_diff
    }

    fn in_order(
        node: &Option<Rc<RefCell<TreeNode>>>,
        min_diff: &mut i32,
        prev_node: &mut Option<i32>,
    ) {
        if let Some(n) = node {
            let n = n.as_ref().borrow();

            Self::in_order(&n.left, min_diff, prev_node);

            if let Some(prev) = prev_node {
                *min_diff = std::cmp::min(*min_diff, n.val - *prev);
            }
            *prev_node = Some(n.val);
            Self::in_order(&n.right, min_diff, prev_node);
        }
    }
}
