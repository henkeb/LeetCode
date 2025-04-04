// Given the root of a binary tree, return the lowest common ancestor of its deepest leaves.
//
// Recall that:
//
// The node of a binary tree is a leaf if and only if it has no children
// The depth of the root of the tree is 0. if the depth of a node is d, the depth of each of its children is d + 1.
// The lowest common ancestor of a set S of nodes, is the node A with the largest depth such that every node in S is in the subtree with root A.
//
//
// Example 1:
//
//
// Input: root = [3,5,1,6,2,0,8,null,null,7,4]
// Output: [2,7,4]
// Explanation: We return the node with value 2, colored in yellow in the diagram.
// The nodes coloured in blue are the deepest leaf-nodes of the tree.
// Note that nodes 6, 0, and 8 are also leaf nodes, but the depth of them is 2, but the depth of nodes 7 and 4 is 3.
// Example 2:
//
// Input: root = [1]
// Output: [1]
// Explanation: The root is the deepest node in the tree, and it's the lca of itself.
// Example 3:
//
// Input: root = [0,1,3,null,2]
// Output: [2]
// Explanation: The deepest leaf node in the tree is 2, the lca of one node is itself.
//
//
// Constraints:
//
// The number of nodes in the tree will be in the range [1, 1000].
// 0 <= Node.val <= 1000
// The values of the nodes in the tree are unique.
//
// Definition for a binary tree node.
//
// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
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
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        use std::cmp::Ordering;
        let mut queue: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        queue.push(root);
        while let Some(Some(node)) = queue.pop() {
            let left_height = Self::find_height(node.borrow().left.clone());
            let right_height = Self::find_height(node.borrow().right.clone());
            println!(
                "node: {}, left_height: {left_height}, right_height: {right_height}",
                node.borrow().val
            );
            match left_height.cmp(&right_height) {
                Ordering::Less => queue.push(node.borrow().right.clone()),
                Ordering::Equal => return Some(node),
                Ordering::Greater => queue.push(node.borrow().left.clone()),
            }
        }
        panic!("should never enter!");
    }

    fn find_height(node: Option<Rc<RefCell<TreeNode>>>) -> usize {
        let mut max_height = 0;
        if node.is_some() {
            println!("FOR NODE: {}", node.clone().unwrap().borrow().val);
        }
        let mut queue: Vec<(Option<Rc<RefCell<TreeNode>>>, usize)> = vec![(node, 1)];
        while let Some((node, height)) = queue.pop() {
            if let Some(node) = node {
                max_height = max_height.max(height);
                queue.push((node.borrow().left.clone(), height + 1));
                queue.push((node.borrow().right.clone(), height + 1));
            }
        }
        println!("HEIGHT {max_height}");
        max_height
    }
}
