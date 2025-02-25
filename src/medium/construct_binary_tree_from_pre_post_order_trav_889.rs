// Given two integer arrays, preorder and postorder where preorder is the preorder traversal of a binary tree of distinct values and postorder is the postorder traversal of the same tree, reconstruct and return the binary tree.
//
// If there exist multiple answers, you can return any of them.
//
//
//
// Example 1:
//
//
// Input: preorder = [1,2,4,5,3,6,7], postorder = [4,5,2,6,7,3,1]
// Output: [1,2,3,4,5,6,7]
// Example 2:
//
// Input: preorder = [1], postorder = [1]
// Output: [1]
//
//
// Constraints:
//
// 1 <= preorder.length <= 30
// 1 <= preorder[i] <= preorder.length
// All the values of preorder are unique.
// postorder.length == preorder.length
// 1 <= postorder[i] <= postorder.length
// All the values of postorder are unique.
// It is guaranteed that preorder and postorder are the preorder traversal and postorder traversal of the same binary tree.
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
type OptNode = Option<Rc<RefCell<TreeNode>>>;
struct Solution;
impl Solution {
    pub fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> OptNode {
        construct(&preorder[..], &postorder[..])
    }
}

fn construct(pre: &[i32], post: &[i32]) -> OptNode {
    match pre.len() {
        0 => None,
        1 => Some(Rc::new(RefCell::new(TreeNode::new(pre[0])))),
        n => {
            let root = pre[0];

            let first = pre[1];
            let idx = post.iter().position(|&x| x == first).unwrap();

            let mut out = TreeNode::new(root);
            out.left = construct(&pre[1..=idx + 1], &post[0..=idx]);
            out.right = construct(&pre[idx + 2..], &post[idx + 1..n - 1]);

            Some(Rc::new(RefCell::new(out)))
        }
    }
}
