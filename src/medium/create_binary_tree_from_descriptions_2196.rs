// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
//You are given a 2D integer array descriptions where descriptions[i] = [parenti, childi, isLefti] indicates that parenti is the parent of childi in a binary tree of unique values. Furthermore,
//
//     If isLefti == 1, then childi is the left child of parenti.
//     If isLefti == 0, then childi is the right child of parenti.
//
// Construct the binary tree described by descriptions and return its root.
//
// The test cases will be generated such that the binary tree is valid.
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
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut nodes: HashMap<i32, Rc<RefCell<TreeNode>>> =
            HashMap::with_capacity(descriptions.len());
        let mut root: HashSet<i32> = HashSet::with_capacity(descriptions.len());
        for description in descriptions.iter() {
            let parent = description[0];
            let child = description[1];
            let is_left = description[2] != 0;

            macro_rules! create_node {
                ($value:expr) => {{
                    root.insert($value);
                    let node = Rc::new(RefCell::new(TreeNode::new($value)));
                    node
                }};
            }

            let parent_node = nodes
                .entry(parent)
                .or_insert_with(|| create_node!(parent))
                .clone();
            let child_node = nodes
                .entry(child)
                .or_insert_with(|| create_node!(child))
                .clone();
            root.remove(&child);
            if is_left {
                parent_node.as_ref().borrow_mut().left = Some(child_node);
            } else {
                parent_node.as_ref().borrow_mut().right = Some(child_node);
            }
        }
        if let Some(root) = root.into_iter().next() {
            nodes.get(&root).cloned()
        } else {
            None
        }
    }
}
