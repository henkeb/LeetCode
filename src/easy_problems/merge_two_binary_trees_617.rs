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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node1: &Option<Rc<RefCell<TreeNode>>>, node2: &Option<Rc<RefCell<TreeNode>>>) {
            if let (Some(n1), Some(n2)) = (node1, node2) {
                let mut n1_borrow = n1.as_ref().borrow_mut();
                let n2_borrow = n2.as_ref().borrow_mut();
                n1_borrow.val += n2_borrow.val;
                match (&n1_borrow.left, &n2_borrow.left) {
                    (None, None) => (),
                    (Some(_), None) => (),
                    (None, Some(l2)) => n1_borrow.left = Some(l2.clone()),
                    (Some(_), Some(_)) => {
                        dfs(&n1_borrow.left, &n2_borrow.left);
                    }
                }
                match (&n1_borrow.right, &n2_borrow.right) {
                    (None, None) => (),
                    (Some(_), None) => (),
                    (None, Some(r2)) => n1_borrow.right = Some(r2.clone()),
                    (Some(_), Some(_)) => {
                        dfs(&n1_borrow.right, &n2_borrow.right);
                    }
                }
            }
        }
        if root1.is_none() {
            return root2;
        }
        dfs(&root1, &root2);
        root1
    }
}
