// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given a binary tree, find its minimum depth.
// The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
// Note: A leaf is a node with no children.
//
//Definition for a binary tree node.
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
type Node = Option<Rc<RefCell<TreeNode>>>;
struct Solution;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, 1)
    }
    fn dfs(node: &Node, depth: i32) -> i32 {
        if let Some(node) = node {
            let n = node.as_ref().borrow();
            match (&n.left, &n.right) {
                (None, None) => depth,
                (Some(..), None) => Self::dfs(&n.left, depth + 1),
                (None, Some(..)) => Self::dfs(&n.right, depth + 1),
                (_, _) => {
                    let left = Self::dfs(&n.left, depth + 1);
                    let right = Self::dfs(&n.right, depth + 1);
                    std::cmp::min(left, right)
                }
            }
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::min_depth(None), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::min_depth(Some(Rc::new(RefCell::new(TreeNode::new(3))))),
            1
        );
    }

    #[test]
    fn test3() {
        let mut root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        root.as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        root.as_mut()
            .unwrap()
            .borrow_mut()
            .right
            .as_mut()
            .unwrap()
            .borrow_mut()
            .left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        root.as_mut()
            .unwrap()
            .borrow_mut()
            .right
            .as_mut()
            .unwrap()
            .borrow_mut()
            .right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        assert_eq!(Solution::min_depth(root), 2);
    }
}
