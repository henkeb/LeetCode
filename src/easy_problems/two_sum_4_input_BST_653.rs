// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
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
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        fn traverse(
            node: &Option<Rc<RefCell<TreeNode>>>,
            root: &Option<Rc<RefCell<TreeNode>>>,
            k: &i32,
        ) -> bool {
            match node {
                None => false,
                Some(n) => {
                    let n_ref = n.as_ref().borrow();
                    let res = dfs(root, node, k - n_ref.val);
                    if res {
                        res
                    } else {
                        traverse(&n_ref.left, root, k) || traverse(&n_ref.right, root, k)
                    }
                }
            }
        }
        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            caller: &Option<Rc<RefCell<TreeNode>>>,
            target: i32,
        ) -> bool {
            use std::cmp::Ordering;
            if let Some(n) = node {
                if n.as_ref().borrow().val == target && node != caller {
                    return true;
                }
                match n.as_ref().borrow().val.cmp(&target) {
                    Ordering::Equal => node != caller,
                    Ordering::Less => dfs(&n.as_ref().borrow().right, caller, target),
                    Ordering::Greater => dfs(&n.as_ref().borrow().left, caller, target),
                }
            } else {
                false
            }
        }
        traverse(&root, &root, &k)
    }
}
