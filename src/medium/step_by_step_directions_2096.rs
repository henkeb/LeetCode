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
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        let mut start_path: Vec<u8> = Vec::new();
        let mut dest_path: Vec<u8> = Vec::new();
        if let Some(root) = root {
            Self::dfs(root.clone(), start_value, &mut start_path);
            Self::dfs(root.clone(), dest_value, &mut dest_path);

            while !start_path.is_empty() && !dest_path.is_empty() {
                let start = start_path.pop().unwrap();
                let dest = dest_path.pop().unwrap();
                if start != dest {
                    start_path.push(start);
                    dest_path.push(dest);
                    break;
                }
            }
            for val in start_path.iter_mut() {
                *val = b'U';
            }
        }
        dest_path.reverse();
        start_path.append(&mut dest_path);
        String::from_utf8(start_path).unwrap()
    }

    fn dfs(node: Rc<RefCell<TreeNode>>, target: i32, path: &mut Vec<u8>) -> bool {
        let node = node.borrow();
        if node.val == target {
            return true;
        }

        if node.left.is_some() && Self::dfs(node.left.clone().unwrap(), target, path) {
            path.push(b'L');
        } else if node.right.is_some() && Self::dfs(node.right.clone().unwrap(), target, path) {
            path.push(b'R');
        }
        !path.is_empty()
    }
}
