// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given the root of a binary tree, return its maximum depth.
//
// A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
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

type BTreeNode = Option<Rc<RefCell<TreeNode>>>;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn max_depth(root: BTreeNode) -> i32 {
        Self::depth(&root)
    }

    fn depth(node: &BTreeNode) -> i32 {
        match node {
            Some(node) => {
                max(
                    Self::depth(&node.borrow().left),
                    Self::depth(&node.borrow().right),
                ) + 1
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_depth_binary_tree() {
        assert_eq!(Solution::max_depth(None), 0);
    }

    #[test]
    fn test_maximum_depth_binary_tree_1() {
        assert_eq!(
            Solution::max_depth(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            1
        );
    }
}
