// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
//Given the root of a binary tree, each node in the tree has a distinct value.
//
// After deleting all nodes with a value in to_delete, we are left with a forest (a disjoint union of trees).
//
// Return the roots of the trees in the remaining forest. You may return the result in any order.
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
use std::collections::HashSet;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut roots: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        let mut set_to_delete: HashSet<i32> = HashSet::from_iter(to_delete);
        if root.is_some() && !Self::post_order(root.clone(), &mut set_to_delete, &mut roots) {
            roots.push(root.clone());
        }
        roots
    }

    fn post_order(
        node: Option<Rc<RefCell<TreeNode>>>,
        set_to_delete: &mut HashSet<i32>,
        roots: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> bool {
        if let Some(node) = node {
            if Self::post_order(node.borrow().left.clone(), set_to_delete, roots) {
                node.borrow_mut().left = None;
            }
            if Self::post_order(node.borrow().right.clone(), set_to_delete, roots) {
                node.borrow_mut().right = None;
            }
            if set_to_delete.contains(&node.borrow().val) {
                set_to_delete.remove(&node.borrow().val);
                if node.borrow().left.is_some() {
                    roots.push(node.borrow().left.clone());
                }
                if node.borrow().right.is_some() {
                    roots.push(node.borrow().right.clone());
                }
                return true;
            }
        }
        false
    }
}
