// We run a preorder depth-first search (DFS) on the root of a binary tree.
//
// At each node in this traversal, we output D dashes (where D is the depth of this node), then we output the value of this node.  If the depth of a node is D, the depth of its immediate child is D + 1.  The depth of the root node is 0.
//
// If a node has only one child, that child is guaranteed to be the left child.
//
// Given the output traversal of this traversal, recover the tree and return its root.
//
//
//
// Example 1:
//
//
// Input: traversal = "1-2--3--4-5--6--7"
// Output: [1,2,5,3,4,6,7]
// Example 2:
//
//
// Input: traversal = "1-2--3---4-5--6---7"
// Output: [1,2,5,3,null,6,null,4,null,7]
// Example 3:
//
//
// Input: traversal = "1-401--349---90--88"
// Output: [1,401,null,349,88,90]
//
//
// Constraints:
//
// The number of nodes in the original tree is in the range [1, 1000].
// 1 <= Node.val <= 109
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
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut parser = traversal.bytes().peekable();
        let mut stack = Vec::new();

        let mut value = 0;
        while let Some(b) = parser.next_if(u8::is_ascii_digit) {
            value = value * 10 + (b - b'0') as i32;
        }

        stack.push((0, Rc::new(RefCell::new(TreeNode::new(value)))));

        loop {
            let mut depth = 0;

            while let Some(_) = parser.next_if_eq(&b'-') {
                depth += 1;
            }

            let mut value = 0;
            while let Some(b) = parser.next_if(u8::is_ascii_digit) {
                value = value * 10 + (b - b'0') as i32;
            }

            if value == 0 {
                return Some(stack.swap_remove(0).1);
            }

            loop {
                let last = stack.pop().unwrap();
                if last.0 < depth {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(value)));
                    let mut last_borrow = last.1.borrow_mut();

                    if last_borrow.left.is_some() {
                        last_borrow.right = Some(new_node.clone());
                    } else {
                        last_borrow.left = Some(new_node.clone());
                    }

                    drop(last_borrow);
                    stack.push(last);
                    stack.push((depth, new_node));
                    break;
                }
            }
        }
    }
}
