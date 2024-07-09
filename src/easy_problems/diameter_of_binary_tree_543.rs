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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diameter = 0;
        if let Some(root) = root {
            let diameter = Self::height(&root.as_ref().borrow().left, &mut max_diameter)
                + Self::height(&root.as_ref().borrow().right, &mut max_diameter);
            max_diameter = std::cmp::max(diameter, max_diameter);
        }
        max_diameter
    }
    fn height(node: &Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
        if let Some(n) = node {
            let l = Self::height(&n.as_ref().borrow().left, max_diameter);
            let r = Self::height(&n.as_ref().borrow().right, max_diameter);
            let diameter = l + r;
            *max_diameter = std::cmp::max(diameter, *max_diameter);
            std::cmp::max(&l, &r) + 1
        } else {
            0
        }
    }
}
