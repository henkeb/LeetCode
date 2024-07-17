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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            node.as_ref()?;
            if node == p || node == q {
                return node;
            }
            let left = dfs(
                node.as_ref().unwrap().borrow().left.clone(),
                p.clone(),
                q.clone(),
            );
            let right = dfs(node.as_ref().unwrap().borrow().right.clone(), p, q);
            match (left, right) {
                (Some(_), Some(_)) => node,
                (Some(tree), None) => Some(tree),
                (None, Some(tree)) => Some(tree),
                _ => None,
            }
        }

        dfs(root, p, q)
    }
}
