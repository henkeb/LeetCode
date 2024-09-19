// Solution has
// Time complexity: O(n)
// Space complexity: O(m)
//
// Given a binary tree root and a linked list with head as the first node.
//
// Return True if all the elements in the linked list starting from the head correspond to some downward path connected in the binary tree otherwise return False.
//
// In this context downward path means a path that starts at some node and goes downwards.
//
//
//
// Example 1:
//
// Input: head = [4,2,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
// Output: true
// Explanation: Nodes in blue form a subpath in the binary Tree.
//
// Example 2:
//
// Input: head = [1,4,2,6], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
// Output: true
//
// Example 3:
//
// Input: head = [1,4,2,6,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
// Output: false
// Explanation: There is no path in the binary tree that contains all the elements of the linked list from head.
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
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
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let r = root.unwrap().clone();
        let nodes = Self::get_list(head);

        let mut tree_queue: Vec<(Rc<RefCell<TreeNode>>, Vec<i32>)> = Vec::new();
        tree_queue.push((r, vec![]));

        while let Some((tree_node, mut path)) = tree_queue.pop() {
            path.push(tree_node.as_ref().borrow().val);
            if path.len() > nodes.len() || nodes[0..path.len()] != path {
                path.remove(0);
            }
            if nodes == path {
                return true;
            }
            if let Some(left) = tree_node.as_ref().borrow().left.clone() {
                tree_queue.push((left, path.clone()));
            }
            if let Some(right) = tree_node.as_ref().borrow().right.clone() {
                tree_queue.push((right, path.clone()));
            }
        }
        false
    }

    fn get_list(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut nodes = vec![];
        while let Some(mut n) = head {
            nodes.push(n.val);
            head = n.next.take();
        }
        nodes
    }
}
