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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::traverse(&p, &q)
    }
    fn traverse(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (Some(p), Some(q)) if p.borrow().val == q.borrow().val => {
                Self::traverse(&p.borrow().left, &q.borrow().left)
                    && Self::traverse(&p.borrow().right, &q.borrow().right)
            }
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_tree() {
        assert!(Solution::is_same_tree(
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        ));
    }
    #[test]
    fn test_is_same_tree2() {
        assert!(!Solution::is_same_tree(
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            Some(Rc::new(RefCell::new(TreeNode::new(2))))
        ))
    }
}
