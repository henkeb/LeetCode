// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(root) => Self::traverse(&root.borrow().left, &root.borrow().right),
            None => true,
        }
    }

    fn traverse(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(left), Some(right)) if left.borrow().val != right.borrow().val => false,
            (Some(left), Some(right)) => {
                Self::traverse(&left.borrow().left, &right.borrow().right)
                    && Self::traverse(&left.borrow().right, &right.borrow().left)
            }
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symmetric() {
        assert!(Solution::is_symmetric(Some(Rc::new(RefCell::new(
            TreeNode::new(1)
        )))));
    }
}
