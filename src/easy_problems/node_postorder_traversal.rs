// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given the root of a binary tree, return the postorder traversal of its nodes' values.
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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut order: Vec<i32> = vec![];
        if root.is_some() {
            Self::traversal(&root, &mut order);
        }
        order
    }
    fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, order: &mut Vec<i32>) {
        if let Some(root) = root {
            let root_ref = root.as_ref().borrow();
            Self::traversal(&root_ref.left, order);
            Self::traversal(&root_ref.right, order);
            order.push(root_ref.val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::postorder_traversal(None), vec![]);
    }

    #[test]
    fn it_works2() {
        assert_eq!(
            Solution::postorder_traversal(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            vec![1]
        );
    }
}
