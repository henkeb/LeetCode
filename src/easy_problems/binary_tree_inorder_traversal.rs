// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
struct Solution;
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
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        Self::traverse(&root, &mut result);
        result
    }
    fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = node {
            Self::traverse(&node.borrow().left, result);
            result.push(node.borrow().val);
            Self::traverse(&node.borrow().right, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_inorder_traversal() {
        assert_eq!(Solution::inorder_traversal(None), vec![]);
    }
    #[test]
    fn test_inorder_traversal2() {
        assert_eq!(
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            vec![1]
        );
    }
}
