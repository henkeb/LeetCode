// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given an integer array nums where the elements are sorted in ascending order, convert it to a
// height-balanced binary search tree.
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
    fn recurse(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            None
        } else {
            let (left, rest) = nums.split_at(nums.len() / 2);
            let (curr, right) = rest.split_first().unwrap();
            Some(Rc::new(RefCell::new(TreeNode {
                val: *curr,
                left: Self::recurse(left),
                right: Self::recurse(right),
            })))
        }
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::recurse(&nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test2() {
        assert_eq!(
            Solution::sorted_array_to_bst(vec![1, 3]),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: None,
            })))
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::sorted_array_to_bst(vec![1, 2]),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: None,
            })))
        );
    }
}
